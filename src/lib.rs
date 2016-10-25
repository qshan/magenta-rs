// Copyright 2016 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate core;
extern crate magenta_sys;

use std::marker::PhantomData;

use magenta_sys as sys;

type Time = sys::mx_time_t;
pub const TIME_INFINITE: Time = sys::MX_TIME_INFINITE;

// Might it be more Rust-like to call this Error?
#[derive(Debug)]
pub struct Status(sys::mx_status_t);

// TODO: proper bitfield type
type Rights = sys::mx_rights_t;

// TODO: proper bitfield type
type Signals = sys::mx_signals_t;

pub use magenta_sys::MX_FLAG_REPLY_PIPE;

pub struct SignalsState(sys::mx_signals_state_t);

impl SignalsState {
    pub fn satisfied(&self) -> Signals {
        self.0.satisfied
    }

    pub fn satisfiable(&self) -> Signals {
        self.0.satisfiable
    }
}

pub fn current_time() -> Time {
    unsafe { sys::mx_current_time() }
}

pub fn nanosleep(time: Time) {
    unsafe { sys::mx_nanosleep(time); }
}

fn into_result<T, F>(status: sys::mx_status_t, f: F) -> Result<T, Status>
    where F: FnOnce() -> T {
    // All non-negative values are assumed successful. Note: calls that don't try
    // to multiplex success values into status return could be more strict here.
    if status >= 0 {
        Ok(f())
    } else {
        Err(Status(status))
    }
}

// Handles

pub struct HandleRef<'a> {
    handle: sys::mx_handle_t,
    phantom: PhantomData<&'a sys::mx_handle_t>,
}

impl<'a> HandleRef<'a> {
    fn duplicate(&self, rights: Rights) -> Result<Handle, Status> {
        let handle = self.handle;
        let result = unsafe { sys::mx_handle_duplicate(handle, rights) };
        if result < 0 {
            Err(Status(result))
        } else {
            Ok(Handle(result))
        }
    }

    fn wait(&self, signals: Signals, timeout: Time) -> Result<SignalsState, Status> {
        let handle = self.handle;
        let mut state = sys::mx_signals_state_t { satisfied: 0, satisfiable: 0 };
        let status = unsafe {
            sys::mx_handle_wait_one(handle, signals, timeout, &mut state)
        };
        into_result(status, || SignalsState(state))
    }
}

pub trait HandleBase: Sized {
    fn get_ref(&self) -> HandleRef;

    fn raw_handle(&self) -> sys::mx_handle_t {
        self.get_ref().handle
    }

    fn duplicate(&self, rights: Rights) -> Result<Self, Status> {
        self.get_ref().duplicate(rights).map(|handle|
            Self::from_handle(handle))
    }

    fn wait(&self, signals: Signals, timeout: Time) -> Result<SignalsState, Status> {
        self.get_ref().wait(signals, timeout)
    }

    fn from_handle(handle: Handle) -> Self;

    // Not implemented as "From" because it would conflict in From<Handle> case
    fn into_handle(self) -> Handle {
        let raw_handle = self.get_ref().handle;
        std::mem::forget(self);
        Handle(raw_handle)
    }
}

fn handle_drop(handle: sys::mx_handle_t) {
    let _ = unsafe { sys::mx_handle_close(handle) };
}

// An untyped handle

pub struct Handle(sys::mx_handle_t);

impl HandleBase for Handle {
    fn get_ref(&self) -> HandleRef {
        HandleRef { handle: self.0, phantom: Default::default() }
    }

    fn from_handle(handle: Handle) -> Self {
        handle
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        handle_drop(self.0)
    }
}

// Message pipes

pub struct MessagePipe(Handle);

impl HandleBase for MessagePipe {
    fn get_ref(&self) -> HandleRef {
        self.0.get_ref()
    }

    fn from_handle(handle: Handle) -> Self {
        MessagePipe(handle)
    }
}

impl MessagePipe {
    pub fn create(flags: u32) -> Result<(MessagePipe, MessagePipe), Status> {
        unsafe {
            let mut handles = [0, 0];
            let status = sys::mx_msgpipe_create(handles.as_mut_ptr(), flags);
            into_result(status, ||
                (Self::from_handle(Handle(handles[0])),
                    Self::from_handle(Handle(handles[1]))))
        }
    }

    pub fn read(&self, flags: u32, buf: &mut MessageBuf) -> Result<(), Status> {
        unsafe {
            buf.reset_handles();
            let raw_handle = self.raw_handle();
            let mut num_bytes: u32 = size_to_u32_sat(buf.bytes.capacity());
            let mut num_handles: u32 = size_to_u32_sat(buf.handles.capacity());
            let mut status = sys::mx_msgpipe_read(raw_handle, buf.bytes.as_mut_ptr(), &mut num_bytes,
                    buf.handles.as_mut_ptr(), &mut num_handles, flags);
            if status == sys::ERR_BUFFER_TOO_SMALL {
                ensure_capacity(&mut buf.bytes, num_bytes as usize);
                ensure_capacity(&mut buf.handles, num_handles as usize);
                num_bytes = size_to_u32_sat(buf.bytes.capacity());
                num_handles = size_to_u32_sat(buf.handles.capacity());
                status = sys::mx_msgpipe_read(raw_handle, buf.bytes.as_mut_ptr(), &mut num_bytes,
                        buf.handles.as_mut_ptr(), &mut num_handles, flags);
            }
            into_result(status, || {
                buf.bytes.set_len(num_bytes as usize);
                buf.handles.set_len(num_handles as usize);
            })
        }
    }

    fn write_raw(handle: sys::mx_handle_t, bytes: &[u8], handles: &mut Vec<Handle>,
            flags: u32) -> Result<(), Status>
    {
        unsafe {
            if bytes.len() > core::u32::MAX as usize || handles.len() > core::u32::MAX as usize {
                return Err(Status(sys::ERR_OUT_OF_RANGE));
            }
            let n_bytes = bytes.len() as u32;
            let n_handles = handles.len() as u32;
            let status = sys::mx_msgpipe_write(handle, bytes.as_ptr(), n_bytes,
                handles.as_ptr() as *const sys::mx_handle_t, n_handles, flags);
            if status != sys::NO_ERROR {
                return Err(Status(status));
            }
            // Handles were successfully transferred, forget them on sender side
            handles.set_len(0);
            Ok(())
        }
    }

    pub fn write(&self, bytes: &[u8], handles: &mut Vec<Handle>, flags: u32)
            -> Result<(), Status>
    {
        Self::write_raw(self.raw_handle(), bytes, handles, flags)
    }

    pub fn write_reply(self, bytes: &[u8], handles: &mut Vec<Handle>, flags: u32)
            -> Result<(), (Self, Status)>
    {
        let raw_handle = self.raw_handle();
        handles.push(self.into_handle());
        Self::write_raw(raw_handle, bytes, handles, flags).map_err(|status|
            (Self::from_handle(handles.pop().unwrap()), status)
        )
    }
}

#[derive(Default)]
pub struct MessageBuf {
    bytes: Vec<u8>,
    handles: Vec<sys::mx_handle_t>,
    unused_ix: usize,
}

impl MessageBuf {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn n_handles(&self) -> usize {
        self.handles.len() - self.unused_ix
    }

    pub fn handles(&mut self) -> HandleIter {
        HandleIter(self)
    }

    fn drop_handles(&mut self) {
        for &handle in &self.handles[self.unused_ix..] {
            handle_drop(handle);
        }
    }

    fn reset_handles(&mut self) {
        self.drop_handles();
        self.unused_ix = 0;
        self.handles.clear();
    }
}

impl Drop for MessageBuf {
    fn drop(&mut self) {
        self.drop_handles();
    }
}

pub struct HandleIter<'a>(&'a mut MessageBuf);

impl<'a> Iterator for HandleIter<'a> {
    type Item = Handle;

    fn next(&mut self) -> Option<Handle> {
        if self.0.unused_ix == self.0.handles.len() {
            return None
        }
        let handle = self.0.handles[self.0.unused_ix];
        self.0.unused_ix += 1;
        Some(Handle(handle))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.0.n_handles();
        (size, Some(size))
    }
}

fn size_to_u32_sat(size: usize) -> u32 {
    std::cmp::min(size, core::u32::MAX as usize) as u32
}

fn ensure_capacity<T>(vec: &mut Vec<T>, size: usize) {
    let len = vec.len();
    if size > len {
        vec.reserve(size - len);
    }
}

// Wait sets

// This is the lowest level interface, strictly in terms of cookies.

pub struct WaitSet(Handle);

impl HandleBase for WaitSet {
    fn get_ref(&self) -> HandleRef {
        self.0.get_ref()
    }

    fn from_handle(handle: Handle) -> Self {
        WaitSet(handle)
    }
}

impl WaitSet {
    pub fn create() -> Result<WaitSet, Status> {
        let status = unsafe { sys::mx_waitset_create() };
        into_result(status, ||
            WaitSet::from_handle(Handle(status)))
    }

    pub fn add<H>(&self, handle: &H, signals: Signals, cookie: u64) -> Result<(), Status>
        where H: HandleBase
    {
        let status = unsafe {
            sys::mx_waitset_add(self.raw_handle(), handle.raw_handle(), signals, cookie)
        };
        into_result(status, || ())
    }

    pub fn remove(&self, cookie: u64) -> Result<(), Status> {
        let status = unsafe { sys::mx_waitset_remove(self.raw_handle(), cookie) };
        into_result(status, || ())
    }

    // Make sure `results` has enough capacity. Return value is max number of results,
    // possibly useful for increasing capacity.
    pub fn wait(&self, timeout: Time, results: &mut Vec<WaitSetResult>)
        -> Result<usize, Status>
    {
        unsafe {
            let mut num_results = size_to_u32_sat(results.capacity());
            let mut max_results = 0;
            let status = sys::mx_waitset_wait(self.raw_handle(), timeout,
                &mut num_results,
                results.as_mut_ptr() as *mut sys::mx_waitset_result_t,
                &mut max_results);
            if status != sys::NO_ERROR {
                results.clear();
                return Err(Status(status));
            }
            results.set_len(num_results as usize);
            Ok(max_results as usize)
        }
    }
}

pub struct WaitSetResult(sys::mx_waitset_result_t);

impl WaitSetResult {
    pub fn cookie(&self) -> u64 {
        self.0.cookie
    }

    pub fn wait_result(&self) -> Status {
        Status(self.0.wait_result)
    }

    pub fn signals_state(&self) -> SignalsState {
        SignalsState(self.0.signals_state)
    }
}

// Virtual Memory Objects

pub struct Vmo(Handle);

impl HandleBase for Vmo {
    fn get_ref(&self) -> HandleRef {
        self.0.get_ref()
    }

    fn from_handle(handle: Handle) -> Self {
        Vmo(handle)
    }
}

impl Vmo {
    pub fn create(size: u64) -> Result<Vmo, Status> {
        let status = unsafe { sys::mx_vmo_create(size) };
        into_result(status, ||
            Vmo::from_handle(Handle(status)))
    }

    pub fn read(&self, data: &mut [u8], offset: u64) -> Result<usize, Status> {
        unsafe {
            let ssize = sys::mx_vmo_read(self.raw_handle(), data.as_mut_ptr(),
                offset, data.len());
            if ssize < 0 {
                Err(Status(ssize as sys::mx_status_t))
            } else {
                Ok(ssize as usize)
            }
        }
    }

    pub fn write(&self, data: &[u8], offset: u64) -> Result<usize, Status> {
        unsafe {
            let ssize = sys::mx_vmo_write(self.raw_handle(), data.as_ptr(),
                offset, data.len());
            if ssize < 0 {
                Err(Status(ssize as sys::mx_status_t))
            } else {
                Ok(ssize as usize)
            }
        }
    }

    pub fn get_size(&self) -> Result<u64, Status> {
        let mut size = 0;
        let status = unsafe { sys::mx_vmo_get_size(self.raw_handle(), &mut size) };
        into_result(status, || size)
    }

    pub fn set_size(&self, size: u64) -> Result<(), Status> {
        let status = unsafe { sys::mx_vmo_set_size(self.raw_handle(), size) };
        into_result(status, || ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_increases() {
        let time1 = current_time();
        let time2 = current_time();
        assert!(time2 > time1);
    }

    #[test]
    fn sleep() {
        let sleep_ns = 1_000_000;  // 1ms
        let time1 = current_time();
        nanosleep(sleep_ns);
        let time2 = current_time();
        assert!(time2 > time1 + sleep_ns);
    }

    #[test]
    fn vmo_size() {
        let size = 16 * 1024 * 1024;
        let vmo = Vmo::create(size).unwrap();
        assert_eq!(size as u64, vmo.get_size().unwrap());
    }

    #[test]
    fn vmo_read_write() {
        let mut vec1 = vec![0; 16];
        let vmo = Vmo::create(vec1.len() as u64).unwrap();
        vmo.write(b"abcdef", 0).unwrap();
        assert_eq!(16, vmo.read(&mut vec1, 0).unwrap());
        assert_eq!(b"abcdef", &vec1[0..6]);
        vmo.write(b"123", 2).unwrap();
        assert_eq!(16, vmo.read(&mut vec1, 0).unwrap());
        assert_eq!(b"ab123f", &vec1[0..6]);
        assert_eq!(15, vmo.read(&mut vec1, 1).unwrap());
        assert_eq!(b"b123f", &vec1[0..5]);
    }

    #[test]
    fn message_pipe_reply_basic() {
        let (p1, p2) = MessagePipe::create(0).unwrap();

        // Don't need to test trying to include self-handle, ownership forbids it
        let mut empty = vec![];
        let (p1, _status) = p1.write_reply(b"hello", &mut empty, 0).err().unwrap();
        assert!(p1.write(b"hello", &mut empty, 0).is_ok());
        let (p2, _status) = p2.write_reply(b"hello", &mut empty, 0).err().unwrap();
        assert!(p2.write(b"hello", &mut empty, 0).is_ok());

        let (p1, p2) = MessagePipe::create(MX_FLAG_REPLY_PIPE).unwrap();
        let (p1, _status) = p1.write_reply(b"hello", &mut empty, 0).err().unwrap();
        assert!(p1.write(b"hello", &mut empty, 0).is_ok());
        assert!(p2.write(b"hello", &mut empty, 0).is_err());
        assert!(p2.write_reply(b"hello", &mut empty, 0).is_ok());
    }
}

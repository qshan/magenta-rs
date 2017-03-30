// Copyright 2016 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]

extern crate core;

#[macro_use]
extern crate bitflags;

pub type mx_handle_t = i32;

pub type mx_status_t = i32;

pub type mx_futex_t = isize;
pub type mx_paddr_t = usize;

// Auto-generated using tools/gen_status.py
pub const NO_ERROR              : mx_status_t = 0;
pub const ERR_INTERNAL          : mx_status_t = -1;
pub const ERR_NOT_SUPPORTED     : mx_status_t = -2;
pub const ERR_NO_RESOURCES      : mx_status_t = -5;
pub const ERR_NO_MEMORY         : mx_status_t = -4;
pub const ERR_INVALID_ARGS      : mx_status_t = -10;
pub const ERR_WRONG_TYPE        : mx_status_t = -54;
pub const ERR_BAD_SYSCALL       : mx_status_t = -11;
pub const ERR_BAD_HANDLE        : mx_status_t = -12;
pub const ERR_OUT_OF_RANGE      : mx_status_t = -13;
pub const ERR_BUFFER_TOO_SMALL  : mx_status_t = -14;
pub const ERR_BAD_STATE         : mx_status_t = -20;
pub const ERR_NOT_FOUND         : mx_status_t = -3;
pub const ERR_ALREADY_EXISTS    : mx_status_t = -15;
pub const ERR_ALREADY_BOUND     : mx_status_t = -16;
pub const ERR_TIMED_OUT         : mx_status_t = -23;
pub const ERR_HANDLE_CLOSED     : mx_status_t = -24;
pub const ERR_REMOTE_CLOSED     : mx_status_t = -25;
pub const ERR_UNAVAILABLE       : mx_status_t = -26;
pub const ERR_SHOULD_WAIT       : mx_status_t = -27;
pub const ERR_ACCESS_DENIED     : mx_status_t = -30;
pub const ERR_IO                : mx_status_t = -40;
pub const ERR_IO_REFUSED        : mx_status_t = -41;
pub const ERR_IO_DATA_INTEGRITY : mx_status_t = -42;
pub const ERR_IO_DATA_LOSS      : mx_status_t = -43;
pub const ERR_BAD_PATH          : mx_status_t = -50;
pub const ERR_NOT_DIR           : mx_status_t = -51;
pub const ERR_NOT_FILE          : mx_status_t = -52;

pub type mx_time_t = u64;
pub const MX_TIME_INFINITE : mx_time_t = core::u64::MAX;

bitflags! {
    #[repr(C)]
    pub flags mx_signals_t: u32 {
        const MX_SIGNAL_NONE              = 0,
        const MX_OBJECT_SIGNAL_ALL        = 0x00ffffff,
        const MX_USER_SIGNAL_ALL          = 0xff000000,
        const MX_OBJECT_SIGNAL_0          = 1 << 0,
        const MX_OBJECT_SIGNAL_1          = 1 << 1,
        const MX_OBJECT_SIGNAL_2          = 1 << 2,
        const MX_OBJECT_SIGNAL_3          = 1 << 3,
        const MX_OBJECT_SIGNAL_4          = 1 << 4,
        const MX_OBJECT_SIGNAL_5          = 1 << 5,
        const MX_OBJECT_SIGNAL_6          = 1 << 6,
        const MX_OBJECT_SIGNAL_7          = 1 << 7,
        const MX_OBJECT_SIGNAL_8          = 1 << 8,
        const MX_OBJECT_SIGNAL_9          = 1 << 9,
        const MX_OBJECT_SIGNAL_10         = 1 << 10,
        const MX_OBJECT_SIGNAL_11         = 1 << 11,
        const MX_OBJECT_SIGNAL_12         = 1 << 12,
        const MX_OBJECT_SIGNAL_13         = 1 << 13,
        const MX_OBJECT_SIGNAL_14         = 1 << 14,
        const MX_OBJECT_SIGNAL_15         = 1 << 15,
        const MX_OBJECT_SIGNAL_16         = 1 << 16,
        const MX_OBJECT_SIGNAL_17         = 1 << 17,
        const MX_OBJECT_SIGNAL_18         = 1 << 18,
        const MX_OBJECT_SIGNAL_19         = 1 << 19,
        const MX_OBJECT_SIGNAL_20         = 1 << 20,
        const MX_OBJECT_SIGNAL_21         = 1 << 21,
        const MX_OBJECT_SIGNAL_22         = 1 << 22,
        const MX_OBJECT_SIGNAL_23         = 1 << 23,
        const MX_USER_SIGNAL_0            = 1 << 24,
        const MX_USER_SIGNAL_1            = 1 << 25,
        const MX_USER_SIGNAL_2            = 1 << 26,
        const MX_USER_SIGNAL_3            = 1 << 27,
        const MX_USER_SIGNAL_4            = 1 << 28,
        const MX_USER_SIGNAL_5            = 1 << 29,
        const MX_USER_SIGNAL_6            = 1 << 30,
        const MX_USER_SIGNAL_7            = 1 << 31,

        // Event
        const MX_EVENT_SIGNALED           = MX_OBJECT_SIGNAL_3.bits,

        // EventPair
        const MX_EPAIR_SIGNALED           = MX_OBJECT_SIGNAL_3.bits,
        const MX_EPAIR_CLOSED             = MX_OBJECT_SIGNAL_2.bits,

        // Task signals (process, thread, job)
        const MX_TASK_TERMINATED          = MX_OBJECT_SIGNAL_3.bits,

        // Channel
        const MX_CHANNEL_READABLE         = MX_OBJECT_SIGNAL_0.bits,
        const MX_CHANNEL_WRITABLE         = MX_OBJECT_SIGNAL_1.bits,
        const MX_CHANNEL_PEER_CLOSED      = MX_OBJECT_SIGNAL_2.bits,

        // Socket
        const MX_SOCKET_READABLE          = MX_OBJECT_SIGNAL_0.bits,
        const MX_SOCKET_WRITABLE          = MX_OBJECT_SIGNAL_1.bits,
        const MX_SOCKET_PEER_CLOSED       = MX_OBJECT_SIGNAL_2.bits,
    }
}

pub type mx_size_t = usize;
pub type mx_ssize_t = isize;

bitflags! {
    #[repr(C)]
    pub flags mx_rights_t: u32 {
        const MX_RIGHT_NONE         = 0,
        const MX_RIGHT_DUPLICATE    = 1 << 0,
        const MX_RIGHT_TRANSFER     = 1 << 1,
        const MX_RIGHT_READ         = 1 << 2,
        const MX_RIGHT_WRITE        = 1 << 3,
        const MX_RIGHT_EXECUTE      = 1 << 4,
        const MX_RIGHT_MAP          = 1 << 5,
        const MX_RIGHT_GET_PROPERTY = 1 << 6,
        const MX_RIGHT_SET_PROPERTY = 1 << 7,
        const MX_RIGHT_DEBUG        = 1 << 8,
        const MX_RIGHT_SAME_RIGHTS  = 1 << 31,
    }
}

// clock ids
pub const MX_CLOCK_MONOTONIC: u32 = 0;

// Socket flags and limits.
pub const MX_SOCKET_HALF_CLOSE: u32 = 1;

#[repr(C)]
pub enum mx_cache_policy_t {
    MX_CACHE_POLICY_CACHED = 0,
    MX_CACHE_POLICY_UNCACHED = 1,
    MX_CACHE_POLICY_UNCACHED_DEVICE = 2,
    MX_CACHE_POLICY_WRITE_COMBINING = 3,
}

#[repr(C)]
pub struct mx_wait_item_t {
    pub handle: mx_handle_t,
    pub waitfor: mx_signals_t,
    pub pending: mx_signals_t,
}

#[repr(C)]
pub struct mx_waitset_result_t {
    pub cookie: u64,
    pub status: mx_status_t,
    pub observed: mx_signals_t,
}

#[repr(C)]
pub struct mx_channel_call_args_t {
    pub wr_bytes: *mut u8,
    pub wr_handles: *mut mx_handle_t,
    pub rd_bytes: *mut u8,
    pub rd_handles: *mut mx_handle_t,
    pub wr_num_bytes: u32,
    pub wr_num_handles: u32,
    pub rd_num_bytes: u32,
    pub rd_num_handles: u32,
}

pub type mx_pci_irq_swizzle_lut_t = [[[u32; 4]; 8]; 32];

#[repr(C)]
pub struct mx_pci_init_arg_t {
    pub dev_pin_to_global_irq: mx_pci_irq_swizzle_lut_t,
    pub num_irqs: u32,
    pub irqs: [mx_irq_t; 32],
    pub ecam_window_count: u32,
    pub ecam_windows: [mx_ecam_window_t],
}

#[repr(C)]
pub struct mx_irq_t {
    pub global_irq: u32,
    pub level_triggered: bool,
    pub active_high: bool,
}

#[repr(C)]
pub struct mx_ecam_window_t {
    pub base: u64,
    pub size: usize,
    pub bus_start: u8,
    pub bus_end: u8,
}

#[repr(C)]
pub struct mx_pcie_get_nth_info_t {
    pub vendor_id: u16,
    pub device_id: u16,
    pub base_class: u8,
    pub sub_class: u8,
    pub program_interface: u8,
    pub revision_id: u8,
    pub bus_id: u8,
    pub dev_id: u8,
    pub func_id: u8,
}

// TODO: Actually a union
pub type mx_rrec_t = [u8; 64];

include!("definitions.rs");

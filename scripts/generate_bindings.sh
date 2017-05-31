set -e
#set -x

command -v bindgen >/dev/null 2>&1 || { echo "Error: I require the bindgen command line tool but it's not installed." >&2; exit 1; }

script_dir=$(dirname $BASH_SOURCE)
script_dir=`cd $script_dir; pwd`
src_dir=`cd $script_dir/../magenta-sys/src; pwd`
default_fuchsia_root=`cd ../..;pwd`

: ${FUCHSIA_ROOT=$default_fuchsia_root} && export FUCHSIA_ROOT

if [ ! -d "$FUCHSIA_ROOT" ] ; then
  echo "Error: Fuchsia root directory '$FUCHSIA_ROOT' does not exist." >&2
  exit 1
fi


: ${FUCHSIA_SYSROOT=$FUCHSIA_ROOT/out/debug-x86-64/sysroot} && export FUCHSIA_SYSROOT

if [ ! -d "$FUCHSIA_SYSROOT" ] ; then
  echo "Error: Fuchsia sysroot '$FUCHSIA_SYSROOT' does not exist." >&2
  exit 1
fi

temp_file_prefix=`basename $0`
TEMP_FILE=`mktemp -t ${temp_file_prefix}`.h || { echo "Error: Can't create temporary file." >&2; exit 1; }
destination=$src_dir/generated_definitions.rs

# bindgen $FUCHSIA_SYSROOT/include/magenta/types.h --whitelist-var MX_.* --whitelist-type mx_.* -o $src_dir/magenta_types.rs -- -I$FUCHSIA_SYSROOT/include
# bindgen $FUCHSIA_SYSROOT/include/magenta/processargs.h --whitelist-var PA.* -o $src_dir/processargs.rs -- -I$FUCHSIA_SYSROOT/include
# bindgen $FUCHSIA_SYSROOT/include/magenta/process.h --whitelist-function mx_.* -o $src_dir/process.rs -- -I$FUCHSIA_SYSROOT/include
bindgen $script_dir/all_headers.h \
    -o $destination \
    --no-layout-tests \
    --use-core \
    --with-derive-default \
    --whitelist-type mx_.* \
    --whitelist-var PA_.* \
    --whitelist-var MX_.* \
    --whitelist-var ERR_.* \
    --whitelist-function mx_.* \
    -- -I$FUCHSIA_SYSROOT/include \
    -Wno-unknown-attributes \
    -std=c11

rustfmt --write-mode overwrite $destination
#rm $TEMP_FILE
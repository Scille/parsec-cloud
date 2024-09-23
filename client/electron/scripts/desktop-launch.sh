#!/bin/bash
# This script is inspired by https://github.com/ubuntu/snapcraft-desktop-helpers
# cspell:words armhf gnueabihf xlocaledir libva vdpau libvdpau libunity glvnd querymodules LOCPATH wdisplay girepository pixbuf moduledir ibus fcitx immodules

declare -A PIDS
function async_exec() {
    $@ &
    PIDS[$!]=$@
}
function wait_for_async_execs() {
    for i in ${!PIDS[@]}
    do
        wait $i && continue || echo "ERROR: ${PIDS[$i]} exited abnormally with status $?"
    done
}

# ensure_dir_exists calls `mkdir -p` if the given path is not a directory.
# This speeds up execution time by avoiding unnecessary calls to mkdir.
#
# Usage: ensure_dir_exists <path> [<mkdir-options>]...
#
function ensure_dir_exists() {
    [ -d "$1" ] ||  mkdir -p "$@"
}

function prepend_dir() {
    local var="$1"
    local dir="$2"
    if [ -d "$dir" ]; then
        eval "export $var=\"\$dir\${$var:+:\$$var}\""
    fi
}

function append_dir() {
    local var="$1"
    local dir="$2"
    if [ -d "$dir" ]; then
        eval "export $var=\"\${$var:+\$$var:}\$dir\""
    fi
}

START=$(date +%s.%N)

# On Fedora $SNAP is under /var and there is some magic to map it to /snap.
# # We need to handle that case and reset $SNAP
SNAP=`echo $SNAP | sed -e "s|/var/lib/snapd||g"`
# SNAP is {snap_root_dir}/parsec/{revision} and we need to get the snap root dir
SNAP_ROOT_DIR=$(realpath $SNAP/../..)
SNAP_META_FILE=$SNAP/meta/snap.yaml
SNAP_CORE=$(grep "^base:" $SNAP_META_FILE | cut -d " " -f2)
# We consider that parsec is using the current core snap
SNAP_CORE_DIR=$SNAP_ROOT_DIR/$SNAP_CORE/current

SNAP_USER_UPDATE_DIR=$SNAP_USER_DATA/.update
ensure_dir_exists $SNAP_USER_UPDATE_DIR -m 700

SNAP_LAST_REVISION_FILE=$SNAP_USER_UPDATE_DIR/last_revision

needs_update=true

. $SNAP_LAST_REVISION_FILE 2>/dev/null || true
if [ "$SNAP_DESKTOP_LAST_REVISION" = "$SNAP_REVISION" ]; then
    needs_update=false
fi

case "$SNAP_ARCH" in
    "amd64") ARCH="x86_64-linux-gnu" ;;
    "armhf") ARCH="arm-linux-gnueabihf" ;;
    "arm64") ARCH="aarch64-linux-gnu" ;;
    "ppc64el") ARCH="powerpc64le-linux-gnu" ;;
    *) ARCH="$SNAP_ARCH-linux-gnu" ;;
esac

export SNAP_LAUNCHER_ARCH_TRIPLET=$ARCH

# Add snap library paths
append_dir LD_LIBRARY_PATH "$SNAP/lib/$ARCH"
append_dir LD_LIBRARY_PATH "$SNAP/usr/lib"
append_dir LD_LIBRARY_PATH "$SNAP/usr/lib/$ARCH"

# Add snap bin paths
append_dir PATH "$SNAP/bin"
append_dir PATH "$SNAP/usr/bin"
append_dir PATH "$SNAP_CORE_DIR/usr/sbin"

# Mesa Libs for OpenGL support
append_dir LD_LIBRARY_PATH $SNAP/usr/lib/$ARCH/mesa
append_dir LD_LIBRARY_PATH $SNAP/usr/lib/$ARCH/mesa-egl

# Set cache folder to local path
export XDG_CACHE_HOME=$SNAP_USER_COMMON/.cache
if [[ -d $SNAP_USER_DATA/.cache && ! -e $XDG_CACHE_HOME ]]; then
  # the .cache directory used to be stored under $SNAP_USER_DATA, migrate it
  mv $SNAP_USER_DATA/.cache $SNAP_USER_COMMON/
fi
ensure_dir_exists $XDG_CACHE_HOME -m 700

# XKB config
export XKB_CONFIG_ROOT=$SNAP/usr/share/X11/xkb

# Give XOpenIM a chance to locate locale data.
# This is required for text input to work in SDL2 games.
export XLOCALEDIR=$SNAP/usr/share/X11/locale

# Tell libGL and libva where to find the drivers
if [ -d $SNAP/usr/lib/$ARCH/dri ]; then
    export LIBGL_DRIVERS_PATH=$SNAP/usr/lib/$ARCH/dri
    append_dir LD_LIBRARY_PATH $LIBGL_DRIVERS_PATH
    export LIBVA_DRIVERS_PATH=$SNAP/usr/lib/$ARCH/dri
fi

# Set where the VDPAU drivers are located
export VDPAU_DRIVER_PATH="/usr/lib/$ARCH/vdpau/"
if [ -e "/var/lib/snapd/lib/gl/vdpau/libvdpau_nvidia.so" ]; then
  export VDPAU_DRIVER_PATH="/var/lib/snapd/lib/gl/vdpau"
  # Prevent picking VA-API (Intel/AMD) over NVIDIA VDPAU; on PRIME systems for example
  unset LIBVA_DRIVERS_PATH
fi

# Unity7 export (workaround for https://launchpad.net/bugs/1638405)
append_dir LD_LIBRARY_PATH $SNAP/usr/lib/$ARCH/libunity

# Pulseaudio export
append_dir LD_LIBRARY_PATH $SNAP/usr/lib/$ARCH/pulseaudio

# EGL vendor files on glvnd enabled systems
[ -d /var/lib/snapd/lib/glvnd/egl_vendor.d ] && \
    prepend_dir __EGL_VENDOR_LIBRARY_DIRS /var/lib/snapd/lib/glvnd/egl_vendor.d

# EGL vendor files
append_dir __EGL_VENDOR_LIBRARY_DIRS $SNAP/etc/glvnd/egl_vendor.d
append_dir __EGL_VENDOR_LIBRARY_DIRS $SNAP/usr/share/glvnd/egl_vendor.d

# Ensure the app finds locale definitions (requires locales-all to be installed)
append_dir LOCPATH $SNAP/usr/lib/locale

# If detect wayland server socket, then set environment so applications prefer
# wayland, and setup compat symlink (until we use user mounts. Remember,
# XDG_RUNTIME_DIR is /run/user/<uid>/snap.$SNAP so look in the parent directory
# for the socket. For details:
# https://forum.snapcraft.io/t/wayland-dconf-and-xdg-runtime-dir/186/10
# Applications that don't support wayland natively may define DISABLE_WAYLAND
# (to any non-empty value) to skip that logic entirely.
wayland_available=false
if [[ -n "$XDG_RUNTIME_DIR" && -z "$DISABLE_WAYLAND" ]]; then
    wdisplay="wayland-0"
    if [ -n "$WAYLAND_DISPLAY" ]; then
        wdisplay="$WAYLAND_DISPLAY"
    fi
    wayland_sock_path="$XDG_RUNTIME_DIR/$wdisplay"
    if [ -S "$wayland_sock_path" ]; then
        # if running under wayland, use it
        #export WAYLAND_DEBUG=1
        wayland_available=true
    else
        # Consider that XDG_RUNTIME_DIR is /run/user/<uid>/snap.$SNAP
        wayland_sock_path="$XDG_RUNTIME_DIR/../$wdisplay"
        wayland_snap_path="$XDG_RUNTIME_DIR/$wdisplay"
        if [ -S "$wayland_sock_path" ]; then
            # if running under wayland, use it
            #export WAYLAND_DEBUG=1
            wayland_available=true
            # create the compat symlink for now
            if [ ! -e "$wayland_snap_path" ]; then
                ln -s "$wayland_sock_path" "$wayland_snap_path"
            fi
        fi
    fi
fi

# Make PulseAudio socket available inside the snap-specific $XDG_RUNTIME_DIR
if [ -n "$XDG_RUNTIME_DIR" ]; then
    pulse_native="pulse/native"
    for path in "$XDG_RUNTIME_DIR/"{,../}; do
        pulseaudio_sock_path="$path/$pulse_native"
        if [ -S "$pulseaudio_sock_path" ]; then
            export PULSE_SERVER="unix:${pulseaudio_sock_path}"
            break
        fi
    done
fi

# GI repository
prepend_dir GI_TYPELIB_PATH $SNAP/usr/lib/$ARCH/girepository-1.0
prepend_dir GI_TYPELIB_PATH $SNAP/usr/lib/girepository-1.0
prepend_dir GI_TYPELIB_PATH $SNAP/usr/lib/gjs/girepository-1.0

# Gio modules and cache (including gsettings module)
export GIO_MODULE_DIR=$XDG_CACHE_HOME/gio-modules
GIO_LAST_UPDATE_FILE=$SNAP_USER_UPDATE_DIR/gio-modules.sha256
function compile_gio_modules {
    if [ -f $1/glib-2.0/gio-querymodules ]; then
        rm -rf $GIO_MODULE_DIR
        ensure_dir_exists $GIO_MODULE_DIR
        ln -s $1/gio/modules/*.so $GIO_MODULE_DIR
        $1/glib-2.0/gio-querymodules $GIO_MODULE_DIR
        sha256sum $1/gio/modules/*.so > $GIO_LAST_UPDATE_FILE
    fi
}
# Re-compile gio module cache if it does not exist or if the modules have changed
# if [ $needs_update = true ]; then
if ! ([ -f $GIO_LAST_UPDATE_FILE ] && sha256sum --check $GIO_LAST_UPDATE_FILE); then
    async_exec compile_gio_modules $SNAP/usr/lib/$ARCH
fi

# Gdk-pixbuf loaders
export GDK_PIXBUF_MODULE_FILE=$XDG_CACHE_HOME/gdk-pixbuf-loaders.cache
export GDK_PIXBUF_MODULEDIR=$SNAP/usr/lib/$ARCH/gdk-pixbuf-2.0/2.10.0/loaders
GDK_PIXBUF_UPDATE_FILE=$SNAP_USER_UPDATE_DIR/gdk-pixbuf-loaders.sha256
function update_gdk_pixbuf {
    rm -f $GDK_PIXBUF_MODULE_FILE
    local loader="$SNAP/usr/lib/$ARCH/gdk-pixbuf-2.0/gdk-pixbuf-query-loaders"
    if [ -f $loader ]; then
        mkdir -p $(dirname $GDK_PIXBUF_MODULE_FILE)
        $loader > $GDK_PIXBUF_MODULE_FILE
        sha256sum $loader $GDK_PIXBUF_MODULEDIR/*.so > $GDK_PIXBUF_UPDATE_FILE
    fi
}
# FIXME: Better update detection
# if [ $needs_update = true ] || [ ! -f $GDK_PIXBUF_MODULE_FILE ]; then
if ! ([ -f $GDK_PIXBUF_UPDATE_FILE ] && sha256sum --check $GDK_PIXBUF_UPDATE_FILE); then
    async_exec update_gdk_pixbuf
fi

if [ "$wayland_available" = true ]; then
    export GDK_BACKEND="wayland"
    export CLUTTER_BACKEND="wayland"
    # Does not hurt to specify this as well, just in case
    export QT_QPA_PLATFORM=wayland-egl
fi
append_dir GTK_PATH $SNAP/usr/lib/$ARCH/gtk-3.0

# ibus and fcitx integration
GTK_IM_MODULE_DIR=$XDG_CACHE_HOME/immodules
export GTK_IM_MODULE_FILE=$GTK_IM_MODULE_DIR/immodules.cache
GTK_IM_MODULE_UPDATE_FILE=$SNAP_USER_UPDATE_DIR/immodules.sha256
function update_gtk_im_module {
    rm -f $GTK_IM_MODULE_FILE
    local query_immodules="$SNAP/usr/lib/$ARCH/libgtk-3-0/gtk-query-immodules-3.0"
    local snap_gtk_im_module_dir="$SNAP/usr/lib/$ARCH/gtk-3.0/3.0.0/immodules"
    ln -s $snap_gtk_im_module_dir/*.so $GTK_IM_MODULE_DIR
    $query_immodules > $GTK_IM_MODULE_FILE
    sha256sum $query_immodules $snap_gtk_im_module_dir/*.so > $GTK_IM_MODULE_UPDATE_FILE
}
# FIXME: Better update detection
# if [ $needs_update = true ]; then
if ! ([ -f $GTK_IM_MODULE_UPDATE_FILE ] && sha256sum --check $GTK_IM_MODULE_UPDATE_FILE); then
    async_exec update_gtk_im_module
fi

[ $needs_update = true ] && echo "SNAP_DESKTOP_LAST_REVISION=$SNAP_REVISION" > $SNAP_LAST_REVISION_FILE

# migrate_devices_from_snap_user_data will move all devices from the snap user data directory to the home config dir ('~/.config/parsec3/libparsec/devices').
# It will search for any `.keys` files that have `parsec` in their path and move them to the new location (but keep a copy at the previous location).
function migrate_devices_from_snap_user_data {
    local config_dir=${XDG_CONFIG_HOME:-$HOME/.config}
    local device_dir=$config_dir/parsec3/libparsec/devices
    mkdir -pv $device_dir
    for old_device_dir in $(find $HOME/snap -type f -name '*.keys' 2>/dev/null | grep parsec | xargs dirname | sort | uniq); do
        local sentinel_file=$old_device_dir/.parsec-device-already-migrated
        if [ -e $sentinel_file ]; then
            continue
        fi
        echo "Migrating devices in $old_device_dir to $device_dir"
        cp --archive --update=none --verbose $old_device_dir/*.keys $device_dir && touch $sentinel_file
    done
}
async_exec migrate_devices_from_snap_user_data $SNAP_USER_DATA

wait_for_async_execs

if [ -n "$SNAP_DESKTOP_DEBUG" ]; then
  echo "desktop-launch elapsed time: " $(date +%s.%N --date="$START seconds ago")
  echo "desktop-launch environment:"
  echo "  SNAP=$SNAP"
  echo "  LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
  echo "  PATH=$PATH"
  echo "Now running: exec $@"
fi

exec "$@"

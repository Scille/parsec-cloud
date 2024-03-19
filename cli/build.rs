// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

fn main() {
    #[cfg(target_os = "windows")]
    // WinFSP is provided as a DLL that our main binary must load.
    // By default Windows look for DLL in 1) the executable directory and 2) the Windows system folders.
    // However this doesn't work for WinFSP given it is distributed as a separate program
    // (and hence end up install in an arbitrary directory we must retrieve at the runtime
    // by querying the Windows Registry) .
    // Long story short, we are here here informing the linker the WinFSP DLL must be lazy
    // loaded ("delayload" option in MSVC) so that we will have time to first configure the
    // lookup directory.
    winfsp_wrs_build::build();
}

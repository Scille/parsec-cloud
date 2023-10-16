// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

fn main() {
    #[cfg(not(target_os = "macos"))]
    {
        use std::path::Path;

        use libparsec_platform_mountpoint::{FileSystemMounted, MemFS};

        env_logger::init();
        let mut args = std::env::args();

        #[cfg(target_os = "windows")]
        winfsp_wrs::init().expect("Can't init WinFSP");

        let arg = args.nth(1).expect("Please provide mountpoint path");
        let path = Path::new(&arg);

        let fs = FileSystemMounted::mount(path, MemFS::default()).unwrap();

        let (tx, rx) = std::sync::mpsc::channel();
        ctrlc::set_handler(move || tx.send(()).unwrap()).unwrap();
        rx.recv().unwrap();

        fs.stop();
    }
}

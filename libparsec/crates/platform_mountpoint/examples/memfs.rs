// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use std::{path::PathBuf, str::FromStr, sync::Arc};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    #[cfg(not(target_os = "macos"))]
    {
        use libparsec_platform_mountpoint::{FileSystemMounted, MemFS};

        env_logger::init();
        let mut args = std::env::args();

        #[cfg(target_os = "windows")]
        winfsp_wrs::init().expect("Can't init WinFSP");

        let arg = args.nth(1).expect("Please provide mountpoint path");
        let path = PathBuf::from_str(&arg).unwrap();

        let fs = FileSystemMounted::mount(path, Arc::new(MemFS::default()))
            .await
            .unwrap();

        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
        // let (tx, rx) = std::sync::mpsc::channel();
        ctrlc::set_handler(move || {
            let _ = tx.try_send(());
        })
        .unwrap();
        rx.recv().await.unwrap();

        fs.stop().await.unwrap();
    }
}

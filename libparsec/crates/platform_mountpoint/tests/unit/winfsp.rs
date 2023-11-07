// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use chrono::Utc;

use libparsec_tests_fixtures::{tmp_path, TmpPath};
use libparsec_tests_lite::{p_assert_eq, parsec_test};
use libparsec_types::{EntryName, FsPath, VlobID};

use crate::{EntryInfo, EntryInfoType, FileSystemMounted, MemFS};

#[parsec_test]
fn winify_applied_for_read_dir(tmp_path: TmpPath) {
    winfsp_wrs::init().expect("Can't init WinFSP");

    // Windows can't mount on existing directory
    let path = tmp_path.join("mountpoint");
    let memfs = MemFS::default();

    {
        let mut entries = memfs.entries.lock().unwrap();
        let root: FsPath = "/".parse().unwrap();

        let now = Utc::now();
        for i in 1..10 {
            let id = VlobID::default();
            let name: EntryName = format!("COM{i}").parse().unwrap();
            let path = root.join(name.clone());

            entries.get_mut(&root).unwrap().0.children.insert(name, id);

            entries.insert(path, (EntryInfo::new(id, EntryInfoType::Dir, now), None));
        }
    }

    let fs = FileSystemMounted::mount(&path, memfs).unwrap();

    let mut entries = std::fs::read_dir(path)
        .unwrap()
        .map(|x| x.unwrap().file_name())
        .collect::<Vec<_>>();

    entries.sort();

    for (i, entry) in entries.iter().enumerate() {
        p_assert_eq!(entry, format!("COM~{:02x}", b'0' + i as u8 + 1).as_str())
    }

    fs.stop();
}

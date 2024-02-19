// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod inode;

use fuser::{
    FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory,
    ReplyEmpty, ReplyEntry, ReplyOpen, ReplyWrite, Request, Session,
};
use libc::{EINVAL, ENAMETOOLONG, ENOENT, ENOTEMPTY, EPERM};
use libparsec_types::{anyhow, EntryName, EntryNameError, EntryNameResult, FileDescriptor};
use std::{ffi::OsStr, path::Path, time::Duration};

use crate::{
    error::MountpointError, EntryInfo, EntryInfoType, FileSystemMounted, FileSystemWrapper,
    MountpointInterface, WriteMode,
};

use inode::Inode;
pub(crate) use inode::InodeManager;

/// TODO: Do we need to change this value ?
/// Validity timeout
const TTL: Duration = Duration::ZERO;

/// TODO: Do we need to handle any other GENERATION ?
///
/// If the file system will be exported over NFS, the inode/generation pairs need
/// to be unique over the file system's lifetime (rather than just the mount
/// time). So if the file system reuses an inode after it has been deleted, it
/// must assign a new, previously unused generation number to the inode at the
/// same time.
const GENERATION: u64 = 0;
const BLOCK_SIZE: u64 = 512;

impl From<EntryInfoType> for FileType {
    fn from(value: EntryInfoType) -> Self {
        match value {
            EntryInfoType::Dir => Self::Directory,
            EntryInfoType::File => Self::RegularFile,
            EntryInfoType::ParsecEntryInfo => Self::RegularFile,
        }
    }
}

fn entry_info_to_file_attr(entry_info: EntryInfo, inode: Inode) -> FileAttr {
    let created = entry_info.created;
    let updated = entry_info.updated;

    let (kind, perm) = match entry_info.ty {
        EntryInfoType::Dir => (FileType::Directory, 0o755),
        EntryInfoType::File => (FileType::RegularFile, 0o644),
        EntryInfoType::ParsecEntryInfo => (FileType::RegularFile, 0),
    };

    FileAttr {
        ino: inode.into(),
        size: entry_info.size,
        blocks: (entry_info.size + BLOCK_SIZE - 1) / BLOCK_SIZE,
        atime: updated.into(),
        mtime: updated.into(),
        ctime: updated.into(),
        crtime: created.into(),
        kind,
        perm,
        nlink: 0,
        uid: entry_info.id.as_u128() as u32,
        gid: entry_info.id.as_u128() as u32,
        rdev: 0,
        blksize: BLOCK_SIZE as u32,
        flags: 0,
    }
}

impl<T: MountpointInterface> Filesystem for FileSystemWrapper<T> {
    /// `lookup` is called everytime it meets a new ressource which the FileSystem
    /// does not know. It transforms the path name to `inode`.
    /// The `inode` is then used by all other operations and is freed by `forget`.
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            _ => {
                reply.error(EINVAL);
                return;
            }
        };

        // Safety: Parent should exists (resolved by lookup method)
        let parent_path = unsafe { self.get_path(Inode::from(parent)) };
        let path = parent_path.join(name);

        match self.interface.entry_info(&path) {
            Ok(entry) => {
                let inode = self.insert_path(path);
                reply.entry(&TTL, &entry_info_to_file_attr(entry, inode), GENERATION)
            }
            Err(_) => reply.error(ENOENT),
        }
    }

    fn forget(&mut self, _req: &Request, ino: u64, nlookup: u64) {
        // Safety: Current inode should exists (resolved by lookup method) and nlookup should be valid
        unsafe { self.remove_path(Inode::from(ino), nlookup) };
    }

    fn setattr(
        &mut self,
        _req: &Request,
        ino: u64,
        _mode: Option<u32>,
        _uid: Option<u32>,
        _gid: Option<u32>,
        _size: Option<u64>,
        _atime: Option<fuser::TimeOrNow>,
        _mtime: Option<fuser::TimeOrNow>,
        _ctime: Option<std::time::SystemTime>,
        _fh: Option<u64>,
        _crtime: Option<std::time::SystemTime>,
        _chgtime: Option<std::time::SystemTime>,
        _bkuptime: Option<std::time::SystemTime>,
        _flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        let inode = Inode::from(ino);
        // Safety: Current file/directory should exists (resolved by lookup method)
        let path = unsafe { self.get_path(inode) };

        // TODO: do we want to handle setattr ?

        let entry = self.interface.entry_info(&path).expect("Should exists");

        reply.attr(&TTL, &entry_info_to_file_attr(entry, inode));
    }

    fn statfs(&mut self, _req: &Request, _ino: u64, reply: fuser::ReplyStatfs) {
        // We have currently no way of easily getting the size of workspace
        // Also, the total size of a workspace is not limited
        // For the moment let's settle on 0 MB used for 1 TB available
        reply.statfs(
            2 * 1024u64.pow(2),
            2 * 1024u64.pow(2),
            2 * 1024u64.pow(2),
            0,
            0,
            512 * 1024,
            255,
            512 * 1024,
        )
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        let inode = Inode::from(ino);
        // Safety: Current file/directory should exists (resolved by lookup method)
        let path = unsafe { self.get_path(inode) };

        if let Ok(entry) = self.interface.entry_info(&path) {
            reply.attr(&TTL, &entry_info_to_file_attr(entry, inode));
            return;
        }

        reply.error(EPERM)
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        mut offset: i64,
        mut reply: ReplyDirectory,
    ) {
        if offset < 1 {
            let _ = reply.add(1, 1, FileType::Directory, ".");
        }

        if offset < 2 {
            let _ = reply.add(1, 2, FileType::Directory, "..");
        }

        // Safety: Current directory should exists (resolved by lookup method)
        let path = unsafe { self.get_path(Inode::from(ino)) };

        if let Ok(entry) = self.interface.entry_info(&path) {
            if offset >= 2 {
                offset -= 2;
                for (i, (name, id)) in entry.children.iter().enumerate().skip(offset as usize) {
                    let path = path.join(name.clone());
                    let entry = self.interface.entry_info(&path).expect("Should exists");

                    if reply.add(
                        id.as_u128() as u64,
                        (i + 3) as i64,
                        entry.ty.into(),
                        name.as_ref(),
                    ) {
                        break;
                    }
                }
            }
            reply.ok();
            return;
        }

        reply.error(EPERM)
    }

    fn create(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        _mode: u32,
        _umask: u32,
        flags: i32,
        reply: ReplyCreate,
    ) {
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            _ => {
                reply.error(EINVAL);
                return;
            }
        };

        // Safety: Parent should exists (resolved by lookup method)
        let parent_path = unsafe { self.get_path(Inode::from(parent)) };
        let path = parent_path.join(name);

        match self.interface.file_create(&path, true) {
            Ok(fd) => {
                let entry = self.interface.entry_info(&path).expect("Just created");
                let inode = self.insert_path(path);
                reply.created(
                    &TTL,
                    &entry_info_to_file_attr(entry, inode),
                    GENERATION,
                    fd.0 as u64,
                    flags as u32,
                )
            }
            Err(MountpointError::NotFound) => reply.error(ENOENT),
            Err(_) => reply.error(EPERM),
        }
    }

    fn open(&mut self, _req: &Request, ino: u64, flags: i32, reply: ReplyOpen) {
        // Safety: Current file/directory should exists (resolved by lookup method)
        let path = unsafe { self.get_path(Inode::from(ino)) };

        let write_mode = (1..=2).contains(&(flags & 0b11));
        let fd = match self.interface.file_open(&path, write_mode) {
            Ok(Some(fd)) => fd,
            Err(MountpointError::NotFound) => {
                reply.error(ENOENT);
                return;
            }
            _ => {
                reply.error(EPERM);
                return;
            }
        };

        reply.opened(fd.0 as u64, flags as u32)
    }

    fn release(
        &mut self,
        _req: &Request,
        _ino: u64,
        fh: u64,
        _flags: i32,
        _lock_owner: Option<u64>,
        _flush: bool,
        reply: ReplyEmpty,
    ) {
        self.interface.fd_close(FileDescriptor(fh as u32));
        reply.ok();
    }

    fn read(
        &mut self,
        _req: &Request,
        _ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock: Option<u64>,
        reply: ReplyData,
    ) {
        let mut buffer = self.buffer.lock().expect("Mutex is poisoned");

        buffer.resize(size as usize, 0);

        match self
            .interface
            .fd_read(FileDescriptor(fh as u32), &mut buffer, offset as u64)
        {
            Ok(_) => reply.data(&buffer),
            Err(MountpointError::NotFound) => reply.error(ENOENT),
            Err(_) => reply.error(EPERM),
        }
    }

    fn write(
        &mut self,
        _req: &Request,
        _ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        match self.interface.fd_write(
            FileDescriptor(fh as u32),
            data,
            offset as u64,
            WriteMode::Normal,
        ) {
            Ok(size) => reply.written(size as u32),
            Err(MountpointError::NotFound) => reply.error(ENOENT),
            Err(_) => reply.error(EPERM),
        }
    }

    fn unlink(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            _ => {
                reply.error(EINVAL);
                return;
            }
        };

        // Safety: Parent should exists (resolved by lookup method)
        let parent_path = unsafe { self.get_path(Inode::from(parent)) };
        let path = parent_path.join(name);

        match self.interface.file_delete(&path) {
            Ok(_) => reply.ok(),
            Err(MountpointError::NotFound) => reply.error(ENOENT),
            Err(_) => reply.error(EPERM),
        }
    }

    fn mkdir(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        _mode: u32,
        _umask: u32,
        reply: ReplyEntry,
    ) {
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            _ => {
                reply.error(EINVAL);
                return;
            }
        };

        // Safety: Parent should exists (resolved by lookup method)
        let parent_path = unsafe { self.get_path(Inode::from(parent)) };
        let path = parent_path.join(name);

        match self.interface.dir_create(&path) {
            Ok(_) => {
                let entry = self.interface.entry_info(&path).expect("Path just created");
                let inode = self.insert_path(path);
                reply.entry(&TTL, &entry_info_to_file_attr(entry, inode), GENERATION)
            }
            Err(MountpointError::NameTooLong) => reply.error(ENAMETOOLONG),
            Err(MountpointError::NotFound) => reply.error(ENOENT),
            Err(_) => reply.error(EPERM),
        }
    }

    fn rmdir(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            _ => {
                reply.error(EINVAL);
                return;
            }
        };

        // Safety: Parent should exists (resolved by lookup method)
        let parent_path = unsafe { self.get_path(Inode::from(parent)) };
        let path = parent_path.join(name);

        match self.interface.dir_delete(&path) {
            Ok(_) => reply.ok(),
            Err(MountpointError::DirNotEmpty) => reply.error(ENOTEMPTY),
            Err(MountpointError::NotFound) => reply.error(ENOENT),
            Err(_) => reply.error(EPERM),
        }
    }

    fn rename(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        new_parent: u64,
        newname: &OsStr,
        _flags: u32,
        reply: ReplyEmpty,
    ) {
        let (name, newname) = match (os_name_to_entry_name(name), os_name_to_entry_name(newname)) {
            (Ok(name), Ok(newname)) => (name, newname),
            _ => {
                reply.error(EINVAL);
                return;
            }
        };

        // Safety: Parent should exists (resolved by lookup method)
        let parent_path = unsafe { self.get_path(Inode::from(parent)) };
        let source = parent_path.join(name);

        // Safety: Parent should exists (resolved by lookup method)
        let new_parent_path = unsafe { self.get_path(Inode::from(new_parent)) };
        let destination = new_parent_path.join(newname);

        match self.interface.entry_rename(&source, &destination, true) {
            Ok(_) => {
                self.rename_path(&source, &destination);
                reply.ok()
            }
            Err(MountpointError::InvalidName) => reply.error(EINVAL),
            Err(MountpointError::NotFound) => reply.error(ENOENT),
            Err(_) => reply.error(EPERM),
        }
    }

    fn flush(&mut self, _req: &Request, _ino: u64, fh: u64, _lock_owner: u64, reply: ReplyEmpty) {
        self.interface.fd_flush(FileDescriptor(fh as u32));
        reply.ok();
    }

    fn fsync(&mut self, _req: &Request, _ino: u64, fh: u64, _datasync: bool, reply: ReplyEmpty) {
        self.interface.fd_flush(FileDescriptor(fh as u32));
        reply.ok();
    }

    fn fsyncdir(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _datasync: bool,
        reply: ReplyEmpty,
    ) {
        // todo
        reply.ok();
    }
}

// We do the same as WinFSP: try to create a directory
pub(super) fn mount<I: MountpointInterface + Send + 'static>(
    mountpoint: &Path,
    interface: I,
) -> anyhow::Result<FileSystemMounted<I>> {
    let fs_wrapper = FileSystemWrapper::new(interface);
    let mountpoint = mountpoint.to_path_buf();
    std::fs::create_dir(&mountpoint)?;

    let mut se = Session::new(
        fs_wrapper,
        &mountpoint,
        &[MountOption::FSName("memfs".into())],
    )?;

    let unmounter = se.unmount_callable();

    std::thread::spawn(move || {
        se.run().expect("Can't run");
    });

    Ok(FileSystemMounted {
        unmounter,
        mountpoint,
        phantom: Default::default(),
    })
}

fn os_name_to_entry_name(name: &OsStr) -> EntryNameResult<EntryName> {
    name.to_str()
        .map(|s| s.parse())
        .ok_or(EntryNameError::InvalidName)?
}

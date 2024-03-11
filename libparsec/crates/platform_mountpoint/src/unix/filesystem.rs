// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    ffi::OsStr,
    sync::{Arc, Mutex},
};

use libparsec_client::workspace::{
    EntryStat, FileStat, OpenOptions, WorkspaceCreateFolderError, WorkspaceFdCloseError,
    WorkspaceFdResizeError, WorkspaceFdStatError, WorkspaceFdWriteError, WorkspaceOpenFileError,
    WorkspaceOps, WorkspaceRemoveEntryError, WorkspaceRenameEntryError, WorkspaceStatEntryError,
};
use libparsec_types::prelude::*;

use super::inode::{Inode, InodesManager};

/// TODO: Do we need to change this value ?
/// Validity timeout
const TTL: std::time::Duration = std::time::Duration::ZERO;

/// TODO: Do we need to handle any other GENERATION ?
///
/// If the file system will be exported over NFS, the inode/generation pairs need
/// to be unique over the file system's lifetime (rather than just the mount
/// time). So if the file system reuses an inode after it has been deleted, it
/// must assign a new, previously unused generation number to the inode at the
/// same time.
const GENERATION: u64 = 0;
const BLOCK_SIZE: u64 = 512;
const PERMISSIONS: u16 = 0o700;

macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => { println!($target, $($arg)+) };
    ($($arg:tt)+) => { println!($($arg)+) };
}

fn os_name_to_entry_name(name: &OsStr) -> EntryNameResult<EntryName> {
    name.to_str()
        .map(|s| s.parse())
        .ok_or(EntryNameError::InvalidName)?
}

fn file_stat_to_file_attr(stat: FileStat, inode: Inode, uid: u32, gid: u32) -> fuser::FileAttr {
    let created: std::time::SystemTime = stat.created.into();
    let updated: std::time::SystemTime = stat.updated.into();
    fuser::FileAttr {
        ino: inode,
        size: stat.size,
        blocks: (stat.size + BLOCK_SIZE - 1) / BLOCK_SIZE,
        atime: updated,
        mtime: updated,
        ctime: created,
        crtime: created,
        kind: fuser::FileType::RegularFile,
        perm: PERMISSIONS,
        nlink: 1,
        uid,
        gid,
        rdev: 0,
        blksize: BLOCK_SIZE as u32,
        flags: 0,
    }
}

fn entry_stat_to_file_attr(stat: EntryStat, inode: Inode, uid: u32, gid: u32) -> fuser::FileAttr {
    match stat {
        EntryStat::File {
            created,
            updated,
            size,
            ..
        } => {
            let created: std::time::SystemTime = created.into();
            let updated: std::time::SystemTime = updated.into();
            fuser::FileAttr {
                ino: inode,
                size,
                blocks: (size + BLOCK_SIZE - 1) / BLOCK_SIZE,
                atime: updated,
                mtime: updated,
                ctime: created,
                crtime: created,
                kind: fuser::FileType::RegularFile,
                perm: PERMISSIONS,
                nlink: 1,
                uid,
                gid,
                rdev: 0,
                blksize: BLOCK_SIZE as u32,
                flags: 0,
            }
        }

        EntryStat::Folder {
            created, updated, ..
        } => {
            let created: std::time::SystemTime = created.into();
            let updated: std::time::SystemTime = updated.into();
            fuser::FileAttr {
                ino: inode,
                size: 0,
                blocks: 0,
                atime: updated,
                mtime: updated,
                ctime: updated,
                crtime: created,
                kind: fuser::FileType::Directory,
                perm: PERMISSIONS,
                nlink: 1,
                uid,
                gid,
                rdev: 0,
                blksize: BLOCK_SIZE as u32,
                flags: 0,
            }
        }
    }
}

pub(super) struct Filesystem {
    ops: Arc<WorkspaceOps>,
    tokio_handle: tokio::runtime::Handle,
    inodes: Arc<Mutex<InodesManager>>,
}

impl Filesystem {
    pub fn new(ops: Arc<WorkspaceOps>, tokio_handle: tokio::runtime::Handle) -> Self {
        Self {
            ops,
            tokio_handle,
            inodes: Arc::new(Mutex::new(InodesManager::new())),
        }
    }
}

// TODO: should do lookup using the EntryId instead of the path
async fn reply_with_lookup(
    ops: &WorkspaceOps,
    uid: u32,
    gid: u32,
    inode: Inode,
    entry_id: VlobID,
    reply: fuser::ReplyEntry,
) {
    match ops.stat_entry_by_id(entry_id).await {
        Ok(stat) => reply.entry(
            &TTL,
            &entry_stat_to_file_attr(stat, inode, uid, gid),
            GENERATION,
        ),
        Err(err) => match err {
            WorkspaceStatEntryError::EntryNotFound => reply.error(libc::ENOENT),
            WorkspaceStatEntryError::Offline => reply.error(libc::EHOSTUNREACH),
            WorkspaceStatEntryError::NoRealmAccess => reply.error(libc::EPERM),
            WorkspaceStatEntryError::Stopped
            | WorkspaceStatEntryError::InvalidKeysBundle(_)
            | WorkspaceStatEntryError::InvalidCertificate(_)
            | WorkspaceStatEntryError::InvalidManifest(_)
            | WorkspaceStatEntryError::Internal(_) => reply.error(libc::EIO),
        },
    }
}

/// Not replying to a fuse request lead to filesystem hanging.
/// This macro ensures a generic error response will be returned even if the
/// thread panics.
macro_rules! reply_on_drop_guard {
    ($reply:expr, $reply_ty:ty) => {{
        struct ReplyOnDrop(Option<$reply_ty>);

        impl ReplyOnDrop {
            fn manual(mut self) -> $reply_ty {
                self.0.take().expect("Taken only once")
            }
            #[allow(dead_code)]
            fn borrow(&mut self) -> &mut $reply_ty {
                self.0.as_mut().expect("Taken only once")
            }
        }

        impl Drop for ReplyOnDrop {
            fn drop(&mut self) {
                match self.0.take() {
                    // Already replied
                    None => (),
                    // Not replied time to do it with ourself !
                    Some(reply) => reply.error(libc::EIO),
                }
            }
        }

        ReplyOnDrop(Some($reply))
    }};
}

// Fuse relies on lookup to do checks (e.g. ensuring an entry doesn't exist
// before trying to create it), however this check is not atomic so the actual
// operation may nevertheless fail.
// The bad new is this make testing this case hard, hence this hook that allow
// us trick the lookup into thinking the entry doesn't exist.
#[cfg(test)]
#[allow(clippy::type_complexity)]
pub(crate) static LOOKUP_HOOK: Mutex<
    Option<Box<dyn FnMut(&FsPath) -> Option<Result<EntryStat, WorkspaceStatEntryError>> + Send>>,
> = Mutex::new(None);

impl fuser::Filesystem for Filesystem {
    fn init(
        &mut self,
        _req: &fuser::Request<'_>,
        _config: &mut fuser::KernelConfig,
    ) -> Result<(), libc::c_int> {
        Ok(())
    }

    // TODO: Make sure opened files are automatically closed on umount, otherwise
    //       we would have some cleanup job do to here !
    fn destroy(&mut self) {}

    /// `lookup` is called everytime Fuse meets a new ressource it doesn't know about.
    /// The lookup transforms the path name to an `inode`.
    /// The `inode` is then used by all other operations and is freed by `forget`.
    fn lookup(
        &mut self,
        req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        debug!("[FUSE] lookup(parent: {:#x?}, name: {:?})", parent, name);
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEntry);

        let uid = req.uid();
        let gid = req.gid();
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(libc::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(libc::EINVAL);
                return;
            }
        };
        let ops = self.ops.clone();
        let inodes = self.inodes.clone();
        self.tokio_handle.spawn(async move {
            let path = {
                let inodes_guard = inodes.lock().expect("mutex is poisoned");
                let parent_path = inodes_guard.get_path_or_panic(parent);
                parent_path.join(name)
            };

            let outcome = {
                #[cfg(test)]
                {
                    let mut maybe_outcome = None;
                    {
                        let mut guard = LOOKUP_HOOK.lock().expect("mutex is poisoned");
                        if let Some(lookup_hook) = guard.as_deref_mut() {
                            maybe_outcome = lookup_hook(&path);
                        }
                    }
                    match maybe_outcome {
                        Some(outcome) => outcome,
                        None => ops.stat_entry(&path).await,
                    }
                }
                #[cfg(not(test))]
                {
                    ops.stat_entry(&path).await
                }
            };

            match outcome {
                Ok(stat) => {
                    let mut inodes_guard = inodes.lock().expect("mutex is poisoned");
                    let inode = inodes_guard.insert_path(path);
                    reply.manual().entry(
                        &TTL,
                        &entry_stat_to_file_attr(stat, inode, uid, gid),
                        GENERATION,
                    )
                }
                Err(err) => match err {
                    WorkspaceStatEntryError::EntryNotFound => reply.manual().error(libc::ENOENT),
                    WorkspaceStatEntryError::Offline => reply.manual().error(libc::EHOSTUNREACH),
                    WorkspaceStatEntryError::NoRealmAccess => reply.manual().error(libc::EPERM),
                    WorkspaceStatEntryError::Stopped
                    | WorkspaceStatEntryError::InvalidKeysBundle(_)
                    | WorkspaceStatEntryError::InvalidCertificate(_)
                    | WorkspaceStatEntryError::InvalidManifest(_)
                    | WorkspaceStatEntryError::Internal(_) => reply.manual().error(libc::EIO),
                },
            }
        });
    }

    fn forget(&mut self, _req: &fuser::Request<'_>, ino: u64, nlookup: u64) {
        self.inodes
            .lock()
            .expect("mutex is poisoned")
            .remove_path_or_panic(ino, nlookup);
    }

    fn statfs(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyStatfs) {
        debug!("[FUSE] statfs(ino: {:#x?})", ino);

        // We have currently no way of easily getting the size of workspace
        // Also, the total size of a workspace is not limited
        // For the moment let's settle on 0 MB used for 1 TB available
        reply.statfs(
            2 * 1024u64.pow(2), // 2 MBlocks is 1 TB
            2 * 1024u64.pow(2), // 2 MBlocks is 1 TB
            2 * 1024u64.pow(2), // 2 MBlocks is 1 TB
            0,
            0,
            512 * 1024, // 512 KB, i.e the default block size
            255,        // 255 bytes as maximum length for filenames
            512 * 1024, // 512 KB, i.e the default block size
        )
    }

    fn getattr(&mut self, req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        debug!("[FUSE] getattr(ino: {:#x?})", ino);
        let reply = reply_on_drop_guard!(reply, fuser::ReplyAttr);

        let uid = req.uid();
        let gid = req.gid();
        let path = self
            .inodes
            .lock()
            .expect("mutex is poisoned")
            .get_path_or_panic(ino);
        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            match ops.stat_entry(&path).await {
                Ok(stat) => reply
                    .manual()
                    .attr(&TTL, &entry_stat_to_file_attr(stat, ino, uid, gid)),
                Err(err) => match err {
                    WorkspaceStatEntryError::EntryNotFound => reply.manual().error(libc::ENOENT),
                    WorkspaceStatEntryError::Offline => reply.manual().error(libc::EHOSTUNREACH),
                    WorkspaceStatEntryError::NoRealmAccess => reply.manual().error(libc::EPERM),
                    WorkspaceStatEntryError::Stopped
                    | WorkspaceStatEntryError::InvalidKeysBundle(_)
                    | WorkspaceStatEntryError::InvalidCertificate(_)
                    | WorkspaceStatEntryError::InvalidManifest(_)
                    | WorkspaceStatEntryError::Internal(_) => reply.manual().error(libc::EIO),
                },
            }
        });
    }

    fn mkdir(
        &mut self,
        req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        _mode: u32,
        _umask: u32,
        reply: fuser::ReplyEntry,
    ) {
        debug!("[FUSE] mkdir(parent: {:#x?}, name: {:?})", parent, name);
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEntry);

        let uid = req.uid();
        let gid = req.gid();
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(libc::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(libc::EINVAL);
                return;
            }
        };
        let ops = self.ops.clone();
        let inodes = self.inodes.clone();
        self.tokio_handle.spawn(async move {
            let path = {
                let inodes_guard = inodes.lock().expect("mutex is poisoned");
                let parent_path = inodes_guard.get_path_or_panic(parent);
                parent_path.join(name)
            };

            match ops.create_folder(path.clone()).await {
                Ok(entry_id) => {
                    let inode = {
                        let mut inodes_guard = inodes.lock().expect("mutex is poisoned");
                        inodes_guard.insert_path(path.clone())
                    };

                    reply_with_lookup(&ops, uid, gid, inode, entry_id, reply.manual()).await;
                }
                Err(err) => match err {
                    WorkspaceCreateFolderError::EntryExists { .. } => {
                        reply.manual().error(libc::EEXIST)
                    }
                    WorkspaceCreateFolderError::ParentIsFile => reply.manual().error(libc::ENOENT),
                    WorkspaceCreateFolderError::ParentNotFound => {
                        reply.manual().error(libc::ENOENT)
                    }
                    WorkspaceCreateFolderError::Offline => reply.manual().error(libc::EHOSTUNREACH),
                    WorkspaceCreateFolderError::NoRealmAccess => reply.manual().error(libc::EPERM),
                    WorkspaceCreateFolderError::ReadOnlyRealm => reply.manual().error(libc::EPERM),
                    WorkspaceCreateFolderError::Stopped
                    | WorkspaceCreateFolderError::InvalidKeysBundle(_)
                    | WorkspaceCreateFolderError::InvalidCertificate(_)
                    | WorkspaceCreateFolderError::InvalidManifest(_)
                    | WorkspaceCreateFolderError::Internal(_) => reply.manual().error(libc::EIO),
                },
            }
        });
    }

    fn rmdir(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEmpty,
    ) {
        debug!("[FUSE] rmdir(parent: {:#x?}, name: {:?})", parent, name);
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(libc::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(libc::EINVAL);
                return;
            }
        };
        let ops = self.ops.clone();
        let inodes = self.inodes.clone();
        self.tokio_handle.spawn(async move {
            let path = {
                let inodes_guard = inodes.lock().expect("mutex is poisoned");
                let parent_path = inodes_guard.get_path_or_panic(parent);
                parent_path.join(name)
            };

            match ops.remove_folder(path.clone()).await {
                Ok(()) => {
                    reply.manual().ok();
                }
                Err(err) => match err {
                    WorkspaceRemoveEntryError::EntryNotFound => reply.manual().error(libc::ENOENT),
                    WorkspaceRemoveEntryError::EntryIsFile => reply.manual().error(libc::ENOTDIR),
                    WorkspaceRemoveEntryError::EntryIsNonEmptyFolder => {
                        reply.manual().error(libc::ENOTEMPTY)
                    }
                    WorkspaceRemoveEntryError::Offline => reply.manual().error(libc::EHOSTUNREACH),
                    WorkspaceRemoveEntryError::CannotRemoveRoot => {
                        reply.manual().error(libc::EPERM)
                    }
                    WorkspaceRemoveEntryError::NoRealmAccess => reply.manual().error(libc::EPERM),
                    WorkspaceRemoveEntryError::ReadOnlyRealm => reply.manual().error(libc::EPERM),
                    WorkspaceRemoveEntryError::Stopped
                    | WorkspaceRemoveEntryError::InvalidKeysBundle(_)
                    | WorkspaceRemoveEntryError::InvalidCertificate(_)
                    | WorkspaceRemoveEntryError::InvalidManifest(_)
                    | WorkspaceRemoveEntryError::Internal(_) => reply.manual().error(libc::EIO),
                    // Never returned given we *are* removing a folder
                    WorkspaceRemoveEntryError::EntryIsFolder => unreachable!(),
                },
            }
        });
    }

    fn unlink(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEmpty,
    ) {
        debug!("[FUSE] unlink(parent: {:#x?}, name: {:?})", parent, name);
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(libc::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(libc::EINVAL);
                return;
            }
        };
        let ops = self.ops.clone();
        let inodes = self.inodes.clone();
        self.tokio_handle.spawn(async move {
            let path = {
                let inodes_guard = inodes.lock().expect("mutex is poisoned");
                let parent_path = inodes_guard.get_path_or_panic(parent);
                parent_path.join(name)
            };

            match ops.remove_file(path.clone()).await {
                Ok(()) => {
                    reply.manual().ok();
                }
                Err(err) => match err {
                    WorkspaceRemoveEntryError::EntryNotFound => reply.manual().error(libc::ENOENT),
                    WorkspaceRemoveEntryError::EntryIsFolder => reply.manual().error(libc::EISDIR),
                    WorkspaceRemoveEntryError::EntryIsNonEmptyFolder => {
                        reply.manual().error(libc::ENOTEMPTY)
                    }
                    WorkspaceRemoveEntryError::Offline => reply.manual().error(libc::EHOSTUNREACH),
                    WorkspaceRemoveEntryError::CannotRemoveRoot => {
                        reply.manual().error(libc::EPERM)
                    }
                    WorkspaceRemoveEntryError::NoRealmAccess => reply.manual().error(libc::EPERM),
                    WorkspaceRemoveEntryError::ReadOnlyRealm => reply.manual().error(libc::EPERM),
                    WorkspaceRemoveEntryError::Stopped
                    | WorkspaceRemoveEntryError::InvalidKeysBundle(_)
                    | WorkspaceRemoveEntryError::InvalidCertificate(_)
                    | WorkspaceRemoveEntryError::InvalidManifest(_)
                    | WorkspaceRemoveEntryError::Internal(_) => reply.manual().error(libc::EIO),
                    // Never returned given we *are* removing a file
                    WorkspaceRemoveEntryError::EntryIsFile => unreachable!(),
                },
            }
        });
    }

    fn rename(
        &mut self,
        _req: &fuser::Request<'_>,
        src_parent: u64,
        src_name: &std::ffi::OsStr,
        dst_parent: u64,
        dst_name: &std::ffi::OsStr,
        flags: u32,
        reply: fuser::ReplyEmpty,
    ) {
        debug!(
            "[FUSE] rename(src_parent: {:#x?}, src_name: {:?}, dst_parent: {:#x?}, \\
            std_name: {:?}, flags: {})",
            src_parent, src_name, dst_parent, dst_name, flags,
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let src_name = match os_name_to_entry_name(src_name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(libc::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(libc::EINVAL);
                return;
            }
        };
        let dst_name = match os_name_to_entry_name(dst_name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(libc::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(libc::EINVAL);
                return;
            }
        };

        let ops = self.ops.clone();
        let inodes = self.inodes.clone();
        self.tokio_handle.spawn(async move {
            let src_path = {
                let inodes_guard = inodes.lock().expect("mutex is poisoned");
                let parent_path = inodes_guard.get_path_or_panic(src_parent);
                parent_path.join(src_name)
            };
            let dst_path = {
                let inodes_guard = inodes.lock().expect("mutex is poisoned");
                let parent_path = inodes_guard.get_path_or_panic(dst_parent);
                parent_path.join(dst_name)
            };

            match ops
                .move_entry(src_path.clone(), dst_path.clone(), false)
                .await
            {
                Ok(()) => {
                    reply.manual().ok();
                }
                Err(err) => match err {
                    WorkspaceRenameEntryError::EntryNotFound => todo!(),
                    WorkspaceRenameEntryError::CannotRenameRoot => todo!(),
                    WorkspaceRenameEntryError::ReadOnlyRealm => todo!(),
                    WorkspaceRenameEntryError::NoRealmAccess => todo!(),
                    WorkspaceRenameEntryError::DestinationExists { .. } => todo!(),
                    // WorkspaceRenameEntryError::EntryNotFound => reply.manual().error(libc::ENOENT),
                    // WorkspaceRenameEntryError::EntryIsFolder => reply.manual().error(libc::EISDIR),
                    // WorkspaceRenameEntryError::EntryIsNonEmptyFolder => {
                    //     reply.manual().error(libc::ENOTEMPTY)
                    // }
                    WorkspaceRenameEntryError::Offline => reply.manual().error(libc::EHOSTUNREACH),
                    // WorkspaceRenameEntryError::CannotRemoveRoot => reply.manual().error(libc::EPERM),
                    // WorkspaceRenameEntryError::NoRealmAccess => reply.manual().error(libc::EPERM),
                    // WorkspaceRenameEntryError::ReadOnlyRealm => reply.manual().error(libc::EPERM),
                    WorkspaceRenameEntryError::Stopped
                    | WorkspaceRenameEntryError::InvalidKeysBundle(_)
                    | WorkspaceRenameEntryError::InvalidCertificate(_)
                    | WorkspaceRenameEntryError::InvalidManifest(_)
                    | WorkspaceRenameEntryError::Internal(_) => reply.manual().error(libc::EIO),
                    // // Never returned given we *are* removing a file
                    // WorkspaceRenameEntryError::EntryIsFile => unreachable!(),
                },
            }
        });
    }

    // Note flags doesn't contains O_CREAT, O_EXCL, O_NOCTTY and O_TRUNC
    fn open(&mut self, _req: &fuser::Request<'_>, ino: u64, flags: i32, reply: fuser::ReplyOpen) {
        debug!("[FUSE] open(ino: {:#x?}, flags: {:#x?})", ino, flags);
        let reply = reply_on_drop_guard!(reply, fuser::ReplyOpen);

        let path = {
            let inodes_guard = self.inodes.lock().expect("mutex is poisoned");
            inodes_guard.get_path_or_panic(ino)
        };

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let options = {
                let mut options = OpenOptions {
                    read: false,
                    write: false,
                    // append: flags & libc::O_APPEND != 0,
                    truncate: flags & libc::O_TRUNC != 0,
                    create: false,
                    create_new: false,
                };
                match flags & libc::O_ACCMODE {
                    libc::O_RDONLY => {
                        // Behavior is undefined, but most filesystems return EACCES
                        if flags & libc::O_TRUNC != 0 {
                            reply.manual().error(libc::EACCES);
                            return;
                        }
                        // TODO: Fuser's reference implementation mentions a `FMODE_EXEC` here
                        // (see: https://github.com/cberner/fuser/blob/be820a8080f229301028546e819b4997af26cf47/examples/simple.rs#L1336)
                        options.read = true;
                    }
                    libc::O_WRONLY => {
                        options.write = true;
                    }
                    libc::O_RDWR => {
                        options.read = true;
                        options.write = true;
                    }
                    // Exactly one access mode flag must be specified
                    _ => {
                        reply.manual().error(libc::EINVAL);
                        return;
                    }
                }
                options
            };

            let fd = match ops.open_file(path, options).await {
                Ok(fd) => fd,
                Err(err) => {
                    return match err {
                        WorkspaceOpenFileError::EntryNotFound => reply.manual().error(libc::ENOENT),
                        WorkspaceOpenFileError::Offline => reply.manual().error(libc::EHOSTUNREACH),
                        WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
                            reply.manual().error(libc::EEXIST)
                        }
                        WorkspaceOpenFileError::EntryNotAFile => reply.manual().error(libc::EISDIR),
                        WorkspaceOpenFileError::NoRealmAccess => reply.manual().error(libc::EPERM),
                        WorkspaceOpenFileError::ReadOnlyRealm => reply.manual().error(libc::EPERM),
                        WorkspaceOpenFileError::Stopped
                        | WorkspaceOpenFileError::InvalidKeysBundle(_)
                        | WorkspaceOpenFileError::InvalidCertificate(_)
                        | WorkspaceOpenFileError::InvalidManifest(_)
                        | WorkspaceOpenFileError::Internal(_) => reply.manual().error(libc::EIO),
                    }
                }
            };

            // Flag is only to enable FOPEN_DIRECT_IO which we don't use
            let open_flags = 0;
            reply.manual().opened(fd.0.into(), open_flags);
        });
    }

    // Note flags doesn't contains O_NOCTTY
    fn create(
        &mut self,
        req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        // Mode & umask are for the file creation, but we don't support them
        _mode: u32,
        _umask: u32,
        flags: i32,
        reply: fuser::ReplyCreate,
    ) {
        debug!(
            "[FUSE] create(parent: {:#x?}, name: {:?}, flags: {:#x?})",
            parent, name, flags
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyCreate);

        let uid = req.uid();
        let gid = req.gid();
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(libc::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(libc::EINVAL);
                return;
            }
        };

        let inodes = self.inodes.clone();
        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let parent_path = {
                let inodes_guard = inodes.lock().expect("mutex is poisoned");
                inodes_guard.get_path_or_panic(parent)
            };
            let path = parent_path.into_child(name);

            let options = {
                let mut options = OpenOptions {
                    read: false,
                    write: false,
                    // append: flags & libc::O_APPEND != 0,
                    truncate: flags & libc::O_TRUNC != 0,
                    // `OpenOptions::create_new` is a subset of `OpenOptions::create`,
                    // which correspond to O_EXCL implying O_CREAT (according to man open:
                    // "In general, the behavior of O_EXCL is undefined if it is used
                    // without O_CREAT")
                    create: flags & libc::O_CREAT != 0,
                    create_new: flags & libc::O_EXCL != 0,
                };
                match flags & libc::O_ACCMODE {
                    libc::O_RDONLY => {
                        // Behavior is undefined, but most filesystems return EACCES
                        if flags & libc::O_TRUNC != 0 {
                            reply.manual().error(libc::EACCES);
                            return;
                        }
                        // TODO: Fuser's reference implementation mentions a `FMODE_EXEC` here
                        // (see: https://github.com/cberner/fuser/blob/be820a8080f229301028546e819b4997af26cf47/examples/simple.rs#L1336)
                        options.read = true;
                    }
                    libc::O_WRONLY => {
                        options.write = true;
                    }
                    libc::O_RDWR => {
                        options.read = true;
                        options.write = true;
                    }
                    // Exactly one access mode flag must be specified
                    _ => {
                        reply.manual().error(libc::EINVAL);
                        return;
                    }
                }
                options
            };

            let fd = match ops.open_file(path.clone(), options).await {
                Ok(fd) => fd,
                Err(err) => {
                    return match err {
                        WorkspaceOpenFileError::EntryNotFound => reply.manual().error(libc::ENOENT),
                        WorkspaceOpenFileError::Offline => reply.manual().error(libc::EHOSTUNREACH),
                        WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
                            reply.manual().error(libc::EEXIST)
                        }
                        WorkspaceOpenFileError::EntryNotAFile => reply.manual().error(libc::EISDIR),
                        WorkspaceOpenFileError::NoRealmAccess => reply.manual().error(libc::EPERM),
                        WorkspaceOpenFileError::ReadOnlyRealm => reply.manual().error(libc::EPERM),
                        WorkspaceOpenFileError::Stopped
                        | WorkspaceOpenFileError::InvalidKeysBundle(_)
                        | WorkspaceOpenFileError::InvalidCertificate(_)
                        | WorkspaceOpenFileError::InvalidManifest(_)
                        | WorkspaceOpenFileError::Internal(_) => reply.manual().error(libc::EIO),
                    }
                }
            };

            // Flag is only to enable FOPEN_DIRECT_IO which we don't use
            let open_flags = 0;

            let inode = {
                let mut inodes_guard = inodes.lock().expect("mutex is poisoned");
                inodes_guard.insert_path(path.clone())
            };

            let stat = match ops.fd_stat(fd).await {
                Ok(stat) => stat,
                Err(err) => {
                    return match err {
                        // Unexpected: We have just created this file descriptor !
                        WorkspaceFdStatError::BadFileDescriptor => reply.manual().error(libc::EIO),
                    };
                }
            };

            reply.manual().created(
                &TTL,
                &file_stat_to_file_attr(stat, inode, uid, gid),
                GENERATION,
                fd.0.into(),
                open_flags,
            );
        });
    }

    // Set file attributes, mostly use for truncating files
    fn setattr(
        &mut self,
        req: &fuser::Request<'_>,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        _atime: Option<fuser::TimeOrNow>,
        _mtime: Option<fuser::TimeOrNow>,
        _ctime: Option<std::time::SystemTime>,
        fh: Option<u64>,
        _crtime: Option<std::time::SystemTime>,
        _chgtime: Option<std::time::SystemTime>,
        _bkuptime: Option<std::time::SystemTime>,
        flags: Option<u32>,
        reply: fuser::ReplyAttr,
    ) {
        debug!(
            "[FUSE] setattr(ino: {:#x?}, mode: {:?}, uid: {:?}, \
            gid: {:?}, size: {:?}, fh: {:?}, flags: {:?})",
            ino, mode, uid, gid, size, fh, flags
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyAttr);
        let uid = req.uid();
        let gid = req.gid();

        if let Some(size) = size {
            // Truncate file by file descriptor

            if let Some(fh) = fh {
                let ops = self.ops.clone();
                self.tokio_handle.spawn(async move {
                    let fd = FileDescriptor(fh as u32);
                    match ops.fd_resize(fd, size, false).await {
                        Ok(()) => (),
                        Err(err) => {
                            return match err {
                                WorkspaceFdResizeError::NotInWriteMode => {
                                    reply.manual().error(libc::EBADF)
                                }
                                // Unexpected: FUSE is supposed to only give us valid file descriptors !
                                WorkspaceFdResizeError::BadFileDescriptor
                                | WorkspaceFdResizeError::Internal(_) => {
                                    reply.manual().error(libc::EIO)
                                }
                            };
                        }
                    }

                    let stat = match ops.fd_stat(fd).await {
                        Ok(stat) => stat,
                        Err(err) => {
                            return match err {
                                // Unexpected: FUSE is supposed to only give us valid file descriptors !
                                WorkspaceFdStatError::BadFileDescriptor => {
                                    reply.manual().error(libc::EIO)
                                }
                            };
                        }
                    };

                    reply
                        .manual()
                        .attr(&TTL, &file_stat_to_file_attr(stat, ino, uid, gid));
                });

                return;
            } else {
                // Truncate file by path

                let path = {
                    let inodes_guard = self.inodes.lock().expect("mutex is poisoned");
                    inodes_guard.get_path_or_panic(ino)
                };

                let ops = self.ops.clone();
                self.tokio_handle.spawn(async move {
                    let options = OpenOptions {
                        read: false,
                        write: true,
                        truncate: true,
                        create: false,
                        create_new: false,
                    };
                    let fd = match ops.open_file(path.clone(), options).await {
                        Ok(fd) => fd,
                        Err(err) => {
                            return match err {
                                WorkspaceOpenFileError::EntryNotFound => {
                                    reply.manual().error(libc::ENOENT)
                                }
                                WorkspaceOpenFileError::Offline => {
                                    reply.manual().error(libc::EHOSTUNREACH)
                                }
                                WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
                                    reply.manual().error(libc::EEXIST)
                                }
                                WorkspaceOpenFileError::EntryNotAFile => {
                                    reply.manual().error(libc::EISDIR)
                                }
                                WorkspaceOpenFileError::NoRealmAccess => {
                                    reply.manual().error(libc::EPERM)
                                }
                                WorkspaceOpenFileError::ReadOnlyRealm => {
                                    reply.manual().error(libc::EPERM)
                                }
                                WorkspaceOpenFileError::Stopped
                                | WorkspaceOpenFileError::InvalidKeysBundle(_)
                                | WorkspaceOpenFileError::InvalidCertificate(_)
                                | WorkspaceOpenFileError::InvalidManifest(_)
                                | WorkspaceOpenFileError::Internal(_) => {
                                    reply.manual().error(libc::EIO)
                                }
                            }
                        }
                    };

                    let stat = match ops.fd_stat(fd).await {
                        Ok(stat) => stat,
                        Err(err) => {
                            return match err {
                                // Unexpected: we have just opened the file !
                                WorkspaceFdStatError::BadFileDescriptor => {
                                    reply.manual().error(libc::EIO)
                                }
                            };
                        }
                    };

                    match ops.fd_close(fd).await {
                        Ok(()) => (),
                        Err(err) => {
                            return match err {
                            WorkspaceFdCloseError::Stopped
                            // Unexpected: we have just opened the file !
                            | WorkspaceFdCloseError::BadFileDescriptor
                            | WorkspaceFdCloseError::Internal(_)
                            => reply.manual().error(libc::EIO),
                        };
                        }
                    }

                    reply
                        .manual()
                        .attr(&TTL, &file_stat_to_file_attr(stat, ino, uid, gid));
                });

                return;
            }
        }

        // Other changes are not supported

        // TODO: support atime/utime change ?

        reply.manual().error(libc::ENOSYS);
    }

    fn read(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        debug!(
            "[FUSE] read(ino: {:#x?}, fh: {}, offset: {}, size: {})",
            ino, fh, offset, size,
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyData);

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let fd = FileDescriptor(fh as u32);
            let mut buf = Vec::with_capacity(size as usize);
            // TODO: investigate if offset can be negative or if this is just poor typing on FUSE's part
            let offset = u64::try_from(offset).expect("Offset is negative");
            match ops.fd_read(fd, offset, size as u64, &mut buf).await {
                Ok(_) => {
                    reply.manual().data(&buf);
                }
                Err(err) => match err {
                    libparsec_client::workspace::WorkspaceFdReadError::Offline => reply.manual().error(libc::EHOSTUNREACH),
                    libparsec_client::workspace::WorkspaceFdReadError::NotInReadMode => reply.manual().error(libc::EBADF),
                    libparsec_client::workspace::WorkspaceFdReadError::Stopped
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    | libparsec_client::workspace::WorkspaceFdReadError::BadFileDescriptor
                    | libparsec_client::workspace::WorkspaceFdReadError::Internal(_)
                    => reply.manual().error(libc::EIO),
                },
            }
        });
    }

    fn write(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyWrite,
    ) {
        debug!(
            "[FUSE] write(ino: {:#x?}, fh: {}, offset: {}, data.len(): {})",
            ino,
            fh,
            offset,
            data.len(),
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyWrite);
        let data = data.to_vec();

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let fd = FileDescriptor(fh as u32);
            // TODO: investigate if offset can be negative or if this is just poor typing on FUSE's part
            let offset = u64::try_from(offset).expect("Offset is negative");
            match ops.fd_write(fd, offset, &data).await {
                Ok(written) => {
                    reply.manual().written(written as u32);
                }
                Err(err) => match err {
                    WorkspaceFdWriteError::NotInWriteMode => reply.manual().error(libc::EBADF),
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    WorkspaceFdWriteError::BadFileDescriptor
                    | WorkspaceFdWriteError::Internal(_) => reply.manual().error(libc::EIO),
                },
            }
        });
    }

    fn release(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        flags: i32,
        lock_owner: Option<u64>,
        flush: bool,
        reply: fuser::ReplyEmpty,
    ) {
        debug!(
            "[FUSE] release(ino: {:#x?}, fh: {}, flags: {:#x?}, lock_owner: {:?}, flush: {})",
            ino, fh, flags, lock_owner, flush
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let fd = FileDescriptor(fh as u32);
            match ops.fd_close(fd).await {
                Ok(()) => {
                    reply.manual().ok();
                }
                Err(err) => match err {
                    libparsec_client::workspace::WorkspaceFdCloseError::Stopped
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    | libparsec_client::workspace::WorkspaceFdCloseError::BadFileDescriptor
                    | libparsec_client::workspace::WorkspaceFdCloseError::Internal(_)
                    => reply.manual().error(libc::EIO),
                },
            }
        });
    }

    fn fsync(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        datasync: bool,
        reply: fuser::ReplyEmpty,
    ) {
        debug!(
            "[FUSE] fsync(ino: {:#x?}, fh: {}, datasync: {})",
            ino, fh, datasync
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let fd = FileDescriptor(fh as u32);
            match ops.fd_flush(fd).await {
                Ok(()) => {
                    reply.manual().ok();
                }
                Err(err) => match err {
                    libparsec_client::workspace::WorkspaceFdFlushError::NotInWriteMode => reply.manual().error(libc::EACCES),
                    libparsec_client::workspace::WorkspaceFdFlushError::Stopped
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    | libparsec_client::workspace::WorkspaceFdFlushError::BadFileDescriptor
                    | libparsec_client::workspace::WorkspaceFdFlushError::Internal(_)
                    => reply.manual().error(libc::EIO),
                },
            }
        });
    }

    // Doing a `ls` is surprisingly more complex than it seems !
    // This is how it works in fuse:
    // - `opendir`
    // - `readdir` (used for blocks of 128 entries, and repeatedly until all entries are listed)
    // - `releasedir`
    //
    // The trick is the folder content may change between the `readdir` calls, in such
    // case the new entries (if any) may or may not be listed, however no already existing
    // entry should get listed twice or not at all.
    // To solve this, we first stat the parent folder `opendir` call (hence getting an
    // immutable list of children), then do the actual stat on each children in the `readdir`
    // calls.
    // For this we have to share a list of children between the `opendir` and `readdir` calls,
    // this is done by sharing a naked pointer that is eventually destroyed in `releasedir`.
    fn opendir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        flags: i32,
        reply: fuser::ReplyOpen,
    ) {
        debug!("[FUSE] opendir(ino: {:#x?}, flags: {:#x?})", ino, flags);
        let reply = reply_on_drop_guard!(reply, fuser::ReplyOpen);

        let path = {
            let inodes_guard = self.inodes.lock().expect("mutex is poisoned");
            inodes_guard.get_path_or_panic(ino)
        };

        let ops = self.ops.clone();
        let inodes = self.inodes.clone();
        self.tokio_handle.spawn(async move {
            let stat = match ops.stat_entry(&path).await {
                Ok(stat) => stat,
                Err(err) => {
                    return match err {
                        WorkspaceStatEntryError::EntryNotFound => {
                            reply.manual().error(libc::ENOENT)
                        }
                        WorkspaceStatEntryError::Offline => {
                            reply.manual().error(libc::EHOSTUNREACH)
                        }
                        WorkspaceStatEntryError::NoRealmAccess => reply.manual().error(libc::EPERM),
                        WorkspaceStatEntryError::Stopped
                        | WorkspaceStatEntryError::InvalidKeysBundle(_)
                        | WorkspaceStatEntryError::InvalidCertificate(_)
                        | WorkspaceStatEntryError::InvalidManifest(_)
                        | WorkspaceStatEntryError::Internal(_) => reply.manual().error(libc::EIO),
                    }
                }
            };

            match stat {
                EntryStat::File { .. } => {
                    reply.manual().error(libc::ENOTDIR);
                }
                EntryStat::Folder { children, .. } => {
                    let mut readdir_items = Vec::with_capacity(children.len());

                    {
                        let mut inodes_guard = inodes.lock().expect("mutex is poisoned");
                        for (child_name, child_id) in children {
                            let child_path = path.join(child_name.clone());
                            let child_inode = { inodes_guard.insert_path(child_path) };
                            readdir_items.push((child_name, child_inode, child_id));
                        }
                    }

                    let open_flags = 0; // TODO: what to set here ?

                    // Hold my beer
                    let boxed_readdir_items = Arc::new(readdir_items);
                    let fh = Arc::into_raw(boxed_readdir_items) as u64;

                    reply.manual().opened(fh, open_flags);
                }
            }
        });
    }

    fn readdir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: fuser::ReplyDirectory,
    ) {
        debug!(
            "[FUSE] readdir(ino: {:#x?}, fh: {}, offset: {})",
            ino, fh, offset
        );
        let mut reply = reply_on_drop_guard!(reply, fuser::ReplyDirectory);

        let boxed_readdir_items = {
            let ptr = fh as *mut Vec<(u64, EntryName, VlobID)>;
            // SAFETY: `ptr` is a valid pointer that have been created by `opendir`
            // We must increment the refcount given `Arc::from_raw` use in the next line
            // "steal" the owner ship of the pointer (and hence will release the data
            // if the counter is not incremented).
            unsafe { Arc::increment_strong_count(ptr) };
            // SAFETY: `ptr` is a valid pointer that have been created by `opendir`
            unsafe { Arc::from_raw(ptr) }
        };

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            for (index, (child_inode, child_name, child_id)) in
                boxed_readdir_items.iter().enumerate().skip(offset as usize)
            {
                let stat = match ops.stat_entry_by_id(*child_id).await {
                    Ok(stat) => stat,
                    Err(err) => {
                        return match err {
                            WorkspaceStatEntryError::EntryNotFound => {
                                reply.manual().error(libc::ENOENT)
                            }
                            WorkspaceStatEntryError::Offline => {
                                reply.manual().error(libc::EHOSTUNREACH)
                            }
                            WorkspaceStatEntryError::NoRealmAccess => {
                                reply.manual().error(libc::EPERM)
                            }
                            WorkspaceStatEntryError::Stopped
                            | WorkspaceStatEntryError::InvalidKeysBundle(_)
                            | WorkspaceStatEntryError::InvalidCertificate(_)
                            | WorkspaceStatEntryError::InvalidManifest(_)
                            | WorkspaceStatEntryError::Internal(_) => {
                                reply.manual().error(libc::EIO)
                            }
                        }
                    }
                };

                let buffer_full = match stat {
                    EntryStat::File { .. } => reply.borrow().add(
                        *child_inode,
                        (index + 1) as i64,
                        fuser::FileType::RegularFile,
                        OsStr::new(child_name.as_ref()),
                    ),
                    EntryStat::Folder { .. } => reply.borrow().add(
                        *child_inode,
                        (index + 1) as i64,
                        fuser::FileType::Directory,
                        OsStr::new(child_name.as_ref()),
                    ),
                };

                if buffer_full {
                    break;
                }
            }

            reply.manual().ok();
        });
    }

    fn releasedir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        flags: i32,
        reply: fuser::ReplyEmpty,
    ) {
        debug!(
            "[FUSE] releasedir(ino: {:#x?}, fh: {}, flags: {:#x?})",
            ino, fh, flags
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        {
            let ptr = fh as *mut Vec<(u64, EntryName, VlobID)>;
            // SAFETY: `ptr` is a valid pointer that have been created by `opendir`
            let _ = unsafe { Arc::from_raw(ptr) };
        }

        reply.manual().ok();
    }

    // TODO: Fuse exposes `readdirplus` to avoid roundtrip for stat calls.

    // TODO: Fuser exposes a `lseek` method to support SEEK_HOLE & SEEK_DATA.
    //       This is an optimisation for filesystem that don't store zero blocks
    //       (which Parsec does !).
    //       See https://lwn.net/Articles/440255/ for a longer explanation.
    //       This is available for FUSE >=7.24 which is fairly common on Linux,
    //       but this might not be the case for OSX.

    // TODO: Fuser exposes a `fallocate` method for FUSE >= 7.19. This would
    //       allow us to set the blocksize according to the expected final
    //       file size.

    // TODO: Fuser exposes a `copy_file_range` method for FUSE >= 7.28. This
    //       would speed up file copy a lot by reusing the same blocks !
}

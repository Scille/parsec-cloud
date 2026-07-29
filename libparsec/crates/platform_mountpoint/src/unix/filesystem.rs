// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    ffi::OsStr,
    fmt::Write,
    sync::{Arc, Mutex},
    time::Duration,
};

use libparsec_client::workspace::{
    EntryStat, FileStat, FolderReader, FolderReaderStatEntryError, FolderReaderStatNextOutcome,
    MoveEntryMode, OpenOptions, WorkspaceCreateFolderError, WorkspaceFdCloseError,
    WorkspaceFdFlushError, WorkspaceFdReadError, WorkspaceFdResizeError, WorkspaceFdStatError,
    WorkspaceFdWriteError, WorkspaceMoveEntryError, WorkspaceOpenFileError,
    WorkspaceOpenFolderReaderError, WorkspaceOps, WorkspaceRemoveEntryError,
    WorkspaceStatEntryError,
};
use libparsec_types::prelude::*;

use super::inode::{INodeNo, InodesManager};
use fuser::{Errno, Generation, InitFlags, Request};

/// Validity timeout set to zero to prevent FUSE from doing caching logic on it.
/// This is because FUSE caching is not aware of external modification (i.e. sync
/// on data modified remotely).
/// see: https://sourceforge.net/p/fuse/mailman/fuse-devel/thread/20140206092944.GA28534@frosties/
const TTL: std::time::Duration = std::time::Duration::ZERO;

/// TODO: Do we need to handle any other GENERATION ?
///
/// If the file system will be exported over NFS, the inode/generation pairs need
/// to be unique over the file system's lifetime (rather than just the mount
/// time). So if the file system reuses an inode after it has been deleted, it
/// must assign a new, previously unused generation number to the inode at the
/// same time.
const GENERATION: Generation = Generation(0);
/// The default block size, set to 512 KB.
const BLOCK_SIZE: u64 = 512 * 1024;
/// Read-write permissions for files and folders.
/// Equivalent to `chmod` flags `all=,u=rwx`.
const READ_WRITE_PERMISSIONS: u16 = 0o700;
/// Read-only permissions for files and folders.
/// Equivalent to `chmod` flags `all=,u=rx`.
const READ_ONLY_PERMISSIONS: u16 = 0o500;

fn os_name_to_entry_name(name: &OsStr) -> EntryNameResult<EntryName> {
    name.to_str()
        .map(|s| s.parse())
        .ok_or(EntryNameError::InvalidName)?
}

fn file_stat_to_file_attr(
    stat: FileStat,
    inode: INodeNo,
    uid: u32,
    gid: u32,
    is_read_only: bool,
) -> fuser::FileAttr {
    let created: std::time::SystemTime = stat.created.into();
    let updated: std::time::SystemTime = stat.updated.into();
    let perm = if is_read_only {
        READ_ONLY_PERMISSIONS
    } else {
        READ_WRITE_PERMISSIONS
    };
    fuser::FileAttr {
        ino: inode,
        size: stat.size,
        blocks: stat.size.div_ceil(BLOCK_SIZE),
        atime: updated,
        mtime: updated,
        ctime: updated,
        crtime: created,
        kind: fuser::FileType::RegularFile,
        perm,
        nlink: 1,
        uid,
        gid,
        rdev: 0,
        blksize: BLOCK_SIZE as u32,
        flags: 0,
    }
}

fn entry_stat_to_file_attr(
    stat: EntryStat,
    inode: INodeNo,
    uid: u32,
    gid: u32,
    is_read_only: bool,
) -> fuser::FileAttr {
    let perm = if is_read_only {
        READ_ONLY_PERMISSIONS
    } else {
        READ_WRITE_PERMISSIONS
    };
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
                blocks: size.div_ceil(BLOCK_SIZE),
                atime: updated,
                mtime: updated,
                ctime: updated,
                crtime: created,
                kind: fuser::FileType::RegularFile,
                perm,
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
                perm,
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
    is_read_only: bool,
}

impl Filesystem {
    pub fn new(
        ops: Arc<WorkspaceOps>,
        tokio_handle: tokio::runtime::Handle,
        is_read_only: bool,
    ) -> Self {
        Self {
            ops,
            tokio_handle,
            inodes: Arc::new(Mutex::new(InodesManager::new())),
            is_read_only,
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn reply_with_lookup(
    ops: &WorkspaceOps,
    uid: u32,
    gid: u32,
    inode: INodeNo,
    entry_id: VlobID,
    reply: fuser::ReplyEntry,
    is_read_only: bool,
    operation_name: &str,
) {
    // Confinement point information is unused here so ignore it
    match ops
        .stat_entry_by_id_ignore_confinement_point(entry_id)
        .await
    {
        Ok(stat) => reply.entry(
            &TTL,
            &entry_stat_to_file_attr(stat, inode, uid, gid, is_read_only),
            GENERATION,
        ),
        Err(err) => match err {
            WorkspaceStatEntryError::EntryNotFound => reply.error(Errno::ENOENT),
            WorkspaceStatEntryError::Offline(_) => reply.error(Errno::EHOSTUNREACH),
            WorkspaceStatEntryError::NoRealmAccess => reply.error(Errno::EPERM),
            WorkspaceStatEntryError::Stopped
            | WorkspaceStatEntryError::RealmDeleted
            | WorkspaceStatEntryError::InvalidKeysBundle(_)
            | WorkspaceStatEntryError::InvalidCertificate(_)
            | WorkspaceStatEntryError::InvalidManifest(_)
            | WorkspaceStatEntryError::Internal(_) => {
                log::warn!("FUSE `{operation_name}` operation failed: {err:?}");
                reply.error(Errno::EIO)
            }
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
                    Some(reply) => reply.error(Errno::EIO),
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

// See https://libfuse.github.io/doxygen/structfuse__operations.html for documentation
// of each of the methods implemented here.
impl fuser::Filesystem for Filesystem {
    fn init(
        &mut self,
        _req: &fuser::Request,
        // see https://libfuse.github.io/doxygen/structfuse__conn__info.html
        config: &mut fuser::KernelConfig,
    ) -> std::io::Result<()> {
        // See libfuse & Linux documentation for a description of the capabilities.
        // See:
        // - https://github.com/torvalds/linux/blob/e32cde8d2bd7d251a8f9b434143977ddf13dcec6/include/uapi/linux/fuse.h#L376-L428
        // - https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L158

        // Here the general idea is to follow libfuse3's default configuration on what
        //    should be enabled.
        // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/lib/fuse_lowlevel.c#L2087-L2108

        // Note Fuser already set the following capabilities:
        // - On Linux: FUSE_ASYNC_READ | FUSE_BIG_WRITES | FUSE_MAX_PAGES
        // - On macOS: FUSE_ASYNC_READ | FUSE_CASE_INSENSITIVE | FUSE_VOL_RENAME | FUSE_XTIMES | FUSE_MAX_PAGES

        // `FUSE_POSIX_LOCKS` (remote locking support) is not enabled since we don't
        // implement `getlk()` & `setlk()` handlers.
        // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L179

        // `FUSE_FLOCK_LOCKS` is not enabled since we don't implement `flock()` handler.
        // Instead we rely on in-kernel POSIX lock emulation.
        // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L246

        // `FUSE_ASYNC_DIO` is not enabled since we don't do direct I/O.
        // See:
        // - https://www.kernel.org/doc/html/latest/filesystems/fuse-io.html
        // - https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L322

        let caps =

            // Do not send separate SETATTR request before open(O_TRUNC).
            // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L188
        InitFlags::FUSE_ATOMIC_O_TRUNC
        ;
        let caps = caps
            | cfg_select! {
                target_os = "macos" => { InitFlags::empty() },
                _ => {
                // Capabilities not supported by MacFuse 4.8
                // Support IOCTL on directories.
                // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L253
                InitFlags::FUSE_HAS_IOCTL_DIR |
                // Tell FUSE the data may change without going through the filesystem, hence attributes
                // validity should be checked on each access (leading to `getattr()` on timeout).
                // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L275
                InitFlags::FUSE_AUTO_INVAL_DATA |
                // Enable `readdirplus` support (readdir + lookup in one a single go).
                // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L283
                InitFlags::FUSE_DO_READDIRPLUS |
                // Adaptative readdirplus optimization support: still use readdir when lookup is
                // has already been done by the previous readdirplus.
                // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L311
                InitFlags::FUSE_READDIRPLUS_AUTO |
                // Allow parallel lookups and readdir on any given folder (default is serialized).
                // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L354
                InitFlags::FUSE_PARALLEL_DIROPS |

                // Fuser doesn't provided splice config on macOS
                // TODO: `FUSE_SPLICE_READ` seems to require `write_buf()` to be implemented, which is
                //       not even exposed in fuser !
                // Allow splice operations (see `man 2 splice`, basically kernel space page-based operations)
                // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L216
                InitFlags::FUSE_SPLICE_WRITE |
                InitFlags::FUSE_SPLICE_MOVE |
                InitFlags::FUSE_SPLICE_READ
                }
            };

        config.add_capabilities(caps).map_err(|e| {
            let mut s = "cannot add following fuse capabilities: ".to_owned();
            let mut it_names = e.iter_names();
            it_names
                .try_for_each(|(name, v)| write!(s, "{name} ({v:#x})"))
                .and_then(|_| {
                    let remaining_bits = it_names.remaining().bits();
                    if remaining_bits != 0 {
                        write!(s, ", remaining bits: {remaining_bits:#x}")
                    } else {
                        Ok(())
                    }
                })
                .expect("Cannot generator error message");
            std::io::Error::new(std::io::ErrorKind::Unsupported, s)
        })?;

        // Inform the Kernel about the granularity of the timestamps supported by our filesystem.
        // See https://github.com/libfuse/libfuse/blob/2aeef499b84b596608181f9b48d589c4f8ffe24a/include/fuse_common.h#L609-L622
        config
            .set_time_granularity(Duration::from_micros(1))
            .expect("Valid granularity");

        log::info!("Initializing filesystem for realm: {}", self.ops.realm_id());
        Ok(())
    }

    // TODO: Make sure opened files are automatically closed on umount, otherwise
    //       we would have some cleanup job do to here !
    fn destroy(&mut self) {
        log::info!("Destroying filesystem for realm: {}", self.ops.realm_id());
    }

    /// `lookup` is called every time Fuse meets a new resource it doesn't know about.
    /// The lookup transforms the path name to an `inode`.
    /// The `inode` is then used by all other operations and is freed by `forget`.
    fn lookup(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        log::debug!("[FUSE] lookup(parent: {parent:#x?}, name: {name:?})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEntry);

        let uid = req.uid();
        let gid = req.gid();
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(Errno::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(Errno::EINVAL);
                return;
            }
        };
        let ops = self.ops.clone();
        let inodes = self.inodes.clone();
        let is_read_only = self.is_read_only;
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
                        &entry_stat_to_file_attr(stat, inode, uid, gid, is_read_only),
                        GENERATION,
                    )
                }
                Err(err) => match err {
                    WorkspaceStatEntryError::EntryNotFound => reply.manual().error(Errno::ENOENT),
                    WorkspaceStatEntryError::Offline(_) => {
                        reply.manual().error(Errno::EHOSTUNREACH)
                    }
                    WorkspaceStatEntryError::NoRealmAccess => reply.manual().error(Errno::EPERM),
                    WorkspaceStatEntryError::Stopped
                    | WorkspaceStatEntryError::RealmDeleted
                    | WorkspaceStatEntryError::InvalidKeysBundle(_)
                    | WorkspaceStatEntryError::InvalidCertificate(_)
                    | WorkspaceStatEntryError::InvalidManifest(_)
                    | WorkspaceStatEntryError::Internal(_) => {
                        log::warn!("FUSE `lookup` operation cannot complete: {err:?}");
                        reply.manual().error(Errno::EIO)
                    }
                },
            }
        });
    }

    fn forget(&self, _req: &Request, ino: INodeNo, nlookup: u64) {
        self.inodes
            .lock()
            .expect("mutex is poisoned")
            .remove_path_or_panic(ino, nlookup);
    }

    fn statfs(&self, _req: &Request, ino: INodeNo, reply: fuser::ReplyStatfs) {
        log::debug!("[FUSE] statfs(ino: {ino:#x?})");

        // We have currently no way of easily getting the size of workspace
        // Also, the total size of a workspace is not limited
        // For the moment let's settle on 0 MB used for 1 TB available
        reply.statfs(
            2 * 1024u64.pow(2), // 2 MBlocks is 1 TB
            2 * 1024u64.pow(2), // 2 MBlocks is 1 TB
            2 * 1024u64.pow(2), // 2 MBlocks is 1 TB
            0,                  // Number of files
            0,                  // Number of remaining files available
            BLOCK_SIZE as u32,  // Size of a file block
            255,                // 255 bytes as maximum length for filenames
            // Fragment size (frsize) is set to the same size of block size
            // (bsize) since we do not handle elements smaller than it.
            BLOCK_SIZE as u32,
        )
    }

    fn getattr(
        &self,
        req: &Request,
        ino: INodeNo,
        _fh: Option<fuser::FileHandle>,
        reply: fuser::ReplyAttr,
    ) {
        log::debug!("[FUSE] getattr(ino: {ino:#x?})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyAttr);

        let uid = req.uid();
        let gid = req.gid();
        let path = self
            .inodes
            .lock()
            .expect("mutex is poisoned")
            .get_path_or_panic(ino);
        let ops = self.ops.clone();
        let is_read_only = self.is_read_only;
        self.tokio_handle.spawn(async move {
            let res = getattr_from_path(&ops, path, ino, uid, gid, is_read_only).await;

            match res {
                Ok(stat) => reply.manual().attr(&TTL, &stat),
                Err(errno) => reply.manual().error(errno),
            }
        });
    }

    fn mkdir(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        _mode: u32,
        _umask: u32,
        reply: fuser::ReplyEntry,
    ) {
        log::debug!("[FUSE] mkdir(parent: {parent:#x?}, name: {name:?})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEntry);

        let uid = req.uid();
        let gid = req.gid();
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(Errno::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(Errno::EINVAL);
                return;
            }
        };
        let ops = self.ops.clone();
        let inodes = self.inodes.clone();
        let is_read_only = self.is_read_only;
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

                    reply_with_lookup(
                        &ops,
                        uid,
                        gid,
                        inode,
                        entry_id,
                        reply.manual(),
                        is_read_only,
                        "mkdir",
                    )
                    .await;
                }
                Err(err) => match err {
                    WorkspaceCreateFolderError::EntryExists { .. } => {
                        reply.manual().error(Errno::EEXIST)
                    }
                    WorkspaceCreateFolderError::ParentNotAFolder => {
                        reply.manual().error(Errno::ENOENT)
                    }
                    WorkspaceCreateFolderError::ParentNotFound => {
                        reply.manual().error(Errno::ENOENT)
                    }
                    WorkspaceCreateFolderError::Offline(_) => {
                        reply.manual().error(Errno::EHOSTUNREACH)
                    }
                    WorkspaceCreateFolderError::NoRealmAccess => reply.manual().error(Errno::EPERM),
                    WorkspaceCreateFolderError::ReadOnlyRealm => reply.manual().error(Errno::EROFS),
                    WorkspaceCreateFolderError::Stopped
                    | WorkspaceCreateFolderError::RealmDeleted
                    | WorkspaceCreateFolderError::InvalidKeysBundle(_)
                    | WorkspaceCreateFolderError::InvalidCertificate(_)
                    | WorkspaceCreateFolderError::InvalidManifest(_)
                    | WorkspaceCreateFolderError::Internal(_) => {
                        log::warn!("FUSE `mkdir` operation cannot complete: {err:?}");
                        reply.manual().error(Errno::EIO)
                    }
                },
            }
        });
    }

    fn rmdir(
        &self,
        _req: &Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEmpty,
    ) {
        log::debug!("[FUSE] rmdir(parent: {parent:#x?}, name: {name:?})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(Errno::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(Errno::EINVAL);
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
                    WorkspaceRemoveEntryError::EntryNotFound => reply.manual().error(Errno::ENOENT),
                    WorkspaceRemoveEntryError::EntryIsFile => reply.manual().error(Errno::ENOTDIR),
                    WorkspaceRemoveEntryError::EntryIsNonEmptyFolder => {
                        reply.manual().error(Errno::ENOTEMPTY)
                    }
                    WorkspaceRemoveEntryError::Offline(_) => {
                        reply.manual().error(Errno::EHOSTUNREACH)
                    }
                    WorkspaceRemoveEntryError::CannotRemoveRoot => {
                        reply.manual().error(Errno::EPERM)
                    }
                    WorkspaceRemoveEntryError::NoRealmAccess => reply.manual().error(Errno::EPERM),
                    WorkspaceRemoveEntryError::ReadOnlyRealm => reply.manual().error(Errno::EROFS),
                    WorkspaceRemoveEntryError::Stopped
                    | WorkspaceRemoveEntryError::RealmDeleted
                    | WorkspaceRemoveEntryError::InvalidKeysBundle(_)
                    | WorkspaceRemoveEntryError::InvalidCertificate(_)
                    | WorkspaceRemoveEntryError::InvalidManifest(_)
                    | WorkspaceRemoveEntryError::Internal(_) => {
                        log::warn!("FUSE `rmdir` operation cannot complete: {err:?}");
                        reply.manual().error(Errno::EIO)
                    }
                    // Never returned given we *are* removing a folder
                    WorkspaceRemoveEntryError::EntryIsFolder => unreachable!(),
                },
            }
        });
    }

    fn unlink(
        &self,
        _req: &Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEmpty,
    ) {
        log::debug!("[FUSE] unlink(parent: {parent:#x?}, name: {name:?})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(Errno::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(Errno::EINVAL);
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
                    WorkspaceRemoveEntryError::EntryNotFound => reply.manual().error(Errno::ENOENT),
                    WorkspaceRemoveEntryError::EntryIsFolder => reply.manual().error(Errno::EISDIR),
                    WorkspaceRemoveEntryError::EntryIsNonEmptyFolder => {
                        reply.manual().error(Errno::ENOTEMPTY)
                    }
                    WorkspaceRemoveEntryError::Offline(_) => {
                        reply.manual().error(Errno::EHOSTUNREACH)
                    }
                    WorkspaceRemoveEntryError::CannotRemoveRoot => {
                        reply.manual().error(Errno::EPERM)
                    }
                    WorkspaceRemoveEntryError::NoRealmAccess => reply.manual().error(Errno::EPERM),
                    WorkspaceRemoveEntryError::ReadOnlyRealm => reply.manual().error(Errno::EROFS),
                    WorkspaceRemoveEntryError::Stopped
                    | WorkspaceRemoveEntryError::RealmDeleted
                    | WorkspaceRemoveEntryError::InvalidKeysBundle(_)
                    | WorkspaceRemoveEntryError::InvalidCertificate(_)
                    | WorkspaceRemoveEntryError::InvalidManifest(_)
                    | WorkspaceRemoveEntryError::Internal(_) => {
                        log::warn!("FUSE `unlink` operation cannot complete: {err:?}");
                        reply.manual().error(Errno::EIO)
                    }
                    // Never returned given we *are* removing a file
                    WorkspaceRemoveEntryError::EntryIsFile => unreachable!(),
                },
            }
        });
    }

    fn rename(
        &self,
        _req: &Request,
        src_parent: INodeNo,
        src_name: &std::ffi::OsStr,
        dst_parent: INodeNo,
        dst_name: &std::ffi::OsStr,
        flags: fuser::RenameFlags,
        reply: fuser::ReplyEmpty,
    ) {
        log::debug!(
            "[FUSE] rename(src_parent: {src_parent:#x?}, src_name: {src_name:?}, dst_parent: {dst_parent:#x?}, \\
            std_name: {dst_name:?}, flags: {flags})",
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        // Flags only contain RENAME_EXCHANGE or RENAME_NOREPLACE
        // (see https://libfuse.github.io/doxygen/structfuse__operations.html#adc484e37f216a8a18b97e01a83c6a6a2)
        #[cfg(target_os = "macos")]
        let mode = MoveEntryMode::CanReplace;
        #[cfg(not(target_os = "macos"))]
        let mode = if flags.contains(fuser::RenameFlags::RENAME_NOREPLACE) {
            MoveEntryMode::NoReplace
        } else if flags.contains(fuser::RenameFlags::RENAME_EXCHANGE) {
            MoveEntryMode::Exchange
        } else {
            MoveEntryMode::CanReplace
        };

        let src_name = match os_name_to_entry_name(src_name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(Errno::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(Errno::EINVAL);
                return;
            }
        };
        let dst_name = match os_name_to_entry_name(dst_name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(Errno::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(Errno::EINVAL);
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
                .move_entry(src_path.clone(), dst_path.clone(), mode)
                .await
            {
                Ok(()) => {
                    reply.manual().ok();
                }
                Err(err) => match err {
                    WorkspaceMoveEntryError::SourceNotFound
                    | WorkspaceMoveEntryError::DestinationNotFound => {
                        reply.manual().error(Errno::ENOENT)
                    }
                    WorkspaceMoveEntryError::DestinationExists { .. } => {
                        reply.manual().error(Errno::EEXIST)
                    }
                    WorkspaceMoveEntryError::CannotMoveRoot => reply.manual().error(Errno::EPERM),
                    WorkspaceMoveEntryError::Offline(_) => {
                        reply.manual().error(Errno::EHOSTUNREACH)
                    }
                    WorkspaceMoveEntryError::NoRealmAccess => reply.manual().error(Errno::EPERM),
                    WorkspaceMoveEntryError::ReadOnlyRealm => reply.manual().error(Errno::EROFS),
                    WorkspaceMoveEntryError::Stopped
                    | WorkspaceMoveEntryError::RealmDeleted
                    | WorkspaceMoveEntryError::InvalidKeysBundle(_)
                    | WorkspaceMoveEntryError::InvalidCertificate(_)
                    | WorkspaceMoveEntryError::InvalidManifest(_)
                    | WorkspaceMoveEntryError::Internal(_) => {
                        log::warn!("FUSE `rename` operation cannot complete: {err:?}");
                        reply.manual().error(Errno::EIO)
                    }
                },
            }
        });
    }

    // Note flags doesn't contains O_CREAT, O_EXCL, O_NOCTTY and O_TRUNC
    fn open(&self, _req: &Request, ino: INodeNo, flags: fuser::OpenFlags, reply: fuser::ReplyOpen) {
        log::debug!("[FUSE] open(ino: {ino:#x?}, flags: {flags:#x?})");
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
                    truncate: flags.0 & libc::O_TRUNC != 0,
                    create: false,
                    create_new: false,
                };
                match flags.acc_mode() {
                    fuser::OpenAccMode::O_RDONLY => {
                        // Behavior is undefined, but most filesystems return EACCES
                        if options.truncate {
                            reply.manual().error(Errno::EACCES);
                            return;
                        }
                        // TODO: Fuser's reference implementation mentions a `FMODE_EXEC` here
                        // (see: https://github.com/cberner/fuser/blob/be820a8080f229301028546e819b4997af26cf47/examples/simple.rs#L1336)
                        options.read = true;
                    }
                    fuser::OpenAccMode::O_WRONLY => {
                        options.write = true;
                    }
                    fuser::OpenAccMode::O_RDWR => {
                        options.read = true;
                        options.write = true;
                    }
                }
                options
            };

            let fd = match ops.open_file(path, options).await {
                Ok(fd) => fd,
                Err(err) => {
                    return match err {
                        WorkspaceOpenFileError::EntryNotFound => {
                            reply.manual().error(Errno::ENOENT)
                        }
                        WorkspaceOpenFileError::Offline(_) => {
                            reply.manual().error(Errno::EHOSTUNREACH)
                        }
                        WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
                            reply.manual().error(Errno::EEXIST)
                        }
                        WorkspaceOpenFileError::EntryNotAFile { .. } => {
                            reply.manual().error(Errno::EISDIR)
                        }
                        WorkspaceOpenFileError::NoRealmAccess => reply.manual().error(Errno::EPERM),
                        WorkspaceOpenFileError::ReadOnlyRealm => reply.manual().error(Errno::EROFS),
                        WorkspaceOpenFileError::Stopped
                        | WorkspaceOpenFileError::RealmDeleted
                        | WorkspaceOpenFileError::InvalidKeysBundle(_)
                        | WorkspaceOpenFileError::InvalidCertificate(_)
                        | WorkspaceOpenFileError::InvalidManifest(_)
                        | WorkspaceOpenFileError::Internal(_) => {
                            log::warn!("FUSE `open` operation cannot complete: {err:?}");
                            reply.manual().error(Errno::EIO)
                        }
                    }
                }
            };

            // Flag is only to enable FOPEN_DIRECT_IO which we don't use
            // TODO: Investigate if we could use FOPEN_PARALLEL_DIRECT_WRITES.
            let open_flags = fuser::FopenFlags::empty();
            reply.manual().opened(fuser::FileHandle(fd.0), open_flags);
        });
    }

    // Note flags doesn't contains O_NOCTTY
    fn create(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        // Mode & umask are for the file creation, but we don't support them
        _mode: u32,
        _umask: u32,
        flags: i32,
        reply: fuser::ReplyCreate,
    ) {
        log::debug!("[FUSE] create(parent: {parent:#x?}, name: {name:?}, flags: {flags:#x?})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyCreate);

        let uid = req.uid();
        let gid = req.gid();
        let name = match os_name_to_entry_name(name) {
            Ok(name) => name,
            Err(EntryNameError::NameTooLong) => {
                reply.manual().error(Errno::ENAMETOOLONG);
                return;
            }
            Err(EntryNameError::InvalidName) => {
                reply.manual().error(Errno::EINVAL);
                return;
            }
        };

        let inodes = self.inodes.clone();
        let ops = self.ops.clone();
        let is_read_only = self.is_read_only;
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
                            reply.manual().error(Errno::EACCES);
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
                        reply.manual().error(Errno::EINVAL);
                        return;
                    }
                }
                options
            };

            let fd = match ops.open_file(path.clone(), options).await {
                Ok(fd) => fd,
                Err(err) => {
                    return match err {
                        WorkspaceOpenFileError::EntryNotFound => {
                            reply.manual().error(Errno::ENOENT)
                        }
                        WorkspaceOpenFileError::Offline(_) => {
                            reply.manual().error(Errno::EHOSTUNREACH)
                        }
                        WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
                            reply.manual().error(Errno::EEXIST)
                        }
                        WorkspaceOpenFileError::EntryNotAFile { .. } => {
                            reply.manual().error(Errno::EISDIR)
                        }
                        WorkspaceOpenFileError::NoRealmAccess => reply.manual().error(Errno::EPERM),
                        WorkspaceOpenFileError::ReadOnlyRealm => reply.manual().error(Errno::EROFS),
                        WorkspaceOpenFileError::Stopped
                        | WorkspaceOpenFileError::RealmDeleted
                        | WorkspaceOpenFileError::InvalidKeysBundle(_)
                        | WorkspaceOpenFileError::InvalidCertificate(_)
                        | WorkspaceOpenFileError::InvalidManifest(_)
                        | WorkspaceOpenFileError::Internal(_) => {
                            log::warn!("FUSE `create` operation cannot complete: {err:?}");
                            reply.manual().error(Errno::EIO)
                        }
                    }
                }
            };

            // Flag is only to enable FOPEN_DIRECT_IO which we don't use
            let open_flags = fuser::FopenFlags::empty();

            let inode = {
                let mut inodes_guard = inodes.lock().expect("mutex is poisoned");
                inodes_guard.insert_path(path.clone())
            };

            let stat = match ops.fd_stat(fd).await {
                Ok(stat) => stat,
                Err(err) => {
                    return match err {
                        // Unexpected: We have just created this file descriptor !
                        WorkspaceFdStatError::BadFileDescriptor
                        | WorkspaceFdStatError::Internal(_) => reply.manual().error(Errno::EIO),
                    };
                }
            };

            reply.manual().created(
                &TTL,
                &file_stat_to_file_attr(stat, inode, uid, gid, is_read_only),
                GENERATION,
                fuser::FileHandle(fd.0),
                open_flags,
            );
        });
    }

    // Set file attributes, mostly use for truncating files
    fn setattr(
        &self,
        req: &Request,
        ino: INodeNo,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        _atime: Option<fuser::TimeOrNow>,
        _mtime: Option<fuser::TimeOrNow>,
        _ctime: Option<std::time::SystemTime>,
        fh: Option<fuser::FileHandle>,
        _crtime: Option<std::time::SystemTime>,
        _chgtime: Option<std::time::SystemTime>,
        _bkuptime: Option<std::time::SystemTime>,
        flags: Option<fuser::BsdFileFlags>,
        reply: fuser::ReplyAttr,
    ) {
        log::debug!(
            "[FUSE] setattr(ino: {ino:#x?}, mode: {mode:?}, uid: {uid:?}, \
            gid: {gid:?}, size: {size:?}, fh: {fh:?}, flags: {flags:?})"
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyAttr);
        let uid = req.uid();
        let gid = req.gid();

        if let Some(size) = size {
            // Truncate file by file descriptor

            if let Some(fh) = fh {
                let ops = self.ops.clone();
                let is_read_only = self.is_read_only;
                self.tokio_handle.spawn(async move {
                    let fd = FileDescriptor(fh.into());
                    match ops.fd_resize(fd, size, false).await {
                        Ok(()) => (),
                        Err(err) => {
                            return match err {
                                WorkspaceFdResizeError::NotInWriteMode => {
                                    reply.manual().error(Errno::EBADF)
                                }
                                // Unexpected: FUSE is supposed to only give us valid file descriptors !
                                WorkspaceFdResizeError::BadFileDescriptor
                                | WorkspaceFdResizeError::Internal(_) => {
                                    reply.manual().error(Errno::EIO)
                                }
                            };
                        }
                    }

                    let stat = match ops.fd_stat(fd).await {
                        Ok(stat) => stat,
                        Err(err) => {
                            return match err {
                                // Unexpected: FUSE is supposed to only give us valid file descriptors !
                                WorkspaceFdStatError::BadFileDescriptor
                                | WorkspaceFdStatError::Internal(_) => {
                                    reply.manual().error(Errno::EIO)
                                }
                            };
                        }
                    };

                    reply.manual().attr(
                        &TTL,
                        &file_stat_to_file_attr(stat, ino, uid, gid, is_read_only),
                    );
                });

                return;
            } else {
                // FIXME: Currently I'm only returning the file attributes without truncating the file
                // Truncate file by path

                let path = {
                    let inodes_guard = self.inodes.lock().expect("mutex is poisoned");
                    inodes_guard.get_path_or_panic(ino)
                };

                let ops = self.ops.clone();
                let is_read_only = self.is_read_only;
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
                                    reply.manual().error(Errno::ENOENT)
                                }
                                WorkspaceOpenFileError::Offline(_) => {
                                    reply.manual().error(Errno::EHOSTUNREACH)
                                }
                                WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
                                    reply.manual().error(Errno::EEXIST)
                                }
                                WorkspaceOpenFileError::EntryNotAFile { .. } => {
                                    reply.manual().error(Errno::EISDIR)
                                }
                                WorkspaceOpenFileError::NoRealmAccess => {
                                    reply.manual().error(Errno::EPERM)
                                }
                                WorkspaceOpenFileError::ReadOnlyRealm => {
                                    reply.manual().error(Errno::EROFS)
                                }
                                WorkspaceOpenFileError::Stopped
                                | WorkspaceOpenFileError::RealmDeleted
                                | WorkspaceOpenFileError::InvalidKeysBundle(_)
                                | WorkspaceOpenFileError::InvalidCertificate(_)
                                | WorkspaceOpenFileError::InvalidManifest(_)
                                | WorkspaceOpenFileError::Internal(_) => {
                                    log::warn!("FUSE `setattr` operation cannot complete: {err:?}");
                                    reply.manual().error(Errno::EIO)
                                }
                            }
                        }
                    };

                    let stat = match ops.fd_stat(fd).await {
                        Ok(stat) => stat,
                        Err(err) => {
                            return match err {
                                // Unexpected: we have just opened the file !
                                WorkspaceFdStatError::BadFileDescriptor
                                | WorkspaceFdStatError::Internal(_) => {
                                    log::warn!("FUSE `setattr` operation cannot complete: {err:?}");
                                    reply.manual().error(Errno::EIO)
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
                            => {
                                log::warn!("FUSE `setattr` operation cannot complete: {err:?}");
                                reply.manual().error(Errno::EIO)
                            }
                        };
                        }
                    }

                    reply.manual().attr(
                        &TTL,
                        &file_stat_to_file_attr(stat, ino, uid, gid, is_read_only),
                    );
                });

                return;
            }
        }

        // Other changes are not supported

        // TODO: support atime/utime change ?

        // Nothing to set, just return the file attr
        // Seems benign but it's important for `setattr` to return the file attributes even if
        // nothing changed.
        // Previously we where returning an error and that caused the issues #8976 and #8991

        let path = {
            let inodes_guard = self.inodes.lock().expect("Mutex is poisoned");
            inodes_guard.get_path_or_panic(ino)
        };

        let ops = self.ops.clone();
        let is_read_only = self.is_read_only;

        self.tokio_handle.spawn(async move {
            let res = getattr_from_path(&ops, path, ino, uid, gid, is_read_only).await;

            match res {
                Ok(attr) => reply.manual().attr(&TTL, &attr),
                Err(errno) => reply.manual().error(errno),
            }
        });
    }

    fn read(
        &self,
        _req: &Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        offset: u64,
        size: u32,
        _flags: fuser::OpenFlags,
        _lock_owner: Option<fuser::LockOwner>,
        reply: fuser::ReplyData,
    ) {
        log::debug!("[FUSE] read(ino: {ino:#x?}, fh: {fh}, offset: {offset}, size: {size})",);
        let reply = reply_on_drop_guard!(reply, fuser::ReplyData);

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let fd = FileDescriptor(fh.into());
            let mut buf = Vec::with_capacity(size as usize);
            match ops.fd_read(fd, offset, size as u64, &mut buf).await {
                Ok(_) => {
                    reply.manual().data(&buf);
                }
                Err(err) => match err {
                    WorkspaceFdReadError::Offline(_) => reply.manual().error(Errno::EHOSTUNREACH),
                    WorkspaceFdReadError::ServerBlockstoreUnavailable => reply.manual().error(Errno::EHOSTUNREACH),
                    WorkspaceFdReadError::NotInReadMode => reply.manual().error(Errno::EBADF),
                    WorkspaceFdReadError::NoRealmAccess => reply.manual().error(Errno::EPERM),
                    WorkspaceFdReadError::Stopped
                    | WorkspaceFdReadError::RealmDeleted
                    | WorkspaceFdReadError::InvalidBlockAccess(_)
                    | WorkspaceFdReadError::InvalidKeysBundle(_)
                    | WorkspaceFdReadError::InvalidCertificate(_)
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    | WorkspaceFdReadError::BadFileDescriptor
                    | WorkspaceFdReadError::Internal(_)
                    => {
                        log::warn!("FUSE `read` operation cannot complete: {err:?}");
                        reply.manual().error(Errno::EIO)
                    }
                },
            }
        });
    }

    fn write(
        &self,
        _req: &Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        offset: u64,
        data: &[u8],
        _write_flags: fuser::WriteFlags,
        _flags: fuser::OpenFlags,
        _lock_owner: Option<fuser::LockOwner>,
        reply: fuser::ReplyWrite,
    ) {
        log::debug!(
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
            let fd = FileDescriptor(fh.into());
            // TODO: investigate if offset can be negative or if this is just poor typing on FUSE's part
            match ops.fd_write(fd, offset, &data).await {
                Ok(written) => {
                    reply.manual().written(written as u32);
                }
                Err(err) => match err {
                    WorkspaceFdWriteError::NotInWriteMode => reply.manual().error(Errno::EBADF),
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    WorkspaceFdWriteError::BadFileDescriptor
                    | WorkspaceFdWriteError::Internal(_) => {
                        log::warn!("FUSE `write` operation cannot complete: {err:?}");
                        reply.manual().error(Errno::EIO)
                    }
                },
            }
        });
    }

    /// According to fuse documentation (https://libfuse.github.io/doxygen/structfuse__operations.html#a6bfecd61ddd58f74820953ee23b19ef3):
    ///
    /// > Flush is called on each close() of a file descriptor, as opposed to release which
    /// > is called on the close of the last file descriptor for a file.
    ///
    /// The key point here is `flush` is called as part of a `close()`, while `release` is
    /// called later on when the kernel module decides a given file descriptor is no longer
    /// referenced (as it could be present in multiple process due to forking).
    ///
    /// Hence it is vital to flush the file data here (hence the name !), otherwise reading
    /// the file right after closing a file descriptor that modified it might end up with
    /// outdated data !
    fn flush(
        &self,
        _req: &Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        lock_owner: fuser::LockOwner,
        reply: fuser::ReplyEmpty,
    ) {
        log::debug!("[FUSE] flush(ino: {ino:#x?}, fh: {fh}, lock_owner: {lock_owner:?})");

        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let fd = FileDescriptor(fh.into());
            match ops.fd_flush(fd).await {
                Ok(()) => {
                    reply.manual().ok();
                }
                Err(err) => match err {
                    WorkspaceFdFlushError::NotInWriteMode => {
                        reply.manual().ok();
                    }
                    WorkspaceFdFlushError::Stopped
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    | WorkspaceFdFlushError::BadFileDescriptor
                    | WorkspaceFdFlushError::Internal(_)
                    => {
                        log::warn!("FUSE `flush` operation cannot complete: {err:?}");
                        reply.manual().error(Errno::EIO)
                    }
                },
            }
        });
    }

    fn release(
        &self,
        _req: &Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        flags: fuser::OpenFlags,
        lock_owner: Option<fuser::LockOwner>,
        flush: bool,
        reply: fuser::ReplyEmpty,
    ) {
        log::debug!(
            "[FUSE] release(ino: {ino:#x?}, fh: {fh}, flags: {flags:#x?}, lock_owner: {lock_owner:?}, flush: {flush})"
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let fd = FileDescriptor(fh.into());
            match ops.fd_close(fd).await {
                Ok(()) => {
                    reply.manual().ok();
                }
                Err(err) => match err {
                    WorkspaceFdCloseError::Stopped
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    | WorkspaceFdCloseError::BadFileDescriptor
                    | WorkspaceFdCloseError::Internal(_)
                    => {
                        log::warn!("FUSE `release` operation cannot complete: {err:?}");
                        reply.manual().error(Errno::EIO)
                    }
                },
            }
        });
    }

    fn fsync(
        &self,
        _req: &Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        datasync: bool,
        reply: fuser::ReplyEmpty,
    ) {
        log::debug!("[FUSE] fsync(ino: {ino:#x?}, fh: {fh}, datasync: {datasync})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let fd = FileDescriptor(fh.into());
            match ops.fd_flush(fd).await {
                Ok(()) => {
                    reply.manual().ok();
                }
                Err(err) => match err {
                    WorkspaceFdFlushError::NotInWriteMode => reply.manual().error(Errno::EACCES),
                    WorkspaceFdFlushError::Stopped
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    | WorkspaceFdFlushError::BadFileDescriptor
                    | WorkspaceFdFlushError::Internal(_)
                    => {
                        log::warn!("FUSE `fsync` operation cannot complete: {err:?}");
                        reply.manual().error(Errno::EIO)
                    }
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
    // To solve this, the `opendir` operations do the path lookup and retrieve an immutable
    // list of children, then the `readdir` operations will stat each children in the list.
    // For this we have to share the list of children (i.e. the `FolderReader` object
    // returned by the workspace ops) between the `opendir` and `readdir` calls, this is
    // done by sharing a naked pointer that is eventually destroyed in `releasedir`.
    //
    // see https://unix.stackexchange.com/a/637666
    fn opendir(
        &self,
        _req: &Request,
        ino: INodeNo,
        flags: fuser::OpenFlags,
        reply: fuser::ReplyOpen,
    ) {
        log::debug!("[FUSE] opendir(ino: {ino:#x?}, flags: {flags:#x?})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyOpen);

        let path = {
            let inodes_guard = self.inodes.lock().expect("mutex is poisoned");
            inodes_guard.get_path_or_panic(ino)
        };

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let folder_reader = match ops.open_folder_reader(&path).await {
                Ok(folder_reader) => folder_reader,
                Err(err) => {
                    return match err {
                        WorkspaceOpenFolderReaderError::EntryNotFound => {
                            reply.manual().error(Errno::ENOENT)
                        }
                        WorkspaceOpenFolderReaderError::EntryIsFile => {
                            reply.manual().error(Errno::ENOTDIR);
                        }
                        WorkspaceOpenFolderReaderError::Offline(_) => {
                            reply.manual().error(Errno::EHOSTUNREACH)
                        }
                        WorkspaceOpenFolderReaderError::NoRealmAccess => {
                            reply.manual().error(Errno::EPERM)
                        }
                        WorkspaceOpenFolderReaderError::Stopped
                        | WorkspaceOpenFolderReaderError::RealmDeleted
                        | WorkspaceOpenFolderReaderError::InvalidKeysBundle(_)
                        | WorkspaceOpenFolderReaderError::InvalidCertificate(_)
                        | WorkspaceOpenFolderReaderError::InvalidManifest(_)
                        | WorkspaceOpenFolderReaderError::Internal(_) => {
                            log::warn!("FUSE `opendir` operation cannot complete: {err:?}");
                            reply.manual().error(Errno::EIO)
                        }
                    }
                }
            };

            // Flag is only to enable FOPEN_DIRECT_IO which we don't use
            let open_flags = fuser::FopenFlags::empty();

            // Hold my beer
            let boxed_path_and_folder_reader = Arc::new((path, folder_reader));
            let fh = Arc::into_raw(boxed_path_and_folder_reader) as u64;

            reply.manual().opened(fuser::FileHandle(fh), open_flags);
        });
    }

    fn readdirplus(
        &self,
        req: &Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        offset: u64,
        reply: fuser::ReplyDirectoryPlus,
    ) {
        log::debug!("[FUSE] readdirplus(ino: {ino:#x?}, fh: {fh}, offset: {offset})");
        let mut reply = reply_on_drop_guard!(reply, fuser::ReplyDirectoryPlus);

        let boxed_path_and_folder_reader = {
            let ptr = fh.0 as *mut (FsPath, FolderReader);
            // SAFETY: `ptr` is a valid pointer that have been created by `opendir`
            // We must increment the refcount given `Arc::from_raw` use in the next line
            // "steal" the owner ship of the pointer (and hence will release the data
            // if the counter is not incremented).
            unsafe { Arc::increment_strong_count(ptr) };
            // SAFETY: `ptr` is a valid pointer that have been created by `opendir`
            unsafe { Arc::from_raw(ptr) }
        };

        let uid = req.uid();
        let gid = req.gid();
        let ops = self.ops.clone();
        let inodes = self.inodes.clone();
        let is_read_only = self.is_read_only;
        self.tokio_handle.spawn(async move {
            let parent_path = &boxed_path_and_folder_reader.0;
            let folder_reader = &boxed_path_and_folder_reader.1;

            let mut offset = offset as usize;
            loop {
                let (child_name, child_stat) = match folder_reader.stat_child(&ops, offset).await {
                    Ok(FolderReaderStatNextOutcome::Entry { name, stat }) => (name, stat),
                    Ok(FolderReaderStatNextOutcome::NoMoreEntries) => break,
                    Ok(FolderReaderStatNextOutcome::InvalidChild) => {
                        // Current entry is invalid, just ignore it
                        offset += 1;
                        continue;
                    }
                    Err(err) => {
                        return match err {
                            FolderReaderStatEntryError::Offline(_) => {
                                reply.manual().error(Errno::EHOSTUNREACH)
                            }
                            FolderReaderStatEntryError::NoRealmAccess => {
                                reply.manual().error(Errno::EPERM)
                            }
                            FolderReaderStatEntryError::Stopped
                            | FolderReaderStatEntryError::RealmDeleted
                            | FolderReaderStatEntryError::InvalidKeysBundle(_)
                            | FolderReaderStatEntryError::InvalidCertificate(_)
                            | FolderReaderStatEntryError::InvalidManifest(_)
                            | FolderReaderStatEntryError::Internal(_) => {
                                log::warn!("FUSE `readdirplus` operation cannot complete: {err:?}");
                                reply.manual().error(Errno::EIO)
                            }
                        }
                    }
                };

                let child_inode = inodes
                    .lock()
                    .expect("mutex is poisoned")
                    .insert_path(parent_path.join(child_name.to_owned()));

                let buffer_full = match child_stat {
                    EntryStat::File { .. } => reply.borrow().add(
                        child_inode,
                        (offset + 1) as u64,
                        OsStr::new(child_name.as_ref()),
                        &TTL,
                        &entry_stat_to_file_attr(child_stat, child_inode, uid, gid, is_read_only),
                        GENERATION,
                    ),
                    EntryStat::Folder { .. } => reply.borrow().add(
                        child_inode,
                        (offset + 1) as u64,
                        OsStr::new(child_name.as_ref()),
                        &TTL,
                        &entry_stat_to_file_attr(child_stat, child_inode, uid, gid, is_read_only),
                        GENERATION,
                    ),
                };
                if buffer_full {
                    break;
                }

                offset += 1;
            }

            reply.manual().ok();
        });
    }

    // `readdir` is only implemented as a fallback in case `readdirplus` is not available
    // on the current FUSE version.
    fn readdir(
        &self,
        _req: &Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        offset: u64,
        reply: fuser::ReplyDirectory,
    ) {
        log::debug!("[FUSE] readdir(ino: {ino:#x?}, fh: {fh}, offset: {offset})");
        let mut reply = reply_on_drop_guard!(reply, fuser::ReplyDirectory);

        let boxed_path_and_folder_reader = {
            let ptr = fh.0 as *mut (FsPath, FolderReader);
            // SAFETY: `ptr` is a valid pointer that have been created by `opendir`
            // We must increment the refcount given `Arc::from_raw` use in the next line
            // "steal" the owner ship of the pointer (and hence will release the data
            // if the counter is not incremented).
            unsafe { Arc::increment_strong_count(ptr) };
            // SAFETY: `ptr` is a valid pointer that have been created by `opendir`
            unsafe { Arc::from_raw(ptr) }
        };

        let ops = self.ops.clone();
        let inodes = self.inodes.clone();
        self.tokio_handle.spawn(async move {
            let parent_path = &boxed_path_and_folder_reader.0;
            let folder_reader = &boxed_path_and_folder_reader.1;

            let mut offset = offset as usize;
            loop {
                let (child_name, child_stat) = match folder_reader.stat_child(&ops, offset).await {
                    Ok(FolderReaderStatNextOutcome::Entry { name, stat }) => (name, stat),
                    Ok(FolderReaderStatNextOutcome::NoMoreEntries) => break,
                    Ok(FolderReaderStatNextOutcome::InvalidChild) => {
                        // Current entry is invalid, just ignore it
                        offset += 1;
                        continue;
                    }
                    Err(err) => {
                        return match err {
                            FolderReaderStatEntryError::Offline(_) => {
                                reply.manual().error(Errno::EHOSTUNREACH)
                            }
                            FolderReaderStatEntryError::NoRealmAccess => {
                                reply.manual().error(Errno::EPERM)
                            }
                            FolderReaderStatEntryError::Stopped
                            | FolderReaderStatEntryError::RealmDeleted
                            | FolderReaderStatEntryError::InvalidKeysBundle(_)
                            | FolderReaderStatEntryError::InvalidCertificate(_)
                            | FolderReaderStatEntryError::InvalidManifest(_)
                            | FolderReaderStatEntryError::Internal(_) => {
                                log::warn!("FUSE `readdir` operation cannot complete: {err:?}");
                                reply.manual().error(Errno::EIO)
                            }
                        }
                    }
                };

                let child_inode = inodes
                    .lock()
                    .expect("mutex is poisoned")
                    .insert_path(parent_path.join(child_name.to_owned()));

                let buffer_full = match child_stat {
                    EntryStat::File { .. } => reply.borrow().add(
                        child_inode,
                        (offset + 1) as u64,
                        fuser::FileType::RegularFile,
                        OsStr::new(child_name.as_ref()),
                    ),
                    EntryStat::Folder { .. } => reply.borrow().add(
                        child_inode,
                        (offset + 1) as u64,
                        fuser::FileType::Directory,
                        OsStr::new(child_name.as_ref()),
                    ),
                };
                if buffer_full {
                    break;
                }

                offset += 1;
            }

            reply.manual().ok();
        });
    }

    fn releasedir(
        &self,
        _req: &Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        flags: fuser::OpenFlags,
        reply: fuser::ReplyEmpty,
    ) {
        log::debug!("[FUSE] releasedir(ino: {ino:#x?}, fh: {fh}, flags: {flags:#x?})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        {
            let ptr = fh.0 as *mut (FsPath, FolderReader);
            // SAFETY: `ptr` is a valid pointer that have been created by `opendir`
            let _ = unsafe { Arc::from_raw(ptr) };
        }

        reply.manual().ok();
    }

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

async fn getattr_from_path(
    ops: &WorkspaceOps,
    path: FsPath,
    ino: INodeNo,
    uid: u32,
    gid: u32,
    is_read_only: bool,
) -> Result<fuser::FileAttr, Errno> {
    match ops
        .stat_entry(&path)
        .await
        .map(|stat| entry_stat_to_file_attr(stat, ino, uid, gid, is_read_only))
        .inspect(|stat| log::trace!("File stat for {ino}: {stat:?}"))
        .inspect_err(|e| log::trace!("File stat for {ino} result in error: {e:?}"))
    {
        Ok(stat) => Ok(stat),
        Err(err) => match err {
            WorkspaceStatEntryError::EntryNotFound => Err(Errno::ENOENT),
            WorkspaceStatEntryError::Offline(_) => Err(Errno::EHOSTUNREACH),
            WorkspaceStatEntryError::NoRealmAccess => Err(Errno::EPERM),
            WorkspaceStatEntryError::Stopped
            | WorkspaceStatEntryError::RealmDeleted
            | WorkspaceStatEntryError::InvalidKeysBundle(_)
            | WorkspaceStatEntryError::InvalidCertificate(_)
            | WorkspaceStatEntryError::InvalidManifest(_)
            | WorkspaceStatEntryError::Internal(_) => {
                log::warn!("FUSE `getattr_from_path` operation cannot complete: {err:?}");
                Err(Errno::EIO)
            }
        },
    }
}

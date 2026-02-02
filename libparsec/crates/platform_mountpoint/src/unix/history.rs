// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    ffi::OsStr,
    sync::{Arc, Mutex},
};

use libparsec_client::workspace_history::{
    WorkspaceHistoryEntryStat, WorkspaceHistoryFdCloseError, WorkspaceHistoryFdReadError,
    WorkspaceHistoryFolderReader, WorkspaceHistoryFolderReaderStatEntryError,
    WorkspaceHistoryFolderReaderStatNextOutcome, WorkspaceHistoryOpenFileError,
    WorkspaceHistoryOpenFolderReaderError, WorkspaceHistoryOps, WorkspaceHistoryStatEntryError,
};
use libparsec_types::prelude::*;

use super::inode::{Inode, InodesManager};

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
const GENERATION: u64 = 0;
const BLOCK_SIZE: u64 = 512;
const PERMISSIONS: u16 = 0o700;

fn os_name_to_entry_name(name: &OsStr) -> EntryNameResult<EntryName> {
    name.to_str()
        .map(|s| s.parse())
        .ok_or(EntryNameError::InvalidName)?
}

fn entry_stat_to_file_attr(
    stat: WorkspaceHistoryEntryStat,
    inode: Inode,
    uid: u32,
    gid: u32,
) -> fuser::FileAttr {
    match stat {
        WorkspaceHistoryEntryStat::File {
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

        WorkspaceHistoryEntryStat::Folder {
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
    ops: Arc<WorkspaceHistoryOps>,
    tokio_handle: tokio::runtime::Handle,
    inodes: Arc<Mutex<InodesManager>>,
}

impl Filesystem {
    pub fn new(ops: Arc<WorkspaceHistoryOps>, tokio_handle: tokio::runtime::Handle) -> Self {
        Self {
            ops,
            tokio_handle,
            inodes: Arc::new(Mutex::new(InodesManager::new())),
        }
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
    Option<
        Box<
            dyn FnMut(
                    &FsPath,
                )
                    -> Option<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>>
                + Send,
        >,
    >,
> = Mutex::new(None);

// See https://libfuse.github.io/doxygen/structfuse__operations.html for documentation
// of each of the methods implemented here.
impl fuser::Filesystem for Filesystem {
    fn init(
        &mut self,
        _req: &fuser::Request<'_>,
        // see https://libfuse.github.io/doxygen/structfuse__conn__info.html
        config: &mut fuser::KernelConfig,
    ) -> Result<(), libc::c_int> {
        // Try to enable readdirplus optimisation, if it's not possible we fallback on regular readdir
        let _ = config.add_capabilities(fuser::consts::FUSE_DO_READDIRPLUS);
        Ok(())
    }

    // TODO: Make sure opened files are automatically closed on umount, otherwise
    //       we would have some cleanup job do to here !
    fn destroy(&mut self) {}

    /// `lookup` is called every time Fuse meets a new resource it doesn't know about.
    /// The lookup transforms the path name to an `inode`.
    /// The `inode` is then used by all other operations and is freed by `forget`.
    fn lookup(
        &mut self,
        req: &fuser::Request<'_>,
        parent: u64,
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
                    WorkspaceHistoryStatEntryError::EntryNotFound => {
                        reply.manual().error(libc::ENOENT)
                    }
                    WorkspaceHistoryStatEntryError::Offline(_) => {
                        reply.manual().error(libc::EHOSTUNREACH)
                    }
                    WorkspaceHistoryStatEntryError::NoRealmAccess => {
                        reply.manual().error(libc::EPERM)
                    }
                    WorkspaceHistoryStatEntryError::Stopped
                    | WorkspaceHistoryStatEntryError::InvalidKeysBundle(_)
                    | WorkspaceHistoryStatEntryError::InvalidCertificate(_)
                    | WorkspaceHistoryStatEntryError::InvalidManifest(_)
                    | WorkspaceHistoryStatEntryError::InvalidHistory(_)
                    | WorkspaceHistoryStatEntryError::Internal(_) => {
                        log::warn!("FUSE `lookup` operation cannot complete: {err:?}");
                        reply.manual().error(libc::EIO)
                    }
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
        log::debug!("[FUSE] statfs(ino: {ino:#x?})");

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

    fn getattr(
        &mut self,
        req: &fuser::Request<'_>,
        ino: u64,
        _fh: Option<u64>,
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
        self.tokio_handle.spawn(async move {
            let outcome = ops.stat_entry(&path).await;
            log::debug!("[FUSE] getattr(ino: {ino:#x?}) -> {outcome:?}");
            match outcome {
                Ok(stat) => reply
                    .manual()
                    .attr(&TTL, &entry_stat_to_file_attr(stat, ino, uid, gid)),
                Err(err) => match err {
                    WorkspaceHistoryStatEntryError::EntryNotFound => {
                        reply.manual().error(libc::ENOENT)
                    }
                    WorkspaceHistoryStatEntryError::Offline(_) => {
                        reply.manual().error(libc::EHOSTUNREACH)
                    }
                    WorkspaceHistoryStatEntryError::NoRealmAccess => {
                        reply.manual().error(libc::EPERM)
                    }
                    WorkspaceHistoryStatEntryError::Stopped
                    | WorkspaceHistoryStatEntryError::InvalidKeysBundle(_)
                    | WorkspaceHistoryStatEntryError::InvalidCertificate(_)
                    | WorkspaceHistoryStatEntryError::InvalidManifest(_)
                    | WorkspaceHistoryStatEntryError::InvalidHistory(_)
                    | WorkspaceHistoryStatEntryError::Internal(_) => {
                        log::warn!("FUSE `getattr` operation cannot complete: {err:?}");
                        reply.manual().error(libc::EIO)
                    }
                },
            }
        });
    }

    // Note flags doesn't contains O_CREAT, O_EXCL, O_NOCTTY and O_TRUNC
    fn open(&mut self, _req: &fuser::Request<'_>, ino: u64, flags: i32, reply: fuser::ReplyOpen) {
        log::debug!("[FUSE] open(ino: {ino:#x?}, flags: {flags:#x?})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyOpen);

        let path = {
            let inodes_guard = self.inodes.lock().expect("mutex is poisoned");
            inodes_guard.get_path_or_panic(ino)
        };

        let ops = self.ops.clone();

        match flags & libc::O_ACCMODE {
            libc::O_RDONLY => (),
            libc::O_WRONLY | libc::O_RDWR => {
                reply.manual().error(libc::EACCES);
                return;
            }
            // Exactly one access mode flag must be specified
            _ => {
                reply.manual().error(libc::EINVAL);
                return;
            }
        }

        self.tokio_handle.spawn(async move {
            let fd = match ops.open_file(path).await {
                Ok(fd) => fd,
                Err(err) => {
                    return match err {
                        WorkspaceHistoryOpenFileError::EntryNotFound => {
                            reply.manual().error(libc::ENOENT)
                        }
                        WorkspaceHistoryOpenFileError::Offline(_) => {
                            reply.manual().error(libc::EHOSTUNREACH)
                        }
                        WorkspaceHistoryOpenFileError::EntryNotAFile { .. } => {
                            reply.manual().error(libc::EISDIR)
                        }
                        WorkspaceHistoryOpenFileError::NoRealmAccess => {
                            reply.manual().error(libc::EPERM)
                        }
                        WorkspaceHistoryOpenFileError::Stopped
                        | WorkspaceHistoryOpenFileError::InvalidKeysBundle(_)
                        | WorkspaceHistoryOpenFileError::InvalidCertificate(_)
                        | WorkspaceHistoryOpenFileError::InvalidManifest(_)
                        | WorkspaceHistoryOpenFileError::InvalidHistory(_)
                        | WorkspaceHistoryOpenFileError::Internal(_) => {
                            log::warn!("FUSE `open` operation cannot complete: {err:?}");
                            reply.manual().error(libc::EIO)
                        }
                    }
                }
            };

            // Flag is only to enable FOPEN_DIRECT_IO which we don't use
            let open_flags = 0;
            reply.manual().opened(fd.0.into(), open_flags);
        });
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
        log::debug!("[FUSE] read(ino: {ino:#x?}, fh: {fh}, offset: {offset}, size: {size})",);
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
                    WorkspaceHistoryFdReadError::Offline(_)
                    | WorkspaceHistoryFdReadError::ServerBlockstoreUnavailable => reply.manual().error(libc::EHOSTUNREACH),
                    WorkspaceHistoryFdReadError::NoRealmAccess => reply.manual().error(libc::EPERM),
                    WorkspaceHistoryFdReadError::Stopped
                    | WorkspaceHistoryFdReadError::InvalidBlockAccess(_)
                    | WorkspaceHistoryFdReadError::InvalidKeysBundle(_)
                    | WorkspaceHistoryFdReadError::InvalidCertificate(_)
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    | WorkspaceHistoryFdReadError::BadFileDescriptor
                    | WorkspaceHistoryFdReadError::Internal(_)
                    => {
                        log::warn!("FUSE `read` operation cannot complete: {err:?}");
                        reply.manual().error(libc::EIO)
                    }
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
        log::debug!(
            "[FUSE] release(ino: {ino:#x?}, fh: {fh}, flags: {flags:#x?}, lock_owner: {lock_owner:?}, flush: {flush})"
        );
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        let ops = self.ops.clone();
        self.tokio_handle.spawn(async move {
            let fd = FileDescriptor(fh as u32);
            match ops.fd_close(fd) {
                Ok(()) => {
                    reply.manual().ok();
                }
                Err(err) => match err {
                    // Unexpected: FUSE is supposed to only give us valid file descriptors !
                    WorkspaceHistoryFdCloseError::BadFileDescriptor
                    | WorkspaceHistoryFdCloseError::Internal(_) => {
                        log::warn!("FUSE `release` operation cannot complete: {err:?}");
                        reply.manual().error(libc::EIO)
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
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        flags: i32,
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
                        WorkspaceHistoryOpenFolderReaderError::EntryNotFound => {
                            reply.manual().error(libc::ENOENT)
                        }
                        WorkspaceHistoryOpenFolderReaderError::EntryIsFile => {
                            reply.manual().error(libc::ENOTDIR);
                        }
                        WorkspaceHistoryOpenFolderReaderError::Offline(_) => {
                            reply.manual().error(libc::EHOSTUNREACH)
                        }
                        WorkspaceHistoryOpenFolderReaderError::NoRealmAccess => {
                            reply.manual().error(libc::EPERM)
                        }
                        WorkspaceHistoryOpenFolderReaderError::Stopped
                        | WorkspaceHistoryOpenFolderReaderError::InvalidKeysBundle(_)
                        | WorkspaceHistoryOpenFolderReaderError::InvalidCertificate(_)
                        | WorkspaceHistoryOpenFolderReaderError::InvalidManifest(_)
                        | WorkspaceHistoryOpenFolderReaderError::InvalidHistory(_)
                        | WorkspaceHistoryOpenFolderReaderError::Internal(_) => {
                            log::warn!("FUSE `opendir` operation cannot complete: {err:?}");
                            reply.manual().error(libc::EIO)
                        }
                    }
                }
            };

            let open_flags = 0; // TODO: what to set here ?

            // Hold my beer
            let boxed_path_and_folder_reader = Arc::new((path, folder_reader));
            let fh = Arc::into_raw(boxed_path_and_folder_reader) as u64;

            reply.manual().opened(fh, open_flags);
        });
    }

    fn readdirplus(
        &mut self,
        req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: fuser::ReplyDirectoryPlus,
    ) {
        log::debug!("[FUSE] readdirplus(ino: {ino:#x?}, fh: {fh}, offset: {offset})");
        let mut reply = reply_on_drop_guard!(reply, fuser::ReplyDirectoryPlus);

        let boxed_path_and_folder_reader = {
            let ptr = fh as *mut (FsPath, WorkspaceHistoryFolderReader);
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
        self.tokio_handle.spawn(async move {
            let parent_path = &boxed_path_and_folder_reader.0;
            let folder_reader = &boxed_path_and_folder_reader.1;

            let mut offset = offset as usize;
            loop {
                let (child_name, child_stat) = match folder_reader.stat_child(&ops, offset).await {
                    Ok(WorkspaceHistoryFolderReaderStatNextOutcome::Entry { name, stat }) => {
                        (name, stat)
                    }
                    Ok(WorkspaceHistoryFolderReaderStatNextOutcome::NoMoreEntries) => break,
                    Ok(WorkspaceHistoryFolderReaderStatNextOutcome::InvalidChild) => {
                        // Current entry is invalid, just ignore it
                        offset += 1;
                        continue;
                    }
                    Err(err) => {
                        return match err {
                            WorkspaceHistoryFolderReaderStatEntryError::Offline(_) => {
                                reply.manual().error(libc::EHOSTUNREACH)
                            }
                            WorkspaceHistoryFolderReaderStatEntryError::NoRealmAccess => {
                                reply.manual().error(libc::EPERM)
                            }
                            WorkspaceHistoryFolderReaderStatEntryError::Stopped
                            | WorkspaceHistoryFolderReaderStatEntryError::InvalidKeysBundle(_)
                            | WorkspaceHistoryFolderReaderStatEntryError::InvalidCertificate(_)
                            | WorkspaceHistoryFolderReaderStatEntryError::InvalidManifest(_)
                            | WorkspaceHistoryFolderReaderStatEntryError::InvalidHistory(_)
                            | WorkspaceHistoryFolderReaderStatEntryError::Internal(_) => {
                                log::warn!("FUSE `readdirplus` operation cannot complete: {err:?}");
                                reply.manual().error(libc::EIO)
                            }
                        }
                    }
                };

                let child_inode = inodes
                    .lock()
                    .expect("mutex is poisoned")
                    .insert_path(parent_path.join(child_name.to_owned()));

                let buffer_full = match child_stat {
                    WorkspaceHistoryEntryStat::File { .. } => reply.borrow().add(
                        child_inode,
                        (offset + 1) as i64,
                        OsStr::new(child_name.as_ref()),
                        &TTL,
                        &entry_stat_to_file_attr(child_stat, child_inode, uid, gid),
                        GENERATION,
                    ),
                    WorkspaceHistoryEntryStat::Folder { .. } => reply.borrow().add(
                        child_inode,
                        (offset + 1) as i64,
                        OsStr::new(child_name.as_ref()),
                        &TTL,
                        &entry_stat_to_file_attr(child_stat, child_inode, uid, gid),
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
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: fuser::ReplyDirectory,
    ) {
        log::debug!("[FUSE] readdir(ino: {ino:#x?}, fh: {fh}, offset: {offset})");
        let mut reply = reply_on_drop_guard!(reply, fuser::ReplyDirectory);

        let boxed_path_and_folder_reader = {
            let ptr = fh as *mut (FsPath, WorkspaceHistoryFolderReader);
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
                    Ok(WorkspaceHistoryFolderReaderStatNextOutcome::Entry { name, stat }) => {
                        (name, stat)
                    }
                    Ok(WorkspaceHistoryFolderReaderStatNextOutcome::NoMoreEntries) => break,
                    Ok(WorkspaceHistoryFolderReaderStatNextOutcome::InvalidChild) => {
                        // Current entry is invalid, just ignore it
                        offset += 1;
                        continue;
                    }
                    Err(err) => {
                        return match err {
                            WorkspaceHistoryFolderReaderStatEntryError::Offline(_) => {
                                reply.manual().error(libc::EHOSTUNREACH)
                            }
                            WorkspaceHistoryFolderReaderStatEntryError::NoRealmAccess => {
                                reply.manual().error(libc::EPERM)
                            }
                            WorkspaceHistoryFolderReaderStatEntryError::Stopped
                            | WorkspaceHistoryFolderReaderStatEntryError::InvalidKeysBundle(_)
                            | WorkspaceHistoryFolderReaderStatEntryError::InvalidCertificate(_)
                            | WorkspaceHistoryFolderReaderStatEntryError::InvalidManifest(_)
                            | WorkspaceHistoryFolderReaderStatEntryError::InvalidHistory(_)
                            | WorkspaceHistoryFolderReaderStatEntryError::Internal(_) => {
                                log::warn!("FUSE `readdir` operation cannot complete: {err:?}");
                                reply.manual().error(libc::EIO)
                            }
                        }
                    }
                };

                let child_inode = inodes
                    .lock()
                    .expect("mutex is poisoned")
                    .insert_path(parent_path.join(child_name.to_owned()));

                let buffer_full = match child_stat {
                    WorkspaceHistoryEntryStat::File { .. } => reply.borrow().add(
                        child_inode,
                        (offset + 1) as i64,
                        fuser::FileType::RegularFile,
                        OsStr::new(child_name.as_ref()),
                    ),
                    WorkspaceHistoryEntryStat::Folder { .. } => reply.borrow().add(
                        child_inode,
                        (offset + 1) as i64,
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
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        flags: i32,
        reply: fuser::ReplyEmpty,
    ) {
        log::debug!("[FUSE] releasedir(ino: {ino:#x?}, fh: {fh}, flags: {flags:#x?})");
        let reply = reply_on_drop_guard!(reply, fuser::ReplyEmpty);

        {
            let ptr = fh as *mut (FsPath, WorkspaceHistoryFolderReader);
            // SAFETY: `ptr` is a valid pointer that have been created by `opendir`
            let _ = unsafe { Arc::from_raw(ptr) };
        }

        reply.manual().ok();
    }
}

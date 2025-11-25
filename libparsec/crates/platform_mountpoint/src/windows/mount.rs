// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use winfsp_wrs::{
    filetime_now, u16cstr, FileSystem, FileSystemInterface, Params, U16CStr, U16CString,
    VolumeParams,
};

use libparsec_client::{MountpointMountStrategy, WorkspaceHistoryOps, WorkspaceOps};
use libparsec_types::prelude::*;

use super::{
    drive_letter::sorted_drive_letters, volume_label::generate_volume_label,
    winify::winify_entry_name,
};

const FILE_SYSTEM_NAME: &U16CStr = u16cstr!("parsec");

/// The sector size is what is set in VolumeParams via the SectorSize
/// property when the file system is first created. (And of course the
/// "sector size" is misnamed for non-disk file systems, but it determines
/// the granularity at which non-buffered I/O is done.)
/// see: https://github.com/winfsp/winfsp/issues/240#issuecomment-518629301
const SECTOR_SIZE: u16 = 512;

pub struct Mountpoint {
    path: PathBuf,
    filesystem: Option<FileSystem>,
}

impl Mountpoint {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn to_os_path(&self, parsec_path: &FsPath) -> PathBuf {
        let mut path = self.path.to_owned();
        path.extend(parsec_path.parts().iter().map(winify_entry_name));
        path
    }

    pub async fn mount(ops: Arc<WorkspaceOps>) -> anyhow::Result<Self> {
        let (workspace_name, self_role) = ops.get_current_name_and_self_role();
        log::debug!("Preparing mounting of workspace {workspace_name} for role {self_role}");
        let is_read_only = !self_role.can_write();
        let volume_label = generate_volume_label(&workspace_name);
        log::debug!(
            "Volume label for workspace {workspace_name} is {}",
            volume_label.display()
        );

        let filesystem_interface = super::filesystem::ParsecFileSystemInterface::new(
            is_read_only,
            ops.clone(),
            tokio::runtime::Handle::current(),
            volume_label,
        );

        let mountpoint_path = match &ops.config().mountpoint_mount_strategy {
            MountpointMountStrategy::Directory { base_dir } => {
                let (workspace_name, _) = ops.get_current_name_and_self_role();
                find_suitable_mountpoint_dir(base_dir, &workspace_name)?
            }
            MountpointMountStrategy::DriveLetter => {
                let (index, total) = ops.get_workspace_index_and_total_workspaces();
                let maybe_drive = find_suitable_drive_letter(index, total);
                match maybe_drive {
                    None => return Err(anyhow::anyhow!("No more available drive letter")),
                    Some(drive) => drive,
                }
            }
            MountpointMountStrategy::Disabled => return Err(anyhow::anyhow!("Mount disabled !")),
        };

        Self::do_mount(filesystem_interface, is_read_only, mountpoint_path).await
    }

    pub async fn mount_history(
        ops: Arc<WorkspaceHistoryOps>,
        mountpoint_name_hint: EntryName,
    ) -> anyhow::Result<Self> {
        let volume_label = generate_volume_label(&mountpoint_name_hint);
        log::debug!(
            "Volume label for history mountpoint {mountpoint_name_hint} is {}",
            volume_label.display()
        );

        let filesystem_interface = super::history::ParsecFileSystemInterface::new(
            ops.clone(),
            tokio::runtime::Handle::current(),
            volume_label,
        );

        let mountpoint_path = match &ops.config().mountpoint_mount_strategy {
            MountpointMountStrategy::Directory { base_dir } => {
                find_suitable_mountpoint_dir(base_dir, &mountpoint_name_hint)?
            }
            MountpointMountStrategy::DriveLetter => {
                let maybe_drive = find_suitable_drive_letter(0, 1);
                match maybe_drive {
                    None => return Err(anyhow::anyhow!("No more available drive letter")),
                    Some(drive) => drive,
                }
            }
            MountpointMountStrategy::Disabled => return Err(anyhow::anyhow!("Mount disabled !")),
        };

        Self::do_mount(filesystem_interface, true, mountpoint_path).await
    }

    async fn do_mount<FS: FileSystemInterface + Send + 'static>(
        filesystem_interface: FS,
        is_read_only: bool,
        mountpoint_path: PathBuf,
    ) -> anyhow::Result<Self> {
        winfsp_wrs::init()
            .map_err(|err| anyhow::anyhow!("Cannot load the WinFSP DLL: error {}", err))?;

        let mountpoint_path_u16cstr =
            U16CString::from_os_str(mountpoint_path.as_os_str()).expect("Unreachable, valid OsStr");

        let params = {
            // See https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getvolumeinformationa
            let mut volume_params = VolumeParams::default();
            volume_params
                .set_sector_size(SECTOR_SIZE)
                .set_sectors_per_allocation_unit(1)
                .set_volume_creation_time(filetime_now())
                .set_volume_serial_number(0)
                .set_file_info_timeout(1000)
                .set_case_sensitive_search(true)
                .set_case_preserved_names(true)
                .set_unicode_on_disk(true)
                .set_persistent_acls(false)
                .set_reparse_point_access_check(false)
                .set_named_streams(false)
                // TODO: Should detect and re-mount when the workspace switched between read-only and read-write
                .set_read_only_volume(is_read_only)
                .set_post_cleanup_when_modified_only(true)
                .set_device_control(false)
                .set_file_system_name(FILE_SYSTEM_NAME)
                .expect("file system name is small enough")
                .set_prefix(u16cstr!(""))
                .expect("empty prefix is small enough")
                // The minimum value for IRP timeout is 1 minute (default is 5)
                .set_irp_timeout(60000)
                // Max component length correspond to max file name length...
                // However there is hack here: WinFSP expresses the limit in number of
                // WCHAR (i.e. 2 bytes to represent UTF16 character without surrogate
                // pair), however Parsec express the limit in bytes (i.e. size of the
                // data once encoded in UTF8).
                // Hence we are facing a dilemma here:
                // - Using `EntryName::MAX_LENGTH_BYTES`, everything is fine as long a the
                //   file name is entirely composed of ASCII characters.
                // - Using `EntryName::MAX_LENGTH_BYTES / 2` any UTF16 character works, but
                //   we have arbitrary split the size in half :/
                .set_max_component_length(EntryName::MAX_LENGTH_BYTES as u16)
                // When an application closes a file, the Windows OS may still keep the file
                // open for a number of reasons (e.g. caching).
                // This is an issue in Parsec since an opened file is considered busy and
                // cannot be synchronized.
                // WinFPS's `FlushAndPurgeOnCleanup ` flag limits the time that Windows
                // keeps files open after an application has closed them.
                .set_flush_and_purge_on_cleanup(true);

            Params {
                volume_params,
                ..Default::default()
            }
        };

        let filesystem =
            FileSystem::start(params, Some(&mountpoint_path_u16cstr), filesystem_interface)
                .map_err(|status| anyhow::anyhow!("Failed to init FileSystem {status}"))?;

        Ok(Self {
            path: mountpoint_path,
            filesystem: Some(filesystem),
        })
    }

    pub async fn unmount(mut self) -> anyhow::Result<()> {
        if let Some(filesystem) = self.filesystem.take() {
            filesystem.stop();
        }
        Ok(())
    }
}

impl Drop for Mountpoint {
    fn drop(&mut self) {
        // `unmount` hasn't been called, the two reasons for that are:
        // - a panic occurred
        // - WorkspaceOps is stopped from libparsec high level API (in that case,
        //   all the workspace's mountpoints are dropped as a fast-close mechanism)
        //
        // In both cases, we must unmount the filesystem in best effort (i.e. we
        // cannot do anything if errors occur).
        if let Some(filesystem) = self.filesystem.take() {
            filesystem.stop();
        }
    }
}

// Find a suitable path where to mount the workspace. The check we are doing
// here are not atomic (and the mount operation is not itself atomic anyway),
// hence there is still edge-cases where the mount can crash due to concurrent
// changes on the mountpoint path.
fn find_suitable_mountpoint_dir(
    base_mountpoint_path: &Path,
    workspace_name: &EntryName,
) -> anyhow::Result<PathBuf> {
    // Ensure the mountpoint base directory exists
    std::fs::create_dir_all(base_mountpoint_path).context("cannot create base mountpoint dir")?;

    // We are going to use the workspace name as a folder name, however there is (at least)
    // one trick here: `EntryName` allows `\` characters, so without sanitize we can
    // end up with a workspace name that becomes a path with multiple components...
    // Worse: if workspace name starts by `\\`, it can be considered as an absolute UNC
    // path, causing the base mountpoint path to be ignored since `Path::join` replaces
    // the current path if the param is absolute :/
    let workspace_name: String = {
        let name = winify_entry_name(workspace_name);
        // Sanity check to ensure we end up with a relative single component path
        let name_as_path = Path::new(&name);
        assert!(
            // `Some(...)` means the path is a relative one with a single component, (while
            // `None` would have meant the path is an absolute one with a single component).
            name_as_path.parent() == Some(Path::new("")),
            "Workspace name `{:?}` cannot form a valid path item",
            workspace_name
        );
        name
    };

    // It is most likely the suitable directory is found on the first attempt,
    // and the likelihood of finding a suitable directory exponentially decreases
    // with each new attempt, hence we limit the number of attempts to avoid
    // infinite loop.
    for attempt in 1..=100 {
        let mountpoint_path = if attempt == 1 {
            base_mountpoint_path.join(&workspace_name)
        } else {
            base_mountpoint_path.join(format!("{} ({})", &workspace_name, attempt))
        };

        // For WinFSP, mounting target must NOT exists
        let outcome = mountpoint_path.try_exists();
        match outcome {
            // match mountpoint_path.try_exists() {
            // The mountpoint already exists, so we must find another one :/
            // Note we don't try to reuse empty directory (like it is done with FUSE), this
            // is because for this we would have to first remove the empty directory which
            // doesn't detect if the directory is a WinFSP mountpoint on an empty workspace !
            Ok(true) => continue,
            // We can't even know if that mountpoint path exists
            // This might be due to a permission error, or something else.
            // In any case, there's not much we can do here, better skip to the next name.
            Err(_) => continue,
            // The mountpoint doesn't exist, so we can use it !
            Ok(false) => (),
        }

        // Artifacts from previous run can remain listed in the directory
        // (even though `mountpoint_path.try_exists()`) returns `Ok(false)`.
        // In this case, `std::fs::remove_dir()` still works and fixes the issue.
        // Note checking the target existence and doing this remove are obviously not
        // one atomic operation, hence a concurrent file creation may mess this up.
        // This is considered "fine enough" though:
        // - This scenario is highly unlikely.
        // - Concurrent creation of a file or a non-empty directory will cause `remove_dir` to fail.
        //   So we only overwrite empty directory which shouldn't be a big deal.
        match std::fs::remove_dir(&mountpoint_path) {
            // The artifact has been successfully removed
            Ok(()) => (),
            // There was no artefact in the first place, it's fine
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => (),
            // Nothing we can do to fix this path :/
            Err(_) => continue,
        }

        // Now the mountpoint path should be non-existent and properly cleaned up
        return Ok(mountpoint_path);
    }

    Err(anyhow::anyhow!("Cannot find a suitable mountpoint path"))
}

pub(crate) fn find_suitable_drive_letter(index: usize, length: usize) -> Option<PathBuf> {
    for candidate_drive_letter in sorted_drive_letters(index, length) {
        let path = PathBuf::from(format!("{}:", candidate_drive_letter));
        if !path.exists() {
            return Some(path);
        }
    }
    None
}

/// Cleanup mountpoint base directory
pub async fn clean_base_mountpoint_dir(
    _mountpoint_base_path: PathBuf,
) -> anyhow::Result<(), libparsec_types::anyhow::Error> {
    // Currently a no-op on Windows. See `clean_base_mountpoint_dir` in `unix/mount.rs`
    Ok(())
}

// TODO: is volume serial number really needed ?
// fn generate_volume_serial_number(device: LocalDevice, workspace_id: VlobID) -> u64 {
//     use std::hash::{Hash, Hasher};

//     // TODO: default hasher provides no guarantee on the implementation stability,
//     //       do we want to make sure the workspace always has the same serial number ?
//     let mut hasher = std::collections::hash_map::DefaultHasher::new();

//     device.organization_id().as_ref().hash(&mut hasher);
//     hasher.write_u8(0);
//     device.device_id.user_id().as_ref().hash(&mut hasher);
//     hasher.write_u8(0);
//     device.device_id.device_name().as_ref().hash(&mut hasher);
//     hasher.write_u8(0);
//     workspace_id.hash(&mut hasher);

//     hasher.finish()
// }

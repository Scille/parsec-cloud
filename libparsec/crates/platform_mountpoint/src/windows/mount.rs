// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;
use winfsp_wrs::{filetime_now, u16cstr, FileSystem, Params, U16CStr, U16CString, VolumeParams};

use libparsec_client::{MountpointMountStrategy, WorkspaceOps};
use libparsec_types::prelude::*;

use super::{
    drive_letter::sorted_drive_letters, filesystem::ParsecFileSystemContext,
    volume_label::generate_volume_label, winify::winify_entry_name,
};

const FILE_SYSTEM_NAME: &U16CStr = u16cstr!("parsec");

/// The sector size is what is set in VolumeParams via the SectorSize
/// property when the file system is first created. (And of course the
/// "sector size" is misnamed for non-disk file systems, but it determines
/// the granularity at which non-buffered I/O is done.)
/// see: https://github.com/winfsp/winfsp/issues/240#issuecomment-518629301
const SECTOR_SIZE: u16 = 512;

#[derive(Debug)]
pub struct Mountpoint {
    path: std::path::PathBuf,
    filesystem: FileSystem<ParsecFileSystemContext>,
}

impl Mountpoint {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn to_os_path(&self, parsec_path: &FsPath) -> std::path::PathBuf {
        let mut path = self.path.to_owned();
        path.extend(parsec_path.parts().iter().map(winify_entry_name));
        path
    }

    pub async fn mount(ops: Arc<WorkspaceOps>) -> anyhow::Result<Self> {
        winfsp_wrs::init()
            .map_err(|err| anyhow::anyhow!("Cannot load the WinFSP DLL: error {}", err))?;

        let (workspace_name, _) = ops.get_current_name_and_self_role();

        let mountpoint_path = match &ops.config().mountpoint_mount_strategy {
            MountpointMountStrategy::Directory { base_dir } => {
                find_suitable_mountpoint_dir(base_dir, &ops)?
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
        let mountpoint_path_u16cstr =
            U16CString::from_os_str(mountpoint_path.as_os_str()).expect("Unreachable, valid OsStr");

        let volume_label = generate_volume_label(&workspace_name);

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
                // .set_read_only_volume(...)
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
                .set_max_component_length(EntryName::MAX_LENGTH_BYTES as u16);

            Params {
                volume_params,
                ..Default::default()
            }
        };

        let context =
            ParsecFileSystemContext::new(ops, tokio::runtime::Handle::current(), volume_label);
        let filesystem = FileSystem::new(params, Some(&mountpoint_path_u16cstr), context)
            .map_err(|status| anyhow::anyhow!("Failed to init FileSystem {status}"))?;

        Ok(Self {
            path: mountpoint_path,
            filesystem,
        })
    }

    pub async fn unmount(self) -> anyhow::Result<()> {
        self.filesystem.stop();
        Ok(())
    }
}

// Find a suitable path where to mount the workspace. The check we are doing
// here are not atomic (and the mount operation is not itself atomic anyway),
// hence there is still edge-cases where the mount can crash due to concurrent
// changes on the mountpoint path.
fn find_suitable_mountpoint_dir(
    base_mountpoint_path: &std::path::Path,
    ops: &WorkspaceOps,
) -> anyhow::Result<std::path::PathBuf> {
    // Ensure the mountpoint base directory exists
    std::fs::create_dir_all(base_mountpoint_path).context("cannot create base mountpoint dir")?;

    let (workspace_name, _) = ops.get_current_name_and_self_role();

    for tentative in 1.. {
        let mountpoint_path = if tentative == 1 {
            base_mountpoint_path.join(workspace_name.as_ref())
        } else {
            base_mountpoint_path.join(format!("{} ({})", workspace_name.as_ref(), tentative))
        };

        // For WinFSP, mounting target must NOT exists
        match mountpoint_path.try_exists() {
            // The mountpoint doesn't exist, so we can use it !
            Ok(false) => (),
            // The mountpoint already exists, so we must find another one :/
            // Note we don't try to reuse empty directory (like it is done with FUSE), this
            // is because for this we would have to first remove the empty directory which
            // doesn't detect if the directory is a WinFSP mountpoint on an empty workspace !
            Ok(true) => continue,
            // An error might be caused by a previous mount that remained listed in the
            // directory (due to bug or crash), in such case removing the faulty entry
            // is enough to fix the issue.
            //
            // Note there is other unrelated reasons that can cause an error here (e.g.
            // permission issues in a parent folder), in this case the remove should also fail.
            Err(_) => {
                match std::fs::remove_file(&mountpoint_path) {
                    Ok(()) => (),
                    Err(err) if err.kind() == std::io::ErrorKind::NotFound => (),
                    // Nothing we can do to fix this path :/
                    Err(_) => continue,
                }
            }
        }

        return Ok(mountpoint_path);
    }

    unreachable!()
}

pub(crate) fn find_suitable_drive_letter(
    index: usize,
    length: usize,
) -> Option<std::path::PathBuf> {
    for candidate_drive_letter in sorted_drive_letters(index, length) {
        let path = std::path::PathBuf::from(format!("{}:", candidate_drive_letter));
        if !path.exists() {
            return Some(path);
        }
    }
    None
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

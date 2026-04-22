pub mod change_authentication;
pub mod export_recovery_device;
pub mod forget_local;
pub mod import_recovery_device;
pub mod list;
pub mod overwrite_server_url;

#[derive(clap::Subcommand)]
pub enum Group {
    /// Forget a local device
    /// It will still exist on the server but not locally anymore
    ForgetLocal(forget_local::Args),
    /// List all devices
    List(list::Args),
    /// Change authentication medium for a device
    ChangeAuthentication(change_authentication::Args),
    /// Export recovery device
    ExportRecoveryDevice(export_recovery_device::Args),
    /// Import recovery device
    ImportRecoveryDevice(import_recovery_device::Args),
    /// Change the server URL for the device, this is normally not needed.
    ///
    /// This is only useful if the organization gets migrated to a new server with
    /// a different domain name.
    OverwriteServerURL(overwrite_server_url::Args),
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::ForgetLocal(args) => forget_local::main(args).await,
        Group::List(args) => list::main(args).await,
        Group::ChangeAuthentication(args) => change_authentication::main(args).await,
        Group::ExportRecoveryDevice(args) => export_recovery_device::main(args).await,
        Group::ImportRecoveryDevice(args) => import_recovery_device::main(args).await,
        Group::OverwriteServerURL(args) => overwrite_server_url::main(args).await,
    }
}

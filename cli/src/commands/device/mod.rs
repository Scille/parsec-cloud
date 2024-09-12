pub mod export_recovery_device;
pub mod import_recovery_device;
pub mod list;
pub mod remove;

#[derive(clap::Subcommand)]
pub enum Group {
    /// Remove device
    Remove(remove::Args),
    /// List all devices
    List(list::Args),
    /// Export recovery device
    ExportRecoveryDevice(export_recovery_device::Args),
    /// Import recovery device
    ImportRecoveryDevice(import_recovery_device::Args),
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::Remove(args) => remove::main(args).await,
        Group::List(args) => list::main(args).await,
        Group::ExportRecoveryDevice(args) => export_recovery_device::main(args).await,
        Group::ImportRecoveryDevice(args) => import_recovery_device::main(args).await,
    }
}

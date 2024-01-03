// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod bootstrap_organization;
mod cancel_invitation;
mod claim_invitation;
mod create_organization;
mod greet_invitation;
mod invite_device;
mod invite_user;
mod list_devices;
mod remove_device;
#[cfg(feature = "testenv")]
mod run_testenv;
mod stats_organization;
#[cfg(all(test, feature = "testenv"))]
mod tests;
mod utils;

use clap::{Parser, Subcommand};

/// Parsec cli
#[derive(Parser)]
#[command(version)]
struct Arg {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Configure new organization
    BootstrapOrganization(bootstrap_organization::BootstrapOrganization),
    /// Cancel invitation
    CancelInvitation(cancel_invitation::CancelInvitation),
    /// Claim invitation
    ClaimInvitation(claim_invitation::ClaimInvitation),
    /// Create new organization
    CreateOrganization(create_organization::CreateOrganization),
    /// Create device invitation
    InviteDevice(invite_device::InviteDevice),
    /// Create user invitation
    InviteUser(invite_user::InviteUser),
    /// Greet invitation
    GreetInvitation(greet_invitation::GreetInvitation),
    /// List all devices
    ListDevices(list_devices::ListDevices),
    /// Remove device
    RemoveDevice(remove_device::RemoveDevice),
    #[cfg(feature = "testenv")]
    /// Create a temporary environment and initialize a test setup for parsec.
    /// #### WARNING: it also leaves an in-memory server running in the background.
    /// This command creates three users, `Alice`, `Bob` and `Toto`,
    /// To run testenv, see the script run_testenv in the current directory.
    RunTestenv(run_testenv::RunTestenv),
    /// Get data & user statistics on organization
    StatsOrganization(stats_organization::StatsOrganization),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arg = Arg::parse();

    match arg.command {
        Command::BootstrapOrganization(bootstrap_organization) => {
            bootstrap_organization::bootstrap_organization(bootstrap_organization).await
        }
        Command::CancelInvitation(cancel_invitation) => {
            cancel_invitation::cancel_invitation(cancel_invitation).await
        }
        Command::ClaimInvitation(claim_invitation) => {
            claim_invitation::claim_invitation(claim_invitation).await
        }
        Command::CreateOrganization(create_organization) => {
            create_organization::create_organization(create_organization).await
        }
        Command::InviteDevice(invite_device) => invite_device::invite_device(invite_device).await,
        Command::InviteUser(invite_user) => invite_user::invite_user(invite_user).await,
        Command::GreetInvitation(greet_invitation) => {
            greet_invitation::greet_invitation(greet_invitation).await
        }
        Command::ListDevices(list_devices) => list_devices::list_devices(list_devices).await,
        Command::RemoveDevice(remove_device) => remove_device::remove_device(remove_device).await,
        #[cfg(feature = "testenv")]
        Command::RunTestenv(run_testenv) => run_testenv::run_testenv(run_testenv).await,
        Command::StatsOrganization(stats_organization) => {
            stats_organization::stats_organization(stats_organization).await
        }
    }
}

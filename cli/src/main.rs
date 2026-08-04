// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use parsec_cli::{commands::*, ui, Arg, Command};

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arg = Arg::parse();
    env_logger::init();

    let ui = ui::Ui {
        format: arg.format,
        progress: arg.progress,
        color: arg.color,
    };

    match arg.command {
        Command::Device(device) => device::dispatch_command(ui, device).await,
        Command::Invite(invitation) => invite::dispatch_command(invitation).await,
        Command::Organization(organization) => organization::dispatch_command(organization).await,
        Command::User(user) => user::dispatch_command(user).await,
        Command::Server(server) => server::dispatch_command(server).await,
        Command::Workspace(workspace) => workspace::dispatch_command(ui, workspace).await,
        Command::Certificate(certificate) => certificate::dispatch_command(certificate).await,
        #[cfg(feature = "testenv")]
        Command::RunTestenv(run_testenv) => run_testenv::run_testenv(run_testenv).await,
        Command::Ls(ls) => ls::main(ls).await,
        Command::Rm(rm) => rm::main(rm).await,
        Command::Tos(tos) => tos::dispatch_command(tos).await,
        Command::SharedRecovery(shared_recovery) => {
            shared_recovery::dispatch_command(shared_recovery).await
        }
        Command::MountRealmExport(mount_realm_export) => {
            mount_realm_export::main(mount_realm_export).await
        }
        #[cfg(feature = "manpage")]
        Command::ManPage(args) => man_page::main(ui, args).await,
    }
}

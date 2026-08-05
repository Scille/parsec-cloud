pub mod commands;
pub mod macros;

#[cfg(any(test, feature = "testenv"))]
pub mod testenv_utils;
pub mod ui;
#[cfg(test)]
#[path = "../tests/unit/mod.rs"]
mod unit_tests;
pub mod utils;

pub(crate) use ui::Ui;

use commands::*;

/// Parsec cli
#[derive(clap::Parser)]
#[command(version(LongVersion))]
pub struct Arg {
    #[command(subcommand)]
    pub command: Command,
    /// How message are displayed
    #[arg(long, default_value_t)]
    pub progress: ui::ProgressStyle,
    /// How to format outputted data
    #[arg(long, default_value_t)]
    pub format: ui::DataFormat,
    #[arg(long, default_value_t)]
    pub color: clap::ColorChoice,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Contains subcommands related to server operations
    #[command(subcommand)]
    Server(server::Group),
    /// Contains subcommands related to devices
    #[command(subcommand)]
    Device(device::Group),
    /// Contains subcommands related to invitation
    #[command(subcommand)]
    Invite(invite::Group),
    /// Contains subcommands related to organization
    #[command(subcommand)]
    Organization(organization::Group),
    /// Contains subcommands related to user
    #[command(subcommand)]
    User(user::Group),
    /// Contains subcommands related to workspace
    #[command(subcommand)]
    Workspace(workspace::Group),
    /// Contains subcommands related to certificate
    #[command(subcommand)]
    Certificate(certificate::Group),
    #[cfg(feature = "testenv")]
    /// Create a temporary environment and initialize a test setup for parsec.
    /// #### WARNING: it also leaves an in-memory server running in the background.
    /// This command creates three users, `Alice`, `Bob` and `Toto`,
    /// To run testenv, see the script run_testenv in the current directory.
    RunTestenv(run_testenv::RunTestenv),
    /// List workspace contents
    Ls(ls::Args),
    /// Remove a file from a workspace
    Rm(rm::Args),
    /// Contains subcommands related to Term of Service (TOS).
    #[command(subcommand)]
    Tos(tos::Group),
    /// Contains subcommands related to shared recovery devices (shamir)
    #[command(subcommand)]
    SharedRecovery(shared_recovery::Group),
    /// Mount a realm export as a workspace.
    MountRealmExport(mount_realm_export::Args),
    /// Print CLI man pages to the console or a file
    #[cfg(feature = "manpage")]
    ManPage(man_page::Args),
    #[command(subcommand)]
    Sequester(sequester::Group),
    #[cfg(feature = "shell-completion")]
    AutoComplete(auto_complete::Args),
}

struct LongVersion;

shadow_rs::shadow!(build);

impl From<LongVersion> for clap::builder::Str {
    fn from(_value: LongVersion) -> Self {
        use build::*;
        use libparsec_crypto::CRYPTO_BACKEND;

        // cspell: words: formatcp
        shadow_rs::formatcp!(
            // One the same line since `clap` prefix with the cli name and expect the version to follow
            r#"{PKG_VERSION}
branch: {BRANCH}
commit_hash: {SHORT_COMMIT}
build_target: {BUILD_TARGET}
build_os: {BUILD_OS}
build_profile: {BUILD_RUST_CHANNEL}
cli_features: {CARGO_FEATURES}
crypto_backend: {CRYPTO_BACKEND}
rust_version: {RUST_VERSION}
rust_channel: {RUST_CHANNEL}
cargo_version: {CARGO_VERSION}
git_clean: {GIT_CLEAN}"#,
        )
        .into()
    }
}

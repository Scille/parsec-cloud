pub mod list;
pub mod revoke;
pub mod totp_reset;

#[derive(clap::Subcommand)]
pub enum Group {
    /// List users
    List(list::Args),
    /// Revoke a user
    Revoke(revoke::Args),
    /// Reset TOTP for a user (administration command)
    TotpReset(totp_reset::Args),
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::List(args) => list::main(args).await,
        Group::Revoke(args) => revoke::main(args).await,
        Group::TotpReset(args) => totp_reset::main(args).await,
    }
}

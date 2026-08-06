pub mod list;
pub mod revoke;
pub mod totp_reset;

#[derive(clap::Subcommand)]
pub enum Group {
    /// List users
    List(list::Args),
    /// Revoke a user
    Revoke(revoke::Args),
    /// Reset TOTP (aka MFA, Multi-Factor Authentication) for a user (administration command)
    TotpReset(totp_reset::Args),
}

pub async fn dispatch_command(ui: crate::Ui, command: Group) -> anyhow::Result<()> {
    match command {
        Group::List(args) => list::main(ui, args).await,
        Group::Revoke(args) => revoke::main(ui, args).await,
        Group::TotpReset(args) => totp_reset::main(ui, args).await,
    }
}

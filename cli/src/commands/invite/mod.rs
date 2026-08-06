mod cancel;
mod claim;
mod device;
mod greet;
mod list;
mod shared_recovery;
mod user;

#[derive(clap::Subcommand)]
pub enum Group {
    /// Cancel an invitation
    Cancel(cancel::Args),
    /// Claim an invitation
    Claim(claim::Args),
    /// Greet an invitation
    Greet(greet::Args),
    /// List invitations
    List(list::Args),
    /// Create user invitation
    User(user::Args),
    /// Create device invitation
    Device(device::Args),
    /// Create shared recovery invitation
    SharedRecovery(shared_recovery::Args),
}

pub async fn dispatch_command(ui: crate::Ui, command: Group) -> anyhow::Result<()> {
    match command {
        Group::Cancel(args) => cancel::main(ui, args).await,
        Group::Claim(args) => claim::main(ui, args).await,
        Group::Greet(args) => greet::main(ui, args).await,
        Group::List(args) => list::main(ui, args).await,
        Group::User(args) => user::main(ui, args).await,
        Group::Device(args) => device::main(ui, args).await,
        Group::SharedRecovery(args) => shared_recovery::main(ui, args).await,
    }
}

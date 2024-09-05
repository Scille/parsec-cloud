mod cancel;
mod claim;
mod device;
mod greet;
mod list;
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
}

pub async fn dispatch_command(command: Group) -> anyhow::Result<()> {
    match command {
        Group::Cancel(args) => cancel::main(args).await,
        Group::Claim(args) => claim::main(args).await,
        Group::Greet(args) => greet::main(args).await,
        Group::List(args) => list::main(args).await,
        Group::User(args) => user::main(args).await,
        Group::Device(args) => device::main(args).await,
    }
}

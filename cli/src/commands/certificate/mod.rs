pub mod forget_all_certificates;
pub mod poll;

#[derive(clap::Subcommand)]
pub enum Group {
    /// Poll the server for new certificates.
    Poll(poll::Args),
    /// Forget all certificates from the local database, this is not needed under normal circumstances.
    ///
    /// Clearing the certificates might be useful in case the server database got rolled back
    /// to a previous state, resulting in the local database containing certificates that are no
    /// longer valid.
    ///
    /// Note that this scenario is technically similar to a server compromise, so the user
    /// should ensure the rollback is legit before clearing the certificates!
    ForgetAllCertificates(forget_all_certificates::Args),
}

pub async fn dispatch_command(ui: crate::Ui, command: Group) -> anyhow::Result<()> {
    match command {
        Group::Poll(args) => poll::main(ui, args).await,
        Group::ForgetAllCertificates(args) => forget_all_certificates::main(ui, args).await,
    }
}

pub mod gen_service_cert;
pub mod mount;

#[derive(clap::Subcommand)]
pub enum Group {
    GenerateServiceCertificate(gen_service_cert::Args),
    Mount(mount::Args),
}

pub async fn dispatch_command(ui: crate::Ui, command: Group) -> anyhow::Result<()> {
    match command {
        Group::GenerateServiceCertificate(args) => gen_service_cert::main(ui, args).await,
        Group::Mount(args) => mount::main(ui, args).await,
    }
}

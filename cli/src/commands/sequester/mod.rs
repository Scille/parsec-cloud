pub mod gen_service_cert;

#[derive(clap::Subcommand)]
pub enum Group {
    GenerateServiceCertificate(gen_service_cert::Args),
}

pub async fn dispatch_command(ui: crate::Ui, command: Group) -> anyhow::Result<()> {
    match command {
        Group::GenerateServiceCertificate(args) => gen_service_cert::main(ui, args).await,
    }
}

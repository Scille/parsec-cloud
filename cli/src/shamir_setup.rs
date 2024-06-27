use clap::Args;

#[derive(Args)]
pub struct ShamirSetupCreate {
    todo: (),
}

pub async fn shamir_setup_create(shamir_setup: ShamirSetupCreate) -> anyhow::Result<()> {
    todo!()
}

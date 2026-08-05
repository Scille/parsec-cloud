use anyhow::Context;
use clap::CommandFactory;
use tokio::io::AsyncWriteExt;

/// Display autocomplete setup script for shell
#[derive(clap::Args)]
pub struct Args {
    /// Shell to generate completion script for (e.g. bash, fish, powershell, zsh, ...)
    shell: clap_complete::Shell,
}

pub async fn main(_ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let Args { shell } = args;

    let mut cmd = crate::Arg::command();
    let bin_name = cmd.get_name().to_owned();
    let mut buf = Vec::with_capacity(2048);
    clap_complete::generate(shell, &mut cmd, bin_name, &mut buf);
    tokio::io::stdout()
        .write_all_buf(&mut std::io::Cursor::new(buf))
        .await
        .context("Failed to write completion script to output")?;
    Ok(())
}

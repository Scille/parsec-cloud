use libparsec_client::Tos;

use crate::utils::load_client;

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin]
    pub struct Args {
    }
);

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        device,
        config_dir,
        password_stdin,
    } = args;
    log::trace!(
        "Listing Term of Service (confdir={}, device={})",
        config_dir.display(),
        device.as_deref().unwrap_or("N/A")
    );

    let client = load_client(&config_dir, device, password_stdin).await?;
    match client.get_tos().await {
        Ok(tos) => display_tos(&tos),
        Err(libparsec_client::ClientGetTosError::NoTos) => {
            no_tos_available();
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

fn no_tos_available() {
    println!("No Terms of Service available");
}

pub(super) fn display_tos(tos: &Tos) -> anyhow::Result<()> {
    use std::io::Write;

    let mut stdout = std::io::stdout().lock();
    writeln!(
        stdout,
        "Terms of Service updated on {}:",
        tos.updated_on.to_rfc3339()
    )?;
    for (locale, url) in &tos.per_locale_urls {
        writeln!(stdout, "- {locale}: {url}")?;
    }
    Ok(())
}

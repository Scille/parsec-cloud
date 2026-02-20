// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec::{
    AvailableDevice, ClientConfig, DeviceLabel, DevicePrimaryProtectionStrategy,
    DeviceSaveStrategy, EmailAddress, HumanHandle, ParsecOrganizationBootstrapAddr, Password,
};

use crate::utils::*;

#[derive(clap::Parser)]
pub struct Args {
    /// Bootstrap address
    /// (e.g: parsec3://127.0.0.1:6770/Org?no_ssl=true&action=bootstrap_organization&token=59961ba6dcc9b018d2fdc9da1c0c762b716a27cff30594562dc813e4b765871a)
    #[arg(short, long)]
    addr: ParsecOrganizationBootstrapAddr,
    /// Device label
    #[arg(short, long)]
    device_label: DeviceLabel,
    /// User fullname
    #[arg(short, long)]
    label: String,
    /// User email
    #[arg(short, long)]
    email: EmailAddress,
    /// Sequester authority verify key path
    #[arg(long)]
    sequester_key: Option<PathBuf>,
    /// Read the password from stdin instead of TTY
    #[arg(long, default_value_t)]
    password_stdin: bool,
}

pub async fn bootstrap_organization_req(
    client_config: ClientConfig,
    addr: ParsecOrganizationBootstrapAddr,
    device_label: DeviceLabel,
    human_handle: HumanHandle,
    password: Password,
    sequester_authority_verify_key_pem: Option<&str>,
) -> anyhow::Result<AvailableDevice> {
    log::trace!(
        "Bootstrapping organization (confdir={}, datadir={})",
        client_config.config_dir.display(),
        client_config.data_base_dir.display()
    );

    libparsec::libparsec_init_set_on_event_callback(Arc::new(|_, _| ()));
    libparsec::bootstrap_organization(
        client_config,
        addr,
        DeviceSaveStrategy {
            totp_protection: None,
            primary_protection: DevicePrimaryProtectionStrategy::Password { password },
        },
        human_handle,
        device_label,
        sequester_authority_verify_key_pem,
    )
    .await
    .map_err(anyhow::Error::from)
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    let Args {
        email,
        label,
        addr,
        device_label,
        password_stdin,
        sequester_key,
    } = args;
    log::trace!("Bootstrapping organization (addr={addr})");

    let human_handle = HumanHandle::new(email, &label)
        .map_err(|e| anyhow::anyhow!("Cannot create human handle: {e}"))?;

    let password = read_password(if password_stdin {
        ReadPasswordFrom::Stdin
    } else {
        ReadPasswordFrom::Tty {
            prompt: "New device password:",
        }
    })?;

    let sequester_authority_verify_key_pem = if let Some(path) = sequester_key {
        let raw = tokio::fs::read_to_string(path).await?;
        Some(raw)
    } else {
        None
    };

    let mut handle = start_spinner("Bootstrapping organization in the server".into());

    let new_device = bootstrap_organization_req(
        ClientConfig::default(),
        addr,
        device_label.clone(),
        human_handle.clone(),
        password,
        sequester_authority_verify_key_pem.as_deref(),
    )
    .await?;

    handle.stop_with_message("Organization bootstrapped".into());

    println!("New device created:");
    println!("{}", &format_single_device(&new_device));

    Ok(())
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, path::PathBuf, sync::Arc};

use libparsec::{
    list_available_devices, load_device, AuthenticatedCmds, AvailableDevice, DeviceAccessStrategy,
    DeviceFileType, LocalDevice, ProxyConfig,
};

pub const GREEN: &str = "\x1B[92m";
pub const RESET: &str = "\x1B[39m";
pub const YELLOW: &str = "\x1B[33m";

pub fn format_devices(devices: &[AvailableDevice]) {
    let n = devices.len();
    // Try to shorten the slughash to make it easier to work with
    let slug_len = 2 + (n + 1).ilog2() as usize;

    for device in devices {
        let slug = &device.slughash()[..slug_len];
        let organization_id = &device.organization_id;
        let human_handle = &device.human_handle;
        let device_label = &device.device_label;
        println!("{YELLOW}{slug}{RESET} - {organization_id}: {human_handle} @ {device_label}");
    }
}

pub async fn load_device_and_run<F, Fut>(
    config_dir: PathBuf,
    device_slughash: Option<String>,
    function: F,
) -> anyhow::Result<()>
where
    F: FnOnce(AuthenticatedCmds, Arc<LocalDevice>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    let devices = list_available_devices(&config_dir).await;

    if let Some(device_slughash) = device_slughash {
        let mut possible_devices = vec![];

        for device in &devices {
            if device.slughash().starts_with(&device_slughash) {
                possible_devices.push(device);
            }
        }

        match possible_devices.len() {
            0 => {
                println!("Device `{device_slughash}` not found, available devices:");
                format_devices(&devices);
            }
            1 => {
                let device = &possible_devices[0];

                let device = match device.ty {
                    DeviceFileType::Password => {
                        #[cfg(feature = "testenv")]
                        let password = "test".to_string().into();
                        #[cfg(not(feature = "testenv"))]
                        let password = rpassword::prompt_password("password:")?.into();

                        let access = DeviceAccessStrategy::Password {
                            key_file: device.key_file_path.clone(),
                            password,
                        };

                        // The password is invalid or the binary is compiled with fast crypto
                        load_device(&config_dir, &access).await?
                    }
                    DeviceFileType::Smartcard => {
                        let access = DeviceAccessStrategy::Smartcard {
                            key_file: device.key_file_path.clone(),
                        };

                        load_device(&config_dir, &access).await?
                    }
                    DeviceFileType::Recovery => {
                        return Err(anyhow::anyhow!(
                            "Unsupported device file authentication `{:?}`",
                            device.ty
                        ));
                    }
                };

                let cmds = AuthenticatedCmds::new(
                    &config_dir,
                    device.clone(),
                    ProxyConfig::new_from_env()?,
                )?;

                function(cmds, device.clone()).await?;
            }
            _ => {
                println!("Multiple devices found for `{device_slughash}`:");
                format_devices(&devices);
            }
        }
    } else {
        println!("Error: Missing option '--device'\n");
        println!("Available devices:");
        format_devices(&devices);
    }

    Ok(())
}

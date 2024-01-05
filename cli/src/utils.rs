// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{future::Future, path::PathBuf, sync::Arc};

use libparsec::{
    internal::{Client, EventBus},
    list_available_devices, load_device, AuthenticatedCmds, AvailableDevice, ClientConfig,
    DeviceAccessStrategy, DeviceFileType, DeviceLabel, HumanHandle, LocalDevice, Password,
    ProxyConfig, SASCode, UserProfile,
};
use terminal_spinners::{SpinnerBuilder, SpinnerHandle, DOTS};

pub const GREEN: &str = "\x1B[92m";
pub const RED: &str = "\x1B[91m";
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
    F: FnOnce(Arc<LocalDevice>) -> Fut,
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
                let device = possible_devices[0];

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

                        // This will fail if the password is invalid, but also if the binary is compiled with fast crypto (see  libparsec_crypto)
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

                function(device).await?;
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

pub async fn load_cmds_and_run<F, Fut>(
    config_dir: PathBuf,
    device_slughash: Option<String>,
    function: F,
) -> anyhow::Result<()>
where
    F: FnOnce(AuthenticatedCmds, Arc<LocalDevice>) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    load_device_and_run(config_dir.clone(), device_slughash, |device| async move {
        let cmds =
            AuthenticatedCmds::new(&config_dir, device.clone(), ProxyConfig::new_from_env()?)?;

        function(cmds, device).await
    })
    .await
}

pub async fn load_client_and_run<F, Fut>(
    config_dir: PathBuf,
    device_slughash: Option<String>,
    function: F,
) -> anyhow::Result<()>
where
    F: FnOnce(Client) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    load_device_and_run(config_dir, device_slughash, |device| async move {
        let client = Client::start(
            Arc::new(ClientConfig::default().into()),
            EventBus::default(),
            device,
        )
        .await?;

        client.user_ops.sync().await?;

        function(client).await
    })
    .await
}

pub fn start_spinner(text: &'static str) -> SpinnerHandle {
    SpinnerBuilder::new().spinner(&DOTS).text(text).start()
}

pub fn choose_password() -> anyhow::Result<Password> {
    #[cfg(feature = "testenv")]
    return Ok("test".to_string().into());
    #[cfg(not(feature = "testenv"))]
    loop {
        let password = rpassword::prompt_password("Enter password for the new device:")?.into();
        let confirm_password = rpassword::prompt_password("Confirm password:")?.into();

        if password == confirm_password {
            return Ok(password);
        } else {
            eprintln!("Password mismatch")
        }
    }
}

pub fn choose_device_label(input: &mut String) -> anyhow::Result<DeviceLabel> {
    loop {
        println!("Enter device label:");
        input.clear();
        std::io::stdin().read_line(input)?;

        match input.trim().parse() {
            Ok(device_label) => return Ok(device_label),
            Err(e) => eprintln!("{e}"),
        }
    }
}

pub fn choose_human_handle(input: &mut String) -> anyhow::Result<HumanHandle> {
    loop {
        println!("Enter email:");
        input.clear();
        std::io::stdin().read_line(input)?;

        let email = input.trim().to_string();

        println!("Enter name:");
        input.clear();
        std::io::stdin().read_line(input)?;

        let name = input.trim();

        match HumanHandle::new(&email, name) {
            Ok(human_handle) => return Ok(human_handle),
            Err(e) => eprintln!("{e}"),
        }
    }
}

pub fn choose_sas_code(
    input: &mut String,
    sas_codes: &[SASCode],
    expected: &SASCode,
) -> anyhow::Result<()> {
    std::io::stdin().read_line(input)?;

    match sas_codes.get(input.trim().parse::<usize>()?) {
        Some(sas_code) if sas_code == expected => Ok(()),
        Some(_) => Err(anyhow::anyhow!("Invalid SAS code")),
        None => Err(anyhow::anyhow!("Invalid input")),
    }
}

pub fn choose_user_profile(input: &mut String) -> anyhow::Result<UserProfile> {
    println!("Which profile? (0, 1, 2)");
    println!(" 0 - {YELLOW}Standard{RESET}");
    println!(" 1 - {YELLOW}Admin{RESET}");
    println!(" 2 - {YELLOW}Outsider{RESET}");
    loop {
        input.clear();
        std::io::stdin().read_line(input)?;

        match input.trim() {
            "0" => return Ok(UserProfile::Standard),
            "1" => return Ok(UserProfile::Admin),
            "2" => return Ok(UserProfile::Outsider),
            _ => eprintln!("Invalid input, choose between 0, 1 or 2"),
        }
    }
}

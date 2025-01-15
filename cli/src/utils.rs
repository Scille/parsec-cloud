// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{fmt::Display, ops::Deref, path::Path, sync::Arc};

use libparsec::{
    internal::{Client, EventBus},
    list_available_devices, AuthenticatedCmds, AvailableDevice, DeviceAccessStrategy,
    DeviceFileType, DeviceLabel, HumanHandle, LocalDevice, Password, ProxyConfig, SASCode,
    UserProfile,
};
use libparsec_platform_ipc::{
    lock_device_for_use, try_lock_device_for_use, InUseDeviceLockGuard, TryLockDeviceForUseError,
};
use spinners::{Spinner, Spinners, Stream};

/// Environment variable to set the Parsec config directory
/// Should not be confused with [`libparsec::PARSEC_BASE_CONFIG_DIR`]
pub const PARSEC_CONFIG_DIR: &str = "PARSEC_CONFIG_DIR";
/// Environment variable to set the Parsec data directory
/// Should not be confused with [`libparsec::PARSEC_BASE_DATA_DIR`]
pub const PARSEC_DATA_DIR: &str = "PARSEC_DATA_DIR";

pub const GREEN: &str = "\x1B[92m";
pub const RED: &str = "\x1B[91m";
pub const RESET: &str = "\x1B[39m";
pub const YELLOW: &str = "\x1B[33m";
pub const GREEN_CHECKMARK: &str = "\x1B[92m✔\x1B[39m";
pub const BULLET_CHAR: &str = "•";

pub fn format_devices(
    devices: &[AvailableDevice],
    mut f: impl std::fmt::Write,
) -> std::fmt::Result {
    let short_id_size = get_minimal_short_id_size(devices.iter().map(|d| &d.device_id));

    for device in devices {
        let short_id = &device.device_id.hex()[..short_id_size];
        let organization_id = &device.organization_id;
        let human_handle = &device.human_handle;
        let device_label = &device.device_label;
        writeln!(
            f,
            "{YELLOW}{short_id}{RESET} - {organization_id}: {human_handle} @ {device_label}"
        )?;
    }

    Ok(())
}

/// The minimal short id size, even if we could go lower,
/// we want to keep a lower bound for the size of the short id to keep it simple to select the value via double click.
pub(crate) const MINIMAL_SHORT_ID_SIZE: usize = 3;
/// The maximal short id size, could not go higher since it's the size of the device id in hex format.
pub(crate) const MAXIMAL_SHORT_ID_SIZE: usize = 32;

pub fn get_minimal_short_id_size<'a, I>(devices: I) -> usize
where
    I: Iterator<Item = &'a libparsec::DeviceID> + Clone,
{
    use itertools::Itertools;

    devices
        // Generate combinations of the ids
        .tuple_combinations::<(_, _)>()
        // Find the first different bit between the two ids:
        // We use the fact that we will display the id in hex format to compare both ids using the bitwise XOR and count the leading zeros.
        // Since it takes 4 bits to represent a hex digit, we divide the leading zeros by 4 to get the position of the first different hex digit.
        .map(|(a, b)| {
            let cmp = a.as_u128() ^ b.as_u128();
            let position = cmp.leading_zeros() / 4;
            position as usize + 1
        })
        // Find the max value or fallback to 1 (meaning we have only one device)
        .max()
        .unwrap_or(MINIMAL_SHORT_ID_SIZE)
        .clamp(MINIMAL_SHORT_ID_SIZE, MAXIMAL_SHORT_ID_SIZE)
}

#[derive(Debug)]
pub enum LoadDeviceError {
    /// No device found for the given prefix device ID
    DeviceNotFound {
        short_dev_id: String,
        devices: Vec<AvailableDevice>,
    },
    /// Multiple devices found for the same prefix device ID
    MultipleDevicesFound {
        short_dev_id: String,
        devices: Vec<AvailableDevice>,
    },
    /// The user did not provide a device ID with the option `--device=<DEVICE_ID>
    MissingDeviceOption(Vec<AvailableDevice>),
}

impl std::error::Error for LoadDeviceError {}

impl Display for LoadDeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadDeviceError::DeviceNotFound {
                short_dev_id: dev_id,
                devices,
            } => {
                writeln!(f, "Device `{dev_id}` not found, available devices:")?;
                format_devices(devices, f)?;
                Ok(())
            }
            LoadDeviceError::MultipleDevicesFound {
                short_dev_id: dev_id,
                devices,
            } => {
                writeln!(f, "Multiple devices found for `{dev_id}`:")?;
                format_devices(devices, f)?;
                Ok(())
            }
            LoadDeviceError::MissingDeviceOption(devices) => {
                writeln!(f, "Missing option '--device'\n")?;
                writeln!(f, "Available devices:")?;
                format_devices(devices, f)?;
                Ok(())
            }
        }
    }
}

pub async fn load_device_file(
    config_dir: &Path,
    device_short_id: Option<String>,
) -> Result<AvailableDevice, LoadDeviceError> {
    let devices = list_available_devices(config_dir).await;

    if let Some(device_short_id) = device_short_id {
        let possible_devices = devices
            .iter()
            .filter(|device| device.device_id.hex().starts_with(&device_short_id))
            .collect::<Vec<_>>();

        match possible_devices.len() {
            0 => Err(LoadDeviceError::DeviceNotFound {
                short_dev_id: device_short_id,
                devices,
            }),
            1 => Ok(possible_devices[0].clone()),
            _ => {
                let possible_devices = possible_devices
                    .into_iter()
                    .cloned()
                    .collect::<Vec<AvailableDevice>>();
                Err(LoadDeviceError::MultipleDevicesFound {
                    short_dev_id: device_short_id,
                    devices: possible_devices,
                })
            }
        }
    } else {
        Err(LoadDeviceError::MissingDeviceOption(devices))
    }
}

#[derive(Debug)]
pub enum LoadAndUnlockDeviceError {
    /// Error while loading the device file
    LoadDevice(LoadDeviceError),
    /// The device file authentication is not supported
    UnsupportedAuthentication(DeviceFileType),
    /// Error while unlocking the device
    UnlockDevice(libparsec::LoadDeviceError),
    /// Internal error
    Internal(anyhow::Error),
}

impl std::error::Error for LoadAndUnlockDeviceError {}

impl Display for LoadAndUnlockDeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadAndUnlockDeviceError::LoadDevice(e) => e.fmt(f),
            LoadAndUnlockDeviceError::UnsupportedAuthentication(ty) => {
                write!(f, "Unsupported device file authentication `{ty:?}`")
            }
            LoadAndUnlockDeviceError::UnlockDevice(e) => e.fmt(f),
            LoadAndUnlockDeviceError::Internal(e) => e.fmt(f),
        }
    }
}

impl From<anyhow::Error> for LoadAndUnlockDeviceError {
    fn from(e: anyhow::Error) -> Self {
        LoadAndUnlockDeviceError::Internal(e)
    }
}

impl From<libparsec::LoadDeviceError> for LoadAndUnlockDeviceError {
    fn from(e: libparsec::LoadDeviceError) -> Self {
        LoadAndUnlockDeviceError::UnlockDevice(e)
    }
}

impl From<LoadDeviceError> for LoadAndUnlockDeviceError {
    fn from(e: LoadDeviceError) -> Self {
        LoadAndUnlockDeviceError::LoadDevice(e)
    }
}

pub async fn load_and_unlock_device(
    config_dir: &Path,
    device: Option<String>,
    password_stdin: bool,
) -> Result<Arc<LocalDevice>, LoadAndUnlockDeviceError> {
    log::trace!(
        "Loading device {device} from {dir}",
        dir = config_dir.display(),
        device = device.as_deref().unwrap_or("N/A")
    );
    let device = load_device_file(config_dir, device).await?;

    log::debug!("Loading device {:?}", device.ty);

    let access_strategy = match device.ty {
        DeviceFileType::Password => {
            let password = read_password(if password_stdin {
                ReadPasswordFrom::Stdin
            } else {
                ReadPasswordFrom::Tty {
                    prompt: "Enter password for the device:",
                }
            })?;

            DeviceAccessStrategy::Password {
                key_file: device.key_file_path.clone(),
                password,
            }
        }
        DeviceFileType::Smartcard => DeviceAccessStrategy::Smartcard {
            key_file: device.key_file_path.clone(),
        },
        DeviceFileType::Keyring => DeviceAccessStrategy::Keyring {
            key_file: device.key_file_path.clone(),
        },
        DeviceFileType::Recovery => {
            return Err(LoadAndUnlockDeviceError::UnsupportedAuthentication(
                device.ty,
            ));
        }
    };

    let device = libparsec::load_device(config_dir, &access_strategy).await?;

    Ok(device)
}

pub async fn load_cmds(
    config_dir: &Path,
    device: Option<String>,
    password_stdin: bool,
) -> anyhow::Result<(AuthenticatedCmds, Arc<LocalDevice>)> {
    let device = load_and_unlock_device(config_dir, device, password_stdin).await?;
    let cmds = AuthenticatedCmds::new(config_dir, device.clone(), ProxyConfig::new_from_env()?)?;

    Ok((cmds, device))
}

pub async fn load_client(
    config_dir: &Path,
    device: Option<String>,
    password_stdin: bool,
) -> anyhow::Result<Arc<StartedClient>> {
    let device = load_and_unlock_device(config_dir, device, password_stdin).await?;
    let client = start_client(device).await?;

    Ok(client)
}

pub async fn load_client_with_config(
    config_dir: &Path,
    device: Option<String>,
    password_stdin: bool,
    config: libparsec_client::ClientConfig,
) -> anyhow::Result<Arc<StartedClient>> {
    let device = load_and_unlock_device(config_dir, device, password_stdin).await?;
    let client = start_client_with_config(device, config).await?;

    Ok(client)
}

pub struct StartedClient {
    pub client: Arc<Client>,
    pub event_bus: EventBus,
    _device_in_use_guard: InUseDeviceLockGuard,
}

impl Deref for StartedClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

pub async fn start_client(device: Arc<LocalDevice>) -> anyhow::Result<Arc<StartedClient>> {
    let config = default_client_config();

    start_client_with_config(device, config).await
}

pub fn default_client_config() -> libparsec_client::ClientConfig {
    libparsec::ClientConfig {
        with_monitors: false,
        ..Default::default()
    }
    .into()
}

pub async fn start_client_with_config(
    device: Arc<LocalDevice>,
    config: libparsec_client::ClientConfig,
) -> anyhow::Result<Arc<StartedClient>> {
    let device_in_use_guard = match try_lock_device_for_use(&config.config_dir, device.device_id) {
        Ok(device_in_use_guard) => device_in_use_guard,
        Err(TryLockDeviceForUseError::AlreadyInUse) => {
            let mut handle =
                start_spinner("Device already used by another process, waiting...".into());
            let device_in_use_guard =
                lock_device_for_use(&config.config_dir, device.device_id).await?;
            handle.stop_with_message(format!("{GREEN_CHECKMARK} Device accessed"));
            device_in_use_guard
        }
        Err(TryLockDeviceForUseError::Internal(err)) => return Err(err),
    };

    let event_bus = EventBus::default();
    let client = Client::start(Arc::new(config), event_bus.clone(), device).await?;

    Ok(Arc::new(StartedClient {
        client,
        event_bus,
        _device_in_use_guard: device_in_use_guard,
    }))
}

pub fn start_spinner(text: String) -> Spinner {
    Spinner::with_stream(Spinners::Dots, text, Stream::Stdout)
}

pub fn choose_password(from: ReadPasswordFrom) -> anyhow::Result<Password> {
    match from {
        ReadPasswordFrom::Stdin => {
            let stdin = std::io::stdin();
            rpassword::read_password_from_bufread(&mut stdin.lock())
                .map(Into::into)
                .map_err(anyhow::Error::from)
        }
        ReadPasswordFrom::Tty { prompt } => loop {
            let password = rpassword::prompt_password(prompt)?.into();
            let confirm_password = rpassword::prompt_password("Confirm password:")?.into();

            if password == confirm_password {
                return Ok(password);
            } else {
                eprintln!("Password mismatch")
            }
        },
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

pub enum ReadPasswordFrom {
    Stdin,
    Tty { prompt: &'static str },
}

pub fn read_password(read_from: ReadPasswordFrom) -> anyhow::Result<libparsec::Password> {
    match read_from {
        ReadPasswordFrom::Stdin => {
            let stdin = std::io::stdin();
            rpassword::read_password_from_bufread(&mut stdin.lock())
        }
        ReadPasswordFrom::Tty { prompt } => rpassword::prompt_password(prompt),
    }
    .map(Into::into)
    .map_err(anyhow::Error::from)
}

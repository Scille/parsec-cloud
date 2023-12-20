// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use clap::Args;
use std::{
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
};

use libparsec::{
    authenticated_cmds::latest::{
        device_create::{self, DeviceCreateRep},
        user_create::{self, UserCreateRep},
    },
    load_device, AuthenticatedCmds, BackendAddr, BackendOrganizationBootstrapAddr, Bytes,
    CertificateSignerOwned, ClientConfig, DateTime, DeviceAccessStrategy, DeviceCertificate,
    DeviceLabel, DeviceName, HumanHandle, LocalDevice, MaybeRedacted, OrganizationID, ProxyConfig,
    SigningKey, UserCertificate, UserProfile, PARSEC_CONFIG_DIR, PARSEC_DATA_DIR, PARSEC_HOME_DIR,
};

use crate::{
    bootstrap_organization::bootstrap_organization_req,
    create_organization::create_organization_req, utils::*,
};

const DEFAULT_ADMINISTRATION_TOKEN: &str = "s3cr3t";
const DEFAULT_DEVICE_PASSWORD: &str = "test";
const RESERVED_PORT_OFFSET: u16 = 1024;
const AVAILABLE_PORT_COUNT: u16 = u16::MAX - RESERVED_PORT_OFFSET;
const LAST_SERVER_PID: &str = "LAST_SERVER_ID";
pub const TESTBED_SERVER_URL: &str = "TESTBED_SERVER_URL";

#[derive(Args)]
pub struct RunTestenv {
    /// Sourced script file
    #[arg(long)]
    source_file: Option<PathBuf>,
    /// Main process id.
    /// When this process stops, the server will be automatically killed
    #[arg(long)]
    main_process_id: u32,
    /// Skip initialization
    #[arg(short, long, default_value_t)]
    empty: bool,
}

/// Maps a process id to a port number
///
/// Tests are run parallelized, so multiple servers are running on localhost.
/// Using the PID allows to retrieve the port in tests without having to store it.
pub fn process_id_to_port(pid: u32) -> u16 {
    // The first ports are reserved, so let's take an available port
    pid as u16 % AVAILABLE_PORT_COUNT + RESERVED_PORT_OFFSET
}

fn create_new_device(
    new_device: Arc<LocalDevice>,
    author: Arc<LocalDevice>,
    now: DateTime,
) -> (Bytes, Bytes) {
    let device_cert = DeviceCertificate {
        author: CertificateSignerOwned::User(author.device_id.clone()),
        timestamp: now,
        device_id: new_device.device_id.clone(),
        device_label: MaybeRedacted::Real(new_device.device_label.clone()),
        verify_key: new_device.verify_key(),
    };

    let device_certificate = device_cert.dump_and_sign(&author.signing_key).into();

    let redacted_device_cert = device_cert.into_redacted();

    let redacted_device_certificate = redacted_device_cert
        .dump_and_sign(&author.signing_key)
        .into();

    (device_certificate, redacted_device_certificate)
}

fn create_new_user(
    new_device: Arc<LocalDevice>,
    author: Arc<LocalDevice>,
    initial_profile: UserProfile,
    now: DateTime,
) -> (Bytes, Bytes) {
    let user_cert = UserCertificate {
        author: CertificateSignerOwned::User(author.device_id.clone()),
        timestamp: now,
        user_id: new_device.device_id.user_id().clone(),
        human_handle: MaybeRedacted::Real(new_device.human_handle.clone()),
        profile: initial_profile,
        public_key: new_device.public_key().clone(),
    };

    let user_certificate = user_cert.dump_and_sign(&author.signing_key).into();

    let redacted_user_cert = user_cert.into_redacted();

    let redacted_user_certificate = redacted_user_cert.dump_and_sign(&author.signing_key).into();

    (user_certificate, redacted_user_certificate)
}

async fn register_new_device(
    client_config: &ClientConfig,
    cmds: &AuthenticatedCmds,
    author: Arc<LocalDevice>,
    device_label: DeviceLabel,
) -> anyhow::Result<Arc<LocalDevice>> {
    let new_device = Arc::new(LocalDevice {
        organization_addr: author.organization_addr.clone(),
        initial_profile: author.initial_profile,
        human_handle: author.human_handle.clone(),
        device_label,
        device_id: format!("{}@{}", author.user_id(), DeviceName::default())
            .parse()
            .expect("Unreachable"),
        signing_key: SigningKey::generate(),
        private_key: author.private_key.clone(),
        user_realm_id: author.user_realm_id,
        time_provider: author.time_provider.clone(),
        user_realm_key: author.user_realm_key.clone(),
        local_symkey: author.local_symkey.clone(),
    });
    let now = author.now();

    let (device_certificate, redacted_device_certificate) =
        create_new_device(new_device.clone(), author, now);

    match cmds
        .send(device_create::Req {
            device_certificate,
            redacted_device_certificate,
        })
        .await
    {
        Ok(rep) => match rep {
            DeviceCreateRep::Ok => (),
            _ => return Err(anyhow::anyhow!("Cannot create device: {rep:?}")),
        },
        Err(e) => return Err(anyhow::anyhow!("{e}")),
    }

    let key_file = libparsec::get_default_key_file(&client_config.config_dir, &new_device);

    let access = DeviceAccessStrategy::Password {
        key_file,
        password: DEFAULT_DEVICE_PASSWORD.to_string().into(),
    };

    libparsec::save_device(Path::new(""), &access, &new_device).await?;

    Ok(new_device)
}

async fn register_new_user(
    client_config: &ClientConfig,
    cmds: &AuthenticatedCmds,
    author: Arc<LocalDevice>,
    device_label: DeviceLabel,
    human_handle: HumanHandle,
    initial_profile: UserProfile,
) -> anyhow::Result<Arc<LocalDevice>> {
    let new_device = Arc::new(LocalDevice::generate_new_device(
        cmds.addr().clone(),
        initial_profile,
        human_handle,
        device_label,
        None,
        None,
        None,
    ));
    let now = author.now();

    let (user_certificate, redacted_user_certificate) =
        create_new_user(new_device.clone(), author.clone(), initial_profile, now);
    let (device_certificate, redacted_device_certificate) =
        create_new_device(new_device.clone(), author, now);

    match cmds
        .send(user_create::Req {
            device_certificate,
            redacted_device_certificate,
            redacted_user_certificate,
            user_certificate,
        })
        .await
    {
        Ok(rep) => match rep {
            UserCreateRep::Ok => (),
            _ => return Err(anyhow::anyhow!("Cannot create user: {rep:?}")),
        },
        Err(e) => return Err(anyhow::anyhow!("{e}")),
    }

    let key_file = libparsec::get_default_key_file(&client_config.config_dir, &new_device);

    let access = DeviceAccessStrategy::Password {
        key_file,
        password: DEFAULT_DEVICE_PASSWORD.to_string().into(),
    };

    libparsec::save_device(Path::new(""), &access, &new_device).await?;

    Ok(new_device)
}

pub async fn initialize_test_organization(
    client_config: ClientConfig,
    addr: BackendAddr,
) -> anyhow::Result<[Arc<LocalDevice>; 3]> {
    let organization_id: OrganizationID = "Org".parse().expect("Unreachable");

    // Create organization
    let bootstrap_token =
        create_organization_req(&organization_id, &addr, DEFAULT_ADMINISTRATION_TOKEN).await?;

    let organization_addr =
        BackendOrganizationBootstrapAddr::new(addr, organization_id, Some(bootstrap_token));

    // Bootstrap organization and Alice user and create device "laptop" for Alice
    let alice_device = bootstrap_organization_req(
        client_config.clone(),
        organization_addr,
        "laptop".parse().expect("Unreachable"),
        HumanHandle::new("alice@example.com", "Alice").expect("Unreachable"),
        DEFAULT_DEVICE_PASSWORD.to_string().into(),
    )
    .await?;

    let access = DeviceAccessStrategy::Password {
        key_file: client_config
            .config_dir
            .join(format!("devices/{}.keys", alice_device.slughash())),
        password: DEFAULT_DEVICE_PASSWORD.to_string().into(),
    };

    let alice_device = load_device(&client_config.config_dir, &access).await?;

    let cmds = AuthenticatedCmds::new(
        &client_config.config_dir,
        alice_device.clone(),
        ProxyConfig::default(),
    )?;

    // Create new device "pc" for Alice
    let other_alice_device = register_new_device(
        &client_config,
        &cmds,
        alice_device.clone(),
        "pc".parse().expect("Unreachable"),
    )
    .await?;

    // Invite Bob in organization
    let bob_device = register_new_user(
        &client_config,
        &cmds,
        alice_device.clone(),
        "laptop".parse().expect("Unreachable"),
        HumanHandle::new("bob@example.com", "Bob").expect("Unreachable"),
        UserProfile::Standard,
    )
    .await?;

    // Invite Toto in organization
    register_new_user(
        &client_config,
        &cmds,
        alice_device.clone(),
        "laptop".parse().expect("Unreachable"),
        HumanHandle::new("toto@example.com", "Toto").expect("Unreachable"),
        UserProfile::Outsider,
    )
    .await?;

    Ok([alice_device, other_alice_device, bob_device])
}

/// Setup the environment variables
/// Set stop_after_process to kill the server once the process will run down
pub async fn new_environment(
    tmp_dir: &Path,
    source_file: Option<PathBuf>,
    stop_after_process: u32,
    empty: bool,
) -> anyhow::Result<()> {
    let port = process_id_to_port(stop_after_process);
    let tmp_dir_str = tmp_dir.to_str().expect("Unreachable");
    let _ = std::fs::create_dir_all(&tmp_dir);

    let cargo_manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set");
    let manifest_dir_path = Path::new(&cargo_manifest_dir);

    #[cfg(target_family = "windows")]
    let (export, mut env) = (
        "set",
        vec![
            (
                TESTBED_SERVER_URL,
                format!("parsec://127.0.0.1:{port}?no_ssl=true"),
            ),
            (PARSEC_HOME_DIR, format!("{tmp_dir_str}\\cache")),
            (PARSEC_DATA_DIR, format!("{tmp_dir_str}\\share")),
            (PARSEC_CONFIG_DIR, format!("{tmp_dir_str}\\config")),
        ],
    );

    #[cfg(target_family = "unix")]
    let (export, mut env) = (
        "export",
        vec![
            (
                TESTBED_SERVER_URL,
                format!("parsec://127.0.0.1:{port}?no_ssl=true"),
            ),
            (PARSEC_HOME_DIR, format!("{tmp_dir_str}/cache")),
            (PARSEC_DATA_DIR, format!("{tmp_dir_str}/share")),
            (PARSEC_CONFIG_DIR, format!("{tmp_dir_str}/config")),
        ],
    );

    if !empty {
        if let Ok(last_server_id) = std::env::var(LAST_SERVER_PID) {
            #[cfg(target_family = "windows")]
            Command::new("taskkill")
                .args(["/F", "/T", "/PID", &last_server_id])
                .output()?;

            #[cfg(target_family = "unix")]
            Command::new("kill").args([&last_server_id]).output()?;
        }

        // Run testbed server
        let child = Command::new("poetry")
            .args([
                "run",
                "python",
                "tests/scripts/run_testbed_server.py",
                "--stop-after-process",
                &stop_after_process.to_string(),
                "--port",
                &port.to_string(),
            ])
            .current_dir(
                manifest_dir_path
                    .parent()
                    .expect("Unreachable")
                    .join("server"),
            )
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .spawn()?;

        let id = child.id();

        let stdout = child.stdout.expect("Unreachable");
        let mut reader = BufReader::new(stdout);
        let mut buf = String::new();

        while reader.read_line(&mut buf).is_ok() {
            if buf.contains("All set !") {
                #[cfg(target_os = "windows")]
                std::thread::sleep(std::time::Duration::from_millis(500));
                break;
            }
            buf.clear();
        }

        println!(
            "Running server with the process id: {YELLOW}{id}{RESET} on port: {YELLOW}{port}{RESET}"
        );

        env.push((LAST_SERVER_PID, id.to_string()));
    }

    if let Some(source_file) = source_file {
        println!("Your environment will be configured with the following commands:");
        let mut buf = String::new();

        for (key, value) in &env {
            let export_key_value = format!("{export} {key}={value}\n");
            buf.push_str(&export_key_value);
        }

        std::fs::write(source_file, buf)?;
    } else {
        println!("[Warning] This code has not been sourced.");
        println!("Please configure your environment with the following commands:");
    }

    for (key, value) in env {
        // We set var for the current process
        std::env::set_var(key, &value);

        println!("   {export} {key}={value}");
    }

    Ok(())
}

pub async fn run_testenv(run_testenv: RunTestenv) -> anyhow::Result<()> {
    let tmp_dir = std::env::temp_dir().join(format!("parsec-testenv-{}", &uuid::Uuid::new_v4()));

    new_environment(
        &tmp_dir,
        run_testenv.source_file,
        run_testenv.main_process_id,
        run_testenv.empty,
    )
    .await?;

    if !run_testenv.empty {
        let port = process_id_to_port(run_testenv.main_process_id);

        let [alice_device, other_alice_device, bob_device] = initialize_test_organization(
            ClientConfig::default(),
            BackendAddr::new("127.0.0.1".into(), Some(port), false),
        )
        .await?;

        println!("Alice & Bob devices (password: {YELLOW}{DEFAULT_DEVICE_PASSWORD}{RESET}):");
        println!(
            "- {YELLOW}{}{RESET} // Alice",
            &alice_device.slughash()[..3]
        );
        println!(
            "- {YELLOW}{}{RESET} // Alice 2nd device",
            &other_alice_device.slughash()[..3]
        );
        println!("- {YELLOW}{}{RESET} // Bob", &bob_device.slughash()[..3]);
    }

    Ok(())
}

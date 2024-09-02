use std::{
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
};

use libparsec::{
    authenticated_cmds::{latest::device_create, v4::user_create},
    AuthenticatedCmds, Bytes, CertificateSignerOwned, ClientConfig, DateTime, DeviceAccessStrategy,
    DeviceCertificate, DeviceID, DeviceLabel, HumanHandle, LocalDevice, MaybeRedacted,
    OrganizationID, ParsecAddr, PrivateKeyAlgorithm, ProxyConfig, SigningKey, SigningKeyAlgorithm,
    UserCertificate, UserProfile, PARSEC_BASE_CONFIG_DIR, PARSEC_BASE_DATA_DIR,
    PARSEC_BASE_HOME_DIR, PARSEC_SCHEME,
};

use crate::{
    bootstrap_organization::bootstrap_organization_req,
    create_organization::create_organization_req,
    utils::{RESET, YELLOW},
};

pub const DEFAULT_ADMINISTRATION_TOKEN: &str = "s3cr3t";
pub(crate) const DEFAULT_DEVICE_PASSWORD: &str = "test";
const RESERVED_PORT_OFFSET: u16 = 1024;
const AVAILABLE_PORT_COUNT: u16 = u16::MAX - RESERVED_PORT_OFFSET;
const LAST_SERVER_PID: &str = "LAST_SERVER_ID";
pub const TESTBED_SERVER: &str = "TESTBED_SERVER";

pub struct TestOrganization {
    pub alice: Arc<LocalDevice>,
    pub other_alice: Arc<LocalDevice>,
    pub bob: Arc<LocalDevice>,
    pub toto: Arc<LocalDevice>,
}

pub async fn initialize_test_organization(
    client_config: ClientConfig,
    addr: ParsecAddr,
    organization_id: OrganizationID,
) -> anyhow::Result<TestOrganization> {
    // Create organization
    let organization_addr =
        create_organization_req(&organization_id, &addr, DEFAULT_ADMINISTRATION_TOKEN).await?;

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
        key_file: libparsec::get_default_key_file(
            &client_config.config_dir,
            &alice_device.device_id,
        ),
        password: DEFAULT_DEVICE_PASSWORD.to_string().into(),
    };

    let alice_device = libparsec::load_device(&client_config.config_dir, &access).await?;

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
    let toto_device = register_new_user(
        &client_config,
        &cmds,
        alice_device.clone(),
        "laptop".parse().expect("Unreachable"),
        HumanHandle::new("toto@example.com", "Toto").expect("Unreachable"),
        UserProfile::Outsider,
    )
    .await?;

    Ok(TestOrganization {
        alice: alice_device,
        other_alice: other_alice_device,
        bob: bob_device,
        toto: toto_device,
    })
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
        user_id: author.user_id,
        device_id: DeviceID::default(),
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
            device_create::DeviceCreateRep::Ok => (),
            _ => return Err(anyhow::anyhow!("Cannot create device: {rep:?}")),
        },
        Err(e) => return Err(anyhow::anyhow!("{e}")),
    }

    let key_file =
        libparsec::get_default_key_file(&client_config.config_dir, &new_device.device_id);

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
        None,
        None,
    ));
    log::trace!(
        "New user: user_id={}, human_handle={}, device_id={}",
        new_device.user_id,
        new_device.human_handle,
        new_device.device_id,
    );
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
            user_create::UserCreateRep::Ok => (),
            _ => return Err(anyhow::anyhow!("Cannot create user: {rep:?}")),
        },
        Err(e) => return Err(anyhow::anyhow!("{e}")),
    }

    let key_file =
        libparsec::get_default_key_file(&client_config.config_dir, &new_device.device_id);

    let access = DeviceAccessStrategy::Password {
        key_file,
        password: DEFAULT_DEVICE_PASSWORD.to_string().into(),
    };

    libparsec::save_device(Path::new(""), &access, &new_device).await?;

    Ok(new_device)
}

pub(crate) fn create_new_user(
    new_device: Arc<LocalDevice>,
    author: Arc<LocalDevice>,
    initial_profile: UserProfile,
    now: DateTime,
) -> (Bytes, Bytes) {
    let user_cert = UserCertificate {
        author: CertificateSignerOwned::User(author.device_id),
        timestamp: now,
        user_id: new_device.user_id,
        human_handle: MaybeRedacted::Real(new_device.human_handle.clone()),
        profile: initial_profile,
        public_key: new_device.public_key().clone(),
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
    };

    let user_certificate = user_cert.dump_and_sign(&author.signing_key).into();

    let redacted_user_cert = user_cert.into_redacted();

    let redacted_user_certificate = redacted_user_cert.dump_and_sign(&author.signing_key).into();

    (user_certificate, redacted_user_certificate)
}

pub(crate) fn create_new_device(
    new_device: Arc<LocalDevice>,
    author: Arc<LocalDevice>,
    now: DateTime,
) -> (Bytes, Bytes) {
    let device_cert = DeviceCertificate {
        author: CertificateSignerOwned::User(author.device_id),
        timestamp: now,
        user_id: new_device.user_id,
        device_id: new_device.device_id,
        device_label: MaybeRedacted::Real(new_device.device_label.clone()),
        verify_key: new_device.verify_key(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    };

    let device_certificate = device_cert.dump_and_sign(&author.signing_key).into();

    let redacted_device_cert = device_cert.into_redacted();

    let redacted_device_certificate = redacted_device_cert
        .dump_and_sign(&author.signing_key)
        .into();

    (device_certificate, redacted_device_certificate)
}

pub enum TestenvConfig {
    ConnectToServer(ParsecAddr),
    StartNewServer { stop_after_process: u32 },
}

/// Setup the environment variables
/// Set stop_after_process to kill the server once the process will run down
pub async fn new_environment(
    tmp_dir: &Path,
    source_file: Option<PathBuf>,
    config: TestenvConfig,
    empty: bool,
) -> anyhow::Result<Option<ParsecAddr>> {
    let _ = std::fs::create_dir_all(tmp_dir);

    let (export_keyword, mut env) = get_env_variables(tmp_dir);

    let url = match config {
        TestenvConfig::ConnectToServer(url) => {
            println!("Using testbed server: {YELLOW}{url}{RESET}");
            env.push((TESTBED_SERVER, url.to_string()));
            Some(url)
        }
        TestenvConfig::StartNewServer { stop_after_process } if !empty => {
            println!("Start a new server");
            if let Ok(last_server_id) = std::env::var(LAST_SERVER_PID) {
                kill_last_testbed_server(last_server_id)?;
            }

            let port_from_pid = process_id_to_port(stop_after_process);

            // Run testbed server
            let child = start_testbed_server(stop_after_process, port_from_pid)?;

            let id = child.id();

            println!(
                "Running server with the process id {YELLOW}{id}{RESET} on port {YELLOW}{port_from_pid}{RESET}"
            );

            let server_addr = ParsecAddr::new("127.0.0.1".into(), Some(port_from_pid), false);
            env.push((TESTBED_SERVER, server_addr.to_url().to_string()));
            env.push((LAST_SERVER_PID, id.to_string()));
            Some(server_addr)
        }
        _ => None,
    };

    if let Some(source_file) = source_file {
        println!("Your environment will be configured with the following commands:");
        let mut buf = String::new();

        for (key, value) in &env {
            let export_key_value = format!("{export_keyword} {key}={value}\n");
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

        println!("   {export_keyword} {key}={value}");
    }

    Ok(url)
}

fn get_env_variables(tmp_dir: &Path) -> (&'static str, Vec<(&'static str, String)>) {
    #[cfg(target_family = "windows")]
    return (
        "set",
        vec![
            (
                PARSEC_BASE_HOME_DIR,
                format!("{dir}\\cache", dir = tmp_dir.display()),
            ),
            (
                PARSEC_BASE_DATA_DIR,
                format!("{dir}\\share", dir = tmp_dir.display()),
            ),
            (
                PARSEC_BASE_CONFIG_DIR,
                format!("{dir}\\config", dir = tmp_dir.display()),
            ),
        ],
    );

    #[cfg(target_family = "unix")]
    return (
        "export",
        vec![
            (
                PARSEC_BASE_HOME_DIR,
                format!("{dir}/cache", dir = tmp_dir.display()),
            ),
            (
                PARSEC_BASE_DATA_DIR,
                format!("{dir}/share", dir = tmp_dir.display()),
            ),
            (
                PARSEC_BASE_CONFIG_DIR,
                format!("{dir}/config", dir = tmp_dir.display()),
            ),
        ],
    );
}

fn kill_last_testbed_server(last_server_id: String) -> anyhow::Result<()> {
    #[cfg(target_family = "windows")]
    Command::new("taskkill")
        .args(["/F", "/T", "/PID", &last_server_id])
        .output()?;
    #[cfg(target_family = "unix")]
    Command::new("kill").args([&last_server_id]).output()?;
    Ok(())
}

/// Maps a process id to a port number
///
/// Tests are run parallelized, so multiple servers are running on localhost.
/// Using the PID allows to retrieve the port in tests without having to store it.
pub fn process_id_to_port(pid: u32) -> u16 {
    // The first ports are reserved, so let's take an available port
    pid as u16 % AVAILABLE_PORT_COUNT + RESERVED_PORT_OFFSET
}

fn start_testbed_server(stop_after_process: u32, port: u16) -> anyhow::Result<std::process::Child> {
    let cargo_manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set");
    let manifest_dir_path = Path::new(&cargo_manifest_dir);

    let mut child = Command::new("poetry")
        .args([
            "run",
            "python",
            "-m",
            "parsec",
            "testbed",
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
        .spawn()
        .map_err(anyhow::Error::from)?;

    wait_testbed_server_to_be_ready(&mut child);

    Ok(child)
}

fn wait_testbed_server_to_be_ready(child: &mut std::process::Child) {
    let stdout = child.stdout.as_mut().expect("Unreachable");
    let mut reader = BufReader::new(stdout);
    let mut buf = String::new();

    while reader.read_line(&mut buf).is_ok() {
        if buf.contains("All set !") {
            #[cfg(target_os = "windows")]
            std::thread::sleep(std::time::Duration::from_millis(100));
            break;
        }
        buf.clear();
    }
}

pub fn parsec_addr_from_http_url(url: &str) -> ParsecAddr {
    let url = if url.starts_with("http://") {
        url.replacen("http", PARSEC_SCHEME, 1) + "?no_ssl=true"
    } else if url.starts_with("https://") {
        url.replacen("https", PARSEC_SCHEME, 1)
    } else if !url.starts_with("parsec3://") {
        format!("{PARSEC_SCHEME}://{url}")
    } else {
        url.to_string()
    };
    ParsecAddr::from_any(&url).expect("Invalid testbed url")
}

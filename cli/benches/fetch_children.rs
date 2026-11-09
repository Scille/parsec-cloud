// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! Measure `stat_folder_children` on a folder whose children are partially
//! (or not at all) available in the local storage.
//!
//! Each iteration starts from a fresh local storage, pre-populates it with a given
//! ratio of the children, then restarts the client (so that the in-memory cache is
//! always cold) before doing the measured `stat_folder_children`.

use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use criterion::{Criterion, SamplingMode, criterion_group, criterion_main};
use libparsec::{
    ClientConfig, FsPath, OrganizationID, PARSEC_BASE_CONFIG_DIR, PARSEC_BASE_DATA_DIR,
    PARSEC_BASE_HOME_DIR, ParsecAddr, TmpPath, VlobID,
};
use libparsec_types::LocalDevice;
use parsec_cli::testenv_utils::{
    TESTBED_SERVER, TestOrganization, TestenvConfig, initialize_test_organization, new_environment,
    parsec_addr_from_http_url,
};
use parsec_cli::utils::{default_client_config, start_client, start_client_with_config};

/// Number of files in the workspace's root folder
const FILES_COUNT: usize = 500;

/// Ratios (in percent) of the children already present in local storage before the measure
const CACHED_PERCENTS: [usize; 3] = [0, 90, 100];

struct Env {
    // Kept alive for the whole bench since it contains the server config & device files
    _tmp_path: TmpPath,
    device: Arc<LocalDevice>,
    workspace_id: VlobID,
    files: Vec<FsPath>,
}

/// Create an organization with a workspace containing `FILES_COUNT` empty files in its
/// root folder, all of them being synchronized with the server.
async fn setup_env() -> Env {
    let tmp_path = TmpPath::create();

    let (_, TestOrganization { alice, .. }, _) =
        bootstrap_cli(&parsec_cli::ui::Ui::default(), &tmp_path)
            .await
            .unwrap();

    let client = start_client(alice.clone()).await.unwrap();
    let workspace_id = client
        .create_workspace("bench".parse().unwrap())
        .await
        .unwrap();
    client.ensure_workspaces_bootstrapped().await.unwrap();
    let workspace = client.start_workspace(workspace_id).await.unwrap();

    let files: Vec<FsPath> = (0..FILES_COUNT)
        .map(|i| FsPath::from_str(&format!("/{i}.txt")).unwrap())
        .collect();
    for path in &files {
        workspace.create_file(path.clone()).await.unwrap();
    }

    // Monitors are disabled in the CLI client, so we must sync by hand
    loop {
        let to_sync = workspace.get_need_outbound_sync(1000).await.unwrap();
        if to_sync.is_empty() {
            break;
        }
        for entry_id in to_sync {
            workspace.outbound_sync(entry_id).await.unwrap();
        }
    }

    drop(workspace);
    client.stop().await;
    drop(client);

    Env {
        _tmp_path: tmp_path,
        device: alice,
        workspace_id,
        files,
    }
}

fn client_config(data_dir: &Path) -> libparsec_client::ClientConfig {
    let mut config = default_client_config();
    config.data_base_dir = data_dir.to_owned();
    config
}

/// Prepare a fresh local storage containing the root manifest and the first
/// `cached_percent`% of the files' manifests.
async fn setup_local_storage(env: &Env, data_dir: &Path, cached_percent: usize) {
    let client = start_client_with_config(env.device.clone(), client_config(data_dir))
        .await
        .unwrap();
    // Fresh local storage: we know nothing about the workspace yet
    client.poll_server_for_new_certificates().await.unwrap();
    client.refresh_workspaces_list().await.unwrap();
    let workspace = client.start_workspace(env.workspace_id).await.unwrap();

    workspace.inbound_sync(env.workspace_id).await.unwrap();

    let cached_count = env.files.len() * cached_percent / 100;
    for path in &env.files[..cached_count] {
        // stat_entry forces a fetch + cache
        workspace.stat_entry(path).await.unwrap();
    }

    drop(workspace);
    client.stop().await;
}

fn benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    println!("Setting up {FILES_COUNT} files...");
    let env = rt.block_on(setup_env());
    println!("Setup done");

    let mut group = c.benchmark_group("fetch_children");
    // Each iteration is slow (up to a full download of the folder's children),
    // so keep the number of iterations as low as Criterion allows.
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(60));

    for cached_percent in CACHED_PERCENTS {
        group.bench_function(format!("{cached_percent}pct_cached"), |b| {
            b.to_async(&rt).iter_custom(|iters| {
                let env = &env;
                async move {
                    let mut total = Duration::ZERO;
                    for _ in 0..iters {
                        // Not measured: prepare the local storage
                        let data_dir = TmpPath::create();
                        setup_local_storage(env, &data_dir, cached_percent).await;

                        // Fresh client, so the in-memory cache is cold
                        let client =
                            start_client_with_config(env.device.clone(), client_config(&data_dir))
                                .await
                                .unwrap();
                        let workspace = client.start_workspace(env.workspace_id).await.unwrap();

                        // Measured
                        let start = Instant::now();
                        let children = workspace
                            .stat_folder_children(&"/".parse().unwrap())
                            .await
                            .unwrap();
                        total += start.elapsed();

                        assert_eq!(children.len(), env.files.len());

                        drop(workspace);
                        client.stop().await;
                    }
                    total
                }
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

async fn bootstrap_cli(
    ui: &parsec_cli::ui::Ui,
    tmp_path: &TmpPath,
) -> anyhow::Result<(ParsecAddr, TestOrganization, OrganizationID)> {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, devices, org_id) = run_local_organization(ui, tmp_path, None, config).await?;

    set_env(tmp_path_str, &url);
    Ok((url, devices, org_id))
}

fn get_testenv_config() -> TestenvConfig {
    if let Ok(testbed_server) = std::env::var("TESTBED_SERVER") {
        TestenvConfig::ConnectToServer(parsec_addr_from_http_url(&testbed_server))
    } else {
        TestenvConfig::StartNewServer {
            stop_after_process: std::process::id(),
        }
    }
}

async fn run_local_organization(
    ui: &parsec_cli::ui::Ui,
    tmp_dir: &Path,
    source_file: Option<PathBuf>,
    config: TestenvConfig,
) -> anyhow::Result<(ParsecAddr, TestOrganization, OrganizationID)> {
    let url = new_environment(ui, tmp_dir, source_file, config, false)
        .await?
        .unwrap();

    println!("Initializing test organization to {url}");
    let org_id = OrganizationID::from_str(&format!(
        "TestOrg-{}",
        &uuid::Uuid::new_v4().as_hyphenated().to_string()[..24]
    ))?;
    initialize_test_organization(ClientConfig::default(), url.clone(), org_id.clone())
        .await
        .map(|v| (url, v, org_id))
}

fn set_env(tmp_dir: &str, url: &ParsecAddr) {
    #[expect(clippy::undocumented_unsafe_blocks)]
    unsafe {
        std::env::set_var(TESTBED_SERVER, url.to_url().to_string());
        std::env::set_var(PARSEC_BASE_HOME_DIR, format!("{tmp_dir}/cache"));
        std::env::set_var(PARSEC_BASE_DATA_DIR, format!("{tmp_dir}/share"));
        std::env::set_var(PARSEC_BASE_CONFIG_DIR, format!("{tmp_dir}/config"));
        // Hidden environ variable only used for CLI testing to customize the throttling time
        // in invitation polling, without that the tests would be much slower for no reason.
        std::env::set_var("_PARSEC_INVITE_POLLING_THROTTLE_MS", "10");

        // Remove the PARSEC_* variables that will conflict with the CLI tests
        std::env::remove_var("PARSEC_DEVICE_ID");
        std::env::remove_var("PARSEC_WORKSPACE_ID");
        std::env::remove_var("PARSEC_ORGANISATION_ID");
        std::env::remove_var("PARSEC_SERVER_ADDR");
        std::env::remove_var("PARSEC_ADMINISTRATION_TOKEN");
        std::env::remove_var("PARSEC_CONFIG_DIR");
        std::env::remove_var("PARSEC_DATA_DIR");
        std::env::remove_var(env_logger::DEFAULT_FILTER_ENV);
    }
}

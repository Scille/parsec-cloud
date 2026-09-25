use criterion::{criterion_group, criterion_main, Criterion};
use libparsec::{
    ClientConfig, FsPath, OpenOptions, OrganizationID, ParsecAddr, TmpPath, PARSEC_BASE_CONFIG_DIR,
    PARSEC_BASE_DATA_DIR, PARSEC_BASE_HOME_DIR,
};
use libparsec_client::WorkspaceOps;
use libparsec_types::LocalDevice;
use parsec_cli::testenv_utils::{
    initialize_test_organization, new_environment, parsec_addr_from_http_url, TestOrganization,
    TestenvConfig, TESTBED_SERVER,
};
use parsec_cli::utils::start_client;
use rand::prelude::*;
use rand::rngs::StdRng;
use std::sync::Arc;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    str::FromStr,
};

struct Params {
    max_depth: usize,
    min_depth: usize,
    max_files_per_dir: u64,
    min_files_per_dir: u64,

    max_dir_per_dir: u64,
    min_dir_per_dir: u64,
    max_file_size: u64,
    min_file_size: u64,
}

impl Params {
    fn small() -> Params {
        Params {
            max_depth: 3,
            min_depth: 0,
            max_files_per_dir: 3,
            min_files_per_dir: 0,
            max_dir_per_dir: 3,
            min_dir_per_dir: 0,
            max_file_size: 5,
            min_file_size: 1,
        }
    }

    fn lots_of_tiny_files() -> Params {
        Params {
            max_depth: 100,
            min_depth: 50,
            max_files_per_dir: 1000,
            min_files_per_dir: 500,
            max_dir_per_dir: 50,
            min_dir_per_dir: 20,
            max_file_size: 4096 * 10,
            min_file_size: 0,
        }
    }

    fn few_big_files() -> Params {
        Params {
            max_depth: 10,
            min_depth: 0,
            max_files_per_dir: 5,
            min_files_per_dir: 1,
            max_dir_per_dir: 5,
            min_dir_per_dir: 0,
            max_file_size: 4096 * 1_000_000_000,
            min_file_size: 4096 * 1_000_000,
        }
    }

    fn extreme() -> Params {
        Params {
            max_depth: 100,
            min_depth: 50,
            max_files_per_dir: 1000,
            min_files_per_dir: 500,
            max_dir_per_dir: 100,
            min_dir_per_dir: 10,
            max_file_size: 4096 * 1_000_000_000,
            min_file_size: 4096 * 1_000_000,
        }
    }
}

async fn setup(
    params: Params,
) -> (
    Arc<LocalDevice>,
    Arc<WorkspaceOps>,
    HashSet<PathBuf>,
    HashSet<(PathBuf, Vec<u8>)>,
) {
    let tmp_path = TmpPath::create();
    let mut rng = StdRng::seed_from_u64(2);

    // Generate local files to upload
    let (folders, files) = random_tree_generator(&mut rng, &tmp_path, params);

    // Setup organization & local client

    let (_, TestOrganization { alice, bob, .. }, _) =
        bootstrap_cli(&parsec_cli::ui::Ui::default(), &tmp_path)
            .await
            .unwrap();

    // setup workspace

    let workspace = {
        let alice_client = start_client(alice.clone()).await.unwrap();

        // Create the workspace used to copy the file to
        let wid = alice_client
            .create_workspace("new-workspace".parse().unwrap())
            .await
            .unwrap();
        alice_client.ensure_workspaces_bootstrapped().await.unwrap();
        let workspace = alice_client.start_workspace(wid).await.unwrap();

        alice_client
            .share_workspace(wid, bob.user_id, Some(libparsec::RealmRole::Reader))
            .await
            .unwrap();

        alice_client.stop().await;

        workspace
    };
    (alice, workspace, folders, files)
}

async fn upload(
    alice: Arc<LocalDevice>,
    workspace: Arc<WorkspaceOps>,
    folders: HashSet<PathBuf>,
    files: HashSet<(PathBuf, Vec<u8>)>,
) {
    let alice_client = start_client(alice.clone()).await.unwrap();
    for folder in folders {
        workspace.create_folder(into_fs_path(folder)).await.unwrap();
    }
    for (path, content) in files {
        let path = into_fs_path(path);
        let _ = workspace.create_file(path.clone()).await.unwrap();
        let fd = workspace
            .open_file(path, OpenOptions::read_write())
            .await
            .unwrap();
        workspace.fd_write(fd, 0, &content).await.unwrap();
    }
    alice_client.stop().await;
}

fn criterion_benchmark(c: &mut Criterion) {
    let params = vec![
        Params::small(),
        Params::lots_of_tiny_files(),
        Params::few_big_files(),
        Params::extreme(),
    ];
    let names = vec!["small", "lots_of_tiny_files", "few_big_files", "extreme"];

    for (param, name) in params.into_iter().zip(names) {
        println!("starting {name}");
        let (alice, workspace, folders, files) = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { setup(param).await });
        println!("{name} setup");
        c.bench_function(&format!("upload_speed {}", name), |b| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| async {
                    upload(
                        alice.clone(),
                        workspace.clone(),
                        folders.clone(),
                        files.clone(),
                    )
                    .await
                });
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn into_fs_path(path: PathBuf) -> FsPath {
    FsPath::from_str(path.to_str().unwrap()).unwrap()
}

fn random_tree_generator(
    rng: &mut StdRng,
    root_dir: &PathBuf,
    params: Params,
) -> (HashSet<PathBuf>, HashSet<(PathBuf, Vec<u8>)>) {
    let mut current_depth_paths: HashSet<_> = vec![root_dir].into_iter().cloned().collect();
    let mut next_depth_paths = HashSet::new();
    let mut folders = HashSet::new();
    let mut files = HashSet::new();

    // iter on current depth
    for _ in 0..rng.gen_range(params.min_depth..params.max_depth) {
        // iter on all folders for a given depth
        while let Some(current_dir) = current_depth_paths.iter().next() {
            // generate new dirs
            let dir_count = rng.gen_range(params.min_dir_per_dir..params.max_dir_per_dir);
            let new_folders: HashSet<_> = (0..dir_count)
                .map(|_| current_dir.join(gen_filename(rng)))
                .collect();
            folders.extend(new_folders.clone());
            next_depth_paths.extend(new_folders);

            // generate new files
            let file_count = rng.gen_range(params.min_files_per_dir..params.max_files_per_dir);
            for _ in 0..file_count {
                let content =
                    random_content_generator(rng, params.min_file_size, params.max_file_size);
                files.insert((current_dir.join(gen_filename(rng)), content));
            }
        }
        // reset next and current depth paths
        current_depth_paths = std::mem::take(&mut next_depth_paths)
    }
    (folders, files)
}

fn gen_filename(rng: &mut StdRng) -> PathBuf {
    String::from_iter(rng.gen::<[char; 15]>()).into()
}

fn random_content_generator(rng: &mut StdRng, min_file_size: u64, max_file_size: u64) -> Vec<u8> {
    (0..rng.gen_range(min_file_size..max_file_size))
        .map(|_| rng.gen())
        .collect()
}

// the following function are in cli/test/integration/mod.rs
// consider moving them to testenv_utils

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

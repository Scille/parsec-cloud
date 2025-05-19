// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec::ClientConfig;

use crate::{
    testenv_utils::{
        DEFAULT_DEVICE_PASSWORD, TestenvConfig, initialize_test_organization, new_environment,
        parsec_addr_from_http_url,
    },
    utils::{RESET, YELLOW},
};

#[derive(clap::Parser)]
pub struct RunTestenv {
    /// Sourced script file
    #[arg(long)]
    source_file: Option<PathBuf>,
    /// Main process id.
    /// When this process stops, the server will be automatically killed
    #[arg(long)]
    main_process_id: Option<u32>,
    /// Skip initialization
    #[arg(short, long, default_value_t)]
    empty: bool,
    /// The organization ID to use for the test environment.
    #[arg(default_value_os = "Org")]
    organization: libparsec::OrganizationID,
}

pub async fn run_testenv(run_testenv: RunTestenv) -> anyhow::Result<()> {
    let RunTestenv {
        main_process_id,
        source_file,
        empty,
        organization,
    } = run_testenv;

    let tmp_dir = std::env::temp_dir().join(format!("parsec-testenv-{}", &uuid::Uuid::new_v4()));

    let testenv_config = match (main_process_id, std::env::var("TESTBED_SERVER")) {
        (_, Ok(testbed_server)) => {
            let url = parsec_addr_from_http_url(&testbed_server);
            TestenvConfig::ConnectToServer(url)
        }
        (Some(main_process_id), _) => TestenvConfig::StartNewServer {
            stop_after_process: main_process_id,
        },
        (None, Err(_)) => anyhow::bail!(concat!(
            "You must at least provide a main process id (via the CLI) ",
            "or set the testbed server url (via the env variable TESTBED_SERVER)"
        )),
    };

    let url = new_environment(&tmp_dir, source_file, testenv_config, empty).await?;

    if !empty {
        let url = url.expect("Mismatch condition in new_environment when starting a new server");
        let org = initialize_test_organization(ClientConfig::default(), url, organization).await?;

        println!("Alice & Bob devices (password: {YELLOW}{DEFAULT_DEVICE_PASSWORD}{RESET}):");
        println!(
            "- {YELLOW}{}{RESET} // Alice",
            &org.alice.device_id.hex()[..3]
        );
        println!(
            "- {YELLOW}{}{RESET} // Alice 2nd device",
            &org.other_alice.device_id.hex()[..3]
        );
        println!("- {YELLOW}{}{RESET} // Bob", &org.bob.device_id.hex()[..3]);
    }

    Ok(())
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{future::Future, io::Read, sync::Arc};

use lazy_static::lazy_static;

use libparsec_types::prelude::*;

use crate::{test_drop_testbed, test_new_testbed, TestbedEnv};

/// Keep track of testbed's lifetime, don't drop this before the test is done !
pub struct TestbedScope {
    pub env: Arc<TestbedEnv>,
}

pub enum Run {
    WithServer,
    WithoutServer,
}

impl TestbedScope {
    pub async fn start(template: &str, with_server: Run) -> Option<Self> {
        let server_addr = match with_server {
            Run::WithServer => match TESTBED_SERVER.0.as_ref() {
                // Here we will skip if TESTBED_SERVER env is not set !
                None => return None,
                addr => addr,
            },
            Run::WithoutServer => None,
        };
        let env = test_new_testbed(template, server_addr).await;
        Some(Self { env })
    }

    pub async fn stop(self) {
        test_drop_testbed(&self.env.discriminant_dir).await;
    }

    pub async fn run_with_server<F, Fut>(template: &str, cb: F)
    where
        F: FnOnce(Arc<TestbedEnv>) -> Fut,
        Fut: Future<Output = ()>,
    {
        // Here we will skip if the server is not configured
        if let Some(scope) = Self::start(template, Run::WithServer).await {
            // TODO: handle async panic
            cb(scope.env.clone()).await;
            // Note in case `cb` panics we won't be able to cleanup the testbed env :'(
            // This is "ok enough" considering:
            // - Using `std::panic::{catch_unwind, resume_unwind}` is cumbersome with async closure
            // - Panic only occurs when a test fails, so at most once per run (unless
            //   `--keep-going` is used, but that's a corner case ^^).
            // Hence leak should be small: no leak if the testbed server has been started by us,
            // leak until the 10mn auto-garbage collection otherwise.
            scope.stop().await;
        }
    }

    pub async fn run<F, Fut>(template: &str, cb: F)
    where
        F: FnOnce(Arc<TestbedEnv>) -> Fut,
        Fut: Future<Output = ()>,
    {
        let scope = Self::start(template, Run::WithoutServer)
            .await
            .expect("run without server is never skipped");
        // TODO: handle async panic
        cb(scope.env.clone()).await;
        // Note in case `cb` panics we won't be able to cleanup the testbed env :'(
        // This is "ok enough" considering:
        // - Using `std::panic::{catch_unwind, resume_unwind}` is cumbersome with async closure
        // - Panic only occurs when a test fails, so at most once per run (unless
        //   `--keep-going` is used, but that's a corner case ^^).
        // Hence leak should be small: no leak if the testbed server has been started by us,
        // leak until the 10mn auto-garbage collection otherwise.
        scope.stop().await;
    }
}

/// Testbed server is lazily started when it is first needed, and then shared
/// across all tests.
/// Alternatively, `TESTBED_SERVER` environ variable can be passed to use an
/// external server.
fn ensure_testbed_server_is_started() -> (Option<BackendAddr>, Option<std::process::Child>) {
    // TESTBED_SERVER should have 3 possible values:
    // - not set, if so the test is skipped (don't forget to print something about it !)
    // - `AUTOSTART` or `1` or just defined to empty: auto start the server
    // - `parsec://<domain>:<port>[?no_ssl=true]`,
    //   `<domain>:<port>`,
    //   `http://<domain>:<port>` or
    //   `https://<domain>:<port>` consider the server is already started
    let testbed_server = std::env::var("TESTBED_SERVER");

    if let Err(e) = testbed_server {
        // That means the env variable isn't set
        log::info!("{e}, so the test is skipped !");
        return (None, None);
    }

    let testbed_server = &testbed_server.unwrap();

    if testbed_server != "AUTOSTART" && testbed_server != "1" && !testbed_server.is_empty() {
        let testbed_server = if testbed_server.starts_with("http://") {
            testbed_server.replacen("http", "parsec", 1) + "?no_ssl=true"
        } else if testbed_server.starts_with("https://") {
            testbed_server.replacen("https", "parsec", 1)
        } else if !testbed_server.starts_with("parsec://") {
            String::from("parsec://") + testbed_server
        } else {
            testbed_server.into()
        };

        let addr = BackendAddr::from_any(&testbed_server)
            .expect("Invalid value in `TESTBED_SERVER` environ variable");
        log::info!("Using already started testbed server at {}", &addr);
        return (Some(addr), None);
    }

    log::info!("Starting testbed server...");
    // We don't really know where we are executed from given it depend on what is
    // the currently tested crate. Hence we only make the assumption we are within
    // the project repository and will eventually reach the project root by testing
    // each parent.
    let script = {
        let mut dir = std::env::current_dir().expect("Cannot retrieve current dir");
        loop {
            let target = dir.join("./tests/scripts/run_testbed_server.py");
            if target.exists() {
                break target;
            }
            dir = match dir.parent() {
                Some(dir) => dir.to_path_buf(),
                None => panic!("Cannot retrieve run_testbed_server.py script !"),
            }
        }
    };

    let mut process = std::process::Command::new("poetry")
        .args([
            "run",
            "--",
            "python",
            script.to_str().expect("Script path contains non-utf8"),
            // Ask the server to stop itself once our own process is finished,
            // this Rust test harness has not global teardown hook that could
            // allow us to do a `process.kill()`.
            "--stop-after-process",
            &std::process::id().to_string(),
        ])
        // We make the server quiet to avoid polluting test output, if you need
        // server log you should start your own server in another terminal and
        // provide it config with the `TESTBED_SERVER` environ variable.
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Cannot start the testbed server");

    // Now we have to retrieve the port the server is listening on...
    // TODO: does it really have to be that complicated ???
    let port = process
        .stderr
        .take()
        .map(|mut stderr| {
            let re = regex::Regex::new(r"127.0.0.1:([0-9]+)").unwrap();
            let mut buf = vec![0u8; 1024];
            let mut total = 0;
            loop {
                let n = stderr
                    .read(&mut buf[total..])
                    .expect("Cannot read testbed server process's stderr");
                total += n;
                let strbuf = std::str::from_utf8(&buf[..total])
                    .expect("testbed server process's stderr contains non-utf8");
                match re.captures(strbuf) {
                    Some(cap) => {
                        // Replace stderr pipe to avoid closing it on drop
                        process.stderr.replace(stderr);
                        break cap
                            .get(1)
                            .expect("unreachable")
                            .as_str()
                            .parse::<u16>()
                            .expect("Invalid testbed server port");
                    }
                    None => {
                        // Not enough output from subprocess, wait 100ms more
                        std::thread::sleep(std::time::Duration::new(0, 100000));
                    }
                }
                if total == buf.len() {
                    // In theory we should increase buf size, but if we are here it
                    // is most likely something went wrong anyway
                    panic!("Cannot retrieve port of testbed server, stderr: {}", strbuf);
                }
            }
        })
        .expect("unreachable");

    log::info!("Testbed server running on 127.0.0.1:{}", port);
    (
        Some(BackendAddr::new("127.0.0.1".to_owned(), Some(port), false)),
        Some(process),
    )
}

// Given static variable are never drop, we cannot handle started testbed server
// lifetime with a struct implementing Drop.
// On top of that Rust test has no global teardown hook hence we are left with no
// way to cleanly close the testbed server once all the tests are done...
// So instead we pass the PID of our own process and ask the testbed server to
// watch for it and close itself accordingly.
// Note we still have to keep reference of the child handle, this is to avoid it
// stderr pipe to be closed which would break subprocess logging.
lazy_static! {
    static ref TESTBED_SERVER: (Option<BackendAddr>, Option<std::process::Child>) =
        ensure_testbed_server_is_started();
}

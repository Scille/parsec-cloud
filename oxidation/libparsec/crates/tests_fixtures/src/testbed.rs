// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use lazy_static::lazy_static;
use std::{io::Read, sync::Arc};

use libparsec_types::*;
// Re-expose
pub use libparsec_testbed::*;

/// Keep track of testbed's lifetime, don't drop this before the test is done !
pub struct TestbedScope {
    pub env: Arc<TestbedEnv>,
}

impl TestbedScope {
    pub async fn start(template: &str, with_server: bool) -> Self {
        let server_addr = if with_server {
            Some(&*TESTBED_SERVER_ADDR)
        } else {
            None
        };
        let env = test_new_testbed(template, server_addr).await;
        Self { env }
    }
    pub async fn stop(self) {
        test_drop_testbed(&self.env.client_config_dir).await;
    }
}

// In theory we'd rather have a `TestbedScope::run(async |env| { ... })` method,
// but async closure are currently unstable, hence we have to rely on this macro instead
// TODO: it would be better to replace this by a `#[testbed("coolorg")]` proc macro
// attribute on top of the async function.
#[macro_export]
macro_rules! run_in_testbed {
    ($template:literal, $env:ident => $cb:expr) => {
        run_in_testbed!($template, false, $env => $cb)
    };
    ($template:literal, $with_server:expr, $env:ident => $cb:expr) => {
        async {
            let scope = ::libparsec_tests_fixtures::TestbedScope::start($template, $with_server).await;
            // TODO: handle async panic
            let $env: &::libparsec_tests_fixtures::TestbedEnv = scope.env.as_ref();
            $cb.await;
            // Note in case `cb` panics we won't be able to cleanup the testbed env :'(
            // This is "ok enough" considering:
            // - Using `std::panic::{catch_unwind, resume_unwind}` is cumbersome with async closure
            // - Panic only occurs when a test fails, so at most once per run (unless
            //   `--keep-going` is used, but that's a corner case ^^).
            // Hence leak should be small: no leak if the testbed server has been started by us,
            // leak until the 10mn auto-garbage collection otherwise.
            scope.stop().await;
        }
    };
}
#[macro_export]
macro_rules! run_in_testbed_with_server {
    ($template:literal, $env:ident => $cb:expr) => {
        run_in_testbed!($template, true, $env => $cb)
    };
}
pub use run_in_testbed;
pub use run_in_testbed_with_server;

/// Testbed server is lazily started when it is first needed, and then shared
/// across all tests.
/// Alternatively, `TESTBED_SERVER` environ variable can be passed to use an
/// external server.
fn ensure_testbed_server_is_started() -> BackendAddr {
    if let Ok(raw) = std::env::var("TESTBED_SERVER") {
        let addr = BackendAddr::from_any(&raw)
            .expect("Invalid value in `TESTBED_SERVER` environ variable");
        println!("Using already started testbed server at {}", &addr);
        return addr;
    }

    println!("Starting testbed server...");
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
        // We make the server quiet to avoid poluting test output, if you need
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
                        break cap
                            .get(1)
                            .expect("unreachable")
                            .as_str()
                            .parse::<u16>()
                            .expect("Invalid testbed server port");
                    }
                    None => {
                        // Not enough stuff on stdout, wait 100ms more
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

    println!("Testbed server running on 127.0.0.1:{}", port);
    BackendAddr::new("127.0.0.1".to_owned(), Some(port), false)
}

// Given static variable are never drop, we cannot handle started testbed server
// lifetime with a struct implementing Drop.
// On top of that Rust test has no global teardown hook hence we are left with no
// way to cleanly close the testbed server once all the tests are done...
// So instead we pass the PID of our own process and ask the testbed server to
// watch for it and close itself accordingly.
lazy_static! {
    static ref TESTBED_SERVER_ADDR: BackendAddr = ensure_testbed_server_is_started();
}

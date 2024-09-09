// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

// The maximum number of attempts to wait for a peer to be ready.
// Note that we only retry if the other peer has cancelled the ongoing
// greeting attempt by starting a new one (which is done automatically)
// This number should be high enough to cover the cases where the peer
// cancels the greeting attempt while the nonces are being exchanged.
// However, it shouldn't be too high to prevent the other peer from
// being able re-roll the nonces too many times, for security reasons.
// Any value between 4 and 32 should match those criteria.
pub const WAIT_PEER_MAX_ATTEMPTS: usize = 8;

// The default throttle duration is too slow when testing the CLI, hence this environ
// variable than allows us to customize it (typically setting it to 10ms during tests)
//
// Note the `_` prefix and the fact this environ variable is not documented: it is only
// here for testing the CLI.
// This is because the CLI's tests run the default CLI binary (i.e. the test code is
// first compiled and run by `cargo test`, and in turn runs the CLI binary to do the tests).
// That means we cannot use time provider to mock time (since the CLI binary is not
// compiled in debug, and since the test code cannot communicate with a time provider
// in the CLI binary).
//
// Also note we don't use a feature-flag given this would further add complexity to
// the steps to obtain a valid CLI binary and do the CLI tests.
const CUSTOMIZE_THROTTLE_ENVIRON_VARIABLE: &str = "_PARSEC_INVITE_POLLING_THROTTLE_MS";

// The throttle duration is defined to 1 second.
// This value limits the amount of polling done by the peer
// to 1 requests per second.
const THROTTLE_DEFAULT_DURATION: Duration = Duration::milliseconds(1000);

pub struct Throttle<'a> {
    time_provider: &'a TimeProvider,
    last_call: Option<DateTime>,
    throttle_duration: Duration,
}

impl<'a> Throttle<'a> {
    pub fn new(time_provider: &'a TimeProvider) -> Self {
        let throttle_duration = std::env::var(CUSTOMIZE_THROTTLE_ENVIRON_VARIABLE)
            .ok()
            // Parse as u32 to avoid corner cases with negative numbers
            // and overflow when converting to i64.
            .and_then(|raw| raw.parse::<u32>().ok())
            .map(|raw| Duration::milliseconds(raw as i64))
            .unwrap_or(THROTTLE_DEFAULT_DURATION);

        Self {
            time_provider,
            last_call: None,
            throttle_duration,
        }
    }

    pub async fn throttle(&mut self) {
        if let Some(last_call) = self.last_call {
            let duration = last_call + self.throttle_duration - self.time_provider.now();
            self.time_provider.sleep(duration).await;
        }
        self.last_call = Some(self.time_provider.now());
    }
}

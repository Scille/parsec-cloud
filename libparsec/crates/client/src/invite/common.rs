// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

// The throttle duration is defined to 100 ms.
// This value limits the amount of polling done by the peer
// to 10 requests per second. This is deliberately fast in order
// improve user experience and to accelerate the tests. However,
// this might be too much for the server and this value might need
// to be adjusted for production. In this case, a decoupling between
// testing and production might be necessary. This could be achieved
// by mocking the time provider passed to the throttler.
static THROTTLE_DURATION: Duration = Duration::milliseconds(100);

pub struct Throttle<'a> {
    time_provider: &'a TimeProvider,
    last_call: Option<DateTime>,
}

impl<'a> Throttle<'a> {
    pub fn new(time_provider: &'a TimeProvider) -> Self {
        Self {
            time_provider,
            last_call: None,
        }
    }

    pub async fn throttle(&mut self) {
        if let Some(last_call) = self.last_call {
            let duration = last_call + THROTTLE_DURATION - self.time_provider.now();
            self.time_provider.sleep(duration).await;
        }
        self.last_call = Some(self.time_provider.now());
    }
}

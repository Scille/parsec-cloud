// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::time::Duration;

pub struct Limited {
    /// How much time we tried to backoff between a reset
    attempt: usize,
    duration: Duration,
}

impl Limited {
    pub const fn new() -> Self {
        Self {
            attempt: 0,
            duration: Duration::from_secs(1),
        }
    }

    pub fn reset(&mut self) {
        self.attempt = 0;
    }

    pub fn set_retry(&mut self, duration: Duration) {
        self.duration = duration;
    }
}

impl Limited {
    pub async fn wait(&mut self) {
        let duration_to_wait = self.get_duration_to_wait();
        self.attempt += 1;
        libparsec_platform_async::sleep(duration_to_wait).await;
    }

    fn get_duration_to_wait(&self) -> Duration {
        let factor = match self.attempt {
            0 => 0,
            1 => 1,
            2 => 5,
            3 => 10,
            4 => 30,
            _ => 60,
        };
        self.duration.mul_f64(f64::from(factor))
    }
}

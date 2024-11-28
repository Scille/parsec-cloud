// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

std::thread_local! {
    static PERFORMANCE: web_sys::Performance = web_sys::window().expect("always exist").performance().expect("always exist");
}

pub struct Instant {
    origin: f64,
}

impl Instant {
    #[inline(always)]
    pub fn now() -> Instant {
        let origin = PERFORMANCE.with(|performance| performance.now());
        Instant { origin }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        let now = PERFORMANCE.with(|performance| performance.now());
        std::time::Duration::from_secs_f64(now - self.origin)
    }
}

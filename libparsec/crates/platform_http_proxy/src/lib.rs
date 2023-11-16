// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;

use libparsec_types::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{HTTPS_PROXY, HTTP_PROXY};

#[derive(Default, Debug, Clone)]
pub struct ProxyConfig {
    #[cfg(not(target_arch = "wasm32"))]
    http_proxy: Option<reqwest::Proxy>,
    #[cfg(not(target_arch = "wasm32"))]
    https_proxy: Option<reqwest::Proxy>,
    #[cfg(target_os = "windows")]
    pac_url: Option<widestring::U16CString>,
}

impl ProxyConfig {
    /// Create a new proxy config with value which is initialized using the env see [ProxyConfig::with_env].
    /// It's an alias over `ProxyConfig::default().with_env()`.
    pub fn new_from_env() -> anyhow::Result<Self> {
        Self::default().with_env()
    }

    /// Will configure the proxy using the environment variables:
    ///
    /// - `HTTP_PROXY` env var. will be used to configure the http proxy.
    /// - `HTTPS_PROXY` env var. will be used to configure the https proxy.
    ///
    /// On `wasm32` this function will do nothing has we cant have access to the env variables since we will be run in a web navigator context.
    pub fn with_env(self) -> anyhow::Result<Self> {
        self.with_env_internal()
    }
}

impl ProxyConfig {
    /// Configure the http client builder with the proxy configuration,
    ///
    /// On `wasm32` this will do nothing as `wasm32` `reqwest` don't expose `Proxy`
    /// And we likely prefer use the web navigator to handle the proxy configuration.
    // TODO: In the future we would like to provide an `url` alongside the build to handle the proxy pac.
    pub fn configure_http_client(&self, builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        self.configure_http_client_internal(builder)
    }
}

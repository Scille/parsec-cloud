// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct ProxyConfig {
    #[cfg(not(target_arch = "wasm32"))]
    http_proxy: Option<reqwest::Proxy>,
    #[cfg(not(target_arch = "wasm32"))]
    https_proxy: Option<reqwest::Proxy>,
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

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use reqwest::Proxy;

    use super::*;

    #[cfg(not(test))]
    pub const HTTP_PROXY: &str = "HTTP_PROXY";
    #[cfg(test)]
    pub const HTTP_PROXY: &str = "TEST_HTTP_PROXY";
    #[cfg(not(test))]
    pub const HTTPS_PROXY: &str = "HTTPS_PROXY";
    #[cfg(test)]
    pub const HTTPS_PROXY: &str = "TEST_HTTPS_PROXY";

    impl ProxyConfig {
        /// Set the http proxy to use, will overwrite the previous configuration.
        pub(crate) fn with_http_proxy(mut self, proxy: String) -> anyhow::Result<Self> {
            let proxy = Proxy::http(proxy)
                .map_err(|e| anyhow::anyhow!("Invalid HTTP proxy configuration: {}", e))?;
            self.http_proxy.replace(proxy);
            Ok(self)
        }

        /// Set the https proxy to use, will overwrite the previous configuration.
        pub(crate) fn with_https_proxy(mut self, proxy: String) -> anyhow::Result<Self> {
            let proxy = Proxy::https(proxy)
                .map_err(|e| anyhow::anyhow!("Invalid HTTPS proxy configuration: {}", e))?;
            self.https_proxy.replace(proxy);
            Ok(self)
        }

        pub(crate) fn with_env_internal(self) -> anyhow::Result<Self> {
            let cfg = self;

            let cfg = if let Ok(proxy) = std::env::var(crate::HTTP_PROXY) {
                cfg.with_http_proxy(proxy)?
            } else {
                cfg
            };

            let cfg = if let Ok(proxy) = std::env::var(crate::HTTPS_PROXY) {
                cfg.with_https_proxy(proxy)?
            } else {
                cfg
            };

            Ok(cfg)
        }

        pub(crate) fn configure_http_client_internal(
            &self,
            mut builder: reqwest::ClientBuilder,
        ) -> reqwest::ClientBuilder {
            builder = if let Some(http_proxy) = self.http_proxy.clone() {
                builder.proxy(http_proxy)
            } else {
                builder
            };

            builder = if let Some(https_proxy) = self.https_proxy.clone() {
                builder.proxy(https_proxy)
            } else {
                builder
            };

            builder
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::{HTTPS_PROXY, HTTP_PROXY};

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;

    impl ProxyConfig {
        pub(crate) fn with_env_internal(self) -> anyhow::Result<Self> {
            Ok(self)
        }

        pub(crate) fn configure_http_client_internal(
            &self,
            builder: reqwest::ClientBuilder,
        ) -> reqwest::ClientBuilder {
            builder
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../tests/unit/tests.rs"]
#[allow(clippy::unwrap_used)]
mod tests;

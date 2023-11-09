// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(target_os = "windows")]
mod windows;

use reqwest::Proxy;

use libparsec_types::anyhow;

use crate::ProxyConfig;

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

        #[cfg(target_os = "windows")]
        let cfg = cfg.with_register()?;

        Ok(cfg)
    }
}

#[cfg(test)]
#[path = "../../tests/unit/native.rs"]
#[allow(clippy::unwrap_used)]
mod tests;

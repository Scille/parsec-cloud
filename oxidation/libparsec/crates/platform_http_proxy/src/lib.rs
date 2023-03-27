// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;

#[cfg(not(test))]
const HTTP_PROXY: &str = "HTTP_PROXY";
#[cfg(test)]
const HTTP_PROXY: &str = "TEST_HTTP_PROXY";
#[cfg(not(test))]
const HTTPS_PROXY: &str = "HTTPS_PROXY";
#[cfg(test)]
const HTTPS_PROXY: &str = "TEST_HTTPS_PROXY";

#[derive(Default)]
pub struct ProxyConfig {
    http_proxy: Option<String>,
    https_proxy: Option<String>,
}

impl ProxyConfig {
    /// Create a new proxy config with value which is initialized using the env see [ProxyConfig::with_env].
    /// It's an alias over `ProxyConfig::default().with_env()`.
    pub fn new_from_env() -> Self {
        Self::default().with_env()
    }

    /// Will configure the proxy using the environment variables:
    ///
    /// - `HTTP_PROXY` env var. will be used to configure the http proxy.
    /// - `HTTPS_PROXY` env var. will be used to configure the https proxy.
    ///
    /// On `wasm32` this function will do nothing has we cant have access to the env variables since we will be run in a web navigator context.
    pub fn with_env(self) -> Self {
        self.with_env_internal()
    }

    /// Set the http proxy to use, will overwrite the previous configuration.
    fn with_http_proxy<S: AsRef<str>>(mut self, proxy: S) -> Self {
        self.http_proxy.replace(proxy.as_ref().to_string());
        self
    }

    /// Set the https proxy to use, will overwrite the previous configuration.
    fn with_https_proxy<S: AsRef<str>>(mut self, proxy: S) -> Self {
        self.https_proxy.replace(proxy.as_ref().to_string());
        self
    }
}

impl ProxyConfig {
    /// Configure the http client builder with the proxy configuration,
    ///
    /// On `wasm32` this will do nothing has `wasm32` `reqwest` don't expose `Proxy`
    /// And we likely prefer use the web navigator to handle the proxy configuration.
    // TODO: In the future we would like to provide an `url` alongside the build to handle the proxy pac.
    pub fn configure_http_client(
        &self,
        builder: reqwest::ClientBuilder,
    ) -> reqwest::Result<reqwest::ClientBuilder> {
        self.configure_http_client_internal(builder)
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod not_wasm32 {
    use std::{iter::Chain, option::IntoIter};

    use reqwest::Proxy;

    use super::ProxyConfig;

    impl ProxyConfig {
        pub(crate) fn with_env_internal(self) -> Self {
            let cls = if let Ok(proxy) = std::env::var(crate::HTTP_PROXY) {
                self.with_http_proxy(proxy)
            } else {
                self
            };

            if let Ok(proxy) = std::env::var(crate::HTTPS_PROXY) {
                cls.with_https_proxy(proxy)
            } else {
                cls
            }
        }

        pub(crate) fn configure_http_client_internal(
            &self,
            mut builder: reqwest::ClientBuilder,
        ) -> reqwest::Result<reqwest::ClientBuilder> {
            for proxy in self.get_proxies() {
                builder = builder.proxy(proxy?);
            }

            Ok(builder)
        }

        pub(crate) fn get_proxies(
            &self,
        ) -> Chain<IntoIter<reqwest::Result<Proxy>>, IntoIter<reqwest::Result<Proxy>>> {
            self.http_proxy
                .as_ref()
                .map(Proxy::http)
                .into_iter()
                .chain(self.https_proxy.as_ref().map(Proxy::https).into_iter())
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use super::ProxyConfig;

    impl ProxyConfig {
        pub(crate) fn with_env_internal(self) -> Self {
            self
        }

        pub(crate) fn configure_http_client_internal(
            &self,
            builder: reqwest::ClientBuilder,
        ) -> reqwest::Result<reqwest::ClientBuilder> {
            Ok(builder)
        }
    }
}

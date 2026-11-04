// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use libparsec_types::anyhow;

use crate::ProxyConfig;

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

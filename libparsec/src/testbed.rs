// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::{Path, PathBuf};

use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum TestbedError {
    #[error("Test features are disabled")]
    Disabled,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[cfg(feature = "test-utils")]
pub async fn test_new_testbed(
    template: &str,
    server_addr: Option<&ParsecAddr>,
) -> Result<PathBuf, TestbedError> {
    let env = libparsec_testbed::test_new_testbed(template, server_addr).await?;

    Ok(env.discriminant_dir.clone())
}

#[cfg(not(feature = "test-utils"))]
pub async fn test_new_testbed(
    _template: &str,
    _server_addr: Option<&ParsecAddr>,
) -> Result<PathBuf, TestbedError> {
    Err(TestbedError::Disabled)
}

#[cfg(feature = "test-utils")]
pub fn test_get_testbed_organization_id(
    discriminant_dir: &Path,
) -> Result<Option<OrganizationID>, TestbedError> {
    Ok(
        libparsec_testbed::test_get_testbed(discriminant_dir)
            .map(|env| env.organization_id.clone()),
    )
}

#[cfg(not(feature = "test-utils"))]
pub fn test_get_testbed_organization_id(
    _discriminant_dir: &Path,
) -> Result<Option<OrganizationID>, TestbedError> {
    Err(TestbedError::Disabled)
}

#[cfg(feature = "test-utils")]
pub fn test_get_testbed_bootstrap_organization_addr(
    discriminant_dir: &Path,
) -> Result<Option<ParsecOrganizationBootstrapAddr>, TestbedError> {
    Ok(
        libparsec_testbed::test_get_testbed(discriminant_dir).map(|env| {
            ParsecOrganizationBootstrapAddr::new(
                env.server_addr.clone(),
                env.organization_id.clone(),
                None,
            )
        }),
    )
}

#[cfg(not(feature = "test-utils"))]
pub fn test_get_testbed_bootstrap_organization_addr(
    _discriminant_dir: &Path,
) -> Result<Option<ParsecOrganizationBootstrapAddr>, TestbedError> {
    Err(TestbedError::Disabled)
}

#[cfg(feature = "test-utils")]
pub async fn test_drop_testbed(discriminant_dir: &Path) -> Result<(), TestbedError> {
    libparsec_testbed::test_drop_testbed(discriminant_dir)
        .await
        .map_err(TestbedError::Internal)
}

#[cfg(not(feature = "test-utils"))]
pub async fn test_drop_testbed(_discriminant_dir: &Path) -> Result<(), TestbedError> {
    Err(TestbedError::Disabled)
}

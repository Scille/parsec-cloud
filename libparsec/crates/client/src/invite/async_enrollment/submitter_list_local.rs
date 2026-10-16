// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::path::Path;

use libparsec_platform_device_loader::ListPendingAsyncEnrollmentsError;
pub use libparsec_platform_device_loader::{
    AvailablePendingAsyncEnrollment, AvailablePendingAsyncEnrollmentIdentitySystem,
};

pub type SubmitterListLocalAsyncEnrollmentsError = ListPendingAsyncEnrollmentsError;

pub async fn submitter_list_local_async_enrollments(
    config_dir: &Path,
) -> Result<Vec<AvailablePendingAsyncEnrollment>, SubmitterListLocalAsyncEnrollmentsError> {
    libparsec_platform_device_loader::list_pending_async_enrollments(config_dir).await
}

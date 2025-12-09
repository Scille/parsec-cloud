// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_types::prelude::*;

pub type SubmitterForgetAsyncEnrollmentError = libparsec_platform_device_loader::RemoveDeviceError;

pub async fn submitter_forget_async_enrollment(
    config_dir: &Path,
    enrollment_id: AsyncEnrollmentID,
) -> Result<(), SubmitterForgetAsyncEnrollmentError> {
    let path = libparsec_platform_device_loader::get_default_pending_async_enrollment_file(
        config_dir,
        enrollment_id,
    );
    libparsec_platform_device_loader::remove_pending_async_enrollment(config_dir, &path).await
}

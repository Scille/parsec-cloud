// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::AsyncEnrollmentID;

use crate::ClientConfig;

pub type SubmitterDestroyAsyncEnrollmentError = libparsec_platform_device_loader::RemoveDeviceError;

pub async fn submitter_destroy_async_enrollment(
    config: &ClientConfig,
    enrollment_id: AsyncEnrollmentID,
) -> Result<(), SubmitterDestroyAsyncEnrollmentError> {
    let path = libparsec_platform_device_loader::get_default_pending_async_enrollment_file(
        &config.config_dir,
        enrollment_id,
    );
    libparsec_platform_device_loader::remove_device(&config.config_dir, &path).await
}

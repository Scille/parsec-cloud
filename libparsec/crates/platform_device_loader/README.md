The goal of this crate is to manage devices and device-like concepts
(such as `PendingAsyncEnrollment` and `AvailablePendingAsyncEnrollment`)

This crate is for the most part platform agnostic except for:
- PKI authentication (not supported on web)
- keyring authentication (not supported on web)
- default directories for config, database and mountpoint

This is susceptible to change.

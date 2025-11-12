// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

// API major versions:
// v1: Original API
// v2 (Parsec 1.14+): Incompatible handshake with system with SAS-based authentication
// - v2.7 (Parsec +2.9): Add `organization_bootstrap` to anonymous commands
// - v2.8 (Parsec 2.11+): Sequester API
// v3 (Parsec 2.9+): Incompatible handshake challenge answer format
// - v3.1 (Parsec 2.10+): Add `user_revoked` return status to `realm_update_role` command
// - v3.2 (Parsec 2.11+): Sequester API
// v4 (Parsec 3.0+): `certificate_get` command & `certificate_updated` event
// - v4.1 (Parsec 3.2+):
//   * Add `ShamirRecovery` variants to `invite_list` and `invite_info`
//   * Add `invite_new_shamir_recovery` command
// v5 (Parsec 3.3+):
// - v5.0 (Parsec 3.3+):
//   * Incompatible handling of sequester in `vlob_create` and `vlob_update`
//   * Incompatible changes to `invite_info` to allow for multiple greeters during a user invitation
//   * Support for Shamir recovery commands, invitation and greeting procedure
//   * Incompatible change to `vlob_update` to add `realm_id` field as request parameter
// - v5.1 (Parsec 3.5+):
//   * Add `allowed_client_agent` field to `events_listen` response
//   * Add `account_vault_strategy` field to `events_listen` response
// - v5.2 (Parsec 3.6+):
//   * Incompatible change in SASCode algorithm and shared secret key. This doesn't actually
//     impact the protocol (as SASCode and shared secret key are computed client-side),
//     however a server supporting API 5.2 will reject any `(invite|greeter)_claimer_step`
//     made by clients with API < 5.2.
// - v5.3 (Parsec 3.7+):
//   * Remove `allowed_client_agent/account_vault_strategy` fields from `events_listen` response
//     (compatible change behavior is reverted to what Parsec server < v5.1 does).
//   * Incompatible change to remove `allowed_client_agent/account_vault_strategy` fields
//     from `organization_self_list` (we consider this acceptable though since the account
//     APIs is not yet used in production).
//   * Incompatible change to rename `/anonymous_account` route `/anonymous_server`
//     (we consider this acceptable though since the account APIs is not yet used in
//     production).
//   * Add `server_config` to anonymous server commands
pub const API_V1_VERSION: &ApiVersion = &ApiVersion {
    version: 1,
    revision: 3,
};
pub const API_V2_VERSION: &ApiVersion = &ApiVersion {
    version: 2,
    revision: 8,
};
pub const API_V3_VERSION: &ApiVersion = &ApiVersion {
    version: 3,
    revision: 2,
};
pub const API_V4_VERSION: &ApiVersion = &ApiVersion {
    version: 4,
    revision: 1,
};
pub const API_V5_VERSION: &ApiVersion = &ApiVersion {
    version: 5,
    revision: 3,
};
pub const API_LATEST_VERSION: &ApiVersion = API_V5_VERSION;

pub const API_LATEST_MAJOR_VERSION: u32 = API_LATEST_VERSION.version;

pub const fn api_version_major_to_full(major_version: u32) -> &'static ApiVersion {
    match major_version {
        1 => API_V1_VERSION,
        2 => API_V2_VERSION,
        3 => API_V3_VERSION,
        4 => API_V4_VERSION,
        5 => API_V5_VERSION,
        _ => panic!("Unknown major version"),
    }
}

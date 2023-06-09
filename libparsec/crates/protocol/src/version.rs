// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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
    revision: 0,
};
pub const API_LATEST_VERSION: &ApiVersion = API_V4_VERSION;

pub const API_LATEST_MAJOR_VERSION: u32 = API_LATEST_VERSION.version;
pub const fn api_version_major_to_full(major_version: u32) -> &'static ApiVersion {
    match major_version {
        1 => API_V1_VERSION,
        2 => API_V2_VERSION,
        3 => API_V3_VERSION,
        4 => API_V4_VERSION,
        _ => panic!("Unknown major version"),
    }
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::num::NonZeroUsize;

use crate::{OrganizationID, VlobID};

/// Editics sharding, used to dispatch the document edition sessions across
/// multiple editics server instances.
///
/// To do that:
/// - Each server must have the same configuration (so all servers must be
///   updated&restarted whenever the configuration changes).
/// - Servers are ordered in a list, then we run:
///
///   ```text
///   server_index = hash(orgID + realmID + docID) % len(servers)
///   ```
///
/// Sharding algorithm uses FNV-1a hash, this is because:
/// - It is dead simple and never overflows, which make implementation trivial
///   in any language (useful if we ever want to implement it in pure Javascript).
/// - Cryptographic hashes such as sha256 are overkill, require ugly logic
///   to do the final modulo operation, and are only available in secure
///   context on Javascript (see `window.crypto.subtle.digest`).
/// - Its distribution is good enough (it used to be the algorithm used
///   in the Python's hash function), especially since the input contains
///   UUIDs.
/// - DOS attack risk is low: a client attacker controls the document ID,
///   so he is able to choose which editics server to connect to, however
///   actual connection requires authentication.
///
/// Finally this whole sharding logic will probably evolve to better support
/// dynamic scaling (currently adding/removing editics server requires to
/// restart all servers, and even without that the hash logic means that ~90%
/// of the document sessions have to change server in this case).
/// So we stay KISS for now.
pub fn editics_compute_sharding_index(
    servers: NonZeroUsize,
    organization_id: &OrganizationID,
    realm_id: VlobID,
    document_id: VlobID,
) -> usize {
    let data = organization_id
        .as_ref()
        .as_bytes()
        .iter()
        .chain(realm_id.as_bytes())
        .chain(document_id.as_bytes());

    // FNV-1a 32bits (given the hash is 32bits, the number of editics servers
    // cannot grow beyond 2^32 anyway, which is more than enough !)
    let mut h: u32 = 0x811C9DC5;
    for b in data {
        h = (h ^ (*b as u32)).wrapping_mul(0x01000193);
    }

    (h as usize) % servers
}

#[cfg(test)]
#[path = "../tests/unit/editics.rs"]
mod tests;

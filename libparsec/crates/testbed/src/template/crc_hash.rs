// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `std::hash::Hash` is not stable accross platforms (e.g. it uses native
// endianness when hashing numbers), hence we have to roll our own system.

use std::sync::Arc;

use libparsec_types::prelude::*;

pub(super) trait CrcHash {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher);
}

impl CrcHash for [u8] {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(self);
    }
}

impl CrcHash for Bytes {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(self.as_ref());
    }
}

impl CrcHash for str {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(self.as_bytes());
    }
}

impl CrcHash for bool {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        let x = match self {
            true => b"1",
            false => b"0",
        };
        hasher.update(x);
    }
}

impl<T: CrcHash> CrcHash for Option<T> {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        match self {
            None => hasher.update(b"None"),
            Some(x) => {
                hasher.update(b"Some");
                x.crc_hash(hasher);
            }
        }
    }
}

impl<A: CrcHash, B: CrcHash> CrcHash for (A, B) {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"Tuple2");
        self.0.crc_hash(hasher);
        self.1.crc_hash(hasher);
    }
}

impl<T: CrcHash> CrcHash for Vec<T> {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"Vec");
        (self.len() as u32).crc_hash(hasher);
        for item in self {
            item.crc_hash(hasher)
        }
    }
}

impl<T: CrcHash> CrcHash for Arc<T> {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        (**self).crc_hash(hasher);
    }
}

macro_rules! impl_crc_hash_for_number {
    ($name:ident) => {
        impl CrcHash for $name {
            fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
                hasher.update(&self.to_le_bytes())
            }
        }
    };
}

// We don't implement crc hash for usize/isize given those types are
// not platform agnostic !
impl_crc_hash_for_number!(u8);
impl_crc_hash_for_number!(u16);
impl_crc_hash_for_number!(u32);
impl_crc_hash_for_number!(u64);
impl_crc_hash_for_number!(u128);
impl_crc_hash_for_number!(i8);
impl_crc_hash_for_number!(i16);
impl_crc_hash_for_number!(i32);
impl_crc_hash_for_number!(i64);
impl_crc_hash_for_number!(i128);

macro_rules! impl_crc_hash_for_str_based {
    ($name:ident) => {
        impl CrcHash for $name {
            fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
                self.as_ref().crc_hash(hasher)
            }
        }
    };
}

impl_crc_hash_for_str_based!(UserID);
impl_crc_hash_for_str_based!(DeviceID);
impl_crc_hash_for_str_based!(HumanHandle);
impl_crc_hash_for_str_based!(DeviceLabel);
impl_crc_hash_for_str_based!(EntryName);

macro_rules! impl_crc_hash_for_uuid_based {
    ($name:ident) => {
        impl CrcHash for $name {
            fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
                hasher.update(self.as_bytes())
            }
        }
    };
}

impl_crc_hash_for_uuid_based!(EntryID);
impl_crc_hash_for_uuid_based!(BlockID);
impl_crc_hash_for_uuid_based!(RealmID);
impl_crc_hash_for_uuid_based!(VlobID);
impl_crc_hash_for_uuid_based!(ChunkID);
impl_crc_hash_for_uuid_based!(SequesterServiceID);
impl_crc_hash_for_uuid_based!(InvitationToken);
impl_crc_hash_for_uuid_based!(EnrollmentID);

impl CrcHash for DateTime {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(&self.get_f64_with_us_precision().to_le_bytes());
    }
}

impl CrcHash for UserProfile {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        match self {
            UserProfile::Admin => hasher.update(b"Admin"),
            UserProfile::Standard => hasher.update(b"Standard"),
            UserProfile::Outsider => hasher.update(b"Outsider"),
        };
    }
}

impl CrcHash for RealmRole {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        match self {
            RealmRole::Owner => hasher.update(b"Owner"),
            RealmRole::Manager => hasher.update(b"Manager"),
            RealmRole::Contributor => hasher.update(b"Contributor"),
            RealmRole::Reader => hasher.update(b"Reader"),
        }
    }
}

impl CrcHash for SecretKey {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(self.as_ref());
    }
}

impl CrcHash for SigningKey {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(self.as_ref());
    }
}

impl CrcHash for PrivateKey {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(self.as_ref());
    }
}

impl CrcHash for SequesterPrivateKeyDer {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(&self.dump());
    }
}

impl CrcHash for SequesterPublicKeyDer {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(&self.dump());
    }
}

impl CrcHash for SequesterSigningKeyDer {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(&self.dump());
    }
}

impl CrcHash for SequesterVerifyKeyDer {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(&self.dump());
    }
}

impl CrcHash for WorkspaceEntry {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"WorkspaceEntry");

        let WorkspaceEntry {
            id,
            name,
            key,
            encryption_revision,
            encrypted_on,
            legacy_role_cache_timestamp,
            legacy_role_cache_value,
        } = self;

        id.crc_hash(hasher);
        name.crc_hash(hasher);
        key.crc_hash(hasher);
        encryption_revision.crc_hash(hasher);
        encrypted_on.crc_hash(hasher);
        legacy_role_cache_timestamp.crc_hash(hasher);
        legacy_role_cache_value.crc_hash(hasher);
    }
}

impl CrcHash for UserManifest {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"UserManifest");

        let UserManifest {
            author,
            timestamp,
            id,
            version,
            created,
            updated,
            last_processed_message,
            workspaces,
        } = self;

        author.crc_hash(hasher);
        timestamp.crc_hash(hasher);
        id.crc_hash(hasher);
        version.crc_hash(hasher);
        created.crc_hash(hasher);
        updated.crc_hash(hasher);
        last_processed_message.crc_hash(hasher);
        workspaces.crc_hash(hasher);
    }
}

impl CrcHash for LocalUserManifest {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"LocalUserManifest");

        let LocalUserManifest {
            base,
            need_sync,
            updated,
            last_processed_message,
            workspaces,
            speculative,
        } = self;

        base.crc_hash(hasher);
        need_sync.crc_hash(hasher);
        updated.crc_hash(hasher);
        last_processed_message.crc_hash(hasher);
        workspaces.crc_hash(hasher);
        speculative.crc_hash(hasher);
    }
}

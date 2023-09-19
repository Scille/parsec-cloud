// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `std::hash::Hash` is not stable across platforms (e.g. it uses native
// endianness when hashing numbers), hence we have to roll our own system.

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU64,
    sync::Arc,
};

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

impl<A: CrcHash + std::cmp::Ord, B: CrcHash> CrcHash for HashMap<A, B> {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"HashMap");
        let mut items: Vec<_> = self.iter().collect();
        items.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        for (k, v) in items {
            k.crc_hash(hasher);
            v.crc_hash(hasher);
        }
    }
}

impl<T: CrcHash + std::cmp::Ord> CrcHash for HashSet<T> {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"HashSet");
        let mut items: Vec<_> = self.iter().collect();
        items.sort_unstable();
        for v in items {
            v.crc_hash(hasher);
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

macro_rules! impl_crc_hash_for_nonzero_number {
    ($name:ident) => {
        impl CrcHash for $name {
            fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
                hasher.update(&self.get().to_le_bytes())
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
impl_crc_hash_for_nonzero_number!(NonZeroU64);

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

impl_crc_hash_for_uuid_based!(VlobID);
impl_crc_hash_for_uuid_based!(BlockID);
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

impl CrcHash for HashDigest {
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

impl CrcHash for WorkspaceManifest {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"WorkspaceManifest");

        let WorkspaceManifest {
            author,
            timestamp,
            id,
            version,
            created,
            updated,
            children,
        } = self;

        author.crc_hash(hasher);
        timestamp.crc_hash(hasher);
        id.crc_hash(hasher);
        version.crc_hash(hasher);
        created.crc_hash(hasher);
        updated.crc_hash(hasher);
        children.crc_hash(hasher);
    }
}

impl CrcHash for LocalWorkspaceManifest {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"LocalWorkspaceManifest");

        let LocalWorkspaceManifest {
            base,
            need_sync,
            updated,
            children,
            speculative,
            local_confinement_points,
            remote_confinement_points,
        } = self;

        base.crc_hash(hasher);
        need_sync.crc_hash(hasher);
        updated.crc_hash(hasher);
        children.crc_hash(hasher);
        speculative.crc_hash(hasher);
        local_confinement_points.crc_hash(hasher);
        remote_confinement_points.crc_hash(hasher);
    }
}

impl CrcHash for FolderManifest {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"FolderManifest");

        let FolderManifest {
            author,
            timestamp,
            id,
            parent,
            version,
            created,
            updated,
            children,
        } = self;

        author.crc_hash(hasher);
        timestamp.crc_hash(hasher);
        id.crc_hash(hasher);
        parent.crc_hash(hasher);
        version.crc_hash(hasher);
        created.crc_hash(hasher);
        updated.crc_hash(hasher);
        children.crc_hash(hasher);
    }
}

impl CrcHash for LocalFolderManifest {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"LocalFolderManifest");

        let LocalFolderManifest {
            base,
            need_sync,
            updated,
            children,
            local_confinement_points,
            remote_confinement_points,
        } = self;

        base.crc_hash(hasher);
        need_sync.crc_hash(hasher);
        updated.crc_hash(hasher);
        children.crc_hash(hasher);
        local_confinement_points.crc_hash(hasher);
        remote_confinement_points.crc_hash(hasher);
    }
}

impl CrcHash for FileManifest {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"FileManifest");

        let FileManifest {
            author,
            timestamp,
            id,
            parent,
            version,
            created,
            updated,
            size,
            blocksize,
            blocks,
        } = self;

        author.crc_hash(hasher);
        timestamp.crc_hash(hasher);
        id.crc_hash(hasher);
        parent.crc_hash(hasher);
        version.crc_hash(hasher);
        created.crc_hash(hasher);
        updated.crc_hash(hasher);
        size.crc_hash(hasher);
        blocksize.crc_hash(hasher);
        blocks.crc_hash(hasher);
    }
}

impl CrcHash for BlockAccess {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"BlockAccess");

        let BlockAccess {
            id,
            key,
            offset,
            size,
            digest,
        } = self;

        id.crc_hash(hasher);
        key.crc_hash(hasher);
        offset.crc_hash(hasher);
        size.crc_hash(hasher);
        digest.crc_hash(hasher);
    }
}
impl CrcHash for Chunk {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"Chunk");

        let Chunk {
            id,
            start,
            stop,
            raw_offset,
            raw_size,
            access,
        } = self;

        id.crc_hash(hasher);
        start.crc_hash(hasher);
        stop.crc_hash(hasher);
        raw_offset.crc_hash(hasher);
        raw_size.crc_hash(hasher);
        access.crc_hash(hasher);
    }
}

impl CrcHash for LocalFileManifest {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"LocalFileManifest");

        let LocalFileManifest {
            base,
            need_sync,
            updated,
            size,
            blocksize,
            blocks,
        } = self;

        base.crc_hash(hasher);
        need_sync.crc_hash(hasher);
        updated.crc_hash(hasher);
        size.crc_hash(hasher);
        blocksize.crc_hash(hasher);
        blocks.crc_hash(hasher);
    }
}

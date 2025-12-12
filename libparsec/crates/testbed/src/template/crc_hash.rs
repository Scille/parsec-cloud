// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `std::hash::Hash` is not stable across platforms (e.g. it uses native
// endianness when hashing numbers), hence we have to roll our own system.

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    num::{NonZeroU64, NonZeroU8},
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

impl CrcHash for String {
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
impl_crc_hash_for_nonzero_number!(NonZeroU8);

macro_rules! impl_crc_hash_for_str_based {
    ($name:ident) => {
        impl CrcHash for $name {
            fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
                hasher.update(stringify!($name).as_bytes());
                self.as_ref().crc_hash(hasher)
            }
        }
    };
}

impl_crc_hash_for_str_based!(HumanHandle);
impl_crc_hash_for_str_based!(DeviceLabel);
impl_crc_hash_for_str_based!(EntryName);

impl CrcHash for EmailAddress {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(stringify!(EmailAddress).as_bytes());
        self.hash(hasher);
    }
}

macro_rules! impl_crc_hash_for_uuid_based {
    ($name:ident) => {
        impl CrcHash for $name {
            fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
                hasher.update(stringify!($name).as_bytes());
                hasher.update(self.as_bytes())
            }
        }
    };
}

impl_crc_hash_for_uuid_based!(UserID);
impl_crc_hash_for_uuid_based!(DeviceID);
impl_crc_hash_for_uuid_based!(VlobID);
impl_crc_hash_for_uuid_based!(BlockID);
impl_crc_hash_for_uuid_based!(ChunkID);
impl_crc_hash_for_uuid_based!(SequesterServiceID);
impl_crc_hash_for_uuid_based!(PKIEnrollmentID);
impl_crc_hash_for_uuid_based!(GreetingAttemptID);

macro_rules! impl_crc_hash_for_token_based {
    ($name:ident) => {
        impl CrcHash for $name {
            fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
                hasher.update(stringify!($name).as_bytes());
                hasher.update(self.as_ref())
            }
        }
    };
}

impl_crc_hash_for_token_based!(InvitationToken);

impl CrcHash for DateTime {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"DateTime");
        hasher.update(&self.as_timestamp_micros().to_le_bytes());
    }
}

impl CrcHash for UserProfile {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"UserProfile");
        match self {
            UserProfile::Admin => hasher.update(b"Admin"),
            UserProfile::Standard => hasher.update(b"Standard"),
            UserProfile::Outsider => hasher.update(b"Outsider"),
        };
    }
}

impl CrcHash for RealmRole {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"RealmRole");
        match self {
            RealmRole::Owner => hasher.update(b"Owner"),
            RealmRole::Manager => hasher.update(b"Manager"),
            RealmRole::Contributor => hasher.update(b"Contributor"),
            RealmRole::Reader => hasher.update(b"Reader"),
        }
    }
}

impl CrcHash for KeyDerivation {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"KeyDerivation");
        hasher.update(self.as_ref());
    }
}

impl CrcHash for SecretKey {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"SecretKey");
        hasher.update(self.as_ref());
    }
}

impl CrcHash for HashDigest {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"HashDigest");
        hasher.update(self.as_ref());
    }
}

impl CrcHash for SigningKey {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"SigningKey");
        hasher.update(self.to_bytes().as_ref());
    }
}

impl CrcHash for VerifyKey {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"VerifyKey");
        hasher.update(self.as_ref());
    }
}

impl CrcHash for PrivateKey {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"PrivateKey");
        hasher.update(self.to_bytes().as_ref());
    }
}

impl CrcHash for PublicKey {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"PublicKey");
        hasher.update(self.as_ref());
    }
}

impl CrcHash for SequesterPrivateKeyDer {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"SequesterPrivateKeyDer");
        hasher.update(&self.dump());
    }
}

impl CrcHash for SequesterPublicKeyDer {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"SequesterPublicKeyDer");
        hasher.update(&self.dump());
    }
}

impl CrcHash for SequesterSigningKeyDer {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"SequesterSigningKeyDer");
        hasher.update(&self.dump());
    }
}

impl CrcHash for SequesterVerifyKeyDer {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"SequesterVerifyKeyDer");
        hasher.update(&self.dump());
    }
}

impl CrcHash for LocalUserManifestWorkspaceEntry {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"LocalUserManifestWorkspaceEntry");

        let LocalUserManifestWorkspaceEntry {
            id,
            name,
            name_origin,
            role,
            role_origin,
        } = self;

        id.crc_hash(hasher);
        name.crc_hash(hasher);
        name_origin.crc_hash(hasher);
        role.crc_hash(hasher);
        role_origin.crc_hash(hasher);
    }
}

impl CrcHash for CertificateBasedInfoOrigin {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"WorkspaceNameOrigin");
        match self {
            CertificateBasedInfoOrigin::Certificate { timestamp } => {
                hasher.update(b"Certificate");
                timestamp.crc_hash(hasher);
            }
            CertificateBasedInfoOrigin::Placeholder => {
                hasher.update(b"Placeholder");
            }
        }
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
        } = self;

        author.crc_hash(hasher);
        timestamp.crc_hash(hasher);
        id.crc_hash(hasher);
        version.crc_hash(hasher);
        created.crc_hash(hasher);
        updated.crc_hash(hasher);
    }
}

impl CrcHash for LocalUserManifest {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"LocalUserManifest");

        let LocalUserManifest {
            base,
            need_sync,
            updated,
            local_workspaces,
            speculative,
        } = self;

        base.crc_hash(hasher);
        need_sync.crc_hash(hasher);
        updated.crc_hash(hasher);
        local_workspaces.crc_hash(hasher);
        speculative.crc_hash(hasher);
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
            parent,
            need_sync,
            updated,
            children,
            local_confinement_points,
            remote_confinement_points,
            speculative,
        } = self;

        base.crc_hash(hasher);
        parent.crc_hash(hasher);
        need_sync.crc_hash(hasher);
        updated.crc_hash(hasher);
        children.crc_hash(hasher);
        local_confinement_points.crc_hash(hasher);
        remote_confinement_points.crc_hash(hasher);
        speculative.crc_hash(hasher);
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
            offset,
            size,
            digest,
        } = self;

        id.crc_hash(hasher);
        offset.crc_hash(hasher);
        size.crc_hash(hasher);
        digest.crc_hash(hasher);
    }
}
impl CrcHash for ChunkView {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"ChunkView");

        let ChunkView {
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
            parent,
            need_sync,
            updated,
            size,
            blocksize,
            blocks,
        } = self;

        base.crc_hash(hasher);
        parent.crc_hash(hasher);
        need_sync.crc_hash(hasher);
        updated.crc_hash(hasher);
        size.crc_hash(hasher);
        blocksize.crc_hash(hasher);
        blocks.crc_hash(hasher);
    }
}

impl CrcHash for HashAlgorithm {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"HashAlgorithm");
        match self {
            HashAlgorithm::Sha256 => hasher.update(b"Sha256"),
        }
    }
}

impl CrcHash for SecretKeyAlgorithm {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"SecretKeyAlgorithm");
        match self {
            SecretKeyAlgorithm::Blake2bXsalsa20Poly1305 => {
                hasher.update(b"Blake2bXsalsa20Poly1305")
            }
        }
    }
}

impl CrcHash for RealmArchivingConfiguration {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        hasher.update(b"RealmArchivingConfiguration");
        match self {
            RealmArchivingConfiguration::Available => hasher.update(b"Available"),
            RealmArchivingConfiguration::Archived => hasher.update(b"Archived"),
            RealmArchivingConfiguration::DeletionPlanned { deletion_date } => {
                hasher.update(b"DeletionPlanned");
                deletion_date.crc_hash(hasher);
            }
        }
    }
}

impl CrcHash for ActiveUsersLimit {
    fn crc_hash(&self, hasher: &mut crc32fast::Hasher) {
        match self {
            ActiveUsersLimit::LimitedTo(limit) => {
                hasher.update(b"LimitedTo");
                limit.crc_hash(hasher);
            }
            ActiveUsersLimit::NoLimit => hasher.update(b"NoLimit"),
        }
    }
}

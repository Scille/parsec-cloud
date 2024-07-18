// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashSet;

use libparsec_platform_storage::certificates::UpTo;
use libparsec_types::prelude::*;

use super::{store::CertifStoreError, CertificateOps};

#[derive(Debug, thiserror::Error)]
pub enum CertifGetRealmNeedsError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<CertifStoreError> for CertifGetRealmNeedsError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RealmNeeds {
    Nothing,
    KeyRotationOnly {
        current_key_index: Option<IndexInt>,
    },
    /// A user that is part of the workspace has been revoked, this creates a cascade
    /// of needs: first the workspace needs to be unshared, which is turn means a key
    /// rotation is needed.
    UnshareThenKeyRotation {
        current_key_index: Option<IndexInt>,
        revoked_users: Vec<UserID>,
    },
}

/// Note realm existence is not checked (and `RealmNeeds::Nothing` is returned if that's not the case)
pub async fn get_realm_needs(
    ops: &CertificateOps,
    realm_id: VlobID,
) -> Result<RealmNeeds, CertifGetRealmNeedsError> {
    ops.store
        .for_read(|store| async move {
            // TODO: This implementation is not efficient, as it fetches all the roles
            //       and users certificates. However this should be good enough for now,
            //       and optimization requires a complex API changes in the storage.

            let mut needs = RealmNeeds::Nothing;

            let last_key_rotation = store
                .get_realm_last_key_rotation_certificate(UpTo::Current, realm_id)
                .await?;
            let current_key_index = last_key_rotation.as_ref().map(|x| x.key_index);

            let roles = store.get_realm_roles(UpTo::Current, realm_id).await?;
            let mut current_users_with_role = HashSet::new();
            // This loop does two things at once:
            // - Collect all users that are currently part of the workspace (will be used in next step)
            // - Update `needs` if a unshare has occured since the last key rotation
            for role in roles {
                match role.role {
                    Some(_) => {
                        current_users_with_role.insert(role.user_id);
                    }
                    None => {
                        current_users_with_role.remove(&role.user_id);

                        match &last_key_rotation {
                            Some(last_key_rotation)
                                if last_key_rotation.timestamp < role.timestamp =>
                            {
                                needs = RealmNeeds::KeyRotationOnly { current_key_index };
                            }
                            None => {
                                needs = RealmNeeds::KeyRotationOnly { current_key_index };
                            }
                            _ => (),
                        }
                    }
                }
            }

            for user_id in current_users_with_role {
                let maybe_revoked = store
                    .get_revoked_user_certificate(UpTo::Current, user_id)
                    .await?;
                if maybe_revoked.is_some() {
                    match &mut needs {
                        RealmNeeds::UnshareThenKeyRotation { revoked_users, .. } => {
                            revoked_users.push(user_id);
                        }
                        _ => {
                            needs = RealmNeeds::UnshareThenKeyRotation {
                                current_key_index,
                                revoked_users: vec![user_id],
                            };
                        }
                    }
                }
            }

            // The use of `HashSet` makes our output non-deterministic, so we fix it here
            if let RealmNeeds::UnshareThenKeyRotation { revoked_users, .. } = &mut needs {
                revoked_users.sort();
            }

            Ok(needs)
        })
        .await?
}

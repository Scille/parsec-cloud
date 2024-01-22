// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::Client;
use crate::{
    certif::{
        CertifDecryptCurrentRealmNameError, CertifStoreError, InvalidEncryptedRealmNameError,
        InvalidKeysBundleError,
    },
    user::UserStoreUpdateError,
};

#[derive(Debug, thiserror::Error)]
pub enum RefreshWorkspacesListError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    InvalidEncryptedRealmName(#[from] InvalidEncryptedRealmNameError),
    #[error(transparent)]
    InvalidKeysBundle(#[from] InvalidKeysBundleError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn refresh_workspaces_list(client: &Client) -> Result<(), RefreshWorkspacesListError> {
    // The strategy here is to first and foremost rely on the certificates locally present,
    // then consider the additional entries from the old local user manifest's workspace
    // list as local-only newly created workspaces.
    //
    // Note this approach may go backward when the certificates database is cleared
    // (e.g. a workspace shared with us will suddenly appeared as local-only newly
    // created given it is present in the local user manifest's workspace list but not
    // in the certificates).
    // However this is no big deal given certificates database clearing is rare and
    // eventual consistency will be reached once the latest certificates are re-fetched.

    // 1) Lock the local user manifest early to avoid concurrent refresh
    // This is important to prevent missing a newly shared workspace due to
    // a concurrent refresh (classical "A fetches data before B, but processes
    // them after B" problem).
    //
    // The downside is that the user manifest is potentially locked for a long time (as
    // `CertifOps::list_self_workspaces` may need to reach the server).

    let mut updater = client.user_ops.for_update_local_workspaces().await;
    let old_local_workspaces = updater.workspaces();

    let mut local_workspaces = Vec::with_capacity(old_local_workspaces.len());

    // 2) Retrieve the list of workspaces according to the certificates locally stored

    let self_realms_role = client
        .certificates_ops
        .get_current_self_realms_role()
        .await
        .map_err(|e| match e {
            CertifStoreError::Stopped => RefreshWorkspacesListError::Stopped,
            CertifStoreError::Internal(err) => {
                err.context("Cannot retrieve self realms role").into()
            }
        })?;

    // 3) Cook the new list of workspaces from the certificates

    for (workspace_id, self_role, role_certificate_timestamp) in self_realms_role {
        // Filter out user realm as it is not a workspace
        if workspace_id == client.device.user_realm_id {
            continue;
        }

        // Ignore the workspace we are no longer part of
        let self_role = match self_role {
            None => continue,
            Some(role) => role,
        };

        // Retrieve the name of the workspace
        let outcome = client
            .certificates_ops
            .decrypt_current_realm_name(workspace_id)
            .await;

        let (name, name_origin) = match outcome {
            Ok((name, timestamp)) => {
                Ok((name, CertificateBasedInfoOrigin::Certificate { timestamp }))
            }
            Err(e) => match e {
                CertifDecryptCurrentRealmNameError::Stopped => Err(RefreshWorkspacesListError::Stopped),
                CertifDecryptCurrentRealmNameError::Offline => Err(RefreshWorkspacesListError::Offline),
                // We have lost access to the workspace concurrently, ignore it
                CertifDecryptCurrentRealmNameError::NotAllowed => continue,
                // This workspace is not fully bootstrapped yet (this is unlikely, as workspace
                // is supposed to be bootstrapped before being shared), so we have to use a
                // placeholder name.
                // Note that, if we are OWNER of the workspace, this placeholder will eventually
                // be used when we will try to finish the bootstrap.
                CertifDecryptCurrentRealmNameError::NoNameCertificate
                // Fallback to default name if we cannot decrypt the actual name.
                // This is done to avoid having a single malfunctioning workspace breaking
                // the whole refresh process.
                // Note that, this may lead to the overwritting of the name while it is the
                // keys bundle that is at fault (e.g. realm name has been made with key
                // index 1, then key rotation for index 2 is corrupted). We consider this
                // okay given how low the risk is.
                | CertifDecryptCurrentRealmNameError::InvalidEncryptedRealmName(_)
                | CertifDecryptCurrentRealmNameError::InvalidKeysBundle(_)
                => {
                    let default_name = format!("Workspace {}", &workspace_id.hex()).parse().expect("valid name");
                    Ok((default_name, CertificateBasedInfoOrigin::Placeholder))
                },
                CertifDecryptCurrentRealmNameError::Internal(err) => Err(err.context("Cannot retrieve workspace name").into()),
            },
        }?;

        local_workspaces.push(LocalUserManifestWorkspaceEntry {
            id: workspace_id,
            name,
            name_origin,
            role: self_role,
            role_origin: CertificateBasedInfoOrigin::Certificate {
                timestamp: role_certificate_timestamp,
            },
        });
    }

    // 4) Pick missing info from the old list of workspaces

    for old_entry in old_local_workspaces {
        match local_workspaces.iter_mut().find(|e| e.id == old_entry.id) {
            // Workspace already present
            Some(new_entry) => {
                // If the workspace name is a placeholder, it means we've just invented it.
                // So we should use the old entry's name instead as it may correspond to a
                // real name (e.g. we are the creator of the workspace, but our last
                // bootstrap attempt didn't go to completion).
                //
                // Note we don't care if the old entry's name comes from a cerficate:
                // instead we rely on the fact the initial name certificate upload is
                // idempotent. This is a more robust approach as it doesn't rely on
                // a certificate that may no longer exist (in case of server rollback
                // or buggy client).
                if matches!(
                    new_entry.name_origin,
                    CertificateBasedInfoOrigin::Placeholder
                ) {
                    new_entry.name = old_entry.name.clone();
                }
            }
            // Workspace is not in the new list, so it is either:
            // - A workspace we've just created (hence currently local-only).
            // - The certificates database is lagging behind (e.g. it has been cleared
            //   because we switch from/to OUTSIDER role).
            //   In this case we can also pretend we have just created the workspace
            //   thank to the workspace bootstrap being idempotent (and we will
            //   eventually receive the certificates and hence correct the entry).
            None => {
                local_workspaces.push(LocalUserManifestWorkspaceEntry {
                    id: old_entry.id,
                    name: old_entry.name.clone(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: old_entry.role,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                });
            }
        }
    }

    // 5) Last cooking: order by name for stability

    local_workspaces.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    // 6) Commit our change

    updater
        .set_workspaces_and_keep_lock(local_workspaces)
        .await
        .map_err(|e| match e {
            UserStoreUpdateError::Stopped => RefreshWorkspacesListError::Stopped,
            UserStoreUpdateError::Internal(err) => {
                err.context("Cannot update the local user manifest").into()
            }
        })?;

    // 7) Last but not least: refresh the running workspaces
    // At this point we hold two locks: `updater` (on local user manifest) and
    // `client.workspaces` (on the list of running workspaces).
    // We need to keep `updater` locked to ensure we are refreshing the running
    // workspaces with the latest info.
    // There is no risk of deadlock as this is the only place thoses two locks are
    // held together.

    let mut running_workspaces = client.workspaces.lock().await;
    for workspace_ops in (*running_workspaces).clone() {
        let found = updater
            .workspaces()
            .iter()
            .find(|e| e.id == workspace_ops.realm_id());
        match found {
            // Workspace not longer shared with us
            None => {
                super::workspace_start::stop_workspace_internal(
                    client,
                    &mut running_workspaces,
                    workspace_ops.realm_id(),
                )
                .await
            }
            // Workspace still shared with us, but it might have changed nevertheless
            Some(workspace_entry) => workspace_ops.update_workspace_entry(|entry| {
                *entry = workspace_entry.to_owned();
            }),
        }
    }

    Ok(())
}

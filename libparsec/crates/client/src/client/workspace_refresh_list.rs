// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::Client;
use crate::{
    certif::{
        CertifDecryptCurrentRealmNameError, CertifGetCurrentSelfRealmsRoleError,
        CertifGetRealmArchivingConfigurationError, CertifGetRealmCanSelfPromoteToOwnerError,
        InvalidEncryptedRealmNameError, InvalidKeysBundleError,
    },
    event_bus::EventWorkspacesSelfListChanged,
    user::UserStoreUpdateError,
};

#[derive(Debug, thiserror::Error)]
pub enum ClientRefreshWorkspacesListError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    InvalidEncryptedRealmName(#[from] Box<InvalidEncryptedRealmNameError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn refresh_workspaces_list(
    client: &Client,
) -> Result<(), ClientRefreshWorkspacesListError> {
    // The strategy here is to first and foremost rely on the certificates locally present,
    // then consider the additional entries from the local user manifest's workspace list
    // as local-only newly created workspaces.
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
    // `CertificateOps::list_self_workspaces` may need to reach the server).

    let mut updater = client.user_ops.for_update_local_workspaces().await;
    let old_local_workspaces = updater.workspaces();

    let mut local_workspaces = Vec::with_capacity(old_local_workspaces.len());
    // Workspaces not longer shared with us or that have been deleted, in either case
    // we can no longer use them and hence we should cleanup their local database.
    let mut to_cleanup_workspaces = vec![];

    // 2) Retrieve the list of workspaces according to the certificates locally stored

    let self_realms_role = client
        .certificates_ops
        .get_current_self_realms_role()
        .await
        .map_err(|e| match e {
            CertifGetCurrentSelfRealmsRoleError::Stopped => {
                ClientRefreshWorkspacesListError::Stopped
            }
            CertifGetCurrentSelfRealmsRoleError::Internal(err) => {
                err.context("Cannot retrieve self realms role").into()
            }
        })?;

    // 3) Cook the new list of workspaces from the certificates

    for (workspace_id, self_role, role_certificate_timestamp) in &self_realms_role {
        // Filter out user realm as it is not a workspace
        if *workspace_id == client.device.user_realm_id {
            continue;
        }

        // Ignore the workspace we are no longer part of
        let self_role = match self_role {
            None => continue,
            Some(role) => role,
        };

        // Ignore the workspace that we consider deleted (see comment below)
        let (archiving_configuration, archiving_configuration_origin) = client
            .certificates_ops
            .get_realm_archiving_configuration(*workspace_id)
            .await
            .map(|(config, maybe_timestamp)| match maybe_timestamp {
                Some(timestamp) => (
                    config,
                    CertificateBasedInfoOrigin::Certificate { timestamp },
                ),
                None => (config, CertificateBasedInfoOrigin::Placeholder),
            })
            .map_err(|e| match e {
                CertifGetRealmArchivingConfigurationError::Stopped => {
                    ClientRefreshWorkspacesListError::Stopped
                }
                CertifGetRealmArchivingConfigurationError::Internal(err) => err
                    .context("Cannot retrieve workspace archiving configuration")
                    .into(),
            })?;

        // We consider as deleted any workspace whose deletion date has been reached.
        // This is not strictly true as the actual deletion is not effective until a job
        // is run server-side, however we have no way of knowing when this is done so
        // it's just simpler to assume this occurs as soon as possible.
        // Also note that if the deletion is eventually cancelled (i.e. a new archiving
        // certificate is uploaded), then the poll certificate -> add certificates -> refresh workspace list
        // pipeline will kick in and the workspace will be added to the list of active
        // workspaces normally.
        if matches!(archiving_configuration, RealmArchivingConfiguration::DeletionPlanned { deletion_date } if deletion_date <= client.device.now())
        {
            to_cleanup_workspaces.push(*workspace_id);
            continue;
        }

        // Retrieve the name of the workspace
        let outcome = client
            .certificates_ops
            .decrypt_current_realm_name(*workspace_id)
            .await;

        let (name, name_origin) = match outcome {
            Ok((name, timestamp)) => {
                Ok((name, CertificateBasedInfoOrigin::Certificate { timestamp }))
            }
            Err(e) => match e {
                CertifDecryptCurrentRealmNameError::Stopped => Err(ClientRefreshWorkspacesListError::Stopped),
                CertifDecryptCurrentRealmNameError::Offline(e) => Err(ClientRefreshWorkspacesListError::Offline(e)),
                // We have lost access to the workspace concurrently, ignore it
                CertifDecryptCurrentRealmNameError::NotAllowed => continue,
                // The realm has been deleted, ignore it
                CertifDecryptCurrentRealmNameError::RealmDeleted => continue,
                // This workspace is not fully bootstrapped yet (this is unlikely, as workspace
                // is supposed to be bootstrapped before being shared), so we have to use a
                // placeholder name.
                // Note that, if we are OWNER of the workspace, this placeholder will eventually
                // be used when we will try to finish the bootstrap.
                CertifDecryptCurrentRealmNameError::NoNameCertificate
                // Fallback to default name if we cannot decrypt the actual name.
                // This is done to avoid having a single malfunctioning workspace breaking
                // the whole refresh process.
                // Note that, this may lead to the overwriting of the name while it is the
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

        let can_self_promote_to_owner = client
            .certificates_ops
            .get_realm_can_self_promote_to_owner(*workspace_id)
            .await
            .map_err(|err| match err {
                CertifGetRealmCanSelfPromoteToOwnerError::Internal(err) => err
                    .context("Cannot retrieve workspace self-promote status")
                    .into(),
                CertifGetRealmCanSelfPromoteToOwnerError::Stopped => {
                    ClientRefreshWorkspacesListError::Stopped
                }
            })?;

        local_workspaces.push(LocalUserManifestWorkspaceEntry {
            id: *workspace_id,
            name,
            name_origin,
            role: *self_role,
            role_origin: CertificateBasedInfoOrigin::Certificate {
                timestamp: *role_certificate_timestamp,
            },
            archiving_configuration: archiving_configuration.into(),
            archiving_configuration_origin: archiving_configuration_origin.into(),
            can_self_promote_to_owner: can_self_promote_to_owner.into(),
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
                // workspace bootstrap attempt didn't go to completion).
                //
                // Note we don't care if the old entry's name comes from a certificate:
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
            // A) A workspace whose deletion date has been reached.
            // B) A workspace we've lost access to.
            // C) A workspace we've just created (hence currently local-only).
            // D) The certificates database is lagging behind (e.g. it has been cleared
            //   because we switch from/to OUTSIDER role).
            //   In this case we can also pretend we have just created the workspace
            //   thank to the workspace bootstrap being idempotent (and we will
            //   eventually receive the certificates and hence correct the entry).
            //
            // Here we only want to keep case C & D (i.e. workspace that are still useful).
            None => {
                // `to_cleanup_workspaces` is already populated with workspaces in
                // case A (this has been done during the previous step)
                if to_cleanup_workspaces.contains(&old_entry.id) {
                    continue;
                }

                // Detect case B
                let no_longer_access = self_realms_role
                    .iter()
                    .find_map(|(wid, role, _)| {
                        if *wid == old_entry.id {
                            Some(role.is_none())
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false);
                if no_longer_access {
                    to_cleanup_workspaces.push(old_entry.id);
                    continue;
                }

                // Now we only have workspaces in case C & D
                local_workspaces.push(LocalUserManifestWorkspaceEntry {
                    id: old_entry.id,
                    name: old_entry.name.clone(),
                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                    role: old_entry.role,
                    role_origin: CertificateBasedInfoOrigin::Placeholder,
                    archiving_configuration: old_entry.archiving_configuration,
                    archiving_configuration_origin: CertificateBasedInfoOrigin::Placeholder.into(),
                    can_self_promote_to_owner: false.into(),
                });
            }
        }
    }

    // 5) Last cooking: order by name for stability

    local_workspaces.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    // Fast fail in case nothing has changed
    if local_workspaces == old_local_workspaces {
        return Ok(());
    }

    // 6) Commit our change

    updater
        .set_workspaces_and_keep_lock(local_workspaces)
        .await
        .map_err(|e| match e {
            UserStoreUpdateError::Stopped => ClientRefreshWorkspacesListError::Stopped,
            UserStoreUpdateError::Internal(err) => {
                err.context("Cannot update the local user manifest").into()
            }
        })?;

    // 7) Refresh the running workspaces
    // At this point we hold two locks: `updater` (on local user manifest) and
    // `client.workspaces` (on the list of running workspaces).
    // We need to keep `updater` locked to ensure we are refreshing the running
    // workspaces with the latest info.
    // There is no risk of deadlock as this is the only place those two locks are
    // held together.

    let workspaces = updater.workspaces();
    let total_workspaces = workspaces.len();
    let mut running_workspaces = client.workspaces.lock().await;
    for workspace_ops in (*running_workspaces).clone() {
        let found = workspaces
            .iter()
            .enumerate()
            .find(|(_, e)| e.id == workspace_ops.realm_id());
        match found {
            // Workspace not longer accessible (unshared or workspace deleted)
            None => {
                super::workspace_start::stop_workspace_internal(
                    client,
                    &mut running_workspaces,
                    workspace_ops.realm_id(),
                )
                .await
            }
            // Workspace still accessible, but it might have changed nevertheless
            Some((workspace_index, workspace_entry)) => workspace_ops
                .update_workspace_external_info(|info| {
                    workspace_entry.clone_into(&mut info.entry);
                    // Note workspace index doesn't have to be stable, so we can just overwrite it
                    info.workspace_index = workspace_index;
                    info.total_workspaces = total_workspaces;
                }),
        }
    }

    // 8) Remove the database for the workspaces we no longer have access to
    for workspace_id in to_cleanup_workspaces {
        if let Err(err) = libparsec_platform_storage::workspace::workspace_storage_remove_data(
            &client.config.data_base_dir,
            &client.device,
            workspace_id,
        )
        .await
        {
            log::warn!("Cannot remove local database for (no longer needed) workspace {workspace_id}: {err}")
        }
    }

    // 9) Finally trigger an event about the refresh
    client.event_bus.send(&EventWorkspacesSelfListChanged);

    Ok(())
}

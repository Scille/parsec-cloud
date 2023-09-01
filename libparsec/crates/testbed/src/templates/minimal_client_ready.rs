// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::TestbedTemplate;

/// Similar to minimal organization (i.e. single Alice user&device), but with some data:
/// - a workspace `wksp1` containing a file `/foo.txt`
/// - file `/foo.txt` composed of a single block with `hello world` as content
/// - Alice user, workspace and certificate storages are populated with all data
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    let mut builder = TestbedTemplate::from_builder("minimal_client_ready");

    builder.bootstrap_organization("alice"); // alice@dev1

    let (realm_id, realm_key, realm_timestamp) = builder
        .new_realm("alice")
        .map(|e| (e.realm_id, e.realm_key.clone(), e.timestamp));

    // TODO: create `/foo.txt` !

    builder
        .new_user_realm("alice")
        .then_create_initial_user_manifest_vlob()
        .customize(|e| {
            Arc::make_mut(&mut e.manifest)
                .workspaces
                .push(WorkspaceEntry::new(
                    realm_id,
                    "wksp1".parse().unwrap(),
                    realm_key,
                    1,
                    realm_timestamp,
                ))
        });

    builder.certificates_storage_fetch_certificates("alice@dev1");
    builder.user_storage_fetch_user_vlob("alice@dev1");

    builder.finalize()
}

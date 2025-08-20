// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{InvalidManifestError, WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError};

use super::utils::workspace_history_ops_with_realm_export_access_factory;

#[parsec_test(testbed = "sequestered")]
async fn decrypt_with(
    #[values(
        "sequester_service_ok",
        "only_sequester_service_ko",
        "sequester_service_ko_then_user_ok"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    enum When {
        SequesterService1Revoked,
        // v3 of `bar.txt` is done after the key rotation following sequester
        // service 1 revocation.
        BarTxtV3,
    }
    let (expected_success, when, decryptors) = match kind {
        "sequester_service_ok" => (
            true,
            When::SequesterService1Revoked,
            ["sequester_service_1"].as_ref(),
        ),
        "only_sequester_service_ko" => (false, When::BarTxtV3, ["sequester_service_1"].as_ref()),
        "sequester_service_ko_then_user_ok" => {
            // Not sequester service 2 is not revoked, but no key rotation occurred
            // since its creation, and hence it never had any access to the realm.
            (
                true,
                When::BarTxtV3,
                ["sequester_service_2", "alice@dev1"].as_ref(),
            )
        }
        unknown => panic!("Unknown kind: {unknown}"),
    };

    let (ops, _tmp_path) = workspace_history_ops_with_realm_export_access_factory(
        env,
        decryptors,
        "server/tests/realm_export/sequestered_export.sqlite",
    )
    .await;

    let timestamp = env
        .template
        .events
        .iter()
        .rev()
        .find_map(|e| match (&when, e) {
            (When::BarTxtV3, TestbedEvent::CreateOrUpdateFileManifestVlob(e))
                if e.manifest.version == 3 =>
            {
                Some(e.manifest.timestamp)
            }
            (When::SequesterService1Revoked, TestbedEvent::RevokeSequesterService(e)) => {
                Some(e.timestamp)
            }
            _ => None,
        })
        .expect("WorkspaceHistoryExportSequesterService event not found");
    ops.set_timestamp_of_interest(timestamp).await.unwrap();

    let outcome = ops.stat_entry_by_id(wksp1_bar_txt_id).await;
    if expected_success {
        p_assert_matches!(outcome, Ok(WorkspaceHistoryEntryStat::File { .. }));
    } else {
        p_assert_matches!(
            outcome,
            Err(WorkspaceHistoryStatEntryError::InvalidManifest(boxed))
            if matches!(*boxed, InvalidManifestError::CorruptedKey { .. })
        );
    }
}

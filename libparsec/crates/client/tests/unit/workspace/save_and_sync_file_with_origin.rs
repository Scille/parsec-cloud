// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_realm_get_keys_bundle,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::OpenOptions;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    const NEW_DATA: &[u8] = b"content saved from the Cryptpad session";
    let channel_id = "channel1".to_string();
    let channel_timestamp: DateTime = "2021-01-10T00:00:00Z".parse().unwrap();

    // Mock server commands for the sync that occurs right after the save

    let expected_base_blocks = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let (key_derivation, key_index) = env.get_last_realm_key(wksp1_id);
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Fetch last workspace keys bundle to encrypt the new manifest
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 2) `block_create`
        {
            let expected_base_blocks = expected_base_blocks.clone();
            let key_derivation = key_derivation.to_owned();
            move |req: authenticated_cmds::latest::block_create::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.key_index, key_index);
                let key = key_derivation.derive_secret_key_from_uuid(*req.block_id);
                p_assert_eq!(key.decrypt(&req.block).unwrap(), NEW_DATA);
                expected_base_blocks.lock().unwrap().push(BlockAccess {
                    id: req.block_id,
                    offset: 0,
                    size: (NEW_DATA.len() as u64).try_into().unwrap(),
                    digest: HashDigest::from_data(NEW_DATA),
                });
                authenticated_cmds::latest::block_create::Rep::Ok {}
            }
        },
        // 3) `vlob_update` succeed on first try !
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.key_index, key_index);
            p_assert_eq!(req.vlob_id, wksp1_bar_txt_id);
            p_assert_eq!(req.version, 2);
            authenticated_cmds::latest::vlob_update::Rep::Ok {}
        },
    );

    let origin = FileManifestOrigin::Cryptpad {
        channel_id: channel_id.clone(),
        timestamp: channel_timestamp,
    };

    wksp1_ops
        .save_and_sync_file_with_origin(wksp1_bar_txt_id, origin.clone(), NEW_DATA)
        .await
        .unwrap();

    // Check the outcome: the file has been overwritten, marked with the Cryptpad
    // origin, and immediately synced with the server.

    let manifest = match wksp1_ops
        .store
        .get_manifest(wksp1_bar_txt_id)
        .await
        .unwrap()
    {
        ArcLocalChildManifest::File(m) => m,
        ArcLocalChildManifest::Folder(m) => panic!("Expected file, got {m:?}"),
    };

    p_assert_eq!(manifest.need_sync, false);
    p_assert_eq!(manifest.base.version, 2);
    p_assert_eq!(manifest.size, NEW_DATA.len() as SizeInt);
    p_assert_eq!(manifest.origin, origin,);
    p_assert_eq!(manifest.base.origin, origin,);
    p_assert_eq!(manifest.base.blocks, *expected_base_blocks.lock().unwrap());

    // Check the actual content is the one provided

    let fd = wksp1_ops
        .open_file_by_id(wksp1_bar_txt_id, OpenOptions::read_only())
        .await
        .unwrap();
    let mut content = Vec::with_capacity(manifest.size as usize);
    wksp1_ops
        .fd_read(fd, 0, manifest.size, &mut content)
        .await
        .unwrap();
    p_assert_eq!(content, NEW_DATA);
    wksp1_ops.fd_close(fd).await.unwrap();

    wksp1_ops.stop().await.unwrap();
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::{OpenOptions, WorkspaceFdReadError};

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(#[values(false, true)] local_cache: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_block_access = env
        .template
        .get_stuff::<BlockAccess>("wksp1_bar_txt_block_access")
        .to_owned();

    if !local_cache {
        env.customize(|builder| {
            builder.filter_client_storage_events(|event| {
                !matches!(
                    event,
                    TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
                        | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                )
            });
        })
        .await;

        let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
        let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(wksp1_id);

        // Mock server command `vlob_update`
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // 1) Fetch the manifest
            {
                let fetch_manifest_rep = env
                    .template
                    .events
                    .iter()
                    .rev()
                    .find_map(|e| match e {
                        TestbedEvent::CreateOrUpdateFileManifestVlob(e)
                            if e.manifest.id == wksp1_bar_txt_id =>
                        {
                            let vlob_read_batch_item = (
                                wksp1_bar_txt_id,
                                e.key_index,
                                e.manifest.author,
                                e.manifest.version,
                                e.manifest.timestamp,
                                e.encrypted(&env.template),
                            );
                            let rep = authenticated_cmds::latest::vlob_read_batch::Rep::Ok {
                                needed_common_certificate_timestamp:
                                    last_common_certificate_timestamp,
                                needed_realm_certificate_timestamp:
                                    last_realm_certificate_timestamp,
                                items: vec![vlob_read_batch_item],
                            };
                            Some(rep)
                        }
                        _ => None,
                    })
                    .unwrap();

                move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
                    p_assert_eq!(req.at, None);
                    p_assert_eq!(req.realm_id, wksp1_id);
                    p_assert_eq!(req.vlobs, [wksp1_bar_txt_id]);
                    fetch_manifest_rep
                }
            },
            // 2) Fetch keys bundle to decrypt the manifest (and later the block)
            {
                let keys_bundle = env.get_last_realm_keys_bundle(wksp1_id);
                let keys_bundle_access =
                    env.get_last_realm_keys_bundle_access_for(wksp1_id, "alice".parse().unwrap());
                move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                    p_assert_eq!(req.realm_id, wksp1_id);
                    p_assert_eq!(req.key_index, 1);
                    authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                        keys_bundle,
                        keys_bundle_access,
                    }
                }
            },
            // 3) Fetch the block
            {
                let fetch_block_rep = env
                    .template
                    .events
                    .iter()
                    .rev()
                    .find_map(|e| match e {
                        TestbedEvent::CreateBlock(e)
                            if e.block_id == wksp1_bar_txt_block_access.id =>
                        {
                            let rep = authenticated_cmds::latest::block_read::Rep::Ok {
                                needed_realm_certificate_timestamp:
                                    last_realm_certificate_timestamp,
                                key_index: e.key_index,
                                block: e.encrypted(&env.template),
                            };
                            Some(rep)
                        }
                        _ => None,
                    })
                    .unwrap();

                move |req: authenticated_cmds::latest::block_read::Req| {
                    p_assert_eq!(req.block_id, wksp1_bar_txt_block_access.id);
                    fetch_block_rep
                }
            },
        );
    }

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let path = "/bar.txt".parse().unwrap();
    let options = OpenOptions {
        read: true,
        write: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd = ops.open_file(path, options).await.unwrap();
    p_assert_eq!(fd.0, 1);

    // 1) Read the whole file with good size

    let mut buf = vec![];
    let expected_buf = b"hello world";

    let read_bytes = ops
        .fd_read(fd, 0, expected_buf.len() as u64, &mut buf)
        .await
        .unwrap();
    spy.assert_no_events();

    p_assert_eq!(read_bytes, expected_buf.len() as u64);
    p_assert_eq!(buf, expected_buf);

    // 2) Read the whole file with too big size

    let mut buf = vec![];
    let expected_buf = b"hello world";

    let read_bytes = ops
        .fd_read(fd, 0, expected_buf.len() as u64 + 1, &mut buf)
        .await
        .unwrap();
    spy.assert_no_events();

    p_assert_eq!(read_bytes, expected_buf.len() as u64);
    p_assert_eq!(buf, expected_buf);

    // 3) Read a subset of the file

    let mut buf = vec![];
    let expected_buf = b"hello";

    let read_bytes = ops
        .fd_read(fd, 0, expected_buf.len() as u64, &mut buf)
        .await
        .unwrap();
    spy.assert_no_events();

    p_assert_eq!(read_bytes, expected_buf.len() as u64);
    p_assert_eq!(buf, expected_buf);

    // 4) Read a subset of the file with offset

    let mut buf = vec![];
    let expected_buf = b"wor";

    let read_bytes = ops
        .fd_read(fd, 6, expected_buf.len() as u64, &mut buf)
        .await
        .unwrap();
    spy.assert_no_events();

    p_assert_eq!(read_bytes, expected_buf.len() as u64);
    p_assert_eq!(buf, expected_buf);

    ops.fd_close(fd).await.unwrap();
    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn cursor_not_in_read_mode(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let path = "/bar.txt".parse().unwrap();
    let options = OpenOptions {
        read: false,
        write: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd = ops.open_file(path, options).await.unwrap();
    p_assert_eq!(fd.0, 1);

    let mut buf = vec![];

    let err = ops.fd_read(fd, 0, 1, &mut buf).await.unwrap_err();
    spy.assert_no_events();
    // TODO: bad error type
    p_assert_matches!(err, WorkspaceFdReadError::NotInReadMode);

    ops.fd_close(fd).await.unwrap();
    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn unknown_file_descriptor(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops
        .fd_read(FileDescriptor(1), 0, 1, &mut vec![])
        .await
        .unwrap_err();
    spy.assert_no_events();
    p_assert_matches!(err, WorkspaceFdReadError::BadFileDescriptor);

    ops.stop().await.unwrap();
}

// TODO: test invalid manifest with less block that in declared size
// TODO: test read with data not in store

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks, test_send_hook_block_read,
    test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{workspace_history_ops_with_server_access_factory, DataAccessStrategy};
use crate::workspace_history::WorkspaceHistoryFdReadError;

#[parsec_test(testbed = "workspace_history")]
async fn ok(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_v1_timestamp: DateTime =
        *env.template.get_stuff("wksp1_bar_txt_v1_timestamp");
    let wksp1_bar_txt_v2_timestamp: DateTime =
        *env.template.get_stuff("wksp1_bar_txt_v2_timestamp");
    let wksp1_bar_txt_v1_block_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v1_block_access");
    let wksp1_bar_txt_v2_block1_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v2_block1_access");
    let wksp1_bar_txt_v2_block2_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v2_block2_access");
    let ops = strategy.start_workspace_history_ops(env).await;

    // 0) Open `bar.txt` v1 and v2

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // Get back `/` manifest
            test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v1_timestamp, wksp1_id, wksp1_id),
            // Note workspace key bundle has already been loaded at workspace history ops startup
        );
    }
    ops.set_timestamp_of_interest(wksp1_bar_txt_v1_timestamp)
        .await
        .unwrap();

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // Get back the `bar.txt` manifest
            test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v1_timestamp, wksp1_id, wksp1_bar_txt_id),
            // Note workspace key bundle has already been loaded at workspace history ops startup
        );
    }
    let fd_v1 = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // Get back `/` manifest
            test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v2_timestamp, wksp1_id, wksp1_id),
            // Note workspace key bundle has already been loaded at workspace history ops startup
        );
    }
    ops.set_timestamp_of_interest(wksp1_bar_txt_v2_timestamp)
        .await
        .unwrap();

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // 1) Get back the `bar.txt` manifest
            test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
            // (workspace keys bundle already fetched)
        );
    }
    let fd_v2 = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();

    macro_rules! assert_fd_read {
        ($fd:expr, $offset:expr, $size:expr, $expected:expr) => {{
            let mut buf = Vec::new();
            ops.fd_read($fd, $offset, $size, &mut buf).await.unwrap();
            p_assert_eq!(buf, $expected);
        }};
    }

    // 1) Access bar.txt's v1

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v1_block_access.id),
        );
    }

    assert_fd_read!(fd_v1, 0, 100, b"Hello v1");
    assert_fd_read!(fd_v1, 0, 1, b"H");
    assert_fd_read!(fd_v1, 4, 3, b"o v");
    assert_fd_read!(fd_v1, 10, 1, b"");

    // 2) Access bar.txt's v2

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v2_block1_access.id),
            test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v2_block2_access.id),
        );
    }

    assert_fd_read!(fd_v2, 0, 100, b"Hello v2 world");
    assert_fd_read!(fd_v2, 100, 1, b"");
    assert_fd_read!(fd_v2, 10, 2, b"or");

    ops.fd_close(fd_v1).unwrap();
    ops.fd_close(fd_v2).unwrap();
}

#[parsec_test(testbed = "workspace_history")]
async fn brute_force_read_combinaisons(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    struct Area {
        name: &'static str,
        start: u64,
        end: u64,
        block_id: Option<BlockID>,
        data: Bytes,
    }
    impl Area {
        fn from_block_access(name: &'static str, block_access: &BlockAccess, data: Bytes) -> Self {
            Self {
                name,
                start: block_access.offset,
                end: block_access.offset + block_access.size.get(),
                block_id: Some(block_access.id),
                data,
            }
        }
        fn from_hole(name: &'static str, start: u64, end: u64) -> Self {
            Self {
                name,
                start,
                end,
                block_id: None,
                data: Bytes::from(vec![0; (end - start) as usize]),
            }
        }
    }

    let areas: Vec<Area> = env
        .customize(|builder| {
            // `bar.txt` v3 is a 180 bytes file splitted into 6 block slots with a blocksize of 32 bytes:
            //
            //     0         32        64        96         128        160     180 192
            //     |         |         |         |          |          |       |   |
            //     |         |         [ block 3 ][ block 4 ]          [block 6]
            //     └─┬───────┘                                           |
            //       └─ The file starts with zeros, so                   └ Block 6 is only 20 bytes long,
            //          blocks 1 & 2 don't exist.                          which is allowed since it's
            //                                                             the last one.
            //
            // Note that it is guaranteed that the block *data* have the same size than
            // what is declared in the corresponding block access, and that size must
            // be equal to the blocksize (except for the last block which can be smaller).
            //
            // In this test we want to do multiple reads with all possible combinaisons
            // of blocks involved.

            // Block slot 1 & 2 (file offset 0 to 64)

            // Nothing to do here, since zero-filled block slots are simply omitted in the manifest

            // Block slot 3 (file offset 64 to 96)

            let block3_data = Bytes::from(b"3".repeat(32));
            let block3_access = builder
                .create_block("bob@dev1", wksp1_id, block3_data.clone())
                .as_block_access(64);

            // Block slot 4 (file offset 96 to 128)

            let block4_data = Bytes::from(b"4".repeat(32));
            let block4_access = builder
                .create_block("bob@dev1", wksp1_id, block4_data.clone())
                .as_block_access(96);

            // Block slot 5 (file offset 128 to 160)

            // Nothing to do here, since zero-filled block slots are simply omitted in the manifest

            // Block slot 6 (file offset 160 to 180, i.e. end of file)

            let block6_data = Bytes::from(b"6".repeat(20));
            let block6_access = builder
                .create_block("bob@dev1", wksp1_id, block6_data.clone())
                .as_block_access(160);

            let areas = vec![
                Area::from_hole("BlockSlot1Hole", 0, 32),
                Area::from_hole("BlockSlot2Hole", 32, block3_access.offset),
                Area::from_block_access("Block3", &block3_access, block3_data),
                Area::from_block_access("Block4", &block4_access, block4_data),
                Area::from_hole(
                    "BlockSlot5Hole",
                    block4_access.offset + block4_access.size.get(),
                    block6_access.offset,
                ),
                Area::from_block_access("Block6", &block6_access, block6_data),
            ];

            builder
                .create_or_update_file_manifest_vlob(
                    "bob@dev1",
                    wksp1_id,
                    wksp1_bar_txt_id,
                    wksp1_id,
                )
                .customize(|e| {
                    let manifest = Arc::make_mut(&mut e.manifest);
                    manifest.size = 180;
                    manifest.blocksize = 32.try_into().unwrap();
                    manifest.blocks = vec![block3_access, block4_access, block6_access];
                });

            areas
        })
        .await;

    // We only run this test with the server-based access since:
    // - The test itself is not about the server access, but the upper layer
    //   that stitches together the manifests & blocks to complete a read.
    // - It would be really cumbersome to have a realm export database containing
    //   the test data.
    let ops = DataAccessStrategy::Server
        .start_workspace_history_ops(env)
        .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back `/` manifest
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_higher_bound(), wksp1_id, wksp1_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );
    ops.set_timestamp_of_interest(ops.timestamp_higher_bound())
        .await
        .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );
    let fd = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();

    // Each block is going to be fetched once then kept in cache, however it is
    // cumbersome to determine the order in which they are going to be fetched.
    // So instead we just register the fact we are going to have 3 block fetch,
    // and each one is allowed to fetch any of the 3 blocks.
    macro_rules! test_send_hook_block_read_any_bar_txt_block {
        () => {
            test_send_hook_block_read!(env, wksp1_id, allowed: [
                // Skip block slot 1 & 2 as they are holes
                areas[2].block_id.unwrap(),  // Block 3
                areas[3].block_id.unwrap(),  // Block 4
                // Skip block slot 5 as it is a hole
                areas[5].block_id.unwrap(),  // Block 5
            ])
        };
    }
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back all `bar.txt` blocks
        test_send_hook_block_read_any_bar_txt_block!(),
        test_send_hook_block_read_any_bar_txt_block!(),
        test_send_hook_block_read_any_bar_txt_block!(),
    );

    for start_area_index in 0..areas.len() {
        for end_area_index in start_area_index..areas.len() {
            // Case A: Set start/end to the bound of the area

            let start = areas[start_area_index].start;
            let end = areas[end_area_index].end;
            println!(
                "Checking: start {} ({}) end {} ({})",
                start, areas[start_area_index].name, end, areas[end_area_index].name
            );

            let expected_size = end - start;
            let mut expected: Vec<u8> = Vec::with_capacity(expected_size as usize);
            for involved_area in areas[start_area_index..=end_area_index].iter() {
                let data = involved_area.data[..(involved_area.end - involved_area.start) as usize]
                    .as_ref();
                expected.extend(data);
            }
            p_assert_eq!(expected.len(), expected_size as usize);

            let mut buf = Vec::new();
            let read_size = ops
                .fd_read(fd, start, expected_size, &mut buf)
                .await
                .unwrap();
            p_assert_eq!(read_size, expected_size);
            p_assert_eq!(buf, expected);

            // Case B: Set start/end in the middle of the area

            let start = areas[start_area_index].start
                + (areas[start_area_index].end - areas[start_area_index].start) / 2;
            let end = areas[end_area_index].start
                + (areas[end_area_index].end - areas[end_area_index].start) / 2
                + 1;
            println!(
                "Checking: start {} ({}) end {} ({})",
                start, areas[start_area_index].name, end, areas[end_area_index].name
            );

            let expected_size = end - start;
            let expected = expected[(start - areas[start_area_index].start) as usize
                ..(end - areas[start_area_index].start) as usize]
                .to_vec();
            p_assert_eq!(expected.len(), expected_size as usize);

            let mut buf = Vec::new();
            let read_size = ops
                .fd_read(fd, start, expected_size, &mut buf)
                .await
                .unwrap();
            p_assert_eq!(read_size, expected_size);
            p_assert_eq!(buf, expected);
        }
    }
}

#[parsec_test(testbed = "workspace_history")]
async fn bad_file_descriptor(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let ops = strategy.start_workspace_history_ops(env).await;

    let mut buff = vec![];
    p_assert_matches!(
        ops.fd_read(FileDescriptor(42), 0, 1, &mut buff)
            .await
            .unwrap_err(),
        WorkspaceHistoryFdReadError::BadFileDescriptor
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn reuse_after_close(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");
    let ops = strategy
        .start_workspace_history_ops_at(env, wksp1_v2_timestamp)
        .await;

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // Get back the `bar.txt` manifest
            test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
            // Note workspace key bundle has already been loaded at workspace history ops startup
        );
    }

    let fd = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();
    ops.fd_close(fd).unwrap();

    let mut buff = vec![];
    p_assert_matches!(
        ops.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::BadFileDescriptor
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn server_only_offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");
    let ops = DataAccessStrategy::Server
        .start_workspace_history_ops_at(env, wksp1_v2_timestamp)
        .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );

    let fd = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();

    let mut buff = vec![];
    p_assert_matches!(
        ops.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::Offline(_)
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
#[ignore] // TODO: how to stop certificates ops ? Can the export flavored stop ?
async fn stopped(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_block_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_block_access");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );

    let fd = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();

    // ops.certificates_ops.stop().await.unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Fetch the block...
        test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_block_access.id),
        // ...the certificate ops is stopped so nothing more happened !
    );

    let mut buff = vec![];
    p_assert_matches!(
        ops.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::Stopped
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn server_only_no_realm_access(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_block_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_block_access");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );

    let fd = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::block_read::Req| {
            p_assert_eq!(req.block_id, wksp1_bar_txt_block_access.id);
            authenticated_cmds::latest::block_read::Rep::AuthorNotAllowed
        }
    );

    let mut buff = vec![];
    p_assert_matches!(
        ops.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::NoRealmAccess
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn block_not_found(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_v1_block_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v1_block_access");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");
    let ops = strategy
        .start_workspace_history_ops_at(env, wksp1_v2_timestamp)
        .await;

    if matches!(strategy, DataAccessStrategy::RealmExport) {
        // We need to modify the realm export database to remove the block...
        // so we do it with a bit of help from our best friend Python ;-)
        std::process::Command::new("python")
            .arg("-c")
            .arg(
                "\
                import sqlite3; \
                c = sqlite3.connect('./workspace_history_export.sqlite'); \
                c.execute('DELETE FROM block'); \
                c.execute('DELETE FROM block_data'); \
                c.commit(); \
            ",
            )
            .current_dir(ops.tmp_path().expect("always defined for realm export"))
            .status()
            .unwrap();
    }

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // Get back the `bar.txt` manifest
            test_send_hook_vlob_read_batch!(env, at: wksp1_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
            // Note workspace key bundle has already been loaded at workspace history ops startup
        );
    }

    let fd = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();

    if matches!(strategy, DataAccessStrategy::Server) {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            move |req: authenticated_cmds::latest::block_read::Req| {
                p_assert_eq!(req.block_id, wksp1_bar_txt_v1_block_access.id);
                authenticated_cmds::latest::block_read::Rep::BlockNotFound
            }
        );
    }

    let mut buff = vec![];
    p_assert_matches!(
        ops.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::ServerBlockstoreUnavailable
    );
}

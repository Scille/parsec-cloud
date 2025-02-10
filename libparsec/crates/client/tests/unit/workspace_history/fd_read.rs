// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks, test_send_hook_block_read,
    test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_history_ops_with_server_access_factory;
use crate::workspace_history::WorkspaceHistoryFdReadError;

#[parsec_test(testbed = "workspace_history")]
async fn ok(env: &TestbedEnv) {
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
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    // 0) Open `bar.txt` v1 and v2

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back `/` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v1_timestamp, wksp1_id, wksp1_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );
    ops.set_timestamp_of_interest(wksp1_bar_txt_v1_timestamp)
        .await
        .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v1_timestamp, wksp1_id, wksp1_bar_txt_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );
    let fd_v1 = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back `/` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v2_timestamp, wksp1_id, wksp1_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );
    ops.set_timestamp_of_interest(wksp1_bar_txt_v2_timestamp)
        .await
        .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
        // (workspace keys bundle already fetched)
    );
    let fd_v2 = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();

    macro_rules! assert_fd_read {
        ($fd:expr, $offset:expr, $size:expr, $expected:expr) => {{
            let mut buf = Vec::new();
            ops.fd_read($fd, $offset, $size, &mut buf).await.unwrap();
            p_assert_eq!(buf, $expected);
        }};
    }

    // 1) Access bar.txt's v1

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v1_block_access.id),
    );

    assert_fd_read!(fd_v1, 0, 100, b"Hello v1");
    assert_fd_read!(fd_v1, 0, 1, b"H");
    assert_fd_read!(fd_v1, 4, 3, b"o v");
    assert_fd_read!(fd_v1, 10, 1, b"");

    // 2) Access bar.txt's v2

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v2_block1_access.id),
        test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v2_block2_access.id),
    );

    assert_fd_read!(fd_v2, 0, 100, b"Hello v2 world");
    assert_fd_read!(fd_v2, 100, 1, b"");
    assert_fd_read!(fd_v2, 10, 2, b"or");

    ops.fd_close(fd_v1).unwrap();
    ops.fd_close(fd_v2).unwrap();
}

#[parsec_test(testbed = "workspace_history")]
#[ignore] // TODO: fix this test now that we are guaranteed that block access size == block data len
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
            // `bar.txt` v3 is a 180 bytes file splitted into 5 blocks with a blocksize of 32 bytes:
            //
            //     0         32        60  64       96   100       128        160   180 192
            //     |         |         |   |        |    |         |          |     |   |
            //     [ block 1    ]
            //         │     [ block 2 ]                 [ block 3 ][ block 4 ]
            //         │       |                                              [ block 5 ]
            //         │       └─ Block 2 is 28 bytes long                            |
            //         │                                                              |
            //         └─ Block 1 is 40 bytes long          Block 5 is 32 bytes long ─┘
            //            but only its 32 first bytes       but only its 20 first bytes
            //            are used.                         are used.
            //
            // Note the block *data* can have different size than blocksize, however
            // the block accesses in the manifest must all be aligned on the blocksize
            // and have blocksize size (expect for the last block that can be smaller).
            //
            // In this test we want to do multiple reads with all possible combinaisons
            // of blocks involved.

            // File offset 0 to 32

            let block1_data = Bytes::from(b"1".repeat(40));
            let block1_access = {
                let mut block1_access = builder
                    .create_block("bob@dev1", wksp1_id, block1_data.clone())
                    .as_block_access(0);
                block1_access.size = 32.try_into().unwrap();
                block1_access
            };

            // File offset 32 to 64

            let block2_data = Bytes::from(b"2".repeat(28));
            let block2_access = builder
                .create_block("bob@dev1", wksp1_id, block2_data.clone())
                .as_block_access(32);

            // File offset 64 to 96

            // Note there is a hole between offset 60 and 100

            // File offset 96 to 128

            let block3_data = Bytes::from(b"3".repeat(28));
            let block3_access = builder
                .create_block("bob@dev1", wksp1_id, block3_data.clone())
                .as_block_access(100);

            // File offset 128 to 160

            let block4_data = Bytes::from(b"4".repeat(32));
            let block4_access = builder
                .create_block("bob@dev1", wksp1_id, block4_data.clone())
                .as_block_access(128);

            // File offset 128 to 180 (end of file)

            let block5_data = Bytes::from(b"5".repeat(32));
            let block5_access = {
                let mut block5_access = builder
                    .create_block("bob@dev1", wksp1_id, block5_data.clone())
                    .as_block_access(160);
                block5_access.size = 20.try_into().unwrap();
                block5_access
            };

            let areas = vec![
                Area::from_block_access("Block1", &block1_access, block1_data),
                Area::from_block_access("Block2", &block2_access, block2_data),
                Area::from_hole(
                    "HoleBetweenBlocks2And3",
                    block2_access.offset + block2_access.size.get(),
                    block3_access.offset,
                ),
                Area::from_block_access("Block3", &block3_access, block3_data),
                Area::from_block_access("Block4", &block4_access, block4_data),
                Area::from_block_access("Block5", &block5_access, block5_data),
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
                    manifest.blocks = vec![
                        block1_access,
                        block2_access,
                        block3_access,
                        block4_access,
                        block5_access,
                    ];
                });

            areas
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

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
    // So instead we just register the fact we are going to have 4 block fetch,
    // and each one is allowed to fetch any of the 4 blocks.
    macro_rules! test_send_hook_block_read_any_bar_txt_block {
        () => {
            test_send_hook_block_read!(env, wksp1_id, allowed: [
                areas[0].block_id.unwrap(),
                areas[1].block_id.unwrap(),
                // Skip the hole as it is has no BlockID !
                areas[3].block_id.unwrap(),
                areas[4].block_id.unwrap(),
                areas[5].block_id.unwrap(),
            ])
        };
    }
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back all `bar.txt` blocks
        test_send_hook_block_read_any_bar_txt_block!(),
        test_send_hook_block_read_any_bar_txt_block!(),
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

#[parsec_test(testbed = "minimal_client_ready")]
async fn bad_file_descriptor(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    let mut buff = vec![];
    p_assert_matches!(
        ops.fd_read(FileDescriptor(42), 0, 1, &mut buff)
            .await
            .unwrap_err(),
        WorkspaceHistoryFdReadError::BadFileDescriptor
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn reuse_after_close(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );

    let fd = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();
    ops.fd_close(fd).unwrap();

    let mut buff = vec![];
    p_assert_matches!(
        ops.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::BadFileDescriptor
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_history_ops_with_server_access_factory(env, &alice, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Get back the `bar.txt` manifest
        test_send_hook_vlob_read_batch!(env, at: ops.timestamp_of_interest(), wksp1_id, wksp1_bar_txt_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
    );

    let fd = ops.open_file_by_id(wksp1_bar_txt_id).await.unwrap();

    let mut buff = vec![];
    p_assert_matches!(
        ops.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::Offline
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
#[ignore] // TODO: `WorkspaceHistoryOps::start` does server access that crash the test...
async fn no_realm_access(env: &TestbedEnv) {
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

#[parsec_test(testbed = "minimal_client_ready")]
async fn block_not_found(env: &TestbedEnv) {
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
            authenticated_cmds::latest::block_read::Rep::BlockNotFound
        }
    );

    let mut buff = vec![];
    p_assert_matches!(
        ops.fd_read(fd, 0, 1, &mut buff).await.unwrap_err(),
        WorkspaceHistoryFdReadError::BlockNotFound
    );
}

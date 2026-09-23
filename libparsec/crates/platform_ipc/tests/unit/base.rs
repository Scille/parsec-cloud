// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::{TryLockDeviceForUseError, lock_device_for_use, try_lock_device_for_use};

fn assert_lock_in_use(config_dir: &Path, device_id: DeviceID) {
    let outcome = try_lock_device_for_use(config_dir, device_id);
    p_assert_matches!(outcome, Err(TryLockDeviceForUseError::AlreadyInUse));
}

#[parsec_test]
async fn lock_unlock_relock(tmp_path: TmpPath) {
    {
        let _l1 = lock_device_for_use(&tmp_path, ALICE_DEV1_DEVICE_ID)
            .await
            .unwrap();
        assert_lock_in_use(&tmp_path, ALICE_DEV1_DEVICE_ID);
    }

    {
        let _l2 = lock_device_for_use(&tmp_path, ALICE_DEV1_DEVICE_ID)
            .await
            .unwrap();
        assert_lock_in_use(&tmp_path, ALICE_DEV1_DEVICE_ID);
    }

    {
        let _l3 = try_lock_device_for_use(&tmp_path, ALICE_DEV1_DEVICE_ID).unwrap();
        assert_lock_in_use(&tmp_path, ALICE_DEV1_DEVICE_ID);
    }
}

#[parsec_test]
async fn lock_different_devices(tmp_path: TmpPath) {
    let _la1 = lock_device_for_use(&tmp_path, ALICE_DEV1_DEVICE_ID).await;
    let _la2 = lock_device_for_use(&tmp_path, ALICE_DEV2_DEVICE_ID).await;
    let _lb1 = try_lock_device_for_use(&tmp_path, BOB_DEV1_DEVICE_ID).unwrap();
    let _lb2 = try_lock_device_for_use(&tmp_path, BOB_DEV2_DEVICE_ID).unwrap();

    assert_lock_in_use(&tmp_path, ALICE_DEV1_DEVICE_ID);
    assert_lock_in_use(&tmp_path, ALICE_DEV2_DEVICE_ID);
    assert_lock_in_use(&tmp_path, BOB_DEV1_DEVICE_ID);
    assert_lock_in_use(&tmp_path, BOB_DEV2_DEVICE_ID);
}

#[parsec_test]
async fn wait_for_lock(tmp_path: TmpPath) {
    let task = {
        let _lock = try_lock_device_for_use(&tmp_path, ALICE_DEV1_DEVICE_ID).unwrap();

        let config_dir = (*tmp_path).to_owned();

        let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(2));
        let task = libparsec_platform_async::spawn({
            let barrier = barrier.clone();
            async move {
                barrier.wait().await;
                lock_device_for_use(&config_dir, ALICE_DEV1_DEVICE_ID).await
            }
        });

        // Here we should wait for the sub-task to start waiting for the lock, and
        // only then release it by dropping `_lock`...
        // However there is no good way to do this (it would requires polling the kernel
        // to know what syscall we are currently doing :/), so instead we use a barrier
        // then wait a bit (should be plenty considering what is running) and hope ^^
        barrier.wait().await;
        libparsec_platform_async::sleep(std::time::Duration::from_millis(10)).await;

        task
    };

    let _lock2 = task.await.unwrap().unwrap();
    assert_lock_in_use(&tmp_path, ALICE_DEV1_DEVICE_ID);
}

#[parsec_test(testbed = "minimal")]
async fn testbed_support(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    {
        let _l1 = lock_device_for_use(&env.discriminant_dir, alice.device_id)
            .await
            .unwrap();
        assert_lock_in_use(&env.discriminant_dir, alice.device_id);
    }

    {
        let _l2 = try_lock_device_for_use(&env.discriminant_dir, alice.device_id).unwrap();
        assert_lock_in_use(&env.discriminant_dir, alice.device_id);
    }
}

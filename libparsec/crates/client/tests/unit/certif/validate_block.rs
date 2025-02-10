// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook,
    test_send_hook_realm_get_keys_bundle, HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::certificates_ops_factory;
use crate::{certif::CertifValidateBlockError, InvalidBlockAccessError};

struct BaseData {
    wksp1_id: VlobID,
    wksp1_last_realm_certificate_timestamp: DateTime,
    wksp1_last_key_index: IndexInt,
    wksp1_bar_txt_manifest: Arc<FileManifest>,
    wksp1_bar_txt_block_access: BlockAccess,
    wksp1_bar_txt_block_encrypted: Bytes,
}

impl BaseData {
    fn extract(env: &TestbedEnv) -> Self {
        let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
        let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
        let wksp1_bar_txt_block_access = env
            .template
            .get_stuff::<BlockAccess>("wksp1_bar_txt_block_access")
            .to_owned();
        let wksp1_bar_txt_manifest = env
            .template
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::CreateOrUpdateFileManifestVlob(e)
                    if e.manifest.id == wksp1_bar_txt_id =>
                {
                    Some(e.manifest.clone())
                }
                _ => None,
            })
            .unwrap();
        let wksp1_bar_txt_block_encrypted = env
            .template
            .events
            .iter()
            .rev()
            .find_map(|e| match e {
                TestbedEvent::CreateBlock(e) if e.block_id == wksp1_bar_txt_block_access.id => {
                    Some(e.encrypted(&env.template))
                }
                _ => None,
            })
            .unwrap();

        let wksp1_last_realm_certificate_timestamp =
            env.get_last_realm_certificate_timestamp(wksp1_id);
        let (_, wksp1_last_key_index) = env.get_last_realm_key(wksp1_id);

        Self {
            wksp1_id,
            wksp1_last_realm_certificate_timestamp,
            wksp1_last_key_index,
            wksp1_bar_txt_manifest,
            wksp1_bar_txt_block_access,
            wksp1_bar_txt_block_encrypted,
        }
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(env: &TestbedEnv) {
    let data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, data.wksp1_id),
    );

    let block = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            data.wksp1_last_key_index,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap();

    p_assert_eq!(block, b"hello world");
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(env: &TestbedEnv) {
    let data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            data.wksp1_last_key_index,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifValidateBlockError::Offline);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(env: &TestbedEnv) {
    let data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            data.wksp1_last_key_index,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifValidateBlockError::Stopped);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_allowed(env: &TestbedEnv) {
    let data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            authenticated_cmds::latest::realm_get_keys_bundle::Rep::AuthorNotAllowed
        },
    );
    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            data.wksp1_last_key_index,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifValidateBlockError::NotAllowed);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_block_access_hash_digest_mismatch(env: &TestbedEnv) {
    let mut data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, data.wksp1_id),
    );

    data.wksp1_bar_txt_block_access.digest =
        hex!("7d486915b914332bb5730fd772223e8b276919e51edca2de0f82c5fc1bce7eb6").into();
    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            data.wksp1_last_key_index,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateBlockError::InvalidBlockAccess(boxed)
        if matches!(*boxed, InvalidBlockAccessError::HashDigestMismatch { .. })
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_block_access_size_mismatch(env: &TestbedEnv) {
    let mut data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, data.wksp1_id),
    );

    data.wksp1_bar_txt_block_access.size = (data.wksp1_bar_txt_block_access.size.get() - 1)
        .try_into()
        .unwrap();
    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            data.wksp1_last_key_index,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateBlockError::InvalidBlockAccess(boxed)
        if matches!(*boxed, InvalidBlockAccessError::SizeMismatch { .. })
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_block_access_cannot_decrypt(env: &TestbedEnv) {
    let data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, data.wksp1_id),
    );

    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            data.wksp1_last_key_index,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            b"<dummy encrypted>".as_ref(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateBlockError::InvalidBlockAccess(boxed)
        if matches!(*boxed, InvalidBlockAccessError::CannotDecrypt { .. })
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_block_access_non_existent_key_index(env: &TestbedEnv) {
    let data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, data.wksp1_id),
    );

    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            42,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateBlockError::InvalidBlockAccess(boxed)
        if matches!(*boxed, InvalidBlockAccessError::NonExistentKeyIndex { .. })
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_block_access_corrupted_key(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    // We do another key rotation to be able to provide our own keys bundle containing
    // a corrupted key for key index 1 (i.e. the key used to encrypt the block).
    env.customize(|builder| {
        builder.rotate_key_realm(wksp1_id).customize(|e| {
            // Sanity check: there should be the initial key rotation + the one we've just added
            p_assert_eq!(e.keys.len(), 2);
            // Corrupt key with index 1
            e.keys[0] = KeyDerivation::generate();
        });
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;
    let data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, data.wksp1_id),
    );

    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            1,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateBlockError::InvalidBlockAccess(boxed)
        if matches!(*boxed, InvalidBlockAccessError::CorruptedKey { .. })
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
#[case::invalid_keys_bundle(
    |env: &TestbedEnv, realm_id, user_id: UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: Bytes::from_static(b""),
        keys_bundle_access: env.get_last_realm_keys_bundle_access_for(realm_id, user_id),
    },
    |err| p_assert_matches!(err, CertifValidateBlockError::InvalidKeysBundle(_))
)]
#[case::invalid_keys_bundle_access(
    |env: &TestbedEnv, realm_id, _: UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: env.get_last_realm_keys_bundle(realm_id),
        keys_bundle_access: Bytes::from_static(b""),
    },
    |err| p_assert_matches!(err, CertifValidateBlockError::InvalidKeysBundle(_))
)]
async fn invalid_keys_bundle(
    #[case] rep: impl FnOnce(
        &TestbedEnv,
        VlobID,
        UserID,
    ) -> authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifValidateBlockError),
    env: &TestbedEnv,
) {
    let data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let rep = rep(env, data.wksp1_id, alice.user_id);
    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| rep,
    );

    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            data.wksp1_last_key_index,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal_client_ready")]
#[case::access_not_available_for_author(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::AccessNotAvailableForAuthor,
    |err| p_assert_matches!(err, CertifValidateBlockError::NotAllowed),
)]
#[case::author_not_allowed(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::AuthorNotAllowed,
    |err| p_assert_matches!(err, CertifValidateBlockError::NotAllowed),
)]
#[case::bad_key_index(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::BadKeyIndex,
    |err| p_assert_matches!(err, CertifValidateBlockError::Internal(_)),
)]
#[case::unknown_status(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::UnknownStatus { unknown_status: "".into(), reason: None },
    |err| p_assert_matches!(err, CertifValidateBlockError::Internal(_)),
)]
async fn server_error(
    #[case] rep: authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifValidateBlockError),
    env: &TestbedEnv,
) {
    let data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| rep,
    );

    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            data.wksp1_last_key_index,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_response(env: &TestbedEnv) {
    let data = BaseData::extract(env);
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::IM_A_TEAPOT,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });

    let err = ops
        .validate_block(
            data.wksp1_last_realm_certificate_timestamp,
            data.wksp1_id,
            data.wksp1_last_key_index,
            &data.wksp1_bar_txt_manifest,
            &data.wksp1_bar_txt_block_access,
            &data.wksp1_bar_txt_block_encrypted,
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifValidateBlockError::Internal(_));
}

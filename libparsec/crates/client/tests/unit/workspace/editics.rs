// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_realm_get_keys_bundle,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::{
    EncryptionUsage,
    workspace::{WorkspaceDecryptEditicsError, WorkspaceEncryptEditicsError},
};

// Flattens the payloads the way the editor host page does it (see
// `oo-encrypt-batch` in `client/editics/parent_host_api.ts`): the i-th
// payload is `flat[offsets[i]..offsets[i+1]]`.
fn flatten(payloads: &[&[u8]]) -> (Vec<u8>, Vec<u32>) {
    let mut flat = Vec::new();
    let mut offsets = vec![0u32];
    for payload in payloads {
        flat.extend_from_slice(payload);
        offsets.push(flat.len() as u32);
    }
    (flat, offsets)
}

fn unflatten(flat: &[u8], offsets: &[u32]) -> Vec<Vec<u8>> {
    offsets
        .windows(2)
        .map(|window| flat[window[0] as usize..window[1] as usize].to_vec())
        .collect()
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn batch_roundtrip(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // The realm keys bundle is not in the local storage yet: the first crypto
    // operation fetches it from the (mocked) server.
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    let document_id = VlobID::default();
    let payloads: Vec<&[u8]> = vec![b"change-0", b"", b"a longer change fragment !", b"change-3"];
    let (flat_cleartexts, offsets) = flatten(&payloads);

    let (key_index, flat_ciphertexts, ciphertext_offsets) = ops
        .editics_encrypt_batch(document_id, &flat_cleartexts, &offsets)
        .await
        .unwrap();

    let (expected_key_derivation, expected_key_index) = env.get_last_realm_key(wksp1_id);
    p_assert_eq!(key_index, expected_key_index);

    // All the payloads of the batch are sealed with the same key, derived for
    // the editics usage of the document.
    let key = expected_key_derivation
        .derive_secret_key_from_uuid(EncryptionUsage::Editics(document_id).key_derivation_uuid());
    let ciphertexts = unflatten(&flat_ciphertexts, &ciphertext_offsets);
    p_assert_eq!(ciphertexts.len(), payloads.len());
    for (ciphertext, payload) in ciphertexts.iter().zip(payloads.iter()) {
        p_assert_eq!(key.decrypt(ciphertext).unwrap(), *payload);
    }

    // The whole batch decrypts back to the original payloads, in order.
    let (roundtrip_cleartexts, roundtrip_offsets) = ops
        .editics_decrypt_batch(
            document_id,
            key_index,
            &flat_ciphertexts,
            &ciphertext_offsets,
        )
        .await
        .unwrap();
    p_assert_eq!(roundtrip_cleartexts, flat_cleartexts);
    p_assert_eq!(roundtrip_offsets, offsets);

    // Bonus: An empty batch is valid (and still resolves the key, like a non-empty
    // batch would).

    let (key_index, flat_ciphertexts, ciphertext_offsets) = ops
        .editics_encrypt_batch(document_id, &[], &[0])
        .await
        .unwrap();
    p_assert_eq!(key_index, 1);
    p_assert_eq!(flat_ciphertexts, Vec::<u8>::new());
    p_assert_eq!(ciphertext_offsets, vec![0u32]);

    let (flat_cleartexts, cleartext_offsets) = ops
        .editics_decrypt_batch(document_id, key_index, &[], &[0])
        .await
        .unwrap();
    p_assert_eq!(flat_cleartexts, Vec::<u8>::new());
    p_assert_eq!(cleartext_offsets, vec![0u32]);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_offsets(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let document_id = VlobID::default();

    const PAYLOAD: &[u8] = b"payload";

    let _do = async |offsets| {
        let err = ops
            .editics_encrypt_batch(document_id, PAYLOAD, offsets)
            .await
            .unwrap_err();
        p_assert_matches!(err, WorkspaceEncryptEditicsError::Internal(_));

        let err = ops
            .editics_decrypt_batch(document_id, 1, PAYLOAD, offsets)
            .await
            .unwrap_err();
        p_assert_matches!(err, WorkspaceDecryptEditicsError::Internal(_));
    };

    // A non-empty flat buffer must be described from offset 0.
    _do(&[1, 1]).await;

    // The offsets must stay within the flat buffer bounds.
    _do(&[0, 8]).await;
    // _do(&[0, (PAYLOAD.len() as u32 + 1)]).await;

    // The offsets must be non-decreasing.
    _do(&[0, 4, 2]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn corrupted_ciphertext(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    let document_id = VlobID::default();

    // Generate a valid ciphertext flat buffer...
    let payloads: Vec<&[u8]> = vec![b"change-0", b"change-1"];
    let (flat_cleartexts, offsets) = flatten(&payloads);
    let (key_index, mut flat_ciphertexts, ciphertext_offsets) = ops
        .editics_encrypt_batch(document_id, &flat_cleartexts, &offsets)
        .await
        .unwrap();
    // ...corrupt it...
    let second_payload_start = ciphertext_offsets[1] as usize;
    flat_ciphertexts[second_payload_start] ^= 0xff;
    // ...and finally check corruption is correctly detected!
    let err = ops
        .editics_decrypt_batch(
            document_id,
            key_index,
            &flat_ciphertexts,
            &ciphertext_offsets,
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceDecryptEditicsError::CorruptedData);
}

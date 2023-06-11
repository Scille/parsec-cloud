// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU64,
    sync::Arc,
};

use hex_literal::hex;

use libparsec_types::prelude::*;

use crate::TestbedTemplate;

/// Similar to minimal organization (i.e. single Alice user&device), but with some data:
/// - a workspace `wksp1` containing a file `/foo.txt` and a folder `/bar/`
/// - file `/foo.txt` composed of a single block with `hello world` as content
/// - Alice user, workspace and certificate storages are populated with all data
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    let alice_device_id: DeviceID = "alice@dev1".parse().unwrap();
    let alice_user_manifest_id: EntryID = hex!("a4031e8bcdd84df8ae12bd3d05e6e20f").into();
    let alice_user_manifest_key: SecretKey =
        hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a").into();
    let alice_signing_key: SigningKey =
        hex!("d544f66ece9c85d5b80275db9124b5f04bb038081622bed139c1e789c5217400").into();
    let alice_local_symkey =
        hex!("323614fc6bd2d300f42d6731059f542f89534cdca11b2cb13d5a9a5a6b19b6ac").into();
    let workspace_id: EntryID = hex!("bb2698816b3e47bab2d2247ac68a68cc").into();
    let workspace_key: SecretKey =
        hex!("c36798c080f3f9657d609d1d4428e98a769e6f2ea3cce045544c9b6dbc304487").into();
    let file_id: EntryID = hex!("d107a52582d2fe68a3b0dfe164a3c2c0").into();
    let file_block_id: BlockID = hex!("8af5e0c038e51fb8dbf1f78bb280fddf").into();
    let file_block_key: SecretKey =
        hex!("99b7320b301f4f670ec6cb9b5173db2896b1aedb4dbb752021858a7086021326").into();

    let file_content = b"Hello, World!";
    let file_block = file_block_key.encrypt(file_content);

    let create_file_vlob_timestamp = "2000-01-05T00:00:00Z".parse().unwrap();
    let (file_vlob, file_local_encrypted) = {
        let manifest = FileManifest {
            author: alice_device_id.clone(),
            timestamp: create_file_vlob_timestamp,
            id: file_id,
            parent: workspace_id,
            version: 1,
            created: create_file_vlob_timestamp,
            updated: create_file_vlob_timestamp,
            blocksize: DEFAULT_BLOCK_SIZE,
            size: file_content.len() as SizeInt,
            blocks: vec![BlockAccess {
                id: file_block_id,
                key: file_block_key,
                offset: 0,
                size: NonZeroU64::new(file_content.len() as u64).unwrap(),
                digest: HashDigest::from_data(&file_block),
            }],
        };
        let vlob = manifest.dump_sign_and_encrypt(&alice_signing_key, &workspace_key);

        let local_manifest = LocalFileManifest {
            size: manifest.size,
            blocksize: manifest.blocksize,
            need_sync: false,
            updated: create_file_vlob_timestamp,
            blocks: vec![vec![Chunk {
                id: file_block_id.into(),
                start: 0,
                stop: NonZeroU64::new(manifest.size).unwrap(),
                raw_offset: 0,
                raw_size: NonZeroU64::new(manifest.size).unwrap(),
                access: Some(manifest.blocks[0].clone()),
            }]],
            base: manifest,
        };
        let local_encrypted = local_manifest.dump_and_encrypt(&alice_local_symkey);

        (vlob, local_encrypted)
    };

    let create_workspace_vlob_timestamp = "2000-01-06T00:00:00Z".parse().unwrap();
    let (workspace_vlob, workspace_local_encrypted) = {
        let manifest = WorkspaceManifest {
            author: alice_device_id.clone(),
            timestamp: create_workspace_vlob_timestamp,
            id: workspace_id,
            version: 1,
            created: create_workspace_vlob_timestamp,
            updated: create_workspace_vlob_timestamp,
            children: HashMap::from([("wksp1".parse().unwrap(), workspace_id)]),
        };
        let vlob = manifest.dump_sign_and_encrypt(&alice_signing_key, &workspace_key);

        let local_manifest = LocalWorkspaceManifest {
            speculative: false,
            need_sync: false,
            updated: create_workspace_vlob_timestamp,
            children: manifest.children.clone(),
            local_confinement_points: HashSet::default(),
            remote_confinement_points: HashSet::default(),
            base: manifest,
        };
        let local_encrypted = local_manifest.dump_and_encrypt(&alice_local_symkey);

        (vlob, local_encrypted)
    };

    let create_user_vlob_timestamp = "2000-01-08T00:00:00Z".parse().unwrap();
    let (user_manifest_blob, user_manifest_local_encrypted) = {
        let manifest = UserManifest {
            author: alice_device_id.clone(),
            timestamp: create_user_vlob_timestamp,
            id: alice_user_manifest_id,
            version: 1,
            created: create_user_vlob_timestamp,
            updated: create_user_vlob_timestamp,
            last_processed_message: 0,
            workspaces: vec![WorkspaceEntry {
                id: workspace_id,
                name: "wksp1".parse().unwrap(),
                encryption_revision: 1,
                encrypted_on: create_user_vlob_timestamp,
                legacy_role_cache_timestamp: create_user_vlob_timestamp,
                legacy_role_cache_value: None,
                key: workspace_key.clone(),
            }],
        };
        let vlob = manifest.dump_sign_and_encrypt(&alice_signing_key, &alice_user_manifest_key);

        let local_manifest = LocalUserManifest {
            speculative: false,
            need_sync: false,
            updated: create_user_vlob_timestamp,
            last_processed_message: manifest.last_processed_message,
            workspaces: manifest.workspaces.clone(),
            base: manifest,
        };
        let local_encrypted = local_manifest.dump_and_encrypt(&alice_local_symkey);

        (vlob, local_encrypted)
    };

    TestbedTemplate::from_builder("alice_cruising")
        .bootstrap_organization(
            hex!("b62e7d2a9ed95187975294a1afb1ba345a79e4beb873389366d6c836d20ec5bc"),
            None,
            alice_device_id.clone(),
            Some("Alicey McAliceFace <alice@example.com>"),
            hex!("74e860967fd90d063ebd64fb1ba6824c4c010099dd37508b7f2875a5db2ef8c9"),
            Some("My dev1 machine"),
            alice_signing_key,
            alice_user_manifest_id,
            alice_user_manifest_key.clone(),
            alice_local_symkey,
            "P@ssw0rd.",
        )
        // 2000-01-03: create workspace's realm
        .new_realm(alice_device_id.clone(), workspace_id, workspace_key)
        // 2000-01-04: upload file content's block
        .new_block(
            alice_device_id.clone(),
            workspace_id,
            file_block_id,
            file_block,
        )
        // 2000-01-05: upload file's vlob
        .new_vlob(
            alice_device_id.clone(),
            workspace_id,
            file_id,
            file_vlob,
            None,
        )
        // 2000-01-06: upload workspace's vlob
        .new_vlob(
            alice_device_id.clone(),
            workspace_id,
            workspace_id,
            workspace_vlob,
            None,
        )
        // 2000-01-07: create user manifest's realm
        .new_realm(
            alice_device_id.clone(),
            alice_user_manifest_id,
            alice_user_manifest_key,
        )
        // 2000-01-08: upload user manifest
        .new_vlob(
            alice_device_id.clone(),
            alice_user_manifest_id,
            alice_user_manifest_id,
            user_manifest_blob,
            None,
        )
        // Local storage fetch all data
        .certificates_storage_fetch_certificates(alice_device_id.clone(), None)
        .user_storage_fetch_user_vlob(alice_device_id.clone(), None)
        .workspace_storage_fetch_vlob(
            alice_device_id.clone(),
            alice_user_manifest_id,
            alice_user_manifest_id,
            None,
        )
        .workspace_storage_fetch_vlob(alice_device_id.clone(), workspace_id, workspace_id, None)
        .workspace_storage_fetch_vlob(alice_device_id.clone(), workspace_id, file_id, None)
        .workspace_storage_fetch_block(alice_device_id.clone(), workspace_id, file_block_id)
        // Create local manifests data with no changes
        .user_storage_local_update(alice_device_id.clone(), user_manifest_local_encrypted)
        .workspace_storage_local_update(
            alice_device_id.clone(),
            workspace_id,
            workspace_id,
            workspace_local_encrypted,
        )
        .workspace_storage_local_update(
            alice_device_id,
            workspace_id,
            file_id,
            file_local_encrypted,
        )
        .finalize()
}

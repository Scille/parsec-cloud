// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/// Register a `vlob_read_batch` RPC call.
///
/// Notes:
/// - Use the `allowed: [...]` parameter in case the order the client will query multiple
///   entries is not guaranteed (typically the case when iterating over children).
/// - Use `at: <DateTime>` to specify the value fo the `at` field the client is
///   expected to provide in the `vlob_read_batch` request.
/// - Use `server_up_to: <DateTime>` to limit the template events used. This is useful to
///   have the client fetch an old version of a vlob (using `client_up_to: <old vlob timestamp>`).
/// - If `at` is specified but not `server_up_to`, then `server_up_to` takes `at` value.
/// - No access control is done ! So only use this for a user that has access to the realm.
/// - Non-existence of `$realm_id` or `$entry_id` is handled fine.
#[macro_export]
macro_rules! test_send_hook_vlob_read_batch {
    (__internal, $strict: expr, $env: expr, $server_up_to:expr, $at:expr, $realm_id: expr, $($entry_id: expr),* $(,)?) => {
        {
            let env: &TestbedEnv = $env;
            let realm_id: VlobID = $realm_id;
            let at: Option<DateTime> = $at;
            let server_up_to: Option<DateTime> = $server_up_to;
            let server_up_to = server_up_to.or(at);

            let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
            let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(realm_id);
            let last_realm_key_index = env.get_last_realm_keys_bundle_index(realm_id);

            let mk_rev_iter_events = || env.template.events.iter().rev()
                .filter(|e| match server_up_to {
                    None => true,
                    Some(up_to) => match e {
                        TestbedEvent::CreateOrUpdateUserManifestVlob(e) => e.manifest.timestamp <= up_to,
                        TestbedEvent::CreateOrUpdateFileManifestVlob(e) => e.manifest.timestamp <= up_to,
                        TestbedEvent::CreateOrUpdateFolderManifestVlob(e) => e.manifest.timestamp <= up_to,
                        TestbedEvent::CreateOrUpdateOpaqueVlob(e) => e.timestamp <= up_to,
                        _ => false,
                    }
                });

            let mut items = vec![];
            let mut expected_vlobs_param = vec![];
            $(
                let entry_id: VlobID = $entry_id;
                expected_vlobs_param.push(entry_id);
                let maybe_found = mk_rev_iter_events()
                    .find_map(|e| match e {
                        TestbedEvent::CreateOrUpdateUserManifestVlob(e)
                        if e.manifest.id == realm_id && e.manifest.id == entry_id => {
                            Some((
                                0,
                                e.manifest.author,
                                e.manifest.version,
                                e.manifest.timestamp,
                                e.encrypted(&env.template)
                            ))
                        }
                        TestbedEvent::CreateOrUpdateFileManifestVlob(e)
                            if e.realm == realm_id && e.manifest.id == entry_id =>
                        {
                            Some((
                                e.key_index,
                                e.manifest.author,
                                e.manifest.version,
                                e.manifest.timestamp,
                                e.encrypted(&env.template)
                            ))
                        }
                        TestbedEvent::CreateOrUpdateFolderManifestVlob(e)
                            if e.realm == realm_id && e.manifest.id == entry_id =>
                        {
                            Some((
                                e.key_index,
                                e.manifest.author,
                                e.manifest.version,
                                e.manifest.timestamp,
                                e.encrypted(&env.template)
                            ))
                        }
                        TestbedEvent::CreateOrUpdateOpaqueVlob(e)
                            if e.realm == realm_id && e.vlob_id == entry_id => {
                            Some((
                                e.key_index,
                                e.author,
                                e.version,
                                e.timestamp,
                                e.encrypted.clone(),
                            ))
                        }
                        _ => None,
                    });

                if let Some((
                    manifest_encrypted_key_index,
                    manifest_author,
                    manifest_version,
                    manifest_timestamp,
                    manifest_encrypted,
                )) = maybe_found {
                    items.push(
                        (
                            entry_id,
                            manifest_encrypted_key_index,
                            manifest_author,
                            manifest_version,
                            manifest_timestamp,
                            manifest_encrypted,
                        )
                    );
                }
            )*

            let realm_exists = if items.is_empty() {
                // If we didn't find any item, it might be because the realm doesn't
                // exist yet, in which case we should return an error
                env.template.events.iter().find_map(|e| match (e, server_up_to) {
                    (TestbedEvent::NewRealm(e), Some(up_to)) if e.timestamp > up_to => Some(false),
                    (TestbedEvent::NewRealm(e), _) if e.realm_id == realm_id => Some(true),
                    _ => None,
                }).unwrap_or(false)
            } else {
                true
            };
            let mut rep = if realm_exists {
                $crate::protocol::authenticated_cmds::latest::vlob_read_batch::Rep::Ok {
                    items,
                    needed_common_certificate_timestamp: last_common_certificate_timestamp,
                    needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
                }
            } else {
                $crate::protocol::authenticated_cmds::latest::vlob_read_batch::Rep::RealmNotFound
            };

            move |req: $crate::protocol::authenticated_cmds::latest::vlob_read_batch::Req| {
                p_assert_eq!(req.realm_id, realm_id);
                p_assert_eq!(req.at, at);
                if $strict {
                    // In strict mode, the client should have requested all the entries.
                    p_assert_eq!(req.vlobs, expected_vlobs_param);
                } else {
                    // In non-strict mode, the client can have requested a subset of the entries.
                    if let $crate::protocol::authenticated_cmds::latest::vlob_read_batch::Rep::Ok{items, ..} = &mut rep {
                        items.retain(|(entry_id, _, _, _, _, _)| req.vlobs.contains(entry_id));
                    }
                }
                rep
            }
        }
    };

    // Base
    ($env: expr, $realm_id: expr, $($entry_id: expr),* $(,)?) => {
        test_send_hook_vlob_read_batch!(__internal, true, $env, None, None, $realm_id, $($entry_id),*)
    };
    // at
    ($env: expr, at: $at: expr, $realm_id: expr, $($entry_id: expr),* $(,)?) => {
        test_send_hook_vlob_read_batch!(__internal, true, $env, None, Some($at), $realm_id, $($entry_id),*)
    };
    // allowed
    ($env: expr, $realm_id: expr, allowed: [$($entry_id: expr),* $(,)?] $(,)?) => {
        test_send_hook_vlob_read_batch!(__internal, false, $env, None, None, $realm_id, $($entry_id),*)
    };
    // at + allowed
    ($env: expr, at: $at: expr, $realm_id: expr, allowed: [$($entry_id: expr),* $(,)?] $(,)?) => {
        test_send_hook_vlob_read_batch!(__internal, false, $env, None, Some($at), $realm_id, $($entry_id),*)
    };
    // server_up_to
    ($env: expr, server_up_to: $server_up_to: expr, $realm_id: expr, $($entry_id: expr),* $(,)?) => {
        test_send_hook_vlob_read_batch!(__internal, true, $env, Some($server_up_to), None, $realm_id, $($entry_id),*)
    };
    // server_up_to + at
    ($env: expr, server_up_to: $server_up_to: expr, at: $at: expr, $realm_id: expr, $($entry_id: expr),* $(,)?) => {
        test_send_hook_vlob_read_batch!(__internal, true, $env, Some($server_up_to), Some($at), $realm_id, $($entry_id),*)
    };
    // server_up_to + allowed
    ($env: expr, server_up_to: $server_up_to: expr, $realm_id: expr, allowed: [$($entry_id: expr),* $(,)?] $(,)?) => {
        test_send_hook_vlob_read_batch!(__internal, false, $env, Some($server_up_to), None, $realm_id, $($entry_id),*)
    };
    // server_up_to + at + allowed
    ($env: expr, server_up_to: $server_up_to: expr, at: $at: expr, $realm_id: expr, allowed: [$($entry_id: expr),* $(,)?] $(,)?) => {
        test_send_hook_vlob_read_batch!(__internal, false, $env, Some($server_up_to), Some($at), $realm_id, $($entry_id),*)
    };
}

/// Register a `vlob_read_versions` RPC call.
///
/// Notes:
/// - Use the `allowed: [...]` parameter in case the order the client will query multiple
///   entries is not guaranteed (typically the case when iterating over children).
/// - Use `server_up_to: <DateTime>` to limit the template events used. This is useful to
///   ignore newer versions of a vlob (using `client_up_to: <old vlob timestamp>`).
/// - No access control is done ! So only use this for a user that has access to the realm.
/// - Non-existence of `$realm_id` or `$entry_id`/`$entry_version` is handled fine.
#[macro_export]
macro_rules! test_send_hook_vlob_read_versions {
    (__internal, $strict: expr, $env: expr, $server_up_to:expr, $realm_id: expr, $(($entry_id: expr, $entry_version: expr)),* $(,)?) => {
        {
            let env: &TestbedEnv = $env;
            let realm_id: VlobID = $realm_id;
            let server_up_to: Option<DateTime> = $server_up_to;

            let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
            let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(realm_id);
            let last_realm_key_index = env.get_last_realm_keys_bundle_index(realm_id);

            let mk_rev_iter_events = || env.template.events.iter().rev()
                .filter(|e| match server_up_to {
                    None => true,
                    Some(up_to) => match e {
                        TestbedEvent::CreateOrUpdateUserManifestVlob(e) => e.manifest.timestamp <= up_to,
                        TestbedEvent::CreateOrUpdateFileManifestVlob(e) => e.manifest.timestamp <= up_to,
                        TestbedEvent::CreateOrUpdateFolderManifestVlob(e) => e.manifest.timestamp <= up_to,
                        TestbedEvent::CreateOrUpdateOpaqueVlob(e) => e.timestamp <= up_to,
                        _ => false,
                    }
                });

            let mut items = vec![];
            let mut expected_items_param = vec![];
            $(
                let entry_id: VlobID = $entry_id;
                let entry_version: VersionInt = $entry_version;
                expected_items_param.push((entry_id, entry_version));
                let maybe_found = mk_rev_iter_events()
                    .find_map(|e| match e {
                        TestbedEvent::CreateOrUpdateUserManifestVlob(e)
                        if e.manifest.id == realm_id && e.manifest.id == entry_id && e.manifest.version == entry_version => {
                            Some((
                                0,
                                e.manifest.author,
                                e.manifest.version,
                                e.manifest.timestamp,
                                e.encrypted(&env.template)
                            ))
                        }
                        TestbedEvent::CreateOrUpdateFileManifestVlob(e)
                            if e.realm == realm_id && e.manifest.id == entry_id && e.manifest.version == entry_version =>
                        {
                            Some((
                                e.key_index,
                                e.manifest.author,
                                e.manifest.version,
                                e.manifest.timestamp,
                                e.encrypted(&env.template)
                            ))
                        }
                        TestbedEvent::CreateOrUpdateFolderManifestVlob(e)
                            if e.realm == realm_id && e.manifest.id == entry_id && e.manifest.version == entry_version =>
                        {
                            Some((
                                e.key_index,
                                e.manifest.author,
                                e.manifest.version,
                                e.manifest.timestamp,
                                e.encrypted(&env.template)
                            ))
                        }
                        TestbedEvent::CreateOrUpdateOpaqueVlob(e)
                            if e.realm == realm_id && e.vlob_id == entry_id && e.version == entry_version => {
                            Some((
                                e.key_index,
                                e.author,
                                e.version,
                                e.timestamp,
                                e.encrypted.clone(),
                            ))
                        }
                        _ => None,
                    });

                if let Some((
                    manifest_encrypted_key_index,
                    manifest_author,
                    manifest_version,
                    manifest_timestamp,
                    manifest_encrypted,
                )) = maybe_found {
                    items.push(
                        (
                            entry_id,
                            manifest_encrypted_key_index,
                            manifest_author,
                            manifest_version,
                            manifest_timestamp,
                            manifest_encrypted,
                        )
                    );
                }
            )*

            let realm_exists = if items.is_empty() {
                // If we didn't find any item, it might be because the realm doesn't
                // exist yet, in which case we should return an error
                env.template.events.iter().find_map(|e| match (e, server_up_to) {
                    (TestbedEvent::NewRealm(e), Some(up_to)) if e.timestamp > up_to => Some(false),
                    (TestbedEvent::NewRealm(e), _) if e.realm_id == realm_id => Some(true),
                    _ => None,
                }).unwrap_or(false)
            } else {
                true
            };
            let mut rep = if realm_exists {
                $crate::protocol::authenticated_cmds::latest::vlob_read_versions::Rep::Ok {
                    items,
                    needed_common_certificate_timestamp: last_common_certificate_timestamp,
                    needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
                }
            } else {
                $crate::protocol::authenticated_cmds::latest::vlob_read_versions::Rep::RealmNotFound
            };

            move |req: $crate::protocol::authenticated_cmds::latest::vlob_read_versions::Req| {
                p_assert_eq!(req.realm_id, realm_id);
                if $strict {
                    // In strict mode, the client should have requested all the entries.
                    p_assert_eq!(req.items, expected_items_param);
                } else {
                    // In non-strict mode, the client can have requested a subset of the entries.
                    if let $crate::protocol::authenticated_cmds::latest::vlob_read_versions::Rep::Ok{items, ..} = &mut rep {
                        items.retain(|(entry_id, _, _, entry_version, _, _)| req.items.contains(&(*entry_id, *entry_version)));
                    }
                }
                rep
            }
        }
    };

    // Base
    ($env: expr, $realm_id: expr, $(($entry_id: expr, $entry_version: expr)),* $(,)?) => {
        test_send_hook_vlob_read_versions!(__internal, true, $env, None, $realm_id, $(($entry_id, $entry_version)),*)
    };
    // allowed
    ($env: expr, $realm_id: expr, allowed: [$(($entry_id: expr, $entry_version: expr)),* $(,)?] $(,)?) => {
        test_send_hook_vlob_read_versions!(__internal, false, $env, None, $realm_id, $(($entry_id, $entry_version)),*)
    };
    // server_up_to
    ($env: expr, server_up_to: $server_up_to: expr, $realm_id: expr, $(($entry_id: expr, $entry_version: expr)),* $(,)?) => {
        test_send_hook_vlob_read_versions!(__internal, true, $env, Some($server_up_to), $realm_id, $(($entry_id, $entry_version)),*)
    };
    // server_up_to + allowed
    ($env: expr, server_up_to: $server_up_to: expr, $realm_id: expr, allowed: [$(($entry_id: expr, $entry_version: expr)),* $(,)?] $(,)?) => {
        test_send_hook_vlob_read_versions!(__internal, false, $env, Some($server_up_to), $realm_id, $(($entry_id, $entry_version)),*)
    };
}

/// Register a `realm_get_keys_bundle` RPC call.
///
/// Notes:
/// - If `$key_index` is not provided, the last one is used.
/// - No access control is done ! So only use this for a user that has access to the realm.
#[macro_export]
macro_rules! test_send_hook_realm_get_keys_bundle {
    (__internal, $env: expr, $author: expr, $realm_id: expr, $key_index: expr) => {{
        let env: &TestbedEnv = $env;
        let realm_id: VlobID = $realm_id;
        let author: UserID = $author;

        let (key_index, keys_bundle, keys_bundle_access) = match $key_index {
            None => {
                let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
                let key_index = env.get_last_realm_keys_bundle_index(realm_id);
                let keys_bundle_access =
                    env.get_last_realm_keys_bundle_access_for(realm_id, author);
                (key_index, keys_bundle, keys_bundle_access)
            }
            Some(key_index) => {
                let keys_bundle = env.get_realm_keys_bundle(realm_id, key_index);
                let keys_bundle_access =
                    env.get_keys_bundle_access_for(realm_id, author, key_index);
                (key_index, keys_bundle, keys_bundle_access)
            }
        };

        move |req: $crate::protocol::authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            p_assert_eq!(req.realm_id, realm_id);
            p_assert_eq!(req.key_index, key_index);
            $crate::protocol::authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            }
        }
    }};

    ($env: expr, $author: expr, $realm_id: expr) => {
        test_send_hook_realm_get_keys_bundle!(__internal, $env, $author, $realm_id, None);
    };
    ($env: expr, $author: expr, $realm_id: expr, $key_index: expr) => {
        test_send_hook_realm_get_keys_bundle!(
            __internal,
            $env,
            $author,
            $realm_id,
            Some($key_index)
        );
    };
}

/// Register a `block_read` RPC call.
///
/// Notes:
/// - No access control is done ! So only use this for a user that has access to the realm.
/// - Use the `allowed: [...]` parameter in case the order the client will query multiple
///   blocks is not guaranteed.
#[macro_export]
macro_rules! test_send_hook_block_read {
    ($env: expr, $realm_id: expr, allowed: [$($block_id: expr),* $(,)?] $(,)?) => {{
        let env: &TestbedEnv = $env;
        let realm_id: VlobID = $realm_id;
        let allowed_block_ids: Vec<BlockID> = vec![$($block_id),*];

        let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(realm_id);
        let allowed_reps = env
            .template
            .events
            .iter()
            .rev()
            .filter_map(|e| match e {
                TestbedEvent::CreateBlock(e) if allowed_block_ids.contains(&e.block_id) => {
                    let rep = $crate::protocol::authenticated_cmds::latest::block_read::Rep::Ok {
                        needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
                        key_index: e.key_index,
                        block: e.encrypted(&env.template),
                    };
                    Some((e.block_id, rep))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        move |req: $crate::protocol::authenticated_cmds::latest::block_read::Req| {
            for (block_id, rep) in allowed_reps.into_iter() {
                if block_id == req.block_id {
                    return rep;
                }
            }
            panic!("Unexpected block_id: {:?} (expected: {:?})", req.block_id, allowed_block_ids);
        }
    }};
    ($env: expr, $realm_id: expr, $block_id: expr) => {
        test_send_hook_block_read!($env, $realm_id, allowed: [$block_id])
    };
}

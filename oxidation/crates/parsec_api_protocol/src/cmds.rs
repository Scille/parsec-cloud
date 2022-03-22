// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

// This macro implements dumps/loads methods for client/server side.
// It checks if both Req and Rep are implemented for a specified command
// It also provides a way to use commands by specifying status, command and type.
// For example:
// Server side
// authenticated_cmds::AnyCmdReq::loads(..)
// authenticated_cmds::block_create::Rep::Ok.dumps()
// Client side
// authenticated_cmds::block_create::Req { .. }.dumps()
// authenticated_cmds::block_create::Rep::loads(..)
macro_rules! cmds_bundle {
    ($cmds: ident, [$(($path: path, $module: ident, $cmd: ident $(, $rename: literal)? $(,)?),)*]) => {

        ::paste::paste! {
            pub mod $cmds {
                use ::serde::{Deserialize, Serialize};

                #[derive(Serialize, Deserialize, PartialEq, Debug)]
                #[serde(tag = "cmd", rename_all = "snake_case")]
                pub enum AnyCmdReq {
                    $(
                        $(#[serde(rename = $rename)])?
                        $cmd(crate::[<$cmd Req>]),
                    )*
                }

                crate::impl_dumps_loads!(AnyCmdReq);

                $(
                    pub mod $module {
                        pub use $path::{[<$cmd Req>] as Req, [<$cmd Rep>] as Rep};
                        use super::AnyCmdReq;

                        impl Req {
                            pub fn dumps(self) -> Result<Vec<u8>, &'static str> {
                                ::rmp_serde::to_vec_named(&AnyCmdReq::$cmd(self))
                                    .map_err(|_| "Serialization failed")
                            }
                        }

                        crate::impl_dumps_loads_for_rep!(Rep);

                        #[test]
                        fn test_unknown_error() {
                            let raw = b"foobar";
                            let data = Rep::loads(raw);

                            match data {
                                Rep::UnknownError { .. } => assert!(true),
                                _ => assert!(false),
                            }
                        }
                    }
                )*
            }
        }
    };
}

cmds_bundle!(
    authenticated_cmds,
    [
        (crate::ping, ping, AuthenticatedPing, "ping"),
        // Block
        (crate::block, block_create, BlockCreate),
        (crate::block, block_read, BlockRead),
        // Events
        (crate::events, events_listen, EventsListen),
        (crate::events, events_subscribe, EventsSubscribe),
        // Invite
        (crate::invite, invite_new, InviteNew),
        (crate::invite, invite_delete, InviteDelete),
        (crate::invite, invite_list, InviteList),
        (
            crate::invite,
            invite_1_greeter_wait_peer,
            Invite1GreeterWaitPeer,
            "invite_1_greeter_wait_peer",
        ),
        (
            crate::invite,
            invite_2a_greeter_get_hashed_nonce,
            Invite2aGreeterGetHashedNonce,
            "invite_2a_greeter_get_hashed_nonce",
        ),
        (
            crate::invite,
            invite_2b_greeter_send_nonce,
            Invite2bGreeterSendNonce,
            "invite_2b_greeter_send_nonce",
        ),
        (
            crate::invite,
            invite_3a_greeter_wait_peer_trust,
            Invite3aGreeterWaitPeerTrust,
            "invite_3a_greeter_wait_peer_trust",
        ),
        (
            crate::invite,
            invite_3b_greeter_signify_trust,
            Invite3bGreeterSignifyTrust,
            "invite_3b_greeter_signify_trust",
        ),
        (
            crate::invite,
            invite_4_greeter_communicate,
            Invite4GreeterCommunicate,
            "invite_4_greeter_communicate",
        ),
        // Message
        (crate::message, message_get, MessageGet),
        // Organization
        (crate::organization, organization_stats, OrganizationStats), // organization_stats has been added in api v2.1
        (crate::organization, organization_config, OrganizationConfig), // organization_config has been added in api v2.2
        // Realm
        (crate::realm, realm_create, RealmCreate),
        (crate::realm, realm_stats, RealmStats),
        (crate::realm, realm_status, RealmStatus),
        (
            crate::realm,
            realm_get_role_certificates,
            RealmGetRoleCertificates
        ),
        (crate::realm, realm_update_roles, RealmUpdateRoles),
        (
            crate::realm,
            realm_start_reencryption_maintenance,
            RealmStartReencryptionMaintenance
        ),
        (
            crate::realm,
            realm_finish_reencryption_maintenance,
            RealmFinishReencryptionMaintenance
        ),
        // User
        (crate::user, user_get, UserGet),
        (crate::user, user_create, UserCreate),
        (crate::user, user_revoke, UserRevoke),
        (crate::user, device_create, DeviceCreate),
        // Human
        (crate::user, human_find, HumanFind),
        // Vlob
        (crate::vlob, vlob_poll_changes, VlobPollChanges),
        (crate::vlob, vlob_create, VlobCreate),
        (crate::vlob, vlob_read, VlobRead),
        (crate::vlob, vlob_update, VlobUpdate),
        (crate::vlob, vlob_list_versions, VlobListVersions),
        (
            crate::vlob,
            vlob_maintenance_get_reencryption_batch,
            VlobMaintenanceGetReencryptionBatch
        ),
        (
            crate::vlob,
            vlob_maintenance_save_reencryption_batch,
            VlobMaintenanceSaveReencryptionBatch
        ),
    ]
);

cmds_bundle!(
    invited_cmds,
    [
        (crate::ping, ping, InvitedPing, "ping"),
        (crate::invite, invite_info, InviteInfo),
        (
            crate::invite,
            invite_1_claimer_wait_peer,
            Invite1ClaimerWaitPeer,
            "invite_1_claimer_wait_peer",
        ),
        (
            crate::invite,
            invite_2a_claimer_send_hashed_nonce_hash_nonce,
            Invite2aClaimerSendHashedNonceHashNonce,
            "invite_2a_claimer_send_hashed_nonce_hash_nonce",
        ),
        (
            crate::invite,
            invite_2b_claimer_send_nonce,
            Invite2bClaimerSendNonce,
            "invite_2b_claimer_send_nonce",
        ),
        (
            crate::invite,
            invite_3a_claimer_signify_trust,
            Invite3aClaimerSignifyTrust,
            "invite_3a_claimer_signify_trust",
        ),
        (
            crate::invite,
            invite_3b_claimer_wait_peer_trust,
            Invite3bClaimerWaitPeerTrust,
            "invite_3b_claimer_wait_peer_trust",
        ),
        (
            crate::invite,
            invite_4_claimer_communicate,
            Invite4ClaimerCommunicate,
            "invite_4_claimer_communicate",
        ),
    ]
);

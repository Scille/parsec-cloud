// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use clap::Parser;

use parsec_api_protocol::authenticated_cmds;
use parsec_api_protocol::invited_cmds;

const AUTHENTICATED_CMDS: &str = r#"
ping_req
ping_rep
block_create_req
block_create_rep
events_listen_req
events_listen_rep
invite_new_req
invite_new_rep
invite_delete_req
invite_delete_rep
invite_list_req
invite_list_rep
invite_1_greeter_wait_peer_req
invite_1_greeter_wait_peer_rep
invite_2a_greeter_get_hashed_nonce_req
invite_2a_greeter_get_hashed_nonce_rep
invite_2b_greeter_send_nonce_req
invite_2b_greeter_send_nonce_rep
invite_3a_greeter_wait_peer_trust_req
invite_3a_greeter_wait_peer_trust_rep
invite_3b_greeter_signify_trust_req
invite_3b_greeter_signify_trust_rep
invite_4_greeter_communicate_req
invite_4_greeter_communicate_rep
message_get_req
message_get_rep
organization_stats_req
organization_stats_rep
organization_config_req
organization_config_rep
realm_create_req
realm_create_rep
realm_stats_req
realm_stats_rep
realm_status_req
realm_status_rep
realm_get_role_certificates_req
realm_get_role_certificates_rep
realm_update_roles_req
realm_update_roles_rep
realm_start_reencryption_maintenance_req
realm_start_reencryption_maintenance_rep
realm_finish_reencryption_maintenance_req
realm_finish_reencryption_maintenance_rep
user_get_req
user_get_rep
user_create_req
user_create_rep
user_revoke_req
user_revoke_rep
device_create_req
device_create_rep
human_find_req
human_find_rep
vlob_poll_changes_req
vlob_poll_changes_rep
vlob_create_req
vlob_create_rep
vlob_read_req
vlob_read_rep
vlob_update_req
vlob_update_rep
vlob_list_versions_req
vlob_list_versions_rep
vlob_maintenance_get_reencryption_batch_req
vlob_maintenance_get_reencryption_batch_rep
vlob_maintenance_save_reencryption_batch_req
vlob_maintenance_save_reencryption_batch_rep
"#;

const INVITED_CMDS: &str = r#"
ping_req
ping_rep
invite_info_req
invite_info_rep
invite_1_claimer_wait_peer_req
invite_1_claimer_wait_peer_rep
invite_2a_claimer_send_hashed_nonce_hash_nonce_req
invite_2a_claimer_send_hashed_nonce_hash_nonce_rep
invite_2b_claimer_send_nonce_req
invite_2b_claimer_send_nonce_rep
invite_3a_claimer_signify_trust_req
invite_3a_claimer_signify_trust_rep
invite_3b_claimer_wait_peer_trust_req
invite_3b_claimer_wait_peer_trust_rep
invite_4_claimer_communicate_req
invite_4_claimer_communicate_rep
"#;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// authenticated
    /// invited
    #[clap(short, long)]
    cmd: String,

    /// --list to show all
    #[clap(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();

    match args.cmd.as_str() {
        "authenticated" => {
            use authenticated_cmds::*;
            let specs = match args.name.as_str() {
                "ping_req" => ping::Req::specs(),
                "ping_rep" => ping::Rep::specs(),
                "block_create_req" => block_create::Req::specs(),
                "block_create_rep" => block_create::Rep::specs(),
                "events_listen_req" => events_listen::Req::specs(),
                "events_listen_rep" => events_listen::Rep::specs(),
                "invite_new_req" => invite_new::Req::specs(),
                "invite_new_rep" => invite_new::Rep::specs(),
                "invite_delete_req" => invite_delete::Req::specs(),
                "invite_delete_rep" => invite_delete::Rep::specs(),
                "invite_list_req" => invite_list::Req::specs(),
                "invite_list_rep" => invite_list::Rep::specs(),
                "invite_1_greeter_wait_peer_req" => invite_1_greeter_wait_peer::Req::specs(),
                "invite_1_greeter_wait_peer_rep" => invite_1_greeter_wait_peer::Rep::specs(),
                "invite_2a_greeter_get_hashed_nonce_req" => {
                    invite_2a_greeter_get_hashed_nonce::Req::specs()
                }
                "invite_2a_greeter_get_hashed_nonce_rep" => {
                    invite_2a_greeter_get_hashed_nonce::Rep::specs()
                }
                "invite_2b_greeter_send_nonce_req" => invite_2b_greeter_send_nonce::Req::specs(),
                "invite_2b_greeter_send_nonce_rep" => invite_2b_greeter_send_nonce::Rep::specs(),
                "invite_3a_greeter_wait_peer_trust_req" => {
                    invite_3a_greeter_wait_peer_trust::Req::specs()
                }
                "invite_3a_greeter_wait_peer_trust_rep" => {
                    invite_3a_greeter_wait_peer_trust::Rep::specs()
                }
                "invite_3b_greeter_signify_trust_req" => {
                    invite_3b_greeter_signify_trust::Req::specs()
                }
                "invite_3b_greeter_signify_trust_rep" => {
                    invite_3b_greeter_signify_trust::Rep::specs()
                }
                "invite_4_greeter_communicate_req" => invite_4_greeter_communicate::Req::specs(),
                "invite_4_greeter_communicate_rep" => invite_4_greeter_communicate::Rep::specs(),
                "message_get_req" => message_get::Req::specs(),
                "message_get_rep" => message_get::Rep::specs(),
                "organization_stats_req" => organization_stats::Req::specs(),
                "organization_stats_rep" => organization_stats::Rep::specs(),
                "organization_config_req" => organization_config::Req::specs(),
                "organization_config_rep" => organization_config::Rep::specs(),
                "realm_create_req" => realm_create::Req::specs(),
                "realm_create_rep" => realm_create::Rep::specs(),
                "realm_stats_req" => realm_stats::Req::specs(),
                "realm_stats_rep" => realm_stats::Rep::specs(),
                "realm_status_req" => realm_status::Req::specs(),
                "realm_status_rep" => realm_status::Rep::specs(),
                "realm_get_role_certificates_req" => realm_get_role_certificates::Req::specs(),
                "realm_get_role_certificates_rep" => realm_get_role_certificates::Rep::specs(),
                "realm_update_roles_req" => realm_update_roles::Req::specs(),
                "realm_update_roles_rep" => realm_update_roles::Rep::specs(),
                "realm_start_reencryption_maintenance_req" => {
                    realm_start_reencryption_maintenance::Req::specs()
                }
                "realm_start_reencryption_maintenance_rep" => {
                    realm_start_reencryption_maintenance::Rep::specs()
                }
                "realm_finish_reencryption_maintenance_req" => {
                    realm_finish_reencryption_maintenance::Req::specs()
                }
                "realm_finish_reencryption_maintenance_rep" => {
                    realm_finish_reencryption_maintenance::Rep::specs()
                }
                "user_get_req" => user_get::Req::specs(),
                "user_get_rep" => user_get::Rep::specs(),
                "user_create_req" => user_create::Req::specs(),
                "user_create_rep" => user_create::Rep::specs(),
                "user_revoke_req" => user_revoke::Req::specs(),
                "user_revoke_rep" => user_revoke::Rep::specs(),
                "device_create_req" => device_create::Req::specs(),
                "device_create_rep" => device_create::Rep::specs(),
                "human_find_req" => human_find::Req::specs(),
                "human_find_rep" => human_find::Rep::specs(),
                "vlob_poll_changes_req" => vlob_poll_changes::Req::specs(),
                "vlob_poll_changes_rep" => vlob_poll_changes::Rep::specs(),
                "vlob_create_req" => vlob_create::Req::specs(),
                "vlob_create_rep" => vlob_create::Rep::specs(),
                "vlob_read_req" => vlob_read::Req::specs(),
                "vlob_read_rep" => vlob_read::Rep::specs(),
                "vlob_update_req" => vlob_update::Req::specs(),
                "vlob_update_rep" => vlob_update::Rep::specs(),
                "vlob_list_versions_req" => vlob_list_versions::Req::specs(),
                "vlob_list_versions_rep" => vlob_list_versions::Rep::specs(),
                "vlob_maintenance_get_reencryption_batch_req" => {
                    vlob_maintenance_get_reencryption_batch::Req::specs()
                }
                "vlob_maintenance_get_reencryption_batch_rep" => {
                    vlob_maintenance_get_reencryption_batch::Rep::specs()
                }
                "vlob_maintenance_save_reencryption_batch_req" => {
                    vlob_maintenance_save_reencryption_batch::Req::specs()
                }
                "vlob_maintenance_save_reencryption_batch_rep" => {
                    vlob_maintenance_save_reencryption_batch::Rep::specs()
                }
                "list" => {
                    println!("{AUTHENTICATED_CMDS}");
                    return;
                }
                name => {
                    println!("{name} doesn't exist");
                    return;
                }
            };
            println!("{}", serde_json::to_string_pretty(&specs).unwrap());
        }
        "invited" => {
            use invited_cmds::*;
            let specs = match args.name.as_str() {
                "ping_req" => ping::Req::specs(),
                "ping_rep" => ping::Rep::specs(),
                "invite_info_req" => invite_info::Req::specs(),
                "invite_info_rep" => invite_info::Rep::specs(),
                "invite_1_claimer_wait_peer_req" => invite_1_claimer_wait_peer::Req::specs(),
                "invite_1_claimer_wait_peer_rep" => invite_1_claimer_wait_peer::Rep::specs(),
                "invite_2a_claimer_send_hashed_nonce_hash_nonce_req" => {
                    invite_2a_claimer_send_hashed_nonce_hash_nonce::Req::specs()
                }
                "invite_2a_claimer_send_hashed_nonce_hash_nonce_rep" => {
                    invite_2a_claimer_send_hashed_nonce_hash_nonce::Rep::specs()
                }
                "invite_2b_claimer_send_nonce_req" => invite_2b_claimer_send_nonce::Req::specs(),
                "invite_2b_claimer_send_nonce_rep" => invite_2b_claimer_send_nonce::Rep::specs(),
                "invite_3a_claimer_signify_trust_req" => {
                    invite_3a_claimer_signify_trust::Req::specs()
                }
                "invite_3a_claimer_signify_trust_rep" => {
                    invite_3a_claimer_signify_trust::Rep::specs()
                }
                "invite_3b_claimer_wait_peer_trust_req" => {
                    invite_3b_claimer_wait_peer_trust::Req::specs()
                }
                "invite_3b_claimer_wait_peer_trust_rep" => {
                    invite_3b_claimer_wait_peer_trust::Rep::specs()
                }
                "invite_4_claimer_communicate_req" => invite_4_claimer_communicate::Req::specs(),
                "invite_4_claimer_communicate_rep" => invite_4_claimer_communicate::Rep::specs(),
                "list" => {
                    println!("{INVITED_CMDS}");
                    return;
                }
                name => {
                    println!("{name} doesn't exist");
                    return;
                }
            };
            println!("{}", serde_json::to_string_pretty(&specs).unwrap());
        }
        _ => unimplemented!(),
    }
}
//         (
//             crate::invite,
//             invite_4_claimer_communicate,
//             Invite4ClaimerCommunicate,
//             "invite_4_claimer_communicate",
//         ),

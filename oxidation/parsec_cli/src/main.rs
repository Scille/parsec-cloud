// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use clap::Parser;

// Should we want a CLI ?

// const AUTHENTICATED_CMDS: &str = r#"
// ping_req
// ping_rep
// block_create_req
// block_create_rep
// events_listen_req
// events_listen_rep
// invite_new_req
// invite_new_rep
// invite_delete_req
// invite_delete_rep
// invite_list_req
// invite_list_rep
// invite_1_greeter_wait_peer_req
// invite_1_greeter_wait_peer_rep
// invite_2a_greeter_get_hashed_nonce_req
// invite_2a_greeter_get_hashed_nonce_rep
// invite_2b_greeter_send_nonce_req
// invite_2b_greeter_send_nonce_rep
// invite_3a_greeter_wait_peer_trust_req
// invite_3a_greeter_wait_peer_trust_rep
// invite_3b_greeter_signify_trust_req
// invite_3b_greeter_signify_trust_rep
// invite_4_greeter_communicate_req
// invite_4_greeter_communicate_rep
// message_get_req
// message_get_rep
// organization_stats_req
// organization_stats_rep
// organization_config_req
// organization_config_rep
// realm_create_req
// realm_create_rep
// realm_stats_req
// realm_stats_rep
// realm_status_req
// realm_status_rep
// realm_get_role_certificates_req
// realm_get_role_certificates_rep
// realm_update_roles_req
// realm_update_roles_rep
// realm_start_reencryption_maintenance_req
// realm_start_reencryption_maintenance_rep
// realm_finish_reencryption_maintenance_req
// realm_finish_reencryption_maintenance_rep
// user_get_req
// user_get_rep
// user_create_req
// user_create_rep
// user_revoke_req
// user_revoke_rep
// device_create_req
// device_create_rep
// human_find_req
// human_find_rep
// vlob_poll_changes_req
// vlob_poll_changes_rep
// vlob_create_req
// vlob_create_rep
// vlob_read_req
// vlob_read_rep
// vlob_update_req
// vlob_update_rep
// vlob_list_versions_req
// vlob_list_versions_rep
// vlob_maintenance_get_reencryption_batch_req
// vlob_maintenance_get_reencryption_batch_rep
// vlob_maintenance_save_reencryption_batch_req
// vlob_maintenance_save_reencryption_batch_rep
// "#;

// const INVITED_CMDS: &str = r#"
// ping_req
// ping_rep
// invite_info_req
// invite_info_rep
// invite_1_claimer_wait_peer_req
// invite_1_claimer_wait_peer_rep
// invite_2a_claimer_send_hashed_nonce_hash_nonce_req
// invite_2a_claimer_send_hashed_nonce_hash_nonce_rep
// invite_2b_claimer_send_nonce_req
// invite_2b_claimer_send_nonce_rep
// invite_3a_claimer_signify_trust_req
// invite_3a_claimer_signify_trust_rep
// invite_3b_claimer_wait_peer_trust_req
// invite_3b_claimer_wait_peer_trust_rep
// invite_4_claimer_communicate_req
// invite_4_claimer_communicate_rep
// "#;

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
    // let args = Args::parse();
    // match args.cmd.as_str() {
    //     _ => unimplemented!()
    // }
    unimplemented!();
}

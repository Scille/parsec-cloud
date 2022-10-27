// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

//! Send Authenticated commands to the server.
//!
//! The HTTP-request will contain the following headers for authentication:
//!
//! ```text
//! Authorization: PARSEC-SIGN-ED25519
//! Author: [base64 string]
//! Timestamp: [DateTime UTC RFC3339 with millisecond]
//! Signature: [base64 ed25519 signature]
//! ```
//! The signature is generated by appending the following data:
//!
//! 1. `author` (in base64, contains in header `Author`)
//! 2. `timestamp` (date time UTC RFC3339 with millisecond, contains in header `Timestamp`)
//! 3. `body` (the http body in bytes)
//!
//! # Why using rfc 3339 instead of rfc 2822 ?
//!
//! > Because I can, that's why
//! > @me
//!
//! The reasons are listed [here](https://datatracker.ietf.org/doc/html/rfc3339#section-5)
//!
//! 1. Ordering
//! 2. Tradeoff between human readability and interoperability
//! 3. Redundant information (i.e.: we don't need the day of week)
//!
//! Beside the headers for authentication, we also add the header `API_VERSION` that contain the [parsec_api_protocol::ApiVersion]
//!

use std::{collections::HashMap, num::NonZeroU64};

use libparsec_crypto::{PublicKey, SigningKey};
use libparsec_protocol::{
    authenticated_cmds::v3::{
        self as authenticated_cmds, invite_delete::InvitationDeletedReason,
        invite_new::UserOrDevice,
    },
    IntegerBetween1And100,
};
use libparsec_types::{
    BackendOrganizationAddr, BlockID, DateTime, DeviceID, InvitationToken, RealmID,
    ReencryptionBatchEntry, SequesterServiceID, UserID, VlobID,
};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    Client, RequestBuilder, Url,
};

use crate::command_error::{self, CommandError};

/// Method name that will be used for the header `Authorization` to indicate that will be using this method.
pub const PARSEC_AUTH_METHOD: &str = "PARSEC-SIGN-ED25519";
/// How we serialize the data before sending a request.
pub const PARSEC_CONTENT_TYPE: &str = "application/msgpack";

pub const API_VERSION_HEADER_NAME: &str = "Api-Version";

/// Factory that send commands in a authenticated context.
pub struct AuthenticatedCmds {
    /// HTTP Client that contain the basic configuration to communicate with the server.
    client: Client,
    url: Url,
    device_id: String,
    signing_key: SigningKey,
}

impl AuthenticatedCmds {
    /// Create a new `AuthenticatedCmds`
    pub fn new(
        client: Client,
        server_url: BackendOrganizationAddr,
        device_id: DeviceID,
        signing_key: SigningKey,
    ) -> Result<Self, url::ParseError> {
        let url = server_url.to_authenticated_http_url();

        let device_id = base64::encode(device_id.to_string().as_bytes());

        Ok(Self {
            client,
            url,
            device_id,
            signing_key,
        })
    }
}

macro_rules! build_req {
    (unit $name:ident, $key:ident) => {
        authenticated_cmds::$name::Req($key)
    };

    ($name:ident, $($key:ident),*) => {
        authenticated_cmds::$name::Req {
            $($key),*
        }
    }
}

macro_rules! impl_auth_cmds {
    (
        $(
            $(#[$outer:meta])*
            $(@$decorator:ident)? $name:ident($($key:ident: $type:ty),*)
        )+
    ) => {
        $(
            $(#[$outer])*
            pub async fn $name(&self, $($key: $type),*) -> command_error::Result<authenticated_cmds::$name::Rep> {
                let request_builder = self.client.post(self.url.clone());

                let data = build_req!($($decorator)? $name, $($key),*)
                    .dump()
                    .expect(concat!("failed to serialize the command ", stringify!($name)));

                let req = prepare_request(request_builder, &self.signing_key, &self.device_id, data).send();
                let resp = dbg!(req.await)?;
                if resp.status() != reqwest::StatusCode::OK {
                    return Err(CommandError::InvalidResponseStatus(resp.status(), resp));
                }

                let response_body = resp.bytes().await?;

                authenticated_cmds::$name::Rep::load(&response_body).map_err(CommandError::InvalidResponseContent)
            }
        )+
    };
}

/// Prepare a new request, the body will be added to the Request using [RequestBuilder::body]
fn prepare_request(
    request_builder: RequestBuilder,
    signing_key: &SigningKey,
    device_id: &str,
    body: Vec<u8>,
) -> RequestBuilder {
    let request_builder = sign_request(request_builder, signing_key, device_id, &body);

    let mut content_headers = HeaderMap::with_capacity(2);
    content_headers.insert(
        API_VERSION_HEADER_NAME,
        HeaderValue::from_str(&libparsec_protocol::API_VERSION.to_string())
            .expect("api version must contains valid char"),
    );
    content_headers.insert(CONTENT_TYPE, HeaderValue::from_static(PARSEC_CONTENT_TYPE));
    content_headers.insert(
        CONTENT_LENGTH,
        HeaderValue::from_str(&body.len().to_string()).expect("numeric value are valid char"),
    );

    request_builder.headers(content_headers).body(body)
}

/// Sing a request by adding specific headers.
fn sign_request(
    request_builder: RequestBuilder,
    signing_key: &SigningKey,
    device_id: &str,
    body: &[u8],
) -> RequestBuilder {
    let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let data_to_sign = Vec::from_iter(
        device_id
            .as_bytes()
            .iter()
            .chain(timestamp.as_bytes())
            .chain(body)
            .copied(),
    );
    let signature = signing_key.sign_only_signature(&data_to_sign);
    let signature = base64::encode(signature);

    let mut authorization_headers = HeaderMap::with_capacity(4);

    authorization_headers.insert(AUTHORIZATION, HeaderValue::from_static(PARSEC_AUTH_METHOD));
    authorization_headers.insert(
        "Author",
        HeaderValue::from_str(device_id).expect("base64 shouldn't contain invalid char"),
    );
    authorization_headers.insert(
        "Timestamp",
        HeaderValue::from_str(&timestamp)
            .expect("should contain only numeric char which are valid char"),
    );
    authorization_headers.insert(
        "Signature",
        HeaderValue::from_str(&signature).expect("base64 shouldn't contain invalid char"),
    );

    request_builder.headers(authorization_headers)
}

impl AuthenticatedCmds {
    impl_auth_cmds!(
        /// Create a new block.
        block_create(block_id: BlockID, realm_id: RealmID, block: Vec<u8>)
        /// Read a block.
        block_read(block_id: BlockID)
    );

    impl_auth_cmds!(
        /// Create a new device.
        device_create(
            device_certificate: Vec<u8>,
            redacted_device_certificate: Vec<u8>
        )
    );

    // TODO: use HTTP SSE to receive continuous events
    // impl_auth_cmd!(
    //     /// Listen to events
    //     events_listen(wait: bool)
    // );
    // impl_auth_cmd!(
    //     /// Subscribe to events
    //     events_subscribe()
    // );

    impl_auth_cmds!(
        /// Search a human.
        human_find(
            query: Option<String>,
            omit_revoked: bool,
            omit_non_human: bool,
            page: NonZeroU64,
            per_page: IntegerBetween1And100
        )
    );

    impl_auth_cmds!(
        /// Wait for the peer to begin invitation procedure.
        invite_1_greeter_wait_peer(token: InvitationToken, greeter_public_key: PublicKey)
        /// Retrieve the hashed nonce during an invitation procedure.
        invite_2a_greeter_get_hashed_nonce(token: InvitationToken)
        /// Send the nonce during an invitation procedure.
        invite_2b_greeter_send_nonce(token: InvitationToken, greeter_nonce: Vec<u8>)
        /// Wait trust from peer during an invitation procedure.
        invite_3a_greeter_wait_peer_trust(token: InvitationToken)
        /// Trust establishment during an invitation procedure.
        invite_3b_greeter_signify_trust(token: InvitationToken)
        /// Last step of an invitation procedure.
        invite_4_greeter_communicate(token: InvitationToken, payload: Vec<u8>)
        /// Delete an invitation.
        invite_delete(token: InvitationToken, reason: InvitationDeletedReason)
        /// Retrieve a list of invited users.
        invite_list()
    );

    impl_auth_cmds!(
        /// Invite a new `User` or `Device`
        @unit invite_new(user_or_device: UserOrDevice)
    );

    impl_auth_cmds!(
        /// Retrieve a message at `offset`
        message_get(offset: u64)
    );

    impl_auth_cmds!(
        /// Retrieve the config of an organization.
        organization_config()
        /// Retrieve the stats of an organization.
        organization_stats()
    );

    impl_auth_cmds!(
        /// Ping the server.
        ping(ping: String)
    );

    impl_auth_cmds!(
        /// Create a new realm
        realm_create(role_certificate: Vec<u8>)
        /// Notify that we've finish re-encrypting the realm
        realm_finish_reencryption_maintenance(realm_id: RealmID, encryption_revision: u64)
        /// Retrieve the role certificates of a realm
        realm_get_role_certificates(realm_id: RealmID)
        /// Start the re-encryption maintenance on a realm and notify participant
        realm_start_reencryption_maintenance(
            realm_id: RealmID,
            encryption_revision: u64,
            timestamp: DateTime,
            per_participant_message: HashMap<UserID, Vec<u8>>
        )
        /// Retrieve the stats of a realm
        realm_stats(realm_id: RealmID)
        /// Get the status of a realm
        realm_status(realm_id: RealmID)
        /// Update role in a realm
        realm_update_roles(
            role_certificate: Vec<u8>,
            recipient_message: Option<Vec<u8>>
        )
    );

    impl_auth_cmds!(
        /// Create a new user with its device
        user_create(
            user_certificate: Vec<u8>,
            device_certificate: Vec<u8>,
            redacted_user_certificate: Vec<u8>,
            redacted_device_certificate: Vec<u8>
        )
        /// Retrieve an user by it's id
        user_get(user_id: UserID)
        /// Revoke a user certificate
        user_revoke(revoked_user_certificate: Vec<u8>)
    );

    impl_auth_cmds!(
        /// Create a new Vlob
        vlob_create(
            realm_id: RealmID,
            encryption_revision: u64,
            vlob_id: VlobID,
            timestamp: DateTime,
            blob: Vec<u8>,
            sequester_blob: Option<HashMap<SequesterServiceID, Vec<u8>>>
        )
        /// List version of a vlob
        vlob_list_versions(vlob_id: VlobID)
        /// Get a Vlob at a certain encryption revision
        vlob_maintenance_get_reencryption_batch(
            realm_id: RealmID,
            encryption_revision: u64,
            size: u64
        )
        /// Save Vlob encryption revision
        vlob_maintenance_save_reencryption_batch(
            realm_id: RealmID,
            encryption_revision: u64,
            batch: Vec<ReencryptionBatchEntry>
        )
        /// Pool changes since last checkpoint
        vlob_poll_changes(realm_id: RealmID, last_checkpoint: u64)
        /// Read a Vlob, can read a vlob at a specific version or time
        vlob_read(
            encryption_revision: u64,
            vlob_id: VlobID,
            version: Option<u32>,
            timestamp: Option<DateTime>
        )
        /// Update a Vlob
        vlob_update(
            encryption_revision: u64,
            vlob_id: VlobID,
            timestamp: DateTime,
            version: u32,
            blob: Vec<u8>,
            sequester_blob: Option<HashMap<SequesterServiceID, Vec<u8>>>
        )
    );
}

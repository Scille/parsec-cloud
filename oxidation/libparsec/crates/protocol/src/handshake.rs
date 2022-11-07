// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{cmp::Ordering, fmt::Display};

use libparsec_crypto::{SigningKey, VerifyKey};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use crate::{impl_dump_load, ChallengeDataReport, HandshakeError};
use libparsec_types::{
    maybe_field, DateTime, DeviceID, InvitationToken, InvitationType, OrganizationID,
};

pub const HANDSHAKE_CHALLENGE_SIZE: usize = 48;

pub const API_V1_VERSION: ApiVersion = ApiVersion {
    version: 1,
    revision: 3,
};
pub const API_V2_VERSION: ApiVersion = ApiVersion {
    version: 2,
    revision: 5,
};
pub const API_VERSION: ApiVersion = API_V2_VERSION;
pub const BALLPARK_CLIENT_EARLY_OFFSET: f64 = 300.0; // seconds
pub const BALLPARK_CLIENT_LATE_OFFSET: f64 = 320.0; // seconds
const BALLPARK_CLIENT_TOLERANCE: f64 = 0.8; // 80%

pub fn timestamps_in_the_ballpark(
    client: DateTime,
    backend: DateTime,
    ballpark_client_early_offset: f64,
    ballpark_client_late_offset: f64,
) -> bool {
    // Useful to compare signed message timestamp with the one stored by the
    // backend.
    let seconds = (backend - client).num_seconds() as f64;
    -ballpark_client_early_offset < seconds && seconds < ballpark_client_late_offset
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiVersion {
    pub version: u32,
    pub revision: u32,
}

impl PartialOrd for ApiVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ApiVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.version.cmp(&other.version) {
            Ordering::Equal => self.revision.cmp(&other.revision),
            order => order,
        }
    }
}

impl Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.version, self.revision)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Answer {
    #[serde(rename = "AUTHENTICATED")]
    Authenticated {
        client_api_version: ApiVersion,
        organization_id: OrganizationID,
        device_id: DeviceID,
        // RustCrypto cache aditional points on the Edward curve so total size is around ~200bytes
        // That's why VerifyKey is boxed to keep approximately the same size as InvitedAnswer
        rvk: Box<VerifyKey>,
        #[serde_as(as = "Bytes")]
        answer: Vec<u8>,
    },
    #[serde(rename = "INVITED")]
    Invited {
        client_api_version: ApiVersion,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
    },
}

impl_dump_load!(Answer);

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename = "signed_answer")]
pub struct SignedAnswer {
    #[serde_as(as = "Bytes")]
    pub answer: [u8; HANDSHAKE_CHALLENGE_SIZE],
}

impl_dump_load!(SignedAnswer);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HandshakeResult {
    BadAdminToken,
    BadIdentity,
    BadProtocol,
    Ok,
    OrganizationExpired,
    RevokedDevice,
    RvkMismatch,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "handshake", rename_all = "snake_case")]
pub enum Handshake {
    Challenge {
        #[serde_as(as = "Bytes")]
        challenge: [u8; HANDSHAKE_CHALLENGE_SIZE],
        supported_api_versions: Vec<ApiVersion>,
        // Those fields have been added to API version 2.4 (Parsec 2.7.0)
        // They are provided to the client in order to allow them to detect whether
        // their system clock is out of sync and let them close the connection.
        // They will be missing for older backend so they cannot be strictly required.
        // TODO: This backward compatibility should be removed once Parsec < 2.4 support is dropped
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        ballpark_client_early_offset: Option<f64>,
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        ballpark_client_late_offset: Option<f64>,
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        backend_timestamp: Option<DateTime>,
    },
    Answer(Answer),
    #[serde(rename = "answer")]
    SignedAnswer(SignedAnswer),
    Result {
        result: HandshakeResult,
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        help: Option<String>,
    },
}

impl_dump_load!(Handshake);

#[derive(Debug)]
pub struct ServerHandshakeStalled {
    pub supported_api_version: Vec<ApiVersion>,
}

impl Default for ServerHandshakeStalled {
    fn default() -> Self {
        Self {
            supported_api_version: vec![API_V2_VERSION, API_V1_VERSION],
        }
    }
}

impl ServerHandshakeStalled {
    fn _build_challenge_req_with_challenge(
        self,
        challenge: [u8; HANDSHAKE_CHALLENGE_SIZE],
        timestamp: DateTime,
    ) -> Result<ServerHandshakeChallenge, HandshakeError> {
        let raw = Handshake::Challenge {
            challenge,
            supported_api_versions: self.supported_api_version.clone(),
            ballpark_client_early_offset: Some(BALLPARK_CLIENT_EARLY_OFFSET),
            ballpark_client_late_offset: Some(BALLPARK_CLIENT_LATE_OFFSET),
            backend_timestamp: Some(timestamp),
        }
        .dump()
        .map_err(HandshakeError::InvalidMessage)?;

        Ok(ServerHandshakeChallenge {
            supported_api_version: self.supported_api_version,
            challenge,
            raw,
        })
    }

    pub fn build_challenge_req(
        self,
        timestamp: DateTime,
    ) -> Result<ServerHandshakeChallenge, HandshakeError> {
        let mut challenge = [0; HANDSHAKE_CHALLENGE_SIZE];
        thread_rng().fill(&mut challenge[..]);
        self._build_challenge_req_with_challenge(challenge, timestamp)
    }

    #[cfg(feature = "test")]
    pub fn build_challenge_req_with_challenge(
        self,
        challenge: [u8; HANDSHAKE_CHALLENGE_SIZE],
        timestamp: DateTime,
    ) -> Result<ServerHandshakeChallenge, HandshakeError> {
        self._build_challenge_req_with_challenge(challenge, timestamp)
    }
}

#[derive(Debug)]
pub struct ServerHandshakeChallenge {
    pub supported_api_version: Vec<ApiVersion>,
    pub challenge: [u8; HANDSHAKE_CHALLENGE_SIZE],
    pub raw: Vec<u8>,
}

impl ServerHandshakeChallenge {
    pub fn process_answer_req(self, req: &[u8]) -> Result<ServerHandshakeAnswer, HandshakeError> {
        if let Handshake::Answer(data) =
            Handshake::load(req).map_err(HandshakeError::InvalidMessage)?
        {
            let client_api_version = match data {
                Answer::Authenticated {
                    client_api_version, ..
                } => client_api_version,
                Answer::Invited {
                    client_api_version, ..
                } => client_api_version,
            };
            if client_api_version.version == 1 || client_api_version.version == 2 {
                return Ok(ServerHandshakeAnswer {
                    client_api_version,
                    challenge: self.challenge,
                    data,
                });
            } else {
                return Err(HandshakeError::APIVersion {
                    backend_versions: self.supported_api_version,
                    client_versions: vec![client_api_version],
                });
            }
        }
        Err(HandshakeError::InvalidMessage("Invalid data"))
    }

    pub fn build_bad_protocol_result_req(
        self,
        help: Option<String>,
    ) -> Result<ServerHandshakeResult, HandshakeError> {
        let help = match help {
            None => Some("Invalid params".into()),
            _ => help,
        };

        Ok(ServerHandshakeResult {
            client_api_version: ApiVersion::default(),
            raw: Handshake::Result {
                result: HandshakeResult::BadProtocol,
                help,
            }
            .dump()
            .map_err(HandshakeError::InvalidMessage)?,
        })
    }
}

#[derive(Debug)]
pub struct ServerHandshakeAnswer {
    client_api_version: ApiVersion,
    challenge: [u8; HANDSHAKE_CHALLENGE_SIZE],
    pub data: Answer,
}

impl ServerHandshakeAnswer {
    const VERSION: ApiVersion = ApiVersion {
        version: 2,
        revision: 5,
    };

    pub fn build_result_req(
        self,
        verify_key: Option<VerifyKey>,
    ) -> Result<ServerHandshakeResult, HandshakeError> {
        if let Answer::Authenticated {
            client_api_version,
            answer,
            ..
        } = self.data
        {
            match verify_key {
                Some(verify_key) => {
                    let returned_challenge = if client_api_version >= Self::VERSION {
                        let answer = verify_key
                            .verify(&answer)
                            .map_err(|_| HandshakeError::FailedChallenge)?;
                        SignedAnswer::load(&answer)
                            .map_err(HandshakeError::InvalidMessage)?
                            .answer
                    } else {
                        verify_key
                            .verify(&answer)
                            .map_err(|_| HandshakeError::FailedChallenge)?
                            .try_into()
                            .unwrap()
                    };

                    if returned_challenge != self.challenge {
                        return Err(HandshakeError::FailedChallenge);
                    }
                }
                _ => {
                    return Err(HandshakeError::InvalidMessage(
                        "`verify_key` param must be provided for authenticated handshake",
                    ))
                }
            }
        }

        Ok(ServerHandshakeResult {
            client_api_version: self.client_api_version,
            raw: Handshake::Result {
                result: HandshakeResult::Ok,
                help: None,
            }
            .dump()
            .map_err(HandshakeError::InvalidMessage)?,
        })
    }

    pub fn build_bad_protocol_result_req(
        self,
        help: Option<String>,
    ) -> Result<ServerHandshakeResult, HandshakeError> {
        let help = match help {
            None => Some("Invalid params".into()),
            _ => help,
        };

        Ok(ServerHandshakeResult {
            client_api_version: ApiVersion::default(),
            raw: Handshake::Result {
                result: HandshakeResult::BadProtocol,
                help,
            }
            .dump()
            .map_err(HandshakeError::InvalidMessage)?,
        })
    }

    pub fn build_bad_administration_token_result_req(
        self,
        help: Option<String>,
    ) -> Result<ServerHandshakeResult, HandshakeError> {
        let help = match help {
            None => Some("Invalid administration token".into()),
            _ => help,
        };

        Ok(ServerHandshakeResult {
            client_api_version: ApiVersion::default(),
            raw: Handshake::Result {
                result: HandshakeResult::BadAdminToken,
                help,
            }
            .dump()
            .map_err(HandshakeError::InvalidMessage)?,
        })
    }

    pub fn build_bad_identity_result_req(
        self,
        help: Option<String>,
    ) -> Result<ServerHandshakeResult, HandshakeError> {
        let help = match help {
            None => Some("Invalid handshake information".into()),
            _ => help,
        };

        Ok(ServerHandshakeResult {
            client_api_version: ApiVersion::default(),
            raw: Handshake::Result {
                result: HandshakeResult::BadIdentity,
                help,
            }
            .dump()
            .map_err(HandshakeError::InvalidMessage)?,
        })
    }

    pub fn build_organization_expired_result_req(
        self,
        help: Option<String>,
    ) -> Result<ServerHandshakeResult, HandshakeError> {
        let help = match help {
            None => Some("Trial organization has expired".into()),
            _ => help,
        };

        Ok(ServerHandshakeResult {
            client_api_version: ApiVersion::default(),
            raw: Handshake::Result {
                result: HandshakeResult::OrganizationExpired,
                help,
            }
            .dump()
            .map_err(HandshakeError::InvalidMessage)?,
        })
    }

    pub fn build_rvk_mismatch_result_req(
        self,
        help: Option<String>,
    ) -> Result<ServerHandshakeResult, HandshakeError> {
        let help = match help {
            None => {
                Some("Root verify key for organization differs between client and server".into())
            }
            _ => help,
        };

        Ok(ServerHandshakeResult {
            client_api_version: ApiVersion::default(),
            raw: Handshake::Result {
                result: HandshakeResult::RvkMismatch,
                help,
            }
            .dump()
            .map_err(HandshakeError::InvalidMessage)?,
        })
    }

    pub fn build_revoked_device_result_req(
        self,
        help: Option<String>,
    ) -> Result<ServerHandshakeResult, HandshakeError> {
        let help = match help {
            None => Some("Device has been revoked".into()),
            _ => help,
        };

        Ok(ServerHandshakeResult {
            client_api_version: ApiVersion::default(),
            raw: Handshake::Result {
                result: HandshakeResult::RevokedDevice,
                help,
            }
            .dump()
            .map_err(HandshakeError::InvalidMessage)?,
        })
    }
}

#[derive(Debug)]
pub struct ServerHandshakeResult {
    pub client_api_version: ApiVersion,
    pub raw: Vec<u8>,
}

#[derive(Debug)]
pub struct AuthenticatedClientHandshakeStalled {
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub user_signkey: SigningKey,
    pub root_verify_key: VerifyKey,
    pub supported_api_versions: Vec<ApiVersion>,
    pub client_timestamp: DateTime,
}

fn _settle_compatible_versions(
    backend_versions: &[ApiVersion],
    client_versions: &[ApiVersion],
) -> Result<(ApiVersion, ApiVersion), HandshakeError> {
    // Try to use the newest version first
    for cv in client_versions.iter().rev() {
        // No need to compare `revision` because only `version` field breaks compatibility
        if let Some(bv) = backend_versions.iter().find(|bv| bv.version == cv.version) {
            return Ok((*bv, *cv));
        }
    }
    Err(HandshakeError::APIVersion {
        backend_versions: backend_versions.to_vec(),
        client_versions: client_versions.to_vec(),
    })
}

fn load_challenge_req(
    req: &[u8],
    _supported_api_versions: &[ApiVersion],
    client_timestamp: DateTime,
) -> Result<
    (
        [u8; HANDSHAKE_CHALLENGE_SIZE],
        ApiVersion,
        ApiVersion,
        Vec<ApiVersion>,
    ),
    HandshakeError,
> {
    let challenge_data = Handshake::load(req).map_err(HandshakeError::InvalidMessage)?;

    if let Handshake::Challenge {
        challenge,
        supported_api_versions,
        backend_timestamp,
        ballpark_client_early_offset,
        ballpark_client_late_offset,
    } = challenge_data
    {
        // API version matching
        let (backend_api_version, client_api_version) =
            _settle_compatible_versions(&supported_api_versions, _supported_api_versions)?;

        // Those fields are missing with parsec API 2.3 and lower
        if let (
            Some(backend_timestamp),
            Some(ballpark_client_early_offset),
            Some(ballpark_client_late_offset),
        ) = (
            backend_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        ) {
            // Check whether our system clock is in sync with the backend
            if !timestamps_in_the_ballpark(
                client_timestamp,
                backend_timestamp,
                // The client is a bit less tolerant than the backend
                ballpark_client_early_offset * BALLPARK_CLIENT_TOLERANCE,
                ballpark_client_late_offset * BALLPARK_CLIENT_TOLERANCE,
            ) {
                // Add `client_timestamp` to challenge data
                // so the dictionnary exposes the same fields as `TimestampOutOfBallparkRepSchema`
                return Err(HandshakeError::OutOfBallpark(ChallengeDataReport {
                    challenge,
                    supported_api_versions,
                    backend_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                }));
            }
        }
        return Ok((
            challenge,
            backend_api_version,
            client_api_version,
            supported_api_versions,
        ));
    }
    Err(HandshakeError::InvalidMessage("Invalid data"))
}

fn process_result_req(req: &[u8]) -> Result<(), HandshakeError> {
    if let Handshake::Result { result, .. } =
        Handshake::load(req).map_err(HandshakeError::InvalidMessage)?
    {
        match result {
            HandshakeResult::BadIdentity => Err(HandshakeError::BadIdentity),
            HandshakeResult::OrganizationExpired => Err(HandshakeError::OrganizationExpired),
            HandshakeResult::RvkMismatch => Err(HandshakeError::RVKMismatch),
            HandshakeResult::RevokedDevice => Err(HandshakeError::RevokedDevice),
            HandshakeResult::BadAdminToken => Err(HandshakeError::BadAdministrationToken),
            HandshakeResult::BadProtocol => Err(HandshakeError::InvalidMessage(
                "Bad protocol replied by peer",
            )),
            HandshakeResult::Ok => Ok(()),
        }
    } else {
        Err(HandshakeError::InvalidMessage("Invalid data"))
    }
}

impl AuthenticatedClientHandshakeStalled {
    const VERSION: ApiVersion = ApiVersion {
        version: 2,
        revision: 5,
    };

    pub fn new(
        organization_id: OrganizationID,
        device_id: DeviceID,
        user_signkey: SigningKey,
        root_verify_key: VerifyKey,
        client_timestamp: DateTime,
    ) -> Self {
        let mut supported_api_versions = vec![API_V2_VERSION, API_V1_VERSION];
        supported_api_versions.sort();

        Self {
            organization_id,
            device_id,
            user_signkey,
            root_verify_key,
            supported_api_versions,
            client_timestamp,
        }
    }

    pub fn process_challenge_req(
        self,
        req: &[u8],
    ) -> Result<AuthenticatedClientHandshakeChallenge, HandshakeError> {
        let (challenge, backend_api_version, client_api_version, supported_api_versions) =
            load_challenge_req(req, &self.supported_api_versions, self.client_timestamp)?;

        // TO-DO remove the else for the next release
        let answer = if backend_api_version >= Self::VERSION {
            // TO-DO Need to use "BaseSignedData" ?
            self.user_signkey.sign(
                &SignedAnswer { answer: challenge }
                    .dump()
                    .map_err(HandshakeError::InvalidMessage)?,
            )
        } else {
            self.user_signkey.sign(&challenge)
        };

        Ok(AuthenticatedClientHandshakeChallenge {
            backend_api_version,
            client_api_version,
            supported_api_versions,
            raw: Handshake::Answer(Answer::Authenticated {
                client_api_version,
                organization_id: self.organization_id,
                device_id: self.device_id,
                rvk: Box::new(self.root_verify_key),
                answer,
            })
            .dump()
            .map_err(HandshakeError::InvalidMessage)?,
        })
    }

    pub fn process_result_req(self, req: &[u8]) -> Result<(), HandshakeError> {
        process_result_req(req)
    }
}

#[derive(Debug)]
pub struct AuthenticatedClientHandshakeChallenge {
    pub backend_api_version: ApiVersion,
    pub client_api_version: ApiVersion,
    pub supported_api_versions: Vec<ApiVersion>,
    pub raw: Vec<u8>,
}

#[derive(Debug)]
pub struct InvitedClientHandshakeStalled {
    pub organization_id: OrganizationID,
    pub invitation_type: InvitationType,
    pub token: InvitationToken,
    pub supported_api_versions: Vec<ApiVersion>,
    pub client_timestamp: DateTime,
}

impl InvitedClientHandshakeStalled {
    pub fn new(
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
        client_timestamp: DateTime,
    ) -> Self {
        let mut supported_api_versions = vec![API_V2_VERSION, API_V1_VERSION];
        supported_api_versions.sort();

        Self {
            organization_id,
            invitation_type,
            token,
            supported_api_versions,
            client_timestamp,
        }
    }

    pub fn process_challenge_req(
        self,
        req: &[u8],
    ) -> Result<InvitedClientHandshakeChallenge, HandshakeError> {
        let (_, backend_api_version, client_api_version, _) =
            load_challenge_req(req, &self.supported_api_versions, self.client_timestamp)?;

        Ok(InvitedClientHandshakeChallenge {
            backend_api_version,
            client_api_version,
            raw: Handshake::Answer(Answer::Invited {
                client_api_version,
                organization_id: self.organization_id,
                invitation_type: self.invitation_type,
                token: self.token,
            })
            .dump()
            .map_err(HandshakeError::InvalidMessage)?,
        })
    }

    pub fn process_result_req(self, req: &[u8]) -> Result<(), HandshakeError> {
        process_result_req(req)
    }
}

#[derive(Debug)]
pub struct InvitedClientHandshakeChallenge {
    pub backend_api_version: ApiVersion,
    pub client_api_version: ApiVersion,
    pub raw: Vec<u8>,
}

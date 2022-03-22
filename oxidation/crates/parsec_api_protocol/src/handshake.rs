// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::cmp::Ordering;

use parsec_api_crypto::{SigningKey, VerifyKey};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use crate::{impl_dumps_loads, ChallengeDataReport, HandshakeError, InvitationType};
use parsec_api_types::{maybe_field, DateTime, DeviceID, InvitationToken, OrganizationID};

pub const API_V1_VERSION: ApiVersion = ApiVersion {
    version: 1,
    revision: 3,
};
pub const API_V2_VERSION: ApiVersion = ApiVersion {
    version: 2,
    revision: 5,
};
pub const API_VERSION: ApiVersion = API_V2_VERSION;
pub const BALLPARK_CLIENT_EARLY_OFFSET: f64 = 50.0; // seconds
pub const BALLPARK_CLIENT_LATE_OFFSET: f64 = 70.0; // seconds
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

fn load_challenge_req(
    req: &[u8],
    _supported_api_versions: &[ApiVersion],
    client_timestamp: DateTime,
) -> Result<(Vec<u8>, ApiVersion, ApiVersion, Vec<ApiVersion>), HandshakeError> {
    let challenge_data = Handshake::loads(req)?;

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
    Err(HandshakeError::InvalidMessage("Invalid data".into()))
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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Answer {
    #[serde(rename = "AUTHENTICATED")]
    Authenticated {
        client_api_version: ApiVersion,
        organization_id: OrganizationID,
        device_id: DeviceID,
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
    #[serde(rename = "signed_answer")]
    SignedAnswer {
        #[serde_as(as = "Bytes")]
        answer: Vec<u8>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

impl_dumps_loads!(Answer);

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "handshake", rename_all = "snake_case")]
pub enum Handshake {
    Challenge {
        #[serde_as(as = "Bytes")]
        challenge: Vec<u8>,
        supported_api_versions: Vec<ApiVersion>,
        // Those fields have been added to API version 2.4
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
    Answer(Box<Answer>),
    Result {
        result: HandshakeResult,
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        help: Option<String>,
    },
}

impl_dumps_loads!(Handshake);

#[derive(Debug)]
pub struct ServerHandshakeStalled {
    pub challenge_size: usize,
    pub supported_api_version: Vec<ApiVersion>,
}

impl Default for ServerHandshakeStalled {
    fn default() -> Self {
        Self {
            challenge_size: 48,
            supported_api_version: vec![API_V2_VERSION, API_V1_VERSION],
        }
    }
}

impl ServerHandshakeStalled {
    pub fn new(challenge_size: usize) -> Self {
        Self {
            challenge_size,
            supported_api_version: vec![API_V2_VERSION, API_V1_VERSION],
        }
    }
    pub fn build_challenge_req(self) -> Result<ServerHandshakeChallenge, HandshakeError> {
        let mut challenge = vec![0; self.challenge_size];
        thread_rng().fill(&mut challenge[..]);

        let raw = Handshake::Challenge {
            challenge: challenge.clone(),
            supported_api_versions: self.supported_api_version.clone(),
            ballpark_client_early_offset: Some(BALLPARK_CLIENT_EARLY_OFFSET),
            ballpark_client_late_offset: Some(BALLPARK_CLIENT_LATE_OFFSET),
            backend_timestamp: Some(DateTime::now()),
        }
        .dumps()?;

        Ok(ServerHandshakeChallenge {
            supported_api_version: self.supported_api_version,
            challenge,
            raw,
        })
    }
    #[cfg(feature = "test")]
    pub fn build_challenge_req_with_challenge(
        self,
        challenge: Vec<u8>,
    ) -> Result<ServerHandshakeChallenge, HandshakeError> {
        let raw = Handshake::Challenge {
            challenge: challenge.clone(),
            supported_api_versions: self.supported_api_version.clone(),
            ballpark_client_early_offset: Some(BALLPARK_CLIENT_EARLY_OFFSET),
            ballpark_client_late_offset: Some(BALLPARK_CLIENT_LATE_OFFSET),
            backend_timestamp: Some(DateTime::now()),
        }
        .dumps()?;

        Ok(ServerHandshakeChallenge {
            supported_api_version: self.supported_api_version,
            challenge,
            raw,
        })
    }
}

#[derive(Debug)]
pub struct ServerHandshakeChallenge {
    pub supported_api_version: Vec<ApiVersion>,
    pub challenge: Vec<u8>,
    pub raw: Vec<u8>,
}

impl ServerHandshakeChallenge {
    pub fn process_answer_req(self, req: &[u8]) -> Result<ServerHandshakeAnswer, HandshakeError> {
        if let Handshake::Answer(data) = Handshake::loads(req)? {
            match *data {
                Answer::Authenticated {
                    client_api_version, ..
                }
                | Answer::Invited {
                    client_api_version, ..
                } => {
                    if client_api_version.version == 1 || client_api_version.version == 2 {
                        return Ok(ServerHandshakeAnswer {
                            client_api_version,
                            challenge: self.challenge,
                            data: *data,
                        });
                    } else {
                        return Err(HandshakeError::APIVersion {
                            backend_versions: self.supported_api_version,
                            client_versions: vec![client_api_version],
                        });
                    }
                }
                _ => return Err(HandshakeError::InvalidMessage("Invalid data".into())),
            }
        }
        Err(HandshakeError::InvalidMessage("Invalid data".into()))
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
            .dumps()?,
        })
    }
}

#[derive(Debug)]
pub struct ServerHandshakeAnswer {
    client_api_version: ApiVersion,
    challenge: Vec<u8>,
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
                        let answer = verify_key.verify(&answer).map_err(|_| {
                            HandshakeError::FailedChallenge("Invalid answer signature".into())
                        })?;
                        match Answer::loads(&answer)? {
                            Answer::SignedAnswer { answer } => answer,
                            _ => return Err(HandshakeError::InvalidMessage("Invalid data".into())),
                        }
                    } else {
                        verify_key.verify(&answer).map_err(|_| {
                            HandshakeError::FailedChallenge("Invalid answer signature".into())
                        })?
                    };

                    if returned_challenge != self.challenge {
                        return Err(HandshakeError::FailedChallenge(
                            "Invalid returned challenge".into(),
                        ));
                    }
                }
                _ => return Err(HandshakeError::Authenticated),
            }
        }

        Ok(ServerHandshakeResult {
            client_api_version: self.client_api_version,
            raw: Handshake::Result {
                result: HandshakeResult::Ok,
                help: None,
            }
            .dumps()?,
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
            .dumps()?,
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
            .dumps()?,
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
            .dumps()?,
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
            .dumps()?,
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
            .dumps()?,
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
            .dumps()?,
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
    ) -> Self {
        let mut supported_api_versions = vec![API_V2_VERSION, API_V1_VERSION];
        supported_api_versions.sort();

        Self {
            organization_id,
            device_id,
            user_signkey,
            root_verify_key,
            supported_api_versions,
            client_timestamp: DateTime::now(),
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
            self.user_signkey
                .sign(&Answer::SignedAnswer { answer: challenge }.dumps()?)
        } else {
            self.user_signkey.sign(&challenge)
        };

        return Ok(AuthenticatedClientHandshakeChallenge {
            backend_api_version,
            client_api_version,
            supported_api_versions,
            raw: Handshake::Answer(Box::new(Answer::Authenticated {
                client_api_version,
                organization_id: self.organization_id,
                device_id: self.device_id,
                rvk: Box::new(self.root_verify_key),
                answer,
            }))
            .dumps()?,
        });
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
    ) -> Self {
        let mut supported_api_versions = vec![API_V2_VERSION, API_V1_VERSION];
        supported_api_versions.sort();

        Self {
            organization_id,
            invitation_type,
            token,
            supported_api_versions,
            client_timestamp: DateTime::now(),
        }
    }
    pub fn process_challenge_req(
        self,
        req: &[u8],
    ) -> Result<InvitedClientHandshakeChallenge, HandshakeError> {
        let (_, backend_api_version, client_api_version, _) =
            load_challenge_req(req, &self.supported_api_versions, self.client_timestamp)?;

        return Ok(InvitedClientHandshakeChallenge {
            backend_api_version,
            client_api_version,
            raw: Handshake::Answer(Box::new(Answer::Invited {
                client_api_version,
                organization_id: self.organization_id,
                invitation_type: self.invitation_type,
                token: self.token,
            }))
            .dumps()?,
        });
    }
}

#[derive(Debug)]
pub struct InvitedClientHandshakeChallenge {
    pub backend_api_version: ApiVersion,
    pub client_api_version: ApiVersion,
    pub raw: Vec<u8>,
}

pub trait BaseClientHandshake: Sized {
    fn process_result_req(self, req: &[u8]) -> Result<(), HandshakeError> {
        if let Handshake::Result { result, help } = Handshake::loads(req)? {
            match result {
                HandshakeResult::BadIdentity => Err(HandshakeError::BadIdentity(help)),
                HandshakeResult::OrganizationExpired => {
                    Err(HandshakeError::OrganizationExpired(help))
                }
                HandshakeResult::RvkMismatch => Err(HandshakeError::RVKMismatch(help)),
                HandshakeResult::RevokedDevice => Err(HandshakeError::RevokedDevice(help)),
                HandshakeResult::BadAdminToken => Err(HandshakeError::BadAdministrationToken(help)),
                HandshakeResult::Ok => Ok(()),
                _ => Err(HandshakeError::InvalidMessage(format!(
                    "Bad `result` handshake: {result:?} ({help:?})"
                ))),
            }
        } else {
            Err(HandshakeError::InvalidMessage("Invalid data".into()))
        }
    }
}

impl BaseClientHandshake for AuthenticatedClientHandshakeStalled {}
impl BaseClientHandshake for InvitedClientHandshakeStalled {}

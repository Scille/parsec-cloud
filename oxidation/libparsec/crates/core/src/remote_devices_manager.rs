// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_crypto::VerifyKey;
use libparsec_protocol::authenticated_cmds::v2::user_get;
use libparsec_types::{
    DeviceCertificate, DeviceID, RevokedUserCertificate, TimeProvider, UserCertificate, UserID,
};

use crate::{RemoteDevicesManagerError, RemoteDevicesManagerResult, TrustchainContext};

const DEFAULT_CACHE_VALIDITY: i64 = 60 * 60; // 3600 seconds, 1 hour;

pub struct RemoteDevicesManager {
    backend_cmds: AuthenticatedCmds,
    trustchain_ctx: TrustchainContext,
}

impl RemoteDevicesManager {
    pub fn new(
        backend_cmds: AuthenticatedCmds,
        root_verify_key: VerifyKey,
        time_provider: TimeProvider,
    ) -> Self {
        Self {
            backend_cmds,
            trustchain_ctx: TrustchainContext::new(
                root_verify_key,
                time_provider,
                DEFAULT_CACHE_VALIDITY,
            ),
        }
    }

    pub fn cache_validity(&self) -> i64 {
        self.trustchain_ctx.cache_validity()
    }

    pub fn invalidate_user_cache(&mut self, user_id: &UserID) {
        self.trustchain_ctx.invalidate_user_cache(user_id)
    }

    pub async fn get_user(
        &mut self,
        user_id: &UserID,
        no_cache: bool,
    ) -> RemoteDevicesManagerResult<(UserCertificate, Option<RevokedUserCertificate>)> {
        let verified_user = if no_cache {
            None
        } else {
            self.trustchain_ctx.get_user(user_id)
        };
        let verified_revoked_user = if no_cache {
            None
        } else {
            self.trustchain_ctx.get_revoked_user(user_id)
        };

        if let Some(verified_user) = verified_user {
            Ok((verified_user.clone(), verified_revoked_user.cloned()))
        } else {
            let (verified_user, verified_revoked_user, _) =
                self.get_user_and_devices(user_id).await?;
            Ok((verified_user, verified_revoked_user))
        }
    }

    pub async fn get_device(
        &mut self,
        device_id: &DeviceID,
        no_cache: bool,
    ) -> RemoteDevicesManagerResult<DeviceCertificate> {
        let verified_device = if no_cache {
            None
        } else {
            self.trustchain_ctx.get_device(device_id)
        };

        if let Some(verified_device) = verified_device {
            Ok(verified_device.clone())
        } else {
            let (_, _, verified_devices) = self.get_user_and_devices(device_id.user_id()).await?;
            verified_devices
                .into_iter()
                .find(|vd| &vd.device_id == device_id)
                .ok_or(RemoteDevicesManagerError::DeviceNotFound {
                    user_id: device_id.user_id().clone(),
                    device_id: device_id.clone(),
                })
        }
    }

    pub async fn get_user_and_devices(
        &mut self,
        user_id: &UserID,
    ) -> RemoteDevicesManagerResult<(
        UserCertificate,
        Option<RevokedUserCertificate>,
        Vec<DeviceCertificate>,
    )> {
        match self
            .backend_cmds
            .send(user_get::Req {
                user_id: user_id.clone(),
            })
            .await
        {
            Ok(user_get::Rep::Ok {
                device_certificates,
                revoked_user_certificate,
                trustchain,
                user_certificate,
            }) => self
                .trustchain_ctx
                .load_user_and_devices(
                    user_get::Trustchain {
                        devices: trustchain.devices,
                        revoked_users: trustchain.revoked_users,
                        users: trustchain.users,
                    },
                    user_certificate,
                    revoked_user_certificate,
                    device_certificates,
                    Some(user_id),
                )
                .map_err(|exc| RemoteDevicesManagerError::InvalidTrustchain { exc }),
            Ok(user_get::Rep::NotFound) => Err(RemoteDevicesManagerError::UserNotFound {
                user_id: user_id.clone(),
            }),
            Ok(user_get::Rep::UnknownStatus { reason, .. }) => {
                Err(RemoteDevicesManagerError::FailedFetchUser {
                    user_id: user_id.clone(),
                    reason: reason.unwrap_or_default(),
                })
            }
            Err(_) => todo!(),
        }
    }
}

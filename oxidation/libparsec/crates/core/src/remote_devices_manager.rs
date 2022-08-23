// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_crypto::VerifyKey;
use libparsec_types::{TimeProvider, UserID};

use crate::TrustchainContext;

const DEFAULT_CACHE_VALIDITY: i64 = 60 * 60; // 3600 seconds, 1 hour;

pub struct RemoteDevicesManager {
    trustchain_ctx: TrustchainContext,
}

impl RemoteDevicesManager {
    pub fn new(root_verify_key: VerifyKey, time_provider: TimeProvider) -> Self {
        Self {
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

    pub async fn get_user(&self) {
        unimplemented!()
    }

    pub async fn get_device(&self) {
        unimplemented!()
    }

    pub async fn get_user_and_devices(&self) {
        unimplemented!()
    }
}

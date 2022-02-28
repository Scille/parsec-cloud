// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod block;
mod events;
mod invite;
mod message;
mod organization;
mod ping;
mod realm;
mod user;
mod vlob;

pub use block::*;
pub use events::*;
pub use invite::*;
pub use message::*;
pub use organization::*;
pub use ping::*;
pub use realm::*;
pub use user::*;
pub use vlob::*;

use serde::{Deserialize, Serialize};

macro_rules! impl_api_protocol_dump_load {
    ($name:ident) => {
        impl $name {
            pub fn dump(&self) -> Vec<u8> {
                rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!())
            }

            pub fn load(data: &[u8]) -> Result<Self, &'static str> {
                rmp_serde::from_read_ref::<_, Self>(&data).map_err(|_| "Invalid data")
            }
        }
    };
}

pub(crate) use impl_api_protocol_dump_load;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
}

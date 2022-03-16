// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod block;
mod cmds;
mod events;
mod invite;
mod message;
mod organization;
mod ping;
mod realm;
mod user;
mod vlob;

use block::*;
pub use cmds::*;
use events::*;
use invite::*;
use message::*;
use organization::*;
use ping::*;
use realm::*;
use user::*;
use vlob::*;

pub use events::APIEvent;
pub use invite::{
    InvitationDeletedReason, InvitationEmailSentStatus, InvitationStatus, InvitationType,
    InviteInfoUserOrDeviceRep, InviteListItem,
};
pub use message::Message;
pub use organization::{OrganizationBootstrapWebhook, UsersPerProfileDetailItem};
pub use realm::MaintenanceType;
pub use user::{HumanFindResultItem, Trustchain};
pub use vlob::ReencryptionBatchEntry;

macro_rules! impl_dumps_loads {
    ($name:ident) => {
        impl $name {
            pub fn dumps(&self) -> Result<Vec<u8>, &'static str> {
                ::rmp_serde::to_vec_named(self).map_err(|_| "Serialization failed")
            }

            pub fn loads(buf: &[u8]) -> Result<Self, &'static str> {
                ::rmp_serde::from_read_ref::<_, Self>(buf).map_err(|_| "Deserialization failed")
            }
        }
    };
}

pub(crate) use impl_dumps_loads;

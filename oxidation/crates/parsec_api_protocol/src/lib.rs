// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod block;
mod cmds;
mod error;
mod events;
mod handshake;
mod invite;
mod message;
mod organization;
mod ping;
mod realm;
mod user;
mod vlob;

pub use cmds::*;
pub use error::*;
pub use events::APIEvent;
pub use handshake::*;
pub use invite::{
    InvitationDeletedReason, InvitationEmailSentStatus, InvitationStatus, InviteInfoUserOrDevice,
    InviteListItem,
};
pub use message::Message;
pub use organization::{OrganizationBootstrapWebhook, UsersPerProfileDetailItem};
pub use realm::MaintenanceType;
pub use user::{HumanFindResultItem, Trustchain};
pub use vlob::ReencryptionBatchEntry;

macro_rules! impl_dump_load {
    ($name:ident) => {
        impl $name {
            pub fn dump(&self) -> Result<Vec<u8>, &'static str> {
                ::rmp_serde::to_vec_named(self).map_err(|_| "Serialization failed")
            }

            pub fn load(buf: &[u8]) -> Result<Self, &'static str> {
                ::rmp_serde::from_read_ref(buf).map_err(|_| "Deserialization failed")
            }
        }
    };
}
pub(crate) use impl_dump_load;

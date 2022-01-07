// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use super::utils::new_uuid_type;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InvitationType {
    User,
    Device,
}

new_uuid_type!(pub InvitationToken);

// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use super::utils::new_uuid_type;

/*
 * EntryID & BlockID
 */

new_uuid_type!(pub EntryID);
new_uuid_type!(pub BlockID);

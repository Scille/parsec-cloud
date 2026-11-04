// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use libparsec_types::ParsecPkiEnrollmentAddr;

use crate::Client;

pub async fn get_addr(client: &Client) -> ParsecPkiEnrollmentAddr {
    ParsecPkiEnrollmentAddr::new(client.organization_addr(), client.organization_id().clone())
}

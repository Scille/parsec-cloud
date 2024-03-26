// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::VersionInt;

pub mod certificates;
mod db;
mod model;
pub mod user;
pub mod workspace;

const DB_VERSION: VersionInt = 1;

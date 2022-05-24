-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS chunks;
DROP TABLE IF EXISTS vlobs;
DROP TABLE IF EXISTS realm_checkpoint;
DROP TABLE IF EXISTS prevent_sync_pattern;

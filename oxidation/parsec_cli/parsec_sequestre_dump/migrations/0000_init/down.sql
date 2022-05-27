-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS block;
DROP TABLE IF EXISTS vlob_atom;
DROP TABLE IF EXISTS realm_role;
DROP TABLE IF EXISTS user_;
DROP TABLE IF EXISTS device;
DROP TABLE IF EXISTS info;

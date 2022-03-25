-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS


INSERT INTO pki_certificate(_id, certificate_id, request_id, request_object, request_timestamp) VALUES (
    307,
    '6ecd8c99-4036-403d-bf84-cf8400f67836',
    '3f333df6-90a4-4fda-8dd3-9485d27cee36'
    E'\\x1234567890abcdef',
    '2021-9-29 11:58:08.841265+02'
);


INSERT INTO pki_certificate(_id, certificate_id, request_id, request_object, request_timestamp, reply_timestamp, reply_object) VALUES (
    309,
    '6ecd8c99-4036-403d-bf84-cf8402f67867',
    '3f333df6-90a4-4fda-8dd3-9483d27cee36'
    E'\\x1234567890abcdef',
    '2021-10-28 11:58:08.841265+02'
    '2021-6-28 11:58:08.841265+02'
    E'\\x1234567890abc',
);
INSERT INTO pki_certificate(_id, certificate_id, request_id, request_object, request_timestamp, reply_timestamp, reply_object, reply_user_id) VALUES (
    311,
    '6ecd8c99-4036-403d-bf84-cf8402f67885',
    '3f333df6-90a4-4fda-8dd3-9483d27cee36'
    E'\\x1234567890abcdef',
    '2021-10-28 11:58:08.841265+02'
    '2021-6-28 11:58:08.841265+02'
    E'\\x1234567890abc',
    '3f333df1-90a4-4fda-8dd3-9483d27cee36'
);

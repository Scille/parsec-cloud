// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

table! {
    chunks (chunk_id) {
        chunk_id -> Binary,
        size -> Integer,
        offline -> Bool,
        accessed_on -> Nullable<Float>,
        data -> Binary,
    }
}

table! {
    prevent_sync_pattern (_id) {
        _id -> Integer,
        pattern -> Text,
        fully_applied -> Bool,
    }
}

table! {
    realm_checkpoint (_id) {
        _id -> Integer,
        checkpoint -> Integer,
    }
}

table! {
    vlobs (vlob_id) {
        vlob_id -> Binary,
        base_version -> Integer,
        remote_version -> Integer,
        need_sync -> Bool,
        blob -> Binary,
    }
}

allow_tables_to_appear_in_same_query!(chunks, prevent_sync_pattern, realm_checkpoint, vlobs,);

// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

table! {
    block (_id) {
        _id -> Nullable<Integer>,
        block_id -> Binary,
        data -> Binary,
        author -> Integer,
        size -> Integer,
        created_on -> Double,
    }
}

table! {
    device (_id) {
        _id -> Nullable<Integer>,
        device_certificate -> Binary,
    }
}

table! {
    info (magic) {
        magic -> Integer,
        version -> Integer,
        realm_id -> Binary,
    }
}

table! {
    realm_role (_id) {
        _id -> Nullable<Integer>,
        role_certificate -> Binary,
    }
}

table! {
    user_ (_id) {
        _id -> Nullable<Integer>,
        user_certificate -> Binary,
    }
}

table! {
    vlob_atom (_id) {
        _id -> Nullable<Integer>,
        vlob_id -> Binary,
        version -> Integer,
        blob -> Binary,
        size -> Integer,
        author -> Integer,
        timestamp -> Double,
    }
}

joinable!(block -> device (author));
joinable!(vlob_atom -> device (author));

allow_tables_to_appear_in_same_query!(block, device, info, realm_role, user_, vlob_atom,);

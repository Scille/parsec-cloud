// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    block (_id) {
        _id -> BigInt,
        block_id -> Binary,
        data -> Binary,
        author -> BigInt,
        size -> BigInt,
        created_on -> Double,
    }
}

table! {
    device (_id) {
        _id -> BigInt,
        device_certificate -> Binary,
    }
}

table! {
    info (magic) {
        magic -> BigInt,
        version -> BigInt,
        realm_id -> Binary,
    }
}

table! {
    realm_role (_id) {
        _id -> BigInt,
        role_certificate -> Binary,
    }
}

table! {
    user_ (_id) {
        _id -> BigInt,
        user_certificate -> Binary,
    }
}

table! {
    vlob_atom (_id) {
        _id -> BigInt,
        vlob_id -> Binary,
        version -> BigInt,
        blob -> Binary,
        size -> BigInt,
        author -> BigInt,
        timestamp -> Double,
    }
}

joinable!(block -> device (author));
joinable!(vlob_atom -> device (author));

allow_tables_to_appear_in_same_query!(block, device, info, realm_role, user_, vlob_atom,);

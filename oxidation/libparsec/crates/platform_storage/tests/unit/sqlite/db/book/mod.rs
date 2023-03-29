// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use super::LocalDatabase;
use diesel::{sql_query, table, Insertable, Queryable, RunQueryDsl};

table! {
    books(id) {
        id -> Integer,
        name -> Text,
        liked -> Nullable<Bool>,
    }
}

#[derive(Insertable, Queryable, Debug, PartialEq, Eq)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub liked: Option<bool>,
}

pub async fn create_table(local_db: &LocalDatabase) {
    let res = local_db
        .exec(|conn| sql_query(std::include_str!("book_table.sql")).execute(conn))
        .await;

    assert_eq!(res, Ok(0));
}

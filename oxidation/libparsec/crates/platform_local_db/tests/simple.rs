// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use diesel::{ExpressionMethods, RunQueryDsl};
use platform_async::Notify;
use platform_local_db::LocalDatabase;

mod book;

#[tokio::test]
async fn creation_deletion() {
    let local_db = LocalDatabase::new_in_memory().await.unwrap();
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    let notified = notify.notified();

    let res = local_db
        .exec(move |_conn| {
            println!("We got executed !");
            notify2.notify_waiters();
            Ok(42)
        })
        .await
        .unwrap();

    assert_eq!(res, 42);

    notified.await;
    println!("We've got notified");

    drop(local_db)
}

#[tokio::test]
async fn basic_test() {
    use book::{books::dsl::*, create_table, Book};

    let local_db = LocalDatabase::new_in_memory().await.unwrap();

    create_table(&local_db).await;

    let res = local_db
        .exec(|conn| {
            let book = (id.eq(42), name.eq("Cyberpunk 2077".to_string()));
            let row = diesel::insert_into(books).values(&book).execute(conn)?;
            diesel::insert_into(books)
                .values(&Book {
                    id: 1,
                    name: "The Witcher".to_string(),
                    liked: Some(true),
                })
                .execute(conn)
                .map(|new_row| new_row + row)
        })
        .await;
    assert_eq!(res, Ok(2));

    let res = local_db.exec(|conn| books.load::<Book>(conn)).await;

    assert_eq!(
        res,
        Ok(vec![
            Book {
                id: 1,
                name: "The Witcher".to_string(),
                liked: Some(true)
            },
            Book {
                id: 42,
                name: "Cyberpunk 2077".to_string(),
                liked: None
            }
        ])
    )
}

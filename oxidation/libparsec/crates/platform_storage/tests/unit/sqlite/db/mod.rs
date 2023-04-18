// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{dsl::sql, sql_types, ExpressionMethods, RunQueryDsl};
use pretty_assertions::assert_eq;
use std::path::PathBuf;
use std::sync::Arc;

use libparsec_platform_async::Notify;
use libparsec_tests_fixtures::{parsec_test, tmp_path, TmpPath};

use crate::sqlite::{db::DatabaseError, AutoVacuum, LocalDatabase, VacuumMode};

mod book;

#[parsec_test]
async fn creation_deletion(tmp_path: TmpPath) {
    let local_db = LocalDatabase::from_path(
        &tmp_path,
        &PathBuf::from("db.sqlite"),
        VacuumMode::default(),
    )
    .await
    .unwrap();
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    let notified = notify.notified();

    let res = local_db
        .exec(move |_conn| {
            log::debug!("We got executed !");
            notify2.notify_waiters();
            Ok(42)
        })
        .await
        .unwrap();

    assert_eq!(res, 42);

    notified.await;
    log::debug!("We've got notified");

    local_db.close().await; // Gracious close to avoid conflict with `tmp_path`'s drop
}

#[parsec_test]
async fn basic_test(tmp_path: TmpPath) {
    use book::{books::dsl::*, create_table, Book};

    let local_db = LocalDatabase::from_path(
        &tmp_path,
        &PathBuf::from("db.sqlite"),
        VacuumMode::default(),
    )
    .await
    .unwrap();

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
    );

    local_db.close().await; // Gracious close to avoid conflict with `tmp_path`'s drop
}

#[parsec_test]
async fn test_set_auto_vacuum(
    tmp_path: TmpPath,
    #[values(AutoVacuum::None, AutoVacuum::Incremental, AutoVacuum::Full)] auto_vacuum: AutoVacuum,
) {
    let local_db = LocalDatabase::from_path(
        &tmp_path,
        &PathBuf::from("db.sqlite"),
        VacuumMode::Automatic(auto_vacuum),
    )
    .await;

    assert!(
        local_db.is_ok(),
        "Failed to create local_db: {}",
        local_db.err().unwrap()
    );

    let local_db = local_db.unwrap();

    let res = local_db
        .exec(|conn| sql::<sql_types::Text>("PRAGMA auto_vacuum").get_result::<AutoVacuum>(conn))
        .await
        .unwrap();

    assert_eq!(res, auto_vacuum);

    local_db.close().await; // Gracious close to avoid conflict with `tmp_path`'s drop
}

#[parsec_test]
async fn test_get_disk_usage(tmp_path: TmpPath) {
    use book::{books::dsl::*, create_table, Book};

    let local_db = LocalDatabase::from_path(
        &tmp_path,
        &PathBuf::from("db.sqlite"),
        VacuumMode::default(),
    )
    .await
    .expect("Failed to open local db");

    let starting_size = local_db.get_disk_usage().await;

    create_table(&local_db).await;

    const ONE_MEGA_BYTES: usize = 1 << 20;
    let foo: String = "A".repeat(ONE_MEGA_BYTES);
    let bar = foo.clone();
    local_db
        .exec(|conn| {
            diesel::insert_into(books)
                .values(&Book {
                    id: 42,
                    name: bar,
                    liked: None,
                })
                .execute(conn)
        })
        .await
        .unwrap();

    let res = local_db
        .exec(|conn| books.first::<Book>(conn))
        .await
        .expect("Missing book");

    assert_eq!(res.name, foo);

    let intermediate_size = local_db.get_disk_usage().await;

    assert!(starting_size < intermediate_size);
    assert!(ONE_MEGA_BYTES < intermediate_size);

    local_db.close().await; // Gracious close to avoid conflict with `tmp_path`'s drop
}

#[parsec_test]
async fn test_cannot_open_database(tmp_path: TmpPath) {
    let db_path = tmp_path.as_path().join("db.sqlite");

    // We create a folder with at the path that `LocalDatabase` will try to open as a file.
    tokio::fs::create_dir(db_path.as_path()).await.unwrap();

    let local_db_err = LocalDatabase::from_path(
        &tmp_path,
        &PathBuf::from("db.sqlite"),
        VacuumMode::default(),
    )
    .await
    .expect_err("We should failed because we can't read the directory as a file");

    let expected_error =
        DatabaseError::DieselConnectionError(diesel::result::ConnectionError::BadConnection(
            "Unable to open the database file".to_string(),
        ));

    assert_eq!(local_db_err, expected_error);
}

#[parsec_test]
async fn test_non_utf8_db_path(tmp_path: TmpPath) {
    let non_utf8_name = {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStringExt;
            // Here, the values 0x66 and 0x6f correspond to 'f' and 'o'
            // respectively. The value 0x80 is a lone continuation byte, invalid
            // in a UTF-8 sequence.
            let source = vec![0x66, 0x6f, 0x80, 0x6f];
            std::ffi::OsString::from_vec(source)
        }
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStringExt;
            // Here the values 0x0066 and 0x006f correspond to 'f' and 'o'
            // respectively. The value 0xD800 is a lone surrogate half, invalid
            // in a UTF-16 sequence.
            let source = [0x0066, 0x006f, 0xD800, 0x006f];
            std::ffi::OsString::from_wide(&source[..])
        }
    };
    let local_db_err = LocalDatabase::from_path(
        &tmp_path,
        &PathBuf::from(non_utf8_name),
        VacuumMode::default(),
    )
    .await
    .unwrap_err();

    let expected_error = DatabaseError::DieselConnectionError(
        diesel::result::ConnectionError::InvalidConnectionUrl(
            "Non-Utf-8 character found in db path".to_owned(),
        ),
    );

    assert_eq!(local_db_err, expected_error);
}

#[parsec_test]
async fn test_full_vacuum(tmp_path: TmpPath) {
    use book::{books::dsl::*, create_table, Book};

    let local_db = LocalDatabase::from_path(
        &tmp_path,
        &PathBuf::from("db.sqlite"),
        VacuumMode::WithThreshold(0),
    )
    .await
    .expect("Cannot open the database");

    let starting_size = local_db.get_disk_usage().await;
    create_table(&local_db).await;

    const ONE_MEGA_BYTES: usize = 1 << 20;
    let foo: String = "A".repeat(ONE_MEGA_BYTES);
    let bar = foo.clone();

    local_db
        .exec(|conn| {
            diesel::insert_into(books)
                .values(&Book {
                    id: 42,
                    name: bar,
                    liked: None,
                })
                .execute(conn)
        })
        .await
        .unwrap();

    let book = local_db
        .exec(|conn| books.first::<Book>(conn))
        .await
        .unwrap();

    assert_eq!(book.name, foo);

    let intermediate_size = local_db.get_disk_usage().await;

    assert!(starting_size < intermediate_size);
    assert!(ONE_MEGA_BYTES < intermediate_size);

    local_db
        .exec(|conn| diesel::delete(books).execute(conn))
        .await
        .unwrap();

    local_db.vacuum().await.unwrap();

    let final_size = local_db.get_disk_usage().await;

    assert!(final_size < intermediate_size);

    local_db.close().await; // Gracious close to avoid conflict with `tmp_path`'s drop
}

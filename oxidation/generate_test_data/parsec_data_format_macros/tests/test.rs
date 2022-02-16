// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use parsec_data_format_macros::ParsecSerialData;

#[test]
fn base() {
    #[derive(ParsecSerialData)]
    struct Person {
        pub name: String,
        pub age: u32,
    }

    Person {
        name: "Zack".to_string(),
        age: 42,
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum Maybe<T> {
    None,
    Some(T),
}

#[test]
fn maybe_field() {
    #[derive(ParsecSerialData)]
    struct Person {
        pub name: Maybe<String>,
        pub age: u32,
    }

    let p = Person {
        name: Maybe::Some("Zack".to_string()),
        age: 42,
    };

    assert_eq!(p.name, Maybe::Some("Zack".to_string()))
}

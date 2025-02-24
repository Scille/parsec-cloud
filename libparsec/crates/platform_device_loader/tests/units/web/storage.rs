// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::{p_assert_eq, p_assert_matches, parsec_test};

// use crate::web::{add_item_to_list, remove_item_from_list};
use crate::web::{add_item_to_list, remove_item_from_list};

#[parsec_test]
fn test_add_item_to_list() {
    const LIST_KEY: &str = stringify!(test_add_item_to_list);
    let window = web_sys::window().unwrap();
    let storage = window.session_storage().unwrap().unwrap();

    // The list is created if it does not exist
    p_assert_eq!(storage.get_item(LIST_KEY).unwrap(), None);
    let res = add_item_to_list(&storage, LIST_KEY, "item1").unwrap();
    assert!(res, "The item was added");
    p_assert_matches!(storage.get_item(LIST_KEY).unwrap(), Some(_));

    let raw = storage.get_item(LIST_KEY).unwrap().unwrap();
    let data = serde_json::from_str::<Vec<&str>>(&raw).unwrap();
    p_assert_eq!(data, ["item1"]);

    // We can add new items to the list
    let res = add_item_to_list(&storage, LIST_KEY, "item2").unwrap();
    assert!(res, "The item was added");
    let raw = storage.get_item(LIST_KEY).unwrap().unwrap();
    let mut data = serde_json::from_str::<Vec<&str>>(&raw).unwrap();
    data.sort();
    p_assert_eq!(data, ["item1", "item2"]);

    // The list should not contain duplicates
    let res = add_item_to_list(&storage, LIST_KEY, "item1").unwrap();
    assert!(!res, "The item should not be added a second time");
    let raw = storage.get_item(LIST_KEY).unwrap().unwrap();
    let mut data = serde_json::from_str::<Vec<&str>>(&raw).unwrap();
    data.sort();
    p_assert_eq!(data, ["item1", "item2"]);
}

#[parsec_test]
fn test_remove_item_to_list() {
    const LIST_KEY: &str = stringify!(test_remove_item_to_list);
    let window = web_sys::window().unwrap();
    let storage = window.session_storage().unwrap().unwrap();

    // The list is not created if it does not exist
    p_assert_eq!(storage.get_item(LIST_KEY).unwrap(), None);
    let res = remove_item_from_list(&storage, LIST_KEY, "item1").unwrap();
    assert!(!res, "Nothing was removed");
    p_assert_eq!(storage.get_item(LIST_KEY).unwrap(), None);

    // We can remove item from the list
    let data = serde_json::json!(["it1", "it2"]).to_string();
    storage.set_item(LIST_KEY, &data).unwrap();
    let res = remove_item_from_list(&storage, LIST_KEY, "it1").unwrap();
    assert!(res, "The item was removed");
    let raw = storage.get_item(LIST_KEY).unwrap().unwrap();
    let mut data = serde_json::from_str::<Vec<&str>>(&raw).unwrap();
    data.sort();
    p_assert_eq!(data, ["it2"]);

    // Try to remove an item that does not exist does nothing
    let res = remove_item_from_list(&storage, LIST_KEY, "it1").unwrap();
    assert!(!res, "Nothing was removed");
    let raw = storage.get_item(LIST_KEY).unwrap().unwrap();
    let mut data = serde_json::from_str::<Vec<&str>>(&raw).unwrap();
    data.sort();
    p_assert_eq!(data, ["it2"]);
}

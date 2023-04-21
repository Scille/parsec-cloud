// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

//! [reqwest::Proxy] don't implement `PartialEq` so we can't compare that easily.
//! I'm reduce to perform string comparaison for now.

use crate::ProxyConfig;
use std::env;

#[test]
fn test_default() {
    let config = ProxyConfig::default();

    assert!(config.http_proxy.is_none());
    assert!(config.https_proxy.is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_with_http_proxy() {
    let config = ProxyConfig::default().with_http_proxy("https://127.0.0.1:1337");

    let proxies = config
        .get_proxies()
        .collect::<reqwest::Result<Vec<_>>>()
        .expect("The provided proxy value should be OK");

    // We should only have 1 proxy configured.
    assert!(proxies.len() == 1);

    assert_eq!(
        format!("{:?}", proxies[0]),
        format!(
            "{:?}",
            reqwest::Proxy::http("https://127.0.0.1:1337").unwrap()
        )
    )
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_with_https_proxy() {
    let config = ProxyConfig::default().with_https_proxy("https://127.0.0.1:1337");

    let proxies = config
        .get_proxies()
        .collect::<reqwest::Result<Vec<_>>>()
        .expect("The provided proxy value should be OK");

    // We should only have 1 proxy configured.
    assert_eq!(proxies.len(), 1, "proxies={:?}", proxies);

    assert_eq!(
        format!("{:?}", proxies[0]),
        format!(
            "{:?}",
            reqwest::Proxy::https("https://127.0.0.1:1337").unwrap()
        )
    )
}

// This test test every possible combination.
// It can't be split because that would cause a `TOC/TOU` (Time to Check Time to use) error
// Where other tests are modifying the env variables `HTTP_PROXY` & `HTTPS_PROXY`.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_with_env() {
    assert_eq!(env::var(crate::HTTPS_PROXY), Err(env::VarError::NotPresent), "HTTPS_PROXY is already configured. Meaning it could be in use elsewhere, this will likely conflict with this test");
    assert_eq!(env::var(crate::HTTP_PROXY), Err(env::VarError::NotPresent), "HTTPS_PROXY is already configured. Meaning it could be in use elsewhere, this will likely conflict with this test");

    let config = ProxyConfig::default().with_env();

    let proxies = config
        .get_proxies()
        .collect::<reqwest::Result<Vec<_>>>()
        .expect("We should not have parsed a proxy url");

    assert!(proxies.is_empty());

    env::remove_var(crate::HTTPS_PROXY);
    env::set_var(crate::HTTP_PROXY, "https://only.http.proxy:1337");

    let config = ProxyConfig::default().with_env();

    let proxies = config
        .get_proxies()
        .collect::<reqwest::Result<Vec<_>>>()
        .expect("The provided proxy value should be OK");

    // We should only have 1 proxy configured.
    assert!(proxies.len() == 1);

    assert_eq!(
        format!("{:?}", proxies[0]),
        format!(
            "{:?}",
            reqwest::Proxy::http("https://only.http.proxy:1337").unwrap()
        )
    );

    env::remove_var(crate::HTTP_PROXY);
    env::set_var(crate::HTTPS_PROXY, "https://only.https.proxy:1337");

    let config = ProxyConfig::default().with_env();

    let proxies = config
        .get_proxies()
        .collect::<reqwest::Result<Vec<_>>>()
        .expect("The provided proxy value should be OK");

    // We should only have 1 proxy configured.
    assert!(proxies.len() == 1);

    assert_eq!(
        format!("{:?}", proxies[0]),
        format!(
            "{:?}",
            reqwest::Proxy::https("https://only.https.proxy:1337").unwrap()
        )
    );

    env::set_var(crate::HTTP_PROXY, "https://both.http.proxy:1337");
    env::set_var(crate::HTTPS_PROXY, "https://both.https.proxy:1337");

    let config = ProxyConfig::default().with_env();

    let proxies = config
        .get_proxies()
        .collect::<reqwest::Result<Vec<_>>>()
        .expect("The provided proxy value should be OK");

    // We should only have 2 proxies configured.
    assert_eq!(
        proxies.len(),
        2,
        "Invalid number of parsed proxies: {:?}",
        proxies
    );

    let proxies_list = format!("{:?}", proxies);

    // We remove the http proxy from the string
    let repr_http_proxy = format!(
        "{:?}",
        reqwest::Proxy::http("https://both.http.proxy:1337").unwrap()
    );
    let proxies_list_without_https_proxy = proxies_list.replacen(&repr_http_proxy, "", 1);

    // We remove the https proxy from the string
    let repr_https_proxy = format!(
        "{:?}",
        reqwest::Proxy::https("https://both.https.proxy:1337").unwrap()
    );
    let proxies_list_without_any_proxy =
        proxies_list_without_https_proxy.replacen(&repr_https_proxy, "", 1);

    assert_eq!(proxies_list_without_any_proxy, "[, ]")
}

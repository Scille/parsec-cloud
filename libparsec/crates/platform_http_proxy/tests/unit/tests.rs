// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! [reqwest::Proxy] don't implement `PartialEq` so we can't compare that easily.
//! I'm reduce to perform string comparaison for now.

use crate::ProxyConfig;
use std::env;

fn assert_http_proxy_eq(proxy: &ProxyConfig, expected: Option<&str>) {
    match &proxy.http_proxy {
        None => assert!(expected.is_none()),
        Some(proxy) => {
            let expected = expected.unwrap();
            assert_eq!(
                format!("{:?}", proxy),
                format!("{:?}", reqwest::Proxy::http(expected).unwrap())
            )
        }
    }
}

fn assert_https_proxy_eq(proxy: &ProxyConfig, expected: Option<&str>) {
    match &proxy.https_proxy {
        None => assert!(expected.is_none()),
        Some(proxy) => {
            let expected = expected.unwrap();
            assert_eq!(
                format!("{:?}", proxy),
                format!("{:?}", reqwest::Proxy::https(expected).unwrap())
            )
        }
    }
}

#[test]
fn default() {
    let config = ProxyConfig::default();

    assert!(config.http_proxy.is_none());
    assert!(config.https_proxy.is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn with_http_proxy() {
    let config = ProxyConfig::default()
        .with_http_proxy("https://127.0.0.1:1337".to_owned())
        .unwrap();

    // We should only have 1 proxy configured.
    assert_http_proxy_eq(&config, Some("https://127.0.0.1:1337"));
    assert_https_proxy_eq(&config, None);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn with_https_proxy() {
    let config = ProxyConfig::default()
        .with_https_proxy("https://127.0.0.1:1337".to_owned())
        .unwrap();

    // We should only have 1 proxy configured.
    assert_http_proxy_eq(&config, None);
    assert_https_proxy_eq(&config, Some("https://127.0.0.1:1337"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn bad_http_proxy() {
    let outcome = ProxyConfig::default().with_http_proxy("<not a valid proxy !>".to_owned());
    let err = outcome.as_ref().unwrap_err();

    let expected = "Invalid HTTP proxy configuration: builder error: builder error: relative URL without a base";
    assert!(err.to_string().starts_with(expected));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn bad_https_proxy() {
    let outcome = ProxyConfig::default().with_https_proxy("<not a valid proxy !>".to_owned());
    let err = outcome.as_ref().unwrap_err();

    let expected = "Invalid HTTPS proxy configuration: builder error: builder error: relative URL without a base";
    assert!(err.to_string().starts_with(expected));
}

// This test test every possible combination.
// It can't be split because that would cause a `TOC/TOU` (Time to Check Time to use) error
// Where other tests are modifying the env variables `HTTP_PROXY` & `HTTPS_PROXY`.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn with_env() {
    assert_eq!(env::var(crate::HTTPS_PROXY), Err(env::VarError::NotPresent), "HTTPS_PROXY is already configured. Meaning it could be in use elsewhere, this will likely conflict with this test");
    assert_eq!(env::var(crate::HTTP_PROXY), Err(env::VarError::NotPresent), "HTTPS_PROXY is already configured. Meaning it could be in use elsewhere, this will likely conflict with this test");

    let config = ProxyConfig::default().with_env().unwrap();
    assert!(config.http_proxy.is_none());
    assert!(config.https_proxy.is_none());

    env::remove_var(crate::HTTPS_PROXY);
    env::set_var(crate::HTTP_PROXY, "https://only.http.proxy:1337");

    let config = ProxyConfig::default().with_env().unwrap();
    assert_http_proxy_eq(&config, Some("https://only.http.proxy:1337"));
    assert_https_proxy_eq(&config, None);

    env::remove_var(crate::HTTP_PROXY);
    env::set_var(crate::HTTPS_PROXY, "https://only.https.proxy:1337");

    let config = ProxyConfig::default().with_env().unwrap();
    assert_http_proxy_eq(&config, None);
    assert_https_proxy_eq(&config, Some("https://only.https.proxy:1337"));

    env::set_var(crate::HTTP_PROXY, "https://both.http.proxy:1337");
    env::set_var(crate::HTTPS_PROXY, "https://both.https.proxy:1337");

    let config = ProxyConfig::default().with_env().unwrap();
    assert_http_proxy_eq(&config, Some("https://both.http.proxy:1337"));
    assert_https_proxy_eq(&config, Some("https://both.https.proxy:1337"));
}

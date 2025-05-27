// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug, PartialEq, Eq, Hash, Clone, SerializeDisplay, DeserializeFromStr)]
pub enum HostAddr {
    IP(std::net::IpAddr),
    Host(String),
}

impl std::fmt::Display for HostAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostAddr::IP(ip_addr) => std::fmt::Display::fmt(ip_addr, f),
            HostAddr::Host(host) => std::fmt::Display::fmt(host, f),
        }
    }
}

impl FromStr for HostAddr {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(v) => Ok(Self::IP(v)),
            Err(_) => Ok(Self::Host(s.to_owned())),
        }
    }
}

impl TryFrom<&str> for HostAddr {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};

use crate::{BlockCreateReq, BlockReadReq};

pub struct Authenticated;
pub struct Invited;
pub trait APICmdReq {
    const CMD: &'static str;
}
pub trait AuthenticatedCmd {}
pub trait InvitedCmd {}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Cmd<'a, T> {
    pub cmd: &'a str,
    #[serde(flatten)]
    pub req: T,
}

impl<'a, T: Deserialize<'a>> Cmd<'a, T> {
    pub fn loads(buf: &'a [u8]) -> Self {
        rmp_serde::from_read_ref::<_, Self>(buf).unwrap()
    }
}

fn dumps<C: APICmdReq + Serialize>(req: &C) -> Result<Vec<u8>, &'static str> {
    rmp_serde::to_vec_named(&Cmd { cmd: C::CMD, req }).map_err(|_| "Serialization failed")
}

impl Authenticated {
    pub fn dumps<C: APICmdReq + AuthenticatedCmd + Serialize>(
        req: &C,
    ) -> Result<Vec<u8>, &'static str> {
        dumps(req)
    }
}

impl Invited {
    pub fn dumps<C: APICmdReq + InvitedCmd + Serialize>(req: &C) -> Result<Vec<u8>, &'static str> {
        dumps(req)
    }
}

impl APICmdReq for BlockCreateReq {
    const CMD: &'static str = "block_create";
}

impl APICmdReq for BlockReadReq {
    const CMD: &'static str = "block_read";
}

impl AuthenticatedCmd for BlockReadReq {}
impl AuthenticatedCmd for BlockCreateReq {}

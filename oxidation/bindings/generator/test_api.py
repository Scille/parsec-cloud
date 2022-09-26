# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# Dummy API for testing the generator in different usecases
from typing import Optional, List, Generic, TypeVar


# Meta-types, not part of the API but to be used to describe the API


class Result(Generic[TypeVar("OK"), TypeVar("ERR")]):  # noqa
    pass


class Ref(Generic[TypeVar("REFERENCED")]):  # noqa
    pass


class StrBasedType:
    pass


class Variant:
    pass


class Structure:
    pass


# Should only be needed when doing tests&hacks...
BINDING_ELECTRON_METHS_INJECT_RUST_CODE = """
// Overwrite actual libparsec crate
mod libparsec {
    pub struct OrganizationID(String);

    impl std::str::FromStr for OrganizationID {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(s.to_owned()))
        }
    }

    impl From<OrganizationID> for String {
        fn from(item: OrganizationID) -> String {
            item.0
        }
    }

    impl std::convert::AsRef<str> for OrganizationID {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    pub fn f_r1(o: OrganizationID) -> OrganizationID { o }
    pub fn f_r2() -> bool {true}
    pub fn f_r3() -> i64 {42}
    pub fn f_r4() -> u32 {42}
    pub fn f_r5() -> f64 {42.}
    pub fn f_r6() -> String { "hello".to_owned() }
    pub fn f_r7() -> Vec<u8> { vec![1, 2, 3] }
    pub fn f_r8() -> Option<Data> { None }

    pub fn f_p1() {}
    pub fn f_p2(_a: bool) {}
    pub fn f_p3(_a: i64) {}
//    pub fn f_p4(_a: u32) {}  // TODO: support integere other than i64 ?
    pub fn f_p5(_a: f64) {}
    pub fn f_p6(_a: &str) {}
    pub fn f_p7(_a: &[u8]) {}

    pub struct SubData {
        pub s1: bool,
    }

    pub struct Data {
        pub p1: bool,
        pub p2: i64,
        pub p3: f64,
        pub p4: String,
        pub p5: Vec<u8>,
        pub p6: SubData,
        pub p7: Option<SubData>,
    }

    pub enum V1or2 {
        V1,
        V2 {v1: bool}
    }

    pub fn f_p8(_a: &Data) {}
    pub fn f_p9() -> Data { Data {
        p1: true,
        p2: 42,
        p3: 24.,
        p4: "foo".to_owned(),
        p5: vec![],
        p6: SubData { s1: false },
        p7: None,
    } }
    pub fn f_p10(_a: Option<bool>) {}
    pub fn f_p11(_a: &Option<Data>) {}
    pub fn f_p111(_a: Option<&Data>) {}
    // pub fn f_p112(_a: Option<&[Data]>) {}  // TODO: not supported yet :'(
    pub fn f_p12(_a: &V1or2) {}
    pub fn f_p13() -> V1or2 { V1or2::V1 }
    pub fn f_p14() -> Result<bool, V1or2> { Ok(true) }
    pub fn f_p15(_a: &[Data]) {}
    pub fn f_p16() -> Vec<Data> { vec![] }

    pub async fn af_r1() -> () {}
    pub async fn af_r2() -> bool {true}
    pub async fn af_r3() -> i64 {42}
    pub async fn af_r4() -> u32 {42}
    pub async fn af_r5() -> f64 {42.}
    pub async fn af_r6() -> String { "hello".to_owned() }
    pub async fn af_r7() -> Vec<u8> { vec![1, 2, 3] }
    pub async fn af_r8() -> Option<Data> { None }

    pub async fn af_p1() {}
    pub async fn af_p2(_a: bool) {}
    pub async fn af_p3(_a: i64) {}
    // pub async fn af_p4(_a: u32) {}  // TODO: support other than i64 ?
    pub async fn af_p5(_a: f64) {}
    pub async fn af_p6(_a: &str) {}
    pub async fn af_p7(_a: &[u8]) {}

    pub async fn af_p8(_a: &Data) {}
    pub async fn af_p9() -> Data { Data {
        p1: true,
        p2: 42,
        p3: 24.,
        p4: "foo".to_owned(),
        p5: vec![],
        p6: SubData { s1: false },
        p7: None,
    } }
    pub async fn af_p10(_a: Option<bool>) {}
    pub async fn af_p11(_a: &Option<Data>) {}
    pub async fn af_p12(_a: &V1or2) {}
    pub async fn af_p13() -> V1or2 { V1or2::V1 }
    pub async fn af_p14() -> Result<bool, V1or2> { Ok(true) }
    pub async fn af_p15(_a: &[Data]) {}
    pub async fn af_p16() -> Vec<Data> { vec![] }

    pub fn foo(a: i64) -> i64 {a}
}
"""


class OrganizationID(StrBasedType):
    pass


class SubData(Structure):
    s1: bool


class Data(Structure):
    p1: bool
    p2: int
    p3: float
    p4: str
    p5: bytes
    p6: SubData
    p7: Optional[SubData]


class V1or2(Variant):
    class V1:
        pass

    class V2:
        v1: bool


def foo(a: int) -> int:
    ...


def f_r1(_o: OrganizationID) -> OrganizationID:
    ...


def f_r2() -> bool:
    ...


def f_r3() -> int:
    ...


def f_r4() -> int:
    ...


def f_r5() -> float:
    ...


def f_r6() -> str:
    ...


def f_r7() -> bytes:
    ...


def f_r8() -> Optional[Data]:
    ...


def f_p1():
    ...


def f_p2(_a: bool):
    ...


def f_p3(_a: int):
    ...


# def f_p4(_a: int): ...
def f_p5(_a: float):
    ...


def f_p6(_a: Ref[str]):
    ...


def f_p7(_a: Ref[bytes]):
    ...


def f_p8(_a: Ref[Data]):
    ...


def f_p9() -> Data:
    ...


def f_p10(_a: Optional[bool]):
    ...


def f_p11(_a: Ref[Optional[Data]]):
    ...


def f_p111(_a: Optional[Ref[Data]]):
    ...


# def f_p112(_a: Optional[Ref[List[Data]]]): ...
def f_p12(_a: Ref[V1or2]):
    ...


def f_p13() -> V1or2:
    ...


def f_p14() -> Result[bool, V1or2]:
    ...


def f_p15(_a: Ref[List[Data]]):
    ...


def f_p16() -> List[Data]:
    ...


async def af_r1() -> None:
    ...


async def af_r2() -> bool:
    ...


async def af_r3() -> int:
    ...


async def af_r4() -> int:
    ...


async def af_r5() -> float:
    ...


async def af_r6() -> str:
    ...


async def af_r7() -> bytes:
    ...


async def af_p1():
    ...


async def af_p2(_a: bool):
    ...


async def af_p3(_a: int):
    ...


# async def af_p4(_a: int): ...
async def af_p5(_a: float):
    ...


async def af_p6(_a: Ref[str]):
    ...


async def af_p7(_a: Ref[bytes]):
    ...


async def af_r8() -> Optional[Data]:
    ...


async def af_p8(_a: Ref[Data]):
    ...


async def af_p9() -> Data:
    ...


async def af_p10(_a: Optional[bool]):
    ...


async def af_p11(_a: Ref[Optional[Data]]):
    ...


async def af_p12(_a: Ref[V1or2]):
    ...


async def af_p13() -> V1or2:
    ...


async def af_p14() -> Result[bool, V1or2]:
    ...


async def af_p15(_a: Ref[List[Data]]):
    ...


async def af_p16() -> List[Data]:
    ...

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// We don't want to have this crate depend on `libparsec_types` for it test
// given `libparsec_types` itself uses this crate... hence in case of bug in
// this crate we wouldn't be able to use the tests !
// So here we simulate `libparsec_types` by implementing the bare minimum that
// our macros need.

pub type Integer = i64;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct DeviceID(pub String);

pub mod rmp_serialize {
    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;

    #[derive(Debug)]
    pub enum Error {
        Io(std::io::Error),
        Encode(rmp::encode::ValueWriteError<std::io::Error>),
        LengthOverflow { kind: &'static str, len: usize },
    }

    pub trait Serialize {
        fn begin(&self) -> Fragment<'_>;
    }

    pub enum Fragment<'a> {
        Null,
        Bool(bool),
        I64(i64),
        U64(u64),
        F64(f64),
        Str(&'a str),
        Bytes(&'a [u8]),
        Seq(Box<dyn Seq + 'a>),
        Map(Box<dyn Map + 'a>),
    }

    pub trait Seq {
        fn len(&self) -> usize;

        fn next(&mut self) -> Option<&dyn Serialize>;
    }

    pub trait Map {
        fn len(&self) -> usize;

        fn next(&mut self) -> Option<(&str, &dyn Serialize)>;
    }

    pub struct SeqEntries<'a> {
        entries: Vec<&'a dyn Serialize>,
        cursor: usize,
    }

    impl<'a> SeqEntries<'a> {
        pub fn new(entries: Vec<&'a dyn Serialize>) -> Self {
            Self { entries, cursor: 0 }
        }
    }

    impl Seq for SeqEntries<'_> {
        fn len(&self) -> usize {
            self.entries.len()
        }

        fn next(&mut self) -> Option<&dyn Serialize> {
            let value = self.entries.get(self.cursor).copied();
            self.cursor += usize::from(value.is_some());
            value
        }
    }

    pub struct MapEntries<'a> {
        entries: Vec<(&'a str, &'a dyn Serialize)>,
        cursor: usize,
    }

    impl<'a> MapEntries<'a> {
        pub fn new(entries: Vec<(&'a str, &'a dyn Serialize)>) -> Self {
            Self { entries, cursor: 0 }
        }
    }

    impl Map for MapEntries<'_> {
        fn len(&self) -> usize {
            self.entries.len()
        }

        fn next(&mut self) -> Option<(&str, &dyn Serialize)> {
            let value = self.entries.get(self.cursor).copied();
            self.cursor += usize::from(value.is_some());
            value
        }
    }

    impl From<rmp::encode::ValueWriteError<std::io::Error>> for Error {
        fn from(value: rmp::encode::ValueWriteError<std::io::Error>) -> Self {
            Self::Encode(value)
        }
    }

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Self {
            Self::Io(value)
        }
    }

    pub fn to_vec(value: &dyn Serialize) -> Result<Vec<u8>, Error> {
        let mut output = Vec::new();
        write_value(&mut output, value)?;
        Ok(output)
    }

    fn usize_to_u32(kind: &'static str, len: usize) -> Result<u32, Error> {
        u32::try_from(len).map_err(|_| Error::LengthOverflow { kind, len })
    }

    fn write_value(writer: &mut Vec<u8>, value: &dyn Serialize) -> Result<(), Error> {
        write_fragment(writer, value.begin())
    }

    fn write_fragment(writer: &mut Vec<u8>, fragment: Fragment<'_>) -> Result<(), Error> {
        match fragment {
            Fragment::Null => {
                rmp::encode::write_nil(writer)?;
            }
            Fragment::Bool(value) => {
                rmp::encode::write_bool(writer, value)?;
            }
            Fragment::I64(value) => {
                let _ = rmp::encode::write_sint(writer, value)?;
            }
            Fragment::U64(value) => {
                let _ = rmp::encode::write_uint(writer, value)?;
            }
            Fragment::F64(value) => {
                let _ = rmp::encode::write_f64(writer, value)?;
            }
            Fragment::Str(value) => {
                let _ = rmp::encode::write_str(writer, value)?;
            }
            Fragment::Bytes(value) => {
                rmp::encode::write_bin(writer, value)?;
            }
            Fragment::Seq(mut seq) => {
                let _ = rmp::encode::write_array_len(writer, usize_to_u32("array", seq.len())?)?;
                while let Some(item) = seq.next() {
                    write_value(writer, item)?;
                }
            }
            Fragment::Map(mut map) => {
                let _ = rmp::encode::write_map_len(writer, usize_to_u32("map", map.len())?)?;
                while let Some((key, value)) = map.next() {
                    let _ = rmp::encode::write_str(writer, key)?;
                    write_value(writer, value)?;
                }
            }
        }
        Ok(())
    }

    impl<T: Serialize + ?Sized> Serialize for &T {
        fn begin(&self) -> Fragment<'_> {
            (*self).begin()
        }
    }

    impl Serialize for bool {
        fn begin(&self) -> Fragment<'_> {
            Fragment::Bool(*self)
        }
    }

    macro_rules! impl_signed_serialize {
        ($($ty:ty),+ $(,)?) => {
            $(
                impl Serialize for $ty {
                    fn begin(&self) -> Fragment<'_> {
                        Fragment::I64((*self).into())
                    }
                }
            )+
        };
    }

    macro_rules! impl_unsigned_serialize {
        ($($ty:ty),+ $(,)?) => {
            $(
                impl Serialize for $ty {
                    fn begin(&self) -> Fragment<'_> {
                        Fragment::U64((*self).into())
                    }
                }
            )+
        };
    }

    impl_signed_serialize!(i8, i16, i32, i64);
    impl_unsigned_serialize!(u8, u16, u32, u64);

    impl Serialize for f32 {
        fn begin(&self) -> Fragment<'_> {
            Fragment::F64((*self).into())
        }
    }

    impl Serialize for f64 {
        fn begin(&self) -> Fragment<'_> {
            Fragment::F64(*self)
        }
    }

    impl Serialize for str {
        fn begin(&self) -> Fragment<'_> {
            Fragment::Str(self)
        }
    }

    impl Serialize for String {
        fn begin(&self) -> Fragment<'_> {
            Fragment::Str(self.as_str())
        }
    }

    impl Serialize for bytes::Bytes {
        fn begin(&self) -> Fragment<'_> {
            Fragment::Bytes(self.as_ref())
        }
    }

    impl<T> Serialize for Option<T>
    where
        T: Serialize,
    {
        fn begin(&self) -> Fragment<'_> {
            match self {
                Some(value) => value.begin(),
                None => Fragment::Null,
            }
        }
    }

    impl<T> Serialize for Vec<T>
    where
        T: Serialize,
    {
        fn begin(&self) -> Fragment<'_> {
            let entries = self
                .iter()
                .map(|item| item as &dyn Serialize)
                .collect::<Vec<_>>();
            Fragment::Seq(Box::new(SeqEntries::new(entries)))
        }
    }

    impl<T> Serialize for HashSet<T>
    where
        T: Serialize + Eq + Hash,
    {
        fn begin(&self) -> Fragment<'_> {
            let entries = self
                .iter()
                .map(|item| item as &dyn Serialize)
                .collect::<Vec<_>>();
            Fragment::Seq(Box::new(SeqEntries::new(entries)))
        }
    }

    impl<T> Serialize for HashMap<String, T>
    where
        T: Serialize,
    {
        fn begin(&self) -> Fragment<'_> {
            let entries = self
                .iter()
                .map(|(key, value)| (key.as_str(), value as &dyn Serialize))
                .collect::<Vec<_>>();
            Fragment::Map(Box::new(MapEntries::new(entries)))
        }
    }

    impl<T> Serialize for super::Maybe<T>
    where
        T: Serialize,
    {
        fn begin(&self) -> Fragment<'_> {
            match self {
                super::Maybe::Present(value) => value.begin(),
                super::Maybe::Absent => Fragment::Null,
            }
        }
    }
}

impl rmp_serialize::Serialize for DeviceID {
    fn begin(&self) -> rmp_serialize::Fragment<'_> {
        rmp_serialize::Serialize::begin(self.0.as_str())
    }
}

#[allow(unused)]
pub enum ProtocolFamily {
    Family,
}

#[expect(dead_code)]
pub trait ProtocolRequest<const V: u32> {
    const FAMILY: ProtocolFamily;
    type Response: for<'de> Deserialize<'de>;

    fn api_dump(&self) -> Result<Vec<u8>, rmp_serde::encode::Error>;

    fn api_load_response(buf: &[u8]) -> Result<Self::Response, rmp_serde::decode::Error>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[expect(dead_code)]
pub struct ApiVersion {
    pub version: u32,
    pub revision: u32,
}

// Copy/paste of Maybe field... cannot go around that :(

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Maybe<T> {
    Present(T),
    Absent,
}

impl<T> Default for Maybe<T> {
    fn default() -> Self {
        Self::Absent
    }
}

impl<T> Maybe<T> {
    pub fn is_absent(&self) -> bool {
        matches!(self, Self::Absent)
    }
}

impl<T> From<T> for Maybe<T> {
    fn from(data: T) -> Self {
        Self::Present(data)
    }
}

impl<T, U> serde_with::SerializeAs<Maybe<T>> for Maybe<U>
where
    U: serde_with::SerializeAs<T>,
{
    fn serialize_as<S>(source: &Maybe<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *source {
            Maybe::Present(ref value) => {
                serializer.serialize_some(&serde_with::ser::SerializeAsWrap::<T, U>::new(value))
            }
            Maybe::Absent => serializer.serialize_none(),
        }
    }
}

use serde::Deserialize;
impl<'de, T, U> serde_with::DeserializeAs<'de, Maybe<T>> for Maybe<U>
where
    U: serde_with::DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Maybe<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Maybe::Present(
            serde_with::de::DeserializeAsWrap::<T, U>::deserialize(deserializer)?.into_inner(),
        ))
    }
}

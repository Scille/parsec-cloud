// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Maybe<T> {
    Present(T),
    #[default]
    Absent,
}

impl<T: Copy> Copy for Maybe<T> {}

impl<T> Maybe<T> {
    pub fn is_absent(&self) -> bool {
        matches!(self, Self::Absent)
    }
    pub fn unwrap_or(self, default: T) -> T {
        self.unwrap_or_else(|| default)
    }
    pub fn unwrap_or_else(self, default: impl FnOnce() -> T) -> T {
        match self {
            Self::Present(data) => data,
            Self::Absent => default(),
        }
    }
}

impl<T> From<T> for Maybe<T> {
    fn from(data: T) -> Self {
        Self::Present(data)
    }
}

impl<T: Default> Maybe<T> {
    pub fn unwrap_or_default(self) -> T {
        match self {
            Self::Present(data) => data,
            Self::Absent => Default::default(),
        }
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

impl<'de, T, U> serde_with::DeserializeAs<'de, Maybe<T>> for Maybe<U>
where
    U: serde_with::DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Maybe<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        Ok(Maybe::Present(
            serde_with::de::DeserializeAsWrap::<T, U>::deserialize(deserializer)?.into_inner(),
        ))
    }
}

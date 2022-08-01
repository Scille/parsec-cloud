// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[macro_export]
macro_rules! new_data_type_enum {
    (
        $name:ident,
        $value:literal
        $(,)?
    ) => {
        // Enum with single value works as a constant field
        #[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $name;

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                serializer.serialize_str($value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(concat!("the `", $value, "` string"))
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        if v == $value {
                            Ok($name)
                        } else {
                            Err(serde::de::Error::invalid_type(
                                serde::de::Unexpected::Str(v),
                                &self,
                            ))
                        }
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }
    };
}

#[macro_export]
macro_rules! new_data_struct_type {
    (
        $name:ident,
        type: $type_value:literal,
        $(
            $( #[$field_cfgs:meta] )*
            $field:ident : $field_type:ty
        ),*
        $(,)?
    ) => {

        ::paste::paste! {
            // Enum with single value works as a constant field
            $crate::data_macros::new_data_type_enum!([<$name DataType>], $type_value);

            #[serde_as]
            #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
            pub struct $name {

                #[serde(rename="type")]
                pub ty: [<$name DataType>],

                $(
                    $(#[$field_cfgs])*
                    pub $field: $field_type
                ),*

            }
        }

    };
}

#[macro_export]
macro_rules! impl_transparent_data_format_conversion {
    ($obj_type:ty, $data_type:ty, $($field:ident),* $(,)?) => {

        impl From<$data_type> for $obj_type {
            fn from(data: $data_type) -> Self {
                Self {
                    $($field: data.$field.into()),*
                }
            }
        }

        impl From<$obj_type> for $data_type {
            fn from(obj: $obj_type) -> Self {
                Self {
                    ty: Default::default(),
                    $($field: obj.$field.into()),*
                }
            }
        }

    };
}

pub use impl_transparent_data_format_conversion;
pub use new_data_struct_type;
pub use new_data_type_enum;

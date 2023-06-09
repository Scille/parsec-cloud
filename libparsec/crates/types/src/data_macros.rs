// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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

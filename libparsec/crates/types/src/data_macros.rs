// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[macro_export]
macro_rules! impl_transparent_data_format_conversion {
    // Variant with Maybe fields explicitly listed
    ($obj_type:ty, $data_type:ty, [$($maybe_field:ident),* $(,)?], $($field:ident),* $(,)?) => {

        impl From<$data_type> for $obj_type {
            #[allow(unused_variables)]
            fn from(data: $data_type) -> Self {
                Self {
                    $($maybe_field: data.$maybe_field.unwrap_or_default(),)*
                    $($field: data.$field.into()),*
                }
            }
        }

        impl From<$obj_type> for $data_type {
            #[allow(unused_variables)]
            fn from(obj: $obj_type) -> Self {
                Self {
                    ty: Default::default(),
                    $($maybe_field: obj.$maybe_field.into(),)*
                    $($field: obj.$field.into()),*
                }
            }
        }

    };
    // Variant without Maybe fields (backward-compatible)
    ($obj_type:ty, $data_type:ty, $($field:ident),* $(,)?) => {

        impl From<$data_type> for $obj_type {
            #[allow(unused_variables)]
            fn from(data: $data_type) -> Self {
                Self {
                    $($field: data.$field.into()),*
                }
            }
        }

        impl From<$obj_type> for $data_type {
            #[allow(unused_variables)]
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

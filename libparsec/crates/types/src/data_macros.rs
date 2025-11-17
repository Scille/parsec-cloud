// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[macro_export]
macro_rules! impl_transparent_data_format_conversion {
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

#[macro_export]
macro_rules! impl_data_format_conversion {
    (@internal $target:expr, can_fail $hook:expr) => {
        $hook($target)?
    };
    (@internal $target:expr, $hook:expr) => {
        $hook($target)
    };
    (@internal $target:expr $(,)?) => {
        $target.into()
    };

    (
        $obj_type:ty,
        $data_type:ty,
        $($field:ident $( with_hooks { serialize: $serialize_hook_fn:expr; deserialize: $deserialize_hook_fn:expr; } )? ),*
        $(,)?
    ) => {

        impl TryFrom<$data_type> for $obj_type {
            type Error = &'static str;

            fn try_from(data: $data_type) -> Result<Self, Self::Error> {
                $(
                    let $field = impl_data_format_conversion!(@internal data.$field, $( can_fail $deserialize_hook_fn )?);
                )*
                Ok(Self {
                    $($field),*
                })
            }
        }

        // Note this is not a `TryFrom` since our internal object cannot represent invalid data.
        impl From<$obj_type> for $data_type {
            fn from(obj: $obj_type) -> Self {
                $(
                    let $field = impl_data_format_conversion!(@internal obj.$field, $( $serialize_hook_fn )?);
                )*
                Self {
                    ty: Default::default(),
                    $($field),*
                }
            }
        }

    };
}

pub use impl_data_format_conversion;

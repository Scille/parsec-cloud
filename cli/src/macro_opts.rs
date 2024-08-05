/// This macros builds a Clap argument parser by combining the provided structure
/// with common CLI options.
///
/// Available common CLI options:
/// - `config_dir` (add `config_dir: std::path::PathBuf`)
/// - `device` (add `device: Option<String>`)
/// - `addr` (add `addr: libparsec::ParsecAddr`)
/// - `token` (add `token: String`)
///
/// Example:
///
/// ```rust
/// crate::clap_parser_with_shared_opts_builder!(
///     #[with = config_dir]
///     pub struct ListUsers {
///         #[arg(short, long, default_value_t)]
///         skip_revoked: bool,
///     }
/// );
/// ```
///
/// Becomes:
///
/// ```rust no_run
/// #[derive(clap::Parser)]
/// pub struct ListUsers {
///     #[arg(short, long, default_value_t)]
///     skip_revoked: bool,
///     #[doc = "Parsec config directory"]
///     #[arg(short, long, default_value_os_t = libparsec::get_default_config_dir(), env = $crate::utils::PARSEC_CONFIG_DIR)]
///     pub(crate) config_dir: std::path::PathBuf,
/// }
/// ```
#[macro_export]
macro_rules! clap_parser_with_shared_opts_builder {
    // Config dir option
    (
        #[with = config_dir $(,$modifier:ident)*]
        $(#[$struct_attr:meta])*
        $visibility:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field:ident: $field_type:ty,
            )*
        }
    ) => {
        $crate::clap_parser_with_shared_opts_builder!(
            #[with = $($modifier),*]
            $(#[$struct_attr])*
            $visibility struct $name {
                #[doc = "Parsec config directory"]
                #[arg(short, long, default_value_os_t = libparsec::get_default_config_dir(), env = $crate::utils::PARSEC_CONFIG_DIR)]
                pub(crate) config_dir: std::path::PathBuf,
                $(
                    $(#[$field_attr])*
                    $field_vis $field: $field_type,
                )*
            }
        );
    };
    // Device option
    (
        #[with = device $(,$modifier:ident)*]
        $(#[$struct_attr:meta])*
        $visibility:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field:ident: $field_type:ty,
            )*
        }
    ) => {
        $crate::clap_parser_with_shared_opts_builder!(
            #[with = $($modifier),*]
            $(#[$struct_attr])*
            $visibility struct $name {
                #[doc = "Device ID"]
                #[arg(short, long, env = "PARSEC_DEVICE_ID")]
                pub(crate) device: Option<String>,
                $(
                    $(#[$field_attr])*
                    $field_vis $field: $field_type,
                )*
            }
        );
    };
    // Password stdin option
    (
        #[with = password_stdin $(,$modifier:ident)*]
        $(#[$struct_attr:meta])*
        $visibility:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field:ident: $field_type:ty,
            )*
        }
    ) => {
        $crate::clap_parser_with_shared_opts_builder!(
            #[with = $($modifier),*]
            $(#[$struct_attr])*
            $visibility struct $name {
                #[doc = "Read the password from stdin instead of TTY"]
                #[doc = "Note: this flag need to be explicitly set, that why it does not have a env var"]
                #[arg(long, default_value_t)]
                pub(crate) password_stdin: bool,
                $(
                    $(#[$field_attr])*
                    $field_vis $field: $field_type,
                )*
            }
        );
    };
    // Server addr option
    (
        #[with = addr $(,$modifier:ident)*]
        $(#[$struct_attr:meta])*
        $visibility:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field:ident: $field_type:ty,
            )*
        }
    ) => {
        $crate::clap_parser_with_shared_opts_builder!(
            #[with = $($modifier),*]
            $(#[$struct_attr])*
            $visibility struct $name {
                #[doc = "Server address (e.g: parsec3://127.0.0.1:6770?no_ssl=true)"]
                #[arg(short, long, env = "PARSEC_SERVER_ADDR")]
                pub(crate) addr: libparsec::ParsecAddr,
                $(
                    $(#[$field_attr])*
                    $field_vis $field: $field_type,
                )*
            }
        );
    };
    // Server administration token option
    (
        #[with = token $(,$modifier:ident)*]
        $(#[$struct_attr:meta])*
        $visibility:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field:ident: $field_type:ty,
            )*
        }
    ) => {
        $crate::clap_parser_with_shared_opts_builder!(
            #[with = $($modifier),*]
            $(#[$struct_attr])*
            $visibility struct $name {
                #[doc = "Administration token"]
                #[arg(short, long, env = "PARSEC_ADMINISTRATION_TOKEN")]
                pub(crate) token: String,
                $(
                    $(#[$field_attr])*
                    $field_vis $field: $field_type,
                )*
            }
        );
    };
    (
        #[with =]
        $(#[$struct_attr:meta])*
        $visibility:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field:ident: $field_type:ty,
            )*
        }
    ) => {
        $(#[$struct_attr])*
        #[derive(clap::Parser)]
        $visibility struct $name {
            $(
                $(#[$field_attr])*
                $field_vis $field: $field_type,
            )*
        }
    };
}

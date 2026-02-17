/// This macro builds a main function that takes the `Args` struct defined in the same module.
///
/// Once called, it will initialize a client using the provided arguments and then call the sub-task and cleanup the client resource before returning.
///
/// the callback takes (args: Args, client: &StartedClient) as arguments and returns anyhow::Result<()>
#[macro_export]
macro_rules! build_main_with_client {
    ($fn_name:ident, $callback:expr, $config:expr) => {
        pub async fn $fn_name(args: Args) -> anyhow::Result<()> {
            let client = $crate::utils::load_client_with_config(
                &args.config_dir,
                args.device.clone(),
                args.password_stdin,
                $config,
            )
            .await?;

            let res = ($callback)(args, client.as_ref()).await;

            client.stop().await;

            res
        }
    };

    ($fn_name:ident, $callback:expr) => {
        $crate::build_main_with_client!(
            $fn_name,
            $callback,
            $crate::utils::default_client_config()
        );
    };
}

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
    // data dir option
    (
        #[with = data_dir $(,$modifier:ident)*]
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
                #[doc = "Parsec data directory"]
                #[arg(short, long, default_value_os_t = libparsec::get_default_data_base_dir(), env = $crate::utils::PARSEC_DATA_DIR)]
                pub(crate) data_dir: std::path::PathBuf,
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
    // Workspace option
    (
        #[with = workspace $(,$modifier:ident)*]
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
                #[doc = "Workspace ID"]
                #[arg(short, long, env = "PARSEC_WORKSPACE_ID", value_parser = libparsec::VlobID::from_hex)]
                pub(crate) workspace: libparsec::VlobID,
                $(
                    $(#[$field_attr])*
                    $field_vis $field: $field_type,
                )*
            }
        );
    };
    // Organization option
    (
        #[with = organization $(,$modifier:ident)*]
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
                #[doc = "Organization ID"]
                #[arg(short, long, env = "PARSEC_ORGANIZATION_ID")]
                pub(crate) organization: libparsec::OrganizationID,
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
        #[derive(clap::Parser)]
        $(#[$struct_attr])*
        $visibility struct $name {
            $(
                $(#[$field_attr])*
                $field_vis $field: $field_type,
            )*
        }
    };
}

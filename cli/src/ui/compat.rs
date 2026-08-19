use crate::ui;

use super::{CLIDisplay, Color, ColorFormatter};
use std::{borrow::Cow, io::Write, ops::Deref};

use reqwest::Url;
use serde::{ser::SerializeStruct, Serialize};

const DEVICE_ID_FIELD: &str = "device_id";
const ORGANIZATION_ID_FIELD: &str = "organization_id";
const USER_HUMAN_HANDLE_FIELD: &str = "human_handle";
const DEVICE_LABEL_FIELD: &str = "device_label";
const DEVICE_TYPE_FIELD: &str = "device_type";
const WORKSPACE_ID_FIELD: &str = "workspace_id";
const WORKSPACE_NAME_FIELD: &str = "workspace_name";
const WORKSPACE_STATUS_FIELD: &str = "workspace_status";
const USER_ID_FIELD: &str = "user_id";
const USER_PROFILE_FIELD: &str = "user_profile";
const USER_ROLE_FIELD: &str = "user_role";

pub struct AvailableDeviceDisplay(pub libparsec::AvailableDevice);

impl Deref for AvailableDeviceDisplay {
    type Target = libparsec::AvailableDevice;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for AvailableDeviceDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("AvailableDevice", 5)?;
        s.serialize_field(DEVICE_ID_FIELD, &self.device_id.hex())?;
        s.serialize_field(ORGANIZATION_ID_FIELD, &self.organization_id)?;
        s.serialize_field(USER_HUMAN_HANDLE_FIELD, &self.human_handle.to_string())?;
        s.serialize_field(DEVICE_LABEL_FIELD, &self.device_label)?;
        s.serialize_field(DEVICE_TYPE_FIELD, &self.ty.to_string())?;

        s.end()
    }
}

impl CLIDisplay for AvailableDeviceDisplay {
    fn plain_write<W: Write>(&self, fmt: &ColorFormatter, mut w: W) -> std::io::Result<()> {
        write!(
            w,
            "{dev_id} - {org_id}: {handle} @ {label} ({ty})",
            dev_id = fmt.wrap_in_color(
                Color::Yellow,
                format_args!(
                    "{id:.prec$}",
                    id = self.device_id.hex(),
                    prec = crate::utils::MINIMAL_SHORT_ID_SIZE
                )
            ),
            org_id = self.organization_id,
            handle = self.human_handle,
            label = self.device_label,
            ty = self.ty,
        )
    }
}

impl<T> CLIDisplay for &[T]
where
    T: CLIDisplay,
{
    fn plain_write<W: Write>(&self, fmt: &ColorFormatter, mut w: W) -> std::io::Result<()> {
        self.iter().try_for_each(|v| {
            w.write_all(b"- ")?;
            v.plain_write(fmt, &mut w)?;
            w.write_all(b"\n")
        })
    }
}

pub struct LocalDeviceDisplayRef<'a>(pub &'a libparsec_types::LocalDevice);

impl<'a> Deref for LocalDeviceDisplayRef<'a> {
    type Target = libparsec_types::LocalDevice;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> Serialize for LocalDeviceDisplayRef<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("LocalDevice", 4)?;
        s.serialize_field(DEVICE_ID_FIELD, &self.device_id.hex())?;
        s.serialize_field(ORGANIZATION_ID_FIELD, &self.organization_id())?;
        s.serialize_field(USER_HUMAN_HANDLE_FIELD, &self.human_handle.to_string())?;
        s.serialize_field(DEVICE_LABEL_FIELD, &self.device_label)?;
        s.end()
    }
}

impl<'a> CLIDisplay for LocalDeviceDisplayRef<'a> {
    fn plain_write<W: Write>(&self, fmt: &ColorFormatter, mut w: W) -> std::io::Result<()> {
        write!(
            w,
            "{dev_id} - {org_id}: {handle} @ {label}",
            dev_id = fmt.wrap_in_color(
                Color::Yellow,
                format_args!(
                    "{id:.prec$}",
                    id = self.device_id.hex(),
                    prec = crate::utils::MINIMAL_SHORT_ID_SIZE
                )
            ),
            org_id = self.organization_id(),
            handle = self.human_handle,
            label = self.device_label,
        )
    }
}

pub struct WorkspaceInfoDisplay(pub libparsec_client::WorkspaceInfo);

impl Deref for WorkspaceInfoDisplay {
    type Target = libparsec_client::WorkspaceInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for WorkspaceInfoDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("WorkspaceInfo", 3)?;
        s.serialize_field(WORKSPACE_ID_FIELD, &self.id.hex())?;
        s.serialize_field(WORKSPACE_NAME_FIELD, &self.name)?;
        s.serialize_field(WORKSPACE_STATUS_FIELD, &self.archiving_configuration)?;
        s.end()
    }
}

impl CLIDisplay for WorkspaceInfoDisplay {
    fn plain_write<W: Write>(&self, fmt: &ColorFormatter, mut w: W) -> std::io::Result<()> {
        let status = match &self.archiving_configuration {
            libparsec::RealmArchivingConfiguration::Available => Cow::Borrowed(""),
            libparsec::RealmArchivingConfiguration::Archived => Cow::Borrowed("[archived]"),
            libparsec::RealmArchivingConfiguration::DeletionPlanned { deletion_date } => {
                if deletion_date <= &libparsec_types::DateTime::now() {
                    Cow::Owned(format!(" [deleted since {deletion_date}]"))
                } else {
                    Cow::Owned(format!(" [deletion planned: {deletion_date}]"))
                }
            }
        };
        write!(
            w,
            "{id} - {name}: {role}{archiving_status}",
            id = fmt.wrap_in_color(Color::Yellow, self.id.hex()),
            name = self.name,
            role = self.self_role,
            archiving_status = status
        )
    }
}

pub struct WorkspaceUserAccessInfoDisplay(pub libparsec_client::WorkspaceUserAccessInfo);

impl Deref for WorkspaceUserAccessInfoDisplay {
    type Target = libparsec_client::WorkspaceUserAccessInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for WorkspaceUserAccessInfoDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("WorkspaceUserAccessInfo", 4)?;
        s.serialize_field(USER_ID_FIELD, &self.user_id.hex())?;
        s.serialize_field(USER_HUMAN_HANDLE_FIELD, &self.human_handle)?;
        s.serialize_field(USER_PROFILE_FIELD, &self.current_profile)?;
        s.serialize_field(USER_ROLE_FIELD, &self.current_role)?;
        s.end()
    }
}

impl CLIDisplay for WorkspaceUserAccessInfoDisplay {
    fn plain_write<W: Write>(&self, fmt: &ColorFormatter, mut w: W) -> std::io::Result<()> {
        write!(
            w,
            "User {id} ({profile}) - {name} ({email}) has role {role}",
            id = fmt.wrap_in_color(Color::Yellow, self.user_id),
            profile = fmt.wrap_in_color(Color::Yellow, self.current_profile),
            name = fmt.wrap_in_color(Color::Green, self.human_handle.label()),
            email = self.human_handle.email(),
            role = self.current_role,
        )
    }
}

impl CLIDisplay for serde_json::Value {
    fn plain_write<W: Write>(&self, _fmt: &ColorFormatter, w: W) -> std::io::Result<()> {
        serde_json::to_writer_pretty(w, self).map_err(Into::into)
    }
}

pub struct InviteItemDisplay<'a>(
    pub libparsec_client::InviteListItem,
    pub &'a [libparsec_client::UserInfo],
);

impl<'a> InviteItemDisplay<'a> {
    fn search_user_from_id(
        &self,
        user_id: libparsec_types::UserID,
    ) -> Option<&libparsec_client::UserInfo> {
        self.1.iter().find(|user| user.id == user_id)
    }
}

impl<'a> Deref for InviteItemDisplay<'a> {
    type Target = libparsec_client::InviteListItem;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Serialize for InviteItemDisplay<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            libparsec_client::InviteListItem::User {
                claimer_email,
                status,
                token,
                ..
            } => {
                let mut s = serializer.serialize_struct("InviteListItemUser", 4)?;
                s.serialize_field("invitation_type", "user")?;
                s.serialize_field("invitation_token", &token.hex())?;
                s.serialize_field("invitation_status", &status)?;
                s.serialize_field("claimer_email", &claimer_email)?;
                s.end()
            }
            libparsec_client::InviteListItem::Device { status, token, .. } => {
                let mut s = serializer.serialize_struct("InviteListItemDevice", 3)?;
                s.serialize_field("invitation_type", "device")?;
                s.serialize_field("invitation_token", &token.hex())?;
                s.serialize_field("invitation_status", &status)?;
                s.end()
            }
            libparsec_client::InviteListItem::ShamirRecovery {
                claimer_user_id,
                status,
                token,
                ..
            } => {
                let user = self.search_user_from_id(*claimer_user_id);
                let mut s = serializer.serialize_struct(
                    "InviteListItemShamir",
                    4 + if user.is_some() { 1 } else { 0 },
                )?;
                s.serialize_field("invitation_type", "shamir")?;
                s.serialize_field("invitation_token", &token.hex())?;
                s.serialize_field("invitation_status", &status)?;

                if let Some(user) = user {
                    s.serialize_field("claimer_human_handle", &user.human_handle)?;
                }
                s.serialize_field("claimer_user_id", &claimer_user_id)?;

                s.end()
            }
        }
    }
}

impl<'a> CLIDisplay for InviteItemDisplay<'a> {
    fn plain_write<W: Write>(&self, fmt: &ColorFormatter, mut w: W) -> std::io::Result<()> {
        fn format_status(
            fmt: &ColorFormatter,
            status: &libparsec_types::InvitationStatus,
        ) -> ui::StyledValue<&'static str> {
            match status {
                libparsec_types::InvitationStatus::Pending => {
                    fmt.wrap_in_color(Color::Yellow, "pending")
                }
                libparsec_types::InvitationStatus::Cancelled => {
                    fmt.wrap_in_color(Color::Red, "cancelled")
                }
                libparsec_types::InvitationStatus::Finished => {
                    fmt.wrap_in_color(Color::Green, "finished")
                }
            }
        }

        match &self.0 {
            libparsec_client::InviteListItem::User {
                claimer_email,
                status,
                token,
                ..
            } => write!(
                w,
                "{token}\t{status}\nuser (email={claimer_email})",
                token = token.hex(),
                status = format_status(fmt, status)
            ),
            libparsec_client::InviteListItem::Device { status, token, .. } => write!(
                w,
                "{token}\t{status}\tdevice",
                token = token.hex(),
                status = format_status(fmt, status)
            ),
            libparsec_client::InviteListItem::ShamirRecovery {
                status,
                token,
                claimer_user_id,
                ..
            } => {
                let user_info = self.search_user_from_id(*claimer_user_id);
                let detail = if let Some(user_info) = user_info {
                    format_args!("{}", user_info.human_handle)
                } else {
                    format_args!("id={claimer_user_id}")
                };
                write!(
                    w,
                    "{token}\t{status}\tshamir recovery ({detail})",
                    token = token.hex(),
                    status = format_status(fmt, status)
                )
            }
        }
    }
}

#[serde_with::serde_as]
#[derive(Serialize)]
pub struct InvitationLink {
    pub token: libparsec_types::AccessToken,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub url: Url,
}

impl CLIDisplay for InvitationLink {
    fn plain_write<W: Write>(&self, fmt: &ColorFormatter, mut w: W) -> std::io::Result<()> {
        writeln!(
            w,
            "Invitation token: {}",
            fmt.wrap_in_color(Color::Yellow, self.token)
        )?;
        write!(
            w,
            "Invitation URL: {}",
            fmt.wrap_in_color(Color::Yellow, &self.url)
        )
    }
}

pub struct TOSDisplay(pub libparsec_client::Tos);

impl serde::Serialize for TOSDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("TOS", 2)?;
        s.serialize_field("updated_on", &self.0.updated_on)?;
        s.serialize_field("per_locale_urls", &self.0.per_locale_urls)?;
        s.end()
    }
}

impl CLIDisplay for TOSDisplay {
    fn plain_write<W: std::io::prelude::Write>(
        &self,
        _fmt: &crate::ui::ColorFormatter,
        mut w: W,
    ) -> std::io::Result<()> {
        writeln!(w, "Terms of Service updated on {}:", self.0.updated_on)?;
        let mut sorted = self.0.per_locale_urls.iter().collect::<Vec<_>>();
        sorted.sort_by_key(|(locale, _)| *locale);
        for (locale, url) in sorted {
            writeln!(w, "- {locale}: {url}")?;
        }
        Ok(())
    }
}

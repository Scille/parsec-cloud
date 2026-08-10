use super::{CLIDisplay, Color, ColorFormatter};
use std::{borrow::Cow, io::Write, ops::Deref};

use serde::{ser::SerializeStruct, Serialize};

const DEVICE_ID_FIELD: &str = "device_id";
const ORGANIZATION_ID_FIELD: &str = "organization_id";
const USER_HUMAN_HANDLE_FIELD: &str = "human_handle";
const DEVICE_LABEL_FIELD: &str = "device_label";
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
        let mut s = serializer.serialize_struct("AvailableDevice", 4)?;
        s.serialize_field(DEVICE_ID_FIELD, &self.device_id.hex())?;
        s.serialize_field(ORGANIZATION_ID_FIELD, &self.organization_id)?;
        s.serialize_field(USER_HUMAN_HANDLE_FIELD, &self.human_handle.to_string())?;
        s.serialize_field(DEVICE_LABEL_FIELD, &self.device_label)?;
        s.end()
    }
}

impl CLIDisplay for AvailableDeviceDisplay {
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
            org_id = self.organization_id,
            handle = self.human_handle,
            label = self.device_label,
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

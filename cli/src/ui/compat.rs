use super::{CLIDisplay, Color, ColorFormatter};
use std::{io::Write, ops::Deref};

use serde::{ser::SerializeStruct, Serialize};

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
        s.serialize_field("device_id", &self.device_id.hex())?;
        s.serialize_field("organization_id", &self.organization_id)?;
        s.serialize_field("human_handle", &self.human_handle.to_string())?;
        s.serialize_field("device_label", &self.device_label)?;
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

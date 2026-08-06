pub mod compat;
mod spinners;
mod style;

use std::{
    fmt,
    io::{StderrLock, Write},
};

pub(crate) use spinners::Spinner;
pub use style::{DataFormat, ProgressStyle};

use clap::ColorChoice;

#[derive(Debug, Default)]
pub struct Ui {
    pub format: DataFormat,
    pub progress: ProgressStyle,
    pub color: ColorChoice,
}

impl Ui {
    pub fn data_print<T: CLIDisplay>(&self, data: &T) -> std::io::Result<()> {
        self.print_data(data, &mut std::io::stdout().lock())
    }

    fn print_data<T: CLIDisplay>(
        &self,
        data: &T,
        out: &mut std::io::StdoutLock,
    ) -> std::io::Result<()> {
        let color_formatter = self.color_formatter(out);

        match self.format {
            DataFormat::Plain => data.plain_write(&color_formatter, out),
            DataFormat::Json => {
                serde_json::to_writer_pretty(&mut *out, &data).map_err(std::io::Error::from)?;
                out.write_all(b"\n")
            }
        }
    }

    pub fn message_println<T: CLIDisplay>(&self, data: T) -> std::io::Result<()> {
        if self.progress == ProgressStyle::Quiet {
            return Ok(());
        }
        let mut stderr = std::io::stderr().lock();
        let color_formatter = self.color_formatter(&stderr);
        data.plain_write(&color_formatter, &mut stderr)?;
        stderr.write_all(b"\n")
    }

    pub fn with_message<F>(&self, f: F) -> std::io::Result<()>
    where
        F: FnOnce(ColorFormatter, &mut StderrLock) -> std::io::Result<()>,
    {
        let mut stderr = std::io::stderr().lock();
        let color_formatter = self.color_formatter(&stderr);
        f(color_formatter, &mut stderr)
    }

    fn color_formatter<T: std::io::IsTerminal>(&self, out: &T) -> ColorFormatter {
        ColorFormatter {
            color_enabled: self.color_enabled(out),
        }
    }

    fn color_enabled<T: std::io::IsTerminal>(&self, out: &T) -> bool {
        match self.color {
            ColorChoice::Auto => out.is_terminal(),
            ColorChoice::Always => true,
            ColorChoice::Never => false,
        }
    }

    pub fn with_spinner<F>(&self, f: F) -> std::io::Result<Spinner>
    where
        F: FnOnce(&ColorFormatter, &mut String) -> std::fmt::Result,
    {
        spinners::Spinner::start(self.progress, self.color_formatter(&std::io::stderr()), f)
    }
}

pub struct ColorFormatter {
    color_enabled: bool,
}

impl ColorFormatter {
    const RESET_STYLE: &str = crate::utils::RESET;

    pub fn wrap_in_color<D>(&self, color: Color, value: D) -> StyledValue<D> {
        StyledValue {
            color: if self.color_enabled {
                Some(color)
            } else {
                None
            },
            value,
        }
    }
}

pub struct StyledValue<D> {
    color: Option<Color>,
    value: D,
}

impl<D: fmt::Display> fmt::Display for StyledValue<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(color) = self.color {
            f.write_fmt(format_args!(
                "{color}{value}{reset}",
                color = color.to_ansi_escape_code(),
                value = self.value,
                reset = ColorFormatter::RESET_STYLE
            ))
        } else {
            self.value.fmt(f)
        }
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    Yellow,
    Green,
}

impl Color {
    const fn to_ansi_escape_code(self) -> &'static str {
        match self {
            Color::Yellow => crate::utils::YELLOW,
            Color::Green => crate::utils::GREEN,
        }
    }
}

pub trait CLIDisplay: serde::Serialize {
    fn plain_write<W: Write>(&self, fmt: &ColorFormatter, w: W) -> std::io::Result<()>;
}

impl<T: CLIDisplay> CLIDisplay for &T {
    fn plain_write<W: Write>(&self, fmt: &ColorFormatter, w: W) -> std::io::Result<()> {
        T::plain_write(self, fmt, w)
    }
}

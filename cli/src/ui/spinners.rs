use crate::ui::{ColorFormatter, ProgressStyle};

pub struct Spinner {
    mode: ProgressStyle,
    spinner: Option<spinners::Spinner>,
    color_formatter: ColorFormatter,
}

impl Spinner {
    pub fn start<F>(
        mode: ProgressStyle,
        color_formatter: ColorFormatter,
        f: F,
    ) -> std::io::Result<Self>
    where
        F: FnOnce(&ColorFormatter, &mut String) -> std::fmt::Result,
    {
        let mut spinner = None;

        if matches!(mode, ProgressStyle::Plain | ProgressStyle::Spinner) {
            let mut msg = String::new();
            f(&color_formatter, &mut msg).map_err(std::io::Error::other)?;
            if mode == ProgressStyle::Plain {
                eprintln!("{msg}");
            } else {
                spinner.replace(spinners::Spinner::with_stream(
                    spinners::Spinners::Dots,
                    msg,
                    spinners::Stream::Stderr,
                ));
            }
        }
        Ok(Self {
            mode,
            spinner,
            color_formatter,
        })
    }

    pub fn stop_with<F>(mut self, f: F) -> std::fmt::Result
    where
        F: FnOnce(&ColorFormatter, &mut String) -> std::fmt::Result,
    {
        if matches!(self.mode, ProgressStyle::Plain | ProgressStyle::Spinner) {
            let mut msg = String::new();
            f(&self.color_formatter, &mut msg)?;
            if let Some(mut spinner) = self.spinner.take() {
                spinner.stop_with_message(msg);
            } else {
                eprintln!("{msg}");
            }
        }
        Ok(())
    }

    pub fn stop(mut self) {
        if let Some(mut spinner) = self.spinner.take() {
            spinner.stop_with_newline();
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if let Some(mut spinner) = self.spinner.take() {
            spinner.stop_with_newline();
        }
    }
}

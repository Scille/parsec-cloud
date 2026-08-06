use libparsec::{EntryName, EntryStat, FsPath};
use serde::{ser::SerializeSeq, Serialize};

use crate::{ui::CLIDisplay, utils::StartedClient};

crate::clap_parser_with_shared_opts_builder!(
    #[with = config_dir, device, password_stdin, workspace]
    pub struct Args {
        /// The absolute workspace path to list contents (e.g. "/foo/bar")
        #[arg(default_value_t, value_hint = clap::ValueHint::AnyPath)]
        path: FsPath,
    }
);

crate::build_main_with_client!(main, ls);

pub async fn ls(ui: crate::Ui, args: Args, client: &StartedClient) -> anyhow::Result<()> {
    let Args {
        workspace, path, ..
    } = args;
    log::trace!("ls: {workspace}:{path}");

    let workspace = client.start_workspace(workspace).await?;
    let entries = workspace.stat_folder_children(&path).await?;

    ui.data_print(&LsEntries(entries))?;

    Ok(())
}

struct LsEntries(Vec<(EntryName, EntryStat)>);

impl Serialize for LsEntries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        self.0
            .iter()
            .try_for_each(|(name, _stat)| seq.serialize_element(name))?;
        seq.end()
    }
}

impl CLIDisplay for LsEntries {
    fn plain_write<W: std::io::prelude::Write>(
        &self,
        _fmt: &crate::ui::ColorFormatter,
        mut w: W,
    ) -> std::io::Result<()> {
        self.0
            .iter()
            .try_for_each(|(name, _stat)| w.write_fmt(format_args!("{name}\n")))
    }
}

use std::{
    fmt::Display,
    io::Cursor,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::{CommandFactory, ValueEnum};
use tokio::{
    io::{AsyncWrite, AsyncWriteExt, BufWriter},
    task::JoinSet,
};

#[derive(clap::Parser)]
pub struct Args {
    /// Man page(s) output path
    ///
    /// The output path should be a file or a directory depending on `mode`.
    // TODO: Support `-` once PR sequester is merged #13116
    #[arg(default_value = "-")]
    output: PathBuf,
    /// How the man pages would be written to the output path
    #[arg(long, default_value_t)]
    mode: Mode,
}

#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// All man entries are written in the single file
    #[default]
    AllInOne,
    /// Each man entry is written to its own file
    Separate,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Using a similar approach to `clap::ColorChoice`
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

pub async fn main(_ui: crate::Ui, args: Args) -> anyhow::Result<()> {
    let Args { output, mode } = args;

    anyhow::ensure!(
        output.as_os_str() != "-" || mode == Mode::AllInOne,
        "Cannot output separated manpage on stdout"
    );

    let mut cmd = crate::Arg::command()
        // Replace long version with short version, make man page generation more idempotent (on the
        // git side)
        .version(clap::crate_version!())
        // Overwrite default value of `sequester generate-service-certificate --datetime` option
        // It use `DateTime::now` making the value change on each call
        .mut_subcommand("sequester", |subcmd| {
            subcmd.mut_subcommand("generate-service-certificate", |cmd| {
                cmd.mut_arg("datetime", |arg| arg.default_value("2026-08-14T12:12:00Z"))
            })
        });

    // Required introspection.
    cmd.build();

    if output.as_os_str() == AsRef::<std::ffi::OsStr>::as_ref("-") {
        let stdout = tokio::io::stdout();
        let mut buffered = tokio::io::BufWriter::new(stdout);
        render_allinone_commands(&cmd, &mut buffered).await?;
        buffered
            .flush()
            .await
            .context("Cannot flush data to output")
    } else {
        match mode {
            Mode::AllInOne => {
                let file = tokio::fs::File::create(output)
                    .await
                    .context("Cannot create output file")?;
                let mut buffered = tokio::io::BufWriter::new(file);
                render_allinone_commands(&cmd, &mut buffered).await?;
                buffered
                    .flush()
                    .await
                    .context("Cannot flush data to output")
            }
            Mode::Separate => render_separate_commands(&cmd, &output).await,
        }
    }
}

async fn render_allinone_commands<W: AsyncWrite + Unpin>(
    cmd: &clap::Command,
    w: &mut BufWriter<W>,
) -> anyhow::Result<()> {
    write_man_to_file(cmd, w).await?;
    for sub_command in cmd.get_subcommands() {
        Box::pin(render_allinone_commands(sub_command, w)).await?;
    }
    Ok(())
}

async fn write_man_to_file<W: AsyncWrite + Unpin>(
    cmd: &clap::Command,
    w: &mut BufWriter<W>,
) -> anyhow::Result<()> {
    let man = clap_mangen::Man::new(cmd.clone());
    let buff = render_man_in_memory(man).with_context(|| {
        let cmd_name = cmd.get_name();

        format!("Failed to render man for command {cmd_name}")
    })?;
    w.write_all_buf(&mut std::io::Cursor::new(buff)).await?;
    Ok(())
}

async fn render_separate_commands(cmd: &clap::Command, out_dir: &Path) -> anyhow::Result<()> {
    anyhow::ensure!(
        tokio::fs::try_exists(out_dir).await?,
        "Output directory is missing: {}",
        out_dir.display()
    );
    let mut tasks = tokio::task::JoinSet::new();
    create_render_task(cmd, out_dir, &mut tasks);
    while let Some(task) = tasks.join_next().await {
        task??;
    }
    Ok(())
}

fn create_render_task(
    cmd: &clap::Command,
    out_dir: &Path,
    join_set: &mut JoinSet<anyhow::Result<()>>,
) {
    let man = clap_mangen::Man::new(cmd.clone());
    let path = out_dir.join(man.get_filename());
    join_set.spawn(write_man_in_file(man, path));
    for sub_command in cmd.get_subcommands() {
        if sub_command.get_name() != "help" {
            create_render_task(sub_command, out_dir, join_set);
        }
    }
}

async fn write_man_in_file(man: clap_mangen::Man, filepath: PathBuf) -> anyhow::Result<()> {
    log::debug!("Writing man page to {}", filepath.display());
    let buff = render_man_in_memory(man)?;

    let mut file = tokio::fs::File::create(&filepath)
        .await
        .with_context(|| format!("Failed to open file {}", filepath.display()))?;

    file.write_all_buf(&mut Cursor::new(buff))
        .await
        .context("Failed to write manpage to writer")?;
    file.flush().await.context("Failed to flush to file")?;
    log::debug!("Done writing man page to {}", filepath.display());
    Ok(())
}

fn render_man_in_memory(man: clap_mangen::Man) -> anyhow::Result<Vec<u8>> {
    let mut buff = Vec::with_capacity(1024);
    man.render(&mut buff).context("Failed to render man page")?;
    Ok(buff)
}

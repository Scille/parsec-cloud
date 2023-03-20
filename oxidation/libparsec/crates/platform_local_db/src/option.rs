// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
//! Module that store the different configuration possible on sqlite.

use std::{
    collections::HashMap,
    fmt::{Display, Write},
    str::FromStr,
};

use diesel::connection::SimpleConnection;

use crate::{executor::SqliteExecutor, DatabaseResult};

/// Represent the different Synchronous mode in Sqlite.
/// see https://www.sqlite.org/pragma.html#pragma_synchronous for more information about the different mode.
pub enum Synchronous {
    /// On top of [Synchronous::Full] mode, it will sync the rollback journal.
    Extra,
    /// Ensure that all content are safely written to the disk before continuing.
    Full,
    /// Normal mode, will sync at the most critical moments.
    Normal,
    /// Disable Synchronous mode.
    Off,
}

impl Display for Synchronous {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Synchronous::Extra => "EXTRA",
            Synchronous::Full => "FULL",
            Synchronous::Normal => "NORMAL",
            Synchronous::Off => "OFF",
        };
        f.write_str(s)
    }
}

impl FromStr for Synchronous {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "OFF" => Ok(Synchronous::Off),
            "1" | "NORMAL" => Ok(Synchronous::Normal),
            "2" | "FULL" => Ok(Synchronous::Full),
            "3" | "EXTRA" => Ok(Synchronous::Extra),
            _ => Err(format!("Invalid Synchronous value `{s}`")),
        }
    }
}

/// Represent the different journal mode in Sqlite.
/// see https://www.sqlite.org/pragma.html#pragma_journal_mode for more information about this pragma.
pub enum JournalMode {
    /// This is the normal behavior.
    /// The rollback journal is deleted after each transaction.
    Delete,
    /// Commit the transaction by truncating the rollback journal instead of deleting it.
    Truncate,
    /// Prevent the rollback journal from being deleted after a transaction.
    Persist,
    /// Store the rollback journal in RAM
    Memory,
    /// Use a Write Ahead log instead of a rollback journal to implement transactions.
    Wal,
    /// Disable the rollback journal
    Off,
}

impl Display for JournalMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            JournalMode::Delete => "DELETE",
            JournalMode::Truncate => "TRUNCATE",
            JournalMode::Persist => "PERSIST",
            JournalMode::Memory => "MEMORY",
            JournalMode::Wal => "WAL",
            JournalMode::Off => "OFF",
        };
        f.write_str(s)
    }
}

impl FromStr for JournalMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DELETE" => Ok(JournalMode::Delete),
            "TRUNCATE" => Ok(JournalMode::Truncate),
            "PERSIST" => Ok(JournalMode::Persist),
            "MEMORY" => Ok(JournalMode::Memory),
            "WAL" => Ok(JournalMode::Wal),
            "OFF" => Ok(JournalMode::Off),
            _ => Err(format!("Invalid JournalMode value `{s}`")),
        }
    }
}
/// Represent the option for auto vacuum mode.
/// Note: If you change the auto vacuum mode you may need to manually execute `VACUUM` to be in a correct state for sqlite.
/// see https://www.sqlite.org/pragma.html#pragma_auto_vacuum for more information.
#[derive(Debug, PartialEq, Eq, Clone, Copy, diesel::deserialize::FromSqlRow)]
pub enum AutoVacuum {
    /// The use of this mode only work when used alongside [`incremental_vacuum` pragma](https://www.sqlite.org/pragma.html#pragma_incremental_vacuum)
    Incremental,
    /// The freelist page are moved to the end of the database file and then the file is truncated.
    Full,
    /// This is the default behavior.
    /// Sqlite does not auto vacuum, you need to execute `VACUUM` for that.
    None,
}

impl Display for AutoVacuum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AutoVacuum::Incremental => "INCREMENTAL",
            AutoVacuum::Full => "FULL",
            AutoVacuum::None => "NONE",
        };
        f.write_str(s)
    }
}

impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for AutoVacuum
where
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite>,
{
    fn from_sql(
        bytes: diesel::backend::RawValue<'_, diesel::sqlite::Sqlite>,
    ) -> diesel::deserialize::Result<Self> {
        use diesel::{deserialize::FromSql, sql_types::Text, sqlite::Sqlite};

        <String as FromSql<Text, Sqlite>>::from_sql(bytes).and_then(|raw| {
            AutoVacuum::from_str(&raw).map_err(Box::<dyn std::error::Error + Send + Sync>::from)
        })
    }
}

impl FromStr for AutoVacuum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "NONE" => Ok(AutoVacuum::None),
            "1" | "FULL" => Ok(AutoVacuum::Full),
            "2" | "INCREMENTAL" => Ok(AutoVacuum::Incremental),
            _ => Err(format!("Invalid Auto vacuum value `{s}`")),
        }
    }
}
impl AutoVacuum {
    /// Method to safely configure the [AutoVacuum] mode.
    /// If the current mode is different than the configured in the Sqlite database,
    /// It will set it and run `VACUUM` as indicated by the documentation
    pub(crate) async fn safely_set_value(&self, conn: &SqliteExecutor) -> DatabaseResult<()> {
        use diesel::{dsl::sql, prelude::*, sql_types::Text};

        let res = conn
            .exec(|conn| sql::<Text>("PRAGMA auto_vacuum").get_result::<AutoVacuum>(conn))
            .await??;
        if &res == self {
            return Ok(());
        }

        let opt = SqliteOptions::default()
            .auto_vacuum(*self)
            .to_sql_batch_query()
            + "VACUUM;";
        conn.exec(move |conn| conn.batch_execute(&opt)).await??;
        Ok(())
    }
}

/// Builder to generate a `sql` query to set different option on a sqlite connection.
#[derive(Default)]
pub struct SqliteOptions {
    /// Store the different pragma to set
    /// The key is the pragma name.
    custom_pragma: HashMap<&'static str, String>,
}

impl SqliteOptions {
    /// Set the pragma `journal_mode`, see [JournalMode] for more information.
    pub fn journal_mode(&mut self, mode: JournalMode) -> &mut Self {
        self.pragma("journal_mode", mode)
    }

    /// Set the pragma `syncrhonous`, see [Synchronous] for more information.
    pub fn synchronous(&mut self, sync: Synchronous) -> &mut Self {
        self.pragma("synchronous", sync)
    }

    /// Set the pragma `auto_vacuum`, see [AutoVacuum] for more information.
    pub fn auto_vacuum(&mut self, mode: AutoVacuum) -> &mut Self {
        self.pragma("auto_vacuum", mode)
    }

    /// Set a pragma value.
    pub fn pragma<S: ToString>(&mut self, key: &'static str, value: S) -> &mut Self {
        self.custom_pragma.insert(key, value.to_string());
        self
    }
}

impl SqliteOptions {
    /// Generate a SQL query to set the different pragma provided to [SqliteOptions].
    pub fn to_sql_batch_query(&self) -> String {
        /// The KEYWORD used by sqlite to be used when working with pragma.
        const PRAGMA_KEY: &str = "PRAGMA";
        /// Represent the mean size of a pragma query.
        /// How it's composed:
        ///
        /// 1. The size of the pragma key word.
        /// 2. A Space between the key word and the key.
        /// 3. The mean size of all possible pragmas.
        /// 4. One for the `=`.
        /// 5. One for the `;`.
        const MEAN_PRAGMA_QUERY_SIZE: usize = PRAGMA_KEY.len() + 1 + 14 + 1 + 1;
        let mut s = String::with_capacity(MEAN_PRAGMA_QUERY_SIZE * self.custom_pragma.len());

        self.custom_pragma.iter().for_each(|(key, value)| {
            s.write_fmt(format_args!("{PRAGMA_KEY} {key}={value};"))
                .expect("Failed to write pragma to the pragma buffer");
        });

        s
    }
}

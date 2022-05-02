// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use diesel::backend::Backend;
use diesel::expression::ValidGrouping;
use diesel::query_builder::{AstPass, QueryFragment};
use diesel::sql_types::BigInt;
use diesel::{Expression, QueryResult};

pub fn coalesce_total_size() -> CoalesceTotalSize {
    CoalesceTotalSize
}

#[derive(Debug, Clone, Copy, QueryId, DieselNumericOps, ValidGrouping)]
pub struct CoalesceTotalSize;

impl Expression for CoalesceTotalSize {
    type SqlType = BigInt;
}

impl<DB: Backend> QueryFragment<DB> for CoalesceTotalSize {
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        out.push_sql("COALESCE(SUM(size), 0)");
        Ok(())
    }
}

impl_selectable_expression!(CoalesceTotalSize);

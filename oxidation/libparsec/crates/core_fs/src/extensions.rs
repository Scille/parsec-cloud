// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::backend::Backend;
use diesel::expression::ValidGrouping;
use diesel::query_builder::{AstPass, QueryFragment};
use diesel::sql_types::BigInt;
use diesel::{impl_selectable_expression, DieselNumericOps, Expression, QueryId, QueryResult};

#[derive(Default, Debug, Clone, Copy, QueryId, DieselNumericOps, ValidGrouping)]
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

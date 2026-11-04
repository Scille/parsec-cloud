# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections.abc import Iterable, Iterator

from asyncpg import Record
from asyncpg.pool import PoolConnectionProxy

from parsec._parsec import OrganizationID

type Conn = PoolConnectionProxy[Record]


async def fetch_org_id_from_org_name(conn: Conn, id: OrganizationID) -> int:
    id = await conn.fetchval("SELECT _id FROM organization WHERE organization_id = $1", id.str)
    assert isinstance(id, int)
    return id


async def fetch_references_across_tables(
    conn: Conn, tables_and_cols: Iterable[tuple[str, str]], referencing_ids: Iterable[int]
) -> tuple[list[int], ...]:
    res = []
    for table, col in tables_and_cols:
        ids = await fetch_many_row_ids_from_references(conn, table, col, referencing_ids)
        res.append(ids)

    return tuple(res)


async def fetch_many_row_ids_from_references(
    conn: Conn, table: str, fkey_col_name: str, references: Iterable[int]
) -> list[int]:
    rows = await conn.fetchmany(
        f"SELECT _id as id FROM {table} WHERE {fkey_col_name} = $1",
        ((id,) for id in references),
    )
    return [int(row["id"]) for row in rows]


def gen_table_ids(table_name: str, ids: Iterable[int]) -> Iterator[tuple[str, int]]:
    yield from ((table_name, id) for id in ids)

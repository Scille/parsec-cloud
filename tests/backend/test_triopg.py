import asyncpg
import trio_asyncio
import pytest

from parsec.backend.drivers.postgresql import triopg
from tests.conftest import TRIOPG_POSTGRESQL_TEST_URL


@pytest.fixture()
@trio_asyncio.trio2aio
async def asyncpg_conn(asyncio_loop):
    return await asyncpg.connect(TRIOPG_POSTGRESQL_TEST_URL)


async def execute_queries(triopg_conn, asyncpg_conn):
    @trio_asyncio.trio2aio
    async def _asyncpg_query(sql):
        return await asyncpg_conn.execute(sql)

    # Execute without transaction
    await triopg_conn.execute(
        """
        DROP TABLE IF EXISTS users;
        CREATE TABLE IF NOT EXISTS users (
            _id SERIAL PRIMARY KEY,
            user_id VARCHAR(32) UNIQUE
        )"""
    )

    assert await _asyncpg_query("""SELECT * FROM users""") == "SELECT 0"
    # Execute in transaction without exception
    async with triopg_conn.transaction():
        await triopg_conn.execute("INSERT INTO users (user_id) VALUES (1)")
    assert await _asyncpg_query("""SELECT * FROM users""") == "SELECT 1"

    # Execute in transaction raising exception
    with pytest.raises(Exception):
        async with triopg_conn.transaction():
            await triopg_conn.execute("INSERT INTO users (user_id) VALUES (2)")
            raise Exception

    assert await _asyncpg_query("""SELECT * FROM users""") == "SELECT 1"


@pytest.mark.trio
async def test_triopg_connection(asyncpg_conn):
    conn = await triopg.connect(TRIOPG_POSTGRESQL_TEST_URL)
    await execute_queries(conn, asyncpg_conn)
    await conn.close()


@pytest.mark.trio
async def test_triopg_pool(asyncpg_conn):
    pool = await triopg.create_pool(TRIOPG_POSTGRESQL_TEST_URL)
    async with pool.acquire() as conn:
        await execute_queries(conn, asyncpg_conn)
    await pool.close()

import pytest

from parsec.backend.drivers.postgresql import triopg


async def execute_queries(conn):
    async with conn.transaction():
        res = await conn.execute("""SELECT * FROM users""")
    assert res == "SELECT 0"

    async with conn.transaction():
        await conn.execute("INSERT INTO users (user_id) VALUES (1)")

    async with conn.transaction():
        res = await conn.execute("""SELECT * FROM users""")
    assert res == "SELECT 1"

    with pytest.raises(Exception):
        async with conn.transaction():
            await conn.execute("INSERT INTO users (user_id) VALUES (2)")
            raise Exception

    async with conn.transaction():
        res = await conn.execute("""SELECT * FROM users""")
    assert res == "SELECT 1"

    # Test get attribute
    conn._addr


@pytest.mark.trio
async def test_triopg_connection(backend_store):
    if not backend_store.startswith("postgresql"):
        pytest.skip()

    conn = await triopg.connect(backend_store)
    await execute_queries(conn)
    await conn.close()


@pytest.mark.trio
async def test_triopg_pool(backend_store):
    if not backend_store.startswith("postgresql"):
        pytest.skip()

    pool = await triopg.create_pool(backend_store)

    success = False
    async with pool.acquire() as conn:

        async with pool.acquire() as conn:
            pass

        await execute_queries(conn)

        success = True

    await pool.close()

    assert success

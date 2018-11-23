import pytest
import triopg

from parsec.backend.drivers.postgresql.handler import init_db, get_root_verify_key


@pytest.mark.trio
async def test_init_db(postgresql_url, backend_factory):
    keys1 = await init_db(postgresql_url, "alice", "pc1", True)

    async with triopg.connect(postgresql_url) as conn:
        verify_key_1 = await get_root_verify_key(conn)
    assert verify_key_1 == keys1[0]

    await init_db(postgresql_url, "alice", "pc1", False)

    async with triopg.connect(postgresql_url) as conn:
        verify_key_2 = await get_root_verify_key(conn)
    assert verify_key_1 == verify_key_2

    keys3 = await init_db(postgresql_url, "bob", "pc1", True)

    async with triopg.connect(postgresql_url) as conn:
        verify_key_3 = await get_root_verify_key(conn)

    assert verify_key_3 != verify_key_1
    assert keys3[1] != keys1[1]
    assert keys3[2] != keys1[2]

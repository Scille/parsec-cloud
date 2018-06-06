import pytest

from tests.common import connect_backend
from parsec.backend.drivers.postgresql.handler import TrioPG
from trio_asyncio import trio2aio


@pytest.mark.trio
async def test_connection(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "ping", "ping": "42"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "pong": "42"}


@pytest.mark.trio
async def test_bad_cmd(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "dummy"})
        rep = await sock.recv()
        assert rep == {"status": "unknown_command", "reason": "Unknown command"}


@pytest.mark.trio
async def test_bad_msg_format(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.sockstream.send_all(b"fooo\n")
        rep = await sock.recv()
        assert rep == {"status": "invalid_msg_format", "reason": "Invalid message format"}


@pytest.mark.trio
async def test_triopg(backend_store):
    if not backend_store.startswith("postgresql"):
        pytest.skip()

    @trio2aio
    async def _execute(sql):
        return await conn.execute(sql)

    async with TrioPG(backend_store) as conn:
        res = await _execute("""SELECT * FROM users""")
        assert res == "SELECT 0"

    async with TrioPG(backend_store) as conn:
        await _execute("INSERT INTO users (user_id) VALUES (1)")

    async with TrioPG(backend_store) as conn:
        res = await _execute("""SELECT * FROM users""")
        assert res == "SELECT 1"

    try:
        async with TrioPG(backend_store) as conn:
            await _execute("INSERT INTO users (user_id) VALUES (2)")
            raise Exception
    except Exception:
        pass

    async with TrioPG(backend_store) as conn:
        res = await _execute("""SELECT * FROM users""")
        assert res == "SELECT 1"

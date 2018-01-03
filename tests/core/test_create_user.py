import pytest

from tests.common import connect_core
from tests.conftest import core as core_factory


@pytest.mark.trio
async def test_user_create_not_logged(core):
    async with connect_core(core) as sock:
        await sock.send({'cmd': 'user_invite', 'id': 'John'})
        rep = await sock.recv()
    assert rep == {'status': 'login_required'}


@pytest.mark.trio
async def test_user_create_backend_offline(core, alice_core_sock):
    await alice_core_sock.send({'cmd': 'user_invite', 'id': 'John'})
    rep = await alice_core_sock.recv()
    assert rep == {'status': 'backend_not_availabled'}


@pytest.mark.trio
async def test_user_create_ok(backend_addr, running_backend, alice_core_sock, core, mallory):
    await alice_core_sock.send({'cmd': 'user_invite', 'id': mallory.user_id})
    rep = await alice_core_sock.recv()
    assert rep['status'] == 'ok'
    assert rep['id'] == 'mallory'
    assert rep['token']
    token = rep['token']

    # Create a brand new core and try to use the token to register
    core_gen = core_factory(backend_addr, ())
    mallory_core = await core_gen.asend(None)
    try:
        async with connect_core(mallory_core) as mallory_core_sock:
            await mallory_core_sock.send({
                'cmd': 'user_claim',
                'id': 'mallory@device1',
                'token': token
            })
            rep = await mallory_core_sock.recv()
            assert rep == {
                'status': 'ok',
            }
    finally:
        try:
            await core_gen.asend(None)
        except StopAsyncIteration:
            pass

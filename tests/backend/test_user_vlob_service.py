import pytest
import asyncio

from parsec.server import BaseServer
from parsec.backend import MockedUserVlobService
from parsec.session import AuthSession
from parsec.tools import to_jsonb64

from tests.common import can_side_effect_or_skip
from tests.backend.common import init_or_skiptest_parsec_postgresql


async def bootstrap_PostgreSQLUserVlobService(request, event_loop):
    can_side_effect_or_skip()
    module, url = await init_or_skiptest_parsec_postgresql()

    server = BaseServer()
    server.register_service(module.PostgreSQLService(url))
    svc = module.PostgreSQLUserVlobService()
    server.register_service(svc)
    await server.bootstrap_services()

    def finalize():
        event_loop.run_until_complete(server.teardown_services())

    request.addfinalizer(finalize)
    return svc


@pytest.fixture(params=[MockedUserVlobService,
                        bootstrap_PostgreSQLUserVlobService],
                ids=['mocked', 'postgresql'])
def user_vlob_svc(request, event_loop):
    if asyncio.iscoroutinefunction(request.param):
        return event_loop.run_until_complete(request.param(request, event_loop))
    else:
        return request.param()


@pytest.fixture
def session():
    return AuthSession(None, 'jdoe@test.com')


@pytest.fixture
def other_session():
    return AuthSession(None, 'bob@test.com')


class TestUserVlobServiceAPI:

    async def _push_version(self, user_vlob_svc, session, version, blob):
        msg = {'cmd': 'user_vlob_update', 'version': version, 'blob': blob}
        ret = await user_vlob_svc.dispatch_msg(msg, session=session)
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    async def test_read_not_existing(self, user_vlob_svc, session):
        ret = await user_vlob_svc.dispatch_msg({'cmd': 'user_vlob_read'}, session=session)
        assert ret == {'status': 'ok', 'version': 0, 'blob': to_jsonb64(b'')}

    @pytest.mark.asyncio
    async def test_update(self, user_vlob_svc, session):
        msg = {'cmd': 'user_vlob_update', 'version': 1, 'blob': to_jsonb64(b'next version')}
        ret = await user_vlob_svc.dispatch_msg(msg, session=session)
        assert ret == {'status': 'ok'}
        ret = await user_vlob_svc.dispatch_msg({'cmd': 'user_vlob_read'}, session=session)
        assert ret == {'status': 'ok', 'version': 1, 'blob': to_jsonb64(b'next version')}

    @pytest.mark.asyncio
    async def test_read(self, user_vlob_svc, session):
        await self._push_version(user_vlob_svc, session, 1, to_jsonb64(b'V1'))
        await self._push_version(user_vlob_svc, session, 2, to_jsonb64(b'V2'))
        ret = await user_vlob_svc.dispatch_msg({'cmd': 'user_vlob_read'}, session=session)
        assert ret == {'status': 'ok', 'version': 2, 'blob': to_jsonb64(b'V2')}
        ret = await user_vlob_svc.dispatch_msg({'cmd': 'user_vlob_read', 'version': 1},
                                               session=session)
        assert ret == {'status': 'ok', 'version': 1, 'blob': to_jsonb64(b'V1')}
        ret = await user_vlob_svc.dispatch_msg({'cmd': 'user_vlob_read', 'version': 0},
                                               session=session)
        assert ret == {'status': 'ok', 'version': 0, 'blob': to_jsonb64(b'')}

    @pytest.mark.asyncio
    async def test_bad_version_read(self, user_vlob_svc, session):
        await self._push_version(user_vlob_svc, session, 1, to_jsonb64(b'V1'))
        await self._push_version(user_vlob_svc, session, 2, to_jsonb64(b'V2'))
        ret = await user_vlob_svc.dispatch_msg({'cmd': 'user_vlob_read', 'version': 3},
                                                session=session)
        assert ret == {'status': 'user_vlob_error', 'label': 'Wrong blob version.'}
        ret = await user_vlob_svc.dispatch_msg({'cmd': 'user_vlob_read', 'version': -1},
                                               session=session)
        assert ret == {'status': 'bad_msg', 'label': {'version': ['Invalid value.']}}

    @pytest.mark.asyncio
    async def test_update_bad_version(self, user_vlob_svc, session):
        bad_version = 111
        msg = {'cmd': 'user_vlob_update', 'version': bad_version,
               'blob': to_jsonb64(b'Next version.')}
        ret = await user_vlob_svc.dispatch_msg(msg, session=session)
        assert ret['status'] == 'user_vlob_error'

    @pytest.mark.asyncio
    async def test_multiple_users(self, user_vlob_svc, session, other_session):
        await self._push_version(user_vlob_svc, session, 1, to_jsonb64(b"John's V1"))
        await self._push_version(user_vlob_svc, other_session, 1, to_jsonb64(b"Bob's V1"))
        await self._push_version(user_vlob_svc, session, 2, to_jsonb64(b"John's V2"))

        ret = await user_vlob_svc.dispatch_msg({'cmd': 'user_vlob_read'}, session=session)
        assert ret == {'status': 'ok', 'version': 2, 'blob': to_jsonb64(b"John's V2")}

        ret = await user_vlob_svc.dispatch_msg({'cmd': 'user_vlob_read'}, session=other_session)
        assert ret == {'status': 'ok', 'version': 1, 'blob': to_jsonb64(b"Bob's V1")}


# TODO: test event
# TODO: test can't listen other user's events

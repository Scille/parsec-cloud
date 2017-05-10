import pytest
import asyncio

from parsec.server import BaseServer
from parsec.backend import MockedNamedVlobService

from tests.common import can_side_effect_or_skip
from tests.backend.common import init_or_skiptest_parsec_postgresql


async def bootstrap_PostgreSQLNamedVlobService(request, event_loop):
    can_side_effect_or_skip()
    module, url = await init_or_skiptest_parsec_postgresql()

    server = BaseServer()
    server.register_service(module.PostgreSQLService(url))
    svc = module.PostgreSQLNamedVlobService()
    server.register_service(svc)
    await server.bootstrap_services()

    def finalize():
        event_loop.run_until_complete(server.teardown_services())

    request.addfinalizer(finalize)
    return svc


@pytest.fixture(params=[MockedNamedVlobService, bootstrap_PostgreSQLNamedVlobService, ], ids=['mocked', 'postgresql'])
def named_vlob_svc(request, event_loop):
    if asyncio.iscoroutinefunction(request.param):
        return event_loop.run_until_complete(request.param(request, event_loop))
    else:
        return request.param()


@pytest.fixture
def vlob(named_vlob_svc, event_loop, id='jdoe@test.com', blob='Initial commit.'):
    return event_loop.run_until_complete(named_vlob_svc.create(id=id, blob=blob))


class TestNamedVlobServiceAPI:

    @pytest.mark.asyncio
    @pytest.mark.parametrize("vlob_payload", [
        {'id': 'foo'},
        {'id': 'bar', 'blob': 'Initial commit.'}])
    async def test_create(self, named_vlob_svc, vlob_payload):
        ret = await named_vlob_svc.dispatch_msg({'cmd': 'named_vlob_create', **vlob_payload})
        assert ret['status'] == 'ok'
        assert ret['id']
        assert ret['read_trust_seed']
        assert ret['write_trust_seed']

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'named_vlob_create', 'id': 'id42', 'content': '...', 'bad_field': 'foo'},
        {'cmd': 'named_vlob_create', 'id': None, 'content': '...'},
        {'cmd': 'named_vlob_create', 'id': 42, 'content': '...'},
        {'cmd': 'named_vlob_create', 'content': '...'},
        {'cmd': 'named_vlob_create', 'id': 'id42', 'content': 42},
        {'cmd': 'named_vlob_create', 'id': 'id42', 'content': None},
        {'cmd': 'named_vlob_create', 'id': 'id42', 'content': '...', 'id': '1234567890'},
        {}])
    async def test_bad_msg_create(self, named_vlob_svc, bad_msg):
        ret = await named_vlob_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_read(self, named_vlob_svc, vlob):
        ret = await named_vlob_svc.dispatch_msg({
            'cmd': 'named_vlob_read',
            'id': vlob.id,
            'trust_seed': vlob.read_trust_seed,
        })
        assert ret == {
            'status': 'ok',
            'id': 'jdoe@test.com',
            'blob': vlob.blob,
            'version': vlob.version
        }

    @pytest.mark.asyncio
    async def test_update(self, named_vlob_svc, vlob):
        called_with = '<not called>'

        def on_event(*args):
            nonlocal called_with
            called_with = args

        blob = 'Next version.'
        named_vlob_svc.on_updated.connect(on_event, sender=vlob.id)
        ret = await named_vlob_svc.dispatch_msg({
            'cmd': 'named_vlob_update',
            'id': vlob.id,
            'trust_seed': vlob.write_trust_seed,
            'version': 2,
            'blob': blob
        })
        assert ret == {'status': 'ok'}
        assert called_with == (vlob.id, )
        v1 = await named_vlob_svc.read(vlob.id, 1)
        assert v1.blob == 'Initial commit.'
        v2 = await named_vlob_svc.read(vlob.id, 2)
        last = await named_vlob_svc.read(vlob.id)
        assert last.blob == v2.blob == 'Next version.'

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'named_vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': '...', 'bad_field': 'foo'},
        {'cmd': 'named_vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': None},
        {'cmd': 'named_vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': 42},
        {'cmd': 'named_vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>'},
        {'cmd': 'named_vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': None, 'blob': '...'},
        {'cmd': 'named_vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': -1, 'blob': '...'},
        {'cmd': 'named_vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': 'zero', 'blob': '...'},
        {'cmd': 'named_vlob_update', 'id': '<id-here>', 'trust_seed': 'dummy_seed',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'named_vlob_update', 'id': '<id-here>', 'trust_seed': None,
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'named_vlob_update', 'id': '<id-here>', 'trust_seed': 42,
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'named_vlob_update', 'id': '<id-here>',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'named_vlob_update', 'id': 42, 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'named_vlob_update', 'id': None, 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'named_vlob_update', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'named_vlob_update', 'id': 'dummy-id', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'named_vlob_update'}, {}])
    async def test_bad_msg_read(self, named_vlob_svc, bad_msg, vlob):
        if bad_msg.get('id') == '<id-here>':
            bad_msg['id'] = vlob.id
        if bad_msg.get('trust_seed') == '<trust_seed-here>':
            bad_msg['trust_seed'] = vlob.write_trust_seed
        if bad_msg.get('version') == '<version-here>':
            bad_msg['version'] = vlob.version
        ret = await named_vlob_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_read_bad_version(self, named_vlob_svc, vlob):
        bad_version = 111
        msg = {'cmd': 'named_vlob_read',
               'id': vlob.id,
               'trust_seed': vlob.read_trust_seed,
               'version': bad_version}
        ret = await named_vlob_svc.dispatch_msg(msg)
        assert ret['status'] == 'not_found'

    @pytest.mark.asyncio
    async def test_update_bad_version(self, named_vlob_svc, vlob):
        bad_version = 111
        msg = {'cmd': 'named_vlob_update', 'id': vlob.id, 'trust_seed': vlob.write_trust_seed,
               'version': bad_version, 'blob': 'Next version.'}
        ret = await named_vlob_svc.dispatch_msg(msg)
        assert ret['status'] == 'not_found'

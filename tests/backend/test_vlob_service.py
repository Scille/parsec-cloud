import pytest

from parsec.backend import MockedVlobService


@pytest.fixture(params=[MockedVlobService, ])
def vlob_svc(request):
    return request.param()


@pytest.fixture
def vlob(vlob_svc, event_loop, blob='Initial commit.'):
    return event_loop.run_until_complete(vlob_svc.create(blob=blob))


class TestVlobServiceAPI:

    @pytest.mark.asyncio
    @pytest.mark.parametrize("vlob_payload", [
        {},
        {'blob': 'Initial commit.'}])
    async def test_create(self, vlob_svc, vlob_payload):
        ret = await vlob_svc.dispatch_msg({'cmd': 'vlob_create', **vlob_payload})
        assert ret['status'] == 'ok'
        assert ret['id']
        assert ret['read_trust_seed']
        assert ret['write_trust_seed']

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'vlob_create', 'content': '...', 'bad_field': 'foo'},
        {'cmd': 'vlob_create', 'content': 42},
        {'cmd': 'vlob_create', 'content': None},
        {'cmd': 'vlob_create', 'content': '...', 'id': '1234567890'},
        {}])
    async def test_bad_msg_create(self, vlob_svc, bad_msg):
        ret = await vlob_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_read_not_found(self, vlob_svc, vlob):
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'vlob_read',
            'id': '1234',
            'trust_seed': vlob.read_trust_seed,
        })
        assert ret == {'status': 'not_found', 'label': 'Cannot find vlob.'}

    @pytest.mark.asyncio
    async def test_read(self, vlob_svc, vlob):
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'vlob_read',
            'id': vlob.id,
            'trust_seed': vlob.read_trust_seed,
        })
        assert ret == {
            'status': 'ok',
            'blob': vlob.blob_versions[-1],
            'version': len(vlob.blob_versions)
        }

    @pytest.mark.asyncio
    async def test_update_not_found(self, vlob_svc, vlob):
        blob = 'Next version.'
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'vlob_update',
            'id': '1234',
            'trust_seed': vlob.write_trust_seed,
            'version': 2,
            'blob': blob
        })
        assert ret == {'status': 'not_found', 'label': 'Cannot find vlob.'}

    @pytest.mark.asyncio
    async def test_update(self, vlob_svc, vlob):
        called_with = '<not called>'

        def on_event(*args):
            nonlocal called_with
            called_with = args

        blob = 'Next version.'
        vlob_svc.on_updated.connect(on_event, sender=vlob.id)
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'vlob_update',
            'id': vlob.id,
            'trust_seed': vlob.write_trust_seed,
            'version': 2,
            'blob': blob
        })
        assert ret == {'status': 'ok'}
        assert vlob.blob_versions == ['Initial commit.', 'Next version.']
        assert called_with == (vlob.id, )

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': '...', 'bad_field': 'foo'},
        {'cmd': 'vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': None},
        {'cmd': 'vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': 42},
        {'cmd': 'vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>'},
        {'cmd': 'vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': None, 'blob': '...'},
        {'cmd': 'vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': -1, 'blob': '...'},
        {'cmd': 'vlob_update', 'id': '<id-here>', 'trust_seed': '<trust_seed-here>',
         'version': 'zero', 'blob': '...'},
        {'cmd': 'vlob_update', 'id': '<id-here>', 'trust_seed': 'dummy_seed',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'vlob_update', 'id': '<id-here>', 'trust_seed': None,
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'vlob_update', 'id': '<id-here>', 'trust_seed': 42,
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'vlob_update', 'id': '<id-here>',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'vlob_update', 'id': 42, 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'vlob_update', 'id': None, 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'vlob_update', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'vlob_update', 'id': 'dummy-id', 'trust_seed': '<trust_seed-here>',
         'version': '<version-here>', 'blob': '...'},
        {'cmd': 'vlob_update'}, {}])
    async def test_bad_msg_read(self, vlob_svc, bad_msg, vlob):
        if bad_msg.get('id') == '<id-here>':
            bad_msg['id'] = vlob.id
        if bad_msg.get('trust_seed') == '<trust_seed-here>':
            bad_msg['trust_seed'] = vlob.write_trust_seed
        if bad_msg.get('version') == '<version-here>':
            bad_msg['version'] = len(vlob.blob_versions)
        ret = await vlob_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_read_bad_version(self, vlob_svc, vlob):
        bad_version = 111
        msg = {'cmd': 'vlob_read',
               'id': vlob.id,
               'trust_seed': vlob.read_trust_seed,
               'version': bad_version}
        ret = await vlob_svc.dispatch_msg(msg)
        assert ret['status'] == 'bad_version'

    @pytest.mark.asyncio
    async def test_update_bad_version(self, vlob_svc, vlob):
        bad_version = 111
        msg = {'cmd': 'vlob_update', 'id': vlob.id, 'trust_seed': vlob.write_trust_seed,
               'version': bad_version, 'blob': 'Next version.'}
        ret = await vlob_svc.dispatch_msg(msg)
        assert ret['status'] == 'bad_version'

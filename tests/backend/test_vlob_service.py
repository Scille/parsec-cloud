import pytest

from parsec.backend import VlobService


@pytest.fixture
def vlob_svc():
    return VlobService()


@pytest.fixture
def vlob(vlob_svc, event_loop, id=None, blob='Initial commit.'):
    return event_loop.run_until_complete(vlob_svc.create(id=id, blob=blob))


class TestVlobServiceAPI:

    @pytest.mark.xfail
    @pytest.mark.asyncio
    @pytest.mark.parametrize("vlob_payload", [
        {},
        {'blob': 'Initial commit.'},
        {'id': 'named-vlob-id0'},
        {'id': 'named-vlob-id0', 'blob': 'Initial commit.'}])
    async def test_create(self, vlob_svc, vlob_payload):
        ret = await vlob_svc.dispatch_msg({'cmd': 'create', **vlob_payload})
        assert ret['status'] == 'ok'
        if 'id' in vlob_payload:
            assert ret['id'] == vlob_payload['id']
        else:
            assert ret['id']
        assert ret['read_trust_seed']
        assert ret['write_trust_seed']

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_read(self, vlob_svc, vlob):
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'read',
            'id': vlob.id,
            'trust_seed': vlob.read_trust_seed,
        })
        assert ret == {
            'status': 'ok',
            'content': vlob.blob_versions[-1],
            'version': len(vlob.blob_versions)
        }

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_update(self, vlob_svc, vlob):
        called_with = '<not called>'

        def on_event(*args):
            nonlocal called_with
            called_with = args

        blob = 'Next version.'
        vlob_svc.on_vlob_updated.connect(on_event, sender=vlob.id)
        ret = await vlob_svc.dispatch_msg({
            'cmd': 'update',
            'id': vlob.id,
            'trust_seed': vlob.write_trust_seed,
            'version': 2,
            'blob': blob
        })
        assert ret == {'status': 'ok'}
        assert vlob.blob_versions == ['Initial commit.', 'Next version.']
        assert called_with == (vlob.id, )

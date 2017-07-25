import pytest
import asyncio
from effect2.testing import const, conste, noop, perform_sequence, asyncio_perform_sequence

from parsec.base import EEvent
from parsec.backend.backend_api import execute_cmd
from parsec.backend.vlob import EVlobCreate, EVlobRead, EVlobUpdate, VlobAtom, MockedVlobComponent
from parsec.exceptions import VlobNotFound, TrustSeedError
from parsec.tools import to_jsonb64

from tests.common import can_side_effect_or_skip
from tests.backend.common import init_or_skiptest_parsec_postgresql


async def bootstrap_PostgreSQLVlobComponent(request, event_loop):
    can_side_effect_or_skip()
    module, url = await init_or_skiptest_parsec_postgresql(event_loop)

    conn = module.PostgreSQLConnection(url)
    await conn.open_connection(event_loop)

    def finalize():
        event_loop.run_until_complete(conn.close_connection())

    request.addfinalizer(finalize)
    return module.PostgreSQLVlobComponent(conn)


@pytest.fixture(params=[MockedVlobComponent, bootstrap_PostgreSQLVlobComponent],
                ids=['mocked', 'postgresql'])
def component(request, event_loop):
    if asyncio.iscoroutinefunction(request.param):
        return event_loop.run_until_complete(request.param(request, event_loop))
    else:
        return request.param()


@pytest.fixture
def vlob(component, event_loop):
    intent = EVlobCreate('123', b'foo')
    eff = component.perform_vlob_create(intent)
    return event_loop.run_until_complete(asyncio_perform_sequence([], eff))


class TestVlobComponent:
    @pytest.mark.asyncio
    async def test_vlob_create(self, component):
        intent = EVlobCreate('123', b'foo')
        eff = component.perform_vlob_create(intent)
        sequence = [
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert isinstance(ret, VlobAtom)
        assert ret.id == '123'
        assert ret.read_trust_seed
        assert ret.write_trust_seed
        assert ret.version == 1

    @pytest.mark.asyncio
    async def test_vlob_create_autoid(self, component):
        intent = EVlobCreate(blob=b'foo')
        eff = component.perform_vlob_create(intent)
        sequence = [
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert isinstance(ret, VlobAtom)
        assert ret.id
        assert ret.read_trust_seed
        assert ret.write_trust_seed
        assert ret.version == 1

    @pytest.mark.asyncio
    async def test_vlob_read_ok(self, component, vlob):
        intent = EVlobRead(vlob.id, vlob.read_trust_seed)
        eff = component.perform_vlob_read(intent)
        sequence = [
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret == vlob

    @pytest.mark.asyncio
    async def test_vlob_read_previous_version(self, component, vlob):
        # Update vlob
        intent = EVlobUpdate(vlob.id, 2, vlob.write_trust_seed, b'Next version.')
        eff = component.perform_vlob_update(intent)
        sequence = [
            (EEvent('vlob_updated', vlob.id), noop)
        ]
        await asyncio_perform_sequence(sequence, eff)
        # Read previous version
        intent = EVlobRead(vlob.id, vlob.read_trust_seed, version=1)
        eff = component.perform_vlob_read(intent)
        sequence = [
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret == vlob

    @pytest.mark.asyncio
    async def test_vlob_read_missing(self, component, vlob):
        intent = EVlobRead('dummy-id', vlob.read_trust_seed)
        with pytest.raises(VlobNotFound):
            eff = component.perform_vlob_read(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)

    @pytest.mark.asyncio
    async def test_vlob_read_wrong_seed(self, component, vlob):
        intent = EVlobRead(vlob.id, 'dummy-seed')
        with pytest.raises(TrustSeedError):
            eff = component.perform_vlob_read(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)
        intent = EVlobRead(vlob.id, vlob.write_trust_seed)
        with pytest.raises(TrustSeedError):
            eff = component.perform_vlob_read(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)

    @pytest.mark.asyncio
    async def test_vlob_read_wrong_version(self, component, vlob):
        intent = EVlobRead(vlob.id, vlob.read_trust_seed, version=42)
        with pytest.raises(VlobNotFound):
            eff = component.perform_vlob_read(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)

    @pytest.mark.asyncio
    async def test_vlob_update_ok(self, component, vlob):
        intent = EVlobUpdate(vlob.id, 2, vlob.write_trust_seed, b'Next version.')
        eff = component.perform_vlob_update(intent)
        sequence = [
            (EEvent('vlob_updated', vlob.id), noop)
        ]
        await asyncio_perform_sequence(sequence, eff)
        # Check back the value
        intent = EVlobRead(vlob.id, vlob.read_trust_seed, version=2)
        eff = component.perform_vlob_read(intent)
        ret = await asyncio_perform_sequence([], eff)
        assert ret.id == vlob.id
        assert ret.version == 2
        assert ret.blob == b'Next version.'

    @pytest.mark.asyncio
    async def test_vlob_update_missing(self, component, vlob):
        intent = EVlobUpdate('dummy-id', 2, vlob.read_trust_seed, b'Next version.')
        with pytest.raises(VlobNotFound):
            eff = component.perform_vlob_update(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)

    @pytest.mark.asyncio
    async def test_vlob_update_wrong_seed(self, component, vlob):
        intent = EVlobUpdate(vlob.id, 2, 'dummy-seed', b'Next version.')
        with pytest.raises(TrustSeedError):
            eff = component.perform_vlob_update(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)
        intent = EVlobUpdate(vlob.id, 2, vlob.read_trust_seed, b'Next version.')
        with pytest.raises(TrustSeedError):
            eff = component.perform_vlob_update(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)

    @pytest.mark.asyncio
    async def test_vlob_update_wrong_version(self, component, vlob):
        intent = EVlobUpdate(vlob.id, 42, vlob.write_trust_seed, b'Next version.')
        with pytest.raises(VlobNotFound):
            eff = component.perform_vlob_update(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)


class TestVlobAPI:

    @pytest.mark.parametrize("id_and_blob", [
        (None, None),
        (None, b'Initial commit.'),
        ('foo', None),
        ('bar', b'Initial commit.')
    ], ids=lambda x: 'id=%s, blob=%s' % x)
    def test_vlob_create_ok(self, id_and_blob):
        id, blob = id_and_blob
        intent = EVlobCreate(id, blob)
        intent_ret = VlobAtom(id, 'readtrustseed-123', 'writetrustseed-123', blob, 1)
        payload = {}
        if id:
            payload['id'] = id
        else:
            intent_ret.id = '123'
        if blob:
            payload['blob'] = to_jsonb64(blob)
        else:
            intent.blob = b''
            intent_ret.blob = b''
        eff = execute_cmd('vlob_create', payload)
        sequence = [
            (intent, const(intent_ret)),
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {
            'status': 'ok',
            'id': intent_ret.id,
            'read_trust_seed': 'readtrustseed-123',
            'write_trust_seed': 'writetrustseed-123'
        }

    @pytest.mark.parametrize('bad_msg', [
        {'blob': to_jsonb64(b'...'), 'bad_field': 'foo'},
        {'blob': 42},
        {'blob': None},
        {'id': 42, 'blob': to_jsonb64(b'...')},
        {'id': '', 'blob': to_jsonb64(b'...')},  # Id is 1 long min
        {'id': 'X' * 33, 'blob': to_jsonb64(b'...')},  # Id is 32 long max
    ])
    def test_vlob_create_bad_msg(self, bad_msg):
        eff = execute_cmd('vlob_create', bad_msg)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'bad_msg'

    def test_vlob_read_not_found(self):
        eff = execute_cmd('vlob_read', {'id': '1234', 'trust_seed': 'TS4242'})
        sequence = [
            (EVlobRead('1234', 'TS4242'), conste(VlobNotFound('Vlob not found.')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {'status': 'vlob_not_found', 'label': 'Vlob not found.'}

    def test_vlob_read_ok(self):
        eff = execute_cmd('vlob_read', {'id': '1234', 'trust_seed': 'TS4242'})
        sequence = [
            (EVlobRead('1234', 'TS4242'),
                const(VlobAtom('1234', 'TS4242', 'WTS4242', b'content v42', 42)))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {
            'status': 'ok',
            'id': '1234',
            'blob': to_jsonb64(b'content v42'),
            'version': 42
        }

    @pytest.mark.parametrize('bad_msg', [
        {'id': '1234', 'trust_seed': 'TS4242', 'bad_field': 'foo'},
        {'id': '1234'},
        {'id': '1234', 'trust_seed': 42},
        {'id': '1234', 'trust_seed': None},
        {'id': 42, 'trust_seed': 'TS4242'},
        {'id': None, 'trust_seed': 'TS4242'},
        # {'id': '1234567890', 'trust_seed': 'TS4242'},  # TODO bad?
        {}
    ])
    def test_vlob_read_bad_msg(self, bad_msg):
        eff = execute_cmd('vlob_read', bad_msg)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'bad_msg'

    def test_read_bad_version(self):
        msg = {
            'id': '123',
            'trust_seed': 'TS42',
            'version': 2
        }
        eff = execute_cmd('vlob_read', msg)
        sequence = [
            (EVlobRead('123', 'TS42', 2),
                conste(VlobNotFound('Vlob not found.')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'vlob_not_found'

    def test_vlob_update_ok(self):
        blob = to_jsonb64(b'Next version.')
        eff = execute_cmd('vlob_update',
            {'id': '1234', 'trust_seed': 'WTS4242', 'version': 2, 'blob': blob})
        sequence = [
            (EVlobUpdate('1234', 2, 'WTS4242', b'Next version.'), noop)
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {'status': 'ok'}

    def test_vlob_update_not_found(self):
        blob = to_jsonb64(b'Next version.')
        eff = execute_cmd('vlob_update',
            {'id': '1234', 'trust_seed': 'WTS4242', 'version': 2, 'blob': blob})
        sequence = [
            (EVlobUpdate('1234', 2, 'WTS4242', b'Next version.'),
                conste(VlobNotFound('Vlob not found.')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {'status': 'vlob_not_found', 'label': 'Vlob not found.'}

    @pytest.mark.parametrize('bad_msg', [
        {'id': '1234', 'trust_seed': 'WTS42',
         'version': '42', 'blob': to_jsonb64(b'...'), 'bad_field': 'foo'},
        {'id': '1234', 'trust_seed': 'WTS42',
         'version': '42', 'blob': None},
        {'id': '1234', 'trust_seed': 'WTS42',
         'version': '42', 'blob': 42},
        {'id': '1234', 'trust_seed': 'WTS42',
         'version': '42'},
        {'id': '1234', 'trust_seed': 'WTS42',
         'version': None, 'blob': to_jsonb64(b'...')},
        {'id': '1234', 'trust_seed': 'WTS42',
         'version': -1, 'blob': to_jsonb64(b'...')},
        {'id': '1234', 'trust_seed': None,
         'version': '42', 'blob': to_jsonb64(b'...')},
        {'id': '1234', 'trust_seed': 42,
         'version': '42', 'blob': to_jsonb64(b'...')},
        {'id': '1234',
         'version': '42', 'blob': to_jsonb64(b'...')},
        {'id': 42, 'trust_seed': 'WTS42',
         'version': '42', 'blob': to_jsonb64(b'...')},
        {'id': None, 'trust_seed': 'WTS42',
         'version': '42', 'blob': to_jsonb64(b'...')},
        {'trust_seed': 'WTS42',
         'version': '42', 'blob': to_jsonb64(b'...')},
        {}
    ])
    def test_vlob_update_bad_msg(self, bad_msg):
        eff = execute_cmd('vlob_update', bad_msg)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'bad_msg'

    def test_update_bad_version(self):
        msg = {
            'id': '123',
            'trust_seed': 'WTS42',
            'version': 2,
            'blob': to_jsonb64(b'Next version.')
        }
        eff = execute_cmd('vlob_update', msg)
        sequence = [
            (EVlobUpdate('123', 2, 'WTS42', b'Next version.'),
                conste(VlobNotFound('Vlob not found.')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'vlob_not_found'

    def test_update_bad_seed(self):
        msg = {
            'id': '123',
            'trust_seed': 'dummy_seed',
            'version': 2,
            'blob': to_jsonb64(b'Next version.')
        }
        eff = execute_cmd('vlob_update', msg)
        sequence = [
            (EVlobUpdate('123', 2, 'dummy_seed', b'Next version.'),
                conste(TrustSeedError('Bad trust seed.')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'trust_seed_error'

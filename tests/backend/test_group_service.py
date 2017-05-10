import copy
import pytest
import asyncio

from parsec.server import BaseServer
from parsec.backend import MockedGroupService

from tests.common import can_side_effect_or_skip
from tests.backend.common import init_or_skiptest_parsec_postgresql


async def bootstrap_PostgreSQLGroupService(request, event_loop):
    can_side_effect_or_skip()
    module, url = await init_or_skiptest_parsec_postgresql()

    server = BaseServer()
    server.register_service(module.PostgreSQLService(url))
    svc = module.PostgreSQLGroupService(url)
    server.register_service(svc)
    await server.bootstrap_services()

    def finalize():
        event_loop.run_until_complete(server.teardown_services())

    request.addfinalizer(finalize)
    return svc


@pytest.fixture(params=[MockedGroupService, bootstrap_PostgreSQLGroupService, ], ids=['mocked', 'postgresql'])
def group_svc(request, event_loop):
    if asyncio.iscoroutinefunction(request.param):
        return event_loop.run_until_complete(request.param(request, event_loop))
    else:
        return request.param()


@pytest.fixture
def group(group_svc, event_loop, name='foo_communiy'):
    event_loop.run_until_complete(group_svc.create(name=name))
    result = event_loop.run_until_complete(group_svc.read(name=name))
    result.update({'name': name})
    return result


class TestGroupServiceAPI:

    @pytest.mark.asyncio
    @pytest.mark.parametrize('group_payload', [
        {'name': 'foo_community'},
        {'name': 'Foo community'}])
    async def test_create(self, group_svc, group_payload):
        # Working
        ret = await group_svc.dispatch_msg({'cmd': 'group_create', **group_payload})
        assert ret == {'status': 'ok'}
        # Already exist
        ret = await group_svc.dispatch_msg({'cmd': 'group_create', **group_payload})
        assert ret == {'status': 'already_exist', 'label': 'Group already exist.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'group_create', 'name': 'foo', 'bad_field': 'foo'},
        {'cmd': 'group_create', 'name': 42},
        {'cmd': 'group_create', 'name': None},
        {}])
    async def test_bad_msg_create(self, group_svc, bad_msg):
        ret = await group_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_read_not_found(self, group_svc, group):
        ret = await group_svc.dispatch_msg({'cmd': 'group_read', 'name': 'unknown'})
        assert ret == {'status': 'not_found', 'label': 'Group not found.'}

    @pytest.mark.asyncio
    async def test_read(self, group_svc, group):
        ret = await group_svc.dispatch_msg({'cmd': 'group_read', 'name': group['name']})
        assert ret == {'status': 'ok', 'admins': group['admins'], 'users': group['users']}

    @pytest.mark.asyncio
    async def test_add_identities_not_found(self, group_svc, group):
        ret = await group_svc.dispatch_msg({'cmd': 'group_add_identities',
                                            'name': 'unknown',
                                            'identities': group['users']})
        assert ret == {'status': 'not_found', 'label': 'Group not found.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('admin', [True, False])
    @pytest.mark.parametrize('identities', [
        [],
        ['id-1', 'id-2']])
    async def test_add_identities(self, group_svc, group, identities, admin):
        origin_group = copy.deepcopy(group)
        # Adding duplicates identities should not raise errors
        for i in range(0, 2):
            ret = await group_svc.dispatch_msg({'cmd': 'group_add_identities',
                                                'name': origin_group['name'],
                                                'identities': identities,
                                                'admin': admin})
            ret = await group_svc.dispatch_msg({'cmd': 'group_read', 'name': origin_group['name']})
            ret['users'] = sorted(ret['users'])
            ret['admins'] = sorted(ret['admins'])
            if admin:
                assert ret == {'status': 'ok',
                               'admins': sorted(identities + origin_group['admins']),
                               'users': origin_group['users']}
            else:
                assert ret == {'status': 'ok',
                               'admins': origin_group['admins'],
                               'users': sorted(identities + origin_group['users'])}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'group_add_identities', 'name': '<name-here>', 'identities': ['id'],
         'bad_field': 'foo'},
        {'cmd': 'group_add_identities', 'name': '<name-here>', 'identities': ['id'],
         'admin': 42},
        {'cmd': 'group_add_identities', 'name': 42, 'identities': ['id']},
        {'cmd': 'group_add_identities', 'name': None, 'identities': ['id']},
        {'cmd': 'group_add_identities', 'name': '<name-here>', 'identities': 'id'},
        {'cmd': 'group_add_identities', 'name': '<name-here>', 'identities': 42},
        {'cmd': 'group_add_identities', 'name': '<name-here>', 'identities': None},
        {'cmd': 'group_add_identities', 'identities': ['id']},
        {'cmd': 'group_add_identities', 'name': '<name-here>'},
        {'cmd': 'group_add_identities'}, {}])
    async def test_bad_add_identities(self, group_svc, bad_msg, group):
        if bad_msg.get('name') == '<name-here>':
            bad_msg['name'] = group['name']
        ret = await group_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_remove_identities_not_found(self, group_svc, group):
        ret = await group_svc.dispatch_msg({'cmd': 'group_remove_identities',
                                            'name': 'unknown',
                                            'identities': group['users']})
        assert ret == {'status': 'not_found', 'label': 'Group not found.'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('admin', [True, False])
    @pytest.mark.parametrize('identities', [
        [],
        ['id-1', 'id-2']])
    async def test_remove_identities(self, group_svc, group, identities, admin):
        origin_group = copy.deepcopy(group)
        # Initial identity that should not be erased further
        ret = await group_svc.dispatch_msg({'cmd': 'group_add_identities',
                                            'name': origin_group['name'],
                                            'identities': identities,
                                            'admin': admin})
        assert ret['status'] == 'ok'
        # Removing non-existant identities should not raise errors
        for i in range(0, 2):
            ret = await group_svc.dispatch_msg({'cmd': 'group_remove_identities',
                                                'name': origin_group['name'],
                                                'identities': identities,
                                                'admin': admin})
            ret = await group_svc.dispatch_msg({'cmd': 'group_read', 'name': origin_group['name']})
            ret['users'] = sorted(ret['users'])
            ret['admins'] = sorted(ret['admins'])
            if admin:
                assert ret == {'status': 'ok',
                               'admins': origin_group['admins'],
                               'users': origin_group['users']}
            else:
                assert ret == {'status': 'ok',
                               'admins': origin_group['admins'],
                               'users': origin_group['users']}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'group_remove_identities', 'name': '<name-here>', 'identities': ['id'],
         'bad_field': 'foo'},
        {'cmd': 'group_remove_identities', 'name': '<name-here>', 'identities': ['id'],
         'admin': 42},
        {'cmd': 'group_remove_identities', 'name': 42, 'identities': ['id']},
        {'cmd': 'group_remove_identities', 'name': None, 'identities': ['id']},
        {'cmd': 'group_remove_identities', 'name': '<name-here>', 'identities': 'id'},
        {'cmd': 'group_remove_identities', 'name': '<name-here>', 'identities': 42},
        {'cmd': 'group_remove_identities', 'name': '<name-here>', 'identities': None},
        {'cmd': 'group_remove_identities', 'identities': ['id']},
        {'cmd': 'group_remove_identities', 'name': '<name-here>'},
        {'cmd': 'group_remove_identities'}, {}])
    async def test_bad_remove_identities(self, group_svc, bad_msg, group):
        if bad_msg.get('name') == '<name-here>':
            bad_msg['name'] = group['name']
        ret = await group_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

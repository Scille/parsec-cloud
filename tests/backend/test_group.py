import pytest
import asyncio
from effect2.testing import const, conste, noop, perform_sequence, asyncio_perform_sequence

from parsec.backend.backend_api import execute_cmd
from parsec.backend.group import (
    EGroupCreate, EGroupRead, EGroupAddIdentities, EGroupRemoveIdentities,
    Group, MockedGroupComponent
)
from parsec.exceptions import GroupError, GroupAlreadyExist, GroupNotFound

from tests.common import can_side_effect_or_skip
from tests.backend.common import init_or_skiptest_parsec_postgresql


async def bootstrap_PostgreSQLGroupComponent(request, loop):
    can_side_effect_or_skip()
    module, url = await init_or_skiptest_parsec_postgresql()

    conn = module.PostgreSQLConnection(url)
    await conn.open_connection()

    def finalize():
        loop.run_until_complete(conn.close_connection())

    request.addfinalizer(finalize)
    return module.PostgreSQLGroupComponent(conn)


@pytest.fixture(params=[MockedGroupComponent, bootstrap_PostgreSQLGroupComponent],
                ids=['mocked', 'postgresql'])
def component(request, loop):
    if asyncio.iscoroutinefunction(request.param):
        return loop.run_until_complete(request.param(request, loop))
    else:
        return request.param()


@pytest.fixture
def group(loop, component, name='super adventure club'):
    intent = EGroupCreate(name)
    eff = component.perform_group_create(intent)
    loop.run_until_complete(asyncio_perform_sequence([], eff))
    return name


async def read_group(component, group_name):
    intent = EGroupRead(group_name)
    eff = component.perform_group_read(intent)
    return await asyncio_perform_sequence([], eff)


async def add_identities_group(component, group_name, identities, admin=False):
    identities = set(identities)
    intent = EGroupAddIdentities(group_name, identities, admin)
    eff = component.perform_group_add_identities(intent)
    await asyncio_perform_sequence([], eff)


async def remove_identities_group(component, group_name, identities, admin=False):
    identities = set(identities)
    intent = EGroupRemoveIdentities(group_name, identities, admin)
    eff = component.perform_group_remove_identities(intent)
    await asyncio_perform_sequence([], eff)


class TestGroupComponent:
    async def test_group_create(self, component):
        intent = EGroupCreate('super adventure club')
        eff = component.perform_group_create(intent)
        sequence = [
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret is None

    async def test_group_create_already_exist(self, component, group):
        intent = EGroupCreate(group)
        with pytest.raises(GroupAlreadyExist):
            eff = component.perform_group_create(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)

    async def test_group_read(self, component, group):
        intent = EGroupRead(group)
        eff = component.perform_group_read(intent)
        sequence = [
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret == Group(group)

    async def test_group_read_unknown(self, component):
        intent = EGroupRead('dummy group')
        with pytest.raises(GroupError):
            eff = component.perform_group_read(intent)
            sequence = [
            ]
            await asyncio_perform_sequence(sequence, eff)

    async def test_group_add_identity(self, component, group):
        await add_identities_group(component, group, {'zack@test.com'}, False)
        # Add admins as well
        await add_identities_group(component, group, {'alice@test.com', 'bob@test.com'}, True)
        res = await read_group(component, group)
        assert res == Group(group, admins={'alice@test.com', 'bob@test.com'},
                            users={'zack@test.com'})

    async def test_unknown_group_add_identity(self, component):
        with pytest.raises(GroupNotFound):
            intent = EGroupAddIdentities('dummy-group', {'alice@test.com'}, False)
            eff = component.perform_group_add_identities(intent)
            await asyncio_perform_sequence([], eff)

    async def test_unknown_group_remove_identity(self, component):
        with pytest.raises(GroupNotFound):
            intent = EGroupRemoveIdentities('dummy-group', {'alice@test.com'}, False)
            eff = component.perform_group_remove_identities(intent)
            await asyncio_perform_sequence([], eff)

    async def test_group_remove_identity(self, component, group):
        await add_identities_group(component, group,
            {'user1@test.com', 'user2@test.com', 'user3@test.com'}, False)
        await add_identities_group(component, group, {'adminA@test.com', 'adminB@test.com'}, True)
        # Remove some users
        await remove_identities_group(component, group, {'user1@test.com', 'user3@test.com'}, False)
        await remove_identities_group(component, group, {'adminB@test.com'}, True)
        # Removing in wrong category does nothing
        await remove_identities_group(component, group, {'user2@test.com'}, True)
        await remove_identities_group(component, group, {'adminA@test.com'}, False)
        # Check the result
        res = await read_group(component, group)
        assert res == Group(group, admins={'adminA@test.com'}, users={'user2@test.com'})


class TestGroupAPI:

    def test_group_create_ok(self):
        eff = execute_cmd('group_create', {'name': 'super adventure club'})
        sequence = [
            (EGroupCreate('super adventure club'), noop)
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {'status': 'ok'}

    def test_group_create_already_exist(self):
        eff = execute_cmd('group_create', {'name': 'super adventure club'})
        sequence = [
            (EGroupCreate('super adventure club'),
                conste(GroupAlreadyExist('Group already exist.')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'group_already_exists'

    @pytest.mark.parametrize('bad_msg', [
        {'name': 'foo', 'bad_field': 'foo'},
        {'name': 42},
        {'name': None},
        {}
    ])
    def test_bad_msg_create(self, bad_msg):
        eff = execute_cmd('group_create', bad_msg)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.parametrize('bad_msg', [
        {'name': 'foo', 'bad_field': 'foo'},
        {'name': 42},
        {'name': None},
        {}
    ])
    def test_bad_msg_read(self, bad_msg):
        eff = execute_cmd('group_read', bad_msg)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'bad_msg'

    def test_group_read_ok(self):
        eff = execute_cmd('group_read', {'name': 'super adventure club'})
        sequence = [
            (EGroupRead('super adventure club'),
                const(Group('super adventure club', {'admin@test.com'}, {'user1@test.com'})))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {
            'status': 'ok',
            'name': 'super adventure club',
            'admins': ['admin@test.com'],
            'users': ['user1@test.com']
        }

    def test_group_read_unknown(self):
        eff = execute_cmd('group_read', {'name': 'dummy-group'})
        sequence = [
            (EGroupRead('dummy-group'), conste(GroupNotFound('Group not found.')))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'group_not_found'

    def test_group_remove_identities_ok(self):
        eff = execute_cmd('group_remove_identities',
            {'name': 'super adventure club', 'identities': ['alice@test.com'], 'admin': True})
        sequence = [
            (EGroupRemoveIdentities('super adventure club', {'alice@test.com'}, True), noop)
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {'status': 'ok'}

    @pytest.mark.parametrize('bad_msg', [
        {'name': 'grp1', 'identities': ['alice'], 'bad_field': 'foo'},
        {'name': 'grp1', 'identities': ['alice'], 'admin': 42},
        {'name': 42, 'identities': ['alice']},
        {'name': None, 'identities': ['alice']},
        {'name': 'grp1', 'identities': 'alice'},
        {'name': 'grp1', 'identities': 42},
        {'name': 'grp1', 'identities': None},
        {'identities': ['alice']},
        {'name': 'grp1'},
        {}
    ])
    def test_group_remove_identities_bad_msg(self, bad_msg):
        eff = execute_cmd('group_remove_identities', bad_msg)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'bad_msg'

    def test_group_add_identities_ok(self):
        eff = execute_cmd('group_add_identities',
            {'name': 'super adventure club', 'identities': ['alice@test.com'], 'admin': True})
        sequence = [
            (EGroupAddIdentities('super adventure club', {'alice@test.com'}, True), noop)
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {'status': 'ok'}

    @pytest.mark.parametrize('bad_msg', [
        {'name': 'grp1', 'identities': ['alice'], 'bad_field': 'foo'},
        {'name': 'grp1', 'identities': ['alice'], 'admin': 42},
        {'name': 42, 'identities': ['alice']},
        {'name': None, 'identities': ['alice']},
        {'name': 'grp1', 'identities': 'alice'},
        {'name': 'grp1', 'identities': 42},
        {'name': 'grp1', 'identities': None},
        {'identities': ['alice']},
        {'name': 'grp1'},
        {}
    ])
    def test_group_add_identities_bad_msg(self, bad_msg):
        eff = execute_cmd('group_add_identities', bad_msg)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'bad_msg'

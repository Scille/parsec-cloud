import pytest
from effect2.testing import const, perform_sequence, raise_, noop

from parsec.backend.backend_api import execute_cmd
from parsec.backend.group import (
    EGroupCreate, EGroupRead, EGroupAddIdentities, EGroupRemoveIdentities,
    Group, MockedGroupComponent
)
from parsec.exceptions import GroupError, GroupAlreadyExist, GroupNotFound


@pytest.fixture
def component():
    return MockedGroupComponent()


@pytest.fixture
def group(component, name='super adventure club'):
    intent = EGroupCreate(name)
    eff = component.perform_group_create(intent)
    perform_sequence([], eff)
    return name


def read_group(component, group_name):
    intent = EGroupRead(group_name)
    eff = component.perform_group_read(intent)
    return perform_sequence([], eff)


def add_identities_group(component, group_name, identities, admin=False):
    identities = set(identities)
    intent = EGroupAddIdentities(group_name, identities, admin)
    eff = component.perform_group_add_identities(intent)
    perform_sequence([], eff)


def remove_identities_group(component, group_name, identities, admin=False):
    identities = set(identities)
    intent = EGroupRemoveIdentities(group_name, identities, admin)
    eff = component.perform_group_remove_identities(intent)
    perform_sequence([], eff)


class TestGroupComponent:
    def test_group_create(self, component):
        intent = EGroupCreate('super adventure club')
        eff = component.perform_group_create(intent)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret is None

    def test_group_create_already_exist(self, component, group):
        intent = EGroupCreate(group)
        with pytest.raises(GroupAlreadyExist):
            eff = component.perform_group_create(intent)
            sequence = [
            ]
            perform_sequence(sequence, eff)

    def test_group_read(self, component, group):
        intent = EGroupRead(group)
        eff = component.perform_group_read(intent)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == Group(group)

    def test_group_read_unknown(self, component):
        intent = EGroupRead('dummy group')
        with pytest.raises(GroupError):
            eff = component.perform_group_read(intent)
            sequence = [
            ]
            perform_sequence(sequence, eff)

    def test_group_add_identity(self, component, group):
        add_identities_group(component, group, {'zack@test.com'}, False)
        # Add admins as well
        add_identities_group(component, group, {'alice@test.com', 'bob@test.com'}, True)
        assert read_group(component, group) == Group(
            group, admins={'alice@test.com', 'bob@test.com'}, users={'zack@test.com'})

    def test_unknown_group_add_identity(self, component):
        with pytest.raises(GroupNotFound):
            intent = EGroupAddIdentities('dummy-group', {'alice@test.com'}, False)
            eff = component.perform_group_add_identities(intent)
            perform_sequence([], eff)

    def test_unknown_group_remove_identity(self, component):
        with pytest.raises(GroupNotFound):
            intent = EGroupRemoveIdentities('dummy-group', {'alice@test.com'}, False)
            eff = component.perform_group_remove_identities(intent)
            perform_sequence([], eff)

    def test_group_remove_identity(self, component, group):
        add_identities_group(component, group,
            {'user1@test.com', 'user2@test.com', 'user3@test.com'}, False)
        add_identities_group(component, group, {'adminA@test.com', 'adminB@test.com'}, True)
        # Remove some users
        remove_identities_group(component, group, {'user1@test.com', 'user3@test.com'}, False)
        remove_identities_group(component, group, {'adminB@test.com'}, True)
        # Removing in wrong category does nothing
        remove_identities_group(component, group, {'user2@test.com'}, True)
        remove_identities_group(component, group, {'adminA@test.com'}, False)
        # Check the result
        assert read_group(component, group) == Group(
            group, admins={'adminA@test.com'}, users={'user2@test.com'})


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
                lambda x: raise_(GroupAlreadyExist('Group already exist.')))
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
            (EGroupRead('dummy-group'), lambda x: raise_(GroupNotFound('Group not found.')))
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

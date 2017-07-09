import pytest

from effect2.testing import perform_sequence

from parsec.core.backend_vlob import (EBackendVlobCreate, EBackendVlobUpdate, EBackendVlobRead,
                                      VlobAccess, VlobAtom)
from parsec.core.backend_user_vlob import (EBackendUserVlobUpdate, EBackendUserVlobRead,
                                           UserVlobAtom)
from parsec.core.block import (Block, EBlockCreate as EBackendBlockCreate,
                               EBlockRead as EBackendBlockRead)
from parsec.core.synchronizer import (
    EBlockCreate, EBlockRead, EBlockDelete, EBlockList,
    EBlockSynchronize, EUserVlobRead, EUserVlobUpdate, EUserVlobExist, EUserVlobDelete,
    EUserVlobSynchronize, EVlobCreate, EVlobRead, EVlobUpdate, EVlobDelete, EVlobList,
    EVlobSynchronize, ESynchronize, SynchronizerComponent)
from parsec.exceptions import BlockError, BlockNotFound, UserVlobNotFound, VlobNotFound


@pytest.fixture
def app():
    return SynchronizerComponent()


def test_perform_block_create(app):
    content = 'foo'
    eff = app.perform_block_create(EBlockCreate(content))
    block_id = perform_sequence([], eff)
    eff = app.perform_block_read(EBlockRead(block_id))
    block = perform_sequence([], eff)
    assert block['content'] == content


def test_perform_block_read(app):
    def raise_(exception):
        raise exception

    local_content = 'foo'
    eff = app.perform_block_create(EBlockCreate(local_content))
    block_id = perform_sequence([], eff)
    eff = app.perform_block_read(EBlockRead(block_id))
    block = perform_sequence([], eff)
    assert sorted(list(block.keys())) == ['content', 'id']
    assert block['id']
    assert block['content'] == local_content
    remote_content = b'bar'
    eff = app.perform_block_read(EBlockRead('123'))
    sequence = [
        (EBackendBlockRead('123'),
            lambda _: Block('123', remote_content))
    ]
    block = perform_sequence(sequence, eff)
    assert sorted(list(block.keys())) == ['content', 'id']
    assert block['id']
    assert block['content'] == remote_content
    # Not found
    eff = app.perform_block_read(EBlockRead('123'))
    sequence = [
        (EBackendBlockRead('123'),
            lambda _: raise_(BlockNotFound('Block not found.')))
    ]
    with pytest.raises(BlockNotFound):
        block = perform_sequence(sequence, eff)
    eff = app.perform_block_read(EBlockRead('123'))
    sequence = [
        (EBackendBlockRead('123'),
            lambda _: raise_(BlockError('Block error.')))  # TODO keep it? usefull with multiple backends...
    ]
    with pytest.raises(BlockNotFound):
        block = perform_sequence(sequence, eff)


def test_perform_block_delete(app):
    content = 'foo'
    eff = app.perform_block_create(EBlockCreate(content))
    block_id = perform_sequence([], eff)
    eff = app.perform_block_delete(EBlockDelete(block_id))
    perform_sequence([], eff)
    with pytest.raises(BlockNotFound):
        eff = app.perform_block_delete(EBlockDelete(block_id))
        perform_sequence([], eff)


def test_perform_block_list(app):
    content = 'foo'
    eff = app.perform_block_create(EBlockCreate(content))
    block_id = perform_sequence([], eff)
    eff = app.perform_block_create(EBlockCreate(content))
    block_2_id = perform_sequence([], eff)
    eff = app.perform_block_list(EBlockList())
    block_list = perform_sequence([], eff)
    assert isinstance(block_list, list)
    assert set(block_list) == set([block_id, block_2_id])


def test_perform_block_synchronize(app):
    content = 'foo'
    eff = app.perform_block_create(EBlockCreate(content))
    block_id = perform_sequence([], eff)
    eff = app.perform_block_synchronize(EBlockSynchronize(block_id))
    sequence = [
        (EBackendBlockCreate(block_id, content),
            lambda _: Block(block_id, content)),
        # (EBlockDelete(block_id),
        #     lambda _: None)  # TODO ok to call directly the perform method?
    ]
    synchronization = perform_sequence(sequence, eff)
    assert synchronization is True
    eff = app.perform_block_list(EBlockList())
    block_list = perform_sequence([], eff)
    assert block_list == []
    # Do nothing
    eff = app.perform_block_synchronize(EBlockSynchronize(block_id))
    synchronization = perform_sequence([], eff)
    assert synchronization is False


def test_perform_user_vlob_read(app):
    local_blob = 'foo'
    eff = app.perform_user_vlob_update(EUserVlobUpdate(1, local_blob))
    perform_sequence([], eff)
    # Read remote user vlob
    remote_blob = b'bar'
    eff = app.perform_user_vlob_read(EUserVlobRead(2))
    sequence = [
        (EBackendUserVlobRead(2),
            lambda _: UserVlobAtom(2, remote_blob))
    ]
    user_vlob = perform_sequence(sequence, eff)
    assert sorted(list(user_vlob.keys())) == ['blob', 'version']
    assert user_vlob['blob'] == remote_blob.decode()  # TODO decode?
    assert user_vlob['version'] == 2
    # Read local user vlob
    eff = app.perform_user_vlob_read(EUserVlobRead(1))
    user_vlob = perform_sequence([], eff)
    assert sorted(list(user_vlob.keys())) == ['blob', 'version']
    assert user_vlob['blob'] == local_blob  # TODO decode?
    assert user_vlob['version'] == 1


def test_perform_user_vlob_update(app):
    eff = app.perform_user_vlob_update(EUserVlobUpdate(1, 'foo'))
    perform_sequence([], eff)
    blob = 'bar'
    eff = app.perform_user_vlob_update(EUserVlobUpdate(1, blob))
    perform_sequence([], eff)
    eff = app.perform_user_vlob_read(EUserVlobRead())
    user_vlob = perform_sequence([], eff)
    assert sorted(list(user_vlob.keys())) == ['blob', 'version']
    assert user_vlob['blob'] == blob
    assert user_vlob['version'] == 1


def test_perform_user_vlob_delete(app):
    eff = app.perform_user_vlob_update(EUserVlobUpdate(1, 'foo'))
    perform_sequence([], eff)
    eff = app.perform_user_vlob_delete(EUserVlobDelete())
    perform_sequence([], eff)
    with pytest.raises(UserVlobNotFound):
        eff = app.perform_user_vlob_delete(EUserVlobDelete())
        perform_sequence([], eff)


def test_perform_user_vlob_exist(app):
    eff = app.perform_user_vlob_exist(EUserVlobExist())
    exist = perform_sequence([], eff)
    assert exist is False
    eff = app.perform_user_vlob_update(EUserVlobUpdate(1, 'foo'))
    perform_sequence([], eff)
    eff = app.perform_user_vlob_exist(EUserVlobExist())
    exist = perform_sequence([], eff)
    assert exist is True


def test_perform_user_vlob_synchronize(app):
    blob = 'foo'
    eff = app.perform_user_vlob_update(EUserVlobUpdate(1, blob))
    perform_sequence([], eff)
    eff = app.perform_user_vlob_synchronize(EUserVlobSynchronize())
    sequence = [
        (EBackendUserVlobUpdate(1, blob.encode()),  # TODO encode ok ?
            lambda _: None)
    ]
    synchronization = perform_sequence(sequence, eff)
    assert synchronization is True
    eff = app.perform_user_vlob_exist(EUserVlobExist())
    exist = perform_sequence([], eff)
    assert exist is False
    # Do nothing
    eff = app.perform_user_vlob_synchronize(EUserVlobSynchronize())
    synchronization = perform_sequence([], eff)
    assert synchronization is False


def test_perform_vlob_create(app):
    blob = 'foo'
    eff = app.perform_vlob_create(EVlobCreate(blob))
    vlob = perform_sequence([], eff)
    vlob_id = vlob['id']
    read_trust_seed = vlob['read_trust_seed']
    assert sorted(list(vlob.keys())) == ['id', 'read_trust_seed', 'write_trust_seed']
    eff = app.perform_vlob_read(EVlobRead(vlob_id, read_trust_seed))
    vlob = perform_sequence([], eff)
    assert vlob['blob'] == blob


def test_perform_vlob_read(app):
    local_blob = 'foo'
    eff = app.perform_vlob_update(EVlobUpdate('123', '43', 1, local_blob))
    perform_sequence([], eff)
    # Read remote vlob
    remote_blob = b'bar'
    eff = app.perform_vlob_read(EVlobRead('123', 'ABC', 2))
    sequence = [
        (EBackendVlobRead('123', 'ABC', 2),
            lambda _: VlobAtom('123', 2, remote_blob))
    ]
    vlob = perform_sequence(sequence, eff)
    assert sorted(list(vlob.keys())) == ['blob', 'id', 'version']
    assert vlob['id'] == '123'
    assert vlob['blob'] == remote_blob.decode()  # TODO decode?
    assert vlob['version'] == 2
    # Read local vlob
    eff = app.perform_vlob_read(EVlobRead('123', '43', 1))
    vlob = perform_sequence([], eff)
    assert sorted(list(vlob.keys())) == ['blob', 'id', 'version']
    assert vlob['id'] == '123'
    assert vlob['blob'] == local_blob  # TODO decode?
    assert vlob['version'] == 1


def test_perform_vlob_update(app):
    eff = app.perform_vlob_update(EVlobUpdate('123', 'ABC', 1, 'foo'))
    perform_sequence([], eff)
    blob = 'bar'
    eff = app.perform_vlob_update(EVlobUpdate('123', 'ABC', 1, blob))
    perform_sequence([], eff)
    eff = app.perform_vlob_read(EVlobRead('123', 'ABC'))
    vlob = perform_sequence([], eff)
    assert sorted(list(vlob.keys())) == ['blob', 'id', 'version']
    assert vlob['id'] == '123'
    assert vlob['blob'] == blob
    assert vlob['version'] == 1


def test_perform_vlob_delete(app):
    eff = app.perform_vlob_update(EVlobUpdate('123', 'ABC', 1, 'foo'))
    perform_sequence([], eff)
    eff = app.perform_vlob_delete(EVlobDelete('123'))
    perform_sequence([], eff)
    with pytest.raises(VlobNotFound):
        eff = app.perform_vlob_delete(EVlobDelete('123'))
        perform_sequence([], eff)


def test_perform_vlob_list(app):
    blob = 'foo'
    eff = app.perform_vlob_create(EVlobCreate(blob))
    vlob = perform_sequence([], eff)
    vlob_id = vlob['id']
    eff = app.perform_vlob_create(EVlobCreate(blob))
    vlob = perform_sequence([], eff)
    vlob_2_id = vlob['id']
    eff = app.perform_vlob_list(EVlobList())
    vlob_list = perform_sequence([], eff)
    assert isinstance(vlob_list, list)
    assert set(vlob_list) == set([vlob_id, vlob_2_id])


def test_perform_vlob_synchronize(app):
    # Create new vlob
    blob = 'foo'
    eff = app.perform_vlob_update(EVlobUpdate('123', 'ABC', 1, blob))
    perform_sequence([], eff)
    eff = app.perform_vlob_synchronize(EVlobSynchronize('123'))
    sequence = [
        (EBackendVlobCreate(blob.encode()),  # TODO encode ok ?
            lambda _: VlobAccess('123', 'ABC', 'DEF'))
    ]
    synchronization = perform_sequence(sequence, eff)
    assert synchronization == {'id': '123', 'read_trust_seed': 'ABC', 'write_trust_seed': 'DEF'}
    eff = app.perform_vlob_list(EVlobList())
    vlob_list = perform_sequence([], eff)
    assert vlob_list == []
    # Update vlob
    blob = 'bar'
    eff = app.perform_vlob_update(EVlobUpdate('123', 'ABC', 2, blob))
    perform_sequence([], eff)
    eff = app.perform_vlob_synchronize(EVlobSynchronize('123'))
    sequence = [
        (EBackendVlobUpdate('123', 'ABC', 2, blob.encode()),  # TODO encode ok ?
            lambda _: None)
    ]
    synchronization = perform_sequence(sequence, eff)
    assert synchronization is True
    eff = app.perform_vlob_list(EVlobList())
    vlob_list = perform_sequence([], eff)
    assert vlob_list == []
    # Do nothing
    eff = app.perform_vlob_synchronize(EVlobSynchronize('123'))
    synchronization = perform_sequence([], eff)
    assert synchronization is False


def test_perform_synchronize(app):
    content = 'foo'
    eff = app.perform_block_create(EBlockCreate(content))
    block_id = perform_sequence([], eff)
    eff = app.perform_block_create(EBlockCreate(content))
    block_2_id = perform_sequence([], eff)
    block_ids = sorted([block_id, block_2_id])
    blob = 'foo'
    eff = app.perform_vlob_create(EVlobCreate(blob))
    perform_sequence([], eff)
    eff = app.perform_vlob_create(EVlobCreate(blob))
    perform_sequence([], eff)
    eff = app.perform_synchronize(ESynchronize())
    sequence = [
        (EBackendBlockCreate(block_ids[0], content),
            lambda _: Block(block_ids[0], content)),
        (EBackendBlockCreate(block_ids[1], content),
            lambda _: Block(block_ids[1], content)),
        (EBackendVlobCreate(blob.encode()),  # TODO encode correct?
            lambda _: VlobAccess('345', 'ABC', 'DEF')),
        (EBackendVlobCreate(blob.encode()),  # TODO encode correct?
            lambda _: VlobAccess('678', 'ABC', 'DEF')),
    ]
    synchronization = perform_sequence(sequence, eff)
    assert synchronization is True
    eff = app.perform_block_list(EBlockList())
    block_list = perform_sequence([], eff)
    assert block_list == []
    eff = app.perform_user_vlob_exist(EUserVlobExist())
    exist = perform_sequence([], eff)
    assert exist is False
    eff = app.perform_vlob_list(EVlobList())
    vlob_list = perform_sequence([], eff)
    assert vlob_list == []
    # Do nothing
    eff = app.perform_synchronize(ESynchronize())
    synchronization = perform_sequence([], eff)
    assert synchronization is False

import pytest

from arrow import Arrow
from effect2.testing import const, noop, perform_sequence, raise_
from freezegun import freeze_time

from parsec.core.backend_vlob import (EBackendVlobCreate, EBackendVlobUpdate, EBackendVlobRead,
                                      VlobAccess, VlobAtom)
from parsec.core.backend_user_vlob import (EBackendUserVlobUpdate, EBackendUserVlobRead,
                                           UserVlobAtom)
from parsec.core.block import (Block, EBlockCreate as EBackendBlockCreate,
                               EBlockRead as EBackendBlockRead)
from parsec.core.synchronizer import (
    EBlockCreate, EBlockRead, EBlockDelete, EBlockList, EBlockSynchronize, ECacheClean,
    EUserVlobRead, EUserVlobUpdate, EUserVlobExist, EUserVlobDelete,
    EUserVlobSynchronize, EVlobCreate, EVlobRead, EVlobUpdate, EVlobDelete, EVlobList,
    EVlobSynchronize, ESynchronize, SynchronizerComponent)
from parsec.exceptions import BlockError, BlockNotFound, UserVlobNotFound, VlobNotFound


@pytest.fixture
def app():
    return SynchronizerComponent()


def test_perform_block_create(app):
    content = 'foo'
    with freeze_time('2012-01-01') as frozen_datetime:
        eff = app.perform_block_create(EBlockCreate(content))
        block_id = perform_sequence([], eff)
        assert app.last_modified == Arrow.fromdatetime(frozen_datetime())
    eff = app.perform_block_read(EBlockRead(block_id))
    block = perform_sequence([], eff)
    assert block['content'] == content


def test_perform_block_read(app):
    local_content = 'foo'
    eff = app.perform_block_create(EBlockCreate(local_content))
    block_id = perform_sequence([], eff)
    # With cache
    eff = app.perform_block_read(EBlockRead(block_id))
    block = perform_sequence([], eff)
    assert sorted(list(block.keys())) == ['content', 'id']
    assert block['id']
    assert block['content'] == local_content
    remote_content = b'bar'
    # Without cache
    eff = app.perform_block_read(EBlockRead('123'))
    sequence = [
        (EBackendBlockRead('123'),
            const(Block('123', remote_content)))
    ]
    block = perform_sequence(sequence, eff)
    assert sorted(list(block.keys())) == ['content', 'id']
    assert block['id']
    assert block['content'] == remote_content
    # Delete block from cache
    eff = app.perform_block_delete(EBlockDelete('123'))
    perform_sequence([], eff)
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
    with freeze_time('2012-01-01') as frozen_datetime:
        eff = app.perform_block_delete(EBlockDelete(block_id))
        perform_sequence([], eff)
        assert app.last_modified == Arrow.fromdatetime(frozen_datetime())
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
    # Synchronized blocks are excluded
    app.buffered_blocks[block_2_id].synchronized = True
    eff = app.perform_block_list(EBlockList())
    block_list = perform_sequence([], eff)
    assert isinstance(block_list, list)
    assert set(block_list) == set([block_id])


def test_perform_block_synchronize(app):
    content = 'foo'
    eff = app.perform_block_create(EBlockCreate(content))
    block_id = perform_sequence([], eff)
    eff = app.perform_block_synchronize(EBlockSynchronize(block_id))
    sequence = [
        (EBackendBlockCreate(block_id, content),
            const(Block(block_id, content)))
    ]
    synchronization = perform_sequence(sequence, eff)
    assert synchronization is True
    eff = app.perform_block_list(EBlockList())
    block_list = perform_sequence([], eff)
    assert block_list == []
    assert app.buffered_blocks[block_id].synchronized
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
            const(UserVlobAtom(2, remote_blob)))
    ]
    user_vlob = perform_sequence(sequence, eff)
    assert sorted(list(user_vlob.keys())) == ['blob', 'version']
    assert user_vlob['blob'] == remote_blob.decode()  # TODO decode?
    assert user_vlob['version'] == 2
    # Delete user vlob from cache
    eff = app.perform_user_vlob_delete(EUserVlobDelete(2))
    perform_sequence([], eff)
    # Read local user vlob
    eff = app.perform_user_vlob_read(EUserVlobRead(1))
    user_vlob = perform_sequence([], eff)
    assert sorted(list(user_vlob.keys())) == ['blob', 'version']
    assert user_vlob['blob'] == local_blob  # TODO decode?
    assert user_vlob['version'] == 1


def test_perform_user_vlob_update(app):
    with freeze_time('2012-01-01') as frozen_datetime:
        eff = app.perform_user_vlob_update(EUserVlobUpdate(1, 'foo'))
        perform_sequence([], eff)
        assert app.last_modified == Arrow.fromdatetime(frozen_datetime())
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
    with freeze_time('2012-01-01') as frozen_datetime:
        eff = app.perform_user_vlob_delete(EUserVlobDelete())
        perform_sequence([], eff)
        assert app.last_modified == Arrow.fromdatetime(frozen_datetime())
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
            noop)
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
    with freeze_time('2012-01-01') as frozen_datetime:
        eff = app.perform_vlob_create(EVlobCreate(blob))
        vlob = perform_sequence([], eff)
        assert app.last_modified == Arrow.fromdatetime(frozen_datetime())
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
            const(VlobAtom('123', 2, remote_blob)))
    ]
    vlob = perform_sequence(sequence, eff)
    assert sorted(list(vlob.keys())) == ['blob', 'id', 'version']
    assert vlob['id'] == '123'
    assert vlob['blob'] == remote_blob.decode()  # TODO decode?
    assert vlob['version'] == 2
    # Delete user vlob from cache
    eff = app.perform_vlob_delete(EVlobDelete('123', 2))
    perform_sequence([], eff)
    # Read local vlob
    eff = app.perform_vlob_read(EVlobRead('123', '43', 1))
    vlob = perform_sequence([], eff)
    assert sorted(list(vlob.keys())) == ['blob', 'id', 'version']
    assert vlob['id'] == '123'
    assert vlob['blob'] == local_blob  # TODO decode?
    assert vlob['version'] == 1


def test_perform_vlob_update(app):
    with freeze_time('2012-01-01') as frozen_datetime:
        eff = app.perform_vlob_update(EVlobUpdate('123', 'ABC', 1, 'foo'))
        perform_sequence([], eff)
        assert app.last_modified == Arrow.fromdatetime(frozen_datetime())
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
    # Synchronized vlobs are excluded
    app.buffered_vlobs[vlob_2_id][1].synchronized = True
    eff = app.perform_vlob_list(EVlobList())
    vlob_list = perform_sequence([], eff)
    assert isinstance(vlob_list, list)
    assert set(vlob_list) == set([vlob_id])


def test_get_current_vlob_version(app):
    with pytest.raises(VlobNotFound):
        app.get_current_vlob_version('123')
    blob = 'bar'
    eff = app.perform_vlob_update(EVlobUpdate('123', 'ABC', 1, blob))
    perform_sequence([], eff)
    eff = app.perform_vlob_update(EVlobUpdate('123', 'ABC', 2, blob))
    perform_sequence([], eff)
    current_version = app.get_current_vlob_version('123')
    assert current_version == 2
    app.buffered_vlobs['123'][2].synchronized = True
    current_version = app.get_current_vlob_version('123')
    assert current_version is 1


def test_perform_vlob_synchronize(app):
    # Create new vlob
    blob = 'foo'
    eff = app.perform_vlob_update(EVlobUpdate('123', 'ABC', 1, blob))
    perform_sequence([], eff)
    eff = app.perform_vlob_synchronize(EVlobSynchronize('123'))
    sequence = [
        (EBackendVlobCreate(blob.encode()),  # TODO encode ok ?
            const(VlobAccess('123', 'ABC', 'DEF')))
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
            noop)
    ]
    synchronization = perform_sequence(sequence, eff)
    assert synchronization is True
    eff = app.perform_vlob_list(EVlobList())
    vlob_list = perform_sequence([], eff)
    assert vlob_list == []
    # Vlob synchronized
    eff = app.perform_vlob_synchronize(EVlobSynchronize('123'))
    synchronization = perform_sequence([], eff)
    assert synchronization is False
    # Delete user vlob from cache
    for version in [1, 2]:
        eff = app.perform_vlob_delete(EVlobDelete('123', version))
        perform_sequence([], eff)
    # Do nothing if vlob not found
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
            const(Block(block_ids[0], content))),
        (EBackendBlockCreate(block_ids[1], content),
            const(Block(block_ids[1], content))),
        (EBackendVlobCreate(blob.encode()),  # TODO encode correct?
            const(VlobAccess('345', 'ABC', 'DEF'))),
        (EBackendVlobCreate(blob.encode()),  # TODO encode correct?
            const(VlobAccess('678', 'ABC', 'DEF'))),
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


def test_perform_cache_clean(app):
    content = 'foo'
    eff = app.perform_block_create(EBlockCreate(content))
    block_id = perform_sequence([], eff)
    eff = app.perform_block_create(EBlockCreate(content))
    block_2_id = perform_sequence([], eff)
    app.buffered_blocks[block_2_id].synchronized = True
    eff = app.perform_vlob_create(EVlobCreate(content))
    vlob = perform_sequence([], eff)
    eff = app.perform_vlob_create(EVlobCreate(content))
    vlob_2 = perform_sequence([], eff)
    app.buffered_vlobs[vlob_2['id']][1].synchronized = True
    eff = app.perform_user_vlob_update(EUserVlobUpdate(1, content))
    perform_sequence([], eff)
    eff = app.perform_user_vlob_update(EUserVlobUpdate(2, content))
    perform_sequence([], eff)
    app.buffered_user_vlob[2].synchronized = True
    eff = app.perform_cache_clean(ECacheClean())
    ret = perform_sequence([], eff)
    assert ret is None
    assert block_id in app.buffered_blocks
    assert block_2_id not in app.buffered_blocks
    assert vlob['id'] in app.buffered_vlobs
    assert vlob_2['id'] not in app.buffered_vlobs
    assert 1 in app.buffered_user_vlob
    assert 2 not in app.buffered_user_vlob


def test_perform_periodic_synchronization(app):
    # TODO
    pass

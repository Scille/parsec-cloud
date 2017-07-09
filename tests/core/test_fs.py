from unittest.mock import Mock
from effect2.testing import perform_sequence, const

from parsec.core.fs import FSComponent
from parsec.core.privkey import EPrivkeyAdd, EPrivkeyGet, EPrivkeyLoad, PrivKeyComponent
from parsec.core.identity import EIdentityLoad, Identity


def test_perform_synchronize():
    app = FSComponent()
    assert app._user_manifest is None
    assert app._synchronizer is None
    eff = app.perform_synchronize(None)
    ret = perform_sequence([], eff)
    assert ret is None


def test_perform_group_create():
    raise NotImplementedError()


def test_perform_dustbin_show():
    raise NotImplementedError()


def test_perform_manifest_history():
    raise NotImplementedError()


def test_perform_manifest_restore():
    raise NotImplementedError()


def test_perform_file_create():
    raise NotImplementedError()


def test_perform_file_read():
    raise NotImplementedError()


def test_perform_file_write():
    raise NotImplementedError()


def test_perform_file_truncate():
    raise NotImplementedError()


def test_perform_file_history():
    raise NotImplementedError()


def test_perform_file_restore():
    raise NotImplementedError()


def test_perform_folder_create():
    raise NotImplementedError()


def test_perform_stat():
    raise NotImplementedError()


def test_perform_move():
    raise NotImplementedError()


def test_perform_delete():
    raise NotImplementedError()


def test_perform_undelete():
    raise NotImplementedError()

import arrow
import pytest
from unittest.mock import Mock

from parsec.core2.manifest import (
    load_user_manifest, dump_user_manifest, FolderEntry, FileManifestAccess, UserManifest
)
from parsec.exceptions import InvalidManifest


@pytest.fixture
def mock_crypto(mocker):
    mock_sym_key = Mock()
    mock_sym_key.key = b"0123456789ABCDEF"
    mocker.patch('parsec.core2.manifest.load_sym_key', return_value=mock_sym_key)
    yield


class TestFolder:

    def test_new(self):
        f = FolderEntry()
        assert isinstance(f.created, arrow.Arrow)
        assert isinstance(f.updated, arrow.Arrow)
        assert not f.children

    def test_bad_access(self):
        root = FolderEntry()
        with pytest.raises(KeyError):
            root.children['missing']
        with pytest.raises(KeyError):
            del root.children['missing']

    def test_new_Entry(self, mock_crypto):
        root = FolderEntry()
        root.children['foo'] = FolderEntry()
        assert sorted(root.children.keys()) == ['foo']


RAW_MANIFEST = ('{'
    '"type": "folder",'
    '"updated": "2017-06-17T16:28:17.646735+00:00",'
    '"created": "2017-06-17T16:28:17.646735+00:00",'
    '"children": {'
        '"foo": {'
        '"type": "folder",'
        '"updated": "2017-06-17T16:28:17.646735+00:00",'
        '"created": "2017-06-17T16:28:17.646735+00:00",'
        '"children": {'
            '"foooo": {'
                '"type": "file",'
                '"id": "e86c4044b3c14c9585c82eb5da720865",'
                '"key": "0123456789ABCDEF",'  # Dummy key
                '"signature": "0123456789ABCDEF",'  # Dummy signature
                '"read_trust_seed": "W9IY2MVFAHTG",'
                '"write_trust_seed": "5Z1I6Q29QO7U"'
            '}'
        '}'
    '},'
    '"bar": {'
        '"type": "file",'
        '"id": "0e66cb0f34114d5a9a0ac28c016f64d7",'
        '"key": "0123456789ABCDEF",'  # Dummy key
        '"signature": "0123456789ABCDEF",'  # Dummy signature
        '"read_trust_seed": "1TGEQKWTDE8W",'
        '"write_trust_seed": "99DR7DQW1M94"'
    '}'
'}'
'}').encode()


class TestUserManifest:
    def test_load_empty_user_manifest(self):
        wksp = load_user_manifest(b'')
        assert isinstance(wksp, UserManifest)
        assert not wksp.children

    def test_load_user_manifest(self, mock_crypto):
        wksp = load_user_manifest(RAW_MANIFEST)
        assert isinstance(wksp, UserManifest)
        assert sorted(wksp.children.keys()) == ['bar', 'foo']
        assert isinstance(wksp.created, arrow.Arrow)
        assert isinstance(wksp.updated, arrow.Arrow)
        foo = wksp.children['foo']
        assert isinstance(foo, FolderEntry)
        assert sorted(foo.children.keys()) == ['foooo']
        bar = wksp.children['bar']
        assert isinstance(bar, FileManifestAccess)

    @pytest.mark.parametrize('patch', [
        ('"type": "folder",', ''),
        ('"type": "folder"', '"type": "file"'),
        ('"type": "folder"', '$$^.syntax error'),
        ('"key": "0123456789ABCDEF",', ''),
        ('"children": {', '"children": {"bad_kid": {},'),
        ('"children": {', '"dummy_field": {'),
    ])
    def test_load_invalid_dump(self, patch, mock_crypto):
        bad_raw_wksp = RAW_MANIFEST.replace(*[x.encode() for x in patch])
        with pytest.raises(InvalidManifest):
            load_user_manifest(bad_raw_wksp)

    def test_dump(self, mock_crypto):
        load1 = load_user_manifest(RAW_MANIFEST)
        dump = dump_user_manifest(load1)
        assert isinstance(dump, bytes)
        load2 = load_user_manifest(dump)
        assert load1 == load2

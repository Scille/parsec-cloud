import pytest
from uuid import UUID
from pendulum import datetime

from parsec.core.schemas import *


REMOVE_FIELD = object()


class TestBlockAccessSchema:
    ORIGINAL = {
        "id": "4f55b4d5b08544e2a784daf73754c7e2",
        "key": b"<my key>",
        "offset": 0,
        "size": 4096,
        "digest": "63b0a598a7f3a679d6bff87e6692ae7248bc88b3d0c23444e25d5b8567039953",
    }

    def test_load_and_dump(self):
        loaded, errors = BlockAccessSchema.load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "id": UUID("4f55b4d5b08544e2a784daf73754c7e2"),
            "key": b"<my key>",
            "offset": 0,
            "size": 4096,
            "digest": "63b0a598a7f3a679d6bff87e6692ae7248bc88b3d0c23444e25d5b8567039953",
        }
        dumped, errors = BlockAccessSchema.dump(loaded)
        assert not errors
        assert dumped == self.ORIGINAL

    @pytest.mark.parametrize(
        "bad_patch",
        [
            {"unkown": "field"},
            {"id": REMOVE_FIELD},
            {"key": REMOVE_FIELD},
            {"offset": REMOVE_FIELD},
            {"size": REMOVE_FIELD},
            {"id": None},
            {"key": None},
            {"offset": None},
            {"size": None},
            {"size": -1},
            {"offset": -1},
            {
                # too long
                "id": "x"
                * 1000
            },
            {"digest": None},
        ],
    )
    def test_bad_load(self, bad_patch):
        bad_data = {**self.ORIGINAL, **bad_patch}
        bad_data = {k: v for k, v in bad_data.items() if v is not REMOVE_FIELD}
        data, errors = BlockAccessSchema.load(bad_data)
        assert errors


class TestFileManifestSchema:
    ORIGINAL = {
        "format": 1,
        "type": "file_manifest",
        "author": "alice@test",
        "version": 2,
        "created": "2017-01-01T00:00:00+00:00",
        "updated": "2017-12-31T23:59:59+00:00",
        "size": 800,
        "blocks": [
            {
                "id": "27537f7cedba434ea3bf7848ba17a2eb",
                "key": "PGJsb2NrIDEga2V5Pg==\n",
                "offset": 0,
                "size": 4096,
                "digest": "NHlUNUUwV0tPeXkweHFUNmVTdVlkUGdTNkpzYUduOGNUdWlEd0IzZ1Jscz0=\n",
            },
            {
                "id": "b64041c3e3d649fc931ca54564701d38",
                "key": "PGJsb2NrIDIga2V5Pg==\n",
                "offset": 4096,
                "size": 3904,
                "digest": "NHlUNUUwV0tPeXkweHFUNmVTdVlkUGdTNkpzYUduOGNUdWlEd0IzZ1Jscz0=\n",
            },
        ],
    }

    @pytest.mark.xfail
    def test_load_and_dump(self):
        loaded, errors = FileManifestSchema.load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "format": 1,
            "type": "file_manifest",
            "author": "alice@test",
            "version": 2,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "size": 800,
            "blocks": [
                {
                    "id": "27537f7cedba434ea3bf7848ba17a2eb",
                    "key": b"<block 1 key>",
                    "offset": 0,
                    "size": 4096,
                    "digest": b"4yT5E0WKOyy0xqT6eSuYdPgS6JsaGn8cTuiDwB3gRls=",
                },
                {
                    "id": "b64041c3e3d649fc931ca54564701d38",
                    "key": b"<block 2 key>",
                    "offset": 4096,
                    "size": 3904,
                    "digest": b"4yT5E0WKOyy0xqT6eSuYdPgS6JsaGn8cTuiDwB3gRls=",
                },
            ],
        }
        dumped, errors = FileManifestSchema.dump(loaded)
        assert not errors
        assert dumped == self.ORIGINAL

    @pytest.mark.parametrize(
        "bad_patch",
        [
            {"unkown": "field"},
            {"format": REMOVE_FIELD},
            {"format": None},
            {"type": REMOVE_FIELD},
            {"type": None},
            {"version": REMOVE_FIELD},
            {"version": None},
            {"created": REMOVE_FIELD},
            {"created": None},
            {"updated": REMOVE_FIELD},
            {"updated": None},
            {"size": REMOVE_FIELD},
            {"size": None},
            {"blocks": REMOVE_FIELD},
            {"blocks": None},
            {"type": "dummy"},
            {"version": -1},
            {"version": 0},
            {"size": -1},
            {"blocks": {}},
        ],
    )
    def test_bad_load(self, bad_patch):
        bad_data = {**self.ORIGINAL, **bad_patch}
        bad_data = {k: v for k, v in bad_data.items() if v is not REMOVE_FIELD}
        data, errors = FileManifestSchema.load(bad_data)
        assert errors


class TestFolderManifestSchema:
    ORIGINAL = {
        "format": 1,
        "type": "folder_manifest",
        "author": "alice@test",
        "version": 2,
        "created": "2017-01-01T00:00:00+00:00",
        "updated": "2017-12-31T23:59:59+00:00",
        "children": {
            "foo": {
                "id": "8aadbc777ece4a4fb5fa0564ecfbb54f",
                "rts": "9809c436b3af4fba9dd6955ad03e0310",
                "wts": "004714d9997147efa52a696127694fdc",
                "key": b"<foo key>",
            },
            "bar.txt": {
                "id": "51c865a60b194d9bb087df000056c299",
                "rts": "5a48035bdd7c4082b1101b28b6656d0c",
                "wts": "9b2ab384ed6b426daa1214f49587458a",
                "key": b"<bar.txt key>",
            },
        },
    }

    def test_load_and_dump(self):
        loaded, errors = FolderManifestSchema.load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "format": 1,
            "type": "folder_manifest",
            "author": "alice@test",
            "version": 2,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "children": {
                "foo": {
                    "id": UUID("8aadbc777ece4a4fb5fa0564ecfbb54f"),
                    "rts": "9809c436b3af4fba9dd6955ad03e0310",
                    "wts": "004714d9997147efa52a696127694fdc",
                    "key": b"<foo key>",
                },
                "bar.txt": {
                    "id": UUID("51c865a60b194d9bb087df000056c299"),
                    "rts": "5a48035bdd7c4082b1101b28b6656d0c",
                    "wts": "9b2ab384ed6b426daa1214f49587458a",
                    "key": b"<bar.txt key>",
                },
            },
        }
        dumped, errors = FolderManifestSchema.dump(loaded)
        assert not errors
        assert dumped == self.ORIGINAL

    @pytest.mark.parametrize(
        "bad_patch",
        [
            {"unkown": "field"},
            {"format": REMOVE_FIELD},
            {"format": None},
            {"type": REMOVE_FIELD},
            {"type": None},
            {"version": REMOVE_FIELD},
            {"version": None},
            {"created": REMOVE_FIELD},
            {"created": None},
            {"updated": REMOVE_FIELD},
            {"updated": None},
            {"children": REMOVE_FIELD},
            {"children": None},
            {"type": "dummy"},
            {"version": -1},
            {"version": 0},
            {"children": []},
        ],
    )
    def test_bad_load(self, bad_patch):
        bad_data = {**self.ORIGINAL, **bad_patch}
        bad_data = {k: v for k, v in bad_data.items() if v is not REMOVE_FIELD}
        data, errors = FolderManifestSchema.load(bad_data)
        assert errors


class TestUserManifestSchema:
    # TODO
    pass


class TestLocalFileManifestSchema:
    ORIGINAL = {
        "format": 1,
        "type": "local_file_manifest",
        "author": "alice@test",
        "base_version": 0,
        "need_sync": True,
        "is_placeholder": True,
        "created": "2017-01-01T00:00:00+00:00",
        "updated": "2017-12-31T23:59:59+00:00",
        "size": 800,
        "blocks": [
            {
                "id": "27537f7cedba434ea3bf7848ba17a2eb",
                "key": "PGJsb2NrIDEga2V5Pg==\n",
                "offset": 0,
                "size": 4096,
                "digest": "NHlUNUUwV0tPeXkweHFUNmVTdVlkUGdTNkpzYUduOGNUdWlEd0IzZ1Jscz0=\n",
            },
            {
                "id": "b64041c3e3d649fc931ca54564701d38",
                "key": "PGJsb2NrIDIga2V5Pg==\n",
                "offset": 5096,
                "size": 2904,
                "digest": "NHlUNUUwV0tPeXkweHFUNmVTdVlkUGdTNkpzYUduOGNUdWlEd0IzZ1Jscz0=\n",
            },
        ],
        "dirty_blocks": [
            {
                "id": "22cd9a5503ef49919f29f1452a9a628c",
                "key": "PGRpcnR5IGJsb2NrIDEga2V5Pg==\n",
                "offset": 4096,
                "size": 1000,
                "digest": "NHlUNUUwV0tPeXkweHFUNmVTdVlkUGdTNkpzYUduOGNUdWlEd0IzZ1Jscz0=\n",
            },
            {
                "id": "03ef5b652762404bb1d2a55474cbc95f",
                "key": "PGRpcnR5IGJsb2NrIDIga2V5Pg==\n",
                "offset": 8000,
                "size": 100,
                "digest": "NHlUNUUwV0tPeXkweHFUNmVTdVlkUGdTNkpzYUduOGNUdWlEd0IzZ1Jscz0=\n",
            },
        ],
    }

    @pytest.mark.xfail
    def test_load_and_dump(self):
        loaded, errors = LocalFileManifestSchema.load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "format": 1,
            "type": "local_file_manifest",
            "author": "alice@test",
            "base_version": 0,
            "need_sync": True,
            "is_placeholder": True,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "size": 800,
            "blocks": [
                {
                    "id": "27537f7cedba434ea3bf7848ba17a2eb",
                    "key": b"<block 1 key>",
                    "offset": 0,
                    "size": 4096,
                    "digest": b"4yT5E0WKOyy0xqT6eSuYdPgS6JsaGn8cTuiDwB3gRls=",
                },
                {
                    "id": "b64041c3e3d649fc931ca54564701d38",
                    "key": b"<block 2 key>",
                    "offset": 5096,
                    "size": 2904,
                    "digest": b"4yT5E0WKOyy0xqT6eSuYdPgS6JsaGn8cTuiDwB3gRls=",
                },
            ],
            "dirty_blocks": [
                {
                    "id": "22cd9a5503ef49919f29f1452a9a628c",
                    "key": b"<dirty block 1 key>",
                    "offset": 4096,
                    "size": 1000,
                    "digest": b"4yT5E0WKOyy0xqT6eSuYdPgS6JsaGn8cTuiDwB3gRls=",
                },
                {
                    "id": "03ef5b652762404bb1d2a55474cbc95f",
                    "key": b"<dirty block 2 key>",
                    "offset": 8000,
                    "size": 100,
                    "digest": b"4yT5E0WKOyy0xqT6eSuYdPgS6JsaGn8cTuiDwB3gRls=",
                },
            ],
        }
        dumped, errors = LocalFileManifestSchema.dump(loaded)
        assert not errors
        assert dumped == self.ORIGINAL

    @pytest.mark.parametrize(
        "bad_patch",
        [
            {"unkown": "field"},
            {"format": REMOVE_FIELD},
            {"format": None},
            {"type": REMOVE_FIELD},
            {"type": None},
            {"base_version": REMOVE_FIELD},
            {"base_version": None},
            {"created": REMOVE_FIELD},
            {"created": None},
            {"updated": REMOVE_FIELD},
            {"updated": None},
            {"size": REMOVE_FIELD},
            {"size": None},
            {"blocks": REMOVE_FIELD},
            {"blocks": None},
            {"dirty_blocks": REMOVE_FIELD},
            {"dirty_blocks": None},
            {"need_sync": REMOVE_FIELD},
            {"need_sync": None},
            {"is_placeholder": REMOVE_FIELD},
            {"is_placeholder": None},
            {"type": "local_dummy"},
            {"base_version": -1},
            {"size": -1},
            {"blocks": {}},
            {"dirty_blocks": {}},
        ],
    )
    def test_bad_load(self, bad_patch):
        bad_data = {**self.ORIGINAL, **bad_patch}
        bad_data = {k: v for k, v in bad_data.items() if v is not REMOVE_FIELD}
        data, errors = LocalFileManifestSchema.load(bad_data)
        assert errors


class TestLocalFolderManifestSchema:
    ORIGINAL = {
        "format": 1,
        "type": "local_folder_manifest",
        "author": "alice@test",
        "base_version": 0,
        "need_sync": True,
        "is_placeholder": True,
        "created": "2017-01-01T00:00:00+00:00",
        "updated": "2017-12-31T23:59:59+00:00",
        "children": {
            "foo": {
                "id": "8aadbc777ece4a4fb5fa0564ecfbb54f",
                "rts": "9809c436b3af4fba9dd6955ad03e0310",
                "wts": "004714d9997147efa52a696127694fdc",
                "key": b"<foo key>",
            },
            "bar.txt": {
                "id": "51c865a60b194d9bb087df000056c299",
                "rts": "d756228815074b1f9ea6e5d383bcb995",
                "wts": "ba4fddebe99d4f3e9f17ccb84a524e19",
                "key": b"<bar.txt key>",
            },
        },
    }

    def test_load_and_dump(self):
        loaded, errors = LocalFolderManifestSchema.load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "format": 1,
            "type": "local_folder_manifest",
            "author": "alice@test",
            "base_version": 0,
            "need_sync": True,
            "is_placeholder": True,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "children": {
                "foo": {
                    "id": UUID("8aadbc777ece4a4fb5fa0564ecfbb54f"),
                    "rts": "9809c436b3af4fba9dd6955ad03e0310",
                    "wts": "004714d9997147efa52a696127694fdc",
                    "key": b"<foo key>",
                },
                "bar.txt": {
                    "id": UUID("51c865a60b194d9bb087df000056c299"),
                    "rts": "d756228815074b1f9ea6e5d383bcb995",
                    "wts": "ba4fddebe99d4f3e9f17ccb84a524e19",
                    "key": b"<bar.txt key>",
                },
            },
        }
        dumped, errors = LocalFolderManifestSchema.dump(loaded)
        assert not errors
        assert dumped == self.ORIGINAL

    @pytest.mark.parametrize(
        "bad_patch",
        [
            {"unkown": "field"},
            {"format": REMOVE_FIELD},
            {"format": None},
            {"type": REMOVE_FIELD},
            {"type": None},
            {"base_version": REMOVE_FIELD},
            {"base_version": None},
            {"created": REMOVE_FIELD},
            {"created": None},
            {"updated": REMOVE_FIELD},
            {"updated": None},
            {"children": REMOVE_FIELD},
            {"children": None},
            {"need_sync": REMOVE_FIELD},
            {"need_sync": None},
            {"is_placeholder": REMOVE_FIELD},
            {"is_placeholder": None},
            {"type": "dummy"},
            {"base_version": -1},
            {"children": []},
        ],
    )
    def test_bad_load(self, bad_patch):
        bad_data = {**self.ORIGINAL, **bad_patch}
        bad_data = {k: v for k, v in bad_data.items() if v is not REMOVE_FIELD}
        data, errors = LocalFolderManifestSchema.load(bad_data)
        assert errors


class TestLocalUserManifestSchema:
    # TODO
    pass

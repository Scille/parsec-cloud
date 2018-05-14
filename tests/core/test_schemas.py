import pytest
from pendulum import datetime

from parsec.core.schemas import *


REMOVE_FIELD = object()


class TestBlockAccessSchema:
    ORIGINAL = {
        "id": "4f55b4d5b08544e2a784daf73754c7e2",
        "key": "PG15IGtleT4=\n",
        "offset": 0,
        "size": 4096,
        "digest": "NHlUNUUwV0tPeXkweHFUNmVTdVlkUGdTNkpzYUduOGNUdWlEd0IzZ1Jscz0=\n",
    }

    def test_load_and_dump(self):
        loaded, errors = BlockAccessSchema().load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "id": "4f55b4d5b08544e2a784daf73754c7e2",
            "key": b"<my key>",
            "offset": 0,
            "size": 4096,
            "digest": b"4yT5E0WKOyy0xqT6eSuYdPgS6JsaGn8cTuiDwB3gRls=",
        }
        dumped, errors = BlockAccessSchema().dump(loaded)
        assert not errors
        assert dumped == self.ORIGINAL

    @pytest.mark.parametrize(
        "bad_patch",
        [
            {"unkown": "field"},
            {
                # bad base64 encoding for key
                "key": "PG15IGtleT4"
            },
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
        data, errors = BlockAccessSchema().load(bad_data)
        assert errors


class TestSyncedAccessSchema:
    ORIGINAL = {
        "id": "4f55b4d5b08544e2a784daf73754c7e2",
        "rts": "d6b769de2102482498b174245757fa5a",
        "wts": "02841ba1970044d8aed93784b956bc8f",
        "key": "PG15IGtleT4=\n",
    }

    def test_load_and_dump(self):
        loaded, errors = SyncedAccessSchema().load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "id": "4f55b4d5b08544e2a784daf73754c7e2",
            "rts": "d6b769de2102482498b174245757fa5a",
            "wts": "02841ba1970044d8aed93784b956bc8f",
            "key": b"<my key>",
        }
        dumped, errors = SyncedAccessSchema().dump(loaded)
        assert not errors
        assert dumped == self.ORIGINAL

    @pytest.mark.parametrize(
        "bad_patch",
        [
            {"unkown": "field"},
            {
                # bad base64 encoding for key
                "key": "PG15IGtleT4"
            },
            {"id": REMOVE_FIELD},
            {"key": REMOVE_FIELD},
            {"rts": REMOVE_FIELD},
            {"wts": REMOVE_FIELD},
            {"id": None},
            {"key": None},
            {"rts": None},
            {"wts": None},
            {
                # too long
                "id": "x"
                * 1000
            },
            {"rts": "x" * 1000},
            {"wts": "x" * 1000},
        ],
    )
    def test_bad_load(self, bad_patch):
        bad_data = {**self.ORIGINAL, **bad_patch}
        bad_data = {k: v for k, v in bad_data.items() if v is not REMOVE_FIELD}
        data, errors = SyncedAccessSchema().load(bad_data)
        assert errors


class TestFileManifestSchema:
    ORIGINAL = {
        "format": 1,
        "type": "file_manifest",
        "user_id": "alice",
        "device_name": "test",
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

    def test_load_and_dump(self):
        loaded, errors = FileManifestSchema().load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "format": 1,
            "type": "file_manifest",
            "user_id": "alice",
            "device_name": "test",
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
        dumped, errors = FileManifestSchema().dump(loaded)
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
        data, errors = FileManifestSchema().load(bad_data)
        assert errors


class TestFolderManifestSchema:
    ORIGINAL = {
        "format": 1,
        "type": "folder_manifest",
        "user_id": "alice",
        "device_name": "test",
        "version": 2,
        "created": "2017-01-01T00:00:00+00:00",
        "updated": "2017-12-31T23:59:59+00:00",
        "children": {
            "foo": {
                "id": "8aadbc777ece4a4fb5fa0564ecfbb54f",
                "rts": "9809c436b3af4fba9dd6955ad03e0310",
                "wts": "004714d9997147efa52a696127694fdc",
                "key": "PGZvbyBrZXk+\n",
            },
            "bar.txt": {
                "id": "51c865a60b194d9bb087df000056c299",
                "rts": "5a48035bdd7c4082b1101b28b6656d0c",
                "wts": "9b2ab384ed6b426daa1214f49587458a",
                "key": "PGJhci50eHQga2V5Pg==\n",
            },
        },
    }

    def test_load_and_dump(self):
        loaded, errors = FolderManifestSchema().load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "format": 1,
            "type": "folder_manifest",
            "user_id": "alice",
            "device_name": "test",
            "version": 2,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
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
        dumped, errors = FolderManifestSchema().dump(loaded)
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
        data, errors = FolderManifestSchema().load(bad_data)
        assert errors


class TestUserManifestSchema:
    # TODO
    pass


class TestTypedAccessSchema:
    ORIGINAL_VLOB = {
        "type": "vlob",
        "id": "4f55b4d5b08544e2a784daf73754c7e2",
        "rts": "d6b769de2102482498b174245757fa5a",
        "wts": "02841ba1970044d8aed93784b956bc8f",
        "key": "PG15IGtleT4=\n",
    }
    ORIGINAL_PLACEHOLDER = {
        "type": "placeholder",
        "id": "4f55b4d5b08544e2a784daf73754c7e2",
        "key": "PG15IGtleT4=\n",
    }

    def test_load_placeholder(self):
        loaded, errors = TypedAccessSchema().load(self.ORIGINAL_PLACEHOLDER)
        assert not errors
        assert loaded == {
            "type": "placeholder",
            "id": "4f55b4d5b08544e2a784daf73754c7e2",
            "key": b"<my key>",
        }
        dumped, errors = TypedAccessSchema().dump(loaded)
        assert not errors
        assert dumped == self.ORIGINAL_PLACEHOLDER

    def test_load_synced(self):
        loaded, errors = TypedAccessSchema().load(self.ORIGINAL_VLOB)
        assert not errors
        assert loaded == {
            "type": "vlob",
            "id": "4f55b4d5b08544e2a784daf73754c7e2",
            "rts": "d6b769de2102482498b174245757fa5a",
            "wts": "02841ba1970044d8aed93784b956bc8f",
            "key": b"<my key>",
        }
        dumped, errors = TypedAccessSchema().dump(loaded)
        assert not errors
        assert dumped == self.ORIGINAL_VLOB

    @pytest.mark.parametrize(
        "bad_patch",
        [
            {"unkown": "field"},
            {
                # bad base64 encoding for key
                "key": "PG15IGtleT4"
            },
            {"type": REMOVE_FIELD},
            {"type": None},
            {"id": REMOVE_FIELD},
            {"id": None},
            {"key": REMOVE_FIELD},
            {"key": None},
            {"type": "dummy"},
            {"type": "vlob"},
        ],
    )
    def test_bad_load_placeholder(self, bad_patch):
        bad_data = {**self.ORIGINAL_PLACEHOLDER, **bad_patch}
        bad_data = {k: v for k, v in bad_data.items() if v is not REMOVE_FIELD}
        data, errors = TypedAccessSchema().load(bad_data)
        assert errors

    @pytest.mark.parametrize(
        "bad_patch",
        [
            {"unkown": "field"},
            {
                # bad base64 encoding for key
                "key": "PG15IGtleT4"
            },
            {"type": REMOVE_FIELD},
            {"type": None},
            {"id": REMOVE_FIELD},
            {"id": None},
            {"key": REMOVE_FIELD},
            {"key": None},
            {"rts": REMOVE_FIELD},
            {"rts": None},
            {"wts": REMOVE_FIELD},
            {"wts": None},
            {"type": "dummy"},
            {"type": "placeholder"},
        ],
    )
    def test_bad_load_synced(self, bad_patch):
        bad_data = {**self.ORIGINAL_VLOB, **bad_patch}
        bad_data = {k: v for k, v in bad_data.items() if v is not REMOVE_FIELD}
        data, errors = TypedAccessSchema().load(bad_data)
        assert errors


class TestLocalFileManifestSchema:
    ORIGINAL = {
        "format": 1,
        "type": "local_file_manifest",
        "user_id": "alice",
        "device_name": "test",
        "base_version": 0,
        "need_sync": True,
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

    def test_load_and_dump(self):
        loaded, errors = LocalFileManifestSchema().load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "format": 1,
            "type": "local_file_manifest",
            "user_id": "alice",
            "device_name": "test",
            "base_version": 0,
            "need_sync": True,
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
        dumped, errors = LocalFileManifestSchema().dump(loaded)
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
        data, errors = LocalFileManifestSchema().load(bad_data)
        assert errors


class TestLocalFolderManifestSchema:
    ORIGINAL = {
        "format": 1,
        "type": "local_folder_manifest",
        "user_id": "alice",
        "device_name": "test",
        "base_version": 0,
        "need_sync": True,
        "created": "2017-01-01T00:00:00+00:00",
        "updated": "2017-12-31T23:59:59+00:00",
        "children": {
            "foo": {
                "type": "vlob",
                "id": "8aadbc777ece4a4fb5fa0564ecfbb54f",
                "rts": "9809c436b3af4fba9dd6955ad03e0310",
                "wts": "004714d9997147efa52a696127694fdc",
                "key": "PGZvbyBrZXk+\n",
            },
            "bar.txt": {
                "type": "placeholder",
                "id": "51c865a60b194d9bb087df000056c299",
                "key": "PGJhci50eHQga2V5Pg==\n",
            },
        },
    }

    def test_load_and_dump(self):
        loaded, errors = LocalFolderManifestSchema().load(self.ORIGINAL)
        assert not errors
        assert loaded == {
            "format": 1,
            "type": "local_folder_manifest",
            "user_id": "alice",
            "device_name": "test",
            "base_version": 0,
            "need_sync": True,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "children": {
                "foo": {
                    "type": "vlob",
                    "id": "8aadbc777ece4a4fb5fa0564ecfbb54f",
                    "rts": "9809c436b3af4fba9dd6955ad03e0310",
                    "wts": "004714d9997147efa52a696127694fdc",
                    "key": b"<foo key>",
                },
                "bar.txt": {
                    "type": "placeholder",
                    "id": "51c865a60b194d9bb087df000056c299",
                    "key": b"<bar.txt key>",
                },
            },
        }
        dumped, errors = LocalFolderManifestSchema().dump(loaded)
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
            {"type": "dummy"},
            {"base_version": -1},
            {"children": []},
        ],
    )
    def test_bad_load(self, bad_patch):
        bad_data = {**self.ORIGINAL, **bad_patch}
        bad_data = {k: v for k, v in bad_data.items() if v is not REMOVE_FIELD}
        data, errors = LocalFolderManifestSchema().load(bad_data)
        assert errors


class TestLocalUserManifestSchema:
    # TODO
    pass

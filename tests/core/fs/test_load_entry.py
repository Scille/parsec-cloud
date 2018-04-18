import pytest
from pendulum import datetime


@pytest.fixture
def access(fs):
    return fs._placeholder_access_cls(id="<foo id>", key=b"<foo key>")


@pytest.mark.trio
@pytest.mark.parametrize(
    "manifest",
    [
        {
            "format": 1,
            "type": "folder_manifest",
            "user_id": "alice",
            "device_name": "test",
            "version": 1,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "children": {},
        },
        {
            "format": 1,
            "type": "folder_manifest",
            "user_id": "alice",
            "device_name": "test",
            "version": 2,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "children": {
                "e1": {"id": "<e1 id>", "rts": "<e1 rts>", "wts": "<e1 wts>", "key": b"<e1 key>"},
                "e2": {"id": "<e2 id>", "rts": "<e2 rts>", "wts": "<e2 wts>", "key": b"<e2 key>"},
                "e3": {"id": "<e3 id>", "rts": "<e3 rts>", "wts": "<e3 wts>", "key": b"<e3 key>"},
            },
        },
        {
            "format": 1,
            "type": "local_folder_manifest",
            "user_id": "alice",
            "device_name": "test",
            "base_version": 0,
            "need_sync": True,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "children": {
                "e1": {
                    "type": "vlob",
                    "id": "<e1 id>",
                    "rts": "<e1 rts>",
                    "wts": "<e1 wts>",
                    "key": b"<e1 key>",
                },
                "e2": {"type": "placeholder", "id": "<e2 id>", "key": b"<e2 key>"},
                "e3": {
                    "type": "vlob",
                    "id": "<e3 id>",
                    "rts": "<e3 rts>",
                    "wts": "<e3 wts>",
                    "key": b"<e3 key>",
                },
            },
        },
    ],
)
async def test_load_folder(manifest, fs, access):
    entry = fs._load_entry(
        access=access,
        user_id="alice",
        device_name="test",
        name="foo",
        parent=None,
        manifest=manifest,
    )
    assert isinstance(entry, fs._folder_entry_cls)
    assert not entry._need_flush


@pytest.mark.trio
@pytest.mark.parametrize(
    "manifest",
    [
        {
            "format": 1,
            "type": "file_manifest",
            "user_id": "alice",
            "device_name": "test",
            "version": 2,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "blocks": [
                {
                    "id": "<block 1 id>",
                    "key": b"<block 1 key",
                    "offset": 0,
                    "size": 11,
                    "digest": b"<block 1 digest>",
                },
                {
                    "id": "<block 2 id>",
                    "key": b"<block 2 key",
                    "offset": 11,
                    "size": 16,
                    "digest": b"<block 2 digest>",
                },
            ],
            "size": 27,
        },
        {
            "format": 1,
            "type": "file_manifest",
            "user_id": "alice",
            "device_name": "test",
            "version": 1,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "blocks": [],
            "size": 0,
        },
        {
            "format": 1,
            "type": "local_file_manifest",
            "user_id": "alice",
            "device_name": "test",
            "need_sync": True,
            "base_version": 0,
            "created": datetime(2017, 1, 1),
            "updated": datetime(2017, 12, 31, 23, 59, 59),
            "blocks": [
                {
                    "id": "<block 1 id>",
                    "key": b"<block 1 key",
                    "offset": 0,
                    "size": 11,
                    "digest": b"<block 1 digest>",
                },
                {
                    "id": "<block 2 id>",
                    "key": b"<block 2 key",
                    "offset": 11,
                    "size": 16,
                    "digest": b"<block 2 digest>",
                },
            ],
            "dirty_blocks": [
                {
                    "id": "<block 3 id>",
                    "key": b"<block 3 key",
                    "offset": 0,
                    "size": 11,
                    "digest": b"<block 3 digest>",
                },
                {
                    "id": "<block 4 id>",
                    "key": b"<block 4 key",
                    "offset": 11,
                    "size": 16,
                    "digest": b"<block 4 digest>",
                },
            ],
            "size": 27,
        },
    ],
)
async def test_load_file(manifest, fs, access):
    entry = fs._load_entry(
        access=access,
        user_id="alice",
        device_name="test",
        name="foo",
        parent=None,
        manifest=manifest,
    )
    assert isinstance(entry, fs._file_entry_cls)
    assert not entry._need_flush

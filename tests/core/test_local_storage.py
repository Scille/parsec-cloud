import pytest


@pytest.mark.trio
async def test_fetch_user_manifest_not_available(local_storage):
    blob = local_storage.fetch_user_manifest()
    assert blob is None


@pytest.mark.trio
async def test_flush_and_fetch_user_manifest(local_storage):
    local_storage.flush_user_manifest(b"<user manifest>")
    blob = local_storage.fetch_user_manifest()
    assert blob == b"<user manifest>"


@pytest.mark.trio
async def test_fetch_manifest_not_available(local_storage):
    blob = local_storage.fetch_manifest("<unknown_id>")
    assert blob is None


@pytest.mark.trio
async def test_flush_and_fetch_manifest(local_storage):
    data = [("<id#%s>" % i, ("<manifest#%s>" % i).encode()) for i in range(3)]
    for id, blob in data:
        local_storage.flush_manifest(id, blob)
    for id, expected_blob in data:
        blob = local_storage.fetch_manifest(id)
        assert blob == expected_blob


@pytest.mark.trio
async def test_move_manifest_manifest(local_storage):
    local_storage.flush_manifest("<id#1>", b"<manifest>")
    local_storage.move_manifest("<id#1>", "<id#2>")
    old = local_storage.fetch_manifest("<id#1>")
    assert old is None
    new = local_storage.fetch_manifest("<id#2>")
    assert new == b"<manifest>"


@pytest.mark.trio
async def test_flush_and_fetch_user_pubkey(local_storage):
    data = [("<user_id#%s>" % i, ("<pubkey#%s>" % i).encode()) for i in range(3)]
    for user_id, pubkey in data:
        local_storage.flush_user_pubkey(user_id, pubkey)
    for user_id, expected_pubkey in data:
        verifykey = local_storage.fetch_user_pubkey(user_id)
        assert verifykey == expected_pubkey


@pytest.mark.trio
async def test_flush_and_fetch_device_verifykey(local_storage):
    data = [
        ("<user_id#%s>" % i, "<device_name#%s>" % i, ("<verifykey#%s>" % i).encode())
        for i in range(3)
    ]
    for user_id, device_name, verifykey in data:
        local_storage.flush_device_verifykey(user_id, device_name, verifykey)
    for user_id, device_name, expected_verifykey in data:
        verifykey = local_storage.fetch_device_verifykey(user_id, device_name)
        assert verifykey == expected_verifykey

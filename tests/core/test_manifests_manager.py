import pytest
import json
from json.decoder import JSONDecodeError
from nacl.encoding import Base64Encoder
from nacl.public import Box
from nacl.secret import SecretBox
from pendulum import datetime
from unittest.mock import Mock

from parsec.core.backend_storage import BackendStorage
from parsec.core.local_storage import LocalStorage
from parsec.core.manifests_manager import (
    ManifestsManager,
    ManifestDecryptionError,
    ManifestSignatureError,
)
from parsec.core.devices_manager import DevicesManager
from parsec.core.schemas import TypedManifestSchema
from tests.common import AsyncMock


@pytest.mark.usefixtures("alice_cleartext_device")
@pytest.fixture
def manifests_manager(alice_cleartext_device, tmpdir):
    local_storage = AsyncMock(spec=LocalStorage)
    backend_storage = AsyncMock(spec=BackendStorage)

    devices_manager = DevicesManager(str(tmpdir))
    alice_device = devices_manager.load_device(alice_cleartext_device)
    backend_connection = AsyncMock()
    backend_connection.send.is_async = True
    backend_connection.send.side_effect = [
        {
            "status": "ok",
            "devices": {
                "test": {
                    "verify_key": alice_device.device_signkey.verify_key.encode(
                        encoder=Base64Encoder
                    ).decode()
                }
            },
        }
    ]

    device = devices_manager.load_device(alice_cleartext_device)
    return ManifestsManager(device, local_storage, backend_storage, backend_connection)


@pytest.fixture
def original_manifest():
    return TypedManifestSchema(strict=False).dump(
        {
            "type": "folder_manifest",
            "user_id": "alice",
            "device_name": "test",
            "children": {},
            "size": 0,
            "version": 1,
            "updated": datetime(2017, 1, 1),
            "created": datetime(2017, 1, 1),
            "format": 1,
        }
    ).data


@pytest.mark.usefixtures("bob_cleartext_device")
@pytest.fixture
def user_manifests_samples(
    manifests_manager, original_manifest, alice_cleartext_device, bob_cleartext_device, tmpdir
):
    manifest_raw = json.dumps(original_manifest).encode()
    # Decryption error
    devices_manager = DevicesManager(str(tmpdir))
    bob_device = devices_manager.load_device(bob_cleartext_device)
    bob_box = Box(bob_device.user_privkey, bob_device.user_pubkey)
    signed = bob_device.device_signkey.sign(manifest_raw)
    decryption_error_manifest = bob_box.encrypt(signed)
    # Signature error
    alice_box = Box(manifests_manager.device.user_privkey, manifests_manager.device.user_pubkey)
    signed = bob_device.device_signkey.sign(manifest_raw)
    signature_error_manifest = alice_box.encrypt(signed)
    # Json decode error
    alice_box = Box(manifests_manager.device.user_privkey, manifests_manager.device.user_pubkey)
    signed = manifests_manager.device.device_signkey.sign(b"foo")
    json_error_manifest = alice_box.encrypt(signed)
    # Ok
    signed = manifests_manager.device.device_signkey.sign(manifest_raw)
    valid_manifest = alice_box.encrypt(signed)
    return {
        "decryption_error": (decryption_error_manifest, ManifestDecryptionError),
        "signature_error": (signature_error_manifest, ManifestSignatureError),
        "json_error": (json_error_manifest, JSONDecodeError),
        "valid": valid_manifest,
    }


@pytest.mark.usefixtures("bob_cleartext_device")
@pytest.fixture
def manifests_samples(manifests_manager, original_manifest, bob_cleartext_device, tmpdir):
    manifest_raw = json.dumps(original_manifest).encode()
    # Decryption error
    devices_manager = DevicesManager(str(tmpdir))
    bob_device = devices_manager.load_device(bob_cleartext_device)
    bob_box = SecretBox(b"b" * 32)
    signed = bob_device.device_signkey.sign(manifest_raw)
    decryption_error_manifest = bob_box.encrypt(signed)
    # Signature error
    alice_box = SecretBox(b"a" * 32)
    signed = bob_device.device_signkey.sign(manifest_raw)
    signature_error_manifest = alice_box.encrypt(signed)
    # Json decode error
    alice_box = SecretBox(b"a" * 32)
    signed = manifests_manager.device.device_signkey.sign(b"foo")
    json_error_manifest = alice_box.encrypt(signed)
    # Ok
    signed = manifests_manager.device.device_signkey.sign(manifest_raw)
    valid_manifest = alice_box.encrypt(signed)
    return {
        "decryption_error": (decryption_error_manifest, ManifestDecryptionError),
        "signature_error": (signature_error_manifest, ManifestSignatureError),
        "json_error": (json_error_manifest, JSONDecodeError),
        "valid": valid_manifest,
    }


@pytest.mark.trio
async def test_manifests_manager_fetch_user_manifest_from_backend_empty(manifests_manager):
    manifests_manager._backend_storage.fetch_user_manifest.return_value = None
    user_manifest = await manifests_manager.fetch_user_manifest_from_backend(3)
    manifests_manager._backend_storage.fetch_user_manifest.assert_called_with(version=3)
    assert user_manifest is None


@pytest.mark.trio
async def test_manifests_manager_fetch_manifest_from_backend_empty(manifests_manager):
    manifests_manager._backend_storage.fetch_manifest.return_value = None
    manifest = await manifests_manager.fetch_from_backend("id", "rts", "key", 3)
    manifests_manager._backend_storage.fetch_manifest.assert_called_with("id", "rts", version=3)
    assert manifest is None


@pytest.mark.trio
async def test_manifests_manager_fetch_user_manifest_from_local_empty(manifests_manager):
    manifests_manager._local_storage.fetch_user_manifest.return_value = None
    user_manifest = await manifests_manager.fetch_user_manifest_from_local()
    manifests_manager._local_storage.fetch_user_manifest.assert_called_with()
    assert user_manifest is None


@pytest.mark.trio
async def test_manifests_manager_fetch_manifest_from_local_empty(manifests_manager):
    manifests_manager._local_storage.fetch_manifest.return_value = None
    manifest = await manifests_manager.fetch_from_local("id", "key")
    manifests_manager._local_storage.fetch_manifest.assert_called_with("id")
    assert manifest is None


@pytest.mark.usefixtures("bob_cleartext_device")
@pytest.mark.parametrize("location", ["local", "backend"])
@pytest.mark.trio
async def test_manifests_manager_fetch_user_manifest_invalid(
    manifests_manager, location, user_manifests_samples
):
    if location == "local":
        storage = manifests_manager._local_storage
        fetch_method = manifests_manager.fetch_user_manifest_from_local
    else:
        storage = manifests_manager._backend_storage
        fetch_method = manifests_manager.fetch_user_manifest_from_backend
    for manifest_sample_type, sample in user_manifests_samples.items():
        if manifest_sample_type == "valid":
            continue

        manifest, exception = sample
        storage.fetch_user_manifest.return_value = manifest
        with pytest.raises(exception):
            await fetch_method()


@pytest.mark.trio
@pytest.mark.parametrize("location", ["local", "backend"])
async def test_manifests_manager_fetch_user_manifest(
    manifests_manager, location, original_manifest, user_manifests_samples
):
    if location == "local":
        storage = manifests_manager._local_storage
        fetch_method = manifests_manager.fetch_user_manifest_from_local
    else:
        storage = manifests_manager._backend_storage
        fetch_method = manifests_manager.fetch_user_manifest_from_backend
    storage.fetch_user_manifest.return_value = user_manifests_samples["valid"]
    retrieved_manifest = await fetch_method()
    assert TypedManifestSchema(strict=False).dump(retrieved_manifest).data == original_manifest


@pytest.mark.usefixtures("bob_cleartext_device")
@pytest.mark.parametrize("location", ["local", "backend"])
@pytest.mark.trio
async def test_manifests_manager_fetch_manifest_invalid(
    manifests_manager, location, manifests_samples
):
    if location == "local":
        storage = manifests_manager._local_storage
        fetch_method = manifests_manager.fetch_from_local
    else:
        storage = manifests_manager._backend_storage
        fetch_method = manifests_manager.fetch_from_backend
    for manifest_sample_type, sample in manifests_samples.items():
        if manifest_sample_type == "valid":
            continue

        manifest, exception = sample
        storage.fetch_manifest.return_value = manifest
        with pytest.raises(exception):
            if location == "local":
                await fetch_method("id", b"a" * 32)
            else:
                await fetch_method("id", "rts", b"a" * 32)


@pytest.mark.usefixtures("bob_cleartext_device")
@pytest.mark.trio
@pytest.mark.parametrize("location", ["local", "backend"])
async def test_manifests_manager_fetch_manifest_belonging_to_other_user(
    manifests_manager, location, original_manifest, manifests_samples, bob_cleartext_device, tmpdir
):
    devices_manager = DevicesManager(str(tmpdir))
    bob_device = devices_manager.load_device(bob_cleartext_device)
    manifests_manager.device = bob_device
    if location == "local":
        storage = manifests_manager._local_storage
        fetch_method = manifests_manager.fetch_from_local
    else:
        storage = manifests_manager._backend_storage
        fetch_method = manifests_manager.fetch_from_backend
    storage.fetch_manifest.return_value = manifests_samples["valid"]
    if location == "local":
        retrieved_manifest = await fetch_method("id", b"a" * 32)
    else:
        retrieved_manifest = await fetch_method("id", "rts", b"a" * 32)
    assert TypedManifestSchema(strict=False).dump(retrieved_manifest).data == original_manifest


@pytest.mark.trio
@pytest.mark.parametrize("location", ["local", "backend"])
async def test_manifests_manager_fetch_manifest_belonging_to_logged_user(
    manifests_manager, location, original_manifest, manifests_samples
):
    if location == "local":
        storage = manifests_manager._local_storage
        fetch_method = manifests_manager.fetch_from_local
    else:
        storage = manifests_manager._backend_storage
        fetch_method = manifests_manager.fetch_from_backend
    storage.fetch_manifest.return_value = manifests_samples["valid"]
    if location == "local":
        retrieved_manifest = await fetch_method("id", b"a" * 32)
    else:
        retrieved_manifest = await fetch_method("id", "rts", b"a" * 32)
    assert TypedManifestSchema(strict=False).dump(retrieved_manifest).data == original_manifest


@pytest.mark.usefixtures("alice_cleartext_device")
@pytest.mark.trio
async def test_manifests_manager_flush_user_manifest_on_local(
    alice_cleartext_device, manifests_manager, original_manifest, tmpdir
):
    manifest, _ = TypedManifestSchema(strict=True).load(original_manifest)
    await manifests_manager.flush_user_manifest_on_local(manifest)
    assert len(manifests_manager._local_storage.flush_user_manifest.call_args_list) == 1
    dumped = manifests_manager._local_storage.flush_user_manifest.call_args_list[0][0][0]
    assert isinstance(dumped, bytes)
    # Reload the manifest
    local_storage = Mock()
    local_storage.fetch_user_manifest.return_value = dumped
    backend_storage = Mock()
    devices_manager = DevicesManager(str(tmpdir))
    device = devices_manager.load_device(alice_cleartext_device)
    backend_connection = Mock()
    manifests_manager2 = ManifestsManager(
        device, local_storage, backend_storage, backend_connection
    )
    manifest2 = await manifests_manager2.fetch_user_manifest_from_local()
    assert manifest == manifest2


@pytest.mark.usefixtures("alice_cleartext_device")
@pytest.mark.trio
async def test_manifests_manager_flush_on_local(
    alice_cleartext_device, manifests_manager, original_manifest, tmpdir
):
    manifest, _ = TypedManifestSchema(strict=True).load(original_manifest)
    await manifests_manager.flush_on_local("id", b"a" * 32, manifest)
    assert len(manifests_manager._local_storage.flush_manifest.call_args_list) == 1
    dumped = manifests_manager._local_storage.flush_manifest.call_args_list[0][0][1]
    assert isinstance(dumped, bytes)
    # Reload the manifest
    local_storage = Mock()
    local_storage.fetch_manifest.return_value = dumped
    backend_storage = Mock()
    devices_manager = DevicesManager(str(tmpdir))
    device = devices_manager.load_device(alice_cleartext_device)
    backend_connection = Mock()
    manifests_manager2 = ManifestsManager(
        device, local_storage, backend_storage, backend_connection
    )
    manifest2 = await manifests_manager2.fetch_from_local("id", b"a" * 32)
    assert manifest == manifest2


@pytest.mark.usefixtures("alice_cleartext_device")
@pytest.mark.trio
async def test_manifests_manager_sync_user_manifest_with_backend(
    alice_cleartext_device, manifests_manager, original_manifest, tmpdir
):
    manifest, _ = TypedManifestSchema(strict=True).load(original_manifest)
    await manifests_manager.sync_user_manifest_with_backend(manifest)
    assert len(manifests_manager._backend_storage.sync_user_manifest.call_args_list) == 1
    version = manifests_manager._backend_storage.sync_user_manifest.call_args_list[0][0][0]
    dumped = manifests_manager._backend_storage.sync_user_manifest.call_args_list[0][0][1]
    assert version == manifest["version"]
    assert isinstance(dumped, bytes)
    # Reload the manifest
    local_storage = Mock()
    local_storage.fetch_user_manifest.return_value = dumped
    backend_storage = Mock()
    devices_manager = DevicesManager(str(tmpdir))
    device = devices_manager.load_device(alice_cleartext_device)
    backend_connection = Mock()
    manifests_manager2 = ManifestsManager(
        device, local_storage, backend_storage, backend_connection
    )
    manifest2 = await manifests_manager2.fetch_user_manifest_from_local()
    assert manifest == manifest2


@pytest.mark.usefixtures("alice_cleartext_device")
@pytest.mark.trio
async def test_manifests_manager_sync_new_entry_with_backend(
    alice_cleartext_device, manifests_manager, original_manifest, tmpdir
):
    manifest, _ = TypedManifestSchema(strict=True).load(original_manifest)
    await manifests_manager.sync_new_entry_with_backend(b"a" * 32, manifest)
    assert len(manifests_manager._backend_storage.sync_new_manifest.call_args_list) == 1
    dumped = manifests_manager._backend_storage.sync_new_manifest.call_args_list[0][0][0]
    assert isinstance(dumped, bytes)
    # Reload the manifest
    local_storage = Mock()
    local_storage.fetch_manifest.return_value = dumped
    backend_storage = Mock()
    devices_manager = DevicesManager(str(tmpdir))
    device = devices_manager.load_device(alice_cleartext_device)
    backend_connection = Mock()
    manifests_manager2 = ManifestsManager(
        device, local_storage, backend_storage, backend_connection
    )
    manifest2 = await manifests_manager2.fetch_from_local("id", b"a" * 32)
    assert manifest == manifest2


@pytest.mark.usefixtures("alice_cleartext_device")
@pytest.mark.trio
async def test_manifests_manager_sync_with_backend(
    alice_cleartext_device, manifests_manager, original_manifest, tmpdir
):
    manifest, _ = TypedManifestSchema(strict=True).load(original_manifest)
    await manifests_manager.sync_with_backend("id", "wts", b"a" * 32, manifest)
    assert len(manifests_manager._backend_storage.sync_manifest.call_args_list) == 1
    id = manifests_manager._backend_storage.sync_manifest.call_args_list[0][0][0]
    wts = manifests_manager._backend_storage.sync_manifest.call_args_list[0][0][1]
    version = manifests_manager._backend_storage.sync_manifest.call_args_list[0][0][2]
    dumped = manifests_manager._backend_storage.sync_manifest.call_args_list[0][0][3]
    assert id == "id"
    assert wts == "wts"
    assert version == 1
    assert isinstance(dumped, bytes)
    # Reload the manifest
    local_storage = Mock()
    local_storage.fetch_manifest.return_value = dumped
    backend_storage = Mock()
    devices_manager = DevicesManager(str(tmpdir))
    device = devices_manager.load_device(alice_cleartext_device)
    backend_connection = Mock()
    manifests_manager2 = ManifestsManager(
        device, local_storage, backend_storage, backend_connection
    )
    manifest2 = await manifests_manager2.fetch_from_local("id", b"a" * 32)
    assert manifest == manifest2

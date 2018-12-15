import pytest

from parsec.core.backend_connection import BackendNotAvailable

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.fixture(autouse=True, scope="module")
def realcrypto(unmock_crypto):
    with unmock_crypto():
        yield


@pytest.mark.trio
async def test_encryption_manager_fetch_remote_device_local_cache(
    tcp_stream_spy, running_backend, backend_addr, encryption_manager, bob
):
    with pytest.raises(BackendNotAvailable):
        with offline(backend_addr):
            await encryption_manager.fetch_remote_device(bob.device_id)

    remote_bob = await encryption_manager.fetch_remote_device(bob.device_id)
    assert remote_bob.verify_key == bob.verify_key

    with offline(backend_addr):
        remot_bob_offline = await encryption_manager.fetch_remote_device(bob.device_id)
    assert remot_bob_offline == remote_bob


@pytest.mark.trio
async def test_encryption_manager_fetch_remote_user_local_cache(
    tcp_stream_spy, running_backend, backend_addr, encryption_manager, bob
):
    with pytest.raises(BackendNotAvailable):
        with offline(backend_addr):
            await encryption_manager.fetch_remote_user(bob.user_id)

    remote_bob = await encryption_manager.fetch_remote_user(bob.user_id)
    assert remote_bob.public_key == bob.public_key

    with offline(backend_addr):
        remot_bob_offline = await encryption_manager.fetch_remote_user(bob.user_id)
    assert remot_bob_offline == remote_bob


@pytest.mark.trio
async def test_encryption_manager_fetch_self_device_offline(encryption_manager, alice):
    remote_device = await encryption_manager.fetch_remote_device(alice.device_id)
    assert remote_device.verify_key == alice.verify_key


@pytest.mark.trio
async def test_encryption_manager_fetch_self_offline(encryption_manager, alice):
    remote_user = await encryption_manager.fetch_remote_user(alice.user_id)
    assert remote_user.public_key == alice.public_key


# TODO: test bad trustchain

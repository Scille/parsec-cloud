# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from collections import namedtuple

import pytest
from pendulum import Pendulum

from parsec.core.remote_devices_manager import (
    RemoteDevicesManagerBackendOfflineError,
    RemoteDevicesManagerInvalidTrustchainError,
    UnverifiedRemoteDevice,
    UnverifiedRemoteUser,
    VerifiedRemoteDevice,
    VerifiedRemoteUser,
    _verify_devices,
    _verify_user,
)
from parsec.crypto import (
    build_device_certificate,
    build_revoked_device_certificate,
    build_user_certificate,
)
from tests.common import freeze_time


@pytest.mark.trio
async def test_retrieve_device(running_backend, alice_remote_devices_manager, bob):
    remote_devices_manager = alice_remote_devices_manager
    d1 = Pendulum(2000, 1, 1)
    with freeze_time(d1):
        # Offline with no cache
        with pytest.raises(RemoteDevicesManagerBackendOfflineError):
            with running_backend.offline():
                await remote_devices_manager.get_device(bob.device_id)

        # Online
        device = await remote_devices_manager.get_device(bob.device_id)
        assert device.device_id == bob.device_id
        assert device.verify_key == bob.verify_key

        # Offline with cache
        with running_backend.offline():
            device2 = await remote_devices_manager.get_device(bob.device_id)
            assert device2 is device

    d2 = d1.add(remote_devices_manager.cache_validity + 1)
    with freeze_time(d2):
        # Offline with cache expired
        with pytest.raises(RemoteDevicesManagerBackendOfflineError):
            with running_backend.offline():
                await remote_devices_manager.get_device(bob.device_id)

        # Online with cache expired
        device = await remote_devices_manager.get_device(bob.device_id)
        assert device.device_id == bob.device_id
        assert device.verify_key == bob.verify_key


@pytest.mark.trio
async def test_retrieve_user(running_backend, alice_remote_devices_manager, bob):
    remote_devices_manager = alice_remote_devices_manager
    d1 = Pendulum(2000, 1, 1)
    with freeze_time(d1):
        # Offline with no cache
        with pytest.raises(RemoteDevicesManagerBackendOfflineError):
            with running_backend.offline():
                await remote_devices_manager.get_user(bob.user_id)

        # Online
        user = await remote_devices_manager.get_user(bob.user_id)
        assert user.user_id == bob.user_id
        assert user.public_key == bob.public_key

        # Offline with cache
        with running_backend.offline():
            user2 = await remote_devices_manager.get_user(bob.user_id)
            assert user2 is user

    d2 = d1.add(remote_devices_manager.cache_validity + 1)
    with freeze_time(d2):
        # Offline with cache expired
        with pytest.raises(RemoteDevicesManagerBackendOfflineError):
            with running_backend.offline():
                await remote_devices_manager.get_user(bob.user_id)

        # Online with cache expired
        user = await remote_devices_manager.get_user(bob.user_id)
        assert user.user_id == bob.user_id
        assert user.public_key == bob.public_key


@pytest.mark.trio
async def test_retrieve_user_and_devices(
    running_backend, alice_remote_devices_manager, alice, alice2
):
    remote_devices_manager = alice_remote_devices_manager
    d1 = Pendulum(2000, 1, 1)
    with freeze_time(d1):
        # Offline
        with pytest.raises(RemoteDevicesManagerBackendOfflineError):
            with running_backend.offline():
                await remote_devices_manager.get_user_and_devices(alice.user_id)

        # Online
        user, devices = await remote_devices_manager.get_user_and_devices(alice.user_id)
        assert user.user_id == alice.user_id
        assert user.public_key == alice.public_key
        assert len(devices) == 2
        assert {d.device_id for d in devices} == {alice.device_id, alice2.device_id}

        # Offline (cache is never used)
        with pytest.raises(RemoteDevicesManagerBackendOfflineError):
            with running_backend.offline():
                await remote_devices_manager.get_user_and_devices(alice.user_id)


@pytest.fixture
def trustchain_ctx_factory(local_device_factory, coolorg):
    now = Pendulum(2000, 1, 1)
    TrustChainCtx = namedtuple(
        "TrustChainCtx",
        "organization_id,root_verify_key,remote_user,trustchain,remote_devices,local_devices",
    )

    def _trustchain_ctx_factory(todo_devices, todo_user):
        local_devices = {}
        remote_devices = {}
        remote_user = None

        def _get_certifier_id_and_key(certifier):
            if not certifier:
                # Certified by root
                certifier_id = None
                certifier_key = coolorg.root_signing_key
            else:
                certifier_ld = local_devices.get(certifier)
                if not certifier_ld:
                    raise RuntimeError(f"Missing `{certifier}` to sign creation of `{todo_device}`")
                certifier_id = certifier_ld.device_id
                certifier_key = certifier_ld.signing_key
            return certifier_id, certifier_key

        # First create all the devices (and indirectly the needed users)
        for todo_device in todo_devices:
            local_device = local_device_factory(todo_device["id"], org=coolorg)
            certifier_id, certifier_key = _get_certifier_id_and_key(todo_device.get("certifier"))
            created_on = todo_device.get("created_on", now)
            revoker = todo_device.get("revoker", None)
            if revoker:
                revoked_on = todo_device.get("revoked_on", now)

            device_certificate = build_device_certificate(
                certifier_id,
                certifier_key,
                local_device.device_id,
                local_device.verify_key,
                created_on,
            )

            revoked_device_certificate = None
            if revoker:
                revoker_ld = local_devices.get(revoker)
                if not revoker_ld:
                    raise RuntimeError(f"Missing `{revoker}` to sign revocation of `{todo_device}`")
                revoker_id = revoker_ld.device_id
                revoker_key = revoker_ld.signing_key
                revoked_device_certificate = build_revoked_device_certificate(
                    revoker_id, revoker_key, local_device.device_id, revoked_on
                )

            remote_devices[local_device.device_id] = UnverifiedRemoteDevice(
                device_certificate, revoked_device_certificate
            )
            local_devices[str(local_device.device_id)] = local_device

        # Now deal with the user
        local_user = next((u for u in local_devices.values() if str(u.user_id) == todo_user["id"]))
        certifier_id, certifier_key = _get_certifier_id_and_key(todo_user.get("certifier"))
        created_on = todo_user.get("created_on", now)
        user_certif = build_user_certificate(
            certifier_id=certifier_id,
            certifier_key=certifier_key,
            user_id=local_user.user_id,
            public_key=local_user.public_key,
            is_admin=False,
            timestamp=created_on,
        )

        remote_user = UnverifiedRemoteUser(user_certificate=user_certif)
        trustchain = {k: v for k, v in remote_devices.items() if k.user_id != local_user.user_id}

        return TrustChainCtx(
            coolorg.organization_id,
            coolorg.root_verify_key,
            remote_user,
            trustchain,
            remote_devices,
            local_devices,
        )

    return _trustchain_ctx_factory


def test_verify_user_with_trustchain_ok(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "alice@dev2", "certifier": "alice@dev1"},
            {"id": "alice@dev3", "certifier": "alice@dev2"},
        ),
        todo_user={"id": "alice"},
    )

    verified_devices = _verify_devices(ctx.root_verify_key, *ctx.remote_devices.values())
    assert verified_devices.keys() == {"alice@dev1", "alice@dev2", "alice@dev3"}
    for vd in verified_devices.values():
        assert isinstance(vd, VerifiedRemoteDevice)

    verified_user = _verify_user(ctx.root_verify_key, ctx.remote_user, verified_devices)
    assert isinstance(verified_user, VerifiedRemoteUser)
    assert verified_user.user_id == "alice"


def test_verify_user_with_trustchain_ok_with_user_certifier(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "bob@dev1"},
            {"id": "alice@dev2", "certifier": "alice@dev1"},
            {"id": "alice@dev3", "certifier": "alice@dev2"},
        ),
        todo_user={"id": "alice", "certifier": "bob@dev1"},
    )

    verified_devices = _verify_devices(ctx.root_verify_key, *ctx.remote_devices.values())
    assert verified_devices.keys() == {"alice@dev1", "alice@dev2", "alice@dev3", "bob@dev1"}

    verified_user = _verify_user(ctx.root_verify_key, ctx.remote_user, verified_devices)
    assert verified_user.user_id == "alice"


def test_verify_user_with_trustchain_with_user_created_by_revoked_certifier(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "alice@dev2", "certifier": "alice@dev1"},
            {"id": "mallory@dev1", "revoker": "alice@dev2", "revoked_on": Pendulum(2000, 1, 3)},
            {"id": "bob@dev1", "revoker": "mallory@dev1", "revoked_on": Pendulum(2000, 1, 2)},
            {"id": "alice@dev3", "certifier": "alice@dev2"},
        ),
        todo_user={"id": "alice", "certifier": "bob@dev1"},
    )

    verified_devices = _verify_devices(ctx.root_verify_key, *ctx.remote_devices.values())
    assert verified_devices.keys() == {
        "alice@dev1",
        "alice@dev2",
        "alice@dev3",
        "bob@dev1",
        "mallory@dev1",
    }

    verified_user = _verify_user(ctx.root_verify_key, ctx.remote_user, verified_devices)
    assert verified_user.user_id == "alice"


def test_verify_user_with_trustchain_with_revoked_user(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "alice@dev2", "certifier": "alice@dev1"},
            {"id": "mallory@dev1", "revoker": "alice@dev2", "revoked_on": Pendulum(2000, 1, 3)},
            {"id": "bob@dev1", "revoker": "mallory@dev1", "revoked_on": Pendulum(2000, 1, 2)},
            {"id": "alice@dev3", "certifier": "bob@dev1"},
        ),
        todo_user={"id": "alice"},
    )

    verified_devices = _verify_devices(ctx.root_verify_key, *ctx.remote_devices.values())
    assert verified_devices.keys() == {
        "alice@dev1",
        "alice@dev2",
        "alice@dev3",
        "bob@dev1",
        "mallory@dev1",
    }

    verified_user = _verify_user(ctx.root_verify_key, ctx.remote_user, verified_devices)
    assert verified_user.user_id == "alice"


def test_verify_user_with_trustchain_broken_chain(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "alice@dev2", "certifier": "alice@dev1"},
            {"id": "mallory@dev1", "revoker": "alice@dev2", "revoked_on": Pendulum(2000, 1, 3)},
            {"id": "bob@dev1", "revoker": "mallory@dev1", "revoked_on": Pendulum(2000, 1, 2)},
            {"id": "alice@dev3", "certifier": "bob@dev1"},
        ),
        todo_user={"id": "alice", "certifier": "mallory@dev1"},
    )

    verified_devices = _verify_devices(ctx.root_verify_key, *ctx.remote_devices.values())

    verified_devices.pop("mallory@dev1")
    with pytest.raises(RemoteDevicesManagerInvalidTrustchainError) as exc:
        _verify_user(ctx.root_verify_key, ctx.remote_user, verified_devices)
    assert exc.value.args == ("`alice` <-create- `mallory@dev1`: Device not provided by backend",)

    ctx.remote_devices.pop("mallory@dev1")
    with pytest.raises(RemoteDevicesManagerInvalidTrustchainError) as exc:
        _verify_devices(ctx.root_verify_key, *ctx.remote_devices.values())
    assert exc.value.args == (
        "`bob@dev1` <-revoke- `mallory@dev1`: Device not provided by backend",
    )


def test_verify_user_with_trustchain_with_invalid_device_creation(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "bob@dev1"},
            {"id": "bob@dev2", "revoker": "bob@dev1", "revoked_on": Pendulum(2000, 1, 2)},
            # Bob already revoked when it was supposed to create alice
            {"id": "alice@dev1", "certifier": "bob@dev2", "created_on": Pendulum(2000, 1, 3)},
        ),
        todo_user={"id": "alice"},
    )

    with pytest.raises(RemoteDevicesManagerInvalidTrustchainError) as exc:
        _verify_devices(ctx.root_verify_key, *ctx.remote_devices.values())
    assert exc.value.args == (
        "`alice@dev1` <-create- `bob@dev2`: Signature (2000-01-03T00:00:00+00:00)"
        " is posterior to device revocation 2000-01-02T00:00:00+00:00)",
    )


def test_verify_user_with_trustchain_with_invalid_user_creation(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "bob@dev1"},
            {"id": "bob@dev2", "revoker": "bob@dev1", "revoked_on": Pendulum(2000, 1, 2)},
            {"id": "alice@dev1"},
        ),
        # Bob already revoked when it was supposed to create alice
        todo_user={"id": "alice", "certifier": "bob@dev2", "created_on": Pendulum(2000, 1, 3)},
    )

    verified_devices = _verify_devices(ctx.root_verify_key, *ctx.remote_devices.values())

    with pytest.raises(RemoteDevicesManagerInvalidTrustchainError) as exc:
        _verify_user(ctx.root_verify_key, ctx.remote_user, verified_devices)
    assert exc.value.args == (
        "`alice` <-create- `bob@dev2`: Signature (2000-01-03T00:00:00+00:00)"
        " is posterior to device revocation 2000-01-02T00:00:00+00:00)",
    )


def test_verify_user_with_trustchain_with_invalid_revocation(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "bob@dev1"},
            {"id": "bob@dev2", "revoker": "bob@dev1", "revoked_on": Pendulum(2000, 1, 2)},
            # Bob already revoked when it was supposed to revoke alice
            {"id": "alice@dev1", "revoker": "bob@dev2", "revoked_on": Pendulum(2000, 1, 3)},
        ),
        todo_user={"id": "alice"},
    )

    with pytest.raises(RemoteDevicesManagerInvalidTrustchainError) as exc:
        _verify_devices(ctx.root_verify_key, *ctx.remote_devices.values())
    assert exc.value.args == (
        "`alice@dev1` <-revoke- `bob@dev2`: Signature (2000-01-03T00:00:00+00:00)"
        " is posterior to device revocation 2000-01-02T00:00:00+00:00)",
    )

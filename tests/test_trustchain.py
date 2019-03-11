# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum
from collections import namedtuple

from parsec.core.types.remote_device import RemoteDevice, RemoteDevicesMapping, RemoteUser
from parsec.trustchain import (
    TrustChainError,
    TrustChainBrokenChainError,
    TrustChainCertifServerMismatchError,
    TrustChainSignedByRevokedDeviceError,
    certified_extract_parts,
    validate_payload_certified_user,
    certify_device,
    certify_device_revocation,
    certify_user,
    validate_payload_certified_device,
    validate_user_with_trustchain,
    MAX_TS_BALLPARK,
)
from parsec.types import DeviceID


def test_bad_certified_extract_parts():
    with pytest.raises(TrustChainError):
        certified_extract_parts(b"")


def test_certify_user(alice, mallory):
    now = Pendulum(2000, 1, 1)

    certification = certify_user(
        alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key, now=now
    )
    assert isinstance(certification, bytes)

    certifier, payload = certified_extract_parts(certification)
    assert certifier == alice.device_id
    assert isinstance(payload, bytes)

    data = validate_payload_certified_user(alice.verify_key, payload, now)
    assert data == {
        "type": "user",
        "user_id": mallory.user_id,
        "public_key": mallory.public_key,
        "timestamp": now,
    }


def test_validate_bad_certified_user(alice):
    now = Pendulum(2000, 1, 1)
    with pytest.raises(TrustChainError):
        validate_payload_certified_user(alice.verify_key, b"", now)


def test_validate_certified_user_bad_certifier(alice, bob, mallory):
    now = Pendulum(2000, 1, 1)
    certification = certify_user(
        alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key, now
    )
    certifier, payload = certified_extract_parts(certification)

    with pytest.raises(TrustChainError):
        validate_payload_certified_user(bob.verify_key, payload, now)


def test_validate_certified_user_too_old(alice, bob, mallory):
    now = Pendulum(2000, 1, 1)
    certification = certify_user(
        alice.device_id, alice.signing_key, mallory.user_id, mallory.public_key, now
    )

    certifier, payload = certified_extract_parts(certification)
    now = now.add(seconds=MAX_TS_BALLPARK + 1)
    with pytest.raises(TrustChainError):
        validate_payload_certified_user(alice.verify_key, payload, now)


def test_certify_device(alice, mallory):
    now = Pendulum(2000, 1, 1)

    certification = certify_device(
        alice.device_id, alice.signing_key, mallory.device_id, mallory.verify_key, now
    )
    assert isinstance(certification, bytes)

    certifier, payload = certified_extract_parts(certification)
    assert certifier == alice.device_id
    assert isinstance(payload, bytes)

    data = validate_payload_certified_device(alice.verify_key, payload, now)
    assert data == {
        "type": "device",
        "device_id": mallory.device_id,
        "verify_key": mallory.verify_key,
        "timestamp": now,
    }


def test_validate_bad_certified_device(alice):
    now = Pendulum(2000, 1, 1)
    with pytest.raises(TrustChainError):
        validate_payload_certified_device(alice.verify_key, b"", now)


def test_validate_certified_device_bad_certifier(alice, bob, mallory):
    now = Pendulum(2000, 1, 1)
    certification = certify_device(
        alice.device_id, alice.signing_key, mallory.device_id, mallory.signing_key, now
    )
    certifier, payload = certified_extract_parts(certification)

    with pytest.raises(TrustChainError):
        validate_payload_certified_device(bob.verify_key, payload, now)


def test_validate_certified_device_too_old(alice, bob, mallory):
    now = Pendulum(2000, 1, 1)
    certification = certify_device(
        alice.device_id, alice.signing_key, mallory.device_id, mallory.signing_key, now
    )

    certifier, payload = certified_extract_parts(certification)
    now = now.add(seconds=MAX_TS_BALLPARK + 1)
    with pytest.raises(TrustChainError):
        validate_payload_certified_device(alice.verify_key, payload, now)


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

            data = {"device_id": local_device.device_id}

            data["certified_device"] = certify_device(
                certifier_id,
                certifier_key,
                local_device.device_id,
                local_device.verify_key,
                now=created_on,
            )
            data["created_on"] = created_on
            data["device_certifier"] = certifier_id

            if revoker:
                revoker_ld = local_devices.get(revoker)
                if not revoker_ld:
                    raise RuntimeError(f"Missing `{revoker}` to sign revocation of `{todo_device}`")
                revoker_id = revoker_ld.device_id
                revoker_key = revoker_ld.signing_key
                data["certified_revocation"] = certify_device_revocation(
                    revoker_id, revoker_key, local_device.device_id, now=revoked_on
                )
                data["revoked_on"] = revoked_on
                data["revocation_certifier"] = revoker_id

            remote_devices[local_device.device_id] = RemoteDevice(**data)
            local_devices[str(local_device.device_id)] = local_device

        # Now deal with the user
        local_user = next((u for u in local_devices.values() if str(u.user_id) == todo_user["id"]))
        certifier_id, certifier_key = _get_certifier_id_and_key(todo_user.get("certifier"))
        created_on = todo_user.get("created_on", now)
        user_certif = certify_user(
            certifier_id=certifier_id,
            certifier_key=certifier_key,
            user_id=local_user.user_id,
            public_key=local_user.public_key,
            now=created_on,
        )

        remote_user = RemoteUser(
            user_id=local_user.user_id,
            certified_user=user_certif,
            user_certifier=certifier_id,
            devices=RemoteDevicesMapping(
                *[d for d in remote_devices.values() if d.user_id == local_user.user_id]
            ),
            created_on=now,
        )
        trustchain = {k: v for k, v in remote_devices.items() if k.user_id != remote_user.user_id}

        return TrustChainCtx(
            coolorg.organization_id,
            coolorg.root_verify_key,
            remote_user,
            trustchain,
            remote_devices,
            local_devices,
        )

    return _trustchain_ctx_factory


def test_validate_user_with_trustchain_ok(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "alice@dev2", "certifier": "alice@dev1"},
            {"id": "alice@dev3", "certifier": "alice@dev2"},
        ),
        todo_user={"id": "alice"},
    )

    validate_user_with_trustchain(ctx.remote_user, ctx.trustchain, ctx.root_verify_key)


def test_validate_user_with_trustchain_ok_with_user_certifier(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "bob@dev1"},
            {"id": "alice@dev2", "certifier": "alice@dev1"},
            {"id": "alice@dev3", "certifier": "alice@dev2"},
        ),
        todo_user={"id": "alice", "certifier": "bob@dev1"},
    )

    validate_user_with_trustchain(ctx.remote_user, ctx.trustchain, ctx.root_verify_key)


def test_validate_user_with_trustchain_with_user_created_by_revoked_certifier(
    trustchain_ctx_factory
):
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

    validate_user_with_trustchain(ctx.remote_user, ctx.trustchain, ctx.root_verify_key)


def test_validate_user_with_trustchain_with_revoked_user(trustchain_ctx_factory):
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

    validate_user_with_trustchain(ctx.remote_user, ctx.trustchain, ctx.root_verify_key)


def test_validate_user_with_trustchain_broken_chain(trustchain_ctx_factory):
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

    ctx.trustchain.pop("mallory@dev1")
    with pytest.raises(TrustChainBrokenChainError) as exc:
        validate_user_with_trustchain(ctx.remote_user, ctx.trustchain, ctx.root_verify_key)
    assert exc.value.args == ("Missing `mallory@dev1` needed to validate `bob@dev1`",)


def test_validate_user_with_trustchain_server_filthy_lier_on_user(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "bob@dev1"},
            {"id": "alice@dev2", "certifier": "alice@dev1"},
            {"id": "alice@dev3", "certifier": "alice@dev2"},
        ),
        todo_user={"id": "alice"},
    )

    lying_user = ctx.remote_user.evolve(user_certifier=DeviceID("bob@dev1"))
    with pytest.raises(TrustChainCertifServerMismatchError) as exc:
        validate_user_with_trustchain(lying_user, ctx.trustchain, ctx.root_verify_key)
    assert exc.value.args == (
        "Device `alice` is said to be signed by `root key` according to "
        "certified payload but by `bob@dev1` according to server",
    )


def test_validate_user_with_trustchain_server_filthy_lier_on_device(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "bob@dev1", "certifier": "alice@dev1"},
            {"id": "alice@dev2", "certifier": "bob@dev1"},
            {"id": "alice@dev3", "certifier": "alice@dev2"},
        ),
        todo_user={"id": "alice"},
    )

    lying_trustchain = {
        DeviceID("bob@dev1"): ctx.trustchain["bob@dev1"].evolve(
            device_certifier=DeviceID("alice@dev2")
        )
    }
    with pytest.raises(TrustChainCertifServerMismatchError) as exc:
        validate_user_with_trustchain(ctx.remote_user, lying_trustchain, ctx.root_verify_key)
    assert exc.value.args == (
        "Device `bob@dev1` is said to be signed by `alice@dev1` according to "
        "certified payload but by `alice@dev2` according to server",
    )


def test_validate_user_with_trustchain_with_invalid_revocation(trustchain_ctx_factory):
    ctx = trustchain_ctx_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "bob@dev1", "revoker": "alice@dev1", "revoked_on": Pendulum(2000, 1, 2)},
            {"id": "alice@dev2", "certifier": "alice@dev1"},
            # Bob already revoked when it is supposed to do revoke alice
            {
                "id": "alice@dev3",
                "certifier": "alice@dev2",
                "revoker": "bob@dev1",
                "revoked_on": Pendulum(2000, 1, 3),
            },
        ),
        todo_user={"id": "alice"},
    )

    with pytest.raises(TrustChainSignedByRevokedDeviceError) as exc:
        validate_user_with_trustchain(ctx.remote_user, ctx.trustchain, ctx.root_verify_key)
    assert exc.value.args == (
        "Device `bob@dev1` signed `alice@dev3` after it revocation "
        "(revoked at 2000-01-02T00:00:00+00:00, signed at 2000-01-03T00:00:00+00:00)",
    )

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from typing import Dict, Optional, Tuple
from parsec._parsec import DateTime

from parsec.api.data import UserCertificate, DeviceCertificate, RevokedUserCertificate, UserProfile
from parsec.api.protocol import UserID
from parsec.api.protocol.types import DeviceID
from parsec.core.types import LocalDevice
from parsec.core.trustchain import TrustchainContext, TrustchainError


class TrustchainData:
    def __init__(self, organization_id, root_verify_key):
        self.organization_id = organization_id
        self.root_verify_key = root_verify_key
        self.users_certifs: Dict[UserID, Tuple[UserCertificate, bytes]] = {}
        self.devices_certifs: Dict[DeviceID, Tuple[DeviceCertificate, bytes]] = {}
        self.revoked_users_certifs: Dict[UserID, Tuple[RevokedUserCertificate, bytes]] = {}
        self.local_devices: Dict[DeviceID, LocalDevice] = {}

    def run_trustchain_load_user_and_devices(self, user: UserID):
        ctx = self.trustchain_ctx_factory()
        return ctx.load_user_and_devices(
            trustchain={
                "users": [certif for _, certif in self.users_certifs.values()],
                "revoked_users": [certif for _, certif in self.revoked_users_certifs.values()],
                "devices": [certif for _, certif in self.devices_certifs.values()],
            },
            user_certif=self.get_user_certif(user),
            revoked_user_certif=self.get_revoked_user_certif(user),
            devices_certifs=self.get_devices_certifs(user),
            expected_user_id=user,
        )

    def trustchain_ctx_factory(self) -> TrustchainContext:
        return TrustchainContext(self.root_verify_key, 1)

    def add_user_certif(self, certif_content: UserCertificate, certif: bytes) -> None:
        self.users_certifs[certif_content.user_id] = (certif_content, certif)

    def add_revoked_user_certif(
        self, certif_content: RevokedUserCertificate, certif: bytes
    ) -> None:
        self.revoked_users_certifs[certif_content.user_id] = (certif_content, certif)

    def add_device_certif(self, certif_content: DeviceCertificate, certif: bytes) -> None:
        self.devices_certifs[certif_content.device_id] = (certif_content, certif)

    def add_local_device(self, local_device: LocalDevice) -> None:
        self.local_devices[local_device.device_id] = local_device

    def get_local_device(self, device_id) -> Optional[LocalDevice]:
        if not isinstance(device_id, DeviceID):
            device_id = DeviceID(device_id)

        return self.local_devices.get(device_id)

    def get_user_certif(self, user_id, content=False):
        if not isinstance(user_id, UserID):
            user_id = UserID(user_id)

        try:
            certif_content, certif = self.users_certifs[user_id]
        except KeyError:
            return None
        return certif_content if content else certif

    def get_revoked_user_certif(self, user_id, content=False):
        if not isinstance(user_id, UserID):
            user_id = UserID(user_id)

        try:
            certif_content, certif = self.revoked_users_certifs[user_id]
        except KeyError:
            return None
        return certif_content if content else certif

    def get_device_certif(self, device_id, content=False):
        if not isinstance(device_id, DeviceID):
            device_id = DeviceID(device_id)

        try:
            certif_content, certif = self.devices_certifs[device_id]
        except KeyError:
            return None
        return certif_content if content else certif

    def get_devices_certifs(self, user_id, content=False):
        if not isinstance(user_id, UserID):
            user_id = UserID(user_id)

        if content:
            return [
                content
                for device_id, (content, _) in self.devices_certifs.items()
                if device_id.user_id == user_id
            ]
        else:
            return [
                certif
                for device_id, (_, certif) in self.devices_certifs.items()
                if device_id.user_id == user_id
            ]


@pytest.fixture
def trustchain_data_factory(local_device_factory, coolorg):
    now = DateTime(2000, 1, 1)

    def _trustchain_data_factory(todo_devices, todo_users):
        data = TrustchainData(coolorg.organization_id, coolorg.root_verify_key)

        def _get_certifier_id_and_key(certifier):
            if not certifier:
                # Certified by root
                certifier_id = None
                certifier_key = coolorg.root_signing_key
            else:
                certifier_ld = data.get_local_device(certifier)
                if not certifier_ld:
                    raise RuntimeError(f"Missing `{certifier}` to sign creation of `{todo_device}`")
                certifier_id = certifier_ld.device_id
                certifier_key = certifier_ld.signing_key
            return certifier_id, certifier_key

        # First create all the devices
        for todo_device in todo_devices:
            local_device = local_device_factory(todo_device["id"], org=coolorg)
            data.add_local_device(local_device)
            # Generate device certificate
            certifier_id, certifier_key = _get_certifier_id_and_key(todo_device.get("certifier"))
            created_on = todo_device.get("created_on", now)
            device_certificate = DeviceCertificate(
                author=certifier_id,
                timestamp=created_on,
                device_id=local_device.device_id,
                device_label=local_device.device_label,
                verify_key=local_device.verify_key,
            )
            data.add_device_certif(
                device_certificate, device_certificate.dump_and_sign(certifier_key)
            )

        # Now deal with the users
        for todo_user in todo_users:
            local_user = next(
                (u for u in data.local_devices.values() if str(u.user_id) == todo_user["id"]), None
            )
            if not local_user:
                raise RuntimeError(f"Missing device for user `{todo_user['id']}`")
            # Generate user certificate
            certifier_id, certifier_key = _get_certifier_id_and_key(todo_user.get("certifier"))
            created_on = todo_user.get("created_on", now)
            user_certif = UserCertificate(
                author=certifier_id,
                timestamp=created_on,
                user_id=local_user.user_id,
                human_handle=local_device.human_handle,
                public_key=local_user.public_key,
                profile=todo_user.get("profile", UserProfile.STANDARD),
            )
            data.add_user_certif(user_certif, user_certif.dump_and_sign(certifier_key))
            # Generate user revocation certificate if needed
            revoker = todo_user.get("revoker", None)
            if revoker:
                revoked_on = todo_user.get("revoked_on", now)
                revoker_ld = data.get_local_device(revoker)
                if not revoker_ld:
                    raise RuntimeError(
                        f"Missing `{revoker}` to sign revocation of `{todo_user['id']}`"
                    )
                revoked_user_certificate = RevokedUserCertificate(
                    author=revoker_ld.device_id, timestamp=revoked_on, user_id=local_user.user_id
                )
                data.add_revoked_user_certif(
                    revoked_user_certificate,
                    revoked_user_certificate.dump_and_sign(revoker_ld.signing_key),
                )

        return data

    return _trustchain_data_factory


def test_bad_expected_user(trustchain_data_factory):
    data = trustchain_data_factory(
        todo_devices=({"id": "alice@dev1"},), todo_users=({"id": "alice"},)
    )
    ctx = data.trustchain_ctx_factory()
    with pytest.raises(TrustchainError) as exc:
        ctx.load_user_and_devices(
            trustchain={"users": [], "revoked_users": [], "devices": []},
            user_certif=data.get_user_certif("alice"),
            revoked_user_certif=None,
            devices_certifs=[data.get_device_certif("alice@dev1")],
            expected_user_id=UserID("bob"),
        )
    assert str(exc.value) == "Unexpected certificate: expected `bob` but got `alice`"


def test_verify_no_trustchain(trustchain_data_factory):
    data = trustchain_data_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "alice@dev2", "certifier": "alice@dev1"},
            {"id": "alice@dev3", "certifier": "alice@dev2"},
        ),
        todo_users=({"id": "alice"},),
    )
    user, revoked_user, devices = data.run_trustchain_load_user_and_devices(UserID("alice"))

    assert user == data.get_user_certif("alice", content=True)
    assert revoked_user is None
    assert data.get_device_certif("alice@dev1", content=True) in devices
    assert data.get_device_certif("alice@dev2", content=True) in devices
    assert data.get_device_certif("alice@dev3", content=True) in devices


def test_bad_user_self_signed(trustchain_data_factory):
    data = trustchain_data_factory(
        todo_devices=({"id": "alice@dev1"},),
        todo_users=({"id": "alice", "certifier": "alice@dev1"},),
    )
    with pytest.raises(TrustchainError) as exc:
        data.run_trustchain_load_user_and_devices(UserID("alice"))
    assert str(exc.value) == "alice: Invalid self-signed user certificate"


def test_bad_revoked_user_self_signed(trustchain_data_factory):
    data = trustchain_data_factory(
        todo_devices=({"id": "alice@dev1"},), todo_users=({"id": "alice", "revoker": "alice@dev1"},)
    )
    with pytest.raises(TrustchainError) as exc:
        data.run_trustchain_load_user_and_devices(UserID("alice"))
    assert str(exc.value) == "alice: Invalid self-signed user revocation certificate"


def test_invalid_loop_on_device_certif_trustchain_error(trustchain_data_factory):
    data = trustchain_data_factory(
        todo_devices=({"id": "alice@dev1"}, {"id": "bob@dev1", "certifier": "alice@dev1"}),
        todo_users=(
            {"id": "alice", "profile": UserProfile.ADMIN},
            {"id": "bob", "profile": UserProfile.ADMIN},
        ),
    )
    # Hack certificates to make the loop
    bob = data.get_local_device("bob@dev1")
    certif = data.get_device_certif("alice@dev1", content=True)
    loop_certif = certif.evolve(author=bob.device_id)
    data.add_device_certif(loop_certif, loop_certif.dump_and_sign(bob.signing_key))

    with pytest.raises(TrustchainError) as exc:
        data.run_trustchain_load_user_and_devices(UserID("alice"))
    assert (
        str(exc.value)
        == "alice@dev1 <-sign- bob@dev1 <-sign- alice@dev1: Invalid signature loop detected"
        or str(exc.value)
        == "bob@dev1 <-sign- alice@dev1 <-sign- bob@dev1: Invalid signature loop detected"
    )


def test_device_signature_while_revoked(trustchain_data_factory):
    d1 = DateTime(2000, 1, 1)
    d2 = DateTime(2000, 1, 2)

    data = trustchain_data_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "bob@dev1"},
            {"id": "mallory@dev1", "certifier": "alice@dev1", "created_on": d2},
        ),
        todo_users=(
            {"id": "alice", "profile": UserProfile.ADMIN, "revoker": "bob@dev1", "revoked_on": d1},
            {"id": "bob", "profile": UserProfile.ADMIN},
            {"id": "mallory"},
        ),
    )
    with pytest.raises(TrustchainError) as exc:
        data.run_trustchain_load_user_and_devices(UserID("mallory"))
    assert str(exc.value) == (
        "mallory@dev1 <-sign- alice@dev1: Signature (2000-01-02T00:00:00+00:00)"
        " is posterior to user revocation (2000-01-01T00:00:00+00:00)"
    )


def test_user_signature_while_revoked(trustchain_data_factory):
    d1 = DateTime(2000, 1, 1)
    d2 = DateTime(2000, 1, 2)

    data = trustchain_data_factory(
        todo_devices=({"id": "alice@dev1"}, {"id": "bob@dev1"}, {"id": "mallory@dev1"}),
        todo_users=(
            {"id": "alice", "profile": UserProfile.ADMIN, "revoker": "bob@dev1", "revoked_on": d1},
            {"id": "bob", "profile": UserProfile.ADMIN},
            {"id": "mallory", "certifier": "alice@dev1", "created_on": d2},
        ),
    )
    with pytest.raises(TrustchainError) as exc:
        data.run_trustchain_load_user_and_devices(UserID("mallory"))
    assert str(exc.value) == (
        "mallory's creation <-sign- alice@dev1: Signature (2000-01-02T00:00:00+00:00)"
        " is posterior to user revocation (2000-01-01T00:00:00+00:00)"
    )


def test_revoked_user_signature_while_revoked(trustchain_data_factory):
    d1 = DateTime(2000, 1, 1)
    d2 = DateTime(2000, 1, 2)

    data = trustchain_data_factory(
        todo_devices=({"id": "alice@dev1"}, {"id": "bob@dev1"}, {"id": "mallory@dev1"}),
        todo_users=(
            {"id": "alice", "profile": UserProfile.ADMIN, "revoker": "bob@dev1", "revoked_on": d1},
            {"id": "bob", "profile": UserProfile.ADMIN},
            {"id": "mallory", "revoker": "alice@dev1", "revoked_on": d2},
        ),
    )
    with pytest.raises(TrustchainError) as exc:
        data.run_trustchain_load_user_and_devices(UserID("mallory"))
    assert str(exc.value) == (
        "mallory's revocation <-sign- alice@dev1: Signature (2000-01-02T00:00:00+00:00)"
        " is posterior to user revocation (2000-01-01T00:00:00+00:00)"
    )


def test_create_user_not_admin(trustchain_data_factory):
    data = trustchain_data_factory(
        todo_devices=({"id": "alice@dev1"}, {"id": "bob@dev1"}),
        todo_users=({"id": "alice", "certifier": "bob@dev1"}, {"id": "bob"}),
    )
    with pytest.raises(TrustchainError) as exc:
        data.run_trustchain_load_user_and_devices(UserID("alice"))
    assert (
        str(exc.value)
        == "alice's creation <-sign- bob@dev1: Invalid signature given bob is not admin"
    )


def test_revoked_user_not_admin(trustchain_data_factory):
    data = trustchain_data_factory(
        todo_devices=({"id": "alice@dev1"}, {"id": "bob@dev1"}),
        todo_users=({"id": "alice", "revoker": "bob@dev1"}, {"id": "bob"}),
    )
    with pytest.raises(TrustchainError) as exc:
        data.run_trustchain_load_user_and_devices(UserID("alice"))
    assert (
        str(exc.value)
        == "alice's revocation <-sign- bob@dev1: Invalid signature given bob is not admin"
    )


def test_verify_user_with_broken_trustchain(trustchain_data_factory):
    data = trustchain_data_factory(
        todo_devices=(
            {"id": "alice@dev1"},
            {"id": "mallory@dev1"},
            {"id": "bob@dev1", "certifier": "mallory@dev1"},
        ),
        todo_users=(
            {"id": "alice", "revoker": "bob@dev1"},
            {"id": "bob", "profile": UserProfile.ADMIN},
            {"id": "mallory", "profile": UserProfile.ADMIN},
        ),
    )
    # Remove a certificate
    data.devices_certifs.pop(DeviceID("mallory@dev1"))
    with pytest.raises(TrustchainError) as exc:
        data.run_trustchain_load_user_and_devices(UserID("alice"))
    assert str(exc.value) == (
        "bob@dev1 <-sign- mallory@dev1: Missing device certificate for mallory@dev1"
    )

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pytest
import pendulum
from collections import defaultdict
from typing import Union, Optional
from async_generator import asynccontextmanager

from parsec.types import DeviceID, BackendOrganizationBootstrapAddr
from parsec.crypto import SigningKey, encrypt_with_secret_key
from parsec.trustchain import certify_user, certify_device, certify_device_revocation
from parsec.core.types import (
    LocalDevice,
    remote_manifest_serializer,
    local_manifest_serializer,
    LocalUserManifest,
)
from parsec.core.devices_manager import generate_new_device
from parsec.backend.user import User as BackendUser, new_user_factory as new_backend_user_factory


@attr.s
class OrganizationFullData:
    bootstrap_addr = attr.ib()
    addr = attr.ib()
    root_signing_key = attr.ib()

    @property
    def bootstrap_token(self):
        return self.bootstrap_addr.bootstrap_token

    @property
    def root_verify_key(self):
        return self.root_signing_key.verify_key

    @property
    def organization_id(self):
        return self.addr.organization_id


@pytest.fixture
def organization_factory(backend_addr):
    organizations = set()
    count = 0

    def _organization_factory(orgname=None):
        nonlocal count

        if not orgname:
            count += 1
            orgname = f"Org{count}"

        assert orgname not in organizations
        organizations.add(orgname)

        bootstrap_token = f"<{orgname}-bootstrap-token>"
        bootstrap_addr = BackendOrganizationBootstrapAddr.build(
            backend_addr, orgname, bootstrap_token
        )

        root_signing_key = SigningKey.generate()
        addr = bootstrap_addr.generate_organization_addr(root_signing_key.verify_key)

        return OrganizationFullData(bootstrap_addr, addr, root_signing_key)

    return _organization_factory


@pytest.fixture
def local_device_factory(coolorg):
    devices = defaultdict(list)
    count = 0

    def _local_device_factory(
        base_device_id: Optional[str] = None, org: OrganizationFullData = coolorg
    ):
        nonlocal count

        if not base_device_id:
            count += 1
            base_device_id = f"user{count}@dev0"

        org_devices = devices[org.organization_id]
        device_id = DeviceID(base_device_id)
        assert not any(d for d in org_devices if d.device_id == device_id)

        device = generate_new_device(device_id, org.addr)
        try:
            # If the user already exists, we must retreive it data
            parent_device = next(d for d in org_devices if d.user_id == device_id.user_id)
            device = device.evolve(
                private_key=parent_device.private_key,
                user_manifest_access=parent_device.user_manifest_access,
            )

        except StopIteration:
            pass

        org_devices.append(device)
        return device

    return _local_device_factory


@pytest.fixture
def coolorg(organization_factory):
    # Fonzie approve this
    return organization_factory("CoolOrg")


@pytest.fixture
def otherorg(organization_factory):
    return organization_factory("OtherOrg")


@pytest.fixture
def otheralice(local_device_factory, otherorg):
    return local_device_factory("alice@dev1", otherorg)


@pytest.fixture
def alice(local_device_factory):
    return local_device_factory("alice@dev1")


@pytest.fixture
def alice2(local_device_factory):
    return local_device_factory("alice@dev2")


@pytest.fixture
def bob(local_device_factory):
    return local_device_factory("bob@dev1")


@pytest.fixture
def mallory(local_device_factory):
    return local_device_factory("mallory@dev1")


class InitialUserManifestState:
    def __init__(self):
        self._v1 = {}

    def _generate_or_retrieve_user_manifest_v1(self, device):
        try:
            return self._v1[device.user_id]

        except KeyError:
            user_manifest = LocalUserManifest(
                author=device.device_id, base_version=1, is_placeholder=False, need_sync=False
            )

            remote_user_manifest = user_manifest.to_remote()
            access = device.user_manifest_access

            ciphered = encrypt_with_secret_key(
                device.device_id,
                device.signing_key,
                access.key,
                local_manifest_serializer.dumps(user_manifest),
            )

            remote_ciphered = encrypt_with_secret_key(
                device.device_id,
                device.signing_key,
                access.key,
                remote_manifest_serializer.dumps(remote_user_manifest),
            )

            self._v1[device.user_id] = (
                user_manifest,
                remote_user_manifest,
                ciphered,
                remote_ciphered,
            )
            return self._v1[device.user_id]

    def get_user_manifest_v1_for_device(self, device, ciphered=False):
        in_device, _, ciphered_in_device, _ = self._generate_or_retrieve_user_manifest_v1(device)
        if ciphered:
            return ciphered_in_device

        else:
            return in_device

    def get_user_manifest_v1_for_backend(self, device, ciphered=False):
        _, in_backend, _, ciphered_in_backend = self._generate_or_retrieve_user_manifest_v1(device)
        if ciphered:
            return ciphered_in_backend

        else:
            return in_backend


@pytest.fixture
def initial_user_manifest_state():
    # User manifest is stored in backend vlob and in devices's local db.
    # Hence this fixture allow us to centralize the first version of this user
    # manifest.
    # In most tests we want to be in a state were backend and devices all
    # store the same user manifest (named the "v1" here).
    # But sometime we want a completly fresh start ("v1" doesn't exist,
    # hence devices and backend are empty) or only a single device to begin
    # with no knowledge of the "v1".
    return InitialUserManifestState()


def local_device_to_backend_user(
    device: LocalDevice, certifier: Union[LocalDevice, OrganizationFullData]
) -> BackendUser:
    if isinstance(certifier, OrganizationFullData):
        certifier_id = None
        certifier_signing_key = certifier.root_signing_key
    else:
        certifier_id = certifier.device_id
        certifier_signing_key = certifier.signing_key

    now = pendulum.now()
    certified_user = certify_user(
        certifier_id, certifier_signing_key, device.user_id, device.public_key, now
    )
    certified_device = certify_device(
        certifier_id, certifier_signing_key, device.device_id, device.verify_key, now
    )
    return new_backend_user_factory(
        device_id=device.device_id,
        is_admin=False,
        certifier=certifier_id,
        certified_user=certified_user,
        certified_device=certified_device,
    )


@pytest.fixture
def backend_data_binder_factory(backend_addr, initial_user_manifest_state):
    class BackendDataBinder:
        def __init__(self, backend, backend_addr=backend_addr):
            self.backend = backend
            self.backend_addr = backend_addr
            self.binded_local_devices = []

        def get_device(self, device_id):
            for d in self.binded_local_devices:
                if d.device_id == device_id:
                    return d
            else:
                raise ValueError(device_id)

        async def bind_organization(
            self,
            org: OrganizationFullData,
            first_device: LocalDevice = None,
            initial_user_manifest_in_v0: bool = False,
        ):
            bootstrap_token = f"<{org.organization_id}-bootstrap-token>"
            await self.backend.organization.create(org.organization_id, bootstrap_token)
            if first_device:
                backend_user = local_device_to_backend_user(first_device, org)
                await self.backend.organization.bootstrap(
                    org.organization_id, backend_user, bootstrap_token, org.root_verify_key
                )
                self.binded_local_devices.append(first_device)

                if not initial_user_manifest_in_v0:
                    ciphered = initial_user_manifest_state.get_user_manifest_v1_for_backend(
                        first_device, ciphered=True
                    )
                    access = first_device.user_manifest_access
                    await self.backend.vlob.create(
                        first_device.organization_id,
                        first_device.device_id,
                        access.id,
                        access.id,
                        ciphered,
                    )

        async def bind_device(
            self,
            device: LocalDevice,
            certifier: Optional[LocalDevice] = None,
            initial_user_manifest_in_v0: bool = False,
        ):
            if not certifier:
                try:
                    certifier = next(
                        d
                        for d in self.binded_local_devices
                        if d.organization_id == device.organization_id
                    )
                except StopIteration:
                    raise RuntimeError(f"Organization `{device.organization_id}` not bootstrapped")

            backend_user = local_device_to_backend_user(device, certifier)

            if any(d for d in self.binded_local_devices if d.user_id == device.user_id):
                # User already created, only add device
                await self.backend.user.create_device(
                    device.organization_id, backend_user.devices[device.device_name]
                )

            else:
                # Add device and user
                await self.backend.user.create_user(device.organization_id, backend_user)

                if not initial_user_manifest_in_v0:
                    ciphered = initial_user_manifest_state.get_user_manifest_v1_for_backend(
                        device, ciphered=True
                    )
                    access = device.user_manifest_access
                    await self.backend.vlob.create(
                        device.organization_id, device.device_id, access.id, access.id, ciphered
                    )

            self.binded_local_devices.append(device)

        async def bind_revocation(self, device: LocalDevice, certifier: LocalDevice):
            certified_revocation = certify_device_revocation(
                certifier.device_id, certifier.signing_key, device.device_id
            )
            await self.backend.user.revoke_device(
                device.organization_id, device.device_id, certified_revocation, certifier.device_id
            )

    # Binder must be unique per backend

    binders = []

    def _backend_data_binder_factory(backend, backend_addr=backend_addr):
        for binder, candidate_backend in binders:
            if candidate_backend is backend:
                return binder

        binder = BackendDataBinder(backend, backend_addr)
        binders.append((binder, backend))
        return binder

    return _backend_data_binder_factory


@pytest.fixture
def sock_from_other_organization_factory(
    backend_sock_factory, backend_data_binder_factory, organization_factory, local_device_factory
):
    @asynccontextmanager
    async def _sock_from_other_organization_factory(backend, mimick=None, anonymous=False):
        binder = backend_data_binder_factory(backend)

        other_org = organization_factory()
        if mimick:
            other_device = local_device_factory(mimick, other_org)
        else:
            other_device = local_device_factory(org=other_org)
        await binder.bind_organization(other_org, other_device)

        if anonymous:
            auth_as = other_org.organization_id
        else:
            auth_as = other_device

        async with backend_sock_factory(backend, auth_as) as sock:
            sock.device = other_device
            yield sock

    return _sock_from_other_organization_factory

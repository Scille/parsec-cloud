# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pytest
import pendulum
from collections import defaultdict
from typing import Union, Optional, Tuple
from async_generator import asynccontextmanager
from pendulum import Pendulum

from parsec.crypto import SigningKey
from parsec.api.data import (
    UserCertificateContent,
    RevokedUserCertificateContent,
    DeviceCertificateContent,
    RealmRoleCertificateContent,
)
from parsec.api.protocol import OrganizationID, UserID, DeviceID, RealmRole
from parsec.api.data import UserManifest
from parsec.core.types import LocalDevice, LocalUserManifest, BackendOrganizationBootstrapAddr
from parsec.core.local_device import generate_new_device
from parsec.backend.user import (
    User as BackendUser,
    Device as BackendDevice,
    new_user_factory as new_backend_user_factory,
)
from parsec.backend.realm import RealmGrantedRole

from tests.common import freeze_time, addr_with_device_subdomain


@attr.s
class OrganizationFullData:
    bootstrap_addr = attr.ib()
    addr = attr.ib()
    root_signing_key = attr.ib()

    @property
    def bootstrap_token(self):
        return self.bootstrap_addr.token

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

    def _organization_factory(orgname=None, expiration_date=None):
        nonlocal count

        if not orgname:
            count += 1
            orgname = f"Org{count}"

        assert orgname not in organizations
        organization_id = OrganizationID(orgname)
        organizations.add(organization_id)

        bootstrap_token = f"<{orgname}-bootstrap-token>"
        bootstrap_addr = BackendOrganizationBootstrapAddr.build(
            backend_addr, organization_id=organization_id, token=bootstrap_token
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
        base_device_id: Optional[str] = None,
        org: OrganizationFullData = coolorg,
        is_admin: bool = None,
    ):
        nonlocal count

        if not base_device_id:
            count += 1
            base_device_id = f"user{count}@dev0"

        org_devices = devices[org.organization_id]
        device_id = DeviceID(base_device_id)
        assert not any(d for d in org_devices if d.device_id == device_id)

        parent_device = None
        try:
            # If the user already exists, we must retreive it data
            parent_device = next(d for d in org_devices if d.user_id == device_id.user_id)
            if is_admin is not None and is_admin is not parent_device.is_admin:
                raise ValueError(
                    "is_admin is set but user already exists, with a different is_admin value."
                )
            is_admin = parent_device.is_admin

        except StopIteration:
            is_admin = bool(is_admin)

        # Force each device to access the backend trough a different hostname so
        # tcp stream spy can switch offline certains while keeping the others online
        org_addr = addr_with_device_subdomain(org.addr, device_id)

        device = generate_new_device(device_id, org_addr, is_admin=is_admin)
        if parent_device is not None:
            device = device.evolve(
                private_key=parent_device.private_key,
                user_manifest_id=parent_device.user_manifest_id,
                user_manifest_key=parent_device.user_manifest_key,
            )
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
def expiredorg(organization_factory):
    expired_org = organization_factory("ExpiredOrg", pendulum.datetime(2010, 1, 1))
    return expired_org


@pytest.fixture
def otheralice(local_device_factory, otherorg):
    return local_device_factory("alice@dev1", otherorg, is_admin=True)


@pytest.fixture
def alice(local_device_factory, initial_user_manifest_state):
    device = local_device_factory("alice@dev1", is_admin=True)
    # Force alice user manifest v1 to be signed by user alice@dev1
    # This is needed given backend_factory bind alice@dev1 then alice@dev2,
    # hence user manifest v1 is stored in backend at a time when alice@dev2
    # doesn't exists.
    with freeze_time("2000-01-01"):
        initial_user_manifest_state.force_user_manifest_v1_generation(device)
    return device


@pytest.fixture
def expiredalice(local_device_factory, initial_user_manifest_state, expiredorg):
    device = local_device_factory("alice@dev1", expiredorg, is_admin=True)
    # Force alice user manifest v1 to be signed by user alice@dev1
    # This is needed given backend_factory bind alice@dev1 then alice@dev2,
    # hence user manifest v1 is stored in backend at a time when alice@dev2
    # doesn't exists.
    with freeze_time("2000-01-01"):
        initial_user_manifest_state.force_user_manifest_v1_generation(device)
    return device


@pytest.fixture
def alice2(local_device_factory):
    return local_device_factory("alice@dev2", is_admin=True)


@pytest.fixture
def adam(local_device_factory):
    return local_device_factory("adam@dev1", is_admin=True)


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
            now = pendulum.now()

            remote_user_manifest = UserManifest(
                author=device.device_id,
                timestamp=now,
                id=device.user_manifest_id,
                version=1,
                created=now,
                updated=now,
                last_processed_message=0,
                workspaces=(),
            )
            local_user_manifest = LocalUserManifest.from_remote(remote_user_manifest)
            self._v1[device.user_id] = (remote_user_manifest, local_user_manifest)
            return self._v1[device.user_id]

    def force_user_manifest_v1_generation(self, device):
        self._generate_or_retrieve_user_manifest_v1(device)

    def get_user_manifest_v1_for_device(self, device, ciphered=False):
        _, local = self._generate_or_retrieve_user_manifest_v1(device)
        return local

    def get_user_manifest_v1_for_backend(self, device):
        remote, _ = self._generate_or_retrieve_user_manifest_v1(device)
        return remote


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
) -> Tuple[BackendUser, BackendDevice]:
    if isinstance(certifier, OrganizationFullData):
        certifier_id = None
        certifier_signing_key = certifier.root_signing_key
    else:
        certifier_id = certifier.device_id
        certifier_signing_key = certifier.signing_key

    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=certifier_id,
        timestamp=now,
        user_id=device.user_id,
        public_key=device.public_key,
        is_admin=device.is_admin,
    ).dump_and_sign(certifier_signing_key)
    device_certificate = DeviceCertificateContent(
        author=certifier_id, timestamp=now, device_id=device.device_id, verify_key=device.verify_key
    ).dump_and_sign(certifier_signing_key)
    return new_backend_user_factory(
        device_id=device.device_id,
        is_admin=device.is_admin,
        certifier=certifier_id,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
    )


class CertificatesStore:
    def __init__(self):
        self._user_certificates = {}
        self._device_certificates = {}
        self._revoked_user_certificates = {}

    def store_user(self, organization_id, user_id, certif):
        key = (organization_id, user_id)
        assert key not in self._user_certificates
        self._user_certificates[key] = certif

    def store_device(self, organization_id, device_id, certif):
        key = (organization_id, device_id)
        assert key not in self._device_certificates
        self._device_certificates[key] = certif

    def store_revoked_user(self, organization_id, user_id, certif):
        key = (organization_id, user_id)
        assert key not in self._revoked_user_certificates
        self._revoked_user_certificates[key] = certif

    def get_user(self, local_user):
        key = (local_user.organization_id, local_user.user_id)
        return self._user_certificates[key]

    def get_device(self, local_device):
        key = (local_device.organization_id, local_device.device_id)
        return self._device_certificates[key]

    def get_revoked_user(self, local_user):
        key = (local_user.organization_id, local_user.user_id)
        return self._revoked_user_certificates.get(key)

    def translate_certif(self, needle):
        for (_, user_id), certif in self._user_certificates.items():
            if needle == certif:
                return f"<{user_id} user certif>"

        for (_, device_id), certif in self._device_certificates.items():
            if needle == certif:
                return f"<{device_id} device certif>"

        for (_, user_id), certif in self._revoked_user_certificates.items():
            if needle == certif:
                return f"<{user_id} revoked user certif>"

        raise RuntimeError("Unknown certificate !")

    def translate_certifs(self, certifs):
        return [self.translate_certif(certif) for certif in certifs]


@pytest.fixture
def certificates_store(backend_data_binder_factory, backend):
    binder = backend_data_binder_factory(backend)
    return binder.certificates_store


@pytest.fixture
def backend_data_binder_factory(request, backend_addr, initial_user_manifest_state):
    class BackendDataBinder:
        def __init__(self, backend, backend_addr=backend_addr):
            self.backend = backend
            self.backend_addr = backend_addr
            self.binded_local_devices = []
            self.certificates_store = CertificatesStore()

        def get_device(self, organization_id, device_id):
            for d in self.binded_local_devices:
                if d.organization_id == organization_id and d.device_id == device_id:
                    return d
            else:
                raise ValueError((organization_id, device_id))

        async def _create_realm_and_first_vlob(self, device):
            manifest = initial_user_manifest_state.get_user_manifest_v1_for_backend(device)
            if manifest.author == device.device_id:
                author = device
            else:
                author = self.get_device(device.organization_id, manifest.author)
            realm_id = author.user_manifest_id

            with self.backend.event_bus.listen() as spy:

                await self.backend.realm.create(
                    organization_id=author.organization_id,
                    self_granted_role=RealmGrantedRole(
                        realm_id=realm_id,
                        user_id=author.user_id,
                        certificate=RealmRoleCertificateContent(
                            author=author.device_id,
                            timestamp=manifest.timestamp,
                            realm_id=realm_id,
                            user_id=author.user_id,
                            role=RealmRole.OWNER,
                        ).dump_and_sign(author.signing_key),
                        role=RealmRole.OWNER,
                        granted_by=author.device_id,
                        granted_on=manifest.timestamp,
                    ),
                )

                await self.backend.vlob.create(
                    organization_id=author.organization_id,
                    author=author.device_id,
                    realm_id=realm_id,
                    encryption_revision=1,
                    vlob_id=author.user_manifest_id,
                    timestamp=manifest.timestamp,
                    blob=manifest.dump_sign_and_encrypt(
                        author_signkey=author.signing_key, key=author.user_manifest_key
                    ),
                )

                # Avoid possible race condition in tests listening for events
                await spy.wait_multiple_with_timeout(
                    [
                        (
                            "realm.roles_updated",
                            {
                                "organization_id": author.organization_id,
                                "author": author.device_id,
                                "realm_id": author.user_manifest_id,
                                "user": author.user_id,
                                "role": RealmRole.OWNER,
                            },
                        ),
                        (
                            "realm.vlobs_updated",
                            {
                                "organization_id": author.organization_id,
                                "author": author.device_id,
                                "realm_id": author.user_manifest_id,
                                "checkpoint": 1,
                                "src_id": author.user_manifest_id,
                                "src_version": 1,
                            },
                        ),
                    ]
                )

        async def bind_organization(
            self,
            org: OrganizationFullData,
            first_device: LocalDevice = None,
            initial_user_manifest_in_v0: bool = False,
            expiration_date: Pendulum = None,
        ):
            bootstrap_token = f"<{org.organization_id}-bootstrap-token>"
            await self.backend.organization.create(
                org.organization_id, bootstrap_token, expiration_date
            )
            if first_device:
                backend_user, backend_first_device = local_device_to_backend_user(first_device, org)
                await self.backend.organization.bootstrap(
                    org.organization_id,
                    backend_user,
                    backend_first_device,
                    bootstrap_token,
                    org.root_verify_key,
                )
                self.certificates_store.store_user(
                    org.organization_id, backend_user.user_id, backend_user.user_certificate
                )
                self.certificates_store.store_device(
                    org.organization_id,
                    backend_first_device.device_id,
                    backend_first_device.device_certificate,
                )
                self.binded_local_devices.append(first_device)

                if not initial_user_manifest_in_v0:
                    await self._create_realm_and_first_vlob(first_device)

        async def bind_device(
            self,
            device: LocalDevice,
            certifier: Optional[LocalDevice] = None,
            initial_user_manifest_in_v0: bool = False,
            is_admin: bool = False,
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

            backend_user, backend_device = local_device_to_backend_user(device, certifier)

            if any(d for d in self.binded_local_devices if d.user_id == device.user_id):
                # User already created, only add device
                await self.backend.user.create_device(device.organization_id, backend_device)
                self.certificates_store.store_device(
                    device.organization_id,
                    backend_device.device_id,
                    backend_device.device_certificate,
                )

            else:
                # Add device and user
                await self.backend.user.create_user(
                    device.organization_id, backend_user, backend_device
                )
                self.certificates_store.store_user(
                    device.organization_id, backend_user.user_id, backend_user.user_certificate
                )
                self.certificates_store.store_device(
                    device.organization_id,
                    backend_device.device_id,
                    backend_device.device_certificate,
                )

                if not initial_user_manifest_in_v0:
                    await self._create_realm_and_first_vlob(device)

            self.binded_local_devices.append(device)

        async def bind_revocation(self, user_id: UserID, certifier: LocalDevice):
            revoked_user_certificate = RevokedUserCertificateContent(
                author=certifier.device_id, timestamp=pendulum.now(), user_id=user_id
            ).dump_and_sign(certifier.signing_key)
            await self.backend.user.revoke_user(
                certifier.organization_id, user_id, revoked_user_certificate, certifier.device_id
            )
            self.certificates_store.store_revoked_user(
                certifier.organization_id, user_id, revoked_user_certificate
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
    async def _sock_from_other_organization_factory(
        backend, mimick=None, anonymous=False, is_admin=False
    ):
        binder = backend_data_binder_factory(backend)

        other_org = organization_factory()
        if mimick:
            other_device = local_device_factory(mimick, other_org, is_admin)
        else:
            other_device = local_device_factory(org=other_org, is_admin=is_admin)
        await binder.bind_organization(other_org, other_device)

        if anonymous:
            auth_as = other_org.organization_id
        else:
            auth_as = other_device

        async with backend_sock_factory(backend, auth_as) as sock:
            sock.device = other_device
            yield sock

    return _sock_from_other_organization_factory

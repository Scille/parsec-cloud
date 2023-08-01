# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from functools import partial

import pytest

from parsec._parsec import (
    BackendEventRealmRolesUpdated,
    BackendEventRealmVlobsUpdated,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    DateTime,
    DeviceID,
    DeviceLabel,
    EntryID,
    HumanHandle,
    OrganizationID,
    PrivateKey,
    PublicKey,
    RealmID,
    RealmRole,
    SecretKey,
    SigningKey,
    UserID,
    UserProfile,
    VerifyKey,
    VlobID,
)
from parsec.api.data import (
    DeviceCertificate,
    RealmRoleCertificate,
    RevokedUserCertificate,
    UserCertificate,
    UserManifest,
)
from parsec.backend.organization import SequesterAuthority
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.user import Device as BackendDevice
from parsec.backend.user import User as BackendUser
from parsec.backend.vlob import VlobSequesterServiceInconsistencyError
from tests.common.sequester import SequesterAuthorityFullData


@dataclass
class LocalDevice:
    organization_addr: BackendOrganizationAddr
    device_id: DeviceID
    device_label: DeviceLabel | None
    human_handle: HumanHandle | None
    signing_key: SigningKey
    private_key: PrivateKey
    profile: UserProfile
    user_manifest_id: EntryID
    user_manifest_key: SecretKey
    local_symkey: SecretKey

    @property
    def organization_id(self) -> OrganizationID:
        return self.organization_addr.organization_id

    @property
    def root_verify_key(self) -> VerifyKey:
        return self.organization_addr.root_verify_key

    @property
    def user_id(self) -> UserID:
        return self.device_id.user_id

    @property
    def public_key(self) -> PublicKey:
        return self.private_key.public_key

    @property
    def verify_key(self) -> VerifyKey:
        return self.signing_key.verify_key

    def timestamp(self) -> DateTime:
        return DateTime.now()

    @classmethod
    def generate_new_device(
        cls,
        organization_addr: BackendOrganizationAddr,
        profile: UserProfile,
        device_id: DeviceID | None = None,
        human_handle: HumanHandle | None = None,
        device_label: DeviceLabel | None = None,
        signing_key: SigningKey | None = None,
        private_key: PrivateKey | None = None,
    ) -> LocalDevice:
        return cls(
            organization_addr=organization_addr,
            device_id=device_id or DeviceID.new(),
            device_label=device_label,
            human_handle=human_handle,
            signing_key=signing_key or SigningKey.generate(),
            private_key=private_key or PrivateKey.generate(),
            profile=profile,
            user_manifest_id=EntryID.new(),
            user_manifest_key=SecretKey.generate(),
            local_symkey=SecretKey.generate(),
        )


@dataclass
class OrganizationFullData:
    bootstrap_addr: BackendOrganizationBootstrapAddr
    addr: BackendOrganizationAddr
    root_signing_key: SigningKey
    sequester_authority: SequesterAuthorityFullData | None

    @property
    def bootstrap_token(self):
        return self.bootstrap_addr.token

    @property
    def root_verify_key(self):
        return self.root_signing_key.verify_key

    @property
    def organization_id(self):
        return self.addr.organization_id


class InitialUserManifestState:
    def __init__(self):
        self._v1: dict[tuple[OrganizationID, UserID], UserManifest] = {}

    def _generate_or_retrieve_user_manifest_v1(self, device: LocalDevice) -> UserManifest:
        try:
            return self._v1[(device.organization_id, device.user_id)]

        except KeyError:
            timestamp = device.timestamp()
            remote_user_manifest = UserManifest(
                author=device.device_id,
                timestamp=timestamp,
                id=device.user_manifest_id,
                version=1,
                created=timestamp,
                updated=timestamp,
                last_processed_message=0,
                workspaces=[],
            )
            self._v1[(device.organization_id, device.user_id)] = remote_user_manifest
            return self._v1[(device.organization_id, device.user_id)]

    def force_user_manifest_v1_generation(self, device):
        self._generate_or_retrieve_user_manifest_v1(device)

    def get_user_manifest_v1_for_backend(self, device: LocalDevice) -> UserManifest:
        return self._generate_or_retrieve_user_manifest_v1(device)


@pytest.fixture
def initial_user_manifest_state() -> InitialUserManifestState:
    # User manifest is stored in backend vlob and in devices's local db.
    # Hence this fixture allow us to centralize the first version of this user
    # manifest.
    # In most tests we want to be in a state were backend and devices all
    # store the same user manifest (named the "v1" here).
    # But sometime we want a completely fresh start ("v1" doesn't exist,
    # hence devices and backend are empty) or only a single device to begin
    # with no knowledge of the "v1".
    return InitialUserManifestState()


def local_device_to_backend_user(
    device: LocalDevice,
    certifier: LocalDevice | OrganizationFullData,
    timestamp: DateTime | None = None,
) -> tuple[BackendUser, BackendDevice]:
    if isinstance(certifier, OrganizationFullData):
        certifier_id = None
        certifier_signing_key = certifier.root_signing_key
    else:
        certifier_id = certifier.device_id
        certifier_signing_key = certifier.signing_key

    timestamp = timestamp or device.timestamp()

    user_certificate = UserCertificate(
        author=certifier_id,
        timestamp=timestamp,
        user_id=device.user_id,
        public_key=device.public_key,
        profile=device.profile,
        human_handle=device.human_handle,
    )
    device_certificate = DeviceCertificate(
        author=certifier_id,
        timestamp=timestamp,
        device_id=device.device_id,
        device_label=device.device_label,
        verify_key=device.verify_key,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)
    redacted_device_certificate = device_certificate.evolve(device_label=None)

    user = BackendUser(
        user_id=device.user_id,
        human_handle=device.human_handle,
        initial_profile=device.profile,
        user_certificate=user_certificate.dump_and_sign(certifier_signing_key),
        redacted_user_certificate=redacted_user_certificate.dump_and_sign(certifier_signing_key),
        user_certifier=certifier_id,
        created_on=timestamp,
    )

    first_device = BackendDevice(
        device_id=device.device_id,
        device_label=device.device_label,
        device_certificate=device_certificate.dump_and_sign(certifier_signing_key),
        redacted_device_certificate=redacted_device_certificate.dump_and_sign(
            certifier_signing_key
        ),
        device_certifier=certifier_id,
        created_on=timestamp,
    )

    return user, first_device


class CertificatesStore:
    def __init__(self):
        self._user_certificates = {}
        self._device_certificates = {}
        self._revoked_user_certificates = {}

    def store_user(self, organization_id, user_id, certif, redacted_certif):
        key = (organization_id, user_id)
        assert key not in self._user_certificates
        self._user_certificates[key] = (certif, redacted_certif)

    def store_device(self, organization_id, device_id, certif, redacted_certif):
        key = (organization_id, device_id)
        assert key not in self._device_certificates
        self._device_certificates[key] = (certif, redacted_certif)

    def store_revoked_user(self, organization_id, user_id, certif):
        key = (organization_id, user_id)
        assert key not in self._revoked_user_certificates
        self._revoked_user_certificates[key] = certif

    def get_user(self, local_user, redacted=False):
        key = (local_user.organization_id, local_user.user_id)
        certif, redacted_certif = self._user_certificates[key]
        return redacted_certif if redacted else certif

    def get_device(self, local_device, redacted=False):
        key = (local_device.organization_id, local_device.device_id)
        certif, redacted_certif = self._device_certificates[key]
        return redacted_certif if redacted else certif

    def get_revoked_user(self, local_user):
        key = (local_user.organization_id, local_user.user_id)
        return self._revoked_user_certificates.get(key)

    def translate_certif(self, needle):
        for (_, user_id), (certif, redacted_certif) in self._user_certificates.items():
            if needle == certif:
                return f"<{user_id.str} user certif>"
            if needle == redacted_certif:
                return f"<{user_id.str} redacted user certif>"

        for (_, device_id), (certif, redacted_certif) in self._device_certificates.items():
            if needle == certif:
                return f"<{device_id.str} device certif>"
            if needle == redacted_certif:
                return f"<{device_id.str} redacted device certif>"

        for (_, user_id), certif in self._revoked_user_certificates.items():
            if needle == certif:
                return f"<{user_id.str} revoked user certif>"

        raise RuntimeError("Unknown certificate !")

    def translate_certifs(self, certifs):
        return sorted(self.translate_certif(certif) for certif in certifs)


@pytest.fixture
def certificates_store(backend_data_binder_factory, backend):
    binder = backend_data_binder_factory(backend)
    return binder.certificates_store


@pytest.fixture
def backend_data_binder_factory(initial_user_manifest_state):
    class BackendDataBinder:
        def __init__(self, backend):
            self.backend = backend
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
            realm_id = RealmID.from_entry_id(author.user_manifest_id)
            vlob_id = VlobID.from_entry_id(author.user_manifest_id)

            with self.backend.event_bus.listen() as spy:
                # The realm needs to be created strictly before the manifest timestamp
                realm_create_timestamp = manifest.timestamp.subtract(microseconds=1)

                await self.backend.realm.create(
                    organization_id=author.organization_id,
                    self_granted_role=RealmGrantedRole(
                        realm_id=realm_id,
                        user_id=author.user_id,
                        certificate=RealmRoleCertificate(
                            author=author.device_id,
                            timestamp=realm_create_timestamp,
                            realm_id=realm_id,
                            user_id=author.user_id,
                            role=RealmRole.OWNER,
                        ).dump_and_sign(author.signing_key),
                        role=RealmRole.OWNER,
                        granted_by=author.device_id,
                        granted_on=realm_create_timestamp,
                    ),
                )
                vlob_create_fn = partial(
                    self.backend.vlob.create,
                    organization_id=author.organization_id,
                    author=author.device_id,
                    realm_id=realm_id,
                    encryption_revision=1,
                    vlob_id=vlob_id,
                    timestamp=manifest.timestamp,
                    blob=manifest.dump_sign_and_encrypt(
                        author_signkey=author.signing_key, key=author.user_manifest_key
                    ),
                )
                try:
                    await vlob_create_fn(sequester_blob=None)
                except VlobSequesterServiceInconsistencyError:
                    # This won't work if some sequester services are defined,
                    # but it works fine enough for the moment :)
                    await vlob_create_fn(sequester_blob={})

                # Avoid possible race condition in tests listening for events
                await spy.wait_multiple(
                    [
                        (
                            BackendEventRealmRolesUpdated(
                                organization_id=author.organization_id,
                                author=author.device_id,
                                realm_id=realm_id,
                                user=author.user_id,
                                role=RealmRole.OWNER,
                            )
                        ),
                        (
                            BackendEventRealmVlobsUpdated(
                                organization_id=author.organization_id,
                                author=author.device_id,
                                realm_id=realm_id,
                                checkpoint=1,
                                src_id=vlob_id,
                                src_version=1,
                            )
                        ),
                    ]
                )

        async def bind_organization(
            self,
            org: OrganizationFullData,
            first_device: LocalDevice,
            initial_user_manifest: str = "v1",
            timestamp: DateTime | None = None,
            create_needed: bool = True,
        ):
            assert initial_user_manifest in ("v1", "not_synced")

            if create_needed:
                await self.backend.organization.create(
                    id=org.organization_id,
                    bootstrap_token=org.bootstrap_token,
                    created_on=timestamp or first_device.timestamp(),
                )
            assert org.organization_id == first_device.organization_id
            backend_user, backend_first_device = local_device_to_backend_user(
                first_device, org, timestamp
            )
            if org.sequester_authority:
                sequester_authority = SequesterAuthority(
                    certificate=org.sequester_authority.certif,
                    verify_key_der=org.sequester_authority.certif_data.verify_key_der,
                )
            else:
                sequester_authority = None
            await self.backend.organization.bootstrap(
                id=org.organization_id,
                user=backend_user,
                first_device=backend_first_device,
                bootstrap_token=org.bootstrap_token,
                root_verify_key=org.root_verify_key,
                sequester_authority=sequester_authority,
            )
            self.certificates_store.store_user(
                org.organization_id,
                backend_user.user_id,
                backend_user.user_certificate,
                backend_user.redacted_user_certificate,
            )
            self.certificates_store.store_device(
                org.organization_id,
                backend_first_device.device_id,
                backend_first_device.device_certificate,
                backend_first_device.redacted_device_certificate,
            )
            self.binded_local_devices.append(first_device)

            if initial_user_manifest == "v1":
                await self._create_realm_and_first_vlob(first_device)

        async def bind_device(
            self,
            device: LocalDevice,
            certifier: LocalDevice | None = None,
            initial_user_manifest: str | None = None,
            timestamp: DateTime | None = None,
        ):
            assert initial_user_manifest in (None, "v1", "not_synced")

            if not certifier:
                try:
                    certifier = next(
                        d
                        for d in self.binded_local_devices
                        if d.organization_id == device.organization_id
                    )
                except StopIteration:
                    raise RuntimeError(
                        f"Organization `{device.organization_id.str}` not bootstrapped"
                    )

            backend_user, backend_device = local_device_to_backend_user(
                device, certifier, timestamp
            )

            if any(d for d in self.binded_local_devices if d.user_id == device.user_id):
                # User already created, only add device

                # For clarity, user manifest state in backend should be only specified
                # when creating the user
                assert initial_user_manifest is None

                await self.backend.user.create_device(device.organization_id, backend_device)
                self.certificates_store.store_device(
                    device.organization_id,
                    backend_device.device_id,
                    backend_device.device_certificate,
                    backend_device.redacted_device_certificate,
                )

            else:
                # Add device and user
                await self.backend.user.create_user(
                    device.organization_id, backend_user, backend_device
                )
                self.certificates_store.store_user(
                    device.organization_id,
                    backend_user.user_id,
                    backend_user.user_certificate,
                    backend_user.redacted_user_certificate,
                )
                self.certificates_store.store_device(
                    device.organization_id,
                    backend_device.device_id,
                    backend_device.device_certificate,
                    backend_device.redacted_device_certificate,
                )
                # By default we create user manifest v1 in backend
                if initial_user_manifest in (None, "v1"):
                    await self._create_realm_and_first_vlob(device)

            self.binded_local_devices.append(device)

        async def bind_revocation(self, user_id: UserID, certifier: LocalDevice):
            timestamp = certifier.timestamp()
            revoked_user_certificate = RevokedUserCertificate(
                author=certifier.device_id, timestamp=timestamp, user_id=user_id
            ).dump_and_sign(certifier.signing_key)
            await self.backend.user.revoke_user(
                certifier.organization_id, user_id, revoked_user_certificate, certifier.device_id
            )
            self.certificates_store.store_revoked_user(
                certifier.organization_id, user_id, revoked_user_certificate
            )

    # Binder must be unique per backend

    binders = []

    def _backend_data_binder_factory(backend):
        for binder, candidate_backend in binders:
            if candidate_backend is backend:
                return binder

        binder = BackendDataBinder(backend)
        binders.append((binder, backend))
        return binder

    return _backend_data_binder_factory

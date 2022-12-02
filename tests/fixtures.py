# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import re
import subprocess
import sys
from collections import defaultdict
from hashlib import sha1
from pathlib import Path
from typing import Iterable, Optional, Tuple, Union

import attr
import pytest

from parsec._parsec import DateTime, EnrollmentID
from parsec.api.data import (
    DataError,
    DeviceCertificate,
    PkiEnrollmentAnswerPayload,
    PkiEnrollmentSubmitPayload,
    RealmRoleCertificate,
    RevokedUserCertificate,
    UserCertificate,
    UserManifest,
)
from parsec.api.protocol import (
    DeviceID,
    DeviceLabel,
    HumanHandle,
    OrganizationID,
    RealmID,
    RealmRole,
    UserID,
    UserProfile,
    VlobID,
)
from parsec.backend.backend_events import BackendEvent
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.user import Device as BackendDevice
from parsec.backend.user import User as BackendUser
from parsec.core.fs.storage import UserStorage
from parsec.core.local_device import (
    DeviceFileType,
    LocalDeviceCryptoError,
    LocalDeviceNotFoundError,
    LocalDevicePackingError,
    _load_device,
    _save_device,
    generate_new_device,
)
from parsec.core.types import (
    BackendOrganizationBootstrapAddr,
    BackendPkiEnrollmentAddr,
    LocalDevice,
    LocalUserManifest,
)
from parsec.core.types.pki import LocalPendingEnrollment, X509Certificate
from parsec.crypto import PrivateKey, SigningKey
from tests.common import addr_with_device_subdomain, freeze_time


@pytest.fixture
def fixtures_customization(request):
    try:
        return request.node.function._fixtures_customization
    except AttributeError:
        return {}


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

    def _organization_factory(orgname=None):
        nonlocal count

        if not orgname:
            count += 1
            orgname = f"Org{count}"

        organization_id = OrganizationID(orgname)
        assert organization_id not in organizations
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
        base_device_id: Optional[Union[str, DeviceID]] = None,
        org: OrganizationFullData = coolorg,
        profile: Optional[UserProfile] = None,
        has_human_handle: bool = True,
        base_human_handle: Optional[Union[str, HumanHandle]] = None,
        has_device_label: bool = True,
        base_device_label: Optional[Union[str, DeviceLabel]] = None,
    ):
        nonlocal count

        if not base_device_id:
            count += 1
            base_device_id = f"user{count}@dev0"

        org_devices = devices[org.organization_id]
        if isinstance(base_device_id, DeviceID):
            device_id = base_device_id
        else:
            device_id = DeviceID(base_device_id)
        assert not any(d for d in org_devices if d.device_id == device_id)

        if not has_device_label:
            assert base_device_label is None
            device_label = None
        elif not base_device_label:
            device_label = DeviceLabel(f"My {device_id.device_name} machine")
        elif isinstance(base_device_label, DeviceLabel):
            device_label = base_device_label
        else:
            device_label = DeviceLabel(base_device_label)

        if not has_human_handle:
            assert base_human_handle is None
            human_handle = None
        elif not base_human_handle:
            name = str(device_id.user_id).capitalize()
            human_handle = HumanHandle(
                email=f"{device_id.user_id.str}@example.com", label=f"{name}y Mc{name}Face"
            )
        elif isinstance(base_human_handle, HumanHandle):
            human_handle = base_human_handle
        else:
            match = re.match(r"(.*) <(.*)>", base_human_handle)
            if match:
                label, email = match.groups()
            else:
                label = base_human_handle
                email = f"{device_id.user_id.str}@example.com"
            human_handle = HumanHandle(email=email, label=label)

        parent_device = None
        try:
            # If the user already exists, we must retrieve it data
            parent_device = next(d for d in org_devices if d.user_id == device_id.user_id)
            if profile is not None and profile != parent_device.profile:
                raise ValueError(
                    "profile is set but user already exists, with a different profile value."
                )
            profile = parent_device.profile

        except StopIteration:
            profile = profile or UserProfile.STANDARD

        # Force each device to access the backend trough a different hostname so
        # tcp stream spy can switch offline certains while keeping the others online
        org_addr = addr_with_device_subdomain(org.addr, device_id)

        device = generate_new_device(
            organization_addr=org_addr,
            device_id=device_id,
            profile=profile,
            human_handle=human_handle,
            device_label=device_label,
        )
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
    expired_org = organization_factory("ExpiredOrg")
    return expired_org


@pytest.fixture
def otheralice(fixtures_customization, local_device_factory, otherorg):
    return local_device_factory(
        "alice@dev1",
        otherorg,
        # otheralice mimics alice
        profile=fixtures_customization.get("alice_profile", UserProfile.ADMIN),
        has_human_handle=fixtures_customization.get("alice_has_human_handle", True),
        has_device_label=fixtures_customization.get("alice_has_device_label", True),
    )


@pytest.fixture
def alice(fixtures_customization, local_device_factory, initial_user_manifest_state):
    device = local_device_factory(
        "alice@dev1",
        profile=fixtures_customization.get("alice_profile", UserProfile.ADMIN),
        has_human_handle=fixtures_customization.get("alice_has_human_handle", True),
        has_device_label=fixtures_customization.get("alice_has_device_label", True),
    )
    # Force alice user manifest v1 to be signed by user alice@dev1
    # This is needed given backend_factory bind alice@dev1 then alice@dev2,
    # hence user manifest v1 is stored in backend at a time when alice@dev2
    # doesn't exists.
    with freeze_time("2000-01-01"):
        initial_user_manifest_state.force_user_manifest_v1_generation(device)
    return device


@pytest.fixture
def expiredorgalice(
    fixtures_customization, local_device_factory, initial_user_manifest_state, expiredorg
):
    device = local_device_factory(
        "alice@dev1",
        expiredorg,
        # expiredorgalice mimics alice
        profile=fixtures_customization.get("alice_profile", UserProfile.ADMIN),
        has_human_handle=fixtures_customization.get("alice_has_human_handle", True),
        has_device_label=fixtures_customization.get("alice_has_device_label", True),
    )
    # Force alice user manifest v1 to be signed by user alice@dev1
    # This is needed given backend_factory bind alice@dev1 then alice@dev2,
    # hence user manifest v1 is stored in backend at a time when alice@dev2
    # doesn't exists.
    with freeze_time("2000-01-01"):
        initial_user_manifest_state.force_user_manifest_v1_generation(device)
    return device


@pytest.fixture
def alice2(fixtures_customization, local_device_factory):
    return local_device_factory(
        "alice@dev2",
        profile=fixtures_customization.get("alice_profile", UserProfile.ADMIN),
        has_human_handle=fixtures_customization.get("alice_has_human_handle", True),
        has_device_label=fixtures_customization.get("alice_has_device_label", True),
    )


@pytest.fixture
def adam(fixtures_customization, local_device_factory):
    return local_device_factory(
        "adam@dev1",
        profile=fixtures_customization.get("adam_profile", UserProfile.ADMIN),
        has_human_handle=fixtures_customization.get("adam_has_human_handle", True),
        has_device_label=fixtures_customization.get("adam_has_device_label", True),
    )


@pytest.fixture
def bob(fixtures_customization, local_device_factory):
    return local_device_factory(
        "bob@dev1",
        profile=fixtures_customization.get("bob_profile", UserProfile.STANDARD),
        has_human_handle=fixtures_customization.get("bob_has_human_handle", True),
        has_device_label=fixtures_customization.get("bob_has_device_label", True),
    )


@pytest.fixture
def mallory(fixtures_customization, local_device_factory):
    return local_device_factory(
        "mallory@dev1",
        profile=fixtures_customization.get("mallory_profile", UserProfile.STANDARD),
        has_human_handle=fixtures_customization.get("mallory_has_human_handle", True),
        has_device_label=fixtures_customization.get("mallory_has_device_label", True),
    )


class InitialUserManifestState:
    def __init__(self):
        self._v1 = {}

    def _generate_or_retrieve_user_manifest_v1(self, device):
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
                workspaces=(),
            )
            local_user_manifest = LocalUserManifest.from_remote(remote_user_manifest)
            self._v1[(device.organization_id, device.user_id)] = (
                remote_user_manifest,
                local_user_manifest,
            )
            return self._v1[(device.organization_id, device.user_id)]

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


@pytest.fixture
def initialize_local_user_manifest(initial_user_manifest_state):
    async def _initialize_local_user_manifest(
        data_base_dir, device, initial_user_manifest: str
    ) -> None:
        assert initial_user_manifest in ("non_speculative_v0", "speculative_v0", "v1")
        # Create a storage just for this operation (the underlying database
        # will be reused by the core's storage thanks to `persistent_mockup`)
        with freeze_time("2000-01-01", device=device) as timestamp:
            async with UserStorage.run(data_base_dir, device) as storage:
                assert storage.get_user_manifest().base_version == 0

                if initial_user_manifest == "v1":
                    user_manifest = initial_user_manifest_state.get_user_manifest_v1_for_device(
                        storage.device
                    )
                    await storage.set_user_manifest(user_manifest)
                    # Chcekpoint 1 *is* the upload of user manifest v1
                    await storage.update_realm_checkpoint(1, {})

                elif initial_user_manifest == "non_speculative_v0":
                    user_manifest = LocalUserManifest.new_placeholder(
                        author=storage.device.device_id,
                        id=storage.device.user_manifest_id,
                        timestamp=timestamp,
                        speculative=False,
                    )
                    await storage.set_user_manifest(user_manifest)

                else:
                    # Nothing to do given speculative placeholder is the default
                    assert initial_user_manifest == "speculative_v0"

    return _initialize_local_user_manifest


def local_device_to_backend_user(
    device: LocalDevice, certifier: Union[LocalDevice, OrganizationFullData]
) -> Tuple[BackendUser, BackendDevice]:
    if isinstance(certifier, OrganizationFullData):
        certifier_id = None
        certifier_signing_key = certifier.root_signing_key
    else:
        certifier_id = certifier.device_id
        certifier_signing_key = certifier.signing_key

    timestamp = device.timestamp()

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
        profile=device.profile,
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

                # The realm needs to be created srictly before the manifest timestamp
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

                await self.backend.vlob.create(
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

                # Avoid possible race condition in tests listening for events
                await spy.wait_multiple(
                    [
                        (
                            BackendEvent.REALM_ROLES_UPDATED,
                            {
                                "organization_id": author.organization_id,
                                "author": author.device_id,
                                "realm_id": realm_id,
                                "user": author.user_id,
                                "role": RealmRole.OWNER,
                            },
                        ),
                        (
                            BackendEvent.REALM_VLOBS_UPDATED,
                            {
                                "organization_id": author.organization_id,
                                "author": author.device_id,
                                "realm_id": realm_id,
                                "checkpoint": 1,
                                "src_id": vlob_id,
                                "src_version": 1,
                            },
                        ),
                    ]
                )

        async def bind_organization(
            self,
            org: OrganizationFullData,
            first_device: LocalDevice,
            initial_user_manifest: str = "v1",
        ):
            assert initial_user_manifest in ("v1", "not_synced")

            await self.backend.organization.create(org.organization_id, org.bootstrap_token)
            assert org.organization_id == first_device.organization_id
            backend_user, backend_first_device = local_device_to_backend_user(first_device, org)
            await self.backend.organization.bootstrap(
                id=org.organization_id,
                user=backend_user,
                first_device=backend_first_device,
                bootstrap_token=org.bootstrap_token,
                root_verify_key=org.root_verify_key,
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
            certifier: Optional[LocalDevice] = None,
            initial_user_manifest: Optional[str] = None,
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

            backend_user, backend_device = local_device_to_backend_user(device, certifier)

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


def create_test_certificates(tmp_path):
    """
    Requires openssl to be installed. Only used on linux when parsec_ext is installed.
    """
    # Create openssl config
    (tmp_path / "openssl.cnf").write_text("keyUsage = digitalSignature")

    # OpenSSL commands for creating the CA key, the CA certificate, the client key and the client certificate.
    commands = """\
openssl req -x509 -newkey rsa:4096 -keyout ca_key.pem -out ca_cert.pem -sha256 -days 365 -subj "/CN=My CA" -addext "keyUsage = digitalSignature" -passout pass:P@ssw0rd
openssl req -out client_request.csr -newkey rsa:4096 -keyout client_key.pem -new -passout pass:P@ssw0rd -subj "/CN=John Doe/emailAddress=john@example.com"
openssl x509 -req -days 365 -in client_request.csr -CA ca_cert.pem -CAkey ca_key.pem -passin pass:P@ssw0rd -CAcreateserial -out client_cert.pem -extfile "./openssl.cnf"
"""

    # Run the commands
    for command in commands.splitlines():
        subprocess.run(command, shell=True, check=True, cwd=tmp_path)

    # Concatenate the key and the certificate
    combined_data = (tmp_path / "client_cert.pem").read_bytes() + (
        tmp_path / "client_key.pem"
    ).read_bytes()
    (tmp_path / "client_combined.pem").write_bytes(combined_data)

    # Return certificates and password
    return tmp_path / "ca_cert.pem", tmp_path / "client_combined.pem", "P@ssw0rd"


def use_actual_parsec_ext_module(monkeypatch, tmp_path):
    """
    All tests using `mocked_parsec_ext_smartcard` can also run with the actual `parsec_ext` module if it is installed.

    It is currently not integrated in the CI but it is useful to run locally.
    """
    # We only generate test certificates for linux, that's enough for the moment
    if sys.platform == "win32":
        return pytest.skip(reason="Linux only")

    # Simply ignore this test if parsec_ext is not installed, which is typicall the case in the CI
    try:
        import parsec_ext.smartcard
    except ModuleNotFoundError:
        return pytest.skip(reason="parsec_ext not installed")

    # Generate tests certificate
    ca_cert, client_cert, password = create_test_certificates(tmp_path)

    # Patch parsec_ext to automatically select those test certificates
    monkeypatch.setattr(
        "parsec_ext.smartcard.linux.prompt_file_selection", lambda: str(client_cert)
    )
    monkeypatch.setattr("parsec_ext.smartcard.linux.prompt_password", lambda _: password)

    # Create the corresponding instance of `X509Certificate``
    (
        der_certificate,
        certificate_id,
        certificate_sha1,
    ) = parsec_ext.smartcard.get_der_encoded_certificate()
    issuer, subject, _ = parsec_ext.smartcard.get_certificate_info(der_certificate)
    default_x509_certificate = X509Certificate(
        issuer=issuer,
        subject=subject,
        der_x509_certificate=der_certificate,
        certificate_id=certificate_id,
        certificate_sha1=certificate_sha1,
    )

    # Add two attributes to the smartcard modules that are going to be used the tests
    monkeypatch.setattr(
        "parsec_ext.smartcard.default_x509_certificate", default_x509_certificate, raising=False
    )
    monkeypatch.setattr("parsec_ext.smartcard.default_trust_root_path", ca_cert, raising=False)

    # Return the non-mocked module
    return parsec_ext.smartcard


@pytest.fixture(params=(True, False), ids=("mock_parsec_ext", "use_parsec_ext"))
def mocked_parsec_ext_smartcard(monkeypatch, request, tmp_path):
    if not request.param:
        return use_actual_parsec_ext_module(monkeypatch, tmp_path)

    class MockedParsecExtSmartcard:
        def __init__(self):
            self._pending_enrollments = defaultdict(list)
            der_x509_certificate = b"der_X509_certificate:42:john@example.com:John Doe:My CA"
            self.default_trust_root_path = "some_path.pem"
            self.default_x509_certificate = X509Certificate(
                issuer={"common_name": "My CA"},
                subject={"common_name": "John Doe", "email_address": "john@example.com"},
                der_x509_certificate=der_x509_certificate,
                certificate_sha1=sha1(der_x509_certificate).digest(),
                certificate_id=42,
            )

        @staticmethod
        def _compute_signature(der_x509_certificate: bytes, payload: bytes) -> bytes:
            return sha1(payload + der_x509_certificate).digest()  # 100% secure crypto \o/

        def pki_enrollment_select_certificate(
            self, owner_hint: Optional[LocalDevice] = None
        ) -> X509Certificate:
            return self.default_x509_certificate

        def pki_enrollment_sign_payload(
            self, payload: bytes, x509_certificate: X509Certificate
        ) -> bytes:
            return self._compute_signature(x509_certificate.der_x509_certificate, payload)

        def pki_enrollment_create_local_pending(
            self,
            config_dir: Path,
            x509_certificate: X509Certificate,
            addr: BackendPkiEnrollmentAddr,
            enrollment_id: EnrollmentID,
            submitted_on: DateTime,
            submit_payload: PkiEnrollmentSubmitPayload,
            signing_key: SigningKey,
            private_key: PrivateKey,
        ):
            pending = LocalPendingEnrollment(
                x509_certificate=x509_certificate,
                addr=addr,
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                submit_payload=submit_payload,
                encrypted_key=b"123",
                ciphertext=b"123",
            )
            secret_part = (signing_key, private_key)
            self._pending_enrollments[config_dir].append((pending, secret_part))
            return pending

        def pki_enrollment_load_local_pending_secret_part(
            self, config_dir: Path, enrollment_id: EnrollmentID
        ) -> Tuple[SigningKey, PrivateKey]:
            for (pending, secret_part) in self._pending_enrollments[config_dir]:
                if pending.enrollment_id == enrollment_id:
                    return secret_part
            else:
                raise LocalDeviceNotFoundError()

        def pki_enrollment_load_submit_payload(
            self,
            der_x509_certificate: bytes,
            payload_signature: bytes,
            payload: bytes,
            extra_trust_roots: Iterable[Path] = (),
        ) -> PkiEnrollmentSubmitPayload:
            computed_signature = self._compute_signature(der_x509_certificate, payload)
            if computed_signature != payload_signature:
                raise LocalDeviceCryptoError()
            try:
                submit_payload = PkiEnrollmentSubmitPayload.load(payload)
            except DataError as exc:
                raise LocalDevicePackingError(str(exc)) from exc

            return submit_payload

        def pki_enrollment_load_accept_payload(
            self,
            der_x509_certificate: bytes,
            payload_signature: bytes,
            payload: bytes,
            extra_trust_roots: Iterable[Path] = (),
        ) -> PkiEnrollmentAnswerPayload:
            computed_signature = self._compute_signature(der_x509_certificate, payload)
            if computed_signature != payload_signature:
                raise LocalDeviceCryptoError()
            try:
                accept_payload = PkiEnrollmentAnswerPayload.load(payload)
            except DataError as exc:
                raise LocalDevicePackingError(str(exc)) from exc

            return accept_payload

        def save_device_with_smartcard(
            self,
            key_file: Path,
            device: LocalDevice,
            force: bool = False,
            certificate_id: Optional[str] = None,
            certificate_sha1: Optional[bytes] = None,
        ) -> None:
            def _encrypt_dump(cleartext: bytes) -> Tuple[DeviceFileType, bytes, dict]:
                extra_args = {
                    "encrypted_key": b"123",
                    "certificate_id": certificate_id,
                    "certificate_sha1": certificate_sha1,
                }
                return DeviceFileType.SMARTCARD, cleartext, extra_args

            _save_device(key_file, device, force, _encrypt_dump)

        def load_device_with_smartcard(self, key_file: Path) -> LocalDevice:
            def _decrypt_ciphertext(data: dict) -> bytes:
                return data["ciphertext"]

            return _load_device(key_file, _decrypt_ciphertext)

        def pki_enrollment_load_peer_certificate(
            self, der_x509_certificate: bytes
        ) -> X509Certificate:
            if not der_x509_certificate.startswith(b"der_X509_certificate:"):
                # Consider the certificate invalid
                raise LocalDeviceCryptoError()

            _, certificate_id, subject_email, subject_cn, issuer_cn = der_x509_certificate.decode(
                "utf8"
            ).split(":")
            return X509Certificate(
                issuer={"common_name": issuer_cn},
                subject={"email_address": subject_email, "common_name": subject_cn},
                der_x509_certificate=der_x509_certificate,
                certificate_sha1=sha1(der_x509_certificate).digest(),
                certificate_id=certificate_id,
            )

    mocked_parsec_ext_smartcard = MockedParsecExtSmartcard()

    def _mocked_load_smartcard_extension():
        return mocked_parsec_ext_smartcard

    monkeypatch.setattr(
        "parsec.core.pki.plumbing._load_smartcard_extension", _mocked_load_smartcard_extension
    )
    monkeypatch.setattr(
        "parsec.core.local_device._load_smartcard_extension", _mocked_load_smartcard_extension
    )
    return mocked_parsec_ext_smartcard

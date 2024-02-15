# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from contextlib import asynccontextmanager
from dataclasses import dataclass, field
from typing import TYPE_CHECKING, Any, AsyncGenerator, Type

from parsec._parsec import (
    BootstrapToken,
    DateTime,
    DeviceCertificate,
    DeviceID,
    OrganizationID,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SequesterServiceCertificate,
    UserUpdateCertificate,
    VerifyKey,
)
from parsec.api import collect_apis
from parsec.components.auth import BaseAuthComponent
from parsec.components.block import BaseBlockComponent
from parsec.components.blockstore import BaseBlockStoreComponent
from parsec.components.events import BaseEventsComponent, EventBus
from parsec.components.invite import BaseInviteComponent
from parsec.components.memory import components_factory as mocked_components_factory
from parsec.components.memory.datamodel import MemoryDatamodel
from parsec.components.organization import BaseOrganizationComponent
from parsec.components.ping import BasePingComponent
from parsec.components.pki import BasePkiEnrollmentComponent
from parsec.components.postgresql import components_factory as postgresql_components_factory
from parsec.components.realm import BaseRealmComponent
from parsec.components.sequester import BaseSequesterComponent
from parsec.components.user import BaseUserComponent
from parsec.components.vlob import BaseVlobComponent
from parsec.config import BackendConfig
from parsec.webhooks import WebhooksComponent

if TYPE_CHECKING:
    from parsec.api import ApiFn


@asynccontextmanager
async def backend_factory(config: BackendConfig) -> AsyncGenerator[Backend, None]:
    if config.db_url == "MOCKED":
        components_factory = mocked_components_factory
    else:
        components_factory = postgresql_components_factory

    async with components_factory(config=config) as components:
        yield Backend(
            config=config,
            mocked_data=components.get("mocked_data"),
            event_bus=components["event_bus"],
            webhooks=components["webhooks"],
            auth=components["auth"],
            user=components["user"],
            invite=components["invite"],
            organization=components["organization"],
            realm=components["realm"],
            vlob=components["vlob"],
            ping=components["ping"],
            blockstore=components["blockstore"],
            block=components["block"],
            pki=components["pki"],
            sequester=components["sequester"],
            events=components["events"],
        )


@dataclass(slots=True, eq=False, repr=False)
class Backend:
    config: BackendConfig
    event_bus: EventBus
    webhooks: WebhooksComponent
    auth: BaseAuthComponent
    user: BaseUserComponent
    invite: BaseInviteComponent
    organization: BaseOrganizationComponent
    realm: BaseRealmComponent
    vlob: BaseVlobComponent
    ping: BasePingComponent
    blockstore: BaseBlockStoreComponent
    block: BaseBlockComponent
    pki: BasePkiEnrollmentComponent
    sequester: BaseSequesterComponent
    events: BaseEventsComponent

    # Only available if `config.db_url == "MOCKED"`
    mocked_data: MemoryDatamodel | None = None

    apis: dict[Type[Any], ApiFn] = field(init=False)

    def __post_init__(self) -> None:
        self.apis = collect_apis(
            self.user,
            self.invite,
            self.organization,
            self.realm,
            self.vlob,
            self.ping,
            self.blockstore,
            self.block,
            self.pki,
            self.events,
            # Ping command is only used in tests
            include_ping=self.config.debug,
        )

    async def test_duplicate_organization(self, id: OrganizationID, new_id: OrganizationID) -> None:
        await self.organization.test_duplicate_organization(id, new_id)

    async def test_drop_organization(self, id: OrganizationID) -> None:
        await self.organization.test_drop_organization(id)

    async def test_load_template(self, template: Any) -> OrganizationID:
        from parsec._parsec import testbed

        org_id = OrganizationID(f"{template.id.capitalize()}OrgTemplate")
        match await self.organization.create(now=DateTime(1970, 1, 1), id=org_id):
            case BootstrapToken() as bootstrap_token:
                pass
            case error:
                assert False, error

        verify_key_per_device: dict[DeviceID, VerifyKey] = {}

        for event in template.events:
            if isinstance(event, testbed.TestbedEventBootstrapOrganization):
                outcome = await self.organization.bootstrap(
                    id=org_id,
                    now=event.timestamp,
                    bootstrap_token=bootstrap_token,
                    root_verify_key=event.root_signing_key.verify_key,
                    user_certificate=event.first_user_raw_certificate,
                    device_certificate=event.first_user_first_device_raw_certificate,
                    redacted_user_certificate=event.first_user_raw_redacted_certificate,
                    redacted_device_certificate=event.first_user_first_device_raw_redacted_certificate,
                    sequester_authority_certificate=event.sequester_authority_raw_certificate,
                )
                assert isinstance(outcome, tuple), outcome
                device_certif = outcome[1]
                verify_key_per_device[device_certif.device_id] = device_certif.verify_key
            elif isinstance(event, testbed.TestbedEventNewSequesterService):
                outcome = await self.sequester.create_storage_service(
                    now=event.timestamp,
                    organization_id=org_id,
                    service_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, SequesterServiceCertificate), outcome
            elif isinstance(event, testbed.TestbedEventRevokeSequesterService):
                outcome = await self.sequester.revoke_service(
                    now=event.timestamp,
                    organization_id=org_id,
                    revoked_service_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, SequesterServiceCertificate), outcome
            elif isinstance(event, testbed.TestbedEventNewUser):
                outcome = await self.user.create_user(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=verify_key_per_device[event.author],
                    user_certificate=event.user_raw_certificate,
                    redacted_user_certificate=event.user_raw_redacted_certificate,
                    device_certificate=event.first_device_raw_certificate,
                    redacted_device_certificate=event.first_device_raw_redacted_certificate,
                )
                assert isinstance(outcome, tuple), outcome
                device_certif = outcome[1]
                verify_key_per_device[device_certif.device_id] = device_certif.verify_key
            elif isinstance(event, testbed.TestbedEventNewDevice):
                outcome = await self.user.create_device(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=verify_key_per_device[event.author],
                    device_certificate=event.raw_certificate,
                    redacted_device_certificate=event.raw_redacted_certificate,
                )
                assert isinstance(outcome, DeviceCertificate), outcome
                device_certif = outcome
                verify_key_per_device[device_certif.device_id] = device_certif.verify_key
            elif isinstance(event, testbed.TestbedEventUpdateUserProfile):
                outcome = await self.user.update_user(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=verify_key_per_device[event.author],
                    user_update_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, UserUpdateCertificate), outcome
            elif isinstance(event, testbed.TestbedEventRevokeUser):
                outcome = await self.user.revoke_user(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=verify_key_per_device[event.author],
                    revoked_user_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, RevokedUserCertificate), outcome
            elif isinstance(event, testbed.TestbedEventNewDeviceInvitation):
                outcome = await self.invite.new_for_device(
                    now=event.created_on,
                    organization_id=org_id,
                    author=event.greeter_user_id,
                    send_email=False,
                    force_token=event.token,
                )
                assert isinstance(outcome, tuple), outcome
            elif isinstance(event, testbed.TestbedEventNewUserInvitation):
                outcome = await self.invite.new_for_user(
                    now=event.created_on,
                    organization_id=org_id,
                    author=event.greeter_user_id,
                    claimer_email=event.claimer_email,
                    send_email=False,
                    force_token=event.token,
                )
                assert isinstance(outcome, tuple), outcome
            elif isinstance(event, testbed.TestbedEventNewRealm):
                outcome = await self.realm.create(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    realm_role_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, RealmRoleCertificate), outcome
            elif isinstance(event, testbed.TestbedEventShareRealm):
                if event.role is None:
                    outcome = await self.realm.unshare(
                        now=event.timestamp,
                        organization_id=org_id,
                        author=event.author,
                        realm_role_certificate=event.raw_certificate,
                    )
                else:
                    assert event.key_index is not None
                    assert event.recipient_keys_bundle_access is not None

                    outcome = await self.realm.share(
                        now=event.timestamp,
                        organization_id=org_id,
                        author=event.author,
                        realm_role_certificate=event.raw_certificate,
                        key_index=event.key_index,
                        recipient_keys_bundle_access=event.recipient_keys_bundle_access,
                    )
                assert isinstance(outcome, RealmRoleCertificate), outcome
            elif isinstance(event, testbed.TestbedEventRenameRealm):
                outcome = await self.realm.rename(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    realm_name_certificate=event.raw_certificate,
                    initial_name_or_fail=False,
                )
                assert isinstance(outcome, RealmNameCertificate)
            elif isinstance(event, testbed.TestbedEventRotateKeyRealm):
                outcome = await self.realm.rotate_key(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    realm_key_rotation_certificate=event.raw_certificate,
                    per_participant_keys_bundle_access=event.per_participant_keys_bundle_access,
                    keys_bundle=event.keys_bundle,
                )
                assert isinstance(outcome, RealmKeyRotationCertificate)
            elif isinstance(event, testbed.TestbedEventArchiveRealm):
                # TODO: Realm archiving not implemented yet !
                raise NotImplementedError
            elif isinstance(event, testbed.TestbedEventNewShamirRecovery):
                # TODO: Shamir not implemented yet !
                raise NotImplementedError
            elif isinstance(event, testbed.TestbedEventCreateBlock):
                outcome = await self.block.create(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    realm_id=event.realm,
                    block_id=event.block_id,
                    block=event.encrypted,
                )
                assert outcome is None, outcome
            elif isinstance(event, testbed.TestbedEventCreateOpaqueBlock):
                outcome = await self.block.create(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    realm_id=event.realm,
                    block_id=event.block_id,
                    block=event.encrypted,
                )
                assert outcome is None, outcome
            elif isinstance(event, testbed.TestbedEventCreateOrUpdateOpaqueVlob):
                if event.version == 1:
                    outcome = await self.vlob.create(
                        now=event.timestamp,
                        organization_id=org_id,
                        author=event.author,
                        realm_id=event.realm,
                        key_index=event.key_index,
                        vlob_id=event.vlob_id,
                        timestamp=event.timestamp,
                        blob=event.encrypted,
                        sequester_blob=event.sequestered,
                    )
                    assert outcome is None, outcome
                else:
                    outcome = await self.vlob.update(
                        now=event.timestamp,
                        organization_id=org_id,
                        author=event.author,
                        vlob_id=event.vlob_id,
                        key_index=event.key_index,
                        version=event.version,
                        timestamp=event.timestamp,
                        blob=event.encrypted,
                        sequester_blob=event.sequestered,
                    )
                    assert outcome is None, outcome

        return org_id

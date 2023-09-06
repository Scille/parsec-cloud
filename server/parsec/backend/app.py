# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from contextlib import asynccontextmanager
from typing import Any, AsyncGenerator, Callable, Dict, Type

import attr

from parsec._parsec import OrganizationID, RealmRole, UserProfile
from parsec.backend.block import BaseBlockComponent
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.client_context import BaseClientContext
from parsec.backend.config import BackendConfig
from parsec.backend.events import EventsComponent
from parsec.backend.invite import BaseInviteComponent
from parsec.backend.memory import components_factory as mocked_components_factory
from parsec.backend.message import BaseMessageComponent
from parsec.backend.organization import BaseOrganizationComponent, SequesterAuthority
from parsec.backend.ping import BasePingComponent
from parsec.backend.pki import BasePkiEnrollmentComponent
from parsec.backend.postgresql import components_factory as postgresql_components_factory
from parsec.backend.realm import BaseRealmComponent, RealmGrantedRole
from parsec.backend.sequester import BaseSequesterComponent, StorageSequesterService
from parsec.backend.user import BaseUserComponent, Device, User
from parsec.backend.utils import collect_apis
from parsec.backend.vlob import BaseVlobComponent
from parsec.backend.webhooks import WebhooksComponent
from parsec.event_bus import EventBus


@asynccontextmanager
async def backend_app_factory(
    config: BackendConfig, event_bus: EventBus | None = None
) -> AsyncGenerator[BackendApp, None]:
    event_bus = event_bus or EventBus()

    if config.db_url == "MOCKED":
        components_factory = mocked_components_factory
    else:
        components_factory = postgresql_components_factory

    async with components_factory(config=config, event_bus=event_bus) as components:
        yield BackendApp(
            config=config,
            event_bus=event_bus,
            webhooks=components["webhooks"],
            user=components["user"],
            invite=components["invite"],
            organization=components["organization"],
            message=components["message"],
            realm=components["realm"],
            vlob=components["vlob"],
            ping=components["ping"],
            blockstore=components["blockstore"],
            block=components["block"],
            pki=components["pki"],
            sequester=components["sequester"],
            events=components["events"],
        )


@attr.s(slots=True, auto_attribs=True, kw_only=True, eq=False, repr=False)
class BackendApp:
    config: BackendConfig
    event_bus: EventBus
    webhooks: WebhooksComponent
    user: BaseUserComponent
    invite: BaseInviteComponent
    organization: BaseOrganizationComponent
    message: BaseMessageComponent
    realm: BaseRealmComponent
    vlob: BaseVlobComponent
    ping: BasePingComponent
    blockstore: BaseBlockStoreComponent
    block: BaseBlockComponent
    pki: BasePkiEnrollmentComponent
    sequester: BaseSequesterComponent
    events: EventsComponent

    apis: Dict[Type[Any], Callable[[BaseClientContext, Any], Any]] = attr.field(init=False)

    def __attrs_post_init__(self) -> None:
        self.apis = collect_apis(
            self.user,
            self.invite,
            self.organization,
            self.message,
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

    def test_duplicate_organization(self, id: OrganizationID, new_id: OrganizationID) -> None:
        self.user.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.invite.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.organization.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.message.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.realm.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.vlob.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.ping.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.blockstore.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.block.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.pki.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]
        self.sequester.test_duplicate_organization(id, new_id)  # type: ignore[attr-defined]

    def test_drop_organization(self, id: OrganizationID) -> None:
        self.user.test_drop_organization(id)  # type: ignore[attr-defined]
        self.invite.test_drop_organization(id)  # type: ignore[attr-defined]
        self.organization.test_drop_organization(id)  # type: ignore[attr-defined]
        self.message.test_drop_organization(id)  # type: ignore[attr-defined]
        self.realm.test_drop_organization(id)  # type: ignore[attr-defined]
        self.vlob.test_drop_organization(id)  # type: ignore[attr-defined]
        self.ping.test_drop_organization(id)  # type: ignore[attr-defined]
        self.blockstore.test_drop_organization(id)  # type: ignore[attr-defined]
        self.block.test_drop_organization(id)  # type: ignore[attr-defined]
        self.pki.test_drop_organization(id)  # type: ignore[attr-defined]
        self.sequester.test_drop_organization(id)  # type: ignore[attr-defined]

    async def test_load_template(self, template: Any) -> OrganizationID:
        from parsec._parsec import testbed

        org_id = OrganizationID(f"{template.id.capitalize()}OrgTemplate")
        await self.organization.create(id=org_id, bootstrap_token="")

        for event in template.events:
            if isinstance(event, testbed.TestbedEventBootstrapOrganization):
                await self.organization.bootstrap(
                    id=org_id,
                    user=User(
                        user_id=event.first_user_device_id.user_id,
                        human_handle=event.first_user_human_handle,
                        user_certificate=event.first_user_raw_certificate,
                        redacted_user_certificate=event.first_user_raw_redacted_certificate,
                        user_certifier=None,
                        initial_profile=UserProfile.ADMIN,
                        created_on=event.timestamp,
                    ),
                    first_device=Device(
                        device_id=event.first_user_device_id,
                        device_label=event.first_user_first_device_label,
                        device_certificate=event.first_user_first_device_raw_certificate,
                        redacted_device_certificate=event.first_user_first_device_raw_redacted_certificate,
                        device_certifier=None,
                        created_on=event.timestamp,
                    ),
                    bootstrap_token="",
                    root_verify_key=event.root_signing_key.verify_key,
                    bootstrapped_on=event.timestamp,
                    sequester_authority=(
                        SequesterAuthority(
                            certificate=event.sequester_authority_raw_certificate,
                            verify_key_der=event.sequester_authority_verify_key,
                        )
                    )
                    if event.sequester_authority_raw_certificate
                    and event.sequester_authority_verify_key
                    else None,
                )
            elif isinstance(event, testbed.TestbedEventNewSequesterService):
                await self.sequester.create_service(
                    organization_id=org_id,
                    service=StorageSequesterService(
                        service_id=event.id,
                        service_label=event.label,
                        service_certificate=event.raw_certificate,
                        created_on=event.timestamp,
                        disabled_on=None,
                    ),
                )
            elif isinstance(event, testbed.TestbedEventNewUser):
                await self.user.create_user(
                    organization_id=org_id,
                    user=User(
                        user_id=event.device_id.user_id,
                        human_handle=event.human_handle,
                        user_certificate=event.user_raw_certificate,
                        redacted_user_certificate=event.user_raw_redacted_certificate,
                        user_certifier=event.author,
                        initial_profile=event.initial_profile,
                        created_on=event.timestamp,
                    ),
                    first_device=Device(
                        device_id=event.device_id,
                        device_label=event.first_device_label,
                        device_certificate=event.first_device_raw_certificate,
                        redacted_device_certificate=event.first_device_raw_redacted_certificate,
                        device_certifier=event.author,
                        created_on=event.timestamp,
                    ),
                )
            elif isinstance(event, testbed.TestbedEventNewDevice):
                await self.user.create_device(
                    organization_id=org_id,
                    device=Device(
                        device_id=event.device_id,
                        device_label=event.device_label,
                        device_certificate=event.raw_certificate,
                        redacted_device_certificate=event.raw_redacted_certificate,
                        device_certifier=event.author,
                        created_on=event.timestamp,
                    ),
                )
            elif isinstance(event, testbed.TestbedEventUpdateUserProfile):
                await self.user.update_user(
                    organization_id=org_id,
                    user_id=event.user,
                    new_profile=event.profile,
                    user_update_certificate=event.raw_certificate,
                    user_update_certifier=event.author,
                    updated_on=event.timestamp,
                )
            elif isinstance(event, testbed.TestbedEventRevokeUser):
                await self.user.revoke_user(
                    organization_id=org_id,
                    user_id=event.user,
                    revoked_user_certificate=event.raw_certificate,
                    revoked_user_certifier=event.author,
                    revoked_on=event.timestamp,
                )
            elif isinstance(event, testbed.TestbedEventNewRealm):
                await self.realm.create(
                    organization_id=org_id,
                    self_granted_role=RealmGrantedRole(
                        certificate=event.raw_certificate,
                        realm_id=event.realm_id,
                        user_id=event.author.user_id,
                        role=RealmRole.OWNER,
                        granted_by=event.author,
                        granted_on=event.timestamp,
                    ),
                )
            elif isinstance(event, testbed.TestbedEventShareRealm):
                await self.realm.update_roles(
                    organization_id=org_id,
                    new_role=RealmGrantedRole(
                        certificate=event.raw_certificate,
                        realm_id=event.realm,
                        user_id=event.author.user_id,
                        role=RealmRole.OWNER,
                        granted_by=event.author,
                        granted_on=event.timestamp,
                    ),
                    recipient_message=event.recipient_message,
                )
            elif isinstance(event, testbed.TestbedEventStartRealmReencryption):
                await self.realm.start_reencryption_maintenance(
                    organization_id=org_id,
                    author=event.author,
                    realm_id=event.realm,
                    encryption_revision=event.encryption_revision,
                    per_participant_message=dict(event.per_participant_message),
                    timestamp=event.timestamp,
                )
            elif isinstance(event, testbed.TestbedEventFinishRealmReencryption):
                await self.realm.finish_reencryption_maintenance(
                    organization_id=org_id,
                    author=event.author,
                    realm_id=event.realm,
                    encryption_revision=event.encryption_revision,
                )
            elif isinstance(event, testbed.TestbedEventNewVlob):
                await self.vlob.create(
                    organization_id=org_id,
                    author=event.author,
                    realm_id=event.realm,
                    encryption_revision=event.encryption_revision,
                    vlob_id=event.vlob_id,
                    timestamp=event.timestamp,
                    blob=event.blob,
                    sequester_blob=event.sequester_blob,
                )
            elif isinstance(event, testbed.TestbedEventUpdateVlob):
                await self.vlob.update(
                    organization_id=org_id,
                    author=event.author,
                    encryption_revision=event.encryption_revision,
                    vlob_id=event.vlob,
                    version=event.version,
                    timestamp=event.timestamp,
                    blob=event.blob,
                    sequester_blob=event.sequester_blob,
                )
            else:
                assert isinstance(event, testbed.TestbedEventNewBlock)
                await self.block.create(
                    organization_id=org_id,
                    author=event.author,
                    block_id=event.block_id,
                    realm_id=event.realm,
                    block=event.block,
                    created_on=event.timestamp,
                )

        return org_id

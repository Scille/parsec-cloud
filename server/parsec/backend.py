# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from collections.abc import AsyncGenerator
from contextlib import asynccontextmanager
from dataclasses import dataclass, field
from typing import TYPE_CHECKING, Any

from parsec._parsec import (
    AccessToken,
    DateTime,
    DeviceCertificate,
    DeviceID,
    OrganizationID,
    RealmArchivingCertificate,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SequesterRevokedServiceCertificate,
    SequesterServiceCertificate,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryDeletionCertificate,
    UserUpdateCertificate,
    VerifyKey,
)
from parsec.api import collect_apis
from parsec.components.account import BaseAccountComponent
from parsec.components.async_enrollment import BaseAsyncEnrollmentComponent
from parsec.components.auth import BaseAuthComponent
from parsec.components.block import BaseBlockComponent
from parsec.components.blockstore import BaseBlockStoreComponent
from parsec.components.events import BaseEventsComponent, EventBus
from parsec.components.invite import BaseInviteComponent
from parsec.components.memory import components_factory as mocked_components_factory
from parsec.components.memory.datamodel import MemoryDatamodel
from parsec.components.organization import BaseOrganizationComponent
from parsec.components.ping import BasePingComponent
from parsec.components.postgresql import components_factory as postgresql_components_factory
from parsec.components.realm import BaseRealmComponent
from parsec.components.scws import ScwsComponent
from parsec.components.sequester import BaseSequesterComponent, SequesterServiceType
from parsec.components.shamir import BaseShamirComponent
from parsec.components.totp import BaseTOTPComponent
from parsec.components.user import BaseUserComponent, UserInfo
from parsec.components.vlob import BaseVlobComponent
from parsec.config import BackendConfig
from parsec.logging import get_logger
from parsec.webhooks import WebhooksComponent

if TYPE_CHECKING:
    from parsec.api import ApiFn

    try:
        from parsec._parsec import testbed

        type TestbedEvent = testbed.TestbedEvent  # pyright: ignore[reportRedeclaration]
        type TestbedTemplateContent = testbed.TestbedTemplateContent  # pyright: ignore[reportRedeclaration]
    except ImportError:
        type TestbedEvent = Any
        type TestbedTemplateContent = Any

logger = get_logger()


@asynccontextmanager
async def backend_factory(config: BackendConfig) -> AsyncGenerator[Backend, None]:
    if config.db_config.is_mocked():
        components_factory = mocked_components_factory
    else:
        components_factory = postgresql_components_factory

    async with components_factory(config=config) as components:
        yield Backend(
            config=config,
            mocked_data=components.get("mocked_data"),
            account=components["account"],
            async_enrollment=components["async_enrollment"],
            auth=components["auth"],
            block=components["block"],
            blockstore=components["blockstore"],
            event_bus=components["event_bus"],
            events=components["events"],
            invite=components["invite"],
            organization=components["organization"],
            ping=components["ping"],
            realm=components["realm"],
            scws=components["scws"],
            sequester=components["sequester"],
            shamir=components["shamir"],
            totp=components["totp"],
            user=components["user"],
            vlob=components["vlob"],
            webhooks=components["webhooks"],
        )


TEST_BOOTSTRAP_TOKEN = AccessToken.from_hex("672bc6ba9c43455da28344e975dc72b7")


@dataclass(slots=True, eq=False, repr=False)
class Backend:
    config: BackendConfig
    event_bus: EventBus

    account: BaseAccountComponent
    async_enrollment: BaseAsyncEnrollmentComponent
    auth: BaseAuthComponent
    block: BaseBlockComponent
    blockstore: BaseBlockStoreComponent
    events: BaseEventsComponent
    invite: BaseInviteComponent
    organization: BaseOrganizationComponent
    ping: BasePingComponent
    realm: BaseRealmComponent
    scws: ScwsComponent
    sequester: BaseSequesterComponent
    shamir: BaseShamirComponent
    totp: BaseTOTPComponent
    user: BaseUserComponent
    vlob: BaseVlobComponent
    webhooks: WebhooksComponent

    # Only available if `config.db_config.type == "MOCKED"`
    mocked_data: MemoryDatamodel | None = None

    apis: dict[type[Any], ApiFn] = field(init=False)  # pyright: ignore[reportMissingTypeArgument] Req/Rep are currently untyped

    def __post_init__(self) -> None:
        self.apis = collect_apis(
            self.account,
            self.async_enrollment,
            self.block,
            self.blockstore,
            self.events,
            self.invite,
            self.organization,
            self.ping,
            self.realm,
            self.scws,
            self.shamir,
            self.totp,
            self.user,
            self.vlob,
            # Ping command is only used in tests
            include_ping=self.config.debug,
        )

    async def test_duplicate_organization(self, id: OrganizationID, new_id: OrganizationID) -> None:
        await self.organization.test_duplicate_organization(id, new_id)

    async def test_customize_organization(
        self,
        id: OrganizationID,
        template: TestbedTemplateContent,
        customization: list[TestbedEvent],
    ) -> None:
        await self.apply_events(
            org_id=id,
            events=template.events + customization,
            skip_events_offset=len(template.events),
        )

    async def test_drop_organization(self, id: OrganizationID) -> None:
        await self.organization.test_drop_organization(id)

    async def test_load_template(self, template: TestbedTemplateContent) -> OrganizationID:
        org_id = OrganizationID(f"{template.id.title().replace('_', '')}OrgTemplate")
        match await self.organization.create(
            now=DateTime(1970, 1, 1), id=org_id, force_bootstrap_token=TEST_BOOTSTRAP_TOKEN
        ):
            case AccessToken():
                pass
            case error:
                assert False, error

        await self.apply_events(org_id=org_id, events=template.events)

        return org_id

    async def apply_events(
        self, org_id: OrganizationID, events: list[TestbedEvent], skip_events_offset: int = 0
    ) -> None:
        from parsec._parsec import testbed

        def _get_device_verify_key(device_id: DeviceID) -> VerifyKey:
            for event in events:
                if isinstance(event, testbed.TestbedEventBootstrapOrganization):
                    if event.first_user_first_device_id == device_id:
                        return event.first_user_first_device_certificate.verify_key
                elif isinstance(event, testbed.TestbedEventNewUser):
                    if event.first_device_id == device_id:
                        return event.first_device_certificate.verify_key
                elif isinstance(event, testbed.TestbedEventNewDevice):
                    if event.device_id == device_id:
                        return event.certificate.verify_key
            else:
                raise ValueError(f"Device {device_id} not found in events")

        for event in events[skip_events_offset:]:
            if isinstance(event, testbed.TestbedEventBootstrapOrganization):
                outcome = await self.organization.bootstrap(
                    id=org_id,
                    now=event.timestamp,
                    bootstrap_token=TEST_BOOTSTRAP_TOKEN,
                    root_verify_key=event.root_signing_key.verify_key,
                    user_certificate=event.first_user_raw_certificate,
                    device_certificate=event.first_user_first_device_raw_certificate,
                    redacted_user_certificate=event.first_user_raw_redacted_certificate,
                    redacted_device_certificate=event.first_user_first_device_raw_redacted_certificate,
                    sequester_authority_certificate=event.sequester_authority_raw_certificate,
                )
                assert isinstance(outcome, tuple), outcome
            elif isinstance(event, testbed.TestbedEventNewSequesterService):
                outcome = await self.sequester.create_service(
                    now=event.timestamp,
                    organization_id=org_id,
                    service_certificate=event.raw_certificate,
                    config=SequesterServiceType.STORAGE,
                )
                assert isinstance(outcome, SequesterServiceCertificate), outcome
            elif isinstance(event, testbed.TestbedEventRevokeSequesterService):
                outcome = await self.sequester.revoke_service(
                    now=event.timestamp,
                    organization_id=org_id,
                    revoked_service_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, SequesterRevokedServiceCertificate), outcome
            elif isinstance(event, testbed.TestbedEventNewUser):
                outcome = await self.user.create_user(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=_get_device_verify_key(event.author),
                    user_certificate=event.user_raw_certificate,
                    redacted_user_certificate=event.user_raw_redacted_certificate,
                    device_certificate=event.first_device_raw_certificate,
                    redacted_device_certificate=event.first_device_raw_redacted_certificate,
                )
                assert isinstance(outcome, tuple), outcome
            elif isinstance(event, testbed.TestbedEventNewDevice):
                outcome = await self.user.create_device(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=_get_device_verify_key(event.author),
                    device_certificate=event.raw_certificate,
                    redacted_device_certificate=event.raw_redacted_certificate,
                )
                assert isinstance(outcome, DeviceCertificate), outcome
            elif isinstance(event, testbed.TestbedEventUpdateUserProfile):
                outcome = await self.user.update_user(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=_get_device_verify_key(event.author),
                    user_update_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, UserUpdateCertificate), outcome
            elif isinstance(event, testbed.TestbedEventRevokeUser):
                outcome = await self.user.revoke_user(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=_get_device_verify_key(event.author),
                    revoked_user_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, RevokedUserCertificate), outcome
            elif isinstance(event, testbed.TestbedEventNewDeviceInvitation):
                outcome = await self.invite.new_for_device(
                    now=event.created_on,
                    organization_id=org_id,
                    author=event.created_by,
                    send_email=False,
                    force_token=event.token,
                )
                assert isinstance(outcome, tuple), outcome
            elif isinstance(event, testbed.TestbedEventNewUserInvitation):
                outcome = await self.invite.new_for_user(
                    now=event.created_on,
                    organization_id=org_id,
                    author=event.created_by,
                    claimer_email=event.claimer_email,
                    send_email=False,
                    force_token=event.token,
                )
                assert isinstance(outcome, tuple), outcome
            elif isinstance(event, testbed.TestbedEventNewShamirRecoveryInvitation):
                outcome = await self.invite.new_for_shamir_recovery(
                    claimer_user_id=event.claimer,
                    now=event.created_on,
                    organization_id=org_id,
                    author=event.created_by,
                    send_email=False,
                    force_token=event.token,
                )
                assert isinstance(outcome, tuple), outcome
            elif isinstance(event, testbed.TestbedEventNewRealm):
                outcome = await self.realm.create(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=_get_device_verify_key(event.author),
                    realm_role_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, RealmRoleCertificate), outcome
            elif isinstance(event, testbed.TestbedEventShareRealm):
                if event.role is None:
                    outcome = await self.realm.unshare(
                        now=event.timestamp,
                        organization_id=org_id,
                        author=event.author,
                        author_verify_key=_get_device_verify_key(event.author),
                        realm_role_certificate=event.raw_certificate,
                    )
                else:
                    assert event.key_index is not None
                    assert event.recipient_keys_bundle_access is not None
                    outcome = await self.realm.share(
                        now=event.timestamp,
                        organization_id=org_id,
                        author=event.author,
                        author_verify_key=_get_device_verify_key(event.author),
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
                    author_verify_key=_get_device_verify_key(event.author),
                    realm_name_certificate=event.raw_certificate,
                    initial_name_or_fail=False,
                )
                assert isinstance(outcome, RealmNameCertificate)
            elif isinstance(event, testbed.TestbedEventRotateKeyRealm):
                outcome = await self.realm.rotate_key(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=_get_device_verify_key(event.author),
                    realm_key_rotation_certificate=event.raw_certificate,
                    per_participant_keys_bundle_access=event.per_participant_keys_bundle_access,
                    per_sequester_service_keys_bundle_access=event.per_sequester_service_keys_bundle_access,
                    keys_bundle=event.keys_bundle,
                )
                assert isinstance(outcome, RealmKeyRotationCertificate)
            elif isinstance(event, testbed.TestbedEventArchiveRealm):
                outcome = await self.realm.update_archiving(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    author_verify_key=_get_device_verify_key(event.author),
                    realm_archiving_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, RealmArchivingCertificate)
            elif isinstance(event, testbed.TestbedEventDeleteRealm):
                outcome = await self.realm.delete_2_do_delete_metadata(
                    now=event.timestamp,
                    organization_id=org_id,
                    realm_id=event.realm,
                )
                assert outcome is None
            elif isinstance(event, testbed.TestbedEventNewShamirRecovery):
                outcome = await self.shamir.setup(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.brief_certificate.author,
                    author_verify_key=_get_device_verify_key(event.author),
                    ciphered_data=event.ciphered_data,
                    reveal_token=event.reveal_token,
                    shamir_recovery_brief_certificate=event.raw_brief_certificate,
                    shamir_recovery_share_certificates=event.raw_shares_certificates,
                )
                assert isinstance(outcome, ShamirRecoveryBriefCertificate)
            elif isinstance(event, testbed.TestbedEventDeleteShamirRecovery):
                outcome = await self.shamir.delete(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.certificate.author,
                    author_verify_key=_get_device_verify_key(event.author),
                    shamir_recovery_deletion_certificate=event.raw_certificate,
                )
                assert isinstance(outcome, ShamirRecoveryDeletionCertificate)
            elif isinstance(event, testbed.TestbedEventCreateBlock):
                outcome = await self.block.create(
                    now=event.timestamp,
                    organization_id=org_id,
                    author=event.author,
                    realm_id=event.realm,
                    block_id=event.block_id,
                    key_index=event.key_index,
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
                    key_index=event.key_index,
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
                    )
                    assert outcome is None, outcome
                else:
                    outcome = await self.vlob.update(
                        now=event.timestamp,
                        organization_id=org_id,
                        author=event.author,
                        realm_id=event.realm,
                        vlob_id=event.vlob_id,
                        key_index=event.key_index,
                        version=event.version,
                        timestamp=event.timestamp,
                        blob=event.encrypted,
                    )
                    assert outcome is None, outcome
            elif isinstance(event, testbed.TestbedEventFreezeUser):
                outcome = await self.user.freeze_user(
                    organization_id=org_id,
                    user_id=event.user,
                    user_email=None,
                    frozen=True,
                )
                assert isinstance(outcome, UserInfo), outcome
            elif isinstance(event, testbed.TestbedEventUpdateOrganization):
                outcome = await self.organization.update(
                    now=event.timestamp,
                    id=org_id,
                    is_expired=event.is_expired,
                    active_users_limit=event.active_users_limit,
                    user_profile_outsider_allowed=event.user_profile_outsider_allowed,
                    realm_minimum_archiving_period_before_deletion=event.realm_minimum_archiving_period_before_deletion,
                    tos=event.tos,
                )
                assert outcome is None, outcome

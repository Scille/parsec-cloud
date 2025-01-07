# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from base64 import b64decode
from contextlib import asynccontextmanager
from dataclasses import dataclass
from typing import AsyncContextManager, AsyncGenerator, AsyncIterator

import pytest
from httpx import ASGITransport, AsyncClient, Response
from httpx_sse import EventSource, ServerSentEvent, aconnect_sse

from parsec._parsec import (
    DateTime,
    DeviceID,
    HumanHandle,
    InvitationToken,
    OrganizationID,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryDeletionCertificate,
    ShamirRecoveryShareCertificate,
    SigningKey,
    UserID,
    VerifyKey,
    VlobID,
    authenticated_cmds,
)
from parsec._parsec import (
    testbed as tb,
)
from parsec.asgi import AsgiApp
from parsec.components.auth import AuthenticatedToken
from tests.common.backend import SERVER_DOMAIN, TestbedBackend
from tests.common.rpc import (
    BaseAnonymousRpcClient,
    BaseAuthenticatedRpcClient,
    BaseInvitedRpcClient,
    BaseTosRpcClient,
)


@pytest.fixture
async def client(app: AsgiApp) -> AsyncGenerator[AsyncClient, None]:
    async with AsyncClient(transport=ASGITransport(app=app)) as client:
        yield client


class AnonymousRpcClient(BaseAnonymousRpcClient):
    def __init__(self, raw_client: AsyncClient, organization_id: OrganizationID):
        self.raw_client = raw_client
        self.organization_id = organization_id
        self.url = f"http://{SERVER_DOMAIN}/anonymous/{organization_id}"
        self.headers = {
            "Content-Type": "application/msgpack",
            "Api-Version": "4.0",
        }

    async def _do_request(self, req: bytes, family: str) -> bytes:
        rep = await self.raw_client.post(self.url, headers=self.headers, content=req)
        if rep.status_code != 200:
            raise RpcTransportError(rep)
        return rep.content


@dataclass(slots=True)
class RpcTransportError(Exception):
    rep: Response


class EventsListenSSE:
    def __init__(self, event_source: EventSource):
        self._event_source = event_source
        self._iter_events = self._event_source.aiter_sse()

    async def next_raw_event(self) -> ServerSentEvent:
        return await self._iter_events.__anext__()

    async def next_event(self) -> authenticated_cmds.latest.events_listen.Rep:
        while True:
            sse = await self._iter_events.__anext__()
            assert sse.event == "message", sse
            return authenticated_cmds.latest.events_listen.Rep.load(b64decode(sse.data))


class AuthenticatedRpcClient(BaseAuthenticatedRpcClient, BaseTosRpcClient):
    def __init__(
        self,
        raw_client: AsyncClient,
        organization_id: OrganizationID,
        user_id: UserID,
        device_id: DeviceID,
        signing_key: SigningKey,
        event: tb.TestbedEventNewUser | tb.TestbedEventBootstrapOrganization | None = None,
    ):
        assert isinstance(user_id, UserID)
        self.raw_client = raw_client
        self.organization_id = organization_id
        self.device_id = device_id
        self.user_id = user_id
        self.signing_key = signing_key
        self.event = event
        self.url = f"http://{SERVER_DOMAIN}/authenticated/{organization_id}"
        self.headers = {
            "Content-Type": "application/msgpack",
            "Api-Version": "4.0",
        }
        # Useful to mock the current time
        self.now_factory = DateTime.now

    @property
    def human_handle(self) -> HumanHandle:
        assert self.event
        match self.event:
            case tb.TestbedEventNewUser() as event:
                return event.human_handle
            case tb.TestbedEventBootstrapOrganization() as event:
                return event.first_user_human_handle

    async def _do_request(self, req: bytes, family: str) -> bytes:
        if family == "tos":
            url = f"{self.url}/tos"
        else:
            url = self.url

        token = AuthenticatedToken.generate_raw(
            device_id=self.device_id,
            timestamp=self.now_factory(),
            key=self.signing_key,
        )
        headers = {
            "Authorization": f"Bearer {token.decode()}",
            **self.headers,
        }
        rep = await self.raw_client.post(url, headers=headers, content=req)
        if rep.status_code != 200:
            raise RpcTransportError(rep)
        return rep.content

    @asynccontextmanager
    async def events_listen(  # pyright: ignore [reportIncompatibleMethodOverride]
        self, last_event_id: str | None = None, now: DateTime | None = None
    ) -> AsyncIterator[EventsListenSSE]:
        now = now or DateTime.now()
        token = AuthenticatedToken.generate_raw(
            device_id=self.device_id,
            timestamp=now,
            key=self.signing_key,
        )
        headers = {
            "Authorization": f"Bearer {token.decode()}",
            "Accept": "text/event-stream",
            **self.headers,
        }
        if last_event_id is not None:
            headers["Last-Event-ID"] = last_event_id
        async with aconnect_sse(
            self.raw_client, "GET", f"{self.url}/events", headers=headers
        ) as event_source:
            yield EventsListenSSE(event_source)

    def raw_sse_connection(self, now: DateTime | None = None) -> AsyncContextManager[Response]:
        now = now or DateTime.now()
        token = AuthenticatedToken.generate_raw(
            device_id=self.device_id,
            timestamp=now,
            key=self.signing_key,
        )
        return self.raw_client.stream(
            method="GET",
            url=f"{self.url}/events",
            headers={
                "Authorization": f"Bearer {token.decode()}",
                "Accept": "text/event-stream",
                **self.headers,
            },
        )


class InvitedRpcClient(BaseInvitedRpcClient):
    def __init__(
        self,
        raw_client: AsyncClient,
        organization_id: OrganizationID,
        event: tb.TestbedEventNewUserInvitation
        | tb.TestbedEventNewDeviceInvitation
        | tb.TestbedEventNewShamirRecoveryInvitation,
    ):
        self.raw_client = raw_client
        self.organization_id = organization_id
        self.event = event
        self.url = f"http://{SERVER_DOMAIN}/invited/{organization_id}"
        self.headers = {
            "Content-Type": "application/msgpack",
            "Api-Version": "4.0",
            "Authorization": f"Bearer {self.token.hex}",
        }

    @property
    def claimer_email(self) -> str:
        assert isinstance(self.event, tb.TestbedEventNewUserInvitation)
        return self.event.claimer_email

    @property
    def token(self) -> InvitationToken:
        return self.event.token

    async def _do_request(self, req: bytes, family: str) -> bytes:
        rep = await self.raw_client.post(self.url, headers=self.headers, content=req)
        if rep.status_code != 200:
            raise RpcTransportError(rep)
        return rep.content


@dataclass(slots=True)
class CoolorgRpcClients:
    """
    Coolorg contains:
    - 3 users: `alice` (admin), `bob` (regular) and `mallory` (outsider)
    - devices `alice@dev1`, `bob@dev1` and `mallory@dev1` whose storages are up to date
    - devices `alice@dev2` and `bob@dev2` whose storages are empty
    - 1 workspace `wksp1` shared between alice (owner) and bob (reader)
    - `wksp1` is bootstrapped (i.e. has it initial key rotation and name certificates).
    - Alice & Bob have their user realm created and user manifest v1 synced with `wksp1` in it
    - Mallory has no user realm created
    - 1 pending invitation from Alice for a new user with email `zack@example.invalid`
    - 1 pending invitation for a new device for Alice

    See `libparsec/crates/testbed/src/templates/coolorg.rs` for it actual definition.
    """

    raw_client: AsyncClient
    testbed_template: tb.TestbedTemplateContent
    organization_id: OrganizationID
    _anonymous: AnonymousRpcClient | None = None
    _alice: AuthenticatedRpcClient | None = None
    _bob: AuthenticatedRpcClient | None = None
    _mallory: AuthenticatedRpcClient | None = None
    _invited_zack: InvitedRpcClient | None = None
    _invited_alice_dev3: InvitedRpcClient | None = None

    @property
    def anonymous(self) -> AnonymousRpcClient:
        self._anonymous = self._anonymous or AnonymousRpcClient(
            self.raw_client, self.organization_id
        )
        return self._anonymous

    @property
    def wksp1_id(self) -> VlobID:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventNewRealm):
                return event.realm_id
        assert False

    @property
    def alice(self) -> AuthenticatedRpcClient:
        if self._alice:
            return self._alice
        self._alice = self._init_for("alice")
        return self._alice

    @property
    def bob(self) -> AuthenticatedRpcClient:
        if self._bob:
            return self._bob
        self._bob = self._init_for("bob")
        return self._bob

    @property
    def mallory(self) -> AuthenticatedRpcClient:
        if self._mallory:
            return self._mallory
        self._mallory = self._init_for("mallory")
        return self._mallory

    @property
    def root_signing_key(self) -> SigningKey:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventBootstrapOrganization):
                return event.root_signing_key
        raise RuntimeError("Organization bootstrap event not found !")

    @property
    def root_verify_key(self) -> VerifyKey:
        return self.root_signing_key.verify_key

    def key_bundle(self, realm_id: VlobID, key_index: int) -> bytes:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventRotateKeyRealm):
                if event.realm == realm_id and event.key_index == key_index:
                    return event.keys_bundle
        raise RuntimeError(
            f"Key bundle for realm `{realm_id}` and key index `{key_index}` not found !"
        )

    def key_bundle_access(self, realm_id: VlobID, key_index: int, user_id: UserID) -> bytes:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventShareRealm):
                if (
                    event.realm == realm_id
                    and event.key_index == key_index
                    and event.user == user_id
                    and event.recipient_keys_bundle_access
                ):
                    return event.recipient_keys_bundle_access
            elif isinstance(event, tb.TestbedEventRotateKeyRealm):
                if event.realm == realm_id and event.key_index == key_index:
                    try:
                        return event.per_participant_keys_bundle_access[user_id]
                    except KeyError:
                        pass
        raise RuntimeError(
            f"Key bundle for realm `{realm_id}` and key index `{key_index}` not found !"
        )

    def _init_for(self, user: str) -> AuthenticatedRpcClient:
        user_id = UserID.test_from_nickname(user)
        for event in self.testbed_template.events:
            if (
                isinstance(event, tb.TestbedEventBootstrapOrganization)
                and event.first_user_id == user_id
            ):
                return AuthenticatedRpcClient(
                    self.raw_client,
                    self.organization_id,
                    user_id=event.first_user_id,
                    device_id=event.first_user_first_device_id,
                    signing_key=event.first_user_first_device_signing_key,
                    event=event,
                )
            elif isinstance(event, tb.TestbedEventNewUser) and event.user_id == user_id:
                return AuthenticatedRpcClient(
                    self.raw_client,
                    self.organization_id,
                    user_id=event.user_id,
                    device_id=event.first_device_id,
                    signing_key=event.first_device_signing_key,
                    event=event,
                )
        else:
            raise RuntimeError(f"`{user}` user creation event not found !")

    @property
    def invited_zack(self) -> InvitedRpcClient:
        if self._invited_zack:
            return self._invited_zack

        for event in self.testbed_template.events:
            if (
                isinstance(event, tb.TestbedEventNewUserInvitation)
                and event.claimer_email == "zack@example.invalid"
            ):
                self._invited_zack = InvitedRpcClient(
                    self.raw_client, self.organization_id, event=event
                )
                return self._invited_zack
        else:
            raise RuntimeError("Zack user invitation event not found !")

    @property
    def invited_alice_dev3(self) -> InvitedRpcClient:
        if self._invited_alice_dev3:
            return self._invited_alice_dev3

        alice_dev1_device_id = DeviceID.test_from_nickname("alice@dev1")
        for event in self.testbed_template.events:
            if (
                isinstance(event, tb.TestbedEventNewDeviceInvitation)
                and event.created_by == alice_dev1_device_id
            ):
                self._invited_alice_dev3 = InvitedRpcClient(
                    self.raw_client, self.organization_id, event=event
                )
                return self._invited_alice_dev3
        else:
            raise RuntimeError("Zack user invitation event not found !")


@pytest.fixture
async def coolorg(app: AsgiApp, testbed: TestbedBackend) -> AsyncGenerator[CoolorgRpcClients, None]:
    async with AsyncClient(transport=ASGITransport(app=app)) as raw_client:
        organization_id, _, template_content = await testbed.new_organization("coolorg")
        yield CoolorgRpcClients(
            raw_client=raw_client,
            organization_id=organization_id,
            testbed_template=template_content,
        )

        await testbed.drop_organization(organization_id)


@dataclass(slots=True)
class MinimalorgRpcClients:
    """
    Minimal organization:
    - Only a single alice user & device
    - No data in local storage

    See `libparsec/crates/testbed/src/templates/minimal.rs` for it actual definition.
    """

    raw_client: AsyncClient
    testbed_template: tb.TestbedTemplateContent
    organization_id: OrganizationID
    _anonymous: AnonymousRpcClient | None = None
    _alice: AuthenticatedRpcClient | None = None

    @property
    def anonymous(self) -> AnonymousRpcClient:
        self._anonymous = self._anonymous or AnonymousRpcClient(
            self.raw_client, self.organization_id
        )
        return self._anonymous

    @property
    def alice(self) -> AuthenticatedRpcClient:
        if self._alice:
            return self._alice
        self._alice = self._init_for("alice")
        return self._alice

    def _init_for(self, user: str) -> AuthenticatedRpcClient:
        user_id = UserID.test_from_nickname(user)
        for event in self.testbed_template.events:
            if (
                isinstance(event, tb.TestbedEventBootstrapOrganization)
                and event.first_user_id == user_id
            ):
                return AuthenticatedRpcClient(
                    self.raw_client,
                    self.organization_id,
                    user_id=event.first_user_id,
                    device_id=event.first_user_first_device_id,
                    signing_key=event.first_user_first_device_signing_key,
                    event=event,
                )
            elif isinstance(event, tb.TestbedEventNewUser) and event.user_id == user_id:
                return AuthenticatedRpcClient(
                    self.raw_client,
                    self.organization_id,
                    user_id=event.user_id,
                    device_id=event.first_device_id,
                    signing_key=event.first_device_signing_key,
                    event=event,
                )
        else:
            raise RuntimeError(f"`{user}` user creation event not found !")


@pytest.fixture
async def minimalorg(
    app: AsgiApp, testbed: TestbedBackend
) -> AsyncGenerator[MinimalorgRpcClients, None]:
    async with AsyncClient(transport=ASGITransport(app=app)) as raw_client:
        organization_id, _, template_content = await testbed.new_organization("minimal")
        yield MinimalorgRpcClients(
            raw_client=raw_client,
            organization_id=organization_id,
            testbed_template=template_content,
        )

        await testbed.drop_organization(organization_id)


@dataclass(slots=True)
class ShamirOrgRpcClients:
    """
    Shamir org contains:
    - 4 users: `alice` (admin), `bob` (standard), `mallory` (outsider) and `mike` (standard)
    - Alice has two devices: `alice@dev1` and `alice@dev2`
    - Bob has two devices: `bob@dev1` and `bob@dev2`
    - Mallory has two devices: `mallory@dev1` and `mallory@dev2`
    - Mike only has one device: `mike@dev1`
    - Alice has a shamir recovery setup (threshold: 2, recipients: Bob with 2 shares,
      Mallory & Mike with 1 share each)
    - Bob has a deleted shamir recovery (used to be threshold: 1, recipients: Alice & Mallory)
    - Mallory has a shamir recovery setup (threshold: 1, recipients: only Mike)
    - Bob has invited Alice to do a Shamir recovery
    - devices `alice@dev1`/`bob@dev1`/`mallory@dev1`/`mike@dev1` starts with up-to-date storages
    - devices `alice@dev2` and `bob@dev2` whose storages are empty

    See `libparsec/crates/testbed/src/templates/shamir.rs` for it actual definition.
    """

    raw_client: AsyncClient
    testbed_template: tb.TestbedTemplateContent
    organization_id: OrganizationID
    _anonymous: AnonymousRpcClient | None = None
    _alice: AuthenticatedRpcClient | None = None
    _bob: AuthenticatedRpcClient | None = None
    _mallory: AuthenticatedRpcClient | None = None
    _mike: AuthenticatedRpcClient | None = None
    _shamir_invited_alice: InvitedRpcClient | None = None

    @property
    def anonymous(self) -> AnonymousRpcClient:
        self._anonymous = self._anonymous or AnonymousRpcClient(
            self.raw_client, self.organization_id
        )
        return self._anonymous

    @property
    def alice(self) -> AuthenticatedRpcClient:
        if self._alice:
            return self._alice
        self._alice = self._init_for("alice")
        return self._alice

    @property
    def bob(self) -> AuthenticatedRpcClient:
        if self._bob:
            return self._bob
        self._bob = self._init_for("bob")
        return self._bob

    @property
    def mallory(self) -> AuthenticatedRpcClient:
        if self._mallory:
            return self._mallory
        self._mallory = self._init_for("mallory")
        return self._mallory

    @property
    def mike(self) -> AuthenticatedRpcClient:
        if self._mike:
            return self._mike
        self._mike = self._init_for("mike")
        return self._mike

    @property
    def shamir_invited_alice(self) -> InvitedRpcClient:
        if self._shamir_invited_alice:
            return self._shamir_invited_alice

        for event in self.testbed_template.events:
            if (
                isinstance(event, tb.TestbedEventNewShamirRecoveryInvitation)
                and event.claimer == self.alice.user_id
            ):
                self._shamir_invited_alice = InvitedRpcClient(
                    self.raw_client, self.organization_id, event=event
                )
                return self._shamir_invited_alice
        else:
            raise RuntimeError("Zack user invitation event not found !")

    @property
    def root_signing_key(self) -> SigningKey:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventBootstrapOrganization):
                return event.root_signing_key
        raise RuntimeError("Organization bootstrap event not found !")

    @property
    def alice_brief_certificate(self) -> ShamirRecoveryBriefCertificate:
        return self._brief_certificate_for("alice")

    @property
    def alice_shares_certificates(self) -> list[ShamirRecoveryShareCertificate]:
        return self._shares_certificates_for("alice")

    @property
    def bob_brief_certificate(self) -> ShamirRecoveryBriefCertificate:
        return self._brief_certificate_for("bob")

    @property
    def bob_shares_certificates(self) -> list[ShamirRecoveryShareCertificate]:
        return self._shares_certificates_for("bob")

    @property
    def bob_remove_certificate(self) -> ShamirRecoveryDeletionCertificate:
        user_id = UserID.test_from_nickname("bob")
        for event in self.testbed_template.events:
            if (
                isinstance(event, tb.TestbedEventDeleteShamirRecovery)
                and event.setup_to_delete_user_id == user_id
            ):
                return event.certificate
        raise RuntimeError("Remove shamir recovery event not found for user `bob` !")

    @property
    def mallory_brief_certificate(self) -> ShamirRecoveryBriefCertificate:
        return self._brief_certificate_for("mallory")

    @property
    def mallory_shares_certificates(self) -> list[ShamirRecoveryShareCertificate]:
        return self._shares_certificates_for("mallory")

    @property
    def alice_shamir_topic_timestamp(self) -> DateTime:
        return self._last_shamir_topic_timestamp_for("alice")

    @property
    def bob_shamir_topic_timestamp(self) -> DateTime:
        return self._last_shamir_topic_timestamp_for("bob")

    @property
    def mallory_shamir_topic_timestamp(self) -> DateTime:
        return self._last_shamir_topic_timestamp_for("mallory")

    @property
    def mike_shamir_topic_timestamp(self) -> DateTime:
        return self._last_shamir_topic_timestamp_for("mike")

    @property
    def alice_shamir_reveal_token(self) -> InvitationToken:
        return self._shamir_reveal_token_for("alice")

    @property
    def bob_shamir_reveal_token(self) -> InvitationToken:
        return self._shamir_reveal_token_for("bob")

    @property
    def mallory_shamir_reveal_token(self) -> InvitationToken:
        return self._shamir_reveal_token_for("mallory")

    @property
    def mike_shamir_reveal_token(self) -> InvitationToken:
        return self._shamir_reveal_token_for("mike")

    @property
    def alice_shamir_ciphered_data(self) -> bytes:
        return self._shamir_ciphered_data_for("alice")

    @property
    def bob_shamir_ciphered_data(self) -> bytes:
        return self._shamir_ciphered_data_for("bob")

    @property
    def mallory_shamir_ciphered_data(self) -> bytes:
        return self._shamir_ciphered_data_for("mallory")

    @property
    def mike_shamir_ciphered_data(self) -> bytes:
        return self._shamir_ciphered_data_for("mike")

    def _last_shamir_topic_timestamp_for(self, user: str) -> DateTime:
        user_id = UserID.test_from_nickname(user)
        for event in reversed(self.testbed_template.events):
            match event:
                case (
                    tb.TestbedEventNewShamirRecovery() as e
                ) if e.user_id == user_id or user_id in e.per_recipient_shares:
                    return e.timestamp
                case (
                    tb.TestbedEventDeleteShamirRecovery() as e
                ) if e.setup_to_delete_user_id == user_id or user_id in e.share_recipients:
                    return e.timestamp
                case _:
                    pass
        raise RuntimeError(f"No shamir topic is empty for user `{user}` !")

    def _brief_certificate_for(self, user: str) -> ShamirRecoveryBriefCertificate:
        user_id = UserID.test_from_nickname(user)
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventNewShamirRecovery) and event.user_id == user_id:
                return event.brief_certificate
        raise RuntimeError(f"New shamir recovery event not found for user `{user}` !")

    def _shares_certificates_for(self, user: str) -> list[ShamirRecoveryShareCertificate]:
        user_id = UserID.test_from_nickname(user)
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventNewShamirRecovery) and event.user_id == user_id:
                return event.shares_certificates
        raise RuntimeError(f"New shamir recovery event not found for user `{user}` !")

    def _shamir_reveal_token_for(self, user: str) -> InvitationToken:
        user_id = UserID.test_from_nickname(user)
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventNewShamirRecovery) and event.user_id == user_id:
                return event.reveal_token
        raise RuntimeError(f"New shamir recovery event not found for user `{user}` !")

    def _shamir_ciphered_data_for(self, user: str) -> bytes:
        user_id = UserID.test_from_nickname(user)
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventNewShamirRecovery) and event.user_id == user_id:
                return event.ciphered_data
        raise RuntimeError(f"New shamir recovery event not found for user `{user}` !")

    @property
    def root_verify_key(self) -> VerifyKey:
        return self.root_signing_key.verify_key

    def _init_for(self, user: str) -> AuthenticatedRpcClient:
        user_id = UserID.test_from_nickname(user)
        for event in self.testbed_template.events:
            if (
                isinstance(event, tb.TestbedEventBootstrapOrganization)
                and event.first_user_id == user_id
            ):
                return AuthenticatedRpcClient(
                    self.raw_client,
                    self.organization_id,
                    user_id=event.first_user_id,
                    device_id=event.first_user_first_device_id,
                    signing_key=event.first_user_first_device_signing_key,
                    event=event,
                )
            elif isinstance(event, tb.TestbedEventNewUser) and event.user_id == user_id:
                return AuthenticatedRpcClient(
                    self.raw_client,
                    self.organization_id,
                    user_id=event.user_id,
                    device_id=event.first_device_id,
                    signing_key=event.first_device_signing_key,
                    event=event,
                )
        else:
            raise RuntimeError(f"`{user}` user creation event not found !")


@pytest.fixture
async def shamirorg(
    app: AsgiApp, testbed: TestbedBackend, skip_if_postgresql: None
) -> AsyncGenerator[ShamirOrgRpcClients, None]:
    async with AsyncClient(transport=ASGITransport(app=app)) as raw_client:
        organization_id, _, template_content = await testbed.new_organization("shamir")
        yield ShamirOrgRpcClients(
            raw_client=raw_client,
            organization_id=organization_id,
            testbed_template=template_content,
        )

        await testbed.drop_organization(organization_id)


@dataclass(slots=True)
class WorkspaceHistoryOrgRpcClients:
    """
    WorkspaceHistory contains:
    - 3 users: `alice` (admin), `bob` (regular) and `mallory` (regular)
    - 3 devices: `alice@dev1`, `bob@dev1` and `mallory@dev1` (every device has its local
      storage up-to-date regarding the certificates)
    - On 2001-01-01, Alice creates and bootstraps workspace `wksp1`
    - On 2001-01-02, Alice uploads workspace manifest in v1 (empty)
    - On 2001-01-03T01:00:00, Alice uploads file manifest `bar.txt` in v1 (containing "Hello v1")
    - On 2001-01-03T02:00:00, Alice uploads folder manifest `foo` in v1 (empty)
    - On 2001-01-03T03:00:00, Alice uploads workspace manifest in v2 (containing `foo` and `bar.txt`)
    - On 2001-01-04, Alice gives Bob access to the workspace (as OWNER)
    - On 2001-01-05, Alice gives Mallory access to the workspace (as CONTRIBUTOR)
    - On 2001-01-06, Bob removes access to the workspace from Alice
    - On 2001-01-07, Mallory uploads file manifest `bar.txt` in v2 (containing "Hello v2 world", stored on 2 blocks)
    - On 2001-01-08T01:00:00, Mallory uploads folder manifest `foo` in v2 (containing `spam` and `egg.txt`)
    - On 2001-01-08T02:00:00, Mallory uploads file manifest `foo/egg.txt` in v1 (empty file)
    - On 2001-01-08T03:00:00, Mallory uploads folder manifest `foo/spam` in v1 (empty)
    - On 2001-01-09, Bob uploads workspace manifest in v3 with renames `foo`->`foo2` and `bar.txt`->`bar2.txt`
    - On 2001-01-10, Bob uploads file manifest `bar.txt` (now named `bar2.txt`) in v3 with content "Hello v3"
    - On 2001-01-11, Bob uploads file manifest `foo/egg.txt` in v2 with content "v2"
    - On 2001-01-12, Bob uploads folder manifest `foo` (now named `foo2`) in v3 with rename `egg.txt`->`egg2.txt` and removed `spam`
    - On 2001-01-30, Bob gives back access to the workspace to Alice (as READER)

    In the end we have the following per-entry history:
      - `/`
        - 2001-01-02: v1 from Alice, content: []
        - 2001-01-03T03:00:00: v2 from Alice, content: ["foo", "bar.txt"]
        - 2001-01-09: v3 from Bob, content: ["foo2", "bar2.txt"]
    - `bar.txt`:
      - 2001-01-03T01:00:00: v1 from Alice, content: "Hello v1"
      - 2001-01-07: v2 from Alice, content: "Hello v2 world"
      - 2001-01-10: v3 from Bob, content "Hello v3"
    - `foo`:
      - 2001-01-03T02:00:00: v1 from Alice, content: []
      - 2001-01-08T01:00:00: v2 from Mallory, content ["spam", "egg.txt"],
        with children not existing themselves yet !
      - 2001-01-12: v3 from Bob, content ["egg2.txt"]
    - `foo/egg.txt`:
      - 2001-01-08T02:00:00: v1 from Mallory, content ""
      - 2001-01-11: v2 from Bob, content "v2"
    - `foo/spam`:
      - 2001-01-08T03:00:00: v1 from Mallory, content: []

    See `libparsec/crates/testbed/src/templates/workspace_history.rs` for it actual definition.
    """

    raw_client: AsyncClient
    testbed_template: tb.TestbedTemplateContent
    organization_id: OrganizationID
    _anonymous: AnonymousRpcClient | None = None
    _alice: AuthenticatedRpcClient | None = None
    _bob: AuthenticatedRpcClient | None = None

    @property
    def anonymous(self) -> AnonymousRpcClient:
        self._anonymous = self._anonymous or AnonymousRpcClient(
            self.raw_client, self.organization_id
        )
        return self._anonymous

    @property
    def wksp1_id(self) -> VlobID:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventNewRealm):
                return event.realm_id
        assert False

    @property
    def alice(self) -> AuthenticatedRpcClient:
        if self._alice:
            return self._alice
        self._alice = self._init_for("alice")
        return self._alice

    @property
    def bob(self) -> AuthenticatedRpcClient:
        if self._bob:
            return self._bob
        self._bob = self._init_for("bob")
        return self._bob

    @property
    def root_signing_key(self) -> SigningKey:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventBootstrapOrganization):
                return event.root_signing_key
        raise RuntimeError("Organization bootstrap event not found !")

    @property
    def root_verify_key(self) -> VerifyKey:
        return self.root_signing_key.verify_key

    def key_bundle(self, realm_id: VlobID, key_index: int) -> bytes:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventRotateKeyRealm):
                if event.realm == realm_id and event.key_index == key_index:
                    return event.keys_bundle
        raise RuntimeError(
            f"Key bundle for realm `{realm_id}` and key index `{key_index}` not found !"
        )

    def key_bundle_access(self, realm_id: VlobID, key_index: int, user_id: UserID) -> bytes:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventShareRealm):
                if (
                    event.realm == realm_id
                    and event.key_index == key_index
                    and event.user == user_id
                    and event.recipient_keys_bundle_access
                ):
                    return event.recipient_keys_bundle_access
            elif isinstance(event, tb.TestbedEventRotateKeyRealm):
                if event.realm == realm_id and event.key_index == key_index:
                    try:
                        return event.per_participant_keys_bundle_access[user_id]
                    except KeyError:
                        pass
        raise RuntimeError(
            f"Key bundle for realm `{realm_id}` and key index `{key_index}` not found !"
        )

    def _init_for(self, user: str) -> AuthenticatedRpcClient:
        user_id = UserID.test_from_nickname(user)
        for event in self.testbed_template.events:
            if (
                isinstance(event, tb.TestbedEventBootstrapOrganization)
                and event.first_user_id == user_id
            ):
                return AuthenticatedRpcClient(
                    self.raw_client,
                    self.organization_id,
                    user_id=event.first_user_id,
                    device_id=event.first_user_first_device_id,
                    signing_key=event.first_user_first_device_signing_key,
                    event=event,
                )
            elif isinstance(event, tb.TestbedEventNewUser) and event.user_id == user_id:
                return AuthenticatedRpcClient(
                    self.raw_client,
                    self.organization_id,
                    user_id=event.user_id,
                    device_id=event.first_device_id,
                    signing_key=event.first_device_signing_key,
                    event=event,
                )
        else:
            raise RuntimeError(f"`{user}` user creation event not found !")


@pytest.fixture
async def workspace_history_org(
    app: AsgiApp, testbed: TestbedBackend
) -> AsyncGenerator[WorkspaceHistoryOrgRpcClients, None]:
    async with AsyncClient(transport=ASGITransport(app=app)) as raw_client:
        organization_id, _, template_content = await testbed.new_organization("workspace_history")
        yield WorkspaceHistoryOrgRpcClients(
            raw_client=raw_client,
            organization_id=organization_id,
            testbed_template=template_content,
        )

        await testbed.drop_organization(organization_id)


def get_last_realm_certificate_timestamp(
    testbed_template: tb.TestbedTemplateContent, realm_id: VlobID
) -> DateTime:
    for event in reversed(testbed_template.events):
        # Be exhaustive in the cases to detect whenever a new type of certificate is introduced
        match event:
            case tb.TestbedEventNewRealm() as event:
                if event.realm_id == realm_id:
                    return event.timestamp
            case tb.TestbedEventShareRealm() as event:
                if event.realm == realm_id:
                    return event.timestamp
            case tb.TestbedEventRenameRealm() as event:
                if event.realm == realm_id:
                    return event.timestamp
            case tb.TestbedEventRotateKeyRealm() as event:
                if event.realm == realm_id:
                    return event.timestamp
            case tb.TestbedEventArchiveRealm() as event:
                if event.realm == realm_id:
                    return event.timestamp
            case (
                tb.TestbedEventBootstrapOrganization()
                | tb.TestbedEventNewSequesterService()
                | tb.TestbedEventRevokeSequesterService()
                | tb.TestbedEventNewUser()
                | tb.TestbedEventNewDevice()
                | tb.TestbedEventUpdateUserProfile()
                | tb.TestbedEventRevokeUser()
                | tb.TestbedEventNewUserInvitation()
                | tb.TestbedEventNewDeviceInvitation()
                | tb.TestbedEventNewShamirRecoveryInvitation()
                | tb.TestbedEventNewShamirRecovery()
                | tb.TestbedEventDeleteShamirRecovery()
                | tb.TestbedEventCreateOrUpdateOpaqueVlob()
                | tb.TestbedEventCreateBlock()
                | tb.TestbedEventCreateOpaqueBlock()
                | tb.TestbedEventFreezeUser()
                | tb.TestbedEventUpdateOrganization()
            ):
                pass

    raise RuntimeError(f"Realm `{realm_id}` not found !")

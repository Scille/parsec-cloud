# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from base64 import b64decode
from contextlib import asynccontextmanager
from dataclasses import dataclass
from typing import AsyncContextManager, AsyncGenerator, AsyncIterator

import pytest
from httpx import AsyncClient, Response
from httpx_sse import EventSource, ServerSentEvent, aconnect_sse

from parsec._parsec import (
    DateTime,
    DeviceID,
    HumanHandle,
    InvitationToken,
    OrganizationID,
    ShamirRecoveryBriefCertificate,
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
    async with AsyncClient(app=app) as client:
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
        event: tb.TestbedEventNewUserInvitation | tb.TestbedEventNewDeviceInvitation,
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
    async with AsyncClient(app=app) as raw_client:
        organization_id, _, template_content = await testbed.new_organization("coolorg")
        yield CoolorgRpcClients(
            raw_client=raw_client,
            organization_id=organization_id,
            testbed_template=template_content,
        )

        await testbed.drop_organization(organization_id)


async def setup_shamir_for_coolorg(
    coolorg: CoolorgRpcClients,
    now: DateTime | None = None,
) -> tuple[bytes, bytes]:
    """
    setup a shamir for alice, with mallory as a share recipient
    returns the associated brief and share as bytes
    """
    now = now or DateTime.now()
    share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=now,
        recipient=coolorg.mallory.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=now,
        threshold=1,
        per_recipient_shares={coolorg.mallory.user_id: 2},
    )

    raw_brief = brief.dump_and_sign(coolorg.alice.signing_key)
    raw_share = share.dump_and_sign(coolorg.alice.signing_key)

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        raw_brief,
        [raw_share],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepOk()
    return (raw_brief, raw_share)


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
    async with AsyncClient(app=app) as raw_client:
        organization_id, _, template_content = await testbed.new_organization("minimal")
        yield MinimalorgRpcClients(
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
                | tb.TestbedEventNewShamirRecovery()
                | tb.TestbedEventCreateOrUpdateOpaqueVlob()
                | tb.TestbedEventCreateOpaqueBlock()
                | tb.TestbedEventFreezeUser()
                | tb.TestbedEventUpdateOrganization()
            ):
                pass

    raise RuntimeError(f"Realm `{realm_id}` not found !")

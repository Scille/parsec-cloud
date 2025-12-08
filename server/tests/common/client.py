# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from base64 import b64decode
from collections.abc import AsyncGenerator, AsyncIterator
from contextlib import AbstractAsyncContextManager, asynccontextmanager
from dataclasses import dataclass

import pytest
from httpx import ASGITransport, AsyncClient, Response
from httpx_sse import EventSource, ServerSentEvent, aconnect_sse

from parsec._parsec import (
    AccountAuthMethodID,
    ApiVersion,
    DateTime,
    DeviceID,
    EmailAddress,
    HumanHandle,
    InvitationToken,
    OrganizationID,
    SecretKey,
    SequesterServiceID,
    SequesterSigningKeyDer,
    SequesterVerifyKeyDer,
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
from parsec.client_context import AuthenticatedAccountClientContext
from parsec.components.auth import (
    AccountAuthenticationToken,
    AuthenticatedToken,
)
from tests.common.backend import SERVER_DOMAIN, TestbedBackend
from tests.common.rpc import (
    BaseAnonymousRpcClient,
    BaseAnonymousServerRpcClient,
    BaseAuthenticatedAccountRpcClient,
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
            "Api-Version": str(ApiVersion.API_LATEST_VERSION),
        }

    async def _do_request(self, req: bytes, family: str) -> bytes:
        rep = await self.raw_client.post(self.url, headers=self.headers, content=req)
        if rep.status_code != 200:
            raise RpcTransportError(rep)
        return rep.content


class AnonymousServerRpcClient(BaseAnonymousServerRpcClient):
    def __init__(self, raw_client: AsyncClient):
        self.raw_client = raw_client
        self.url = f"http://{SERVER_DOMAIN}/anonymous_server"
        self.headers = {
            "Content-Type": "application/msgpack",
            "Api-Version": str(ApiVersion.API_LATEST_VERSION),
        }

    async def _do_request(self, req: bytes, family: str) -> bytes:
        rep = await self.raw_client.post(self.url, headers=self.headers, content=req)
        if rep.status_code != 200:
            raise RpcTransportError(rep)
        return rep.content


class AuthenticatedAccountRpcClient(BaseAuthenticatedAccountRpcClient):
    def __init__(
        self,
        raw_client: AsyncClient,
        account_email: EmailAddress,
        auth_method_id: AccountAuthMethodID,
        auth_method_mac_key: SecretKey,
    ):
        self.raw_client = raw_client
        self.url = f"http://{SERVER_DOMAIN}/authenticated_account"
        self.headers = {
            "Content-Type": "application/msgpack",
            "Api-Version": str(ApiVersion.API_LATEST_VERSION),
        }
        self.account_email = account_email
        self.auth_method_id = auth_method_id
        self.auth_method_mac_key = auth_method_mac_key
        self.now_factory = DateTime.now

    async def _do_request(self, req: bytes, family: str) -> bytes:
        token = AccountAuthenticationToken.generate_raw(
            timestamp=self.now_factory(),
            auth_method_id=self.auth_method_id,
            auth_method_mac_key=self.auth_method_mac_key,
        )
        headers = {"Authorization": f"Bearer {token.decode()}", **self.headers}
        rep = await self.raw_client.post(self.url, headers=headers, content=req)
        if rep.status_code != 200:
            raise RpcTransportError(rep)
        return rep.content

    def to_context(self) -> AuthenticatedAccountClientContext:
        return AuthenticatedAccountClientContext(
            client_api_version=ApiVersion.API_LATEST_VERSION,
            settled_api_version=ApiVersion.API_LATEST_VERSION,
            account_email=self.account_email,
            auth_method_id=self.auth_method_id,
            client_user_agent="Test",
            client_ip_address="",
        )


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
            "Api-Version": str(ApiVersion.API_LATEST_VERSION),
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

    def raw_sse_connection(
        self, now: DateTime | None = None
    ) -> AbstractAsyncContextManager[Response]:
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
            "Api-Version": str(ApiVersion.API_LATEST_VERSION),
            "Authorization": f"Bearer {self.token.hex}",
        }

    @property
    def claimer_email(self) -> EmailAddress:
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
    See `libparsec/crates/testbed/src/templates/coolorg.rs`.
    """

    raw_client: AsyncClient
    testbed_template: tb.TestbedTemplateContent
    organization_id: OrganizationID
    _anonymous: AnonymousRpcClient | None = None
    _anonymous_server: AnonymousServerRpcClient | None = None
    _authenticated_account: AuthenticatedAccountRpcClient | None = None
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
    def anonymous_server(self) -> AnonymousServerRpcClient:
        self._anonymous_server = self._anonymous_server or AnonymousServerRpcClient(
            self.raw_client,
        )
        return self._anonymous_server

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
            if isinstance(
                event, tb.TestbedEventNewUserInvitation
            ) and event.claimer_email == EmailAddress("zack@example.invalid"):
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
    _authenticated_account: AuthenticatedAccountRpcClient | None = None
    _alice: AuthenticatedRpcClient | None = None

    @property
    def root_signing_key(self) -> SigningKey:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventBootstrapOrganization):
                return event.root_signing_key
        raise RuntimeError("Organization bootstrap event not found !")

    @property
    def root_verify_key(self) -> VerifyKey:
        return self.root_signing_key.verify_key

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


@dataclass(slots=True)
class ShamirOrgRpcClients:
    """
    See `libparsec/crates/testbed/src/templates/shamir.rs`.
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
    def shamir_topic_timestamp(self) -> DateTime:
        return self._last_shamir_topic_timestamp()

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

    def _last_shamir_topic_timestamp(self) -> DateTime:
        for event in reversed(self.testbed_template.events):
            match event:
                case tb.TestbedEventNewShamirRecovery() as e:
                    return e.timestamp
                case tb.TestbedEventDeleteShamirRecovery() as e:
                    return e.timestamp
                case _:
                    pass
        raise RuntimeError("Shamir topic is empty !")

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
    app: AsgiApp, testbed: TestbedBackend
) -> AsyncGenerator[ShamirOrgRpcClients, None]:
    async with AsyncClient(transport=ASGITransport(app=app)) as raw_client:
        organization_id, _, template_content = await testbed.new_organization("shamir")
        yield ShamirOrgRpcClients(
            raw_client=raw_client,
            organization_id=organization_id,
            testbed_template=template_content,
        )


@dataclass(slots=True)
class SequesteredOrgRpcClients:
    """
    See `libparsec/crates/testbed/src/templates/sequestered.rs`.
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

    @property
    def sequester_authority_signing_key(self) -> SequesterSigningKeyDer:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventBootstrapOrganization):
                assert event.sequester_authority_signing_key is not None
                return event.sequester_authority_signing_key
        raise RuntimeError("Organization bootstrap event not found !")

    @property
    def sequester_authority_verify_key(self) -> SequesterVerifyKeyDer:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventBootstrapOrganization):
                assert event.sequester_authority_verify_key is not None
                return event.sequester_authority_verify_key
        raise RuntimeError("Organization bootstrap event not found !")

    @property
    def sequester_service_1_id(self) -> SequesterServiceID:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventNewSequesterService):
                if event.label == "Sequester Service 1":
                    return event.id
        raise RuntimeError("Sequester service 1 creation event not found !")

    @property
    def sequester_service_2_id(self) -> SequesterServiceID:
        for event in self.testbed_template.events:
            if isinstance(event, tb.TestbedEventNewSequesterService):
                if event.label == "Sequester Service 2":
                    return event.id
        raise RuntimeError("Sequester service 2 creation event not found !")

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
async def sequestered_org(
    app: AsgiApp, testbed: TestbedBackend
) -> AsyncGenerator[SequesteredOrgRpcClients, None]:
    async with AsyncClient(transport=ASGITransport(app=app)) as raw_client:
        organization_id, _, template_content = await testbed.new_organization("sequestered")
        yield SequesteredOrgRpcClients(
            raw_client=raw_client,
            organization_id=organization_id,
            testbed_template=template_content,
        )


@dataclass(slots=True)
class WorkspaceHistoryOrgRpcClients:
    """
    See `libparsec/crates/testbed/src/templates/workspace_history.rs`.
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

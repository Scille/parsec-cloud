# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from typing import Optional
from pendulum import datetime
from contextlib import asynccontextmanager
from structlog import get_logger

from parsec.api.data import RealmRoleCertificateContent
from parsec.api.protocol import (
    OrganizationID,
    VlobID,
    RealmID,
    RealmRole,
    InvitationToken,
    InvitationType,
    AuthenticatedClientHandshake,
    InvitedClientHandshake,
    APIV1_AnonymousClientHandshake,
)
from parsec.api.transport import Transport
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.asgi import app_factory
from parsec.backend.backend_events import BackendEvent
from parsec.core.types import LocalDevice

from tests.common import FreezeTestOnTransportError
from tests.backend.common import do_http_request


@pytest.fixture
def backend_asgi_app(backend):
    return app_factory(backend)


@pytest.fixture
def backend_invited_ws_factory():
    @asynccontextmanager
    async def _backend_invited_ws_factory(
        backend_asgi_app,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
    ):
        client = backend_asgi_app.test_client()
        async with client.websocket("/ws") as ws:

            ch = InvitedClientHandshake(
                organization_id=organization_id, invitation_type=invitation_type, token=token
            )
            challenge_req = await ws.receive()
            answer_req = ch.process_challenge_req(challenge_req)
            await ws.send(answer_req)
            result_req = await ws.receive()
            ch.process_result_req(result_req)

            yield ws

    return _backend_invited_ws_factory


@pytest.fixture
def backend_authenticated_ws_factory():
    # APIv2's invited handshake is not compatible with this
    # fixture because it requires purpose information (invitation_type/token)
    @asynccontextmanager
    async def _backend_authenticated_ws_factory(backend_asgi_app, auth_as: LocalDevice):
        client = backend_asgi_app.test_client()
        async with client.websocket("/ws") as ws:
            # Handshake
            ch = AuthenticatedClientHandshake(
                auth_as.organization_id,
                auth_as.device_id,
                auth_as.signing_key,
                auth_as.root_verify_key,
            )
            challenge_req = await ws.receive()
            answer_req = ch.process_challenge_req(challenge_req)
            await ws.send(answer_req)
            result_req = await ws.receive()
            ch.process_result_req(result_req)

            yield ws

    return _backend_authenticated_ws_factory


@pytest.fixture
async def alice_ws(backend_asgi_app, alice, backend_authenticated_ws_factory):
    async with backend_authenticated_ws_factory(backend_asgi_app, alice) as sock:
        yield sock


# TODO: legacy fixture, remove this when APIv1 is deprecated
@pytest.fixture
def apiv1_backend_ws_factory(coolorg):
    @asynccontextmanager
    async def _apiv1_backend_ws_factory(backend_asgi_app, auth_as):
        client = backend_asgi_app.test_client()
        async with client.websocket("/ws") as ws:
            if auth_as:
                # Handshake
                if isinstance(auth_as, OrganizationID):
                    ch = APIV1_AnonymousClientHandshake(auth_as)
                elif auth_as == "anonymous":
                    # TODO: for legacy test, refactorise this ?
                    ch = APIV1_AnonymousClientHandshake(coolorg.organization_id)
                elif auth_as == "administration":
                    assert False, "APIv1 Administration sock no longer supported"
                else:
                    assert False, "APIv1 Authenticated sock no longer supported"
                challenge_req = await ws.receive()
                answer_req = ch.process_challenge_req(challenge_req)
                await ws.send(answer_req)
                result_req = await ws.receive()
                ch.process_result_req(result_req)

            yield ws

    return _apiv1_backend_ws_factory


@pytest.fixture
async def apiv1_anonymous_ws(apiv1_backend_ws_factory, backend_asgi_app):
    async with apiv1_backend_ws_factory(backend_asgi_app, "anonymous") as sock:
        yield sock


@pytest.fixture
def backend_raw_transport_factory(server_factory):
    @asynccontextmanager
    async def _backend_sock_factory(backend, freeze_on_transport_error=True, keepalive=None):
        async with server_factory(backend.handle_client) as server:
            stream = await server.connection_factory()
            transport = await Transport.init_for_client(stream, "127.0.0.1", keepalive)
            if freeze_on_transport_error:
                transport = FreezeTestOnTransportError(transport)

            yield transport

    return _backend_sock_factory


# TODO: legacy fixture, remove this when APIv1 is deprecated
@pytest.fixture
def apiv1_backend_sock_factory(backend_raw_transport_factory, coolorg):
    @asynccontextmanager
    async def _backend_sock_factory(backend, auth_as, freeze_on_transport_error=True):
        async with backend_raw_transport_factory(
            backend, freeze_on_transport_error=freeze_on_transport_error
        ) as transport:
            if auth_as:
                # Handshake
                if isinstance(auth_as, OrganizationID):
                    ch = APIV1_AnonymousClientHandshake(auth_as)
                elif auth_as == "anonymous":
                    # TODO: for legacy test, refactorise this ?
                    ch = APIV1_AnonymousClientHandshake(coolorg.organization_id)
                elif auth_as == "administration":
                    assert False, "APIv1 Administration sock no longer supported"
                else:
                    assert False, "APIv1 Authenticated sock no longer supported"
                challenge_req = await transport.recv()
                answer_req = ch.process_challenge_req(challenge_req)
                await transport.send(answer_req)
                result_req = await transport.recv()
                ch.process_result_req(result_req)

            yield transport

    return _backend_sock_factory


@pytest.fixture
async def apiv1_anonymous_backend_sock(apiv1_backend_sock_factory, backend):
    async with apiv1_backend_sock_factory(backend, "anonymous") as sock:
        yield sock


# TODO: rename into `apiv1_administration_backend_sock`
@pytest.fixture
async def administration_backend_sock(apiv1_backend_sock_factory, backend):
    async with apiv1_backend_sock_factory(backend, "administration") as sock:
        yield sock


@pytest.fixture
async def apiv1_alice_backend_sock(apiv1_backend_sock_factory, backend, alice):
    async with apiv1_backend_sock_factory(backend, alice) as sock:
        yield sock


@pytest.fixture
async def apiv1_alice2_backend_sock(apiv1_backend_sock_factory, backend, alice2):
    async with apiv1_backend_sock_factory(backend, alice2) as sock:
        yield sock


@pytest.fixture
async def apiv1_otheralice_backend_sock(apiv1_backend_sock_factory, backend, otheralice):
    async with apiv1_backend_sock_factory(backend, otheralice) as sock:
        yield sock


@pytest.fixture
async def apiv1_adam_backend_sock(apiv1_backend_sock_factory, backend, adam):
    async with apiv1_backend_sock_factory(backend, adam) as sock:
        yield sock


@pytest.fixture
async def apiv1_bob_backend_sock(apiv1_backend_sock_factory, backend, bob):
    async with apiv1_backend_sock_factory(backend, bob) as sock:
        yield sock


@pytest.fixture
def backend_sock_factory(backend_raw_transport_factory):
    # APIv2's invited handshake is not compatible with this
    # fixture because it requires purpose information (invitation_type/token)
    @asynccontextmanager
    async def _backend_sock_factory(
        backend, auth_as: LocalDevice, freeze_on_transport_error=True, keepalive=None
    ):
        async with backend_raw_transport_factory(
            backend, freeze_on_transport_error=freeze_on_transport_error, keepalive=keepalive
        ) as transport:
            # Handshake
            ch = AuthenticatedClientHandshake(
                auth_as.organization_id,
                auth_as.device_id,
                auth_as.signing_key,
                auth_as.root_verify_key,
            )
            challenge_req = await transport.recv()
            answer_req = ch.process_challenge_req(challenge_req)
            await transport.send(answer_req)
            result_req = await transport.recv()
            ch.process_result_req(result_req)

            yield transport

    return _backend_sock_factory


@pytest.fixture
async def alice_backend_sock(backend_sock_factory, backend, alice):
    async with backend_sock_factory(backend, alice) as sock:
        yield sock


@pytest.fixture
async def alice2_backend_sock(backend_sock_factory, backend, alice2):
    async with backend_sock_factory(backend, alice2) as sock:
        yield sock


@pytest.fixture
async def otheralice_backend_sock(backend_sock_factory, backend, otheralice):
    async with backend_sock_factory(backend, otheralice) as sock:
        yield sock


@pytest.fixture
async def adam_backend_sock(backend_sock_factory, backend, adam):
    async with backend_sock_factory(backend, adam) as sock:
        yield sock


@pytest.fixture
async def bob_backend_sock(backend_sock_factory, backend, bob):
    async with backend_sock_factory(backend, bob) as sock:
        yield sock


@pytest.fixture
def backend_invited_sock_factory(backend_raw_transport_factory):
    @asynccontextmanager
    async def _backend_sock_factory(
        backend,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
        freeze_on_transport_error: bool = True,
    ):
        async with backend_raw_transport_factory(
            backend, freeze_on_transport_error=freeze_on_transport_error
        ) as transport:
            ch = InvitedClientHandshake(
                organization_id=organization_id, invitation_type=invitation_type, token=token
            )
            challenge_req = await transport.recv()
            answer_req = ch.process_challenge_req(challenge_req)
            await transport.send(answer_req)
            result_req = await transport.recv()
            ch.process_result_req(result_req)

            yield transport

    return _backend_sock_factory


class AnonymousTransport:
    """
    Mimic regular Transport to be compatible with the `tests.backend.common.CmdSock`,
    the trick is given we use regular HTTP instead of Websocket we create a new
    socket each time and do send&recv at the same time.
    """

    def __init__(self, connection_factory, organization_id: OrganizationID):
        self.connection_factory = connection_factory
        self.organization_id = organization_id
        self.last_request_response: Optional[bytes] = None
        self.logger = get_logger()

    async def send(self, msg: bytes) -> None:
        assert self.last_request_response is None
        stream = await self.connection_factory()
        async with stream:
            status, _, body = await do_http_request(
                stream=stream,
                target=f"/anonymous/{self.organization_id}",
                method="POST",
                headers={"Content-Type": "application/msgpack"},
                body=msg,
            )
            assert status[0] == 200
            self.last_request_response = body

    async def recv(self) -> bytes:
        assert self.last_request_response is not None
        rep = self.last_request_response
        self.last_request_response = None
        return rep


@pytest.fixture
def anonymous_backend_sock_factory(server_factory):
    @asynccontextmanager
    async def _backend_sock_factory(backend, organization_id: OrganizationID):
        async with server_factory(backend.handle_client) as server:
            yield AnonymousTransport(server.connection_factory, organization_id)

    return _backend_sock_factory


@pytest.fixture
async def anonymous_backend_sock(anonymous_backend_sock_factory, backend, coolorg):
    async with anonymous_backend_sock_factory(backend, coolorg.organization_id) as sock:
        yield sock


@pytest.fixture
def realm_factory(next_timestamp):
    async def _realm_factory(backend, author, realm_id=None, now=None):
        realm_id = realm_id or RealmID.new()
        now = now or next_timestamp()
        certif = RealmRoleCertificateContent.build_realm_root_certif(
            author=author.device_id, timestamp=now, realm_id=realm_id
        ).dump_and_sign(author.signing_key)
        with backend.event_bus.listen() as spy:
            await backend.realm.create(
                organization_id=author.organization_id,
                self_granted_role=RealmGrantedRole(
                    realm_id=realm_id,
                    user_id=author.user_id,
                    certificate=certif,
                    role=RealmRole.OWNER,
                    granted_by=author.device_id,
                    granted_on=now,
                ),
            )
            await spy.wait_with_timeout(BackendEvent.REALM_ROLES_UPDATED)
        return realm_id

    return _realm_factory


@pytest.fixture
async def realm(backend, alice, realm_factory):
    realm_id = RealmID.from_hex("A0000000000000000000000000000000")
    return await realm_factory(backend, alice, realm_id, datetime(2000, 1, 2))


@pytest.fixture
async def vlobs(backend, alice, realm):
    vlob_ids = (
        VlobID.from_hex("10000000000000000000000000000000"),
        VlobID.from_hex("20000000000000000000000000000000"),
    )
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=vlob_ids[0],
        timestamp=datetime(2000, 1, 2, 1),
        blob=b"r:A b:1 v:1",
    )
    await backend.vlob.update(
        organization_id=alice.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        vlob_id=vlob_ids[0],
        version=2,
        timestamp=datetime(2000, 1, 3),
        blob=b"r:A b:1 v:2",
    )
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=vlob_ids[1],
        timestamp=datetime(2000, 1, 4),
        blob=b"r:A b:2 v:1",
    )
    return vlob_ids


@pytest.fixture
async def vlob_atoms(vlobs):
    return [(vlobs[0], 1), (vlobs[0], 2), (vlobs[1], 1)]


@pytest.fixture
async def other_realm(backend, alice, realm_factory):
    realm_id = RealmID.from_hex("B0000000000000000000000000000000")
    return await realm_factory(backend, alice, realm_id, datetime(2000, 1, 2))


@pytest.fixture
async def bob_realm(backend, bob, realm_factory):
    realm_id = RealmID.from_hex("C0000000000000000000000000000000")
    return await realm_factory(backend, bob, realm_id, datetime(2000, 1, 2))

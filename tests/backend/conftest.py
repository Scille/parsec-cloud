# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from parsec._parsec import DateTime
from contextlib import asynccontextmanager

from parsec.api.data import RealmRoleCertificate
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
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.backend_events import BackendEvent
from parsec.core.types import LocalDevice


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
    async with backend_authenticated_ws_factory(backend_asgi_app, alice) as ws:
        yield ws


@pytest.fixture
async def alice2_ws(backend_asgi_app, alice2, backend_authenticated_ws_factory):
    async with backend_authenticated_ws_factory(backend_asgi_app, alice2) as ws:
        yield ws


@pytest.fixture
async def bob_ws(backend_asgi_app, bob, backend_authenticated_ws_factory):
    async with backend_authenticated_ws_factory(backend_asgi_app, bob) as ws:
        yield ws


@pytest.fixture
async def adam_ws(backend_asgi_app, adam, backend_authenticated_ws_factory):
    async with backend_authenticated_ws_factory(backend_asgi_app, adam) as ws:
        yield ws


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
    async with apiv1_backend_ws_factory(backend_asgi_app, "anonymous") as ws:
        yield ws


class AnonymousClientFakingWebsocket:
    def __init__(self, client, organization_id: OrganizationID):
        self.organization_id = organization_id
        self.client = client
        self.last_request_response = None

    async def send(self, msg: bytes):
        response = await self.client.post(
            f"/anonymous/{self.organization_id.str}",
            headers={"Content-Type": "application/msgpack"},
            data=msg,
        )
        assert response.status_code == 200
        self.last_request_response = await response.get_data()

    async def receive(self) -> bytes:
        assert self.last_request_response is not None
        rep = self.last_request_response
        self.last_request_response = None
        return rep


@pytest.fixture
def backend_anonymous_ws_factory():
    """
    Not really a ws, but we keep the name because it usage is similar than alice_ws&co
    """

    @asynccontextmanager
    async def _backend_anonymous_ws_factory(backend_asgi_app, organization_id: OrganizationID):
        client = backend_asgi_app.test_client()
        yield AnonymousClientFakingWebsocket(client, organization_id)

    return _backend_anonymous_ws_factory


@pytest.fixture
async def anonymous_backend_ws(backend_asgi_app, backend_anonymous_ws_factory, coolorg):
    async with backend_anonymous_ws_factory(backend_asgi_app, coolorg.organization_id) as ws:
        yield ws


@pytest.fixture
def realm_factory(next_timestamp):
    async def _realm_factory(backend, author, realm_id=None, now=None):
        realm_id = realm_id or RealmID.new()
        now = now or next_timestamp()
        certif = RealmRoleCertificate.build_realm_root_certif(
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
    realm_id = RealmID.from_str("A0000000000000000000000000000000")
    return await realm_factory(backend, alice, realm_id, DateTime(2000, 1, 2))


@pytest.fixture
async def vlobs(backend, alice, realm):
    vlob_ids = (
        VlobID.from_str("10000000000000000000000000000000"),
        VlobID.from_str("20000000000000000000000000000000"),
    )
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=vlob_ids[0],
        timestamp=DateTime(2000, 1, 2, 1),
        blob=b"r:A b:1 v:1",
    )
    await backend.vlob.update(
        organization_id=alice.organization_id,
        author=alice.device_id,
        encryption_revision=1,
        vlob_id=vlob_ids[0],
        version=2,
        timestamp=DateTime(2000, 1, 3),
        blob=b"r:A b:1 v:2",
    )
    await backend.vlob.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        realm_id=realm,
        encryption_revision=1,
        vlob_id=vlob_ids[1],
        timestamp=DateTime(2000, 1, 4),
        blob=b"r:A b:2 v:1",
    )
    return vlob_ids


@pytest.fixture
async def vlob_atoms(vlobs):
    return [(vlobs[0], 1), (vlobs[0], 2), (vlobs[1], 1)]


@pytest.fixture
async def other_realm(backend, alice, realm_factory):
    realm_id = RealmID.from_str("B0000000000000000000000000000000")
    return await realm_factory(backend, alice, realm_id, DateTime(2000, 1, 2))


@pytest.fixture
async def bob_realm(backend, bob, realm_factory):
    realm_id = RealmID.from_str("C0000000000000000000000000000000")
    return await realm_factory(backend, bob, realm_id, DateTime(2000, 1, 2))

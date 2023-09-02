# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager

import pytest

from parsec._parsec import DateTime, RealmArchivingConfiguration
from parsec.api.data import RealmArchivingCertificate, RealmRoleCertificate
from parsec.api.protocol import (
    AuthenticatedClientHandshake,
    InvitationToken,
    InvitationType,
    InvitedClientHandshake,
    OrganizationID,
    RealmID,
    RealmRole,
    VlobID,
)
from parsec.backend.backend_events import BackendEvent
from parsec.backend.realm import RealmArchivingConfigurationRequest, RealmGrantedRole
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
def archived_realm_factory(realm_factory, next_timestamp):
    async def _archived_realm_factory(backend, author, realm_id=None, now=None):
        now = now or next_timestamp()
        realm = await realm_factory(backend, author, realm_id=realm_id, now=now)
        after_now = next_timestamp()
        certificate = RealmArchivingCertificate(
            author=author.device_id,
            timestamp=after_now,
            realm_id=realm_id,
            configuration=RealmArchivingConfiguration.archived(),
        )
        signed = certificate.dump_and_sign(author.signing_key)
        with backend.event_bus.listen() as spy:
            await backend.realm.update_archiving(
                organization_id=author.organization_id,
                archiving_configuration_request=RealmArchivingConfigurationRequest(
                    certificate=signed,
                    realm_id=realm,
                    configuration=certificate.configuration,
                    configured_by=certificate.author,
                    configured_on=certificate.timestamp,
                ),
            )
            await spy.wait_with_timeout(BackendEvent.REALM_ARCHIVING_UPDATED)
        return realm_id

    return _archived_realm_factory


@pytest.fixture
def deleted_realm_factory(realm_factory, next_timestamp, monkeypatch):
    async def _deleted_realm_factory(backend, author, realm_id=None, now=None):
        now = now or next_timestamp()
        realm = await realm_factory(backend, author, realm_id=realm_id, now=now)
        after_now = next_timestamp()
        certificate = RealmArchivingCertificate(
            author=author.device_id,
            timestamp=after_now,
            realm_id=realm_id,
            configuration=RealmArchivingConfiguration.deletion_planned(after_now),
        )
        signed = certificate.dump_and_sign(author.signing_key)
        with monkeypatch.context() as context:
            context.setattr(
                "parsec.backend.realm.RealmArchivingConfigurationRequest.is_valid_archiving_configuration",
                lambda *args: True,
            )
            with backend.event_bus.listen() as spy:
                await backend.realm.update_archiving(
                    organization_id=author.organization_id,
                    archiving_configuration_request=RealmArchivingConfigurationRequest(
                        certificate=signed,
                        realm_id=realm,
                        configuration=certificate.configuration,
                        configured_by=certificate.author,
                        configured_on=certificate.timestamp,
                    ),
                )
                await spy.wait_with_timeout(BackendEvent.REALM_ARCHIVING_UPDATED)
        return realm_id

    return _deleted_realm_factory


@pytest.fixture
async def realm(backend, alice, realm_factory):
    realm_id = RealmID.from_hex("A0000000000000000000000000000000")
    return await realm_factory(backend, alice, realm_id, DateTime(2000, 1, 2))


@pytest.fixture
async def archived_realm(backend, alice, archived_realm_factory):
    realm_id = RealmID.from_hex("B0000000000000000000000000000000")
    realm = await archived_realm_factory(backend, alice, realm_id, DateTime(2000, 1, 2))
    return realm


@pytest.fixture(params=(False, True), ids=["available_realm", "archived_realm"])
async def maybe_archived_realm(request, realm, archived_realm):
    return archived_realm if request.param else realm


@pytest.fixture
async def deleted_realm(backend, alice, deleted_realm_factory):
    realm_id = RealmID.from_hex("C0000000000000000000000000000000")
    return await deleted_realm_factory(backend, alice, realm_id, DateTime(2000, 1, 2))


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
    realm_id = RealmID.from_hex("B0000000000000000000000000000000")
    return await realm_factory(backend, alice, realm_id, DateTime(2000, 1, 2))


@pytest.fixture
async def bob_realm(backend, bob, realm_factory):
    realm_id = RealmID.from_hex("C0000000000000000000000000000000")
    return await realm_factory(backend, bob, realm_id, DateTime(2000, 1, 2))

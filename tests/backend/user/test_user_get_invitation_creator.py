# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.api.protocole import user_get_invitation_creator_serializer
from parsec.backend.user import UserInvitation, INVITATION_VALIDITY

from tests.common import freeze_time


@pytest.fixture
async def mallory_invitation(backend, alice, mallory):
    invitation = UserInvitation(user_id=mallory.user_id, creator=alice.device_id)
    await backend.user.create_user_invitation(alice.organization_id, invitation)
    return invitation


async def user_get_invitation_creator(sock, **kwargs):
    await sock.send(
        user_get_invitation_creator_serializer.req_dumps(
            {"cmd": "user_get_invitation_creator", **kwargs}
        )
    )
    raw_rep = await sock.recv()
    return user_get_invitation_creator_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_user_get_invitation_creator_too_late(anonymous_backend_sock, mallory_invitation):
    with freeze_time(mallory_invitation.created_on.add(seconds=INVITATION_VALIDITY + 1)):
        rep = await user_get_invitation_creator(
            anonymous_backend_sock, invited_user_id=mallory_invitation.user_id
        )
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_user_get_invitation_creator_bad_id(anonymous_backend_sock):
    rep = await user_get_invitation_creator(anonymous_backend_sock, invited_user_id=42)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_user_get_invitation_creator_unknown(anonymous_backend_sock, mallory):
    rep = await user_get_invitation_creator(anonymous_backend_sock, invited_user_id=mallory.user_id)
    assert rep == {"status": "not_found"}


# TODO: user_get_invitation_creator with a creator not certified by root


@pytest.mark.trio
async def test_user_get_invitation_creator_ok(
    certificates_store, alice, mallory_invitation, anonymous_backend_sock
):
    rep = await user_get_invitation_creator(
        anonymous_backend_sock, invited_user_id=mallory_invitation.user_id
    )
    assert rep == {
        "status": "ok",
        "user_id": alice.user_id,
        "user_certificate": certificates_store.get_user(alice),
        "trustchain": [],
    }


@pytest.mark.trio
async def test_user_get_invitation_creator_with_trustchain_ok(
    certificates_store,
    backend_data_binder_factory,
    local_device_factory,
    backend,
    alice,
    mallory,
    anonymous_backend_sock,
):
    binder = backend_data_binder_factory(backend)

    roger1 = local_device_factory("roger@dev1")
    mike1 = local_device_factory("mike@dev1")

    await binder.bind_device(roger1, certifier=alice)
    await binder.bind_device(mike1, certifier=roger1)
    await binder.bind_revocation(mike1, certifier=roger1)

    invitation = UserInvitation(user_id=mallory.user_id, creator=mike1.device_id)
    await backend.user.create_user_invitation(alice.organization_id, invitation)

    rep = await user_get_invitation_creator(
        anonymous_backend_sock, invited_user_id=invitation.user_id
    )
    rep["trustchain"] = sorted(rep["trustchain"], key=lambda x: x["device_id"])
    assert rep == {
        "status": "ok",
        "user_id": mike1.user_id,
        "user_certificate": certificates_store.get_user(mike1),
        "trustchain": [
            {
                "device_id": alice.device_id,
                "device_certificate": certificates_store.get_device(alice),
                "revoked_device_certificate": certificates_store.get_revoked_device(alice),
            },
            {
                "device_id": roger1.device_id,
                "device_certificate": certificates_store.get_device(roger1),
                "revoked_device_certificate": certificates_store.get_revoked_device(roger1),
            },
        ],
    }

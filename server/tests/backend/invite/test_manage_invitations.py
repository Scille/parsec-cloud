# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    BackendEventInviteStatusChanged,
    DateTime,
    InvitationType,
)
from parsec.api.protocol import (
    ApiV2V3_APIEventInviteStatusChanged,
    ApiV2V3_EventsListenRepOk,
    HandshakeBadIdentity,
    InvitationDeletedReason,
    InvitationEmailSentStatus,
    InvitationStatus,
    InviteDeleteRepAlreadyDeleted,
    InviteDeleteRepNotFound,
    InviteDeleteRepOk,
    InviteInfoRepOk,
    InviteListItemDevice,
    InviteListItemUser,
    InviteListRepOk,
    InviteNewRepAlreadyMember,
    InviteNewRepNotAllowed,
    InviteNewRepNotAvailable,
    InviteNewRepOk,
    UserOrDeviceDevice,
    UserOrDeviceUser,
    UserProfile,
)
from tests.backend.common import (
    apiv2v3_events_listen_wait,
    apiv2v3_events_subscribe,
    invite_delete,
    invite_info,
    invite_list,
    invite_new,
)
from tests.common import customize_fixtures, freeze_time, real_clock_timeout


@pytest.mark.trio
async def test_user_new_invitation_and_info(
    backend_asgi_app, alice, alice_ws, alice2_ws, backend_invited_ws_factory
):
    # Provide other unrelated invitations that should stay unchanged
    with backend_asgi_app.backend.event_bus.listen() as spy:
        other_device_invitation = await backend_asgi_app.backend.invite.new_for_device(
            organization_id=alice.organization_id,
            greeter_user_id=alice.user_id,
            created_on=DateTime(2000, 1, 2),
        )
        other_user_invitation = await backend_asgi_app.backend.invite.new_for_user(
            organization_id=alice.organization_id,
            greeter_user_id=alice.user_id,
            claimer_email="other@example.com",
            created_on=DateTime(2000, 1, 3),
        )
        await spy.wait_multiple_with_timeout(
            [BackendEventInviteStatusChanged, BackendEventInviteStatusChanged]
        )

    await apiv2v3_events_subscribe(alice2_ws)

    with freeze_time("2000-01-04"):
        rep = await invite_new(alice_ws, type=InvitationType.USER, claimer_email="zack@example.com")

    assert isinstance(rep, InviteNewRepOk)
    token = rep.token

    async with real_clock_timeout():
        rep = await apiv2v3_events_listen_wait(alice2_ws)
    assert rep == ApiV2V3_EventsListenRepOk(
        ApiV2V3_APIEventInviteStatusChanged(token, InvitationStatus.IDLE)
    )

    rep = await invite_list(alice_ws)

    assert rep == InviteListRepOk(
        [
            InviteListItemDevice(
                other_device_invitation.token, DateTime(2000, 1, 2), InvitationStatus.IDLE
            ),
            InviteListItemUser(
                other_user_invitation.token,
                DateTime(2000, 1, 3),
                "other@example.com",
                InvitationStatus.IDLE,
            ),
            InviteListItemUser(
                token, DateTime(2000, 1, 4), "zack@example.com", InvitationStatus.IDLE
            ),
        ]
    )

    async with backend_invited_ws_factory(
        backend_asgi_app,
        organization_id=alice.organization_id,
        invitation_type=InvitationType.USER,
        token=token,
    ) as invited_ws:
        rep = await invite_info(invited_ws)
        assert rep == InviteInfoRepOk(
            UserOrDeviceUser("zack@example.com", alice.user_id, alice.human_handle)
        )


@pytest.mark.trio
async def test_device_new_invitation_and_info(
    backend_asgi_app, alice, alice_ws, alice2_ws, backend_invited_ws_factory
):
    # Provide other unrelated invitations that should stay unchanged
    with backend_asgi_app.backend.event_bus.listen() as spy:
        other_user_invitation = await backend_asgi_app.backend.invite.new_for_user(
            organization_id=alice.organization_id,
            greeter_user_id=alice.user_id,
            claimer_email="other@example.com",
            created_on=DateTime(2000, 1, 2),
        )
        await spy.wait_multiple_with_timeout([BackendEventInviteStatusChanged])

    await apiv2v3_events_subscribe(alice2_ws)

    with freeze_time("2000-01-03"):
        rep = await invite_new(alice_ws, type=InvitationType.DEVICE)
    assert isinstance(rep, InviteNewRepOk)
    token = rep.token

    async with real_clock_timeout():
        rep = await apiv2v3_events_listen_wait(alice2_ws)
    assert rep == ApiV2V3_EventsListenRepOk(
        ApiV2V3_APIEventInviteStatusChanged(token, InvitationStatus.IDLE)
    )

    rep = await invite_list(alice_ws)
    assert rep == InviteListRepOk(
        [
            InviteListItemUser(
                other_user_invitation.token,
                DateTime(2000, 1, 2),
                "other@example.com",
                InvitationStatus.IDLE,
            ),
            InviteListItemDevice(token, DateTime(2000, 1, 3), InvitationStatus.IDLE),
        ]
    )

    async with backend_invited_ws_factory(
        backend_asgi_app,
        organization_id=alice.organization_id,
        invitation_type=InvitationType.DEVICE,
        token=token,
    ) as invited_ws:
        rep = await invite_info(invited_ws)
        assert rep == InviteInfoRepOk(UserOrDeviceDevice(alice.user_id, alice.human_handle))


@pytest.mark.trio
async def test_invite_with_send_mail(alice, alice_ws, email_letterbox):
    base_url = (
        "https" if alice.organization_addr.use_ssl else "http"
    ) + f"://127.0.0.1:{alice.organization_addr.port}"

    # User invitation
    rep = await invite_new(
        alice_ws, type=InvitationType.USER, claimer_email="zack@example.com", send_email=True
    )

    assert isinstance(rep, InviteNewRepOk)
    assert rep.email_sent == InvitationEmailSentStatus.SUCCESS
    token = rep.token
    email = await email_letterbox.get_next_with_timeout()
    assert email == ("zack@example.com", ANY)

    # Lame checks on the sent email
    body = str(email[1])
    assert body.startswith("Content-Type: multipart/alternative;")
    assert 'Content-Type: text/plain; charset="us-ascii"' in body
    assert 'Content-Type: text/html; charset="us-ascii"' in body
    assert "Subject: [Parsec] Alicey McAliceFace invited you to CoolOrg" in body
    assert "From: Parsec <no-reply@parsec.com>" in body
    assert "To: zack@example.com" in body
    assert "Reply-To: =?utf-8?q?Alicey_McAliceFace?= <alice@example.com>" in body
    assert token.hex in body
    assert (
        "You have received an invitation from Alicey McAliceFace to join the CoolOrg organization on Parsec."
        in body
    )
    # Check urls in the email
    assert (
        f'<a href="{base_url}/redirect/CoolOrg?action=claim_user&token={token.hex}" rel="noopener noreferrer" target="_blank">Claim invitation</a>'
        in body
    )
    assert (
        f'<img src="{base_url}/static/parsec-vert.png" alt="Parsec Logo" title="Parsec" width="150" height="100" />'
        in body
    )

    # Device invitation
    rep = await invite_new(alice_ws, type=InvitationType.DEVICE, send_email=True)
    assert isinstance(rep, InviteNewRepOk)
    assert rep.email_sent == InvitationEmailSentStatus.SUCCESS
    token = rep.token
    email = await email_letterbox.get_next_with_timeout()
    assert email == (alice.human_handle.email, ANY)

    # Lame checks on the sent email
    body = str(email[1])
    assert body.startswith("Content-Type: multipart/alternative;")
    assert 'Content-Type: text/plain; charset="us-ascii"' in body
    assert 'Content-Type: text/html; charset="us-ascii"' in body
    assert "Subject: [Parsec] New device invitation to CoolOrg" in body
    assert "From: Parsec <no-reply@parsec.com>" in body
    assert "To: alice@example.com" in body
    assert "Reply-To: " not in body
    assert token.hex in body
    assert (
        "You have received an invitation to add a new device to the CoolOrg organization on Parsec."
        in body
    )
    # Check urls in the email
    assert (
        f'<a href="{base_url}/redirect/CoolOrg?action=claim_device&token={token.hex}" rel="noopener noreferrer" target="_blank">Claim invitation</a>'
        in body
    )
    assert (
        f'<img src="{base_url}/static/parsec-vert.png" alt="Parsec Logo" title="Parsec" width="150" height="100" />'
        in body
    )


@pytest.mark.trio
async def test_invite_with_mail_error(alice, alice_ws, monkeypatch):
    async def _mocked_send_email(email_config, to_addr, message):
        from parsec.backend.invite import InvitationEmailConfigError

        raise InvitationEmailConfigError(Exception())

    monkeypatch.setattr("parsec.backend.invite.send_email", _mocked_send_email)

    # User invitation
    rep = await invite_new(
        alice_ws, type=InvitationType.USER, claimer_email="zack@example.com", send_email=True
    )

    assert isinstance(rep, InviteNewRepOk)
    assert rep.email_sent == InvitationEmailSentStatus.NOT_AVAILABLE

    # Device invitation
    rep = await invite_new(alice_ws, type=InvitationType.DEVICE, send_email=True)
    assert isinstance(rep, InviteNewRepOk)
    assert rep.email_sent == InvitationEmailSentStatus.NOT_AVAILABLE

    async def _mocked_send_email(email_config, to_addr, message):
        from parsec.backend.invite import InvitationEmailRecipientError

        raise InvitationEmailRecipientError(Exception())

    monkeypatch.setattr("parsec.backend.invite.send_email", _mocked_send_email)

    # User invitation
    rep = await invite_new(
        alice_ws, type=InvitationType.USER, claimer_email="zack@example.com", send_email=True
    )

    assert isinstance(rep, InviteNewRepOk)
    assert rep.email_sent == InvitationEmailSentStatus.BAD_RECIPIENT

    # Device invitation
    rep = await invite_new(alice_ws, type=InvitationType.DEVICE, send_email=True)

    assert isinstance(rep, InviteNewRepOk)
    assert rep.email_sent == InvitationEmailSentStatus.BAD_RECIPIENT


@pytest.mark.trio
@customize_fixtures(alice_has_human_handle=False)
async def test_invite_with_send_mail_and_greeter_without_human_handle(
    alice, alice_ws, email_letterbox
):
    # User invitation
    rep = await invite_new(
        alice_ws, type=InvitationType.USER, claimer_email="zack@example.com", send_email=True
    )

    assert isinstance(rep, InviteNewRepOk)
    assert rep.email_sent == InvitationEmailSentStatus.SUCCESS
    token = rep.token
    email = await email_letterbox.get_next_with_timeout()
    assert email == ("zack@example.com", ANY)

    # Lame checks on the sent email
    body = str(email[1])
    assert body.startswith("Content-Type: multipart/alternative;")
    assert 'Content-Type: text/plain; charset="us-ascii"' in body
    assert 'Content-Type: text/html; charset="us-ascii"' in body
    assert "Subject: [Parsec] alice invited you to CoolOrg" in body
    assert "From: Parsec <no-reply@parsec.com>" in body
    assert "To: zack@example.com" in body
    assert "Reply-To: " not in body
    assert token.hex in body

    # Device invitation (not available given no human_handle means no email !)
    rep = await invite_new(alice_ws, type=InvitationType.DEVICE, send_email=True)
    assert isinstance(rep, InviteNewRepNotAvailable)


@pytest.mark.trio
@customize_fixtures(alice_profile=UserProfile.OUTSIDER)
async def test_invite_new_limited_for_outsider(alice_ws):
    rep = await invite_new(alice_ws, type=InvitationType.DEVICE)
    assert isinstance(rep, InviteNewRepOk)

    # Only ADMIN can invite new users
    rep = await invite_new(alice_ws, type=InvitationType.USER, claimer_email="zack@example.com")
    assert isinstance(rep, InviteNewRepNotAllowed)


@pytest.mark.trio
@customize_fixtures(alice_profile=UserProfile.STANDARD)
async def test_invite_new_limited_for_standard(alice_ws):
    # Outsider can only invite new devices
    rep = await invite_new(alice_ws, type=InvitationType.DEVICE)
    assert isinstance(rep, InviteNewRepOk)

    # Only ADMIN can invite new users
    rep = await invite_new(alice_ws, type=InvitationType.USER, claimer_email="zack@example.com")
    assert isinstance(rep, InviteNewRepNotAllowed)


@pytest.mark.trio
async def test_delete_invitation(
    alice, backend_asgi_app, alice_ws, alice2_ws, backend_invited_ws_factory
):
    with backend_asgi_app.backend.event_bus.listen() as spy:
        invitation = await backend_asgi_app.backend.invite.new_for_device(
            organization_id=alice.organization_id,
            greeter_user_id=alice.user_id,
            created_on=DateTime(2000, 1, 2),
        )
        await spy.wait_multiple_with_timeout([BackendEventInviteStatusChanged])

    await apiv2v3_events_subscribe(alice2_ws)

    with backend_asgi_app.backend.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            rep = await invite_delete(
                alice_ws, token=invitation.token, reason=InvitationDeletedReason.CANCELLED
            )

        assert isinstance(rep, InviteDeleteRepOk)
        await spy.wait_with_timeout(BackendEventInviteStatusChanged)

    async with real_clock_timeout():
        rep = await apiv2v3_events_listen_wait(alice2_ws)
    assert rep == ApiV2V3_EventsListenRepOk(
        ApiV2V3_APIEventInviteStatusChanged(invitation.token, InvitationStatus.DELETED)
    )

    # Deleted invitation are no longer visible
    rep = await invite_list(alice_ws)
    assert isinstance(rep, InviteListRepOk)

    # Can no longer use this invitation to connect to the backend
    with pytest.raises(HandshakeBadIdentity):
        async with backend_invited_ws_factory(
            backend_asgi_app,
            organization_id=alice.organization_id,
            invitation_type=InvitationType.DEVICE,
            token=invitation.token,
        ):
            pass


@pytest.mark.trio
@pytest.mark.parametrize("is_revoked", [True, False])
async def test_new_user_invitation_on_already_member(
    backend_data_binder, alice, bob, alice_ws, is_revoked
):
    if is_revoked:
        await backend_data_binder.bind_revocation(user_id=bob.user_id, certifier=alice)

    rep = await invite_new(alice_ws, type=InvitationType.USER, claimer_email=bob.human_handle.email)
    if not is_revoked:
        assert isinstance(rep, InviteNewRepAlreadyMember)
    else:
        assert isinstance(rep, InviteNewRepOk)


@pytest.mark.trio
async def test_idempotent_new_user_invitation(alice, backend, alice_ws):
    claimer_email = "zack@example.com"

    invitation = await backend.invite.new_for_user(
        organization_id=alice.organization_id,
        claimer_email=claimer_email,
        greeter_user_id=alice.user_id,
        created_on=DateTime(2000, 1, 2),
    )

    # Calling invite_new should be idempotent
    with freeze_time("2000-01-03"):
        rep = await invite_new(alice_ws, type=InvitationType.USER, claimer_email=claimer_email)
        assert isinstance(rep, InviteNewRepOk)
        assert rep.token == invitation.token

        rep = await invite_new(alice_ws, type=InvitationType.USER, claimer_email=claimer_email)
        assert isinstance(rep, InviteNewRepOk)
        assert rep.token == invitation.token

    rep = await invite_list(alice_ws)
    assert rep == InviteListRepOk(
        [
            InviteListItemUser(
                invitation.token, DateTime(2000, 1, 2), claimer_email, InvitationStatus.IDLE
            )
        ]
    )


@pytest.mark.trio
async def test_idempotent_new_device_invitation(alice, backend, alice_ws):
    invitation = await backend.invite.new_for_device(
        organization_id=alice.organization_id,
        greeter_user_id=alice.user_id,
        created_on=DateTime(2000, 1, 2),
    )

    # Calling invite_new should be idempotent
    with freeze_time("2000-01-03"):
        rep = await invite_new(alice_ws, type=InvitationType.DEVICE)
        assert isinstance(rep, InviteNewRepOk)
        assert rep.token == invitation.token

        rep = await invite_new(alice_ws, type=InvitationType.DEVICE)
        assert isinstance(rep, InviteNewRepOk)
        assert rep.token == invitation.token

    rep = await invite_list(alice_ws)
    assert rep == InviteListRepOk(
        [InviteListItemDevice(invitation.token, DateTime(2000, 1, 2), InvitationStatus.IDLE)]
    )


@pytest.mark.trio
async def test_new_user_invitation_after_invitation_deleted(alice, backend, alice_ws):
    claimer_email = "zack@example.com"
    invitation = await backend.invite.new_for_user(
        organization_id=alice.organization_id,
        claimer_email=claimer_email,
        greeter_user_id=alice.user_id,
        created_on=DateTime(2000, 1, 2),
    )
    await backend.invite.delete(
        organization_id=alice.organization_id,
        greeter=invitation.greeter_user_id,
        token=invitation.token,
        on=DateTime(2000, 1, 3),
        reason=InvitationDeletedReason.FINISHED,
    )

    # Deleted invitation shouldn't prevent from creating a new one

    with freeze_time("2000-01-04"):
        rep = await invite_new(alice_ws, type=InvitationType.USER, claimer_email=claimer_email)
    assert isinstance(rep, InviteNewRepOk)
    new_token = rep.token
    assert new_token != invitation.token

    rep = await invite_list(alice_ws)
    assert rep == InviteListRepOk(
        [InviteListItemUser(new_token, DateTime(2000, 1, 4), claimer_email, InvitationStatus.IDLE)]
    )


@pytest.mark.trio
async def test_new_device_invitation_after_invitation_deleted(alice, backend, alice_ws):
    invitation = await backend.invite.new_for_device(
        organization_id=alice.organization_id,
        greeter_user_id=alice.user_id,
        created_on=DateTime(2000, 1, 2),
    )
    await backend.invite.delete(
        organization_id=alice.organization_id,
        greeter=invitation.greeter_user_id,
        token=invitation.token,
        on=DateTime(2000, 1, 3),
        reason=InvitationDeletedReason.FINISHED,
    )

    # Deleted invitation shouldn't prevent from creating a new one

    with freeze_time("2000-01-04"):
        rep = await invite_new(alice_ws, type=InvitationType.DEVICE)
    assert isinstance(rep, InviteNewRepOk)
    new_token = rep.token
    assert new_token != invitation.token

    rep = await invite_list(alice_ws)
    assert rep == InviteListRepOk(
        [InviteListItemDevice(new_token, DateTime(2000, 1, 4), InvitationStatus.IDLE)]
    )


@pytest.mark.trio
async def test_delete_already_deleted_invitation(alice, backend, alice_ws):
    invitation = await backend.invite.new_for_device(
        organization_id=alice.organization_id, greeter_user_id=alice.user_id
    )

    await backend.invite.delete(
        organization_id=alice.organization_id,
        greeter=alice.user_id,
        token=invitation.token,
        on=DateTime(2000, 1, 2),
        reason=InvitationDeletedReason.ROTTEN,
    )

    rep = await invite_delete(
        alice_ws, token=invitation.token, reason=InvitationDeletedReason.CANCELLED
    )

    assert isinstance(rep, InviteDeleteRepAlreadyDeleted)


@pytest.mark.trio
async def test_invitation_deletion_isolated_between_users(bob, backend, alice_ws):
    invitation = await backend.invite.new_for_device(
        organization_id=bob.organization_id, greeter_user_id=bob.user_id
    )

    rep = await invite_list(alice_ws)
    assert isinstance(rep, InviteListRepOk)

    rep = await invite_delete(
        alice_ws, token=invitation.token, reason=InvitationDeletedReason.CANCELLED
    )

    assert isinstance(rep, InviteDeleteRepNotFound)


@pytest.mark.trio
async def test_invitation_deletion_isolated_between_organizations(
    alice, other_alice, backend_asgi_app, backend_invited_ws_factory, alice_ws
):
    invitation = await backend_asgi_app.backend.invite.new_for_device(
        organization_id=other_alice.organization_id, greeter_user_id=other_alice.user_id
    )

    rep = await invite_list(alice_ws)
    assert isinstance(rep, InviteListRepOk)

    rep = await invite_delete(
        alice_ws, token=invitation.token, reason=InvitationDeletedReason.CANCELLED
    )

    assert isinstance(rep, InviteDeleteRepNotFound)

    with pytest.raises(HandshakeBadIdentity):
        async with backend_invited_ws_factory(
            backend_asgi_app,
            organization_id=alice.organization_id,
            invitation_type=InvitationType.DEVICE,
            token=invitation.token,
        ):
            pass

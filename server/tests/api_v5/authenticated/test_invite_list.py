# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    DateTime,
    InvitationStatus,
    ShamirRecoveryDeletionCertificate,
    authenticated_cmds,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    ShamirOrgRpcClients,
)


async def test_authenticated_invite_list_ok_with_shamir_recovery(
    shamirorg: ShamirOrgRpcClients,
    backend: Backend,
) -> None:
    expected_invitations = [
        authenticated_cmds.latest.invite_list.InviteListItemShamirRecovery(
            created_on=shamirorg.shamir_invited_alice.event.created_on,
            status=InvitationStatus.IDLE,
            claimer_user_id=shamirorg.alice.user_id,
            shamir_recovery_created_on=shamirorg.alice_brief_certificate.timestamp,
            token=shamirorg.shamir_invited_alice.token,
        )
    ]

    rep = await shamirorg.alice.invite_list()
    assert isinstance(rep, authenticated_cmds.latest.invite_list.RepOk)
    assert rep.invitations == []

    rep = await shamirorg.bob.invite_list()
    assert isinstance(rep, authenticated_cmds.latest.invite_list.RepOk)
    assert rep.invitations == expected_invitations

    rep = await shamirorg.mike.invite_list()
    assert isinstance(rep, authenticated_cmds.latest.invite_list.RepOk)
    assert rep.invitations == expected_invitations

    rep = await shamirorg.mallory.invite_list()
    assert isinstance(rep, authenticated_cmds.latest.invite_list.RepOk)
    assert rep.invitations == expected_invitations


async def test_authenticated_invite_list_ok(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    expected_invitations = []

    # IDLE device invitation
    t1 = DateTime(2020, 1, 1)
    outcome = await backend.invite.new_for_device(
        now=t1,
        organization_id=minimalorg.organization_id,
        author=minimalorg.alice.device_id,
        send_email=False,
    )
    assert isinstance(outcome, tuple)
    expected_invitations.append(
        authenticated_cmds.latest.invite_list.InviteListItemDevice(
            created_on=t1,
            status=InvitationStatus.IDLE,
            token=outcome[0],
        )
    )

    # IDLE user invitation
    t2 = DateTime(2020, 1, 2)
    outcome = await backend.invite.new_for_user(
        now=t2,
        organization_id=minimalorg.organization_id,
        author=minimalorg.alice.device_id,
        claimer_email="zack@example.invalid",
        send_email=False,
    )
    assert isinstance(outcome, tuple)
    expected_invitations.append(
        authenticated_cmds.latest.invite_list.InviteListItemUser(
            created_on=t2,
            status=InvitationStatus.IDLE,
            claimer_email="zack@example.invalid",
            token=outcome[0],
        )
    )

    # # READY user invitation
    # t3 = DateTime(2020, 1, 3)
    # outcome = await backend.invite.new_for_user(
    #     now=t3,
    #     organization_id=shamirorg.organization_id,
    #     author=shamirorg.alice.user_id,
    #     claimer_email="zack@example.invalid",
    #     send_email=False,
    # )
    # assert isinstance(outcome, tuple)
    # expected_invitations.append(
    #     authenticated_cmds.latest.invite_list.InviteListItemUser(
    #         created_on=t3,
    #         status=InvitationStatus.READY,
    #         claimer_email="zack@example.invalid",
    #         token=outcome[0],
    #     )
    # )
    # TODO: have the invitation switch to READY

    # DELETED user invitation
    t4 = DateTime(2020, 1, 4)
    outcome = await backend.invite.new_for_user(
        now=t4,
        organization_id=minimalorg.organization_id,
        author=minimalorg.alice.device_id,
        claimer_email="deleted@example.invalid",
        send_email=False,
    )
    assert isinstance(outcome, tuple)
    t5 = DateTime(2020, 1, 5)
    await backend.invite.cancel(
        now=t5,
        organization_id=minimalorg.organization_id,
        author=minimalorg.alice.device_id,
        token=outcome[0],
    )
    expected_invitations.append(
        authenticated_cmds.latest.invite_list.InviteListItemUser(
            created_on=t4,
            status=InvitationStatus.CANCELLED,
            claimer_email="deleted@example.invalid",
            token=outcome[0],
        )
    )

    rep = await minimalorg.alice.invite_list()
    assert isinstance(rep, authenticated_cmds.latest.invite_list.RepOk)
    assert rep.invitations == expected_invitations


async def test_authenticated_invite_list_with_deleted_shamir(
    shamirorg: ShamirOrgRpcClients,
) -> None:
    # Get invitations
    rep = await shamirorg.bob.invite_list()
    assert isinstance(rep, authenticated_cmds.latest.invite_list.RepOk)
    (previous_invitation,) = rep.invitations
    assert isinstance(
        previous_invitation, authenticated_cmds.latest.invite_list.InviteListItemShamirRecovery
    )

    # Delete Alice shamir recovery
    dt = DateTime.now()
    author = shamirorg.alice
    brief = shamirorg.alice_brief_certificate
    deletion = ShamirRecoveryDeletionCertificate(
        author=author.device_id,
        timestamp=dt,
        setup_to_delete_timestamp=brief.timestamp,
        setup_to_delete_user_id=brief.user_id,
        share_recipients=set(brief.per_recipient_shares.keys()),
    ).dump_and_sign(author.signing_key)
    rep = await shamirorg.alice.shamir_recovery_delete(deletion)
    assert rep == authenticated_cmds.latest.shamir_recovery_delete.RepOk()

    # Expected invitation
    expected = authenticated_cmds.latest.invite_list.InviteListItemShamirRecovery(
        token=previous_invitation.token,
        created_on=previous_invitation.created_on,
        claimer_user_id=previous_invitation.claimer_user_id,
        shamir_recovery_created_on=previous_invitation.shamir_recovery_created_on,
        status=InvitationStatus.CANCELLED,
    )

    # Check invitations
    rep = await shamirorg.bob.invite_list()
    assert isinstance(rep, authenticated_cmds.latest.invite_list.RepOk)
    (invitation,) = rep.invitations
    assert invitation == expected


async def test_authenticated_invite_list_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.invite_list()

    await authenticated_http_common_errors_tester(do)

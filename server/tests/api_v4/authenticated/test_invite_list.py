# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    DateTime,
    InvitationStatus,
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
        authenticated_cmds.v4.invite_list.InviteListItemShamirRecovery(
            created_on=shamirorg.shamir_invited_alice.event.created_on,
            status=InvitationStatus.IDLE,
            claimer_user_id=shamirorg.alice.user_id,
            token=shamirorg.shamir_invited_alice.token,
        )
    ]

    rep = await shamirorg.bob.invite_list()
    assert isinstance(rep, authenticated_cmds.v4.invite_list.RepOk)
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
        authenticated_cmds.v4.invite_list.InviteListItemDevice(
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
        authenticated_cmds.v4.invite_list.InviteListItemUser(
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
    #     authenticated_cmds.v4.invite_list.InviteListItemUser(
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
        authenticated_cmds.v4.invite_list.InviteListItemUser(
            created_on=t4,
            status=InvitationStatus.CANCELLED,
            claimer_email="deleted@example.invalid",
            token=outcome[0],
        )
    )

    rep = await minimalorg.alice.invite_list()
    assert isinstance(rep, authenticated_cmds.v4.invite_list.RepOk)
    assert rep.invitations == expected_invitations


async def test_authenticated_invite_list_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.invite_list()

    await authenticated_http_common_errors_tester(do)

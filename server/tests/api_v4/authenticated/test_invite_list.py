# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    InvitationStatus,
    InvitationToken,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryShareCertificate,
    authenticated_cmds,
)
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, MinimalorgRpcClients


@pytest.fixture
async def alice_shamir(backend: Backend, coolorg: CoolorgRpcClients, with_postgresql: bool) -> None:
    if with_postgresql:
        pytest.xfail("TODO: postgre not implemented yet")
    dt = DateTime.now()
    bob_share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.bob.user_id,
        ciphered_share=b"abc",
    )
    mallory_share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.mallory.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        threshold=2,
        per_recipient_shares={coolorg.bob.user_id: 1, coolorg.mallory.user_id: 1},
    )

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [
            bob_share.dump_and_sign(coolorg.alice.signing_key),
            mallory_share.dump_and_sign(coolorg.alice.signing_key),
        ],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepOk()


async def test_authenticated_invite_list_ok_with_shamir_recovery(
    coolorg: CoolorgRpcClients, backend: Backend, alice_shamir: None
) -> None:
    expected_invitations = []

    # Shamir recovery invitation
    t6 = DateTime(2020, 1, 6)
    outcome = await backend.invite.new_for_shamir_recovery(
        now=t6,
        organization_id=coolorg.organization_id,
        author=coolorg.bob.device_id,
        claimer_user_id=coolorg.alice.user_id,
        send_email=False,
    )
    assert isinstance(outcome, tuple)
    expected_invitations.append(
        authenticated_cmds.v4.invite_list.InviteListItemShamirRecovery(
            created_on=t6,
            status=InvitationStatus.IDLE,
            claimer_user_id=coolorg.alice.user_id,
            token=outcome[0],
        )
    )

    rep = await coolorg.bob.invite_list()
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
    #     organization_id=coolorg.organization_id,
    #     author=coolorg.alice.user_id,
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

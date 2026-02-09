# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    AccessToken,
    DateTime,
    GreetingAttemptID,
    ShamirRecoveryDeletionCertificate,
    authenticated_cmds,
)
from tests.common import CoolorgRpcClients, HttpCommonErrorsTester, ShamirOrgRpcClients

Response = authenticated_cmds.latest.invite_greeter_start_greeting_attempt.Rep | None


async def test_authenticated_invite_greeter_start_greeting_attempt_ok(
    coolorg: CoolorgRpcClients,
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    # Greeter goes first
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    first_greeting_attempt = rep.greeting_attempt

    # Greeter starts again, getting a second greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    second_greeting_attempt = rep.greeting_attempt
    assert second_greeting_attempt != first_greeting_attempt


async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=AccessToken.new(),
    )

    assert (
        rep
        == authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepInvitationNotFound()
    )


async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_cancelled(
    coolorg: CoolorgRpcClients,
) -> None:
    await coolorg.alice.invite_cancel(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert (
        rep
        == authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepInvitationCancelled()
    )


async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_completed(
    coolorg: CoolorgRpcClients,
) -> None:
    await coolorg.alice.invite_complete(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert (
        rep
        == authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepInvitationCompleted()
    )


async def test_authenticated_invite_greeter_start_greeting_attempt_author_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.bob.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert (
        rep == authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepAuthorNotAllowed()
    )


async def test_authenticated_invite_greeter_start_greeting_attempt_author_not_allowed_with_shamir(
    shamirorg: ShamirOrgRpcClients,
) -> None:
    rep = await shamirorg.mike.invite_new_shamir_recovery(
        shamirorg.mallory.user_id, send_email=False
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_new_shamir_recovery.RepOk)
    rep = await shamirorg.bob.invite_greeter_start_greeting_attempt(
        token=rep.token,
    )
    assert (
        rep == authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepAuthorNotAllowed()
    )


async def test_authenticated_invite_greeter_start_greeting_attempt_with_deleted_shamir(
    shamirorg: ShamirOrgRpcClients,
) -> None:
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

    rep = await shamirorg.bob.invite_greeter_start_greeting_attempt(
        token=shamirorg.shamir_invited_alice.token,
    )
    assert (
        rep
        == authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepInvitationCancelled()
    )


async def test_authenticated_invite_greeter_start_greeting_attempt_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.invite_greeter_start_greeting_attempt(
            token=coolorg.invited_alice_dev3.token,
        )

    await authenticated_http_common_errors_tester(do)

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, GreetingAttemptID, RevokedUserCertificate, invited_cmds
from parsec.components.invite import (
    InvitationCreatedByUser,
    UserGreetingAdministrator,
    UserOnlineStatus,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    ShamirOrgRpcClients,
    bob_becomes_admin,
)


@pytest.mark.parametrize("user_or_device", ("user", "device"))
async def test_invited_invite_info_ok(user_or_device: str, coolorg: CoolorgRpcClients) -> None:
    match user_or_device:
        case "user":
            rep = await coolorg.invited_zack.invite_info()
            assert rep == invited_cmds.latest.invite_info.RepOk(
                invited_cmds.latest.invite_info.InvitationTypeUser(
                    claimer_email=coolorg.invited_zack.claimer_email,
                    created_by=InvitationCreatedByUser(
                        user_id=coolorg.alice.user_id,
                        human_handle=coolorg.alice.human_handle,
                    ).for_invite_info(),
                    administrators=[
                        UserGreetingAdministrator(
                            user_id=coolorg.alice.user_id,
                            human_handle=coolorg.alice.human_handle,
                            online_status=UserOnlineStatus.UNKNOWN,
                            last_greeting_attempt_joined_on=None,
                        ),
                    ],
                )
            )

        case "device":
            rep = await coolorg.invited_alice_dev3.invite_info()
            assert rep == invited_cmds.latest.invite_info.RepOk(
                invited_cmds.latest.invite_info.InvitationTypeDevice(
                    claimer_user_id=coolorg.alice.user_id,
                    claimer_human_handle=coolorg.alice.human_handle,
                    created_by=InvitationCreatedByUser(
                        user_id=coolorg.alice.user_id,
                        human_handle=coolorg.alice.human_handle,
                    ).for_invite_info(),
                )
            )

        case unknown:
            assert False, unknown


async def test_invited_invite_info_for_user_with_greeting_attempt(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    now = DateTime.now()
    rep = await backend.invite.greeter_start_greeting_attempt(
        now,
        coolorg.organization_id,
        coolorg.alice.device_id,
        coolorg.alice.user_id,
        coolorg.invited_zack.token,
    )
    assert isinstance(rep, GreetingAttemptID)

    rep = await coolorg.invited_zack.invite_info()
    assert rep == invited_cmds.latest.invite_info.RepOk(
        invited_cmds.latest.invite_info.InvitationTypeUser(
            claimer_email=coolorg.invited_zack.claimer_email,
            created_by=InvitationCreatedByUser(
                user_id=coolorg.alice.user_id,
                human_handle=coolorg.alice.human_handle,
            ).for_invite_info(),
            administrators=[
                UserGreetingAdministrator(
                    user_id=coolorg.alice.user_id,
                    human_handle=coolorg.alice.human_handle,
                    online_status=UserOnlineStatus.UNKNOWN,
                    last_greeting_attempt_joined_on=now,
                ),
            ],
        )
    )


async def test_invited_invite_info_for_user_with_multiple_admins(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    await bob_becomes_admin(coolorg, backend)

    rep = await coolorg.invited_zack.invite_info()
    assert rep == invited_cmds.latest.invite_info.RepOk(
        invited_cmds.latest.invite_info.InvitationTypeUser(
            claimer_email=coolorg.invited_zack.claimer_email,
            created_by=InvitationCreatedByUser(
                user_id=coolorg.alice.user_id,
                human_handle=coolorg.alice.human_handle,
            ).for_invite_info(),
            administrators=[
                UserGreetingAdministrator(
                    user_id=coolorg.alice.user_id,
                    human_handle=coolorg.alice.human_handle,
                    online_status=UserOnlineStatus.UNKNOWN,
                    last_greeting_attempt_joined_on=None,
                ),
                UserGreetingAdministrator(
                    user_id=coolorg.bob.user_id,
                    human_handle=coolorg.bob.human_handle,
                    online_status=UserOnlineStatus.UNKNOWN,
                    last_greeting_attempt_joined_on=None,
                ),
            ],
        )
    )

    # New invitation for non-zack
    t0 = DateTime.now()
    rep = await backend.invite.new_for_user(
        t0,
        coolorg.organization_id,
        coolorg.alice.device_id,
        "non.zack@example.invalid",
        False,
    )
    assert isinstance(rep, tuple)
    non_zack_token, _ = rep

    # Alice starts greeting attempt for zack
    t1 = DateTime.now()
    rep = await backend.invite.greeter_start_greeting_attempt(
        t1,
        coolorg.organization_id,
        coolorg.alice.device_id,
        coolorg.alice.user_id,
        coolorg.invited_zack.token,
    )
    assert isinstance(rep, GreetingAttemptID)

    # Bob starts greeting attempt for non-zack
    t2 = DateTime.now()
    rep = await backend.invite.greeter_start_greeting_attempt(
        t2,
        coolorg.organization_id,
        coolorg.bob.device_id,
        coolorg.bob.user_id,
        non_zack_token,
    )
    assert isinstance(rep, GreetingAttemptID)

    # Check the invite info for zack
    rep = await coolorg.invited_zack.invite_info()
    assert rep == invited_cmds.latest.invite_info.RepOk(
        invited_cmds.latest.invite_info.InvitationTypeUser(
            claimer_email=coolorg.invited_zack.claimer_email,
            created_by=InvitationCreatedByUser(
                user_id=coolorg.alice.user_id,
                human_handle=coolorg.alice.human_handle,
            ).for_invite_info(),
            administrators=[
                UserGreetingAdministrator(
                    user_id=coolorg.alice.user_id,
                    human_handle=coolorg.alice.human_handle,
                    online_status=UserOnlineStatus.UNKNOWN,
                    last_greeting_attempt_joined_on=t1,
                ),
                UserGreetingAdministrator(
                    user_id=coolorg.bob.user_id,
                    human_handle=coolorg.bob.human_handle,
                    online_status=UserOnlineStatus.UNKNOWN,
                    last_greeting_attempt_joined_on=None,
                ),
            ],
        )
    )

    # Alice re-starts greeting attempt for zack
    t3 = DateTime.now()
    rep = await backend.invite.greeter_start_greeting_attempt(
        t3,
        coolorg.organization_id,
        coolorg.alice.device_id,
        coolorg.alice.user_id,
        coolorg.invited_zack.token,
    )
    assert isinstance(rep, GreetingAttemptID)

    # Bob starts greeting attempt for zack
    t4 = DateTime.now()
    rep = await backend.invite.greeter_start_greeting_attempt(
        t4,
        coolorg.organization_id,
        coolorg.bob.device_id,
        coolorg.bob.user_id,
        coolorg.invited_zack.token,
    )

    # Check the invite info for zack
    rep = await coolorg.invited_zack.invite_info()
    assert rep == invited_cmds.latest.invite_info.RepOk(
        invited_cmds.latest.invite_info.InvitationTypeUser(
            claimer_email=coolorg.invited_zack.claimer_email,
            created_by=InvitationCreatedByUser(
                user_id=coolorg.alice.user_id,
                human_handle=coolorg.alice.human_handle,
            ).for_invite_info(),
            administrators=[
                UserGreetingAdministrator(
                    user_id=coolorg.alice.user_id,
                    human_handle=coolorg.alice.human_handle,
                    online_status=UserOnlineStatus.UNKNOWN,
                    last_greeting_attempt_joined_on=t3,
                ),
                UserGreetingAdministrator(
                    user_id=coolorg.bob.user_id,
                    human_handle=coolorg.bob.human_handle,
                    online_status=UserOnlineStatus.UNKNOWN,
                    last_greeting_attempt_joined_on=t4,
                ),
            ],
        )
    )


async def test_invited_invite_info_ok_with_shamir(
    shamirorg: ShamirOrgRpcClients, backend: Backend
) -> None:
    # Revoke Mallory first
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=shamirorg.alice.device_id,
        user_id=shamirorg.mallory.user_id,
        timestamp=now,
    )
    await backend.user.revoke_user(
        now=now,
        organization_id=shamirorg.organization_id,
        author=shamirorg.alice.device_id,
        author_verify_key=shamirorg.alice.signing_key.verify_key,
        revoked_user_certificate=certif.dump_and_sign(shamirorg.alice.signing_key),
    )

    # Check the invite info
    rep = await shamirorg.shamir_invited_alice.invite_info()
    assert rep == invited_cmds.latest.invite_info.RepOk(
        invited_cmds.latest.invite_info.InvitationTypeShamirRecovery(
            claimer_user_id=shamirorg.alice.user_id,
            claimer_human_handle=shamirorg.alice.human_handle,
            created_by=InvitationCreatedByUser(
                user_id=shamirorg.bob.user_id,
                human_handle=shamirorg.bob.human_handle,
            ).for_invite_info(),
            shamir_recovery_created_on=shamirorg.alice_brief_certificate.timestamp,
            threshold=2,
            recipients=[
                invited_cmds.latest.invite_info.ShamirRecoveryRecipient(
                    user_id=shamirorg.bob.user_id,
                    human_handle=shamirorg.bob.human_handle,
                    shares=2,
                    revoked_on=None,
                    online_status=UserOnlineStatus.UNKNOWN,
                ),
                invited_cmds.latest.invite_info.ShamirRecoveryRecipient(
                    user_id=shamirorg.mallory.user_id,
                    human_handle=shamirorg.mallory.human_handle,
                    shares=1,
                    revoked_on=now,
                    online_status=UserOnlineStatus.UNKNOWN,
                ),
                invited_cmds.latest.invite_info.ShamirRecoveryRecipient(
                    user_id=shamirorg.mike.user_id,
                    human_handle=shamirorg.mike.human_handle,
                    shares=1,
                    revoked_on=None,
                    online_status=UserOnlineStatus.UNKNOWN,
                ),
            ],
        )
    )


async def test_invited_invite_info_http_common_errors(
    coolorg: CoolorgRpcClients, invited_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.invited_alice_dev3.invite_info()

    await invited_http_common_errors_tester(do)

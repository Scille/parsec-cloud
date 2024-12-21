# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, RevokedUserCertificate, invited_cmds
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, ShamirOrgRpcClients


@pytest.mark.parametrize("user_or_device", ("user", "device"))
async def test_invited_invite_info_ok(user_or_device: str, coolorg: CoolorgRpcClients) -> None:
    match user_or_device:
        case "user":
            rep = await coolorg.invited_zack.invite_info()
            assert rep == invited_cmds.v4.invite_info.RepOk(
                invited_cmds.v4.invite_info.InvitationTypeUser(
                    claimer_email=coolorg.invited_zack.claimer_email,
                    greeter_user_id=coolorg.alice.user_id,
                    greeter_human_handle=coolorg.alice.human_handle,
                )
            )

        case "device":
            rep = await coolorg.invited_alice_dev3.invite_info()
            assert rep == invited_cmds.v4.invite_info.RepOk(
                invited_cmds.v4.invite_info.InvitationTypeDevice(
                    greeter_user_id=coolorg.alice.user_id,
                    greeter_human_handle=coolorg.alice.human_handle,
                )
            )

        case unknown:
            assert False, unknown


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
    assert rep == invited_cmds.v4.invite_info.RepOk(
        invited_cmds.v4.invite_info.InvitationTypeShamirRecovery(
            claimer_user_id=shamirorg.alice.user_id,
            claimer_human_handle=shamirorg.alice.human_handle,
            shamir_recovery_created_on=shamirorg.alice_brief_certificate.timestamp,
            threshold=2,
            recipients=[
                invited_cmds.latest.invite_info.ShamirRecoveryRecipient(
                    user_id=shamirorg.bob.user_id,
                    human_handle=shamirorg.bob.human_handle,
                    shares=2,
                    revoked_on=None,
                ),
                invited_cmds.latest.invite_info.ShamirRecoveryRecipient(
                    user_id=shamirorg.mallory.user_id,
                    human_handle=shamirorg.mallory.human_handle,
                    shares=1,
                    revoked_on=now,
                ),
                invited_cmds.latest.invite_info.ShamirRecoveryRecipient(
                    user_id=shamirorg.mike.user_id,
                    human_handle=shamirorg.mike.human_handle,
                    shares=1,
                    revoked_on=None,
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

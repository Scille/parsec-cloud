# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import authenticated_cmds, invited_cmds
from tests.common import CoolorgRpcClients, HttpCommonErrorsTester, ShamirOrgRpcClients
from tests.common.client import InvitedRpcClient


@pytest.mark.parametrize("user_or_device", ("user", "device"))
async def test_invited_invite_info_ok(user_or_device: str, coolorg: CoolorgRpcClients) -> None:
    match user_or_device:
        case "user":
            rep = await coolorg.invited_zack.invite_info()
            assert rep == invited_cmds.v4.invite_info.RepOk(
                invited_cmds.v4.invite_info.UserOrDeviceUser(
                    claimer_email=coolorg.invited_zack.claimer_email,
                    greeter_user_id=coolorg.alice.user_id,
                    greeter_human_handle=coolorg.alice.human_handle,
                )
            )

        case "device":
            rep = await coolorg.invited_alice_dev3.invite_info()
            assert rep == invited_cmds.v4.invite_info.RepOk(
                invited_cmds.v4.invite_info.UserOrDeviceDevice(
                    greeter_user_id=coolorg.alice.user_id,
                    greeter_human_handle=coolorg.alice.human_handle,
                )
            )

        case unknown:
            assert False, unknown


async def test_invited_invite_info_ok_with_shamir(shamirorg: ShamirOrgRpcClients) -> None:
    # TODO: add `TestbedEventNewShamirRecoveryInvitation` event
    rep = await shamirorg.bob.invite_new_shamir_recovery(
        send_email=False,
        claimer_user_id=shamirorg.alice.user_id,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_new_shamir_recovery.RepOk)

    shamir_invited_bob = InvitedRpcClient(shamirorg.raw_client, shamirorg.organization_id, rep)  # type: ignore
    rep = await shamir_invited_bob.invite_info()
    assert rep == invited_cmds.v4.invite_info.RepOk(
        invited_cmds.v4.invite_info.UserOrDeviceShamirRecovery(
            threshold=2,
            recipients=[
                invited_cmds.latest.invite_info.ShamirRecoveryRecipient(
                    user_id=shamirorg.bob.user_id,
                    human_handle=shamirorg.bob.human_handle,
                    shares=2,
                ),
                invited_cmds.latest.invite_info.ShamirRecoveryRecipient(
                    user_id=shamirorg.mallory.user_id,
                    human_handle=shamirorg.mallory.human_handle,
                    shares=1,
                ),
                invited_cmds.latest.invite_info.ShamirRecoveryRecipient(
                    user_id=shamirorg.mike.user_id,
                    human_handle=shamirorg.mike.human_handle,
                    shares=1,
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

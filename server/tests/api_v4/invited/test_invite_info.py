# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import invited_cmds
from tests.common import CoolorgRpcClients


@pytest.mark.parametrize("user_or_device", ("user", "device"))
async def test_invited_invite_info_ok(user_or_device: str, coolorg: CoolorgRpcClients) -> None:
    match user_or_device:
        case "user":
            rep = await coolorg.invited_zack.invite_info()
            assert rep == invited_cmds.v4.invite_info.RepOk(
                invited_cmds.v4.invite_info.UserOrDeviceUser(
                    claimer_email=coolorg.invited_zack.claimer_email,
                    greeter_user_id=coolorg.alice.device_id.user_id,
                    greeter_human_handle=coolorg.alice.human_handle,
                )
            )

        case "device":
            rep = await coolorg.invited_alice_dev3.invite_info()
            assert rep == invited_cmds.v4.invite_info.RepOk(
                invited_cmds.v4.invite_info.UserOrDeviceDevice(
                    greeter_user_id=coolorg.alice.device_id.user_id,
                    greeter_human_handle=coolorg.alice.human_handle,
                )
            )

        case unknown:
            assert False, unknown

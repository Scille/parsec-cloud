# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import authenticated_cmds
from parsec.components.user import UserInfo
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester


async def test_authenticated_list_frozen_users_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    rep = await coolorg.alice.list_frozen_users()
    assert rep == authenticated_cmds.v4.list_frozen_users.RepOk(frozen_users=[])

    outcome = await backend.user.freeze_user(
        organization_id=coolorg.organization_id,
        user_id=coolorg.mallory.user_id,
        user_email=None,
        frozen=True,
    )
    assert isinstance(outcome, UserInfo)

    rep = await coolorg.alice.list_frozen_users()
    assert rep == authenticated_cmds.v4.list_frozen_users.RepOk(
        frozen_users=[coolorg.mallory.user_id]
    )


async def test_authenticated_list_frozen_users_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    pass
    rep = await coolorg.mallory.list_frozen_users()  # mallory is an outsider
    assert rep == authenticated_cmds.v4.list_frozen_users.RepAuthorNotAllowed()


async def test_authenticated_list_frozen_users_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.list_frozen_users()

    await authenticated_http_common_errors_tester(do)

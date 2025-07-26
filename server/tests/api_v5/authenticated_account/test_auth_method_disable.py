# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    SecretKey,
    authenticated_account_cmds,
)
from tests.common import (
    AsyncClient,
    AuthenticatedAccountRpcClient,
    Backend,
    HttpCommonErrorsTester,
    RpcTransportError,
)


async def test_authenticated_account_auth_method_disable_ok(
    alice_account: AuthenticatedAccountRpcClient,
    backend: Backend,
    client: AsyncClient,
) -> None:
    # First create a new auth method for alice...

    new_auth_method_id = AccountAuthMethodID.new()
    new_auth_method_mac_key = SecretKey.generate()

    outcome = await backend.account.auth_method_create(
        now=DateTime.now(),
        auth_method_id=alice_account.auth_method_id,
        created_by_user_agent="",
        created_by_ip="",
        new_auth_method_id=new_auth_method_id,
        new_auth_method_mac_key=new_auth_method_mac_key,
        new_auth_method_password_algorithm=None,
        new_vault_key_access=b"<vault_key_access_data>",
    )
    assert outcome is None

    # ...and disable it

    rep = await alice_account.auth_method_disable(
        auth_method_id=new_auth_method_id,
    )
    assert rep == authenticated_account_cmds.latest.auth_method_disable.RepOk()

    # The disabled auth method should no longer work

    new_alice_account = AuthenticatedAccountRpcClient(
        client,
        alice_account.account_email,
        new_auth_method_id,
        new_auth_method_mac_key,
    )
    with pytest.raises(RpcTransportError) as ctx:
        await new_alice_account.ping(ping="ping")
    assert ctx.value.rep.status_code == 403

    # But obviously the other auth method should still work

    await alice_account.ping(ping="ping")


@pytest.mark.parametrize(
    "kind", ("non_existing", "from_other_account", "from_other_account_already_disabled")
)
async def test_authenticated_account_auth_method_disable_auth_method_not_found(
    alice_account: AuthenticatedAccountRpcClient,
    bob_account: AuthenticatedAccountRpcClient,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "non_existing":
            target_auth_method_id = AccountAuthMethodID.new()
        case "from_other_account":
            target_auth_method_id = bob_account.auth_method_id
        case "from_other_account_already_disabled":
            # Create a new auth method for bob...

            new_auth_method_id = AccountAuthMethodID.new()
            new_auth_method_mac_key = SecretKey.generate()

            outcome = await backend.account.auth_method_create(
                now=DateTime.now(),
                auth_method_id=bob_account.auth_method_id,
                created_by_user_agent="",
                created_by_ip="",
                new_auth_method_id=new_auth_method_id,
                new_auth_method_mac_key=new_auth_method_mac_key,
                new_auth_method_password_algorithm=None,
                new_vault_key_access=b"<vault_key_access_data>",
            )
            assert outcome is None

            # ...and disable it

            outcome = await backend.account.auth_method_disable(
                now=DateTime.now(),
                auth_method_id=bob_account.auth_method_id,
                to_disable_auth_method_id=new_auth_method_id,
            )
            assert outcome is None

            target_auth_method_id = new_auth_method_id

        case unknown:
            assert False, unknown

    rep = await alice_account.auth_method_disable(
        auth_method_id=target_auth_method_id,
    )
    assert rep == authenticated_account_cmds.latest.auth_method_disable.RepAuthMethodNotFound()


async def test_authenticated_account_auth_method_disable_auth_method_already_disabled(
    alice_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    # First create a new auth method...

    new_auth_method_id = AccountAuthMethodID.new()
    new_auth_method_mac_key = SecretKey.generate()

    outcome = await backend.account.auth_method_create(
        now=DateTime.now(),
        auth_method_id=alice_account.auth_method_id,
        created_by_user_agent="",
        created_by_ip="",
        new_auth_method_id=new_auth_method_id,
        new_auth_method_mac_key=new_auth_method_mac_key,
        new_auth_method_password_algorithm=None,
        new_vault_key_access=b"<vault_key_access_data>",
    )
    assert outcome is None

    # ...and disable it

    outcome = await backend.account.auth_method_disable(
        now=DateTime.now(),
        auth_method_id=alice_account.auth_method_id,
        to_disable_auth_method_id=new_auth_method_id,
    )
    assert outcome is None

    # Cannot disable it again!

    rep = await alice_account.auth_method_disable(
        auth_method_id=new_auth_method_id,
    )
    assert (
        rep == authenticated_account_cmds.latest.auth_method_disable.RepAuthMethodAlreadyDisabled()
    )


async def test_authenticated_account_auth_method_disable_self_disable_not_allowed(
    alice_account: AuthenticatedAccountRpcClient,
) -> None:
    rep = await alice_account.auth_method_disable(
        auth_method_id=alice_account.auth_method_id,
    )
    assert rep == authenticated_account_cmds.latest.auth_method_disable.RepSelfDisableNotAllowed()


async def test_authenticated_account_auth_method_disable_http_common_errors(
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await alice_account.auth_method_disable(
            # The http common errors should be handled before the check preventing self disabling
            auth_method_id=alice_account.auth_method_id,
        )

    await authenticated_account_http_common_errors_tester(do)

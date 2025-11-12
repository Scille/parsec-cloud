# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    RevokedUserCertificate,
    UserProfile,
    authenticated_account_cmds,
)
from tests.common import (
    AuthenticatedAccountRpcClient,
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    ShamirOrgRpcClients,
    bob_becomes_admin_and_changes_alice,
)


async def test_authenticated_account_organization_self_list_ok(
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
    bob_account: AuthenticatedAccountRpcClient,
    minimalorg: MinimalorgRpcClients,
    shamirorg: ShamirOrgRpcClients,
    coolorg: CoolorgRpcClients,
) -> None:
    def cook_and_compare(
        got: authenticated_account_cmds.latest.organization_self_list.Rep,
        expected: authenticated_account_cmds.latest.organization_self_list.Rep,
    ):
        if isinstance(got, authenticated_account_cmds.latest.organization_self_list.RepOk):
            to_keep_org_ids = (
                minimalorg.organization_id,
                shamirorg.organization_id,
                coolorg.organization_id,
            )
            cooked_got_active = sorted(
                # Only consider the organizations we know about, since there is also
                # template organizations and organizations created from previous tests.
                (x for x in got.active if x.organization_id in to_keep_org_ids),
                key=lambda x: x.organization_id.str,
            )
            cooked_revoked = sorted(
                (x for x in got.revoked if x.organization_id in to_keep_org_ids),
                key=lambda x: (x.organization_id.str, x.revoked_on),
            )
            cooked_got = authenticated_account_cmds.latest.organization_self_list.RepOk(
                active=cooked_got_active,
                revoked=cooked_revoked,
            )
        else:
            cooked_got = got

        if isinstance(expected, authenticated_account_cmds.latest.organization_self_list.RepOk):
            cooked_expected = authenticated_account_cmds.latest.organization_self_list.RepOk(
                active=sorted(expected.active, key=lambda x: x.organization_id.str),
                revoked=sorted(
                    expected.revoked, key=lambda x: (x.organization_id.str, x.revoked_on)
                ),
            )
        else:
            cooked_expected = expected

        assert cooked_got == cooked_expected

    # Frozen should have no effect (but it is a common mistake to confuse it with revoked)
    await backend.user.freeze_user(
        minimalorg.organization_id, user_id=minimalorg.alice.user_id, user_email=None, frozen=True
    )

    # Revoke Alice from Coolorg
    _, (certif, _) = await bob_becomes_admin_and_changes_alice(
        coolorg, backend, new_alice_profile=None
    )
    assert isinstance(certif, RevokedUserCertificate)
    coolorg_alice_revoked_on = certif.timestamp

    outcome = await backend.organization.update(
        now=DateTime.now(),
        id=minimalorg.organization_id,
        is_expired=True,
        active_users_limit=ActiveUsersLimit.limited_to(1),
        user_profile_outsider_allowed=False,
    )
    assert outcome is None

    rep = await alice_account.organization_self_list()
    cook_and_compare(
        rep,
        authenticated_account_cmds.latest.organization_self_list.RepOk(
            active=[
                authenticated_account_cmds.latest.organization_self_list.ActiveUser(
                    organization_id=minimalorg.organization_id,
                    user_id=minimalorg.alice.user_id,
                    created_on=DateTime(2000, 1, 2),
                    is_frozen=True,
                    current_profile=UserProfile.ADMIN,
                    organization_config=authenticated_account_cmds.latest.organization_self_list.OrganizationConfig(
                        is_expired=True,
                        user_profile_outsider_allowed=False,
                        active_users_limit=ActiveUsersLimit.limited_to(1),
                    ),
                ),
                authenticated_account_cmds.latest.organization_self_list.ActiveUser(
                    organization_id=shamirorg.organization_id,
                    user_id=shamirorg.alice.user_id,
                    created_on=DateTime(2000, 1, 2),
                    is_frozen=False,
                    current_profile=UserProfile.ADMIN,
                    organization_config=authenticated_account_cmds.latest.organization_self_list.OrganizationConfig(
                        is_expired=False,
                        user_profile_outsider_allowed=True,
                        active_users_limit=ActiveUsersLimit.NO_LIMIT,
                    ),
                ),
            ],
            revoked=[
                authenticated_account_cmds.latest.organization_self_list.RevokedUser(
                    organization_id=coolorg.organization_id,
                    user_id=coolorg.alice.user_id,
                    created_on=DateTime(2000, 1, 2),
                    revoked_on=coolorg_alice_revoked_on,
                    current_profile=UserProfile.ADMIN,
                ),
            ],
        ),
    )


async def test_authenticated_account_organization_self_list_http_common_errors(
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await alice_account.organization_self_list()

    await authenticated_account_http_common_errors_tester(do)

from parsec._parsec import DateTime, authenticated_cmds
from parsec.backend import Backend
from parsec.components.user import UserInvitationInfo
from tests.common.client import AuthenticatedRpcClient, CoolorgRpcClients, MinimalorgRpcClients


async def create_invitation(client: AuthenticatedRpcClient, email: str) -> None:
    res = await client.invite_new_user(claimer_email=email, send_email=False)
    assert isinstance(res, authenticated_cmds.latest.invite_new_user.RepOk)


async def test_list_invitations(
    coolorg: CoolorgRpcClients, minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    email = "foo@example.com"
    # 1. Create some invitations for user of `foo@example.com` email
    await create_invitation(coolorg.alice, email)
    await create_invitation(minimalorg.alice, email)

    # 2. List invitations
    res = await backend.user.list_user_invitations(email)
    now = DateTime.now()

    # We are not able to freeze the time for client, so we need to overwrite the `created_on` field.
    def change_created_on(i: UserInvitationInfo) -> UserInvitationInfo:
        i.created_on = now
        return i

    assert list(map(change_created_on, res)) == [
        UserInvitationInfo(
            organization=coolorg.organization_id,
            created_by=coolorg.alice.human_handle.email,
            created_on=now,
        ),
        UserInvitationInfo(
            organization=minimalorg.organization_id,
            created_by=minimalorg.alice.human_handle.email,
            created_on=now,
        ),
    ]

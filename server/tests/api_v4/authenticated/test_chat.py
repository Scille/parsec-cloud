from parsec._parsec import authenticated_cmds
from parsec.events import EventChatReceived
from tests.common import Backend, MinimalorgRpcClients


async def test_authenticated_chat_ok(minimalorg: MinimalorgRpcClients, backend: Backend) -> None:
    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.chat_post(messageContent="toto")
        # assert False
        # assert rep == authenticated_cmds.v4.chat_post.RepOk()
        # await spy.wait_event_occurred(
        #     EventChatReceived(
        #         organization_id=minimalorg.organization_id,
        #         message="toto",
        #     )
        # )

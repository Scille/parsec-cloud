from typing import cast
from uuid import UUID

from parsec._parsec import DateTime, authenticated_cmds
from parsec.events import EventChatReceived
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_chat_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    with backend.event_bus.spy() as spy:
        # Idk how to pull real keys from the backend
        # Good enough for now
        bobSKey = coolorg.bob.signing_key
        bobPKey = bobSKey.verify_key

        # aliceSKey = coolOrg.alice.signing_key
        # alicePKey = aliceSKey.verify_key

        msgContent = "Hello Bob!"

        # alice create a message and sign it with bob's public key
        msg = await coolorg.alice.chat_create(
            id=123,
            author=coolorg.alice.device_id,
            timestamp=DateTime.now(),
            recipient=coolorg.bob.user_id,
            messageEncrypted= msgContent.encode("utf-8"), # TODO Encrypt the message
        )

        assert type(msg) is authenticated_cmds.v4.chat_create.RepOk
        msg = cast(authenticated_cmds.v4.chat_create.RepOk, msg)

        # alice send the message to the organisation
        rep = await coolorg.alice.chat_post(messageEncrypted=msg.dump())
        assert rep == authenticated_cmds.v4.chat_post.RepOk()
        await spy.wait_event_occurred(
            EventChatReceived(
                organization_id=coolorg.organization_id,
                messageContent=msg,
            )
        )

        # bob receive the message and decrypt it
        lastBobEvent = await coolorg.bob.events_listen()
        assert lastBobEvent == EventChatReceived(
            organization_id=coolorg.organization_id,
            messageContent=msg,
        )
        lastBobEvent = cast(EventChatReceived, lastBobEvent)
        lastBobEvent.messageContent
        assert "Hello Bob !" == bobSKey.sign(lastBobEvent.messageContent)



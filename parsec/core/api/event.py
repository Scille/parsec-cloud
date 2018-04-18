import trio
import logbook

from parsec.networking import ClientContext
from parsec.core.app import Core
from parsec.schema import BaseCmdSchema, fields


# TODO: move event handling into networking module


logger = logbook.Logger("parsec.api.event")


class cmd_EVENT_LISTEN_Schema(BaseCmdSchema):
    wait = fields.Boolean(missing=True)


class cmd_EVENT_SUBSCRIBE_Schema(BaseCmdSchema):
    event = fields.String(required=True)
    subject = fields.String(missing=None)


async def event_subscribe(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_EVENT_SUBSCRIBE_Schema().load_or_abort(req)
    event = msg["event"]
    subject = msg["subject"]

    def _handle_event(sender):
        try:
            core.auth_events.put_nowait((event, sender))
        except trio.WouldBlock:
            logger.warning("event queue is full")

    core.auth_subscribed_events[event, subject] = _handle_event
    if event == "device_try_claim_submitted":
        await core.backend_connection.subscribe_event(event, subject)
    if subject:
        core.signal_ns.signal(event).connect(_handle_event, sender=subject, weak=True)
    else:
        core.signal_ns.signal(event).connect(_handle_event, weak=True)
    return {"status": "ok"}


async def event_unsubscribe(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_EVENT_SUBSCRIBE_Schema().load_or_abort(req)
    try:
        del core.auth_subscribed_events[msg["event"], msg["subject"]]
    except KeyError:
        return {"status": "not_subscribed", "reason": "Not subscribed to this event/subject couple"}

    return {"status": "ok"}


async def event_listen(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_EVENT_LISTEN_Schema().load_or_abort(req)
    if msg["wait"]:
        event, subject = await core.auth_events.get()
    else:
        try:
            event, subject = core.auth_events.get_nowait()
        except trio.WouldBlock:
            return {"status": "ok"}

    # TODO: make more generic
    if event == "device_try_claim_submitted":
        rep = await core.backend_connection.send(
            {"cmd": "device_get_configuration_try", "configuration_try_id": subject}
        )
        assert rep["status"] == "ok"
        core._config_try_pendings[subject] = rep
        return {
            "status": "ok",
            "event": event,
            "device_name": rep["device_name"],
            "configuration_try_id": subject,
        }

    else:
        return {"status": "ok", "event": event, "subject": subject}


async def event_list_subscribed(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    BaseCmdSchema().load_or_abort(req)  # empty msg expected
    return {"status": "ok", "subscribed": list(core.auth_subscribed_events.keys())}

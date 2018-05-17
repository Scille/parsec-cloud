import trio
import logbook

from parsec.core.app import Core, ClientContext
from parsec.core.backend_connection import BackendNotAvailable
from parsec.schema import UnknownCheckedSchema, BaseCmdSchema, fields, validate


logger = logbook.Logger("parsec.api.event")

ALLOWED_SIGNALS = {"ping", "fuse_mountpoint_need_stop", "new_sharing"}
ALLOWED_BACKEND_EVENTS = {"device_try_claim_submitted"}


class BackendGetConfigurationTrySchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    device_name = fields.String(required=True)
    configuration_status = fields.String(required=True)
    device_verify_key = fields.Base64Bytes(required=True)
    user_privkey_cypherkey = fields.Base64Bytes(required=True)


backend_get_configuration_try_schema = BackendGetConfigurationTrySchema()


class cmd_EVENT_LISTEN_Schema(BaseCmdSchema):
    wait = fields.Boolean(missing=True)


class cmd_EVENT_SUBSCRIBE_Schema(BaseCmdSchema):
    event = fields.String(
        required=True, validate=validate.OneOf(ALLOWED_SIGNALS | ALLOWED_BACKEND_EVENTS)
    )
    subject = fields.String(missing=None)


async def event_subscribe(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_EVENT_SUBSCRIBE_Schema().load_or_abort(req)
    event = msg["event"]
    subject = msg["subject"]

    try:
        # Note here we consider `None` as `blinker.ANY` for simplicity sake
        if subject:
            client_ctx.subscribe_signal(event, subject)
        else:
            client_ctx.subscribe_signal(event)
    except KeyError as exc:
        return {
            "status": "already_subscribed",
            "reason": "Already subscribed to this event/subject couple",
        }

    if event in ALLOWED_BACKEND_EVENTS:
        try:
            await core.backend_events_manager.subscribe_backend_event(event, subject)
        except KeyError:
            # Event already registered by another client context
            pass

    return {"status": "ok"}


async def event_unsubscribe(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_EVENT_SUBSCRIBE_Schema().load_or_abort(req)
    event = msg["event"]
    subject = msg["subject"]

    try:
        # Note here we consider `None` as `blinker.ANY` for simplicity sake
        if subject:
            client_ctx.unsubscribe_signal(event, subject)
        else:
            client_ctx.unsubscribe_signal(event)
    except KeyError as exc:
        return {"status": "not_subscribed", "reason": "Not subscribed to this event/subject couple"}

    # Cannot unsubscribe the backend event given it could be in use by another
    # client context...

    return {"status": "ok"}


async def event_listen(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_EVENT_LISTEN_Schema().load_or_abort(req)
    if msg["wait"]:
        event, subject = await client_ctx.received_signals.get()
    else:
        try:
            event, subject = client_ctx.received_signals.get_nowait()
        except trio.WouldBlock:
            return {"status": "ok"}

    # TODO: make more generic
    if event == "device_try_claim_submitted":
        try:
            rep = await core.backend_connection.send(
                {"cmd": "device_get_configuration_try", "configuration_try_id": subject}
            )
        except BackendNotAvailable:
            return {"status": "backend_not_available", "reason": "Backend not available"}

        _, errors = backend_get_configuration_try_schema.load(rep)
        if errors:
            return {
                "status": "backend_error",
                "reason": "Bad response from backend: %r (%r)" % (rep, errors),
            }
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
    return {"status": "ok", "subscribed": list(client_ctx.registered_signals.keys())}

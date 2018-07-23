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
    exchange_cipherkey = fields.Base64Bytes(required=True)
    salt = fields.Base64Bytes(required=True)


backend_get_configuration_try_schema = BackendGetConfigurationTrySchema()


class _cmd_EVENT_LISTEN_Schema(BaseCmdSchema):
    wait = fields.Boolean(missing=True)


cmd_EVENT_LISTEN_Schema = _cmd_EVENT_LISTEN_Schema()


class _cmd_EVENT_SUBSCRIBE_Schema(BaseCmdSchema):
    event = fields.String(
        required=True,
        validate=validate.OneOf(
            ("pinged", "fuse_mountpoint_need_stop", "new_sharing", "device_try_claim_submitted")
        ),
    )
    subject = fields.String(missing=None)


cmd_EVENT_SUBSCRIBE_Schema = _cmd_EVENT_SUBSCRIBE_Schema()


cmd_EVENT_LIST_SUBSCRIBED = BaseCmdSchema()


async def event_subscribe(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    # TODO: change this api along with front
    msg = cmd_EVENT_SUBSCRIBE_Schema.load(req)

    try:
        client_ctx.subscribe_signal(msg["event"], msg["subject"])
    except KeyError as exc:
        return {
            "status": "already_subscribed",
            "reason": "Already subscribed to this event/subject couple",
        }

    return {"status": "ok"}


async def event_unsubscribe(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_EVENT_SUBSCRIBE_Schema.load(req)

    try:
        client_ctx.unsubscribe_signal(msg["event"], msg["subject"])
    except KeyError as exc:
        return {"status": "not_subscribed", "reason": "Not subscribed to this event/subject couple"}

    # Cannot unsubscribe the backend event given it could be in use by another
    # client context...

    return {"status": "ok"}


async def event_listen(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_EVENT_LISTEN_Schema.load(req)
    if msg["wait"]:
        event_msg = await client_ctx.received_signals.get()
    else:
        try:
            event_msg = client_ctx.received_signals.get_nowait()
        except trio.WouldBlock:
            return {"status": "ok"}

    # TODO: make more generic
    if event_msg["event"] == "device_try_claim_submitted":
        config_try_id = event_msg["config_try_id"]
        try:
            rep = await core.backend_cmds_sender.send(
                {"cmd": "device_get_configuration_try", "config_try_id": config_try_id}
            )
        except BackendNotAvailable:
            return {"status": "backend_not_available", "reason": "Backend not available"}

        _, errors = backend_get_configuration_try_schema.load(rep)
        if errors:
            return {
                "status": "backend_error",
                "reason": "Bad response from backend: %r (%r)" % (rep, errors),
            }

    return {"status": "ok", **event_msg}


async def event_list_subscribed(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    cmd_EVENT_LIST_SUBSCRIBED.load(req)  # empty msg expected
    return {"status": "ok", "subscribed": list(client_ctx.registered_signals.keys())}

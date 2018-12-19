import base64
import json
import attr
import trio
import inspect
from functools import wraps
from uuid import UUID
from pendulum import Pendulum, parse as pendulum_parse


def to_jsonb64(raw: bytes):
    return base64.encodebytes(raw).decode("utf8").replace("\\n", "")


def from_jsonb64(msg: str):
    return base64.decodebytes(msg.encode("utf8"))


def _json_serial(obj):
    """JSON serializer for objects not serializable by default json code"""
    if isinstance(obj, Pendulum):
        return {"__type__": "datetime", "__value__": obj.isoformat()}

    if isinstance(obj, UUID):
        return {"__type__": "uuid", "__value__": obj.hex}

    elif isinstance(obj, bytes):
        return {"__type__": "bytes", "__value__": to_jsonb64(obj)}

    raise TypeError("Type %s not serializable" % type(obj))


def ejson_dumps(obj):
    return json.dumps(obj, default=_json_serial, sort_keys=True)


def ejson_loads(raw):
    def _recursive_load_special_types(data):
        if isinstance(data, list):
            return [_recursive_load_special_types(x) for x in data]
        elif isinstance(data, dict):
            try:
                type = data["__type__"]
                value = data["__value__"]
                if type == "bytes":
                    return from_jsonb64(value)
                elif type == "uuid":
                    return UUID(value)
                elif type == "datetime":
                    return pendulum_parse(value)

            except KeyError:
                pass

            return {k: _recursive_load_special_types(v) for k, v in data.items()}

        else:
            return data

    return _recursive_load_special_types(json.loads(raw))


def _sync_wrap_method(method):
    if inspect.iscoroutinefunction(method):

        @wraps(method)
        async def wrapper(self, *args, **kwargs):
            return await self._trio_portal.run(method, self, *args, **kwargs)

    else:
        return method


class BaseSync:
    def __init__(self, portal, internal):
        self._trio_portal = portal
        self._internal = internal


def sync_wrap(cls, methods):
    nmspc = {mname: _sync_wrap_method(getattr(cls, mname)) for mname in methods}
    sync_cls = type(f"Sync{cls.__name__}", (BaseSync,), nmspc)
    return sync_cls


@attr.s
class CallController:
    need_stop = attr.ib(factory=trio.Event)
    stopped = attr.ib(factory=trio.Event)
    obj = attr.ib(default=None)

    async def stop(self):
        self.need_stop.set()
        await self.stopped.wait()


async def call_with_control(controlled_fn, *, task_status=trio.TASK_STATUS_IGNORED):
    controller = CallController()

    async def _started_cb(obj=None):
        controller.obj = obj
        task_status.started(controller)
        await controller.need_stop.wait()

    try:
        await controlled_fn(_started_cb)

    finally:
        controller.stopped.set()

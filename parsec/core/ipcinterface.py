import platform
import trio
from functools import partial
from contextlib import contextmanager
from async_generator import asynccontextmanager
from structlog import get_logger
from pathlib import Path

from parsec.serde import (
    BaseSchema,
    OneOfSchema,
    fields,
    packb,
    Unpacker,
    SerdeError,
    MsgpackSerializer,
)
from parsec.core.types import (
    BackendOrganizationBootstrapAddr,
    BackendOrganizationClaimUserAddr,
    BackendOrganizationClaimDeviceAddr,
)


logger = get_logger()


class IPCServerError(Exception):
    pass


class IPCServerBadResponse(IPCServerError):
    def __init__(self, rep):
        self.rep = rep

    def __repr__(self):
        return f"Bad response from IPC server: {self.rep}"


class IPCServerNotRunning(IPCServerError):
    pass


class IPCServerAlreadyRunning(IPCServerError):
    pass


class ForegroundReqSchema(BaseSchema):
    cmd = fields.CheckedConstant("foreground", required=True)


def _parse_new_instance_url(raw):
    for type in (
        BackendOrganizationBootstrapAddr,
        BackendOrganizationClaimUserAddr,
        BackendOrganizationClaimDeviceAddr,
    ):
        try:
            return type(raw)
        except ValueError:
            pass
    raise ValueError("Invalid URL format")


class NewInstanceReqSchema(BaseSchema):
    cmd = fields.CheckedConstant("new_instance", required=True)
    url = fields.str_based_field_factory(_parse_new_instance_url)(allow_none=True)


class CommandReqSchema(OneOfSchema):
    type_field = "cmd"
    type_field_remove = False
    type_schemas = {"foreground": ForegroundReqSchema, "new_instance": NewInstanceReqSchema}

    def get_obj_type(self, obj):
        return obj["cmd"]


class CommandRepSchema(BaseSchema):
    status = fields.String(required=True)
    reason = fields.String(allow_none=True)


cmd_req_serializer = MsgpackSerializer(CommandReqSchema)
cmd_rep_serializer = MsgpackSerializer(CommandRepSchema)


DEFAULT_WIN32_MUTEX_NAME = "parsec-cloud"


@contextmanager
def _install_win32_mutex(mutex_name: str):

    import win32event
    import win32api
    import winerror

    mutex = win32event.CreateMutex(None, False, mutex_name)
    status = win32api.GetLastError()
    if status == winerror.ERROR_ALREADY_EXISTS:
        raise IPCServerAlreadyRunning(f"Mutex `{mutex_name}` already exists")
    elif not winerror.SUCCEEDED(status):
        raise IPCServerError(f"Cannot create mutex `{mutex_name}`: error {status}")

    try:
        yield

    finally:
        win32api.CloseHandle(mutex)


@contextmanager
def _install_posix_file_lock(socket_file: Path):

    import fcntl

    try:
        with open(socket_file, "a") as fd:
            fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
            yield
            # Lock is released on file descriptor closing
    except OSError as exc:
        raise IPCServerAlreadyRunning(f"Cannot lock file `{socket_file}`: {exc}") from exc


@asynccontextmanager
async def run_ipc_server(
    cmd_handler, socket_file: Path, win32_mutex_name: str = DEFAULT_WIN32_MUTEX_NAME
):
    if platform.system() == "Windows":
        with _install_win32_mutex(win32_mutex_name):
            async with _run_tcp_server(socket_file, cmd_handler):
                yield
    else:
        with _install_posix_file_lock(socket_file):
            async with _run_tcp_server(socket_file, cmd_handler):
                yield


@asynccontextmanager
async def _run_tcp_server(socket_file: Path, cmd_handler):
    async def _client_handler(stream):
        unpacker = Unpacker()

        try:

            while True:
                raw = await stream.receive_some(1000)
                unpacker.feed(raw)
                for cmd in unpacker:
                    cmd = cmd_req_serializer.load(cmd)
                    rep = await cmd_handler(cmd)
                    logger.info("Command processed", cmd=cmd, rep=rep)
                    raw_rep = cmd_rep_serializer.dumps(rep)
                    await stream.send_all(raw_rep)

        except trio.BrokenResourceError:
            # Peer has closed the connection
            pass

        except SerdeError as exc:
            try:
                await stream.send_all(packb({"status": "invalid_format", "reason": str(exc)}))
                await stream.aclose()
            except trio.BrokenResourceError:
                pass

        except Exception as exc:
            try:
                await stream.aclose()
            except trio.BrokenResourceError:
                pass
            logger.error("Unexpected crash", exc_info=exc)

    try:
        async with trio.open_nursery() as nursery:
            listeners = await nursery.start(
                partial(trio.serve_tcp, _client_handler, 0, host="127.0.0.1")
            )
            port = listeners[0].socket.getsockname()[1]
            socket_file.write_text(str(port))

            logger.info("IPC server ready", port=port)
            try:
                yield
            finally:
                nursery.cancel_scope.cancel()

    except OSError as exc:
        raise IPCServerError(f"Cannot start IPC server: {exc}") from exc


async def send_to_ipc_server(socket_file: Path, cmd, **kwargs):
    try:
        socket_port = int(socket_file.read_text().strip())

    except ValueError as exc:
        raise IPCServerNotRunning("Invalid IPC socket file") from exc

    try:
        stream = await trio.open_tcp_stream("127.0.0.1", socket_port)

        raw_req = cmd_req_serializer.dumps({"cmd": cmd, **kwargs})
        await stream.send_all(raw_req)
        unpacker = Unpacker(exc_cls=IPCServerError)
        while True:
            raw = await stream.receive_some(1000)
            if not raw:
                raise IPCServerError(f"IPC server has closed the connection unexpectly")
            unpacker.feed(raw)
            raw_rep = next(unpacker, None)
            rep = cmd_rep_serializer.load(raw_rep)
            if rep:
                if rep["status"] != "ok":
                    raise IPCServerBadResponse(rep)
                return rep

    except SerdeError as exc:
        raise IPCServerError(f"Invalid message format: {exc}") from exc

    except (OSError, trio.BrokenResourceError) as exc:
        raise IPCServerNotRunning(f"Impossible to connect to IPC server: {exc}") from exc

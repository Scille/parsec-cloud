import os
import trio
from trio import socket as tsocket
from trio._highlevel_open_tcp_listeners import _compute_backlog


# Should be implemented in trio 0.2.0 or 0.3.0...
# (see https://github.com/python-trio/trio/issues/279)


async def serve_unix(
    handler,
    path,
    *,
    mode=0o666,
    backlog=None,
    handler_nursery=None,
    task_status=trio.TASK_STATUS_IGNORED
):
    listeners = await open_unix_listeners(path, mode=mode, backlog=backlog)
    await trio.serve_listeners(
        handler, listeners, handler_nursery=handler_nursery, task_status=task_status
    )


async def open_unix_listeners(path, *, mode=0o666, backlog=None):
    try:
        os.unlink(path)
    except FileNotFoundError:
        pass
    backlog = _compute_backlog(backlog)
    listeners = []
    sock = tsocket.socket(trio.socket.AF_UNIX, tsocket.SOCK_STREAM, 0)
    try:
        sock.bind(path)
        os.chmod(path, mode)
        sock.listen(backlog)
    except:
        sock.close()
        raise

    listeners.append(trio.SocketListener(sock))
    return listeners


async def open_unix_stream(path):
    sock = tsocket.socket(trio.socket.AF_UNIX, tsocket.SOCK_STREAM, 0)
    try:
        sock.connect(path)
    except:
        sock.close()
        raise

    return trio.SocketStream(sock)


def monkey_patch():
    # Monkeeeeeey patch !
    trio.serve_unix = serve_unix
    trio.open_unix_listeners = open_unix_listeners
    trio.open_unix_stream = open_unix_stream
    trio.__all__ += ["serve_unix", "open_unix_listeners", "open_unix_stream"]

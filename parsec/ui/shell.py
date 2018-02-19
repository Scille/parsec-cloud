import trio
import click
from urllib.parse import urlparse

from parsec.core.cli import DEFAULT_CORE_SOCKET
from parsec.utils import CookedSocket


class ReloadShell(Exception):
    pass


async def repl(socket_path):
    try:
        if socket_path.startswith('unix://'):
            sockstream = await trio.open_unix_stream(socket_path[len('unix://'):])
        elif socket_path.startswith('tcp://'):
            parsed = urlparse(socket_path)
            sockstream = await trio.open_tcp_stream(port=parsed.port, host=parsed.hostname)
    except OSError as exc:
        raise SystemExit(exc)
    sock = CookedSocket(sockstream)
    try:

        quit = False
        while not quit:
            data = input('>>> ')
            if not data:
                continue
            elif data in ('quit', 'q'):
                return
            elif data in ('help', 'h'):
                print('No help for the braves !')
                continue
            elif data in ('reload', 'r'):
                raise ReloadShell()
                continue
            try:
                await sock.sockstream.send_all(data.encode() + b'\n')
                rep = await sock.recv()
            except trio.BrokenStreamError as exc:
                raise SystemExit(exc)
            print('Received: %r' % rep)

    finally:
        await sock.aclose()


@click.command()
@click.option('--socket', '-s', default=DEFAULT_CORE_SOCKET,
              help='Core socket to connect to (default: %s).' %
              DEFAULT_CORE_SOCKET)
def cli(socket):
    try:
        import readline  # For Linux
    except ModuleNotFoundError:
        pass
    from parsec import __version__

    print('Parsec shell version: %s' % __version__)
    print('Connecting to: %s' % socket)
    while True:
        try:
            trio.run(repl, socket)
        except ReloadShell:
            pass
        else:
            break

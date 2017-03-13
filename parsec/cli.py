from socket import socket, AF_UNIX, SOCK_STREAM
import click

from parsec.server import UnixSocketServer, WebSocketServer
from parsec.backend import MessageService, VlobService
from parsec.ui.shell import start_shell
from parsec.ui.fuse import start_fuse


CORE_UNIX_SOCKET = '/tmp/parsec'


@click.group()
def cli():
    pass


@click.command()
@click.argument('id')
@click.argument('args', nargs=-1)
def cmd(id, args):
    sock = socket(AF_UNIX, SOCK_STREAM)
    sock.connect(CORE_UNIX_SOCKET)
    try:
        msg = '%s %s' % (id, args)
        sock.send(msg.encode())
        resp = sock.recv(4096)
        print(resp)
    finally:
        sock.close()


@click.command()
@click.option('--socket', '-s', default=CORE_UNIX_SOCKET,
              help='Path to the UNIX socket (default: %s).' % CORE_UNIX_SOCKET)
def shell(socket):
    start_shell(socket)


@click.command()
@click.argument('mountpoint', type=click.Path(exists=True, file_okay=False))
@click.option('--debug', '-d', is_flag=True, default=False)
@click.option('--nothreads', is_flag=True, default=False)
@click.option('--socket', '-s', default=CORE_UNIX_SOCKET,
              help='Path to the UNIX socket (default: %s).' % CORE_UNIX_SOCKET)
def fuse(mountpoint, debug, nothreads, socket):
    start_fuse(socket, mountpoint, debug=debug, nothreads=nothreads)


@click.command()
@click.option('--socket', '-s', default=CORE_UNIX_SOCKET,
              help='Path to the UNIX socket exposing the core API (default: %s).' %
              CORE_UNIX_SOCKET)
@click.option('--backend-host', '-H', default='localhost')
@click.option('--backend-port', '-P', default=6777, type=int)
def core(socket, backend_host, backend_port):
    server = UnixSocketServer()
    # server.register_service()
    server.start(socket)


@click.command()
@click.option('--host', '-H', default='localhost')
@click.option('--port', '-P', default=6777, type=int)
def backend(host, port):
    server = WebSocketServer()
    server.register_service(MessageService())
    server.register_service(VlobService())
    server.start(host, port)


cli.add_command(cmd)
cli.add_command(fuse)
cli.add_command(shell)
cli.add_command(core)
cli.add_command(backend)


if __name__ == '__main__':
    cli()

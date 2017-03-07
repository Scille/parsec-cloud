from sys import argv
from socket import socket, AF_UNIX, SOCK_STREAM
import click

from parsec.server import start_server
from parsec.ui.shell import start_shell
from parsec.ui.fuse import start_fuse


SOCKET_PATH = '/tmp/parsec'


@click.group()
def cli():
    pass


@click.command()
@click.argument('id')
@click.argument('args', nargs=-1)
def cmd(id, args):
    sock = socket(AF_UNIX, SOCK_STREAM)
    sock.connect(SOCKET_PATH)
    try:
        msg = '%s %s' % (id, args)
        sock.send(msg.encode())
        resp = sock.recv(4096)
        print(resp)
    finally:
        sock.close()


@click.command()
@click.option('--socket', '-s', default=SOCKET_PATH,
              help='Path to the UNIX socket (default: %s).' % SOCKET_PATH)
def shell(socket):
    start_shell(socket)


@click.command()
@click.argument('mountpoint', type=click.Path(exists=True, file_okay=False))
@click.option('--debug', '-d', is_flag=True, default=False)
@click.option('--nothreads', is_flag=True, default=False)
@click.option('--socket', '-s', default=SOCKET_PATH,
              help='Path to the UNIX socket (default: %s).' % SOCKET_PATH)
def fuse(mountpoint, debug, nothreads, socket):
    start_fuse(socket, mountpoint, debug=debug, nothreads=nothreads)


@click.command()
@click.option('--socket', '-s', default=SOCKET_PATH,
              help='Path to the UNIX socket (default: %s).' % SOCKET_PATH)
def server(socket):
    start_server(socket)


cli.add_command(cmd)
cli.add_command(fuse)
cli.add_command(shell)
cli.add_command(server)


if __name__ == '__main__':
    cli()

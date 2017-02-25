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
def shell():
    start_shell(SOCKET_PATH)


@click.command()
@click.argument('mountpoint', type=click.Path(exists=True, file_okay=False))
@click.option('--debug', '-d', is_flag=True, default=False)
@click.option('--nothreads', is_flag=True, default=False)
def fuse(mountpoint, debug, nothreads):
    start_fuse(SOCKET_PATH, mountpoint, debug=debug, nothreads=nothreads)


@click.command()
def server():
    start_server(SOCKET_PATH)


cli.add_command(cmd)
cli.add_command(fuse)
cli.add_command(shell)
cli.add_command(server)


if __name__ == '__main__':
    cli()

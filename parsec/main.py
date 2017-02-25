from sys import argv
from socket import socket, AF_UNIX, SOCK_STREAM
import click

from parsec.server import start_server
from parsec.shell import start_shell


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
def server():
    start_server(SOCKET_PATH)


cli.add_command(cmd)
cli.add_command(shell)
cli.add_command(server)


if __name__ == '__main__':
    cli()

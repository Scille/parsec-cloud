import asyncio
import json
from functools import partial
import readline  # noqa: side effect powaaa !


async def repl(socket_path):
    from parsec import __version__
    print('Parsec shell version: %s' % __version__)
    print('Connecting to: %s' % socket_path)
    open_conn = partial(asyncio.open_unix_connection, path=socket_path)
    reader, writer = await open_conn()
    quit = False
    while not quit:
        data = input('>>> ')
        if data in ('quit', 'q'):
            writer.close()
            return
        elif data in ('help', 'h'):
            print('No help for the braves !')
            continue
        elif data in ('reload', 'r'):
            writer.close()
            reader, writer = await open_conn()
            continue
        writer.write(data.encode())
        writer.write(b'\n')
        raw_resp = await reader.readline()
        resp = json.loads(raw_resp.decode())
        print('Received: %r' % resp)


def start_shell(socket_path):
    loop = asyncio.get_event_loop()
    loop.run_until_complete(repl(socket_path))


if __name__ == '__main__':
    start_shell()

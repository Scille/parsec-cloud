import asyncio
import json
import readline  # noqa: side effect powaaa !


async def repl(socket_path):
    reader, writer = await asyncio.open_unix_connection(path=socket_path)
    quit = False
    while not quit:
        data = input(">>> ")
        if data in ('quit', 'q'):
            writer.close()
            return
        elif data in ('help', 'h'):
            print('No help for the braves !')
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

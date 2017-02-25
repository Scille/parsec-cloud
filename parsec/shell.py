import asyncio
import readline  # noqa: side effect powaaa !


async def repl(socket_path):
    reader, writer = await asyncio.open_unix_connection(path=socket_path)
    quit = False
    while not quit:
        cmd = input(">>> ")
        if cmd in ('quit', 'q'):
            writer.close()
            return
        writer.write(cmd.encode())
        writer.write(b'\n')
        resp = await reader.readline()
        print('Received: %s' % resp)


def start_shell(socket_path):
    loop = asyncio.get_event_loop()
    loop.run_until_complete(repl(socket_path))


if __name__ == '__main__':
    start_shell()

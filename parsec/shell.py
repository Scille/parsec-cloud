import asyncio
import readline  # noqa: side effect powaaa !

from parsec.server import Request, Response


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
        args = data.split(' ', maxsplit=1)
        cmdid = args[0]
        req = Request(cmdid, body=(args[1].encode() if len(args) == 2 else b''))
        writer.write(req.pack())
        writer.write(b'\n')
        raw_resp = await reader.readline()
        resp = Response.from_raw(raw_resp[:-1])
        print('Received: %r' % resp)


def start_shell(socket_path):
    loop = asyncio.get_event_loop()
    loop.run_until_complete(repl(socket_path))


if __name__ == '__main__':
    start_shell()

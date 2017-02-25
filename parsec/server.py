import os
import asyncio


async def _connection(reader, writer):
    print('Connected with client')
    while True:
        data = await reader.readline()
        if not data:
            print('End of connection')
            return
        print("Received:", data.decode())
        writer.write('teuebe.\n'.encode())


def start_server(socket_path):
    loop = asyncio.get_event_loop()
    try:
        connect_coro = asyncio.start_unix_server(_connection, path=socket_path, loop=loop)
        loop.create_task(connect_coro)
        loop.run_forever()
    finally:
        loop.close()
        os.remove(socket_path)


if __name__ == '__main__':
    start_server()

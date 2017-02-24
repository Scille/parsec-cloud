import os
import asyncio


class ParsecProtocol(asyncio.Protocol):
    transport = None

    def connection_made(self, transport):
        self.transport = transport

    def data_received(self, data):
        print("Received:", data.decode())
        self.transport.write("teube.".encode())

        # # We are done: close the transport (it will call connection_lost())
        # self.transport.close()

    # def connection_lost(self, exc):
    #     # The socket has been closed, stop the event loop
    #     loop.stop()


def start_server(socket_path):
    loop = asyncio.get_event_loop()
    try:
        connect_coro = loop.create_unix_server(ParsecProtocol, path=socket_path)
        loop.run_until_complete(connect_coro)
        loop.run_forever()
    finally:
        loop.close()
        os.remove(socket_path)


if __name__ == '__main__':
    start_server()

import asyncio
# from threading import Thread


class ParsecShellProtocol(asyncio.Protocol):
    def __init__(self, on_connection_lost, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.transport = None
        self.inbox = []
        self.on_connection_lost = on_connection_lost

    def connection_made(self, transport):
        self.transport = transport

    def data_received(self, data):
        self.inbox.append(data)

    def connection_lost(self, exc):
        self.on_connection_lost()


# def start_shell_threads(socket_path):
#     loop = asyncio.get_event_loop()
#     transport, protocol = loop.run_until_complete(loop.create_unix_connection(ParsecShellProtocol, path=socket_path))

#     io_thread = Thread(target=loop.run_forever)
#     io_thread.start()

#     quit = False
#     while not quit:
#         cmd = input(">>> ")
#         transport
#         if cmd in ('quit', 'q'):
#             loop.call_soon_threadsafe(loop.stop)
#             quit = True
#         loop.call_soon_threadsafe(transport.write, cmd.encode())
#     loop.close()


def start_shell(socket_path):
    loop = asyncio.get_event_loop()

    async def repl(transport, protocol):
        quit = False
        # transport, protocol = await loop.create_unix_connection(ParsecShellProtocol, path=socket_path)
        while not quit:
            cmd = input(">>> ")
            if cmd in ('quit', 'q'):
                loop.stop()
                return
            transport.write(cmd.encode())
            await asyncio.sleep(0.1)
            for msg in protocol.inbox:
                print('Received: %s' % msg)
            protocol.inbox = []

    def on_connection_lost():
        print('Connection lost !')
        loop.call_soon(loop.stop())

    transport, protocol = loop.run_until_complete(loop.create_unix_connection(
        lambda: ParsecShellProtocol(on_connection_lost), path=socket_path))
    loop.create_task(repl(transport, protocol))
    loop.run_forever()
    loop.close()


if __name__ == '__main__':
    start_shell()

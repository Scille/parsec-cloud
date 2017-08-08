import os
import pytest
import asyncio
from unittest.mock import Mock

from parsec.core.server import run_unix_socket_server


async def test_socket_exists(loop):
    async with run_unix_socket_server(on_connection=Mock(), loop=loop) as server:
        socket_path = server.socket_path
        assert os.path.stat.S_ISSOCK(os.stat(socket_path).st_mode)
    assert not os.path.exists(socket_path)


async def test_socket_communication(loop):
    async def on_connection(reader, writer):
        cmd = await reader.readline()
        assert cmd == b'ping\n'
        writer.write(b'pong\n')

    async with run_unix_socket_server(on_connection=on_connection, loop=loop) as server:
        reader, writer = await asyncio.open_unix_connection(server.socket_path, loop=loop)
        try:
            writer.write(b'ping\n')
            resp = await reader.readline()
            assert resp == b'pong\n'
        finally:
            writer.close()


async def test_big_socket_communication(loop):
    big_payload = b'X' * int(10e6)  # 10mo big payload

    async def big_read(reader):
        buff = b''
        while True:
            buff += await reader.read(65536)
            if buff[-1] == ord(b'\n'):
                break
        return buff

    async def on_connection(reader, writer):
        cmd = await big_read(reader)
        writer.write(cmd)

    async with run_unix_socket_server(on_connection=on_connection, loop=loop) as server:
        reader, writer = await asyncio.open_unix_connection(server.socket_path, loop=loop)
        try:
            writer.write(big_payload)
            writer.write(b'\n')
            resp = await big_read(reader)
            assert resp[:-1] == big_payload
        finally:
            writer.close()


# async def test_notification():
#     async def on_connection(reader, writer):
#         cmd = await reader.readline()
#         assert cmd == b'ping\n'
#         writer.write(b'pong\n')

#     async with run_unix_socket_server(on_connection=on_connection) as server:
#         reader, writer = await asyncio.open_unix_connection(server.socket_path)
#         try:
#             writer.write(b'ping\n')
#             resp = await reader.readline()
#             assert resp == b'pong\n'
#         finally:
#             writer.close()


#     server, socket_path = unix_socket_server
#     reader, writer = await asyncio.open_unix_connection(socket_path)
#     try:
#         # Subscribe event
#         writer.write(b'{"cmd": "subscribe", "event": "on_ping", "sender": "hello"}\n')
#         resp = await reader.readline()
#         assert json.loads(resp[:-1].decode()) == {"status": "ok"}
#         # Trigger event
#         writer.write(b'{"cmd": "ping", "ping": "hello"}\n')
#         resp = await reader.readline()
#         assert json.loads(resp[:-1].decode()) == {"status": "ok", "pong": "hello"}
#         # Should receive a notification
#         notif = await reader.readline()
#         assert json.loads(notif[:-1].decode()) == {'event': 'on_ping', 'sender': 'hello'}
#     finally:
#         writer.close()


# async def test_unsubscribe_notification(unix_socket_server):
#     server, socket_path = unix_socket_server
#     reader, writer = await asyncio.open_unix_connection(socket_path)
#     try:
#         # Subscribe event
#         # Subscribe same event with two different senders
#         async def subscribe(name):
#             writer.write(b'{"cmd": "subscribe", "event": "on_ping", "sender": "%s"}\n' % name.encode())
#             resp = await reader.readline()
#             assert json.loads(resp[:-1].decode()) == {"status": "ok"}
#         await subscribe('alice')
#         await subscribe('bob')

#         # Then unsubscribe bob's event
#         writer.write(b'{"cmd": "unsubscribe", "event": "on_ping", "sender": "bob"}\n')
#         resp = await reader.readline()
#         assert json.loads(resp[:-1].decode()) == {"status": "ok"}

#         # Trigger bob's event
#         writer.write(b'{"cmd": "ping", "ping": "bob"}\n')
#         resp = await reader.readline()
#         assert json.loads(resp[:-1].decode()) == {"status": "ok", "pong": "bob"}

#         # No event should arrive
#         with pytest.raises(asyncio.TimeoutError):
#             reading = reader.readline()
#             resp = await asyncio.wait_for(reading, 0.1)
#         try:
#             await reading
#         except asyncio.CancelledError:
#             pass


#         # Of courses alice's event should still work
#         writer.write(b'{"cmd": "ping", "ping": "alice"}\n')
#         resp = await reader.readline()
#         assert json.loads(resp[:-1].decode()) == {"status": "ok", "pong": "alice"}

#         reading = reader.readline()
#         resp = await asyncio.wait_for(reading, 0.01)
#         assert json.loads(resp[:-1].decode()) == {'event': 'on_ping', 'sender': 'alice'}

#     finally:
#         writer.close()


# async def test_multi_sender_notification(unix_socket_server):
#     server, socket_path = unix_socket_server
#     reader, writer = await asyncio.open_unix_connection(socket_path)
#     try:
#         # Subscribe same event with two different senders
#         async def subscribe(name):
#             writer.write(b'{"cmd": "subscribe", "event": "on_ping", "sender": "%s"}\n' % name.encode())
#             resp = await reader.readline()
#             assert json.loads(resp[:-1].decode()) == {"status": "ok"}
#         await subscribe('alice')
#         await subscribe('bob')

#         # Trigger event
#         async def do_ping(name):
#             writer.write(b'{"cmd": "ping", "ping": "%s"}\n' % name.encode())
#             resp = await reader.readline()
#             assert json.loads(resp[:-1].decode()) == {"status": "ok", "pong": name}

#         async def check_event(name):
#             reading = reader.readline()
#             resp = await asyncio.wait_for(reading, 0.01)
#             assert json.loads(resp[:-1].decode()) == {'event': 'on_ping', 'sender': name}

#         async def check_no_event():
#             reading = reader.readline()
#             event = None
#             try:
#                 resp = await asyncio.wait_for(reading, 0.1)
#                 event = json.loads(resp[:-1].decode())
#             except asyncio.TimeoutError:
#                 try:
#                     await reading
#                 except asyncio.CancelledError:
#                     pass
#             assert not event

#         await do_ping('bob')
#         await check_event('bob')
#         await do_ping('john')  # should not receive this one
#         await check_no_event()
#         await do_ping('alice')
#         await check_event('alice')
#     finally:
#         writer.close()


# async def test_base_cmds(unix_socket_server):
#     server, socket_path = unix_socket_server
#     reader, writer = await asyncio.open_unix_connection(socket_path)
#     try:
#         writer.write(b'{"cmd": "list_cmds"}\n')
#         resp = await reader.readline()
#         assert json.loads(resp[:-1].decode()) == {
#             "status": "ok", "cmds": ['list_cmds', 'list_events', 'ping', 'subscribe', 'unsubscribe']}

#         writer.write(b'{"cmd": "list_events"}\n')
#         resp = await reader.readline()
#         assert json.loads(resp[:-1].decode()) == {"status": "ok", "events": ['on_ping']}
#     finally:
#         writer.close()

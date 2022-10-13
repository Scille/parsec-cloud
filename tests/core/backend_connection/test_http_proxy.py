# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import re
import pytest
import os
import trio
from functools import partial

from parsec.api.protocol import DeviceID
from parsec.crypto import SigningKey
from parsec.core.types import BackendOrganizationAddr, BackendInvitationAddr
from parsec.core.backend_connection.transport import (
    connect_as_invited,
    connect_as_authenticated,
    BackendNotAvailable,
    http_request,
)

from tests.common import real_clock_timeout


async def start_proxy_for_websocket(nursery, target_port, event_hook):
    async def _proxy_client_handler(stream):
        buff = b""

        async def _next_req():
            nonlocal buff
            while True:
                try:
                    msg, buff = buff.split(b"\r\n\r\n", 1)
                    return msg + b"\r\n\r\n"
                except ValueError:
                    buff = await stream.receive_some(1024)

        # To simplify things, we consider a the http requests/responses are
        # contained in single tcp trame. This is not strictly true in real life
        # but is close enough when staying on localhost
        # 1) Receive proxy connection request
        req = await _next_req()
        match = re.match(
            (
                rb"^CONNECT 127.0.0.1:([0-9]+) HTTP/1.1\r\n"
                rb"Host: 127.0.0.1:([0-9]+)\r\n"
                rb"User-Agent: parsec/[^\r]+\r\n"
                rb"\r\n$"
            ),
            req,
        )
        assert match
        t1, t2 = match.groups()
        assert t1 == t2 == str(target_port).encode()
        await stream.send_all(b"HTTP/1.1 200 OK\r\n\r\n")
        event_hook("Connected to proxy")

        # 2) Proxy is connected, receive actual target request
        req = await _next_req()
        assert re.match(
            (
                rb"^GET /ws HTTP/1.1\r\n"
                rb"Host: 127.0.0.1\r\n"
                rb"Upgrade: WebSocket\r\n"
                rb"Connection: Upgrade\r\n"
                rb"Sec-WebSocket-Key: [^\r]+\r\n"
                rb"Sec-WebSocket-Version: 13\r\n"
                rb"User-Agent: parsec/[^\r]+\r\n"
                rb"\r\n$"
            ),
            req,
        )
        # 3) Here we should in theory connect to the target, but we already
        # saw what we wanted so no need to go further ;-)
        event_hook("Reaching target through proxy")
        # Peer is going to close the connection when receiving this status
        # code, hence any cod after that won't be executed consistently
        await stream.send_all(b"HTTP/1.1 503 OK\r\n\r\n")
        await trio.sleep_forever()  # Let peer close the connection

    proxy_listeners = await nursery.start(
        partial(trio.serve_tcp, _proxy_client_handler, 0, host="127.0.0.1")
    )
    proxy_port = proxy_listeners[0].socket.getsockname()[1]

    return proxy_port


async def start_pac_server(nursery, pac_rule, event_hook):
    async def _pac_client_handler(stream):
        pac = f"""function FindProxyForURL(url, host) {{ return "{pac_rule}"; }}""".encode()
        event_hook("PAC file retreived from server")

        await stream.send_all(
            (
                b"HTTP/1.1 200 OK\r\n"
                b"Content-Type: application/x-ns-proxy-autoconfig\r\n"
                b"Content-Length: %s\r\n"
                b"\r\n"
                b"%s"
            )
            % (str(len(pac)).encode(), pac)
        )
        await trio.sleep_forever()  # Let peer close the connection

    pac_listeners = await nursery.start(
        partial(trio.serve_tcp, _pac_client_handler, 0, host="127.0.0.1")
    )
    pac_port = pac_listeners[0].socket.getsockname()[1]

    return pac_port


async def start_port_watchdog(nursery, event_hook):
    async def _target_client_handler(steam):
        # The _proxy_client_handler never actually reach the target, so if we
        # end up here it means the proxy configuration has been incorrectly ignored
        event_hook("Target directly reached, proxy config has been ignored !")
        assert False, "proxy config has been ignored !"
        await trio.sleep_forever()  # Let peer close the connection

    target_listeners = await nursery.start(
        partial(trio.serve_tcp, _target_client_handler, 0, host="127.0.0.1")
    )
    target_port = target_listeners[0].socket.getsockname()[1]

    return target_port


@pytest.mark.trio
@pytest.mark.real_tcp
@pytest.mark.parametrize("connection_type", ["authenticated", "invited"])
@pytest.mark.parametrize("proxy_type", ["http_proxy", "http_proxy_pac"])
async def test_proxy_with_websocket(monkeypatch, connection_type, proxy_type):
    signing_key = SigningKey.generate()
    device_id = DeviceID("zack@pc1")
    proxy_events = []

    def _event_hook(event):
        proxy_events.append(event)

    async with trio.open_nursery() as nursery:
        target_port = await start_port_watchdog(nursery, _event_hook)
        proxy_port = await start_proxy_for_websocket(nursery, target_port, _event_hook)

        if proxy_type == "http_proxy":
            proxy_url = f"http://127.0.0.1:{proxy_port}"
            monkeypatch.setitem(os.environ, "http_proxy", proxy_url)
        else:
            assert proxy_type == "http_proxy_pac"
            pac_server_port = await start_pac_server(
                nursery=nursery, pac_rule=f"PROXY 127.0.0.1:{proxy_port}", event_hook=_event_hook
            )
            pac_server_url = f"http://127.0.0.1:{pac_server_port}"
            monkeypatch.setitem(os.environ, "http_proxy_pac", pac_server_url)
            # HTTP_PROXY_PAC has priority over HTTP_PROXY
            monkeypatch.setitem(os.environ, "http_proxy", f"http://127.0.0.1:{target_port}")

        async with real_clock_timeout():
            with pytest.raises(BackendNotAvailable):
                if connection_type == "authenticated":
                    await connect_as_authenticated(
                        addr=BackendOrganizationAddr.from_url(
                            f"parsec://127.0.0.1:{target_port}/CoolOrg?no_ssl=true&rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss"
                        ),
                        device_id=device_id,
                        signing_key=signing_key,
                    )

                else:
                    assert connection_type == "invited"
                    await connect_as_invited(
                        addr=BackendInvitationAddr.from_url(
                            f"parsec://127.0.0.1:{target_port}/CoolOrg?no_ssl=true&action=claim_user&token=3a50b191122b480ebb113b10216ef343"
                        )
                    )

        assert proxy_events == [
            *(["PAC file retreived from server"] if proxy_type == "http_proxy_pac" else []),
            "Connected to proxy",
            "Reaching target through proxy",
        ]

        nursery.cancel_scope.cancel()


@pytest.mark.trio
@pytest.mark.real_tcp
@pytest.mark.parametrize("proxy_type", ["http_proxy", "http_proxy_pac"])
async def test_proxy_with_http(monkeypatch, proxy_type):
    proxy_events = []

    def _event_hook(event):
        proxy_events.append(event)

    async with trio.open_nursery() as nursery:

        async def _proxy_client_handler(stream):
            # To simplify things, we consider a the http requests/responses are
            # contained in single tcp trame. This is not strictly true in real life
            # but is close enough when staying on 127.0.0.1.
            req = await stream.receive_some(1024)
            match = re.match(
                (
                    rb"^POST http://127.0.0.1:([0-9]+)/foo HTTP/1.1\r\n"
                    rb"Accept-Encoding: identity\r\n"
                    rb"Content-Length: 0\r\n"
                    rb"Host: 127.0.0.1:([0-9]+)\r\n"
                    rb"User-Agent: [^\r]+\r\n"
                    rb"Connection: close\r\n"
                    rb"\r\n$"
                ),
                req,
            )
            assert match
            t1, t2 = match.groups()
            assert t1 == t2 == str(target_port).encode()
            await stream.send_all(b"HTTP/1.1 200 OK\r\nContent-Size: 5\r\n\r\nhello")
            _event_hook("Connected to proxy")

        proxy_listeners = await nursery.start(
            partial(trio.serve_tcp, _proxy_client_handler, 0, host="127.0.0.1")
        )
        proxy_port = proxy_listeners[0].socket.getsockname()[1]
        target_port = await start_port_watchdog(nursery, _event_hook)

        if proxy_type == "http_proxy":
            proxy_url = f"http://127.0.0.1:{proxy_port}"
            monkeypatch.setitem(os.environ, "http_proxy", proxy_url)
        else:
            assert proxy_type == "http_proxy_pac"
            pac_server_port = await start_pac_server(
                nursery=nursery, pac_rule=f"PROXY 127.0.0.1:{proxy_port}", event_hook=_event_hook
            )
            pac_server_url = f"http://127.0.0.1:{pac_server_port}"
            monkeypatch.setitem(os.environ, "http_proxy_pac", pac_server_url)
            # HTTP_PROXY_PAC has priority over HTTP_PROXY
            monkeypatch.setitem(os.environ, "http_proxy", f"http://127.0.0.1:{target_port}")

        async with real_clock_timeout():
            rep = await http_request(f"http://127.0.0.1:{target_port}/foo", method="POST")
            assert rep == b"hello"

        assert proxy_events == [
            *(["PAC file retreived from server"] if proxy_type == "http_proxy_pac" else []),
            "Connected to proxy",
        ]

        nursery.cancel_scope.cancel()


# Cheap check just to be sure...
@pytest.mark.trio
@pytest.mark.real_tcp
@pytest.mark.parametrize("type", ["no_config", "no_proxy_from_env", "no_proxy_from_pac"])
async def test_no_proxy_with_http(monkeypatch, type):
    proxy_events = []

    def _event_hook(event):
        proxy_events.append(event)

    async with trio.open_nursery() as nursery:

        if type == "no_config":
            pass
        elif type == "no_proxy_from_env":
            dont_use_proxy_port = await start_port_watchdog(nursery, _event_hook)
            dont_use_proxy_url = f"http://127.0.0.1:{dont_use_proxy_port}"
            monkeypatch.setitem(os.environ, "no_proxy", "*")
            # Should be ignored
            monkeypatch.setitem(os.environ, "http_proxy", dont_use_proxy_url)
            monkeypatch.setitem(os.environ, "https_proxy", dont_use_proxy_url)
        else:
            assert type == "no_proxy_from_pac"
            dont_use_proxy_port = await start_port_watchdog(nursery, _event_hook)
            dont_use_proxy_url = f"http://127.0.0.1:{dont_use_proxy_port}"
            pac_server_port = await start_pac_server(
                nursery=nursery, pac_rule="DIRECT", event_hook=_event_hook
            )
            pac_server_url = f"http://127.0.0.1:{pac_server_port}"
            monkeypatch.setitem(os.environ, "http_proxy_pac", pac_server_url)
            # Should be ignored
            monkeypatch.setitem(os.environ, "http_proxy", dont_use_proxy_url)
            monkeypatch.setitem(os.environ, "https_proxy", dont_use_proxy_url)

        async def _target_client_handler(stream):
            # To simplify things, we consider a the http requests/responses are
            # contained in single tcp trame. This is not strictly true in real life
            # but is close enough when staying on 127.0.0.1.
            req = await stream.receive_some(1024)
            match = re.match(
                (
                    rb"^POST /foo HTTP/1.1\r\n"
                    rb"Accept-Encoding: identity\r\n"
                    rb"Content-Length: 0\r\n"
                    rb"Host: 127.0.0.1:([0-9]+)\r\n"
                    rb"User-Agent: [^\r]+\r\n"
                    rb"Connection: close\r\n"
                    rb"\r\n$"
                ),
                req,
            )
            assert match
            t1 = match.group(1)
            assert t1 == str(target_port).encode()
            await stream.send_all(b"HTTP/1.1 200 OK\r\nContent-Size: 5\r\n\r\nhello")
            _event_hook("Connected to target")

        target_listeners = await nursery.start(
            partial(trio.serve_tcp, _target_client_handler, 0, host="127.0.0.1")
        )
        target_port = target_listeners[0].socket.getsockname()[1]

        async with real_clock_timeout():
            rep = await http_request(f"http://127.0.0.1:{target_port}/foo", method="POST")
            assert rep == b"hello"

        assert proxy_events == [
            *(["PAC file retreived from server"] if type == "no_proxy_from_pac" else []),
            "Connected to target",
        ]

        nursery.cancel_scope.cancel()

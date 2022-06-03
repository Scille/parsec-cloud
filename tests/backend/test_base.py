# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import trio
import struct
import socket
from functools import partial
from trio.testing import open_stream_to_socket_listener

from parsec.api.protocol import packb, unpackb, AuthenticatedClientHandshake
from parsec.api.transport import Transport

from tests.common import real_clock_timeout


@pytest.mark.trio
async def test_connection(alice_ws):
    await alice_ws.send(packb({"cmd": "ping", "ping": "42"}))
    rep = await alice_ws.receive()
    assert unpackb(rep) == {"status": "ok", "pong": "42"}


@pytest.mark.trio
async def test_bad_cmd(alice_ws):
    await alice_ws.send(packb({"cmd": "dummy"}))
    rep = await alice_ws.receive()
    assert unpackb(rep) == {"status": "unknown_command", "reason": "Unknown command"}


@pytest.mark.trio
@pytest.mark.parametrize(
    "close_on",
    [
        "tcp_ready",
        "after_http_request",
        "before_http_request",
        "websocket_ready",
        "handshake_started",
        "handshake_done",
    ],
)
@pytest.mark.parametrize("clean_close", [True, False])
async def test_handle_client_coroutine_destroyed_on_client_left(
    backend, alice, close_on, clean_close, recwarn
):
    # For this test we want to use a real TCP socket (instead of relying on
    # the `tcp_stream_spy` mock fixture) test the backend on

    outcome = None
    outcome_available = trio.Event()

    async def _handle_client_with_captured_outcome(stream):
        nonlocal outcome
        try:
            ret = await backend.handle_client(stream)
        except BaseException as exc:
            outcome = ("exception", exc)
            outcome_available.set()
            raise
        else:
            outcome = ("return", ret)
            outcome_available.set()
            return ret

    async with trio.open_nursery() as nursery:
        try:
            # Start server
            listeners = await nursery.start(
                partial(
                    trio.serve_tcp, _handle_client_with_captured_outcome, port=0, host="127.0.0.1"
                )
            )

            # Client connect to the server
            client_stream = await open_stream_to_socket_listener(listeners[0])

            async def _do_close_client():
                if clean_close:
                    await client_stream.aclose()
                else:
                    # Reset the tcp socket instead of regular clean close
                    # See https://stackoverflow.com/a/54065411
                    l_onoff = 1
                    l_linger = 0
                    client_stream.setsockopt(
                        socket.SOL_SOCKET, socket.SO_LINGER, struct.pack("ii", l_onoff, l_linger)
                    )
                    client_stream.socket.close()

                async with real_clock_timeout():
                    await outcome_available.wait()

            if close_on == "tcp_ready":
                await _do_close_client()

            else:
                if close_on == "before_http_request":
                    # Send the beginning of an http request
                    await client_stream.send_all(b"GET / HTTP/1.1\r\n")
                    await _do_close_client()

                elif close_on in ("after_http_request"):
                    # Send an entire http request
                    await client_stream.send_all(b"GET / HTTP/1.0\r\n\r\n")
                    # Peer will realize connection is closed after having sent
                    # the answer for the previous request
                    await _do_close_client()

                else:
                    # First request doing websocket negotiation
                    hostname = f"127.0.0.1:{listeners[0].socket.getsockname()}"
                    transport = await Transport.init_for_client(client_stream, hostname)

                    if close_on == "websocket_ready":
                        await _do_close_client()
                    else:
                        # Client do the handshake
                        ch = AuthenticatedClientHandshake(
                            alice.organization_id,
                            alice.device_id,
                            alice.signing_key,
                            alice.root_verify_key,
                        )
                        challenge_req = await transport.recv()
                        answer_req = ch.process_challenge_req(challenge_req)

                        if close_on == "handshake_started":
                            await _do_close_client()

                        else:
                            await transport.send(answer_req)
                            result_req = await transport.recv()
                            ch.process_result_req(result_req)

                            assert close_on == "handshake_done"  # Sanity check
                            await _do_close_client()

            # Outcome should aways be the same
            assert outcome == ("return", None)

        finally:
            nursery.cancel_scope.cancel()


# @pytest.mark.trio
# async def test_bad_msg_format(alice_backend_sock):
#     await alice_backend_sock.stream.send_all(b"\x00\x00\x00\x04fooo")
#     rep = await alice_backend_sock.recv()
#     assert unpackb(rep) == {"status": "invalid_msg_format", "reason": "Invalid message format"}

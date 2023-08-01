# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager

import trio

from parsec._parsec import DateTime, InvitationType
from parsec.api.protocol import anonymous_cmds, authenticated_cmds, invited_cmds
from tests.common import BaseRpcApiClient, real_clock_timeout


def craft_http_request(
    target: str, method: str, headers: dict, body: bytes | None, protocol: str = "1.0"
) -> bytes:
    if body is None:
        body = b""
    else:
        assert isinstance(body, bytes)
        headers = {**headers, "content-length": len(body)}

    # Use HTTP 1.0 by default given 1.1 requires Host header
    req = f"{method} {target} HTTP/{protocol}\r\n"
    req += "\r\n".join(f"{key}: {value}" for key, value in headers.items())
    while not req.endswith("\r\n\r\n"):
        req += "\r\n"

    return req.encode("ascii") + body


def parse_http_response(raw: bytes):
    head, _ = raw.split(b"\r\n\r\n", 1)  # Ignore the body part
    status, *headers = head.split(b"\r\n")
    protocol, status_code, status_label = status.split(b" ", 2)
    assert protocol == b"HTTP/1.1"
    cooked_status = (int(status_code.decode("ascii")), status_label.decode("ascii"))
    cooked_headers = {}
    for header in headers:
        key, value = header.split(b": ")
        cooked_headers[key.decode("ascii").lower()] = value.decode("ascii")
    return cooked_status, cooked_headers


async def do_http_request(
    stream: trio.abc.Stream,
    target: str | None = None,
    method: str = "GET",
    req: bytes | None = None,
    headers: dict | None = None,
    body: bytes | None = None,
):
    if req is None:
        assert target is not None
        req = craft_http_request(target, method, headers or {}, body)
    else:
        assert target is None
        assert headers is None
        assert body is None
    await stream.send_all(req)

    # In theory there is no guarantee `stream.receive_some()` outputs
    # an entire HTTP request (it typically depends on the TCP stack and
    # the network).
    # However given we communicate only on the localhost loop, we can
    # cross our fingers really hard and expect the http header part will come
    # as a single trame.
    rep = b""
    while b"\r\n\r\n" not in rep:
        part = await stream.receive_some()
        if not part:
            # Connection closed by peer
            raise trio.BrokenResourceError
        rep += part
    status, rep_headers = parse_http_response(rep)
    rep_content = rep.split(b"\r\n\r\n", 1)[1]
    content_size = int(rep_headers.get("content-length", "0"))
    if content_size:
        while len(rep_content) < content_size:
            rep_content += await stream.receive_some()
        # No need to check for another request beeing put after the
        # body in the buffer given we don't use keep alive
        assert len(rep_content) == content_size
    else:
        # In case the current request is a connection upgrade to websocket, the
        # server is allowed to start sending websocket messages right away that
        # may end up as part of the TCP trame that contained the response
        if b"Connection: Upgrade" not in rep:
            assert rep_content == b""

    return status, rep_headers, rep_content


class CmdSock:
    def __init__(self, cmd_module, parse_args=None, check_rep_by_default=False):
        self.cmd_module = cmd_module
        self.check_rep_by_default = check_rep_by_default
        self.parse_args = parse_args

    async def __call__(self, ws_or_rpc, *args, check_rep=None, **kwargs):
        if self.parse_args:
            kwargs = self.parse_args(*args, **kwargs)
            req = self.cmd_module.Req(**kwargs)
        else:
            req = self.cmd_module.Req(*args, **kwargs)

        if isinstance(ws_or_rpc, BaseRpcApiClient):
            raw_rep = await ws_or_rpc.send(
                req=req.dump(),
            )
            return self.cmd_module.Rep.load(raw_rep)

        else:
            raw_req = req.dump()
            await ws_or_rpc.send(raw_req)
            raw_rep = await ws_or_rpc.receive()
            rep = self.cmd_module.Rep.load(raw_rep)
            check_rep = check_rep if check_rep is not None else self.check_rep_by_default
            if check_rep:
                assert type(rep).__name__ == "RepOk"
            return rep

    class AsyncCallRepBox:
        def __init__(self, do_recv):
            self._do_recv = do_recv
            self.rep_done = False
            self._rep = None

        @property
        def rep(self):
            assert self.rep_done
            return self._rep

        async def do_recv(self):
            assert not self.rep_done
            self.rep_done = True
            self._rep = await self._do_recv()

    @asynccontextmanager
    async def async_call(self, sock, *args, check_rep=None, **kwargs):
        if self.parse_args:
            kwargs = self.parse_args(*args, **kwargs)
            req = self.cmd_module.Req(**kwargs)
        else:
            req = self.cmd_module.Req(*args, **kwargs)

        raw_req = req.dump()
        await sock.send(raw_req)

        check_rep = check_rep if check_rep is not None else self.check_rep_by_default

        async def _do_rep():
            raw_rep = await sock.receive()
            rep = self.cmd_module.Rep.load(raw_rep)
            if check_rep:
                assert type(rep).__name__ == "RepOk"
            return rep

        box = self.AsyncCallRepBox(do_recv=_do_rep)
        yield box

        if not box.rep_done:
            async with real_clock_timeout():
                await box.do_recv()


### Ping ###


authenticated_ping = CmdSock(
    authenticated_cmds.latest.ping,
    parse_args=lambda ping="": {"ping": ping},
    check_rep_by_default=True,
)

invited_ping = CmdSock(
    invited_cmds.latest.ping,
    check_rep_by_default=True,
)


### Organization ###


organization_config = CmdSock(
    authenticated_cmds.latest.organization_config, check_rep_by_default=True
)


organization_stats = CmdSock(
    authenticated_cmds.latest.organization_stats, check_rep_by_default=True
)


organization_bootstrap = CmdSock(
    anonymous_cmds.latest.organization_bootstrap,
    check_rep_by_default=True,
)


### Block ###


block_create = CmdSock(
    authenticated_cmds.latest.block_create,
    check_rep_by_default=True,
)
block_read = CmdSock(
    authenticated_cmds.latest.block_read, parse_args=lambda block_id: {"block_id": block_id}
)


### Realm ###


realm_create = CmdSock(
    authenticated_cmds.latest.realm_create,
)
realm_status = CmdSock(
    authenticated_cmds.latest.realm_status,
)
realm_stats = CmdSock(
    authenticated_cmds.latest.realm_stats, parse_args=lambda realm_id: {"realm_id": realm_id}
)
apiv2v3_realm_get_role_certificates = CmdSock(
    authenticated_cmds.v3.realm_get_role_certificates,
)
realm_update_roles = CmdSock(
    authenticated_cmds.latest.realm_update_roles,
    parse_args=lambda role_certificate, recipient_message=None: {
        "role_certificate": role_certificate,
        "recipient_message": recipient_message,
    },
    check_rep_by_default=True,
)
realm_start_reencryption_maintenance = CmdSock(
    authenticated_cmds.latest.realm_start_reencryption_maintenance,
    check_rep_by_default=True,
)
realm_finish_reencryption_maintenance = CmdSock(
    authenticated_cmds.latest.realm_finish_reencryption_maintenance,
    check_rep_by_default=True,
)


### Vlob ###


vlob_create = CmdSock(
    authenticated_cmds.latest.vlob_create,
    parse_args=lambda realm_id, vlob_id, blob, timestamp=None, encryption_revision=1, sequester_blob=None: {
        "realm_id": realm_id,
        "vlob_id": vlob_id,
        "blob": blob,
        "timestamp": timestamp or DateTime.now(),
        "encryption_revision": encryption_revision,
        "sequester_blob": sequester_blob,
    },
    check_rep_by_default=True,
)
vlob_read = CmdSock(
    authenticated_cmds.latest.vlob_read,
    parse_args=lambda vlob_id, version=None, timestamp=None, encryption_revision=1: {
        "vlob_id": vlob_id,
        "version": version,
        "timestamp": timestamp,
        "encryption_revision": encryption_revision,
    },
)
apiv2v3_vlob_read = CmdSock(
    authenticated_cmds.v3.vlob_read,
    parse_args=lambda vlob_id, version=None, timestamp=None, encryption_revision=1: {
        "vlob_id": vlob_id,
        "version": version,
        "timestamp": timestamp,
        "encryption_revision": encryption_revision,
    },
)
vlob_update = CmdSock(
    authenticated_cmds.latest.vlob_update,
    parse_args=lambda vlob_id, version, blob, timestamp=None, encryption_revision=1, sequester_blob=None: {
        "vlob_id": vlob_id,
        "version": version,
        "blob": blob,
        "encryption_revision": encryption_revision,
        "timestamp": timestamp or DateTime.now(),
        "sequester_blob": sequester_blob,
    },
    check_rep_by_default=True,
)
vlob_list_versions = CmdSock(
    authenticated_cmds.latest.vlob_list_versions,
)
vlob_poll_changes = CmdSock(
    authenticated_cmds.latest.vlob_poll_changes,
)
vlob_maintenance_get_reencryption_batch = CmdSock(
    authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch,
    parse_args=lambda realm_id, encryption_revision, size=100: {
        "realm_id": realm_id,
        "encryption_revision": encryption_revision,
        "size": size,
    },
)
vlob_maintenance_save_reencryption_batch = CmdSock(
    authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch,
    check_rep_by_default=True,
)


### Events ###


_events_listen = CmdSock(authenticated_cmds.latest.events_listen)


@asynccontextmanager
async def events_listen(sock):
    async with _events_listen.async_call(sock, wait=True) as box:
        yield box


apiv2v3_events_subscribe = CmdSock(authenticated_cmds.v3.events_subscribe)
_apiv2v3_events_listen = CmdSock(
    authenticated_cmds.v3.events_listen, parse_args=lambda wait: {"wait": wait}
)


async def apiv2v3_events_listen_nowait(sock):
    return await _apiv2v3_events_listen(sock, wait=False)


async def apiv2v3_events_listen_wait(sock):
    return await _apiv2v3_events_listen(sock, wait=True)


@asynccontextmanager
async def apiv2v3_events_listen(sock):
    async with _apiv2v3_events_listen.async_call(sock, wait=True) as box:
        yield box


### Message ###


message_get = CmdSock(
    authenticated_cmds.latest.message_get, parse_args=lambda offset=0: {"offset": offset}
)


apiv2v3_message_get = CmdSock(
    authenticated_cmds.v3.message_get, parse_args=lambda offset=0: {"offset": offset}
)


### User ###


apiv2v3_user_get = CmdSock(
    authenticated_cmds.v3.user_get, parse_args=lambda user_id: {"user_id": user_id}
)
apiv2v3_human_find = CmdSock(
    authenticated_cmds.v3.human_find,
    parse_args=lambda query=None, omit_revoked=False, omit_non_human=False, page=1, per_page=100: {
        "query": query,
        "omit_revoked": omit_revoked,
        "omit_non_human": omit_non_human,
        "page": page,
        "per_page": per_page,
    },
)
user_create = CmdSock(
    authenticated_cmds.latest.user_create,
    parse_args=lambda user_certificate, device_certificate, redacted_user_certificate, redacted_device_certificate: {
        k: v
        for k, v in {
            "user_certificate": user_certificate,
            "device_certificate": device_certificate,
            "redacted_user_certificate": redacted_user_certificate,
            "redacted_device_certificate": redacted_device_certificate,
        }.items()
        if v is not None
    },
)
user_revoke = CmdSock(
    authenticated_cmds.latest.user_revoke,
)
device_create = CmdSock(
    authenticated_cmds.latest.device_create,
)


### Invite ###


invite_new = CmdSock(
    authenticated_cmds.latest.invite_new,
    parse_args=lambda type, send_email=False, claimer_email=None: {
        "unit": (
            authenticated_cmds.latest.invite_new.UserOrDeviceUser(
                claimer_email=claimer_email, send_email=send_email
            )
            if type == InvitationType.USER
            else authenticated_cmds.latest.invite_new.UserOrDeviceDevice(send_email)
        )
    },
)
invite_list = CmdSock(authenticated_cmds.latest.invite_list)
invite_delete = CmdSock(
    authenticated_cmds.latest.invite_delete,
)
invite_info = CmdSock(invited_cmds.latest.invite_info)
invite_1_claimer_wait_peer = CmdSock(
    invited_cmds.latest.invite_1_claimer_wait_peer,
)
invite_1_greeter_wait_peer = CmdSock(
    authenticated_cmds.latest.invite_1_greeter_wait_peer,
)
invite_2a_claimer_send_hashed_nonce = CmdSock(
    invited_cmds.latest.invite_2a_claimer_send_hashed_nonce,
)
invite_2a_greeter_get_hashed_nonce = CmdSock(
    authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce,
)
invite_2b_greeter_send_nonce = CmdSock(
    authenticated_cmds.latest.invite_2b_greeter_send_nonce,
)
invite_2b_claimer_send_nonce = CmdSock(
    invited_cmds.latest.invite_2b_claimer_send_nonce,
)
invite_3a_greeter_wait_peer_trust = CmdSock(
    authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust,
)
invite_3a_claimer_signify_trust = CmdSock(invited_cmds.latest.invite_3a_claimer_signify_trust)
invite_3b_claimer_wait_peer_trust = CmdSock(invited_cmds.latest.invite_3b_claimer_wait_peer_trust)
invite_3b_greeter_signify_trust = CmdSock(
    authenticated_cmds.latest.invite_3b_greeter_signify_trust,
)
invite_4_greeter_communicate = CmdSock(
    authenticated_cmds.latest.invite_4_greeter_communicate,
)
invite_4_claimer_communicate = CmdSock(
    invited_cmds.latest.invite_4_claimer_communicate,
)


### PKI enrollment ###


pki_enrollment_submit = CmdSock(
    anonymous_cmds.latest.pki_enrollment_submit,
)
pki_enrollment_info = CmdSock(
    anonymous_cmds.latest.pki_enrollment_info,
)
pki_enrollment_list = CmdSock(authenticated_cmds.latest.pki_enrollment_list)
pki_enrollment_reject = CmdSock(
    authenticated_cmds.latest.pki_enrollment_reject,
)
pki_enrollment_accept = CmdSock(
    authenticated_cmds.latest.pki_enrollment_accept,
)

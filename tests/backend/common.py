# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import trio
from typing import Optional, Callable
from functools import partial
from parsec._parsec import DateTime
from contextlib import asynccontextmanager

from parsec.serde import packb
from parsec.api.protocol import (
    ping_serializer,
    organization_stats_serializer,
    organization_config_serializer,
    organization_bootstrap_serializer,
    block_create_serializer,
    block_read_serializer,
    realm_create_serializer,
    realm_status_serializer,
    realm_stats_serializer,
    realm_get_role_certificates_serializer,
    realm_update_roles_serializer,
    realm_start_reencryption_maintenance_serializer,
    realm_finish_reencryption_maintenance_serializer,
    vlob_create_serializer,
    vlob_read_serializer,
    vlob_update_serializer,
    vlob_list_versions_serializer,
    vlob_poll_changes_serializer,
    vlob_maintenance_get_reencryption_batch_serializer,
    vlob_maintenance_save_reencryption_batch_serializer,
    events_subscribe_serializer,
    events_listen_serializer,
    user_get_serializer,
    human_find_serializer,
    user_create_serializer,
    user_revoke_serializer,
    device_create_serializer,
    invite_new_serializer,
    invite_list_serializer,
    invite_delete_serializer,
    invite_info_serializer,
    invite_1_claimer_wait_peer_serializer,
    invite_1_greeter_wait_peer_serializer,
    invite_2a_claimer_send_hashed_nonce_serializer,
    invite_2a_greeter_get_hashed_nonce_serializer,
    invite_2b_greeter_send_nonce_serializer,
    invite_2b_claimer_send_nonce_serializer,
    invite_3a_greeter_wait_peer_trust_serializer,
    invite_3a_claimer_signify_trust_serializer,
    invite_3b_claimer_wait_peer_trust_serializer,
    invite_3b_greeter_signify_trust_serializer,
    invite_4_greeter_communicate_serializer,
    invite_4_claimer_communicate_serializer,
    pki_enrollment_submit_serializer,
    pki_enrollment_info_serializer,
    pki_enrollment_list_serializer,
    pki_enrollment_reject_serializer,
    pki_enrollment_accept_serializer,
)

from tests.common import real_clock_timeout


def craft_http_request(
    target: str, method: str, headers: dict, body: Optional[bytes], protocol: str = "1.0"
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
    protocole, status_code, status_label = status.split(b" ", 2)
    assert protocole == b"HTTP/1.1"
    cooked_status = (int(status_code.decode("ascii")), status_label.decode("ascii"))
    cooked_headers = {}
    for header in headers:
        key, value = header.split(b": ")
        cooked_headers[key.decode("ascii").lower()] = value.decode("ascii")
    return cooked_status, cooked_headers


async def do_http_request(
    stream: trio.abc.Stream,
    target: Optional[str] = None,
    method: str = "GET",
    req: Optional[bytes] = None,
    headers: Optional[dict] = None,
    body: Optional[bytes] = None,
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
    def __init__(self, cmd, serializer, parse_args=lambda self: {}, check_rep_by_default=False):
        self.cmd = cmd
        self.serializer = serializer
        self.parse_args = parse_args
        self.check_rep_by_default = check_rep_by_default

    async def _do_send(self, ws, req_post_processing, args, kwargs):
        req = {"cmd": self.cmd, **self.parse_args(self, *args, **kwargs)}
        if req_post_processing:
            raw_req = packb(req_post_processing(self.serializer.req_dump(req)))
        else:
            raw_req = self.serializer.req_dumps(req)
        await ws.send(raw_req)

    async def _do_recv(self, ws, check_rep):
        raw_rep = await ws.receive()
        rep = self.serializer.rep_loads(raw_rep)

        if check_rep:
            try:
                assert rep["status"] == "ok"
            except:
                assert rep.is_ok()

        return rep

    async def __call__(self, ws, *args, req_post_processing: Callable = None, **kwargs):
        check_rep = kwargs.pop("check_rep", self.check_rep_by_default)
        await self._do_send(ws, req_post_processing, args, kwargs)
        return await self._do_recv(ws, check_rep)

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
    async def async_call(self, sock, *args, req_post_processing: Callable = None, **kwargs):
        check_rep = kwargs.pop("check_rep", self.check_rep_by_default)
        await self._do_send(sock, req_post_processing, args, kwargs)

        box = self.AsyncCallRepBox(do_recv=partial(self._do_recv, sock, check_rep))
        yield box

        if not box.rep_done:
            async with real_clock_timeout():
                await box.do_recv()


### Ping ###


ping = CmdSock(
    "ping",
    ping_serializer,
    parse_args=lambda self, ping="foo": {"ping": ping},
    check_rep_by_default=True,
)


### Organization ###


organization_config = CmdSock(
    "organization_config", organization_config_serializer, check_rep_by_default=True
)


organization_stats = CmdSock(
    "organization_stats", organization_stats_serializer, check_rep_by_default=True
)


organization_bootstrap = CmdSock(
    "organization_bootstrap",
    organization_bootstrap_serializer,
    parse_args=lambda self, bootstrap_token, root_verify_key, user_certificate, device_certificate, redacted_user_certificate, redacted_device_certificate, sequester_authority_certificate=None: {
        "bootstrap_token": bootstrap_token,
        "root_verify_key": root_verify_key,
        "user_certificate": user_certificate,
        "device_certificate": device_certificate,
        "redacted_user_certificate": redacted_user_certificate,
        "redacted_device_certificate": redacted_device_certificate,
        "sequester_authority_certificate": sequester_authority_certificate,
    },
    check_rep_by_default=True,
)


### Block ###


block_create = CmdSock(
    "block_create",
    block_create_serializer,
    parse_args=lambda self, block_id, realm_id, block: {
        "block_id": block_id,
        "realm_id": realm_id,
        "block": block,
    },
    check_rep_by_default=True,
)
block_read = CmdSock(
    "block_read", block_read_serializer, parse_args=lambda self, block_id: {"block_id": block_id}
)


### Realm ###


realm_create = CmdSock(
    "realm_create",
    realm_create_serializer,
    parse_args=lambda self, role_certificate: {"role_certificate": role_certificate},
)
realm_status = CmdSock(
    "realm_status",
    realm_status_serializer,
    parse_args=lambda self, realm_id: {"realm_id": realm_id},
)
realm_stats = CmdSock(
    "realm_stats", realm_stats_serializer, parse_args=lambda self, realm_id: {"realm_id": realm_id}
)
realm_get_role_certificates = CmdSock(
    "realm_get_role_certificates",
    realm_get_role_certificates_serializer,
    parse_args=lambda self, realm_id: {"realm_id": realm_id},
)
realm_update_roles = CmdSock(
    "realm_update_roles",
    realm_update_roles_serializer,
    parse_args=lambda self, role_certificate, recipient_message=None: {
        "role_certificate": role_certificate,
        "recipient_message": recipient_message,
    },
    check_rep_by_default=True,
)
realm_start_reencryption_maintenance = CmdSock(
    "realm_start_reencryption_maintenance",
    realm_start_reencryption_maintenance_serializer,
    parse_args=lambda self, realm_id, encryption_revision, timestamp, per_participant_message: {
        "realm_id": realm_id,
        "encryption_revision": encryption_revision,
        "timestamp": timestamp,
        "per_participant_message": per_participant_message,
    },
    check_rep_by_default=True,
)
realm_finish_reencryption_maintenance = CmdSock(
    "realm_finish_reencryption_maintenance",
    realm_finish_reencryption_maintenance_serializer,
    parse_args=lambda self, realm_id, encryption_revision: {
        "realm_id": realm_id,
        "encryption_revision": encryption_revision,
    },
    check_rep_by_default=True,
)


### Vlob ###


vlob_create = CmdSock(
    "vlob_create",
    vlob_create_serializer,
    parse_args=lambda self, realm_id, vlob_id, blob, timestamp=None, encryption_revision=1, sequester_blob=None: {
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
    "vlob_read",
    vlob_read_serializer,
    parse_args=lambda self, vlob_id, version=None, timestamp=None, encryption_revision=1: {
        "vlob_id": vlob_id,
        "version": version,
        "timestamp": timestamp,
        "encryption_revision": encryption_revision,
    },
)
vlob_update = CmdSock(
    "vlob_update",
    vlob_update_serializer,
    parse_args=lambda self, vlob_id, version, blob, timestamp=None, encryption_revision=1, sequester_blob=None: {
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
    "vlob_list_versions",
    vlob_list_versions_serializer,
    parse_args=lambda self, vlob_id, encryption_revision=1: {
        "vlob_id": vlob_id,
        "encryption_revision": encryption_revision,
    },
)
vlob_poll_changes = CmdSock(
    "vlob_poll_changes",
    vlob_poll_changes_serializer,
    parse_args=lambda self, realm_id, last_checkpoint: {
        "realm_id": realm_id,
        "last_checkpoint": last_checkpoint,
    },
)
vlob_maintenance_get_reencryption_batch = CmdSock(
    "vlob_maintenance_get_reencryption_batch",
    vlob_maintenance_get_reencryption_batch_serializer,
    parse_args=lambda self, realm_id, encryption_revision, size=100: {
        "realm_id": realm_id,
        "encryption_revision": encryption_revision,
        "size": size,
    },
)
vlob_maintenance_save_reencryption_batch = CmdSock(
    "vlob_maintenance_save_reencryption_batch",
    vlob_maintenance_save_reencryption_batch_serializer,
    parse_args=lambda self, realm_id, encryption_revision, batch: {
        "realm_id": realm_id,
        "encryption_revision": encryption_revision,
        "batch": batch,
    },
    check_rep_by_default=True,
)


### Events ###


events_subscribe = CmdSock("events_subscribe", events_subscribe_serializer)

_events_listen = CmdSock(
    "events_listen", events_listen_serializer, parse_args=lambda self, wait: {"wait": wait}
)


async def events_listen_nowait(sock):
    return await _events_listen(sock, wait=False)


async def events_listen_wait(sock):
    return await _events_listen(sock, wait=True)


@asynccontextmanager
async def events_listen(sock):
    async with _events_listen.async_call(sock, wait=True) as box:
        yield box


### User ###


user_get = CmdSock(
    "user_get", user_get_serializer, parse_args=lambda self, user_id: {"user_id": user_id}
)
human_find = CmdSock(
    "human_find",
    human_find_serializer,
    parse_args=lambda self, query=None, omit_revoked=False, omit_non_human=False, page=1, per_page=100: {
        "query": query,
        "omit_revoked": omit_revoked,
        "omit_non_human": omit_non_human,
        "page": page,
        "per_page": per_page,
    },
)
user_create = CmdSock(
    "user_create",
    user_create_serializer,
    parse_args=lambda self, user_certificate, device_certificate, redacted_user_certificate, redacted_device_certificate: {
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
    "user_revoke",
    user_revoke_serializer,
    parse_args=lambda self, revoked_user_certificate: {
        "revoked_user_certificate": revoked_user_certificate
    },
)
device_create = CmdSock(
    "device_create",
    device_create_serializer,
    parse_args=lambda self, device_certificate, redacted_device_certificate: {
        "device_certificate": device_certificate,
        "redacted_device_certificate": redacted_device_certificate,
    },
)


### Invite ###


invite_new = CmdSock(
    "invite_new",
    invite_new_serializer,
    parse_args=lambda self, type, send_email=False, claimer_email=None: {
        "type": type,
        "send_email": send_email,
        "claimer_email": claimer_email,
    },
)
invite_list = CmdSock("invite_list", invite_list_serializer)
invite_delete = CmdSock(
    "invite_delete",
    invite_delete_serializer,
    parse_args=lambda self, token, reason: {"token": token, "reason": reason},
)
invite_info = CmdSock("invite_info", invite_info_serializer)
invite_1_claimer_wait_peer = CmdSock(
    "invite_1_claimer_wait_peer",
    invite_1_claimer_wait_peer_serializer,
    parse_args=lambda self, claimer_public_key: {"claimer_public_key": claimer_public_key},
)
invite_1_greeter_wait_peer = CmdSock(
    "invite_1_greeter_wait_peer",
    invite_1_greeter_wait_peer_serializer,
    parse_args=lambda self, token, greeter_public_key: {
        "token": token,
        "greeter_public_key": greeter_public_key,
    },
)
invite_2a_claimer_send_hashed_nonce = CmdSock(
    "invite_2a_claimer_send_hashed_nonce",
    invite_2a_claimer_send_hashed_nonce_serializer,
    parse_args=lambda self, claimer_hashed_nonce: {"claimer_hashed_nonce": claimer_hashed_nonce},
)
invite_2a_greeter_get_hashed_nonce = CmdSock(
    "invite_2a_greeter_get_hashed_nonce",
    invite_2a_greeter_get_hashed_nonce_serializer,
    parse_args=lambda self, token: {"token": token},
)
invite_2b_greeter_send_nonce = CmdSock(
    "invite_2b_greeter_send_nonce",
    invite_2b_greeter_send_nonce_serializer,
    parse_args=lambda self, token, greeter_nonce: {"token": token, "greeter_nonce": greeter_nonce},
)
invite_2b_claimer_send_nonce = CmdSock(
    "invite_2b_claimer_send_nonce",
    invite_2b_claimer_send_nonce_serializer,
    parse_args=lambda self, claimer_nonce: {"claimer_nonce": claimer_nonce},
)
invite_3a_greeter_wait_peer_trust = CmdSock(
    "invite_3a_greeter_wait_peer_trust",
    invite_3a_greeter_wait_peer_trust_serializer,
    parse_args=lambda self, token: {"token": token},
)
invite_3a_claimer_signify_trust = CmdSock(
    "invite_3a_claimer_signify_trust", invite_3a_claimer_signify_trust_serializer
)
invite_3b_claimer_wait_peer_trust = CmdSock(
    "invite_3b_claimer_wait_peer_trust", invite_3b_claimer_wait_peer_trust_serializer
)
invite_3b_greeter_signify_trust = CmdSock(
    "invite_3b_greeter_signify_trust",
    invite_3b_greeter_signify_trust_serializer,
    parse_args=lambda self, token: {"token": token},
)
invite_4_greeter_communicate = CmdSock(
    "invite_4_greeter_communicate",
    invite_4_greeter_communicate_serializer,
    parse_args=lambda self, token, payload: {"token": token, "payload": payload},
)
invite_4_claimer_communicate = CmdSock(
    "invite_4_claimer_communicate",
    invite_4_claimer_communicate_serializer,
    parse_args=lambda self, payload: {"payload": payload},
)


### PKI enrollment ###


pki_enrollment_submit = CmdSock(
    "pki_enrollment_submit",
    pki_enrollment_submit_serializer,
    parse_args=lambda self, enrollment_id, force, submitter_der_x509_certificate, submitter_der_x509_certificate_email, submit_payload_signature, submit_payload: {
        "enrollment_id": enrollment_id,
        "force": force,
        "submitter_der_x509_certificate": submitter_der_x509_certificate,
        "submitter_der_x509_certificate_email": submitter_der_x509_certificate_email,
        "submit_payload_signature": submit_payload_signature,
        "submit_payload": submit_payload,
    },
)
pki_enrollment_info = CmdSock(
    "pki_enrollment_info",
    pki_enrollment_info_serializer,
    parse_args=lambda self, enrollment_id: {"enrollment_id": enrollment_id},
)
pki_enrollment_list = CmdSock("pki_enrollment_list", pki_enrollment_list_serializer)
pki_enrollment_reject = CmdSock(
    "pki_enrollment_reject",
    pki_enrollment_reject_serializer,
    parse_args=lambda self, enrollment_id: {"enrollment_id": enrollment_id},
)
pki_enrollment_accept = CmdSock(
    "pki_enrollment_accept",
    pki_enrollment_accept_serializer,
    parse_args=lambda self, enrollment_id, accepter_der_x509_certificate, accept_payload_signature, accept_payload, user_certificate, device_certificate, redacted_user_certificate, redacted_device_certificate: {
        "enrollment_id": enrollment_id,
        "accepter_der_x509_certificate": accepter_der_x509_certificate,
        "accept_payload_signature": accept_payload_signature,
        "accept_payload": accept_payload,
        "user_certificate": user_certificate,
        "device_certificate": device_certificate,
        "redacted_user_certificate": redacted_user_certificate,
        "redacted_device_certificate": redacted_device_certificate,
    },
)

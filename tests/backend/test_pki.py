# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from uuid import uuid4
import pendulum
import pytest
from parsec.api.data.pki import PkiEnrollmentRequestInfo, PkiEnrollmentReplyInfo

from parsec.api.protocol.types import DeviceID, DeviceLabel, HumanHandle, UserProfile


from parsec.core.backend_connection.cmds import pki_enrollment_get_requests, pki_enrollment_reply
from parsec.api.protocol.pki import PkiEnrollmentRequest, PkiEnrollmentReply


@pytest.mark.trio
async def test_pki_get_empty_request(backend, alice, bob, alice_backend_sock):
    data = await backend.pki.pki_enrollment_get_requests()
    assert not data
    rep = await pki_enrollment_get_requests(alice_backend_sock.transport)
    assert rep["status"] == "ok"
    assert rep["requests"] == []


@pytest.mark.trio
async def test_pki_send_request_and_reply(
    backend, alice, bob, alice_backend_sock, bob_backend_sock
):
    ref_time = pendulum.now()
    pki_request_info = PkiEnrollmentRequestInfo(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_human_handle=HumanHandle(email="t@t.t", label="t"),
        requested_device_name=DeviceLabel("label"),
    )
    pki_request = PkiEnrollmentRequest(
        der_x509_certificate=b"1234567890ABCDEF",
        signature=b"123",
        requested_human_handle=HumanHandle(email="t@t.t", label="t"),
        pki_request_info=pki_request_info,
    )
    certificate_id = b"certificate_id"
    request_id = uuid4()

    await backend.pki.pki_enrollment_request(certificate_id, request_id, pki_request, False)
    rep = await pki_enrollment_get_requests(alice_backend_sock.transport)

    assert rep["status"] == "ok"
    assert len(rep["requests"]) == 1
    assert rep["requests"][0][0] == certificate_id

    raw = "ali-c_e@d-e_v"
    pki_reply_info = PkiEnrollmentReplyInfo(
        device_id=DeviceID(raw),
        root_verify_key=alice.verify_key,
        device_label=DeviceLabel("device"),
        human_handle=HumanHandle(email="t@t.t", label="t"),
        profile=UserProfile.ADMIN,
    )
    pki_reply = PkiEnrollmentReply(
        der_x509_admin_certificate=b"admin_cert", signature=b"123", pki_reply_info=pki_reply_info
    )
    rep = await pki_enrollment_reply(
        alice_backend_sock.transport,
        certificate_id=certificate_id,
        request_id=request_id,
        reply=pki_reply,
        user_id="Hello",
    )

    assert rep["status"] == "ok"
    assert rep["timestamp"] > ref_time

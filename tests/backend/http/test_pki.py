# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS
from uuid import uuid4
import pendulum
import pytest
from parsec.api.data.pki import PkiEnrollmentRequestInfo
from parsec.api.protocol.pki import (
    pki_enrollment_request_serializer,
    pki_enrollment_get_reply_serializer,
    PkiEnrollmentRequest,
)
from parsec.api.protocol.types import DeviceLabel, HumanHandle
from parsec.crypto import VerifyKey, PublicKey, generate_nonce


async def send_pki_http_post_request(backend_http_send, backend_addr, organization_id, cmd, body):
    headers = {}
    headers.setdefault("content-type", "application/msgpack")
    status, _, body = await backend_http_send(
        target=f"/anonymous/pki/{organization_id}/{cmd}",
        method="POST",
        headers=headers,
        body=body,
        sanity_checks=True,
        addr=backend_addr,
    )
    return status, body


@pytest.mark.trio
async def test_pki_rest_wrong_organisation(alice, backend, backend_http_send, backend_addr):
    for command in ["enrollment_request", "enrollment_get_reply"]:
        organization_id = "not_an_organisation"
        status, data = await send_pki_http_post_request(
            backend_http_send=backend_http_send,
            backend_addr=backend_addr,
            organization_id=organization_id,
            cmd=command,
            body=b"some_data",
        )
        assert status == (404, "Not Found")
        assert data == b""


@pytest.mark.trio
async def test_pki_rest_send_request_and_get_reply(alice, backend, backend_http_send, backend_addr):
    organization_id = alice.organization_id
    ref_timestamp = pendulum.now()
    pki_request_info = PkiEnrollmentRequestInfo(
        verify_key=VerifyKey(generate_nonce(32)),
        public_key=PublicKey(generate_nonce(32)),
        requested_human_handle=HumanHandle(email="t@t.t", label="t"),
        requested_device_name=DeviceLabel("test"),
    )
    pki_request = PkiEnrollmentRequest(
        der_x509_certificate=b"1234567890ABCDEF",
        signature=b"123",
        requested_human_handle=HumanHandle(email="t@t.t", label="t"),
        pki_request_info=pki_request_info,
    )
    certificate_id = b"certificate_id"
    request_id = uuid4()

    data = pki_enrollment_request_serializer.req_dumps(
        {
            "certificate_id": certificate_id,
            "request_id": request_id,
            "request": pki_request,
            "force_flag": False,
        }
    )

    status, body = await send_pki_http_post_request(
        backend_http_send=backend_http_send,
        backend_addr=backend_addr,
        organization_id=organization_id,
        cmd="enrollment_request",
        body=data,
    )
    assert status == (200, "OK")
    rep = pki_enrollment_request_serializer.rep_loads(body)
    assert rep["status"] == "ok"
    request_timestamp = rep["timestamp"]
    assert ref_timestamp < request_timestamp < pendulum.now()

    data = pki_enrollment_get_reply_serializer.req_dumps(
        {"certificate_id": certificate_id, "request_id": request_id}
    )
    status, body = await send_pki_http_post_request(
        backend_http_send=backend_http_send,
        backend_addr=backend_addr,
        organization_id=organization_id,
        cmd="enrollment_get_reply",
        body=data,
    )
    assert status == (200, "OK")
    rep = pki_enrollment_get_reply_serializer.rep_loads(body)
    assert rep["status"] == "pending"
    assert request_timestamp == rep["timestamp"]

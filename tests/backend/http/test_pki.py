# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS
import pendulum
import pytest
from parsec.api.protocol.pki import (
    pki_enrollment_request_req_serializer,
    pki_enrollment_request_rep_serializer,
    PkiRequest,
)
from parsec.api.protocol.types import HumanHandle
from parsec.crypto import VerifyKey, PublicKey, generate_nonce


@pytest.mark.trio
async def test_pki_rest_wrong_organisation(alice, backend, backend_http_send, backend_addr):
    organization_id = "not_an_organisation"

    headers = {}
    headers.setdefault("content-type", "application/msgpack")
    status, _, body = await backend_http_send(
        target=f"/anonymous/pki/{organization_id}/enrollment_request",
        method="POST",
        headers=headers,
        body=b"some_data",
        sanity_checks=True,
        addr=backend_addr,
    )
    assert status == (404, "Not Found")


@pytest.mark.trio
async def test_pki_rest_send_request(alice, backend, backend_http_send, backend_addr):
    organization_id = alice.organization_id
    ref_timestamp = pendulum.now()
    pki_request = PkiRequest(
        der_x509_certificate=b"1234567890ABCDEF",
        verify_key=VerifyKey(generate_nonce(32)),
        public_key=PublicKey(generate_nonce(32)),
        signature=b"123",
        requested_human_handle=HumanHandle(email="t@t.t", label="t"),
        requested_device_name="some_name",
    )
    certificate_id = b"certificate_id"
    request_id = b"request_id"

    data = pki_enrollment_request_req_serializer.dumps(
        {
            "certificate_id": certificate_id,
            "request_id": request_id,
            "request": pki_request,
            "force_flag": False,
        }
    )
    pki_enrollment_request_req_serializer.loads(data)

    headers = {}
    headers.setdefault("content-type", "application/msgpack")
    status, _, body = await backend_http_send(
        target=f"/anonymous/pki/{organization_id}/enrollment_request",
        method="POST",
        headers=headers,
        body=data,
        sanity_checks=True,
        addr=backend_addr,
    )
    assert status == (200, "OK")
    rep = pki_enrollment_request_rep_serializer.loads(body)
    assert rep["status"] == "ok"
    assert ref_timestamp < rep["timestamp"] < pendulum.now()

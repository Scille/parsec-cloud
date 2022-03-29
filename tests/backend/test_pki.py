# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from uuid import uuid4
import pendulum
import pytest
from parsec.api.data.pki import PkiEnrollmentRequestInfo

from parsec.api.protocol.types import DeviceID, DeviceLabel, HumanHandle, UserProfile
from parsec.api.data import InviteUserConfirmation

from parsec.core.backend_connection.cmds import pki_enrollment_get_requests, pki_enrollment_reply
from parsec.api.protocol.pki import PkiEnrollmentRequest, PkiEnrollmentReply
from parsec.crypto import PublicKey, VerifyKey, generate_nonce
from parsec.core.invite.greeter import _create_new_user_certificates


@pytest.mark.trio
async def test_pki_get_empty_request(backend, alice, bob, alice_backend_sock):
    data = await backend.pki.pki_enrollment_get_requests()
    assert not data
    rep = await pki_enrollment_get_requests(alice_backend_sock.transport)
    assert rep["status"] == "ok"
    assert rep["requests"] == []


async def _setup_backend_request(backend, organization_id):
    certificate_id = b"certificate_id"
    request_id = uuid4()

    requested_human_handle = HumanHandle(
        email="zapp.brannigan@earthgovernment.com", label="Zapp Brannigan"
    )
    requested_device_label = DeviceLabel("Nimbus Computer")

    pki_request_info = PkiEnrollmentRequestInfo(
        verify_key=VerifyKey(generate_nonce(32)),
        public_key=PublicKey(generate_nonce(32)),
        requested_human_handle=requested_human_handle,
        requested_device_label=requested_device_label,
    )
    pki_request = PkiEnrollmentRequest(
        der_x509_certificate=b"1234567890ABCDEF",
        signature=b"123",
        requested_human_handle=requested_human_handle,
        pki_request_info=pki_request_info.dump(),
    )
    await backend.pki.pki_enrollment_request(
        organization_id, certificate_id, request_id, pki_request, False
    )
    return certificate_id, request_id, requested_human_handle, requested_device_label, pki_request


@pytest.mark.trio
async def test_pki_send_accepted_reply(backend, alice, alice_backend_sock):
    ref_time = pendulum.now()
    (
        certificate_id,
        request_id,
        requested_human_handle,
        requested_device_label,
        *_,
    ) = await _setup_backend_request(backend, alice.organization_id)

    rep = await pki_enrollment_get_requests(alice_backend_sock.transport)

    assert rep["status"] == "ok"
    assert len(rep["requests"]) == 1
    assert rep["requests"][0][0] == certificate_id

    pki_reply_info = InviteUserConfirmation(
        device_id=DeviceID("ZappB@nimbus"),
        root_verify_key=alice.verify_key,
        device_label=requested_device_label,
        human_handle=requested_human_handle,
        profile=UserProfile.ADMIN,
    )

    pki_reply = PkiEnrollmentReply(
        der_x509_admin_certificate=b"admin_cert",
        signature=b"123",
        pki_reply_info=pki_reply_info.dump(),
    )
    (
        user_certificate,
        redacted_user_certificate,
        device_certificate,
        redacted_device_certificate,
        user_confirmation,
    ) = _create_new_user_certificates(
        alice,
        requested_device_label,
        requested_human_handle,
        UserProfile.ADMIN,
        alice.public_key,
        alice.verify_key,
    )

    rep = await pki_enrollment_reply(
        alice_backend_sock.transport,
        certificate_id=certificate_id,
        request_id=request_id,
        reply=pki_reply,
        user_id=user_confirmation.device_id.user_id,
        device_certificate=device_certificate,
        user_certificate=user_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )

    assert rep["status"] == "ok"
    assert rep["timestamp"] > ref_time

    (rep_object, _, _, _, rep_admin) = await backend.pki.pki_enrollment_get_reply(
        certificate_id, request_id
    )
    assert rep_object == pki_reply
    assert rep_admin == alice.human_handle

    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=requested_human_handle.email
    )

    assert rep[1] == 1
    rep_human_handle = rep[0][0]
    assert not rep_human_handle.revoked
    assert rep_human_handle.user_id == user_confirmation.device_id.user_id
    assert rep_human_handle.human_handle == requested_human_handle


@pytest.mark.trio
async def test_pki_send_rejected_reply(backend, alice, alice_backend_sock):
    ref_time = pendulum.now()
    (
        certificate_id,
        request_id,
        requested_human_handle,
        requested_device_label,
        *_,
    ) = await _setup_backend_request(backend, alice.organization_id)

    rep = await pki_enrollment_get_requests(alice_backend_sock.transport)

    assert rep["status"] == "ok"
    assert len(rep["requests"]) == 1
    assert rep["requests"][0][0] == certificate_id

    rep = await pki_enrollment_reply(
        alice_backend_sock.transport,
        certificate_id=certificate_id,
        request_id=request_id,
        reply=None,
        user_id=None,
        device_certificate=None,
        user_certificate=None,
        redacted_user_certificate=None,
        redacted_device_certificate=None,
    )

    assert rep["status"] == "ok"
    assert rep["timestamp"] > ref_time

    (rep_object, _, _, _, rep_admin) = await backend.pki.pki_enrollment_get_reply(
        certificate_id, request_id
    )
    assert not rep_object
    assert rep_admin == alice.human_handle

    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=requested_human_handle.email
    )

    assert rep == ([], 0)

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from uuid import uuid4
import pytest

from parsec.api.data import PkiEnrollmentSubmitPayload
from parsec.core.backend_connection.cmds import pki_enrollment_submit

# from parsec.api.data.pki import PkiEnrollmentRequestInfo
# from parsec.api.protocol.types import DeviceID, DeviceLabel, HumanHandle, UserProfile
# from parsec.api.data import InviteUserConfirmation
# from parsec.api.protocol.pki import PkiEnrollmentRequest, PkiEnrollmentReply
# from parsec.crypto import PublicKey, VerifyKey, generate_nonce
# from parsec.core.invite.greeter import _create_new_user_certificates

from tests.backend.common import pki_enrollment_list

# async def _setup_backend_request(backend, organization_id):
#     certificate_id = b"certificate_id"
#     request_id = uuid4()

#     requested_human_handle = HumanHandle(
#         email="zapp.brannigan@earthgovernment.com", label="Zapp Brannigan"
#     )
#     requested_device_label = DeviceLabel("Nimbus Computer")

#     pki_request_info = PkiEnrollmentRequestInfo(
#         verify_key=VerifyKey(generate_nonce(32)),
#         public_key=PublicKey(generate_nonce(32)),
#         requested_human_handle=requested_human_handle,
#         requested_device_label=requested_device_label,
#     )
#     pki_request = PkiEnrollmentRequest(
#         der_x509_certificate=b"1234567890ABCDEF",
#         signature=b"123",
#         requested_human_handle=requested_human_handle,
#         pki_request_info=pki_request_info.dump(),
#     )
#     await backend.pki.pki_enrollment_request(
#         organization_id, certificate_id, request_id, pki_request, False
#     )
#     return certificate_id, request_id, requested_human_handle, requested_device_label, pki_request


@pytest.mark.trio
async def test_pki_list_empty(alice_backend_sock):
    rep = await pki_enrollment_list(alice_backend_sock)
    assert rep == {"status": "ok", "enrollments": []}


@pytest.mark.trio
async def test_pki_submit(anonymous_backend_sock, bob):
    payload = PkiEnrollmentSubmitPayload(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_human_handle=bob.human_handle,
        requested_device_label=bob.device_label,
    ).dump()
    rep = await pki_enrollment_submit(
        anonymous_backend_sock,
        enrollment_id=uuid4(),
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert rep == {"status": "ok"}

    # Retry without force

    rep = await pki_enrollment_submit(
        anonymous_backend_sock,
        enrollment_id=uuid4(),
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert rep == {"status": "already_submitted"}

    # Retry with force

    rep = await pki_enrollment_submit(
        anonymous_backend_sock,
        enrollment_id=uuid4(),
        force=True,
        submitter_der_x509_certificate=b"<x509 certif>",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert rep == {"status": "ok"}


# @pytest.mark.trio
# async def test_pki_get_requests(backend, alice, alice_backend_sock):
#     (certificate_id, *_) = await _setup_backend_request(backend, alice.organization_id)

#     rep = await pki_enrollment_get_requests(alice_backend_sock.transport)

#     assert rep["status"] == "ok"
#     assert len(rep["requests"]) == 1
#     assert rep["requests"][0][0] == certificate_id


# @pytest.mark.trio
# async def test_pki_send_reply_accepted(backend, alice, alice_backend_sock):
#     ref_time = pendulum.now()
#     (
#         certificate_id,
#         request_id,
#         requested_human_handle,
#         requested_device_label,
#         *_,
#     ) = await _setup_backend_request(backend, alice.organization_id)

#     pki_reply_info = InviteUserConfirmation(
#         device_id=DeviceID("ZappB@nimbus"),
#         root_verify_key=alice.verify_key,
#         device_label=requested_device_label,
#         human_handle=requested_human_handle,
#         profile=UserProfile.ADMIN,
#     )

#     pki_reply = PkiEnrollmentReply(
#         der_x509_admin_certificate=b"admin_cert",
#         signature=b"123",
#         pki_reply_info=pki_reply_info.dump(),
#     )
#     (
#         user_certificate,
#         redacted_user_certificate,
#         device_certificate,
#         redacted_device_certificate,
#         user_confirmation,
#     ) = _create_new_user_certificates(
#         alice,
#         requested_device_label,
#         requested_human_handle,
#         UserProfile.ADMIN,
#         alice.public_key,
#         alice.verify_key,
#     )

#     rep = await pki_enrollment_reply(
#         alice_backend_sock.transport,
#         certificate_id=certificate_id,
#         request_id=request_id,
#         reply=pki_reply,
#         user_id=user_confirmation.device_id.user_id,
#         device_certificate=device_certificate,
#         user_certificate=user_certificate,
#         redacted_user_certificate=redacted_user_certificate,
#         redacted_device_certificate=redacted_device_certificate,
#     )

#     assert rep["status"] == "ok"
#     assert rep["timestamp"] > ref_time

#     (rep_object, _, _, _, rep_admin) = await backend.pki.pki_enrollment_get_reply(
#         certificate_id, request_id
#     )
#     assert rep_object == pki_reply
#     assert rep_admin == alice.human_handle

#     rep = await backend.user.find_humans(
#         organization_id=alice.organization_id, query=requested_human_handle.email
#     )

#     assert rep[1] == 1
#     rep_human_handle = rep[0][0]
#     assert not rep_human_handle.revoked
#     assert rep_human_handle.user_id == user_confirmation.device_id.user_id
#     assert rep_human_handle.human_handle == requested_human_handle


# @pytest.mark.trio
# async def test_pki_send_reply_rejected(backend, alice, alice_backend_sock):
#     ref_time = pendulum.now()
#     (certificate_id, request_id, requested_human_handle, *_) = await _setup_backend_request(
#         backend, alice.organization_id
#     )

#     rep = await pki_enrollment_reply(
#         alice_backend_sock.transport,
#         certificate_id=certificate_id,
#         request_id=request_id,
#         reply=None,
#         user_id=None,
#         device_certificate=None,
#         user_certificate=None,
#         redacted_user_certificate=None,
#         redacted_device_certificate=None,
#     )

#     assert rep["status"] == "ok"
#     assert rep["timestamp"] > ref_time

#     (rep_object, _, _, _, rep_admin) = await backend.pki.pki_enrollment_get_reply(
#         certificate_id, request_id
#     )
#     assert not rep_object
#     assert rep_admin == alice.human_handle

#     rep = await backend.user.find_humans(
#         organization_id=alice.organization_id, query=requested_human_handle.email
#     )

#     assert rep == ([], 0)


# @pytest.mark.trio
# async def test_pki_send_reply_wrong_certificate_id(backend, alice, alice_backend_sock):
#     (_, request_id, *_) = await _setup_backend_request(backend, alice.organization_id)

#     rep = await pki_enrollment_reply(
#         alice_backend_sock.transport,
#         certificate_id=b"wrong_certificate_id",
#         request_id=request_id,
#         reply=None,
#         user_id=None,
#         device_certificate=None,
#         user_certificate=None,
#         redacted_user_certificate=None,
#         redacted_device_certificate=None,
#     )
#     assert rep["status"] == "certificate not found"


# @pytest.mark.trio
# async def test_pki_send_reply_wrond_request_id(backend, alice, alice_backend_sock):
#     (certificate_id, *_) = await _setup_backend_request(backend, alice.organization_id)

#     rep = await pki_enrollment_reply(
#         alice_backend_sock.transport,
#         certificate_id=certificate_id,
#         request_id=uuid4(),
#         reply=None,
#         user_id=None,
#         device_certificate=None,
#         user_certificate=None,
#         redacted_user_certificate=None,
#         redacted_device_certificate=None,
#     )
#     assert rep["status"] == "request not found"

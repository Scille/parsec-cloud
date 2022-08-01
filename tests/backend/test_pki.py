# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import pendulum
from uuid import uuid4

from parsec.api.data import PkiEnrollmentSubmitPayload
from parsec.api.data.certif import RevokedUserCertificateContent
from parsec.api.data.pki import PkiEnrollmentAcceptPayload
from parsec.api.protocol.pki import PkiEnrollmentStatus
from parsec.api.protocol.types import UserProfile
from parsec.core.invite.greeter import _create_new_user_certificates

from tests.backend.common import pki_enrollment_submit, pki_enrollment_info
from tests.backend.common import (
    pki_enrollment_accept,
    pki_enrollment_list,
    pki_enrollment_reject,
    user_revoke,
)


# Helpers


async def _submit_request(
    anonymous_backend_ws,
    bob,
    certif=b"<x509 certif>",
    signature=b"<signature>",
    request_id=None,
    force=False,
    certif_email="new_challenger@jointhebattle.com",
):
    if not request_id:
        request_id = uuid4()
    payload = PkiEnrollmentSubmitPayload(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_device_label=bob.device_label,
    ).dump()
    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=request_id,
        force=force,
        submitter_der_x509_certificate=certif,
        submitter_der_x509_certificate_email=certif_email,
        submit_payload_signature=signature,
        submit_payload=payload,
    )
    assert rep["status"] == "ok"


def _prepare_accept_reply(admin, invitee):
    (
        user_certificate,
        redacted_user_certificate,
        device_certificate,
        redacted_device_certificate,
        user_confirmation,
    ) = _create_new_user_certificates(
        admin,
        invitee.device_label,
        invitee.human_handle,
        UserProfile.STANDARD,
        admin.public_key,
        admin.verify_key,
    )
    payload = PkiEnrollmentAcceptPayload(
        device_id=invitee.device_id,
        device_label=invitee.device_label,
        human_handle=invitee.human_handle,
        profile=UserProfile.STANDARD,
        root_verify_key=admin.root_verify_key,
    ).dump()
    kwargs = {
        "accepter_der_x509_certificate": b"<accepter_der_x509_certificate>",
        "accept_payload_signature": b"<signature>",
        "accept_payload": payload,
        "user_certificate": user_certificate,
        "device_certificate": device_certificate,
        "redacted_user_certificate": redacted_user_certificate,
        "redacted_device_certificate": redacted_device_certificate,
    }

    return (user_confirmation, kwargs)


# Test pki_enrollment_submit


@pytest.mark.trio
async def test_pki_submit(anonymous_backend_ws, bob):
    payload = PkiEnrollmentSubmitPayload(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_device_label=bob.device_label,
    ).dump()

    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=uuid4(),
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )

    assert rep["status"] == "ok"
    # Retry without force

    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=uuid4(),
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )

    assert rep["status"] == "already_submitted"
    assert rep["submitted_on"]

    # Retry with force

    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=uuid4(),
        force=True,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert rep["status"] == "ok"


@pytest.mark.trio
async def test_pki_submit_same_id(anonymous_backend_ws, bob):
    payload = PkiEnrollmentSubmitPayload(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_device_label=bob.device_label,
    ).dump()
    enrollment_id = uuid4()

    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=enrollment_id,
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert rep["status"] == "ok"

    # Same enrollment ID without Force
    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=enrollment_id,
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert rep["status"] == "enrollment_id_already_used"

    # Same enrollment ID with Froce
    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=enrollment_id,
        force=True,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert rep["status"] == "enrollment_id_already_used"


@pytest.mark.trio
async def test_pki_submit_already_used_email(anonymous_backend_ws, bob):
    payload = PkiEnrollmentSubmitPayload(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_device_label=bob.device_label,
    ).dump()
    enrollment_id = uuid4()

    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=enrollment_id,
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email=bob.human_handle.email,  # bob user with this email already exist
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert rep["status"] == "email_already_used"


@pytest.mark.trio
async def test_pki_submit_no_email_provided(anonymous_backend_ws, bob):
    # Test backend compatibility with core version < 2.8.3 that does not provide an email address field
    payload = PkiEnrollmentSubmitPayload(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_device_label=bob.device_label,
    ).dump()
    enrollment_id = uuid4()

    def _req_without_email_field(req):
        req.pop("submitter_der_x509_certificate_email")
        return req

    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        req_post_processing=_req_without_email_field,
        enrollment_id=enrollment_id,
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="removed.in@post.processing",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )

    assert rep["status"] == "ok"


# Test pki_enrollment_list


@pytest.mark.trio
async def test_pki_list(anonymous_backend_ws, bob, adam, alice_ws):
    ref_time = pendulum.now()
    bob_certif = b"<x509 certif>"
    bob_request_id = uuid4()
    bob_certif_signature = b"<signature>"

    await _submit_request(
        anonymous_backend_ws, bob, bob_certif, bob_certif_signature, bob_request_id
    )

    rep = await pki_enrollment_list(alice_ws)

    assert rep["status"] == "ok"
    assert len(rep["enrollments"]) == 1

    submited_request = rep["enrollments"][0]
    assert submited_request["enrollment_id"] == bob_request_id
    assert submited_request["submitter_der_x509_certificate"] == bob_certif
    assert submited_request["submit_payload_signature"] == bob_certif_signature
    # In theory we should have sumitted_on > ref_time, but clock resolution on Windows is poor
    assert submited_request["submitted_on"] >= ref_time

    submited_payload = PkiEnrollmentSubmitPayload.load(submited_request["submit_payload"])
    assert submited_payload.verify_key == bob.verify_key
    assert submited_payload.public_key == bob.public_key
    assert submited_payload.requested_device_label == bob.device_label

    # Add another user

    await _submit_request(
        anonymous_backend_ws, adam, b"<adam's x509 certif>", b"<signature>", uuid4()
    )
    rep = await pki_enrollment_list(alice_ws)

    assert rep["status"] == "ok"
    assert len(rep["enrollments"]) == 2


@pytest.mark.trio
async def test_pki_list_empty(alice_ws):
    rep = await pki_enrollment_list(alice_ws)
    assert rep == {"status": "ok", "enrollments": []}


# Test pki_enrollment_accept


@pytest.mark.trio
async def test_pki_accept(anonymous_backend_ws, mallory, alice, alice_ws, backend):
    # Assert mallory does not exist
    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=mallory.human_handle.email
    )
    assert rep == ([], 0)

    # Create request
    request_id = uuid4()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    # Send reply
    user_confirmation, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep == {"status": "ok"}

    # Assert user has been created
    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=mallory.human_handle.email
    )
    assert rep[1] == 1
    rep_human_handle = rep[0][0]
    assert not rep_human_handle.revoked
    assert rep_human_handle.user_id == user_confirmation.device_id.user_id
    assert rep_human_handle.human_handle == mallory.human_handle

    # Send reply twice
    user_confirmation, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep["status"] == "no_longer_available"


@pytest.mark.trio
async def test_pki_accept_not_found(mallory, alice, alice_ws, backend):
    request_id = uuid4()

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep["status"] == "not_found"

    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=mallory.human_handle.email
    )
    assert rep == ([], 0)


@pytest.mark.trio
async def test_pki_accept_invalid_certificate(mallory, alice, alice_ws, backend):
    request_id = uuid4()

    # Create certificate with mallory user instead of alice
    _, kwargs = _prepare_accept_reply(admin=mallory, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep["status"] == "invalid_certification"

    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=mallory.human_handle.email
    )
    assert rep == ([], 0)


@pytest.mark.trio
async def test_pki_accept_outdated_submit(anonymous_backend_ws, mallory, alice, alice_ws, backend):
    # First request
    request_id = uuid4()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)
    # Second request
    await _submit_request(anonymous_backend_ws, mallory, force=True)

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep["status"] == "no_longer_available"

    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=mallory.human_handle.email
    )
    assert rep == ([], 0)


@pytest.mark.trio
async def test_pki_accept_user_already_exist(anonymous_backend_ws, bob, alice, alice_ws):
    request_id = uuid4()
    await _submit_request(anonymous_backend_ws, bob, request_id=request_id)

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=bob)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep["status"] == "already_exists"

    # Revoke user
    now = pendulum.now()
    bob_revocation = RevokedUserCertificateContent(
        author=alice.device_id, timestamp=now, user_id=bob.user_id
    ).dump_and_sign(alice.signing_key)

    rep = await user_revoke(alice_ws, revoked_user_certificate=bob_revocation)
    assert rep == {"status": "ok"}

    # Accept revoked user
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep["status"] == "ok"


@pytest.mark.trio
async def test_pki_accept_limit_reached(backend, anonymous_backend_ws, mallory, alice, alice_ws):
    # Change organization settings
    await backend.organization.update(alice.organization_id, is_expired=False, active_users_limit=1)
    request_id = uuid4()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep["status"] == "active_users_limit_reached"


@pytest.mark.trio
async def test_pki_accept_already_rejected(backend, anonymous_backend_ws, mallory, alice, alice_ws):
    request_id = uuid4()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    # Reject
    rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
    assert rep["status"] == "ok"

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep["status"] == "no_longer_available"


# TODO: test_pki_accept_limit_expired ??

# Test pki_entollment_reject


@pytest.mark.trio
async def test_pki_reject(anonymous_backend_ws, mallory, alice_ws):
    # Create request
    request_id = uuid4()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
    assert rep == {"status": "ok"}

    # Reject twice
    rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
    assert rep["status"] == "no_longer_available"


@pytest.mark.trio
async def test_pki_reject_not_found(alice_ws):
    request_id = uuid4()

    rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
    assert rep["status"] == "not_found"


@pytest.mark.trio
async def test_pki_reject_already_accepted(anonymous_backend_ws, mallory, alice, alice_ws):
    request_id = uuid4()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)
    # Accept request
    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep == {"status": "ok"}

    # Reject accepted request
    rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
    assert rep["status"] == "no_longer_available"


@pytest.mark.trio
async def test_pki_submit_already_accepted(anonymous_backend_ws, mallory, alice, alice_ws, backend):
    request_id = uuid4()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    user_confirmation, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep == {"status": "ok"}

    # Pki enrollment is accepted and user not revoked
    payload = PkiEnrollmentSubmitPayload(
        verify_key=mallory.verify_key,
        public_key=mallory.public_key,
        requested_device_label=mallory.device_label,
    ).dump()
    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=uuid4(),
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert rep["status"] == "already_enrolled"

    # Revoke user
    now = pendulum.now()
    revocation = RevokedUserCertificateContent(
        author=alice.device_id, timestamp=now, user_id=user_confirmation.device_id.user_id
    ).dump_and_sign(alice.signing_key)

    rep = await user_revoke(alice_ws, revoked_user_certificate=revocation)
    assert rep == {"status": "ok"}

    # Pki enrollment is accepted and user revoked
    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=uuid4(),
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )

    assert rep["status"] == "ok"


# Test pki_enrollment_info


@pytest.mark.trio
async def test_pki_info(anonymous_backend_ws, mallory, alice, alice_ws):
    request_id = uuid4()

    # Request not found
    rep = await pki_enrollment_info(anonymous_backend_ws, request_id)
    assert rep == {"reason": "", "status": "not_found"}

    # Request submitted
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)
    rep = await pki_enrollment_info(anonymous_backend_ws, request_id)
    assert rep["status"] == "ok"
    assert rep["enrollment_status"] == PkiEnrollmentStatus.SUBMITTED

    # Request cancelled
    new_request_id = uuid4()
    await _submit_request(anonymous_backend_ws, mallory, request_id=new_request_id, force=True)
    rep = await pki_enrollment_info(anonymous_backend_ws, request_id)
    assert rep["status"] == "ok"
    assert rep["enrollment_status"] == PkiEnrollmentStatus.CANCELLED


@pytest.mark.trio
async def test_pki_info_accepted(anonymous_backend_ws, mallory, alice, alice_ws):
    request_id = uuid4()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert rep == {"status": "ok"}

    rep = await pki_enrollment_info(anonymous_backend_ws, request_id)
    assert rep["status"] == "ok"
    assert rep["enrollment_status"] == PkiEnrollmentStatus.ACCEPTED


@pytest.mark.trio
async def test_pki_info_rejected(anonymous_backend_ws, mallory, alice_ws):
    request_id = uuid4()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
    assert rep["status"] == "ok"

    rep = await pki_enrollment_info(anonymous_backend_ws, request_id)
    assert rep["status"] == "ok"
    assert rep["enrollment_status"] == PkiEnrollmentStatus.REJECTED


@pytest.mark.trio
async def test_pki_complete_sequence(anonymous_backend_ws, mallory, alice_ws, alice):
    async def _cancel():
        await _submit_request(anonymous_backend_ws, mallory, force=True)
        # Create more than once cancel request
        await _submit_request(anonymous_backend_ws, mallory, force=True)

    async def _reject():

        request_id = uuid4()
        await _submit_request(anonymous_backend_ws, mallory, request_id=request_id, force=True)
        rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
        assert rep["status"] == "ok"

    async def _accept():
        request_id = uuid4()
        await _submit_request(anonymous_backend_ws, mallory, request_id=request_id, force=True)
        user_confirmation, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
        rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
        assert rep == {"status": "ok"}
        return user_confirmation.device_id.user_id

    async def _revoke(user_id):
        now = pendulum.now()
        revocation = RevokedUserCertificateContent(
            author=alice.device_id, timestamp=now, user_id=user_id
        ).dump_and_sign(alice.signing_key)

        rep = await user_revoke(alice_ws, revoked_user_certificate=revocation)
        assert rep == {"status": "ok"}

    for _ in range(2):
        await _cancel()
        await _reject()
        await _cancel()
        user_id = await _accept()
        await _revoke(user_id)

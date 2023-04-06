# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    EnrollmentID,
    EventsListenRepOkPkiEnrollmentUpdated,
    PkiEnrollmentAcceptRepActiveUsersLimitReached,
    PkiEnrollmentAcceptRepAlreadyExists,
    PkiEnrollmentAcceptRepInvalidCertification,
    PkiEnrollmentAcceptRepNoLongerAvailable,
    PkiEnrollmentAcceptRepNotFound,
    PkiEnrollmentAcceptRepOk,
    PkiEnrollmentInfoRepNotFound,
    PkiEnrollmentInfoRepOk,
    PkiEnrollmentListRepOk,
    PkiEnrollmentRejectRepNoLongerAvailable,
    PkiEnrollmentRejectRepNotFound,
    PkiEnrollmentRejectRepOk,
    PkiEnrollmentStatus,
    PkiEnrollmentSubmitRepAlreadyEnrolled,
    PkiEnrollmentSubmitRepAlreadySubmitted,
    PkiEnrollmentSubmitRepEmailAlreadyUsed,
    PkiEnrollmentSubmitRepIdAlreadyUsed,
    PkiEnrollmentSubmitRepOk,
    UserRevokeRepOk,
)
from parsec.api.data import (
    PkiEnrollmentAnswerPayload,
    PkiEnrollmentSubmitPayload,
    RevokedUserCertificate,
)
from parsec.api.protocol.types import UserProfile
from parsec.backend.backend_events import BackendEvent
from parsec.core.invite.greeter import _create_new_user_certificates
from tests.backend.common import (
    events_listen_nowait,
    events_subscribe,
    pki_enrollment_accept,
    pki_enrollment_info,
    pki_enrollment_list,
    pki_enrollment_reject,
    pki_enrollment_submit,
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
        request_id = EnrollmentID.new()
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
    assert isinstance(rep, PkiEnrollmentSubmitRepOk)


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
    payload = PkiEnrollmentAnswerPayload(
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
async def test_pki_submit(backend, anonymous_backend_ws, bob, bob_ws):
    payload = PkiEnrollmentSubmitPayload(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_device_label=bob.device_label,
    ).dump()

    await events_subscribe(bob_ws)

    with backend.event_bus.listen() as spy:
        rep = await pki_enrollment_submit(
            anonymous_backend_ws,
            enrollment_id=EnrollmentID.new(),
            force=False,
            submitter_der_x509_certificate=b"<x509 certif>",
            submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
            submit_payload_signature=b"<signature>",
            submit_payload=payload,
        )
        assert isinstance(rep, PkiEnrollmentSubmitRepOk)
        await spy.wait_with_timeout(BackendEvent.PKI_ENROLLMENTS_UPDATED)
    assert await events_listen_nowait(bob_ws) == EventsListenRepOkPkiEnrollmentUpdated()

    # Retry without force

    with backend.event_bus.listen() as spy:
        rep = await pki_enrollment_submit(
            anonymous_backend_ws,
            enrollment_id=EnrollmentID.new(),
            force=False,
            submitter_der_x509_certificate=b"<x509 certif>",
            submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
            submit_payload_signature=b"<signature>",
            submit_payload=payload,
        )

        assert isinstance(rep, PkiEnrollmentSubmitRepAlreadySubmitted)
        assert rep.submitted_on is not None
        assert not spy.events

    # Retry with force

    with backend.event_bus.listen() as spy:
        rep = await pki_enrollment_submit(
            anonymous_backend_ws,
            enrollment_id=EnrollmentID.new(),
            force=True,
            submitter_der_x509_certificate=b"<x509 certif>",
            submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
            submit_payload_signature=b"<signature>",
            submit_payload=payload,
        )
        assert isinstance(rep, PkiEnrollmentSubmitRepOk)
        await spy.wait_with_timeout(BackendEvent.PKI_ENROLLMENTS_UPDATED)
    assert await events_listen_nowait(bob_ws) == EventsListenRepOkPkiEnrollmentUpdated()


@pytest.mark.trio
async def test_pki_submit_same_id(anonymous_backend_ws, bob):
    payload = PkiEnrollmentSubmitPayload(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_device_label=bob.device_label,
    ).dump()
    enrollment_id = EnrollmentID.new()

    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=enrollment_id,
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert isinstance(rep, PkiEnrollmentSubmitRepOk)

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
    assert isinstance(rep, PkiEnrollmentSubmitRepIdAlreadyUsed)

    # Same enrollment ID with Force
    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=enrollment_id,
        force=True,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert isinstance(rep, PkiEnrollmentSubmitRepIdAlreadyUsed)


@pytest.mark.trio
async def test_pki_submit_already_used_email(anonymous_backend_ws, bob):
    payload = PkiEnrollmentSubmitPayload(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_device_label=bob.device_label,
    ).dump()
    enrollment_id = EnrollmentID.new()

    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=enrollment_id,
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email=bob.human_handle.email,  # bob user with this email already exist
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert isinstance(rep, PkiEnrollmentSubmitRepEmailAlreadyUsed)


@pytest.mark.trio
async def test_pki_submit_no_email_provided(anonymous_backend_ws, bob):
    # Test backend compatibility with core version < 2.8.3 that does not provide an email address field
    payload = PkiEnrollmentSubmitPayload(
        verify_key=bob.verify_key,
        public_key=bob.public_key,
        requested_device_label=bob.device_label,
    ).dump()
    enrollment_id = EnrollmentID.new()

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

    assert isinstance(rep, PkiEnrollmentSubmitRepOk)


# Test pki_enrollment_list


@pytest.mark.trio
async def test_pki_list(anonymous_backend_ws, bob, adam, alice_ws):
    ref_time = DateTime.now()
    bob_certif = b"<x509 certif>"
    bob_request_id = EnrollmentID.new()
    bob_certif_signature = b"<signature>"

    await _submit_request(
        anonymous_backend_ws, bob, bob_certif, bob_certif_signature, bob_request_id
    )

    rep = await pki_enrollment_list(alice_ws)

    assert isinstance(rep, PkiEnrollmentListRepOk)
    assert len(rep.enrollments) == 1

    submitted_request = rep.enrollments[0]
    assert submitted_request.enrollment_id == bob_request_id
    assert submitted_request.submitter_der_x509_certificate == bob_certif
    assert submitted_request.submit_payload_signature == bob_certif_signature
    # In theory we should have submitted_on > ref_time, but clock resolution on Windows is poor
    assert submitted_request.submitted_on >= ref_time

    submitted_payload = PkiEnrollmentSubmitPayload.load(submitted_request.submit_payload)
    assert submitted_payload.verify_key == bob.verify_key
    assert submitted_payload.public_key == bob.public_key
    assert submitted_payload.requested_device_label == bob.device_label

    # Add another user

    await _submit_request(
        anonymous_backend_ws, adam, b"<adam's x509 certif>", b"<signature>", EnrollmentID.new()
    )
    rep = await pki_enrollment_list(alice_ws)

    assert isinstance(rep, PkiEnrollmentListRepOk)
    assert len(rep.enrollments) == 2


@pytest.mark.trio
async def test_pki_list_empty(alice_ws):
    rep = await pki_enrollment_list(alice_ws)
    assert isinstance(rep, PkiEnrollmentListRepOk)
    assert rep.enrollments == []


# Test pki_enrollment_accept


@pytest.mark.trio
async def test_pki_accept(backend, anonymous_backend_ws, mallory, alice, alice_ws):
    await events_subscribe(alice_ws)

    # Assert mallory does not exist
    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=mallory.human_handle.email
    )
    assert rep == ([], 0)

    # Create request
    request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    # Send reply
    with backend.event_bus.listen() as spy:
        user_confirmation, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
        rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
        assert isinstance(rep, PkiEnrollmentAcceptRepOk)
        await spy.wait_with_timeout(BackendEvent.PKI_ENROLLMENTS_UPDATED)
    assert await events_listen_nowait(alice_ws) == EventsListenRepOkPkiEnrollmentUpdated()

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
    with backend.event_bus.listen() as spy:
        rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
        assert isinstance(rep, PkiEnrollmentAcceptRepNoLongerAvailable)
        assert not spy.events


@pytest.mark.trio
async def test_pki_accept_not_found(mallory, alice, alice_ws, backend):
    request_id = EnrollmentID.new()

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert isinstance(rep, PkiEnrollmentAcceptRepNotFound)

    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=mallory.human_handle.email
    )
    assert rep == ([], 0)


@pytest.mark.trio
async def test_pki_accept_invalid_certificate(mallory, alice, alice_ws, backend):
    request_id = EnrollmentID.new()

    # Create certificate with mallory user instead of alice
    _, kwargs = _prepare_accept_reply(admin=mallory, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert isinstance(rep, PkiEnrollmentAcceptRepInvalidCertification)

    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=mallory.human_handle.email
    )
    assert rep == ([], 0)


@pytest.mark.trio
async def test_pki_accept_outdated_submit(anonymous_backend_ws, mallory, alice, alice_ws, backend):
    # First request
    request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)
    # Second request
    await _submit_request(anonymous_backend_ws, mallory, force=True)

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert isinstance(rep, PkiEnrollmentAcceptRepNoLongerAvailable)

    rep = await backend.user.find_humans(
        organization_id=alice.organization_id, query=mallory.human_handle.email
    )
    assert rep == ([], 0)


@pytest.mark.trio
async def test_pki_accept_user_already_exist(anonymous_backend_ws, bob, alice, alice_ws):
    request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, bob, request_id=request_id)

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=bob)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert isinstance(rep, PkiEnrollmentAcceptRepAlreadyExists)

    # Revoke user
    now = DateTime.now()
    bob_revocation = RevokedUserCertificate(
        author=alice.device_id, timestamp=now, user_id=bob.user_id
    ).dump_and_sign(alice.signing_key)

    rep = await user_revoke(alice_ws, revoked_user_certificate=bob_revocation)
    assert isinstance(rep, UserRevokeRepOk)

    # Accept revoked user
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert isinstance(rep, PkiEnrollmentAcceptRepOk)


@pytest.mark.trio
async def test_pki_accept_limit_reached(backend, anonymous_backend_ws, mallory, alice, alice_ws):
    # Change organization settings
    await backend.organization.update(
        alice.organization_id, is_expired=False, active_users_limit=ActiveUsersLimit.LimitedTo(1)
    )
    request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)

    assert isinstance(rep, PkiEnrollmentAcceptRepActiveUsersLimitReached)


@pytest.mark.trio
async def test_pki_accept_already_rejected(backend, anonymous_backend_ws, mallory, alice, alice_ws):
    request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    # Reject
    rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
    assert isinstance(rep, PkiEnrollmentRejectRepOk)

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert isinstance(rep, PkiEnrollmentAcceptRepNoLongerAvailable)


# TODO: test_pki_accept_limit_expired ??

# Test pki_enrollment_reject


@pytest.mark.trio
async def test_pki_reject(backend, anonymous_backend_ws, mallory, alice_ws):
    await events_subscribe(alice_ws)

    # Create request
    request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    with backend.event_bus.listen() as spy:
        rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
        assert isinstance(rep, PkiEnrollmentRejectRepOk)
        await spy.wait_with_timeout(BackendEvent.PKI_ENROLLMENTS_UPDATED)
    assert await events_listen_nowait(alice_ws) == EventsListenRepOkPkiEnrollmentUpdated()

    # Reject twice
    with backend.event_bus.listen() as spy:
        rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
        assert isinstance(rep, PkiEnrollmentRejectRepNoLongerAvailable)
        assert not spy.events


@pytest.mark.trio
async def test_pki_reject_not_found(alice_ws):
    request_id = EnrollmentID.new()

    rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
    assert isinstance(rep, PkiEnrollmentRejectRepNotFound)


@pytest.mark.trio
async def test_pki_reject_already_accepted(anonymous_backend_ws, mallory, alice, alice_ws):
    request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)
    # Accept request
    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert isinstance(rep, PkiEnrollmentAcceptRepOk)

    # Reject accepted request
    rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
    assert isinstance(rep, PkiEnrollmentRejectRepNoLongerAvailable)


@pytest.mark.trio
async def test_pki_submit_already_accepted(anonymous_backend_ws, mallory, alice, alice_ws, backend):
    request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    user_confirmation, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert isinstance(rep, PkiEnrollmentAcceptRepOk)

    # Pki enrollment is accepted and user not revoked
    payload = PkiEnrollmentSubmitPayload(
        verify_key=mallory.verify_key,
        public_key=mallory.public_key,
        requested_device_label=mallory.device_label,
    ).dump()
    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=EnrollmentID.new(),
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )
    assert isinstance(rep, PkiEnrollmentSubmitRepAlreadyEnrolled)

    # Revoke user
    now = DateTime.now()
    revocation = RevokedUserCertificate(
        author=alice.device_id, timestamp=now, user_id=user_confirmation.device_id.user_id
    ).dump_and_sign(alice.signing_key)

    rep = await user_revoke(alice_ws, revoked_user_certificate=revocation)
    assert isinstance(rep, UserRevokeRepOk)

    # Pki enrollment is accepted and user revoked
    rep = await pki_enrollment_submit(
        anonymous_backend_ws,
        enrollment_id=EnrollmentID.new(),
        force=False,
        submitter_der_x509_certificate=b"<x509 certif>",
        submitter_der_x509_certificate_email="new_challenger@jointhebattle.com",
        submit_payload_signature=b"<signature>",
        submit_payload=payload,
    )

    assert isinstance(rep, PkiEnrollmentSubmitRepOk)


# Test pki_enrollment_info


@pytest.mark.trio
async def test_pki_info(anonymous_backend_ws, mallory, alice, alice_ws):
    request_id = EnrollmentID.new()

    # Request not found
    rep = await pki_enrollment_info(anonymous_backend_ws, request_id)
    assert isinstance(rep, PkiEnrollmentInfoRepNotFound)

    # Request submitted
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)
    rep = await pki_enrollment_info(anonymous_backend_ws, request_id)
    assert isinstance(rep, PkiEnrollmentInfoRepOk)
    assert rep.enrollment_status.status == PkiEnrollmentStatus.SUBMITTED

    # Request cancelled
    new_request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, mallory, request_id=new_request_id, force=True)
    rep = await pki_enrollment_info(anonymous_backend_ws, request_id)
    assert isinstance(rep, PkiEnrollmentInfoRepOk)
    assert rep.enrollment_status.status == PkiEnrollmentStatus.CANCELLED


@pytest.mark.trio
async def test_pki_info_accepted(anonymous_backend_ws, mallory, alice, alice_ws):
    request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    _, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
    rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
    assert isinstance(rep, PkiEnrollmentAcceptRepOk)

    rep = await pki_enrollment_info(anonymous_backend_ws, request_id)
    assert isinstance(rep, PkiEnrollmentInfoRepOk)
    assert rep.enrollment_status.status == PkiEnrollmentStatus.ACCEPTED


@pytest.mark.trio
async def test_pki_info_rejected(anonymous_backend_ws, mallory, alice_ws):
    request_id = EnrollmentID.new()
    await _submit_request(anonymous_backend_ws, mallory, request_id=request_id)

    rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
    assert isinstance(rep, PkiEnrollmentRejectRepOk)

    rep = await pki_enrollment_info(anonymous_backend_ws, request_id)
    assert isinstance(rep, PkiEnrollmentInfoRepOk)
    assert rep.enrollment_status.status == PkiEnrollmentStatus.REJECTED


@pytest.mark.trio
async def test_pki_complete_sequence(anonymous_backend_ws, mallory, alice_ws, alice):
    async def _cancel():
        await _submit_request(anonymous_backend_ws, mallory, force=True)
        # Create more than once cancel request
        await _submit_request(anonymous_backend_ws, mallory, force=True)

    async def _reject():
        request_id = EnrollmentID.new()
        await _submit_request(anonymous_backend_ws, mallory, request_id=request_id, force=True)
        rep = await pki_enrollment_reject(alice_ws, enrollment_id=request_id)
        assert isinstance(rep, PkiEnrollmentRejectRepOk)

    async def _accept():
        request_id = EnrollmentID.new()
        await _submit_request(anonymous_backend_ws, mallory, request_id=request_id, force=True)
        user_confirmation, kwargs = _prepare_accept_reply(admin=alice, invitee=mallory)
        rep = await pki_enrollment_accept(alice_ws, enrollment_id=request_id, **kwargs)
        assert isinstance(rep, PkiEnrollmentAcceptRepOk)
        return user_confirmation.device_id.user_id

    async def _revoke(user_id):
        now = DateTime.now()
        revocation = RevokedUserCertificate(
            author=alice.device_id, timestamp=now, user_id=user_id
        ).dump_and_sign(alice.signing_key)

        rep = await user_revoke(alice_ws, revoked_user_certificate=revocation)
        assert isinstance(rep, UserRevokeRepOk)

    for _ in range(2):
        await _cancel()
        await _reject()
        await _cancel()
        user_id = await _accept()
        await _revoke(user_id)

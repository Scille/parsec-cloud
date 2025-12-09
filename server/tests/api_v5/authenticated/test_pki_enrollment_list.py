# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    EmailAddress,
    HumanHandle,
    PKIEnrollmentID,
    PkiEnrollmentSubmitPayload,
    PkiSignatureAlgorithm,
    PrivateKey,
    SigningKey,
    UserProfile,
    authenticated_cmds,
)
from parsec.components.memory.pki import MemoryPkiEnrollmentComponent
from tests.api_v5.authenticated.test_pki_enrollment_accept import (
    generate_accept_params,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    TestPki,
    bob_becomes_admin_and_changes_alice,
)


async def test_authenticated_pki_enrollment_list_ok(
    coolorg: CoolorgRpcClients, backend: Backend, test_pki: TestPki, xfail_if_postgresql: None
) -> None:
    # 1) Check with no enrollments available

    rep = await coolorg.alice.pki_enrollment_list()
    assert rep == authenticated_cmds.latest.pki_enrollment_list.RepOk(enrollments=[])

    # 2) Add enrollments

    expected_enrollments: list[
        authenticated_cmds.latest.pki_enrollment_list.PkiEnrollmentListItem
    ] = []

    # leaf subject -> (intermediate subject, root subject)
    certs = {
        "alice": ("", "black_mesa"),
        "mallory-encrypt": ("glados_dev_team", "aperture_science"),
        "bob": ("", "black_mesa"),
        "old-boby": ("", "black_mesa"),
    }
    every_cert = (
        [test_pki.cert[x] for x in ["alice", "mallory-encrypt", "bob", "old-boby"]]
        + [test_pki.intermediate["glados_dev_team"]]
        + [test_pki.root[x] for x in ["black_mesa", "aperture_science"]]
    )
    for subject, (i_cert, root_cert) in certs.items():
        enrollment_id = PKIEnrollmentID.new()
        submitted_on = DateTime.now()
        human_handle = HumanHandle(
            label=f"User{subject}", email=EmailAddress(f"{subject}@example.invalid")
        )
        submit_payload = PkiEnrollmentSubmitPayload(
            verify_key=SigningKey.generate().verify_key,
            public_key=PrivateKey.generate().public_key,
            device_label=DeviceLabel("Dev1"),
        ).dump()

        if len(i_cert) > 0:
            i_certs = [
                test_pki.cert[subject],
                test_pki.intermediate[i_cert],
                test_pki.root[root_cert],
            ]

        else:
            i_certs = [
                test_pki.cert[subject],
                test_pki.root[root_cert],
            ]
        intermediate_der_x509_certificates = [cert.der_certificate for cert in i_certs]

        expected_enrollment_item = (
            authenticated_cmds.latest.pki_enrollment_list.PkiEnrollmentListItem(
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                der_x509_certificate=test_pki.cert[subject].der_certificate,
                intermediate_der_x509_certificates=intermediate_der_x509_certificates,
                payload_signature=f"<user {subject} submit payload signature>".encode(),
                payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
                payload=submit_payload,
            )
        )
        outcome = await backend.pki.submit(
            now=submitted_on,
            organization_id=coolorg.organization_id,
            enrollment_id=enrollment_id,
            force=False,
            submitter_human_handle=human_handle,
            submitter_der_x509_certificate=expected_enrollment_item.der_x509_certificate,
            intermediate_certificates=[x.der_certificate for x in every_cert],
            submit_payload_signature=expected_enrollment_item.payload_signature,
            submit_payload_signature_algorithm=expected_enrollment_item.payload_signature_algorithm,
            submit_payload=expected_enrollment_item.payload,
        )
        assert outcome is None
        expected_enrollments.append(expected_enrollment_item)

    rep = await coolorg.alice.pki_enrollment_list()
    match rep:
        case authenticated_cmds.latest.pki_enrollment_list.RepOk() as e:
            for res, expected in zip(e.enrollments, expected_enrollments):
                assert res.enrollment_id == expected.enrollment_id
                assert res.submitted_on == expected.submitted_on
                assert res.der_x509_certificate == expected.der_x509_certificate
                assert (
                    res.intermediate_der_x509_certificates
                    == expected.intermediate_der_x509_certificates
                )
                assert res.payload_signature == expected.payload_signature
                assert res.payload_signature_algorithm == expected.payload_signature_algorithm
                assert res.payload == expected.payload

        case _:
            assert False

    # 3) Also ensure `ACCEPTED/CANCELLED/REJECTED` enrollments are ignored

    to_accept = expected_enrollments.pop()
    outcome = await backend.pki.accept(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        **generate_accept_params(coolorg, to_accept.enrollment_id, test_pki),
    )
    assert isinstance(outcome, tuple)

    to_reject = expected_enrollments.pop()
    outcome = await backend.pki.reject(
        now=DateTime.now(),
        author=coolorg.alice.device_id,
        organization_id=coolorg.organization_id,
        enrollment_id=to_reject.enrollment_id,
    )
    assert outcome is None

    to_cancel = expected_enrollments.pop()
    canceller_enrollment_id = PKIEnrollmentID.new()
    canceller_submitted_on = DateTime.now()
    human_handle = HumanHandle(label="Canceller", email=EmailAddress("canceller@example.invalid"))
    canceller_submit_payload = PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        device_label=DeviceLabel("Dev1"),
    ).dump()
    canceller_expected_enrollment_item = (
        authenticated_cmds.latest.pki_enrollment_list.PkiEnrollmentListItem(
            enrollment_id=canceller_enrollment_id,
            submitted_on=canceller_submitted_on,
            der_x509_certificate=to_cancel.der_x509_certificate,
            intermediate_der_x509_certificates=to_cancel.intermediate_der_x509_certificates,
            payload_signature=b"<canceller submit payload signature>",
            payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
            payload=canceller_submit_payload,
        )
    )
    outcome = await backend.pki.submit(
        now=canceller_submitted_on,
        organization_id=coolorg.organization_id,
        enrollment_id=canceller_enrollment_id,
        force=True,
        submitter_human_handle=human_handle,
        submitter_der_x509_certificate=canceller_expected_enrollment_item.der_x509_certificate,
        intermediate_certificates=canceller_expected_enrollment_item.intermediate_der_x509_certificates,
        submit_payload_signature=canceller_expected_enrollment_item.payload_signature,
        submit_payload_signature_algorithm=canceller_expected_enrollment_item.payload_signature_algorithm,
        submit_payload=canceller_expected_enrollment_item.payload,
    )
    assert outcome is None
    expected_enrollments.append(canceller_expected_enrollment_item)

    rep = await coolorg.alice.pki_enrollment_list()

    match rep:
        case authenticated_cmds.latest.pki_enrollment_list.RepOk() as e:
            assert len(e.enrollments) == len(expected_enrollments)

            for res, expected in zip(e.enrollments, expected_enrollments):
                assert res.enrollment_id == expected.enrollment_id
                assert res.submitted_on == expected.submitted_on
                assert res.der_x509_certificate == expected.der_x509_certificate
                assert (
                    res.intermediate_der_x509_certificates
                    == expected.intermediate_der_x509_certificates
                )
                assert res.payload_signature == expected.payload_signature
                assert res.payload_signature_algorithm == expected.payload_signature_algorithm
                assert res.payload == expected.payload

        case _:
            assert False


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_pki_enrollment_list_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    match kind:
        case "never_allowed":
            author = coolorg.bob

        case "no_longer_allowed":
            await bob_becomes_admin_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_profile=UserProfile.STANDARD
            )
            author = coolorg.alice

        case unknown:
            assert False, unknown

    rep = await author.pki_enrollment_list()
    assert rep == authenticated_cmds.latest.pki_enrollment_list.RepAuthorNotAllowed()


async def test_authenticated_pki_enrollment_list_invalid_submitter_x509_certificates(
    coolorg: CoolorgRpcClients, backend: Backend, test_pki: TestPki, xfail_if_postgresql
) -> None:
    bob = test_pki.cert["bob"]
    payload = PkiEnrollmentSubmitPayload(
        verify_key=SigningKey.generate().verify_key,
        public_key=PrivateKey.generate().public_key,
        device_label=DeviceLabel("Dev1"),
    )
    now = DateTime.now()
    enrollment_id = PKIEnrollmentID.new()
    rep = await backend.pki.submit(
        now=now,
        organization_id=coolorg.organization_id,
        enrollment_id=enrollment_id,
        force=True,
        submitter_human_handle=bob.cert_info.human_handle(),
        submitter_der_x509_certificate=bob.der_certificate,
        intermediate_certificates=[],
        submit_payload_signature=b"<a signature>",
        submit_payload_signature_algorithm=PkiSignatureAlgorithm.RSASSA_PSS_SHA256,
        submit_payload=payload.dump(),
    )

    assert rep is None

    assert isinstance(backend.pki, MemoryPkiEnrollmentComponent)
    org = backend.pki._data.organizations[coolorg.organization_id]
    org.pki_enrollments[enrollment_id].submitter_der_x509_certificate = b"<not a valid cer>"

    rep = await coolorg.alice.pki_enrollment_list()
    assert (
        rep == authenticated_cmds.latest.pki_enrollment_list.RepInvalidSubmitterX509Certificates()
    )


async def test_authenticated_pki_enrollment_list_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.pki_enrollment_list()

    await authenticated_http_common_errors_tester(do)


# TODO test when leaf fingerprint not in db ->  #11871

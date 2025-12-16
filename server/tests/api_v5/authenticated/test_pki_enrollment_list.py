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
from tests.api_v5.authenticated.test_pki_enrollment_accept import (
    generate_accept_params,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    Enrollment,
    HttpCommonErrorsTester,
    TestPki,
    bob_becomes_admin_and_changes_alice,
)


async def test_authenticated_pki_enrollment_list_ok(
    coolorg: CoolorgRpcClients, backend: Backend, test_pki: TestPki, clear_pki_certificate
) -> None:
    # 1) Check with no enrollments available

    rep = await coolorg.alice.pki_enrollment_list()
    assert rep == authenticated_cmds.latest.pki_enrollment_list.RepOk(enrollments=[])

    # 2) Add enrollments

    expected_enrollments: list[
        authenticated_cmds.latest.pki_enrollment_list.PkiEnrollmentListItem
    ] = []

    every_cert = [*test_pki.cert.values(), *test_pki.intermediate.values(), *test_pki.root.values()]
    for subject in ("alice", "mallory-encrypt", "bob", "old-boby"):
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

        trustchain = await backend.pki.build_trustchain(
            test_pki.cert[subject].der_certificate, (x.der_certificate for x in every_cert)
        )
        assert isinstance(trustchain, list)

        expected_enrollment_item = (
            authenticated_cmds.latest.pki_enrollment_list.PkiEnrollmentListItem(
                enrollment_id=enrollment_id,
                submitted_on=submitted_on,
                der_x509_certificate=trustchain[0].content,
                intermediate_der_x509_certificates=list(map(lambda x: x.content, trustchain[1:])),
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
            submitter_trustchain=trustchain,
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

        case e:
            print(e)
            assert False

    # 3) Also ensure `ACCEPTED/CANCELLED/REJECTED` enrollments are ignored

    to_accept = expected_enrollments.pop()
    outcome = await backend.pki.accept(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        **generate_accept_params(coolorg, to_accept.enrollment_id, test_pki).__dict__,
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
    trustchain = await backend.pki.build_trustchain(
        canceller_expected_enrollment_item.der_x509_certificate,
        canceller_expected_enrollment_item.intermediate_der_x509_certificates,
    )
    assert isinstance(trustchain, list)
    outcome = await backend.pki.submit(
        now=canceller_submitted_on,
        organization_id=coolorg.organization_id,
        enrollment_id=canceller_enrollment_id,
        force=True,
        submitter_human_handle=human_handle,
        submitter_trustchain=trustchain,
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


async def test_authenticated_pki_enrollment_list_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.pki_enrollment_list()

    await authenticated_http_common_errors_tester(do)


# Not tested if postgre because the database consistency is explicitly enforced
async def test_authenticated_pki_enrollment_list_certificate_not_found(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    existing_enrollment: Enrollment,
    monkeypatch: pytest.MonkeyPatch,
    xfail_if_postgresql,
) -> None:
    async def _mocked_get_cert(*args, **kwargs):
        return None

    monkeypatch.setattr(
        "parsec.components.memory.datamodel.MemoryOrganization.get_cert", _mocked_get_cert
    )
    rep = await coolorg.alice.pki_enrollment_list()
    assert rep == authenticated_cmds.latest.pki_enrollment_list.RepCertificateNotFound()

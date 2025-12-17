# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    PKIEnrollmentID,
    UserProfile,
    authenticated_cmds,
)
from parsec.events import EventPkiEnrollment
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    bob_becomes_admin_and_changes_alice,
)
from tests.common.pki import PkiEnrollment


async def test_authenticated_pki_enrollment_reject_ok(
    coolorg: CoolorgRpcClients, backend: Backend, existing_pki_enrollment: PkiEnrollment
) -> None:
    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.pki_enrollment_reject(
            enrollment_id=existing_pki_enrollment.enrollment_id
        )
        assert rep == authenticated_cmds.latest.pki_enrollment_reject.RepOk()

        await spy.wait_event_occurred(
            EventPkiEnrollment(
                organization_id=coolorg.organization_id,
            )
        )


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_pki_enrollment_reject_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, existing_pki_enrollment: PkiEnrollment, kind: str
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

    rep = await author.pki_enrollment_reject(enrollment_id=existing_pki_enrollment.enrollment_id)
    assert rep == authenticated_cmds.latest.pki_enrollment_reject.RepAuthorNotAllowed()


async def test_authenticated_pki_enrollment_reject_enrollment_no_longer_available(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    existing_pki_enrollment: PkiEnrollment,
) -> None:
    outcome = await backend.pki.reject(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        enrollment_id=existing_pki_enrollment.enrollment_id,
    )
    assert outcome is None

    rep = await coolorg.alice.pki_enrollment_reject(
        enrollment_id=existing_pki_enrollment.enrollment_id
    )
    assert rep == authenticated_cmds.latest.pki_enrollment_reject.RepEnrollmentNoLongerAvailable()


async def test_authenticated_pki_enrollment_reject_enrollment_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.pki_enrollment_reject(enrollment_id=PKIEnrollmentID.new())
    assert rep == authenticated_cmds.latest.pki_enrollment_reject.RepEnrollmentNotFound()


async def test_authenticated_pki_enrollment_reject_http_common_errors(
    coolorg: CoolorgRpcClients,
    existing_pki_enrollment: PkiEnrollment,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.alice.pki_enrollment_reject(
            enrollment_id=existing_pki_enrollment.enrollment_id
        )

    await authenticated_http_common_errors_tester(do)

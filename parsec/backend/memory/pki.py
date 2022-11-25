# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Any, Callable, Coroutine, Dict, List
from collections import defaultdict
import attr
from parsec._parsec import DateTime, EnrollmentID

from parsec.api.protocol import OrganizationID, DeviceID
from parsec.backend.backend_events import BackendEvent
from parsec.backend.memory.user import (
    MemoryUserComponent,
    UserAlreadyExistsError,
    UserActiveUsersLimitReached,
)
from parsec.backend.user_type import User, Device
from parsec.backend.pki import (
    PkiEnrollementEmailAlreadyUsedError,
    PkiEnrollmentIdAlreadyUsedError,
    PkiEnrollmentInfo,
    PkiEnrollmentInfoSubmitted,
    PkiEnrollmentInfoAccepted,
    PkiEnrollmentInfoRejected,
    PkiEnrollmentInfoCancelled,
    PkiEnrollmentListItem,
    BasePkiEnrollmentComponent,
    PkiEnrollmentAlreadyEnrolledError,
    PkiEnrollmentCertificateAlreadySubmittedError,
    PkiEnrollmentNoLongerAvailableError,
    PkiEnrollmentAlreadyExistError,
    PkiEnrollmentActiveUsersLimitReached,
    PkiEnrollmentNotFoundError,
)


@attr.s(slots=True, auto_attribs=True)
class PkiEnrollment:
    enrollment_id: EnrollmentID
    info: PkiEnrollmentInfo
    submitter_der_x509_certificate: bytes
    submit_payload_signature: bytes
    submit_payload: bytes
    accepter: DeviceID | None = None
    accepted: DeviceID | None = None


class MemoryPkiEnrollmentComponent(BasePkiEnrollmentComponent):
    def __init__(self, send_event: Callable[..., Coroutine[Any, Any, None]]) -> None:
        self._send_event = send_event
        self._user_component: MemoryUserComponent | None = None
        self._enrollments: Dict[OrganizationID, List[PkiEnrollment]] = defaultdict(list)

    def register_components(self, user: MemoryUserComponent, **other_components: Any) -> None:
        self._user_component = user

    async def submit(
        self,
        organization_id: OrganizationID,
        enrollment_id: EnrollmentID,
        force: bool,
        submitter_der_x509_certificate: bytes,
        submitter_der_x509_certificate_email: str,
        submit_payload_signature: bytes,
        submit_payload: bytes,
        submitted_on: DateTime,
    ) -> None:
        assert self._user_component is not None

        # Assert enrollment id not used already
        for enrollment in reversed(self._enrollments[organization_id]):
            if enrollment.enrollment_id == enrollment_id:
                raise PkiEnrollmentIdAlreadyUsedError()

        # Try to retrieve the last attempt with this x509 certificate
        for enrollment in reversed(self._enrollments[organization_id]):
            if enrollment.submitter_der_x509_certificate == submitter_der_x509_certificate:
                if isinstance(enrollment.info, PkiEnrollmentInfoSubmitted):
                    # Previous attempt is still pending, overwrite it if force flag is set...
                    if force:
                        enrollment.info = PkiEnrollmentInfoCancelled(
                            enrollment_id=enrollment_id,
                            submitted_on=enrollment.info.submitted_on,
                            cancelled_on=submitted_on,
                        )
                        await self._send_event(
                            BackendEvent.PKI_ENROLLMENTS_UPDATED, organization_id=organization_id
                        )
                    else:
                        # ...otherwise nothing we can do
                        raise PkiEnrollmentCertificateAlreadySubmittedError(
                            submitted_on=enrollment.info.submitted_on
                        )
                elif isinstance(
                    enrollment.info, (PkiEnrollmentInfoRejected, PkiEnrollmentInfoCancelled)
                ):
                    # Previous attempt was unsuccessful, so we are clear to submit a new attempt !
                    pass
                elif isinstance(enrollment.info, PkiEnrollmentInfoAccepted):
                    # Previous attempt end successfully, we are not allowed to submit
                    # unless the created user has been revoked
                    assert enrollment.accepter is not None and enrollment.accepted is not None
                    user = self._user_component._get_user(
                        organization_id=organization_id, user_id=enrollment.accepted.user_id
                    )
                    if not user.is_revoked():
                        raise PkiEnrollmentAlreadyEnrolledError(enrollment.info.accepted_on)
                else:
                    assert False
                # There is no need looking for older enrollments given the
                # last one represent the current state of this x509 certificate.
                break

        # Optional check for client compatibility with version < 2.8.3
        if submitter_der_x509_certificate_email:
            # Assert email not used.
            _, total = self._user_component._find_humans(
                organization_id=organization_id,
                query=submitter_der_x509_certificate_email,
                omit_revoked=True,
            )
            if total:
                raise PkiEnrollementEmailAlreadyUsedError()
        self._enrollments[organization_id].append(
            PkiEnrollment(
                enrollment_id=enrollment_id,
                submitter_der_x509_certificate=submitter_der_x509_certificate,
                submit_payload_signature=submit_payload_signature,
                submit_payload=submit_payload,
                info=PkiEnrollmentInfoSubmitted(
                    enrollment_id=enrollment_id, submitted_on=submitted_on
                ),
            )
        )
        await self._send_event(
            BackendEvent.PKI_ENROLLMENTS_UPDATED, organization_id=organization_id
        )

    async def info(
        self, organization_id: OrganizationID, enrollment_id: EnrollmentID
    ) -> PkiEnrollmentInfo:
        for enrollment in reversed(self._enrollments[organization_id]):
            if enrollment.enrollment_id == enrollment_id:
                return enrollment.info
        else:
            raise PkiEnrollmentNotFoundError()

    async def list(self, organization_id: OrganizationID) -> List[PkiEnrollmentListItem]:
        items = []
        for e in self._enrollments[organization_id]:
            if isinstance(e.info, PkiEnrollmentInfoSubmitted):
                items.append(
                    PkiEnrollmentListItem(
                        enrollment_id=e.enrollment_id,
                        submitted_on=e.info.submitted_on,
                        submitter_der_x509_certificate=e.submitter_der_x509_certificate,
                        submit_payload_signature=e.submit_payload_signature,
                        submit_payload=e.submit_payload,
                    )
                )
        return items

    async def reject(
        self, organization_id: OrganizationID, enrollment_id: EnrollmentID, rejected_on: DateTime
    ) -> None:
        for enrollment in reversed(self._enrollments[organization_id]):
            if enrollment.enrollment_id == enrollment_id:
                if isinstance(enrollment.info, PkiEnrollmentInfoSubmitted):
                    enrollment.info = PkiEnrollmentInfoRejected(
                        enrollment_id=enrollment_id,
                        submitted_on=enrollment.info.submitted_on,
                        rejected_on=rejected_on,
                    )
                    await self._send_event(
                        BackendEvent.PKI_ENROLLMENTS_UPDATED, organization_id=organization_id
                    )
                    return
                else:
                    raise PkiEnrollmentNoLongerAvailableError()

                break

        else:
            raise PkiEnrollmentNotFoundError()

    async def accept(
        self,
        organization_id: OrganizationID,
        enrollment_id: EnrollmentID,
        accepter_der_x509_certificate: bytes,
        accept_payload_signature: bytes,
        accept_payload: bytes,
        accepted_on: DateTime,
        user: User,
        first_device: Device,
    ) -> None:
        assert self._user_component is not None

        for enrollment in reversed(self._enrollments[organization_id]):
            if enrollment.enrollment_id == enrollment_id:
                if not isinstance(enrollment.info, PkiEnrollmentInfoSubmitted):
                    raise PkiEnrollmentNoLongerAvailableError()

                try:
                    await self._user_component.create_user(
                        organization_id=organization_id, user=user, first_device=first_device
                    )

                except UserAlreadyExistsError as exc:
                    raise PkiEnrollmentAlreadyExistError from exc

                except UserActiveUsersLimitReached as exc:
                    raise PkiEnrollmentActiveUsersLimitReached from exc

                enrollment.info = PkiEnrollmentInfoAccepted(
                    enrollment_id=enrollment_id,
                    submitted_on=enrollment.info.submitted_on,
                    accepted_on=accepted_on,
                    accepter_der_x509_certificate=accepter_der_x509_certificate,
                    accept_payload_signature=accept_payload_signature,
                    accept_payload=accept_payload,
                )
                # Certifier is empty only for organization bootstrap
                assert user.user_certifier is not None
                enrollment.accepter = user.user_certifier
                enrollment.accepted = first_device.device_id

                await self._send_event(
                    BackendEvent.PKI_ENROLLMENTS_UPDATED, organization_id=organization_id
                )
                break

        else:
            raise PkiEnrollmentNotFoundError()

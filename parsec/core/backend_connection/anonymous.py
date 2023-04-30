# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from structlog import get_logger

from parsec import FEATURE_FLAGS
from parsec._parsec import (
    EnrollmentID,
    ProtocolError,
    VerifyKey,
)
from parsec._version import __version__
from parsec.api.protocol import (
    OrganizationBootstrapRep,
    OrganizationBootstrapRepBadTimestamp,
    OrganizationBootstrapRepUnknownStatus,
    OrganizationBootstrapReq,
    PkiEnrollmentInfoRep,
    PkiEnrollmentInfoRepUnknownStatus,
    PkiEnrollmentInfoReq,
    PkiEnrollmentSubmitRep,
    PkiEnrollmentSubmitRepUnknownStatus,
    PkiEnrollmentSubmitReq,
)
from parsec.core.backend_connection import (
    BackendNotAvailable,
    BackendOutOfBallparkError,
    BackendProtocolError,
)
from parsec.core.backend_connection.transport import http_request
from parsec.core.types import BackendOrganizationBootstrapAddr, BackendPkiEnrollmentAddr

if FEATURE_FLAGS["UNSTABLE_OXIDIZED_CLIENT_CONNECTION"]:
    from parsec._parsec import AnonymousCmds


logger = get_logger()


REQUEST_HEADERS = {
    "Content-Type": "application/msgpack",
    "User-Agent": f"parsec/{__version__}",
}


async def _anonymous_cmd(
    addr: BackendPkiEnrollmentAddr | BackendOrganizationBootstrapAddr,
    raw_req: bytes,
) -> bytes:
    """
    Raises:
        BackendNotAvailable
    """
    url = addr.to_http_domain_url(f"/anonymous/{addr.organization_id.str}")
    try:
        return await http_request(url=url, method="POST", headers=REQUEST_HEADERS, data=raw_req)
    except OSError as exc:
        raise BackendNotAvailable(exc) from exc


async def pki_enrollment_submit(
    addr: BackendPkiEnrollmentAddr,
    enrollment_id: EnrollmentID,
    force: bool,
    submitter_der_x509_certificate: bytes,
    submitter_der_x509_certificate_email: str,
    submit_payload_signature: bytes,
    submit_payload: bytes,
) -> PkiEnrollmentSubmitRep:
    if FEATURE_FLAGS["UNSTABLE_OXIDIZED_CLIENT_CONNECTION"]:
        rep = await AnonymousCmds(addr).pki_enrollment_submit(
            enrollment_id=enrollment_id,
            force=force,
            submitter_der_x509_certificate=submitter_der_x509_certificate,
            submitter_der_x509_certificate_email=submitter_der_x509_certificate_email,
            submit_payload_signature=submit_payload_signature,
            submit_payload=submit_payload,
        )
    else:
        req = PkiEnrollmentSubmitReq(
            enrollment_id=enrollment_id,
            force=force,
            submitter_der_x509_certificate=submitter_der_x509_certificate,
            submitter_der_x509_certificate_email=submitter_der_x509_certificate_email,
            submit_payload_signature=submit_payload_signature,
            submit_payload=submit_payload,
        )
        raw_rep = await _anonymous_cmd(addr, req.dump())
        try:
            rep = PkiEnrollmentSubmitRep.load(raw_rep)

        except ProtocolError as exc:
            raise BackendProtocolError("Invalid response data") from exc

    # `invalid_msg_format` is a special case (it is returned only in case the request was invalid) so it is
    # not included among the protocol json schema's regular response statuses
    if isinstance(rep, PkiEnrollmentSubmitRepUnknownStatus) and rep.status == "invalid_msg_format":
        logger.error(
            "Invalid request data according to backend", cmd="pki_enrollment_submit", rep=rep
        )
        raise BackendProtocolError("Invalid request data according to backend")

    return rep


async def pki_enrollment_info(
    addr: BackendPkiEnrollmentAddr, enrollment_id: EnrollmentID
) -> PkiEnrollmentInfoRep:
    if FEATURE_FLAGS["UNSTABLE_OXIDIZED_CLIENT_CONNECTION"]:
        rep = await AnonymousCmds(addr).pki_enrollment_info(enrollment_id=enrollment_id)
    else:
        req = PkiEnrollmentInfoReq(
            enrollment_id=enrollment_id,
        )
        raw_rep = await _anonymous_cmd(addr, req.dump())
        try:
            rep = PkiEnrollmentInfoRep.load(raw_rep)

        except ProtocolError as exc:
            raise BackendProtocolError("Invalid response data") from exc

    # `invalid_msg_format` is a special case (it is returned only in case the request was invalid) so it is
    # not included among the protocol json schema's regular response statuses
    if isinstance(rep, PkiEnrollmentInfoRepUnknownStatus) and rep.status == "invalid_msg_format":
        logger.error(
            "Invalid request data according to backend", cmd="pki_enrollment_info", rep=rep
        )
        raise BackendProtocolError("Invalid request data according to backend")

    return rep


async def organization_bootstrap(
    addr: BackendOrganizationBootstrapAddr,
    root_verify_key: VerifyKey,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
    sequester_authority_certificate: bytes | None,
) -> OrganizationBootstrapRep:
    if FEATURE_FLAGS["UNSTABLE_OXIDIZED_CLIENT_CONNECTION"]:
        rep = await AnonymousCmds(addr).organization_bootstrap(
            bootstrap_token=addr.token,
            root_verify_key=root_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
            sequester_authority_certificate=sequester_authority_certificate,
        )
    else:
        req = OrganizationBootstrapReq(
            bootstrap_token=addr.token,
            root_verify_key=root_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
            sequester_authority_certificate=sequester_authority_certificate,
        )
        raw_rep = await _anonymous_cmd(addr, req.dump())
        try:
            rep = OrganizationBootstrapRep.load(raw_rep)

        except ProtocolError as exc:
            raise BackendProtocolError("Invalid response data") from exc

    # `invalid_msg_format` is a special case (it is returned only in case the request was invalid) so it is
    # not included among the protocol json schema's regular response statuses
    if (
        isinstance(rep, OrganizationBootstrapRepUnknownStatus)
        and rep.status == "invalid_msg_format"
    ):
        logger.error(
            "Invalid request data according to backend", cmd="organization_bootstrap", rep=rep
        )
        raise BackendProtocolError("Invalid request data according to backend")

    elif isinstance(rep, OrganizationBootstrapRepBadTimestamp):
        raise BackendOutOfBallparkError(rep)

    return rep

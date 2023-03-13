# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import cast

from structlog import get_logger

from parsec import FEATURE_FLAGS
from parsec._parsec import (
    EnrollmentID,
    OrganizationBootstrapRep,
    OrganizationBootstrapRepBadTimestamp,
    OrganizationBootstrapRepUnknownStatus,
    OrganizationID,
    PkiEnrollmentInfoRep,
    PkiEnrollmentInfoRepUnknownStatus,
    PkiEnrollmentSubmitRep,
    PkiEnrollmentSubmitRepUnknownStatus,
    ProtocolError,
    VerifyKey,
)
from parsec._version import __version__
from parsec.api.protocol import (
    organization_bootstrap_serializer,
    pki_enrollment_info_serializer,
    pki_enrollment_submit_serializer,
)
from parsec.api.protocol.base import ApiCommandSerializer
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
    serializer: ApiCommandSerializer,
    addr: BackendPkiEnrollmentAddr | BackendOrganizationBootstrapAddr,
    organization_id: OrganizationID,
    **req: object,
) -> PkiEnrollmentSubmitRep | PkiEnrollmentInfoRep | OrganizationBootstrapRep:
    """
    Raises:
        BackendNotAvailable
        BackendProtocolError
    """
    # Be careful, `serializer.req_dumps` pops the `cmd` key
    cmd = req["cmd"]

    logger.info("Request", cmd=cmd)
    try:
        raw_req = serializer.req_dumps(req)

    except ProtocolError as exc:
        logger.exception("Invalid request data", cmd=cmd, error=exc)
        raise BackendProtocolError("Invalid request data") from exc

    url = addr.to_http_domain_url(f"/anonymous/{organization_id.str}")
    try:
        raw_rep = await http_request(url=url, method="POST", headers=REQUEST_HEADERS, data=raw_req)
    except OSError as exc:
        logger.debug("Request failed (backend not available)", cmd=cmd, exc_info=exc)
        raise BackendNotAvailable(exc) from exc

    try:
        rep = serializer.rep_loads(raw_rep)

    except ProtocolError as exc:
        logger.exception("Invalid response data", cmd=cmd, error=exc)
        raise BackendProtocolError("Invalid response data") from exc

    return rep


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
        rep = cast(
            PkiEnrollmentSubmitRep,
            await _anonymous_cmd(
                serializer=pki_enrollment_submit_serializer,
                cmd="pki_enrollment_submit",
                addr=addr,
                organization_id=addr.organization_id,
                enrollment_id=enrollment_id,
                force=force,
                submitter_der_x509_certificate=submitter_der_x509_certificate,
                submitter_der_x509_certificate_email=submitter_der_x509_certificate_email,
                submit_payload_signature=submit_payload_signature,
                submit_payload=submit_payload,
            ),
        )

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
        rep = cast(
            PkiEnrollmentInfoRep,
            await _anonymous_cmd(
                serializer=pki_enrollment_info_serializer,
                cmd="pki_enrollment_info",
                addr=addr,
                organization_id=addr.organization_id,
                enrollment_id=enrollment_id,
            ),
        )

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
        rep = cast(
            OrganizationBootstrapRep,
            await _anonymous_cmd(
                organization_bootstrap_serializer,
                cmd="organization_bootstrap",
                addr=addr,
                organization_id=addr.organization_id,
                bootstrap_token=addr.token,
                root_verify_key=root_verify_key,
                user_certificate=user_certificate,
                device_certificate=device_certificate,
                redacted_user_certificate=redacted_user_certificate,
                redacted_device_certificate=redacted_device_certificate,
                sequester_authority_certificate=sequester_authority_certificate,
            ),
        )

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

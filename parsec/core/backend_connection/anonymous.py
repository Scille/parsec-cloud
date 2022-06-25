# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from uuid import UUID
from structlog import get_logger

from parsec.crypto import VerifyKey
from parsec.api.protocol import (
    OrganizationID,
    ProtocolError,
    pki_enrollment_submit_serializer,
    pki_enrollment_info_serializer,
    organization_bootstrap_serializer,
)
from parsec.core.types import (
    BackendAddr,
    BackendPkiEnrollmentAddr,
    BackendOrganizationBootstrapAddr,
)
from parsec.core.backend_connection.transport import http_request
from parsec.core.backend_connection.exceptions import (
    BackendNotAvailable,
    BackendProtocolError,
    BackendOutOfBallparkError,
)

logger = get_logger()


async def _anonymous_cmd(
    serializer, addr: BackendAddr, organization_id: OrganizationID, **req
) -> dict:
    """
    Raises:
        BackendNotAvailable
        BackendProtocolError
    """
    logger.info("Request", cmd=req["cmd"])

    try:
        raw_req = serializer.req_dumps(req)

    except ProtocolError as exc:
        logger.exception("Invalid request data", cmd=req["cmd"], error=exc)
        raise BackendProtocolError("Invalid request data") from exc

    url = addr.to_http_domain_url(f"/anonymous/{organization_id}")
    try:
        raw_rep = await http_request(url=url, method="POST", data=raw_req)
    except OSError as exc:
        logger.debug("Request failed (backend not available)", cmd=req["cmd"], exc_info=exc)
        raise BackendNotAvailable(exc) from exc

    try:
        rep = serializer.rep_loads(raw_rep)

    except ProtocolError as exc:
        logger.exception("Invalid response data", cmd=req["cmd"], error=exc)
        raise BackendProtocolError("Invalid response data") from exc

    if rep["status"] == "invalid_msg_format":
        logger.error("Invalid request data according to backend", cmd=req["cmd"], rep=rep)
        raise BackendProtocolError("Invalid request data according to backend")

    if rep["status"] == "bad_timestamp":
        raise BackendOutOfBallparkError(rep)

    return rep


async def pki_enrollment_submit(
    addr: BackendPkiEnrollmentAddr,
    enrollment_id: UUID,
    force: bool,
    submitter_der_x509_certificate: bytes,
    submitter_der_x509_certificate_email: str,
    submit_payload_signature: bytes,
    submit_payload: bytes,
) -> dict:
    return await _anonymous_cmd(
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
    )


async def pki_enrollment_info(addr: BackendPkiEnrollmentAddr, enrollment_id: UUID) -> dict:
    return await _anonymous_cmd(
        serializer=pki_enrollment_info_serializer,
        cmd="pki_enrollment_info",
        addr=addr,
        organization_id=addr.organization_id,
        enrollment_id=enrollment_id,
    )


async def organization_bootstrap(
    addr: BackendOrganizationBootstrapAddr,
    root_verify_key: VerifyKey,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
    sequester_authority_key_certificate: bytes,
) -> dict:
    return await _anonymous_cmd(
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
        sequester_authority_key_certificate=sequester_authority_key_certificate,
    )

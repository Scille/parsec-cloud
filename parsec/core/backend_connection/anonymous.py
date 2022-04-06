# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from uuid import UUID
from typing import Optional
from structlog import get_logger
from urllib.request import Request, urlopen
from urllib.error import URLError
import trio

from parsec.api.protocol import (
    OrganizationID,
    ProtocolError,
    pki_enrollment_submit_serializer,
    pki_enrollment_info_serializer,
)
from parsec.api.transport import TransportError
from parsec.core.types import BackendAddr, BackendPkiEnrollmentAddr
from parsec.core.backend_connection.exceptions import BackendNotAvailable, BackendProtocolError

logger = get_logger()


async def _http_request(url: str, method: str, data: Optional[bytes] = None) -> bytes:
    def _do_req() -> bytes:
        req = Request(url=url, method=method, data=data)
        try:
            with urlopen(req) as rep:
                return rep.read()

        except URLError as exc:
            raise TransportError(f"Bad response from backend: {exc}") from exc

    return await trio.to_thread.run_sync(_do_req)


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
        raw_rep = await _http_request(url=url, method="POST", data=raw_req)

    except TransportError as exc:
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

    return rep


async def pki_enrollment_submit(
    addr: BackendPkiEnrollmentAddr,
    enrollment_id: UUID,
    force: bool,
    submitter_der_x509_certificate: bytes,
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

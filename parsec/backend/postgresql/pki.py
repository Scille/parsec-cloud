# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from collections import namedtuple
from typing import List, Optional, Tuple

import pendulum
from parsec.api.data.pki import PkiReply, PkiRequest
from parsec.backend.pki import (
    BasePkiCertificateComponent,
    PkiCertificateAlreadyEnrolledError,
    PkiCertificateAlreadyRequestedError,
    PkiCertificateNotFoundError,
    PkiCertificateRequestNotFoundError,
)
from parsec.backend.postgresql import PGHandler
from parsec.backend.postgresql.utils import Q
from pendulum import DateTime

_q_insert_certificate = Q(
    f"""
    INSERT INTO pki_certificate (certificate_id, request_id, request_timestamp, request_object)
    VALUES (
        $certificate_id,
        $request_id,
        $request_timestamp,
        $request_object,

    )
    """
)

# TODO update request shall reset reply ?
_q_update_certificate_request = Q(
    f"""
    UPDATE pki_certificate
    SET
        request_id = $request_id,
        request_timestamp=$request_timestamp,
        request_object=$request_object,
        reply_object=$reply_object,
        reply_timesamp=$reply_timestamp,
        reply_user_id=reply_user_id,
    WHERE certificate_id=certificate_id
"""
)

_q_update_certificate_reply = Q(
    f"""
    UPDATE pki_certificate
    SET
        reply_object=$reply_object,
        reply_timesamp=$reply_timestamp,
        reply_user_id=reply_user_id,
    WHERE certificate_id=certificate_id
    """
)

_q_get_certificate = Q(
    f"""
    SELECT *
    FROM pki_certificate
    WHERE
        certificate_id=$certificate_id
    ORDER BY _id ASC
    """
)

_q_get_certificates = Q(
    f"""
    SELECT *
    FROM pki_certificate
    ORDER BY _id ASC
    """
)


DBPkiCertificate = namedtuple(
    "DBPkiCertificate",
    [
        "db_id",
        "certificate_id",
        "request_id",
        "request_timestamp",
        "request_object",
        "reply_user_id",
        "reply_timestamp",
        "reply_object",
    ],
)


def build_certificate_from_db(entry) -> DBPkiCertificate:
    return DBPkiCertificate(
        entry[0], entry[1], entry[2], entry[3], entry[4], entry[5], entry[6], entry[7]
    )


class PGCertificateComponent(BasePkiCertificateComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def pki_enrollment_request(
        self,
        certificate_id: str,
        request_id: str,
        request_object: PkiRequest,
        force_flag: bool = False,
    ) -> DateTime:

        async with self.dbh.pool.acquire() as conn, conn.transaction():
            data = await conn.fetch(*_q_get_certificate(certificate_id=certificate_id))
            if len(data):
                existing_certificate = build_certificate_from_db(data[0])
                if existing_certificate.reply_object and existing_certificate.reply_user_id:
                    raise PkiCertificateAlreadyEnrolledError(
                        existing_certificate.request_timestamp,
                        f"Certificate {certificate_id} already attributed",
                    )

                if not force_flag:
                    raise PkiCertificateAlreadyRequestedError(
                        existing_certificate.reply_timestamp,
                        f"Certificate {certificate_id} already used in request {request_id}",
                    )
                else:
                    request_timestamp = pendulum.now()
                    await conn.fetchval(
                        *_q_update_certificate_request(
                            certificate_id=certificate_id,
                            request_id=request_id,
                            request_timestamp=request_timestamp,
                            request_object=request_object.dump(),
                        )
                    )
                    return request_timestamp

            ## TODO # Check human handle already used
            ## for pki_certificate in self._pki_certificates.values():
            ##     if (
            ##         pki_certificate.request_object.requested_human_handle
            ##         == request_object.requested_human_handle
            ##     ):
            ##         raise PkiCertificateEmailAlreadyAttributedError(
            ##             f"email f{request_object.requested_human_handle} already attributed"
            ##         )
            else:
                request_timestamp = pendulum.now()
                await conn.fetchval(
                    *_q_insert_certificate(
                        certificate_id=certificate_id,
                        request_id=request_id,
                        request_timestamp=request_timestamp,
                        request_object=request_object.dump(),
                    )
                )
                return request_timestamp

    async def pki_enrollment_get_requests(self) -> List[Tuple[str, str, PkiRequest]]:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            data = await conn.fetch(*_q_get_certificates())

        return [(d[1], d[2], d[3]) for d in data]

    async def pki_enrollment_reply(
        self,
        certificate_id: str,
        request_id: str,
        reply_object: PkiReply,
        user_id: Optional[str] = None,
    ) -> DateTime:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            data = await conn.fetch(*_q_get_certificate(certificate_id=certificate_id))
            if not len(data):
                raise PkiCertificateNotFoundError(f"Certificate {certificate_id} not found")
            pki_certificate = build_certificate_from_db(data[0])
            if pki_certificate.request_id != request_id:
                raise PkiCertificateRequestNotFoundError(
                    f"Request {request_id} not found for certificate {certificate_id}"
                )
            reply_timestamp = pendulum.now()
            await conn.fetchval(
                *_q_update_certificate_reply(
                    certificate_id=certificate_id,
                    reply_object=reply_object.dump(),
                    reply_user_id=user_id,
                    reply_timestamp=reply_timestamp,
                )
            )
            return reply_timestamp

    async def pki_enrollment_get_reply(
        self, certificate_id, request_id
    ) -> Tuple[Optional[PkiReply], Optional[DateTime], DateTime, Optional[str]]:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            data = await conn.fetch(*_q_get_certificate(certificate_id=certificate_id))
            if not len(data):
                raise PkiCertificateNotFoundError(f"Certificate {certificate_id} not found")
            pki_certificate = build_certificate_from_db(data[0])
            if pki_certificate.request_id != request_id:
                raise PkiCertificateRequestNotFoundError(
                    f"Request {request_id} not found for certificate {certificate_id}"
                )
        return (
            data[0][7],  # reply_object
            data[0][6],  # reply_timestamp
            data[0][3],  # request_timestamp
            data[0][5],  # user_id
        )

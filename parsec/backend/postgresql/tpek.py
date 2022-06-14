# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


from uuid import UUID
from parsec.api.protocol.tpek import TpekServiceType
from parsec.api.protocol.types import OrganizationID
from parsec.backend.postgresql.handler import PGHandler

from parsec.backend.tpek import BaseTpekComponent

from parsec.backend.postgresql.utils import Q, q_organization_internal_id

_q_create_tpek_service = Q(
    f"""
    INSERT  INTO tpek(
        service_type,
        service_id,
        organization,
        encryption_key
    )
    VALUES(
        $service_type,
        $service_id,
        { q_organization_internal_id("$organization_id") },
        $encryption_key
    )

    """
)

_q_update_tpek_service = Q(
    f"""
    UPDATE tpek
    SET
        encryption_key=$encryption_key,
        service_type=$service_type
    WHERE
        organization = { q_organization_internal_id("$organization_id") }
        AND service_id=$service_id

    """
)

_q_get_tpek_service_for_update = Q(
    f"""
    SELECT _id
    FROM tpek
    WHERE
        organization={q_organization_internal_id("$organization_id") }
        AND service_id = $service_id
    FOR UPDATE
    LIMIT 1
    """
)

_q_get_organisation_tpek_verify_key = Q(
    f"""
    SELECT tpek_verify_key
    FROM organization
    WHERE organization.organization_id={q_organization_internal_id("$organization_id")}
    """
)


class PGPTpekComponent(BaseTpekComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def register_service(
        self,
        organization_id: OrganizationID,
        service_id: UUID,
        service_type: TpekServiceType,
        tpek_encryption_key_payload: bytes,
        tpek_encryption_key_payload_signature: bytes,
    ):
        pass

        # async with self.dbh.pool.acquire() as conn, conn.transaction():
        #     row = await conn.fetchrow(
        #         *_q_get_organisation_tpek_verify_key(organization_id=organization_id.str)
        #     )
        #     if not row:
        #         # TODO:error
        #         pass
        #     tpek_verify_key = load_der_public_key(row["tpek_verify_key"])
        #     verify_tpek_der_signature(
        #         tpek_verify_key,
        #         tpek_certificate_signed_encryption_key,
        #         tpek_certificate_encryption_key.unwrap().dump(),
        #     )
        #     row = await conn.fetchrow(
        #         *_q_get_tpek_service_for_update(
        #             organization_id=organization_id.str, sercice_id=service_id
        #         )
        #     )
        #     if not row:
        #         result = await conn.execute(
        #             *_q_create_tpek_service(
        #                 organization_id=organization_id.str,
        #                 service_type=service_type.value,
        #                 service_id=service_id,
        #                 encryption_key=tpek_certificate,
        #             )
        #         )
        #         if result != "INSERT 0 1":
        #             raise TpekError(f"Insertion Error: {result}")
        #     else:
        #         result = await conn.execute(
        #             *_q_update_tpek_service(
        #                 service_type=service_type.value, encryption_key=tpek_certificate
        #             )
        #         )
        #         if result != "UPDATE 1":
        #             raise TpekError(f"Update Error: {result}")

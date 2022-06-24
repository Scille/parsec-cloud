# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


from uuid import UUID

import pendulum
from parsec.api.protocol.sequester import SequesterServiceType
from parsec.api.protocol.types import OrganizationID
from parsec.backend.postgresql.handler import PGHandler

from parsec.backend.sequester import BaseSequesterComponent, SequesterError

from parsec.backend.postgresql.utils import Q, q_organization_internal_id
from parsec.sequester_crypto import load_sequester_public_key

_q_create_sequester_service = Q(
    f"""
    INSERT  INTO sequester(
        service_type,
        service_id,
        organization,
        encryption_key_certificate,
        encryption_key_certificate_signature,
        created_on
    )
    VALUES(
        $service_type,
        $service_id,
        { q_organization_internal_id("$organization_id") },
        $encryption_key_certificate,
        $encryption_key_certificate_signature,
        $created_on
    )

    """
)

_q_update_sequester_service = Q(
    f"""
    UPDATE sequester
    SET
        encryption_key_certificate=$encryption_key_certificate,
        encryption_key_certificate_signature=$encryption_key_certificate_signature,
        service_type=$service_type
    WHERE
        organization = { q_organization_internal_id("$organization_id") }
        AND service_id=$service_id

    """
)

_q_get_sequester_service_for_update = Q(
    f"""
    SELECT _id
    FROM sequester_service
    WHERE
        organization={q_organization_internal_id("$organization_id") }
        AND service_id = $service_id
    FOR UPDATE
    LIMIT 1
    """
)

_q_get_organisation_sequester_verify_key = Q(
    f"""
    SELECT sequester_verify_key
    FROM organization
    WHERE organization.organization_id={q_organization_internal_id("$organization_id")}
    """
)


class PGPSequesterComponent(BaseSequesterComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh

    async def _register_service(
        self,
        organization_id: OrganizationID,
        service_id: UUID,
        service_type: SequesterServiceType,
        sequester_encryption_certificate: bytes,
        sequester_encryption_certificate_signature: bytes,
    ):

        async with self.dbh.pool.acquire() as conn, conn.transaction():
            row = await conn.fetchrow(
                *_q_get_organisation_sequester_verify_key(organization_id=organization_id.str)
            )
            if not row:
                # TODO:error
                pass
            sequester_verify_key = load_sequester_public_key(row["sequester_verify_key"])

            # TODO: handle errors
            self.verify_sequester_signature(
                sequester_verify_key,
                sequester_encryption_certificate,
                sequester_encryption_certificate_signature,
            )

            row = await conn.fetchrow(
                *_q_get_sequester_service_for_update(
                    organization_id=organization_id.str, sercice_id=service_id
                )
            )
            # New sequester kay
            if not row:
                result = await conn.execute(
                    *_q_create_sequester_service(
                        organization_id=organization_id.str,
                        service_type=service_type.value,
                        service_id=service_id,
                        encryption_key_certificate=sequester_encryption_certificate,
                        encryption_key_certificate_signature=sequester_encryption_certificate_signature,
                        created_on=pendulum.now(),
                    )
                )
                if result != "INSERT 0 1":
                    raise SequesterError(f"Insertion Error: {result}")
            else:
                result = await conn.execute(
                    *_q_update_sequester_service(
                        service_type=service_type.value,
                        encryption_key_certificate_signature=sequester_encryption_certificate_signature,
                    )
                )
                if result != "UPDATE 1":
                    raise SequesterError(f"Update Error: {result}")

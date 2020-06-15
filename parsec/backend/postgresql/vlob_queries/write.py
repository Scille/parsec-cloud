# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

    @retry_on_unique_violation
    async def update(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        encryption_revision: int,
        vlob_id: UUID,
        version: int,
        timestamp: pendulum.Pendulum,
        blob: bytes,
    ) -> None:
        async with self.dbh.pool.acquire() as conn, conn.transaction():

            realm_id = await _get_realm_id_from_vlob_id(conn, organization_id, vlob_id)
            await _check_realm_and_write_access(
                conn, organization_id, author, realm_id, encryption_revision
            )

            query = """
SELECT
    version,
    created_on
FROM vlob_atom
WHERE
    organization = ({})
    AND vlob_id = $2
ORDER BY version DESC LIMIT 1
""".format(
                q_organization_internal_id(Parameter("$1"))
            )

            previous = await conn.fetchrow(query, organization_id, vlob_id)
            if not previous:
                raise VlobNotFoundError(f"Vlob `{vlob_id}` doesn't exist")

            elif previous["version"] != version - 1:
                raise VlobVersionError()

            elif previous["created_on"] > timestamp:
                raise VlobTimestampError()

            query = """
INSERT INTO vlob_atom (
    organization,
    vlob_encryption_revision,
    vlob_id,
    version,
    blob,
    size,
    author,
    created_on
)
SELECT
    ({}),
    ({}),
    $5,
    $9,
    $6,
    $7,
    ({}),
    $8
RETURNING _id
""".format(
                q_organization_internal_id(Parameter("$1")),
                q_vlob_encryption_revision_internal_id(
                    organization_id=Parameter("$1"),
                    realm_id=Parameter("$3"),
                    encryption_revision=Parameter("$4"),
                ),
                q_device_internal_id(organization_id=Parameter("$1"), device_id=Parameter("$2")),
            )

            try:
                vlob_atom_internal_id = await conn.fetchval(
                    query,
                    organization_id,
                    author,
                    realm_id,
                    encryption_revision,
                    vlob_id,
                    blob,
                    len(blob),
                    timestamp,
                    version,
                )

            except UniqueViolationError:
                # Should not occurs in theory given we are in a transaction
                raise VlobVersionError()

            await _vlob_updated(
                conn, vlob_atom_internal_id, organization_id, author, realm_id, vlob_id, version
            )

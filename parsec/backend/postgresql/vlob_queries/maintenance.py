# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

_q_query_maintenance_get_reencryption_batch

async def query_maintenance_get_reencryption_batch(
    self,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: UUID,
    encryption_revision: int,
    size: int,
) -> List[Tuple[UUID, int, bytes]]:
        async with conn.transaction():

        await _check_realm_and_maintenance_access(
            conn, organization_id, author, realm_id, encryption_revision
        )

    query = """
WITH cte_to_encrypt AS (
    SELECT vlob_id, version, blob
    FROM vlob_atom
    WHERE vlob_encryption_revision = ({})
),
cte_encrypted AS (
    SELECT vlob_id, version
    FROM vlob_atom
    WHERE vlob_encryption_revision = ({})
)
SELECT
    cte_to_encrypt.vlob_id,
    cte_to_encrypt.version,
    blob
FROM cte_to_encrypt
LEFT JOIN cte_encrypted
ON cte_to_encrypt.vlob_id = cte_encrypted.vlob_id AND cte_to_encrypt.version = cte_encrypted.version
WHERE cte_encrypted.vlob_id IS NULL
LIMIT $4
""".format(
                q_vlob_encryption_revision_internal_id(
                    organization_id=Parameter("$1"),
                    realm_id=Parameter("$2"),
                    encryption_revision=Parameter("$3") - 1,
                ),
                q_vlob_encryption_revision_internal_id(
                    organization_id=Parameter("$1"),
                    realm_id=Parameter("$2"),
                    encryption_revision=Parameter("$3"),
                ),
            )

            rep = await conn.fetch(query, organization_id, realm_id, encryption_revision, size)
            return [(row["vlob_id"], row["version"], row["blob"]) for row in rep]

    async def maintenance_save_reencryption_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: UUID,
        encryption_revision: int,
        batch: List[Tuple[UUID, int, bytes]],
    ) -> Tuple[int, int]:
        async with self.dbh.pool.acquire() as conn, conn.transaction():

            await _check_realm_and_maintenance_access(
                conn, organization_id, author, realm_id, encryption_revision
            )
            for vlob_id, version, blob in batch:
                query = """
INSERT INTO vlob_atom(
    organization,
    vlob_encryption_revision,
    vlob_id,
    version,
    blob,
    size,
    author,
    created_on,
    deleted_on
)
SELECT
    organization,
    ({}),
    $3,
    $4,
    $6,
    $7,
    author,
    created_on,
    deleted_on
FROM vlob_atom
WHERE
    organization = ({})
    AND vlob_id = $3
    AND version = $4
ON CONFLICT DO NOTHING
""".format(
                    q_vlob_encryption_revision_internal_id(
                        organization_id=Parameter("$1"),
                        realm_id=Parameter("$2"),
                        encryption_revision=Parameter("$5"),
                    ),
                    q_organization_internal_id(Parameter("$1")),
                )

                await conn.execute(
                    query,
                    organization_id,
                    realm_id,
                    vlob_id,
                    version,
                    encryption_revision,
                    blob,
                    len(blob),
                )

            query = """
SELECT (
    SELECT COUNT(*)
    FROM vlob_atom
    WHERE vlob_encryption_revision = ({})
),
(
    SELECT COUNT(*)
    FROM vlob_atom
    WHERE vlob_encryption_revision = ({})
)
""".format(
                q_vlob_encryption_revision_internal_id(
                    organization_id=Parameter("$1"),
                    realm_id=Parameter("$2"),
                    encryption_revision=Parameter("$3") - 1,
                ),
                q_vlob_encryption_revision_internal_id(
                    organization_id=Parameter("$1"),
                    realm_id=Parameter("$2"),
                    encryption_revision=Parameter("$3"),
                ),
            )

            rep = await conn.fetchrow(query, organization_id, realm_id, encryption_revision)

            return rep[0], rep[1]

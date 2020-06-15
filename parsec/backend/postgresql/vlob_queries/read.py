# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

_q_read = Q(
    f"""
"""
)


async def query_read(
    conn,
    organization_id: OrganizationID,
    author: DeviceID,
    encryption_revision: int,
    vlob_id: UUID,
    version: Optional[int] = None,
    timestamp: Optional[pendulum.Pendulum] = None,
) -> Tuple[int, bytes, DeviceID, pendulum.Pendulum]:

    async with conn.transaction():

        realm_id = await _get_realm_id_from_vlob_id(conn, organization_id, vlob_id)
        await _check_realm_and_read_access(
            conn, organization_id, author, realm_id, encryption_revision
        )

        if version is None:
            if timestamp is None:
                    query = """
SELECT
    version,
    blob,
    ({}) as author,
    created_on
FROM vlob_atom
WHERE
    vlob_encryption_revision = ({})
    AND vlob_id = $4
ORDER BY version DESC
LIMIT 1
""".format(
                        q_device(_id=Parameter("author")).select("device_id"),
                        q_vlob_encryption_revision_internal_id(
                            organization_id=Parameter("$1"),
                            realm_id=Parameter("$2"),
                            encryption_revision=Parameter("$3"),
                        ),
                    )

                    data = await conn.fetchrow(
                        query, organization_id, realm_id, encryption_revision, vlob_id
                    )
                    assert data  # _get_realm_id_from_vlob_id checks vlob presence

                else:
                    query = """
SELECT
    version,
    blob,
    ({}) as author,
    created_on
FROM vlob_atom
WHERE
    vlob_encryption_revision = ({})
    AND vlob_id = $4
    AND created_on <= $5
ORDER BY version DESC
LIMIT 1
""".format(
                        q_device(_id=Parameter("author")).select("device_id"),
                        q_vlob_encryption_revision_internal_id(
                            organization_id=Parameter("$1"),
                            realm_id=Parameter("$2"),
                            encryption_revision=Parameter("$3"),
                        ),
                    )

                    data = await conn.fetchrow(
                        query, organization_id, realm_id, encryption_revision, vlob_id, timestamp
                    )
                    if not data:
                        raise VlobVersionError()

            else:
                query = """
SELECT
    version,
    blob,
    ({}) as author,
    created_on
FROM vlob_atom
WHERE
    vlob_encryption_revision = ({})
    AND vlob_id = $4
    AND version = $5
""".format(
                    q_device(_id=Parameter("author")).select("device_id"),
                    q_vlob_encryption_revision_internal_id(
                        organization_id=Parameter("$1"),
                        realm_id=Parameter("$2"),
                        encryption_revision=Parameter("$3"),
                    ),
                )

                data = await conn.fetchrow(
                    query, organization_id, realm_id, encryption_revision, vlob_id, version
                )
                if not data:
                    raise VlobVersionError()

        return list(data)


_q_list_versions = Q(
    f"""
SELECT
    version,
    { q_device(_id="vlob_atom.author", select="device_id") } as author,
    created_on
FROM vlob_atom
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND vlob_id = $vlob_id
ORDER BY version DESC
"""
)


async def query_list_versions(
    conn, organization_id: OrganizationID, author: DeviceID, vlob_id: UUID
) -> Dict[int, Tuple[pendulum.Pendulum, DeviceID]]:
    async with conn.transaction():

        realm_id = await _get_realm_id_from_vlob_id(conn, organization_id, vlob_id)
        await _check_realm_and_read_access(conn, organization_id, author, realm_id, None)

        rows = await conn.fetch(*_q_list_versions(organization_id=organization_id, vlob_id=vlob_id))
        assert rows
        if not rows:
            raise VlobNotFoundError(f"Vlob `{vlob_id}` doesn't exist")

        return {row["version"]: (row["created_on"], row["author"]) for row in rows}


_q_poll_changes = Q(
    f"""
SELECT
    index,
    vlob_id,
    vlob_atom.version
FROM realm_vlob_update LEFT JOIN vlob_atom
ON realm_vlob_update.vlob_atom = vlob_atom._id
WHERE
    realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
    AND index > $checkpoint
ORDER BY index ASC
"""
)


async def poll_changes(
    self, organization_id: OrganizationID, author: DeviceID, realm_id: UUID, checkpoint: int
) -> Tuple[int, Dict[UUID, int]]:
    async with conn.transaction():
        await _check_realm_and_read_access(conn, organization_id, author, realm_id, None)

        ret = await conn.fetch(_q_poll_changes(organization_id=organization_id, realm_id=realm_id, checkpoint=checkpoint))

        changes_since_checkpoint = {src_id: src_version for _, src_id, src_version in ret}
        new_checkpoint = ret[-1]["index"] if ret else checkpoint
        return (new_checkpoint, changes_since_checkpoint)

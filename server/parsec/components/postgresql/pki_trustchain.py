# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from dataclasses import dataclass

from parsec.components.pki import PkiCertificate
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_insert_pki_certificate = Q("""
INSERT INTO pki_certificate (
    sha256_fingerprint,
    signed_by,
    der_content
) VALUES (
    $sha256_fingerprint,
    $signed_by,
    $der_content
)
ON CONFLICT DO NOTHING
""")


async def save_trustchain(conn: AsyncpgConnection, trustchain: list[PkiCertificate]):
    await conn.executemany(
        _q_insert_pki_certificate.sql,
        (
            _q_insert_pki_certificate.arg_only(
                sha256_fingerprint=cert.fingerprint_sha256,
                signed_by=cert.signed_by,
                der_content=cert.content,
            )
            # Consider that trustchain list is ordered in such a way that the leaf in the first element and the end of the chain at the end.
            # We need to start form the end and walk our way back to the leaf to have a correct chaining of each database entry.
            for cert in reversed(trustchain)
        ),
    )


_q_get_pki_certificate = Q("""
WITH RECURSIVE trustchain AS (
    SELECT *
    FROM pki_certificate
    WHERE sha256_fingerprint = $leaf_sha256_fingerprint

    UNION ALL

    SELECT parent.*
    FROM pki_certificate AS parent
    INNER JOIN trustchain AS child ON parent.sha256_fingerprint = child.signed_by
)

SELECT
    sha256_fingerprint,
    signed_by,
    der_content
FROM trustchain;
""")


@dataclass(frozen=True, slots=True)
class PGPkiCertificate:
    sha256_fingerprint: bytes
    signed_by: bytes | None
    der_content: bytes

    def __repr__(self) -> str:
        fingerprint = self.sha256_fingerprint.hex()
        signed_by = self.signed_by.hex() if self.signed_by else None
        return f"{self.__class__.__qualname__}({fingerprint=}, {signed_by=})"


async def get_trustchain(
    conn: AsyncpgConnection, leaf_fingerprint: bytes
) -> list[PGPkiCertificate]:
    entries = await conn.fetch(*_q_get_pki_certificate(leaf_sha256_fingerprint=leaf_fingerprint))

    return [
        PGPkiCertificate(
            sha256_fingerprint=row["sha256_fingerprint"],
            signed_by=row["signed_by"],
            der_content=row["der_content"],
        )
        for row in entries
    ]

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    EmailAddress,
    UntrustedPasswordAlgorithm,
    UntrustedPasswordAlgorithmArgon2id,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_get_password_algorithm = Q("""
SELECT
    vault_authentication_method.password_algorithm,
    vault_authentication_method.password_algorithm_argon2id_opslimit,
    vault_authentication_method.password_algorithm_argon2id_memlimit_kb,
    vault_authentication_method.password_algorithm_argon2id_parallelism
FROM vault_authentication_method
INNER JOIN vault ON vault_authentication_method.vault = vault._id
INNER JOIN account ON vault.account = account._id
WHERE
    account.email = $email
    AND vault_authentication_method.disabled_on IS NULL
    AND vault_authentication_method.password_algorithm IS NOT NULL
    -- Extra safety check since a deleted account should not have any active authentication method
    AND account.deleted_on IS NULL
-- There is at most a single active password authentication method per account
LIMIT 1
""")


async def get_password_algorithm(
    conn: AsyncpgConnection,
    email: EmailAddress,
) -> UntrustedPasswordAlgorithm | None:
    row = await conn.fetchrow(*_q_get_password_algorithm(email=email.str))
    if not row:
        # Account does not exists or it does not have an active password algorithm.
        return None

    match row["password_algorithm"]:
        case "ARGON2ID":
            match row["password_algorithm_argon2id_opslimit"]:
                case int() as algorithm_argon2id_opslimit:
                    pass
                case _:
                    assert False, row

            match row["password_algorithm_argon2id_memlimit_kb"]:
                case int() as algorithm_argon2id_memlimit_kb:
                    pass
                case _:
                    assert False, row

            match row["password_algorithm_argon2id_parallelism"]:
                case int() as algorithm_argon2id_parallelism:
                    pass
                case _:
                    assert False, row

            return UntrustedPasswordAlgorithmArgon2id(
                opslimit=algorithm_argon2id_opslimit,
                memlimit_kb=algorithm_argon2id_memlimit_kb,
                parallelism=algorithm_argon2id_parallelism,
            )

        case _:
            assert False, row

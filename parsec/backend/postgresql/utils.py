# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from functools import wraps


def query(in_transaction=False):
    if in_transaction:

        def decorator(fn):
            @wraps(fn)
            async def wrapper(conn, *args, **kwargs):
                async with conn.transaction():
                    return await fn(conn, *args, **kwargs)

            return wrapper

    else:

        def decorator(fn):
            return fn

    return decorator

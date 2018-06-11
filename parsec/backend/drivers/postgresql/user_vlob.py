from parsec.backend.user_vlob import BaseUserVlobComponent, UserVlobAtom
from parsec.backend.exceptions import VersionError


class PGUserVlobComponent(BaseUserVlobComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def read(self, user_id, version=None):
        if version == 0:
            return UserVlobAtom(user_id=user_id)

        async with self.dbh.pool.acquire() as conn:
            if version is None:
                data = await conn.fetchrow(
                    """
                    SELECT version, blob FROM user_vlobs WHERE user_id=$1 ORDER BY version DESC limit 1
                    """,
                    user_id,
                )
                if not data:
                    return UserVlobAtom(user_id=user_id)

                else:
                    version, blob = data
            else:
                data = await conn.fetchrow(
                    "SELECT blob FROM user_vlobs WHERE user_id=$1 AND version=$2", user_id, version
                )
                if not data:
                    raise VersionError("Wrong blob version.")

                else:
                    blob = data[0]

        return UserVlobAtom(user_id=user_id, version=version, blob=blob)

    async def update(self, user_id, version, blob):
        async with self.dbh.pool.acquire() as conn:
            async with conn.transaction():
                vlobcount, = await conn.fetchrow(
                    "SELECT COUNT(user_id) FROM user_vlobs WHERE user_id=$1", user_id
                )

                if vlobcount != version - 1:
                    raise VersionError("Wrong blob version.")

                await conn.execute(
                    "INSERT INTO user_vlobs (user_id, version, blob) VALUES ($1, $2, $3)",
                    user_id,
                    version,
                    blob,
                )
        self._signal_user_vlob_updated.send(user_id)

from parsec.backend.user_vlob import BaseUserVlobComponent, UserVlobAtom
from parsec.backend.exceptions import VersionError


class PGUserVlobComponent(BaseUserVlobComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def read(self, user_id, version=None):
        vlobs = await self.dbh.fetch_many(
            'SELECT user_id, version, blob FROM user_vlobs WHERE user_id=%s',
            (user_id,)
        )

        if version == 0 or (version is None and not vlobs):
            return UserVlobAtom(user_id=user_id)

        try:
            if version is None:
                user_id, version, blob = vlobs[-1]
            else:
                user_id, version, blob = vlobs[version - 1]

        except IndexError:
            raise VersionError('Wrong blob version.')

        return UserVlobAtom(user_id=user_id, version=version, blob=blob)

    async def update(self, user_id, version, blob):
        # TODO: atomic operations
        vlobcount, = await self.dbh.fetch_one(
            'SELECT COUNT(user_id) FROM user_vlobs WHERE user_id=%s',
            (user_id,)
        )

        if vlobcount != version - 1:
            raise VersionError('Wrong blob version.')

        await self.dbh.insert_one(
            'INSERT INTO user_vlobs (user_id, version, blob) VALUES (%s, %s, %s)',
            (user_id, version, blob)
        )
        self._signal_user_vlob_updated.send(user_id)

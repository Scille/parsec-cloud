from parsec.backend.user_vlob import BaseUserVlobComponent, UserVlobAtom
from parsec.backend.exceptions import VersionError


class PGUserVlobComponent(BaseUserVlobComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def read(self, id, version=None):
        vlobs = await self.dbh.fetch_many(
            'SELECT id, version, blob FROM user_vlobs WHERE id=%s',
            (id,)
        )

        if version == 0 or (version is None and not vlobs):
            return UserVlobAtom(id=id)

        try:
            if version is None:
                id, version, blob = vlobs[-1]
            else:
                id, version, blob = vlobs[version - 1]

        except IndexError:
            raise VersionError('Wrong blob version.')

        return UserVlobAtom(id=id, version=version, blob=blob)

    async def update(self, id, version, blob):
        # TODO: atomic operations
        vlobcount, = await self.dbh.fetch_one(
            'SELECT COUNT(id) FROM user_vlobs WHERE id=%s',
            (id,)
        )

        if vlobcount != version - 1:
            raise VersionError('Wrong blob version.')

        await self.dbh.insert_one(
            'INSERT INTO user_vlobs (id, version, blob) VALUES (%s, %s, %s)',
            (id, version, blob)
        )
        self._signal_user_vlob_updated.send(id)

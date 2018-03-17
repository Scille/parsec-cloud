from parsec.backend.vlob import VlobAtom, BaseVlobComponent
from parsec.backend.exceptions import (
    TrustSeedError,
    VersionError,
    NotFoundError
)


class PGVlob:
    def __init__(self, *args, **kwargs):
        atom = VlobAtom(*args, **kwargs)
        self.id = atom.id
        self.read_trust_seed = atom.read_trust_seed
        self.write_trust_seed = atom.write_trust_seed
        self.blob_versions = [atom.blob]


class PGVlobComponent(BaseVlobComponent):
    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def create(self, id, rts, wts, blob):
        await self.dbh.insert_one(
            'INSERT INTO vlobs (id, rts, wts, version, blob) VALUES (%s, %s, %s, 0, %s)',
            (id, rts, wts, blob)
        )

        return VlobAtom(
            id=id,
            read_trust_seed=rts,
            write_trust_seed=wts,
            blob=blob
        )

    async def read(self, id, trust_seed, version=None):
        # TODO: use version in query to avoid retreiving all data each time
        vlobs = await self.dbh.fetch_many(
            'SELECT id, rts, wts, blob FROM vlobs WHERE id = %s ORDER BY version ASC',
            (id,)
        )
        vlobcount = len(vlobs)

        if vlobcount == 0:
            raise NotFoundError('Vlob not found.')

        version = version or vlobcount
        try:
            id, rts, wts, blob = vlobs[version - 1]
        except IndexError:
            raise VersionError('Wrong blob version.')

        if rts != trust_seed:
            raise TrustSeedError()

        return VlobAtom(
            id=id,
            read_trust_seed=rts,
            write_trust_seed=wts,
            blob=blob,
            version=version
        )

    async def update(self, id, trust_seed, version, blob):
        vlobs = await self.dbh.fetch_many(
            'SELECT id, rts, wts FROM vlobs WHERE id = %s',
            (id,)
        )
        vlobcount = len(vlobs)

        if vlobcount == 0:
            raise NotFoundError('Vlob not found.')

        id, rts, wts = vlobs[0]

        if wts != trust_seed:
            raise TrustSeedError('Invalid write trust seed.')

        if version - 1 != vlobcount:
            raise VersionError('Wrong blob version.')

        await self.dbh.insert_one(
            'INSERT INTO vlobs (id, rts, wts, version, blob) VALUES (%s, %s, %s, %s, %s)',
            (id, rts, wts, version, blob)
        )

        self._signal_vlob_updated.send(id)

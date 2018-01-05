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
            'INSERT INTO vlobs (id, rts, wts, blob) VALUES (%s, %s, %s, %s)',
            (id, rts, wts, blob)
        )

        return VlobAtom(
            id=id,
            read_trust_seed=rts,
            write_trust_seed=wts,
            blob=blob
        )

    async def read(self, id, trust_seed, version=None):
        vlobs = await self.dbh.fetch_many(
            'SELECT id, rts, wts, blob FROM vlobs WHERE id = %s',
            (id,)
        )
        vlobcount = len(vlobs)

        if vlobcount == 0:
            raise NotFoundError('Vlob not found.')

        version = version or vlobcount
        vlob = vlobs[version - 1]

        if vlob['rts'] != trust_seed:
            raise TrustSeedError()

        return VlobAtom(
            id=vlob['id'],
            read_trust_seed=vlob['rts'],
            write_trust_seed=vlob['wts'],
            blob=vlob['blob'],
            version=version
        )

    async def update(self, id, trust_seed, version, blob):
        vlobs = await self.dbh.fetch_many(
            'SELECT id, rts, wts, blob FROM vlobs WHERE id = %s',
            (id,)
        )
        vlobcount = len(vlobs)

        if vlobcount == 0:
            raise NotFoundError('Vlob not found.')

        vlob = vlobs[0]

        if vlob['wts'] != trust_seed:
            raise TrustSeedError('Invalid write trust seed.')

        if version - 1 == vlobcount:
            await self.create(vlob['id'], vlob['rts'], vlob['wts'], blob)

        else:
            raise VersionError('Wrong blob version.')

        self._signal_vlob_updated.send(id)

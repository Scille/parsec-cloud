from parsec.backend.vlob import VlobAtom, BaseVlobComponent
from parsec.backend.exceptions import TrustSeedError, VersionError, NotFoundError
from .handler import atomic


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

    @atomic
    async def create(self, conn, id, rts, wts, blob):
        await self.dbh.insert_one(
            conn,
            "INSERT INTO vlobs (id, rts, wts, version, blob) VALUES ($1, $2, $3, 1, $4)",
            id,
            rts,
            wts,
            blob,
        )

        return VlobAtom(id=id, read_trust_seed=rts, write_trust_seed=wts, blob=blob)

    @atomic
    async def read(self, conn, id, trust_seed, version=None):
        if version is None:
            data = await self.dbh.fetch_one(
                conn,
                """
                SELECT rts, wts, version, blob FROM vlobs WHERE id=$1 ORDER BY version DESC limit 1
                """,
                id,
            )
            if not data:
                raise NotFoundError("Vlob not found.")

            else:
                rts, wts, version, blob = data
        else:
            data = await self.dbh.fetch_one(
                conn, "SELECT rts, wts, blob FROM vlobs WHERE id=$1 AND version=$2", id, version
            )
            if not data:
                # TODO: not cool to need 2nd request to know the error...
                exists = await self.dbh.fetch_one(conn, "SELECT true FROM vlobs WHERE id=$1", id)
                if exists:
                    raise VersionError("Wrong blob version.")

                else:
                    raise NotFoundError("Vlob not found.")

            else:
                rts, wts, blob = data

        if rts != trust_seed:
            raise TrustSeedError()

        return VlobAtom(
            id=id, read_trust_seed=rts, write_trust_seed=wts, blob=blob, version=version
        )

    @atomic
    async def update(self, conn, id, trust_seed, version, blob):
        vlobs = await self.dbh.fetch_many(conn, "SELECT id, rts, wts FROM vlobs WHERE id = $1", id)
        vlobcount = len(vlobs)

        if vlobcount == 0:
            raise NotFoundError("Vlob not found.")

        id, rts, wts = vlobs[0]

        if wts != trust_seed:
            raise TrustSeedError("Invalid write trust seed.")

        if version - 1 != vlobcount:
            raise VersionError("Wrong blob version.")

        await self.dbh.insert_one(
            conn,
            "INSERT INTO vlobs (id, rts, wts, version, blob) VALUES ($1, $2, $3, $4, $5)",
            id,
            rts,
            wts,
            version,
            blob,
        )

        self._signal_vlob_updated.send(id)

from parsec.backend.vlob import VlobAtom, BaseVlobComponent
from parsec.backend.exceptions import (TrustSeedError, VersionError, NotFoundError)


class MemoryVlob:

    def __init__(self, *args, **kwargs):
        atom = VlobAtom(*args, **kwargs)
        self.id = atom.id
        self.read_trust_seed = atom.read_trust_seed
        self.write_trust_seed = atom.write_trust_seed
        self.blob_versions = [atom.blob]


class MemoryVlobComponent(BaseVlobComponent):

    def __init__(self, *args):
        super().__init__(*args)
        self.vlobs = {}

    async def create(self, id, rts, wts, blob):
        vlob = MemoryVlob(id, rts, wts, blob)
        self.vlobs[vlob.id] = vlob
        return VlobAtom(
            id=vlob.id,
            read_trust_seed=vlob.read_trust_seed,
            write_trust_seed=vlob.write_trust_seed,
            blob=vlob.blob_versions[0],
        )

    async def read(self, id, trust_seed, version=None):
        try:
            vlob = self.vlobs[id]
            if vlob.read_trust_seed != trust_seed:
                raise TrustSeedError()

        except KeyError:
            raise NotFoundError("Vlob not found.")

        version = version or len(vlob.blob_versions)
        try:
            return VlobAtom(
                id=vlob.id,
                read_trust_seed=vlob.read_trust_seed,
                write_trust_seed=vlob.write_trust_seed,
                blob=vlob.blob_versions[version - 1],
                version=version,
            )

        except IndexError:
            raise VersionError("Wrong blob version.")

    async def update(self, id, trust_seed, version, blob):
        try:
            vlob = self.vlobs[id]
            if vlob.write_trust_seed != trust_seed:
                raise TrustSeedError("Invalid write trust seed.")

        except KeyError:
            raise NotFoundError("Vlob not found.")

        if version - 1 == len(vlob.blob_versions):
            vlob.blob_versions.append(blob)
        else:
            raise VersionError("Wrong blob version.")

        self._signal_vlob_updated.send(id)

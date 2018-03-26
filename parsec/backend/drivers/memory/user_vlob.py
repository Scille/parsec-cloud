from parsec.backend.user_vlob import BaseUserVlobComponent, UserVlobAtom
from parsec.backend.exceptions import VersionError

from collections import defaultdict


class MemoryUserVlobComponent(BaseUserVlobComponent):
    def __init__(self, *args):
        super().__init__(*args)
        self.vlobs = defaultdict(list)

    async def read(self, user_id, version=None):
        vlobs = self.vlobs[user_id]
        if version == 0 or (version is None and not vlobs):
            return UserVlobAtom(user_id=user_id)
        try:
            if version is None:
                return vlobs[-1]
            else:
                return vlobs[version - 1]
        except IndexError:
            raise VersionError('Wrong blob version.')

    async def update(self, user_id, version, blob):
        vlobs = self.vlobs[user_id]
        if len(vlobs) != version - 1:
            raise VersionError('Wrong blob version.')
        vlobs.append(UserVlobAtom(user_id=user_id, version=version, blob=blob))
        self._signal_user_vlob_updated.send(user_id)

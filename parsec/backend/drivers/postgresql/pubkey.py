from nacl.public import PublicKey
from nacl.signing import VerifyKey

from parsec.backend.pubkey import BasePubKeyComponent

from parsec.backend.exceptions import (AlreadyExistsError, NotFoundError)


class PGPubKeyComponent(BasePubKeyComponent):

    def __init__(self, dbh, *args):
        super().__init__(*args)
        self.dbh = dbh

    async def add(self, id, pubkey, verifykey):
        assert isinstance(pubkey, (bytes, bytearray))
        assert isinstance(verifykey, (bytes, bytearray))

        key = await self.dbh.fetch_one("SELECT 1 FROM pubkeys WHERE id = %s", (id,))

        if key is not None:
            raise AlreadyExistsError("Identity `%s` already has a public key" % id)

        await self.dbh.insert_one(
            "INSERT INTO pubkeys (id, pubkey, verifykey) VALUES (%s, %s, %s)",
            (id, pubkey, verifykey),
        )

    async def get(self, id, cooked=False):
        key = await self.dbh.fetch_one(
            "SELECT pubkey, verifykey FROM pubkeys WHERE id = %s", (id,)
        )

        if key is None:
            raise NotFoundError("No public key for identity `%s`" % id)

        if cooked:
            return (PublicKey(key["pubkey"]), VerifyKey(key["verifykey"]))

        else:
            return (key["pubkey"], key["verifykey"])

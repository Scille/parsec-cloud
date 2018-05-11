from parsec.utils import from_jsonb64, to_jsonb64
from parsec.core.backend_connection import BackendError

from parsec.core.base import BaseAsyncComponent


# TODO: improve exceptions


class BackendConcurrencyError(BackendError):
    pass


class BackendStorage(BaseAsyncComponent):

    def __init__(self, backend_connection):
        super().__init__()
        self._backend_connection = backend_connection

    async def _init(self, nursery):
        pass

    async def _teardown(self):
        pass

    async def fetch_user_manifest(self, version=None):
        payload = {"cmd": "user_vlob_read"}
        if version is not None:
            payload["version"] = version
        rep = await self._backend_connection.send(payload)
        if rep["status"] == "ok":
            return from_jsonb64(rep["blob"])

        else:
            raise BackendError("Error %s: %s" % (rep["status"], rep["reason"]))

    async def sync_user_manifest(self, version, blob):
        rep = await self._backend_connection.send(
            {"cmd": "user_vlob_update", "version": version, "blob": to_jsonb64(blob)}
        )
        if rep["status"] != "ok":
            raise BackendConcurrencyError("Error %s: %s" % (rep["status"], rep["reason"]))

    async def fetch_manifest(self, id, rts, version=None):
        payload = {"cmd": "vlob_read", "id": id, "trust_seed": rts}
        if version is not None:
            payload["version"] = version
        rep = await self._backend_connection.send(payload)
        if rep["status"] == "ok":
            return from_jsonb64(rep["blob"])

        else:
            raise BackendError("Error %s: %s" % (rep["status"], rep["reason"]))

    async def sync_manifest(self, id, wts, version, blob):
        rep = await self._backend_connection.send(
            {
                "cmd": "vlob_update",
                "id": id,
                "trust_seed": wts,
                "version": version,
                "blob": to_jsonb64(blob),
            }
        )
        if rep["status"] != "ok":
            raise BackendConcurrencyError("Error %s: %s" % (rep["status"], rep["reason"]))

    async def sync_new_manifest(self, blob):
        rep = await self._backend_connection.send({"cmd": "vlob_create", "blob": to_jsonb64(blob)})
        if rep["status"] != "ok":
            raise BackendError("Error %s: %s" % (rep["status"], rep["reason"]))

        return rep["id"], rep["read_trust_seed"], rep["write_trust_seed"]

    async def sync_new_block(self, block):
        rep = await self._backend_connection.send(
            {"cmd": "blockstore_post", "block": to_jsonb64(block)}
        )
        if rep["status"] != "ok":
            raise BackendError("Error %s: %s" % (rep["status"], rep["reason"]))

        return rep["id"]

    async def fetch_block(self, id):
        rep = await self._backend_connection.send({"cmd": "blockstore_get", "id": id})
        if rep["status"] == "ok":
            return from_jsonb64(rep["block"])

        else:
            raise BackendError("Error %s: %s" % (rep["status"], rep["reason"]))

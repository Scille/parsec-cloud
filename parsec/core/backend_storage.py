class BaseBackendStorage:

    def __init__(self, backend_connection):
        self.backend_conn = backend_connection

    async def fetch_user_manifest(self):
        raise NotImplementedError()

    async def sync_user_manifest(self, blob):
        raise NotImplementedError()

    async def fetch_manifest(self, entry):
        raise NotImplementedError()

    async def sync_manifest(self, entry, blob):
        raise NotImplementedError()


# TODO...
class BackendStorage(BaseBackendStorage):
    pass

class BaseLocalStorage:
    def __init__(self, user):
        self.user = user

    def fetch_user_manifest(self):
        raise NotImplementedError()

    def flush_user_manifest(self, blob):
        raise NotImplementedError()

    def fetch_manifest(self, entry):
        raise NotImplementedError()

    def flush_manifest(self, entry, blob):
        raise NotImplementedError()


# TODO...
class LocalStorage(BaseLocalStorage):
    pass

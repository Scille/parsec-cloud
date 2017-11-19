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

    # def get_user_manifest(self):
    #     raise NotImplementedError()

    # def save_user_manifest(self):
    #     raise NotImplementedError()

    # def get_block(self, id):
    #     raise NotImplementedError()

    # def get_dirty_block(self, id):
    #     raise NotImplementedError()

    # def save_dirty_block(self, id, data):
    #     raise NotImplementedError()

    # def get_manifest(self, entry):
    #     raise NotImplementedError()

    # def save_manifest(self, entry, manifest):
    #     raise NotImplementedError()


# TODO...
class LocalStorage(BaseLocalStorage):
    def fetch_user_manifest(self):
        return None

    def fetch_manifest(self, entry):
        return None


# @attr.s
# class MockedLocalStorage(BaseLocalStorage):
#     _manifests = attr.ib(default=attr.Factory(dict), init=False)

#     def get_manifest(self, entry):
#         return self._manifests.get(entry.id)

#     def save_manifest(self, entry, data):
#         self._manifests[entry.id] = data

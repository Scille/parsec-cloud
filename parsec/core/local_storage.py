class BaseLocalStorage:
    def __init__(self, user):
        self.user = user

    def fetch_user_manifest(self):
        raise NotImplementedError()

    def flush_user_manifest(self, blob):
        raise NotImplementedError()

    def fetch_manifest(self, id):
        raise NotImplementedError()

    def flush_manifest(self, id, blob):
        raise NotImplementedError()

    def move_manifest(self, id, new_id):
        raise NotImplementedError()

    def fetch_block(self, id):
        raise NotImplementedError()

    def flush_block(self, id, blob):
        raise NotImplementedError()

    def fetch_dirty_block(self, id):
        raise NotImplementedError()

    def flush_dirty_block(self, id, blob):
        raise NotImplementedError()


# TODO...
class LocalStorage(BaseLocalStorage):
    pass

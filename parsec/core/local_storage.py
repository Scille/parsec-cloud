class BaseLocalStorage:

    def get_block(self, id):
        raise NotImplementedError()

    def get_file_manifest(self, id, version=None):
        raise NotImplementedError()

    def get_local_user_manifest(self):
        raise NotImplementedError()

    def save_local_user_manifest(self, data):
        raise NotImplementedError()

    def get_dirty_block(self, id):
        raise NotImplementedError()

    def save_dirty_block(self, id, data):
        raise NotImplementedError()

    def get_dirty_file_manifest(self, id):
        raise NotImplementedError()

    def save_dirty_file_manifest(self, id, data):
        raise NotImplementedError()

    def get_placeholder_file_manifest(self, id):
        raise NotImplementedError()

    def save_placeholder_file_manifest(self, id, data):
        raise NotImplementedError()


# TODO...
class LocalStorage:
    def __init__(self):
        raise NotImplementedError()

import attr


@attr.s
class OracleFS:
    root = attr.ib(default=attr.Factory(dict))

    def create_file(self, parent_path, name):
        parent_folder = self.get_folder(parent_path)
        if parent_folder is None or name in parent_folder:
            return 'invalid_path'
        parent_folder[name] = '<file>'
        return 'ok'

    def create_folder(self, parent_path, name):
        parent_folder = self.get_folder(parent_path)
        if parent_folder is None or name in parent_folder:
            return 'invalid_path'
        parent_folder[name] = {}
        return 'ok'

    def delete(self, path):
        parent_path, name = path.rsplit('/', 1)
        parent_dir = self.get_path(parent_path)
        if isinstance(parent_dir, dict) and name in parent_dir:
            del parent_dir[name]
            return 'ok'
        else:
            return 'invalid_path'

    def move(self, src, dst):
        parent_src, name_src = src.rsplit('/', 1)
        parent_dst, name_dst = dst.rsplit('/', 1)

        parent_dir_src = self.get_folder(parent_src)
        parent_dir_dst = self.get_folder(parent_dst)

        if parent_dir_src is None or name_src not in parent_dir_src:
            return 'invalid_path'
        if parent_dir_dst is None or name_dst in parent_dir_dst:
            return 'invalid_path'

        parent_dir_dst[name_dst] = parent_dir_src.pop(name_src)
        return 'ok'

    def get_folder(self, path):
        elem = self.get_path(path)
        return elem if elem != '<file>' else None

    def get_file(self, path):
        elem = self.get_path(path)
        return elem if elem == '<file>' else None

    def get_path(self, path):
        current_folder = self.root
        try:
            for item in path.split('/'):
                if item:
                    current_folder = current_folder[item]
        except (KeyError, TypeError):
            # Item not in folder or we reached a file instead of a folder
            return None
        return current_folder

    def flush(self, path):
        return 'ok' if self.get_path(path) is not None else 'invalid_path'

    def sync(self, parent_path, name):
        parent_folder = self.get_folder(parent_path)
        if parent_folder is None or name not in parent_folder:
            return 'invalid_path'
        return 'ok'

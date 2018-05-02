import attr
from functools import wraps
from hypothesis.stateful import rule as vanilla_rule
from huepy import red, bold


def rule(**config):

    def dec(fn):

        @vanilla_rule(**config)
        @wraps(fn)
        def wrapper(*args, **kwargs):
            print(red(bold("%s(%s)" % (fn.__name__, kwargs))))
            try:
                return fn(*args, **kwargs)

            except AssertionError as exc:
                import tests.hypothesis.test_core_online_tree_and_sync

                # import pdb; pdb.set_trace()
                raise

        return wrapper

    return dec


class File:
    pass


class Folder(dict):
    pass


def normalize_path(path):
    return "/" + "/".join([x for x in path.split("/") if x])


@attr.s
class OracleFS:
    root = attr.ib(default=attr.Factory(Folder))

    def create_file(self, path):
        path = normalize_path(path)
        parent_path, name = path.rsplit("/", 1)

        parent_folder = self.get_folder(parent_path)
        if parent_folder is None or name in parent_folder:
            return "invalid_path"

        parent_folder[name] = File()
        return "ok"

    def create_folder(self, path):
        path = normalize_path(path)
        parent_path, name = path.rsplit("/", 1)

        parent_folder = self.get_folder(parent_path)
        if parent_folder is None or name in parent_folder:
            return "invalid_path"

        parent_folder[name] = Folder()
        return "ok"

    def delete(self, path):
        path = normalize_path(path)
        if path == "/":
            return "invalid_path"

        parent_path, name = path.rsplit("/", 1)
        parent_dir = self.get_path(parent_path or "/")
        if isinstance(parent_dir, Folder) and name in parent_dir:
            del parent_dir[name]
            return "ok"

        else:
            return "invalid_path"

    def move(self, src, dst):
        src = normalize_path(src)
        dst = normalize_path(dst)

        if src == "/" or dst == "/":
            return "invalid_path"

        if src == dst or dst.startswith(src + "/"):
            return "invalid_path"

        parent_src, name_src = src.rsplit("/", 1)
        parent_dst, name_dst = dst.rsplit("/", 1)

        parent_dir_src = self.get_folder(parent_src)
        parent_dir_dst = self.get_folder(parent_dst)

        if parent_dir_src is None or name_src not in parent_dir_src:
            return "invalid_path"

        if parent_dir_dst is None or name_dst in parent_dir_dst:
            return "invalid_path"

        parent_dir_dst[name_dst] = parent_dir_src.pop(name_src)
        return "ok"

    def get_folder(self, path):
        path = normalize_path(path)

        elem = self.get_path(path)
        return elem if isinstance(elem, Folder) else None

    def get_file(self, path):
        path = normalize_path(path)

        elem = self.get_path(path)
        return elem if isinstance(elem, File) else None

    def get_path(self, path):
        path = normalize_path(path)

        current_entry = self.root
        try:
            for item in path.split("/"):
                if item:
                    current_entry = current_entry[item]
        except (KeyError, TypeError):
            # Next entry is not in folder or we reached a file instead of a folder
            return None

        return current_entry

    def flush(self, path):
        return "ok" if self.get_path(path) is not None else "invalid_path"

    def sync(self, path):
        return "ok" if self.get_path(path) is not None else "invalid_path"

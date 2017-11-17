import attr
import pendulum
import json

from nacl.public import Box, PrivateKey

from parsec.utils import ParsecError


class InvalidPath(ParsecError):
    status = 'invalid_path'


def _create_root_tree():
    now = pendulum.utcnow().isoformat()
    return {
        'type': 'folder',
        'created': now,
        'children': {}
    }


def decrypt_and_load_local_user_manifest(privkey, ciphered_data):
    box = Box(privkey, privkey.public_key)
    data = json.loads(box.decrypt(ciphered_data).decode())
    data['dirty_files'] = set(data['dirty_files'])
    data['file_placeholders'] = set(data['file_placeholders'])
    assert data.pop('format') == 1
    return LocalUserManifest(**data)


def dump_and_encrypt_local_user_manifest(privkey, local_user_manifest):
    box = Box(privkey, privkey.public_key)
    return box.encrypt(json.dumps(local_user_manifest.dump()).encode())


@attr.s
class LocalUserManifest:
    tree = attr.ib(default=attr.Factory(_create_root_tree))
    base_version = attr.ib(default=0)
    file_placeholders = attr.ib(default=attr.Factory(set))
    dirty_files = attr.ib(default=attr.Factory(set))
    is_dirty = attr.ib(default=False)

    def dump(self):
        return {
            'format': 1,
            'tree': self.tree,
            'base_version': self.base_version,
            'is_dirty': self.is_dirty,
            'file_placeholders': list(self.file_placeholders),
            'dirty_files': list(self.dirty_files)
        }

    def check_path(self, path, should_exists=True, type=None):
        if path == '/':
            if not should_exists or type not in ('folder', None):
                raise InvalidPath('Root `/` folder always exists')
            else:
                return
        dirpath, leafname = path.rsplit('/', 1)
        obj = self.retrieve_path(dirpath)
        if obj['type'] != 'folder':
            raise InvalidPath("Path `%s` is not a folder" % path)
        try:
            leafobj = obj['children'][leafname]
            if not should_exists:
                raise InvalidPath("Path `%s` already exist" % path)
            if type is not None:
                if type == 'folder':
                    if leafobj['type'] != 'folder':
                        raise InvalidPath("Path `%s` is not a folder" % path)
                else:
                    if leafobj['type'] not in ('file', 'placeholder_file'):
                        raise InvalidPath("Path `%s` is not a file" % path)
        except KeyError:
            if should_exists:
                raise InvalidPath("Path `%s` doesn't exist" % path)

    def retrieve_path(self, path):
        # Try to retrieve the
        if not path:
            return self.tree
        if not path.startswith('/'):
            raise InvalidPath("Path must start with `/`")
        cur_dir = self.tree
        reps = path.split('/')
        for rep in reps:
            if not rep:
                continue
            else:
                try:
                    cur_dir = cur_dir['children'][rep]
                except KeyError:
                    raise InvalidPath("Path `%s` doesn't exist" % path)
        return cur_dir

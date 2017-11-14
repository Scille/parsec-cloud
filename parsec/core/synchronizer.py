import trio
from copy import deepcopy
from nacl.public import PrivateKey
from nacl.secret import SecretBox
import nacl.utils

from parsec.utils import from_jsonb64, to_jsonb64


WAIT_BEFORE_SYNC = 1  # 1 second


class Synchronizer:
    def __init__(self, local_fs, backend_conn):
        self.local_fs = local_fs
        self.backend_conn = backend_conn
        self._event_queue = trio.Queue(100)

    async def init(self, nursery):
        self.nursery = nursery
        nursery.start_soon(self._synchronizer_runner)
        # Determine who needs to be synchronized
        if self.local_fs.local_user_manifest.is_dirty:
            pass

    async def teardown(self):
        pass

    async def _synchronizer_runner(self):
        while True:
            await trio.sleep(WAIT_BEFORE_SYNC)
            print('START BACKEND SYNCHRONIZATION')
            async with trio.open_nursery() as nursery:
                nursery.start_soon(self._synchronize_user_manifest, nursery)
                nursery.start_soon(self._synchronize_files, nursery)
            print('DONE BACKEND SYNCHRONIZATION')

    async def _synchronize_files(self, nursery):
        # Retrieve and upload the dirty blocks
        # Upload the new file manifest
        for file in self.local_fs.files_manager.get_dirty_files():
            nursery.start_soon(self._synchronize_file, file)

    async def _upload_block(block):
        id = await self.backend_conn.block_save(block)

    async def _synchronize_file(self, file):
        new_blocks = []
        old_blocks = file.data['blocks']
        file.sync()
        # TODO: Too tired to do better, but here we should determine
        # new blocks to upload using the patches and existing blocks
        full_buffer = file.read()
        block_key = nacl.utils.random(SecretBox.KEY_SIZE)
        box = SecretBox(block_key)
        full_ciphered = box.encrypt(full_buffer)
        block_id = await self._upload_block(full_ciphered)
        file.data['buffers'] = [{
            'id': block_id,
            'key': to_jsonb64(block_key),
            'offset': 0,
            'size': len(full_buffer)
        }]
        file.data['dirty_blocks'] = []
        print('FILE %s is sync with backend' % file.id)

    async def _synchronize_placeholder_file(self, obj):
        assert obj['type'] == 'placeholder_file'
        manifest_key = from_jsonb64(obj['key'])
        file = self.local_fs.file_manager.get_placeholder_file(
            obj['id'], manifest_key)
        file.sync()

        # TODO: Too tired to do better, but here we should determine
        # new blocks to upload using the patches and existing blocks
        full_buffer = file.read()
        block_key = nacl.utils.random(SecretBox.KEY_SIZE)
        full_ciphered = SecretBox(block_key).encrypt(full_buffer)
        block_id = await self._upload_block(full_ciphered)

        manifest = {
            'version': 1,
            'created': file.data['created'],
            'updated': file.data['updated'],
            'size': file.data['size'],
            'blocks': [
                {
                    'id': block_id,
                    'key': to_jsonb64(block_key),
                    'offset': 0,
                    'size': len(full_buffer)
                }
            ]
        }

        manifest_ciphered = SecretBox(manifest_key).encrypt(json.dumps(manifest).encode())
        rep = await self.backend_conn.send({'cmd': 'vlob_create', 'blob': to_jsonb64(manifest_ciphered)})
        if rep['status'] != 'ok':
            # TODO: improve error handling...
            raise RuntimeError('Cannot upload file manifest %r: %r' % (manifest, rep))

        # Now modify the obj dict like we don't care about side effets...
        obj.update({
            'type': 'file',
            'id': rep['id'],
            'read_trust_seed': rep['read_trust_seed'],
            'write_trust_seed': rep['write_trust_seed']
            # Only `key` entry doesn't change
        })
        # Finally file_manager
        del self.file_manager.destroy_placeholder_file[obj['id']]
        print('PLACEHOLDER FILE %s is sync with backend, now %s' % (file.id, rep['id']))

    async def _synchronize_user_manifest(self, nursery):
        if not self.local_fs.local_user_manifest.is_dirty:
            return
        # Make a snapshot of the local_user_manifest given it could be modified
        # while we do the sync
        snapshot = deepcopy(self.local_fs.local_user_manifes)
        # Retrieve the placeholder files and turn them into real files
        async with trio.open_nursery() as nursery:
            for path in snapshot.file_placeholders:
                obj = snapshot.retrieve_path(path)
                nursery.start_soon(self._synchronize_placeholder_file, obj)
        # Finally merge the snapshot with the real local user manifest
        # TODO
        self.local_fs.local_user_manifest.is_dirty = False

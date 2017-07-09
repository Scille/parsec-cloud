import sys

from effect2 import Effect, do

from parsec.crypto import generate_sym_key, load_sym_key
from parsec.core.synchronizer import (EVlobCreate, EVlobList, EVlobRead, EVlobUpdate, EVlobDelete, EVlobSynchronize,
                                      EBlockCreate, EBlockSynchronize, EBlockRead, EBlockDelete)
from parsec.exceptions import BlockNotFound, FileError, VlobNotFound
from parsec.tools import from_jsonb64, to_jsonb64, ejson_dumps, ejson_loads, digest


class File:

    @classmethod
    @do
    def create(cls):
        self = File()
        blob = yield self._build_file_blocks(b'')
        blob = [blob]
        blob = ejson_dumps(blob).encode()
        self.encryptor = generate_sym_key()
        encrypted_blob = self.encryptor.encrypt(blob)
        encrypted_blob = to_jsonb64(encrypted_blob)
        vlob = yield Effect(EVlobCreate(encrypted_blob))
        self.id = vlob['id']
        self.read_trust_seed = vlob['read_trust_seed']
        self.write_trust_seed = vlob['write_trust_seed']
        self.dirty = True
        self.version = 0
        return self

    @classmethod
    @do
    def load(cls, id, key, read_trust_seed, write_trust_seed, version=None):
        self = File()
        self.id = id
        self.read_trust_seed = read_trust_seed
        self.write_trust_seed = write_trust_seed
        self.encryptor = load_sym_key(from_jsonb64(key))
        vlob = yield Effect(EVlobRead(self.id, self.read_trust_seed, version))
        self.version = vlob['version']
        self.dirty = False
        vlob_list = yield Effect(EVlobList())
        if vlob['id'] in vlob_list:
            self.dirty = True
            self.version -= 1
        return self

    def get_vlob(self):
        vlob = {}
        vlob['id'] = self.id
        vlob['read_trust_seed'] = self.read_trust_seed
        vlob['write_trust_seed'] = self.write_trust_seed
        vlob['key'] = to_jsonb64(self.encryptor.key)
        return vlob

    @do
    def get_blocks(self):
        blocks = yield self._find_matching_blocks()
        block_ids = []
        # TODO keep pre_excluded_blocks ? (block with size=0)
        for blocks_and_key in blocks['pre_excluded_blocks'] + blocks['included_blocks']:
            for block_properties in blocks_and_key['blocks']:
                block_ids.append(block_properties['block'])
        return block_ids

    def get_version(self):
        return self.version + 1 if self.dirty else self.version

    @do
    def read(self, size=None, offset=0):
        # Get data
        matching_blocks = yield self._find_matching_blocks(size, offset)
        data = matching_blocks['pre_included_data']
        for blocks_and_key in matching_blocks['included_blocks']:
            block_key = blocks_and_key['key']
            decoded_block_key = from_jsonb64(block_key)
            encryptor = load_sym_key(decoded_block_key)
            for block_properties in blocks_and_key['blocks']:
                block = yield Effect(EBlockRead(block_properties['block']))
                # Decrypt
                block_content = from_jsonb64(block['content'])
                chunk_data = encryptor.decrypt(block_content)
                # Check integrity
                assert digest(chunk_data) == block_properties['digest']
                assert len(chunk_data) == block_properties['size']
                data += chunk_data
        data += matching_blocks['post_included_data']
        return data

    @do
    def write(self, data, offset):
        previous_block_ids = yield self.get_blocks()
        matching_blocks = yield self._find_matching_blocks(len(data), offset)
        new_data = matching_blocks['pre_excluded_data']
        new_data += data
        new_data += matching_blocks['post_excluded_data']
        blob = []
        blob += matching_blocks['pre_excluded_blocks']
        new_blocks = yield self._build_file_blocks(new_data)
        blob.append(new_blocks)
        blob += matching_blocks['post_excluded_blocks']
        blob = ejson_dumps(blob)
        blob = blob.encode()
        encrypted_blob = self.encryptor.encrypt(blob)
        encrypted_blob = to_jsonb64(encrypted_blob)
        yield Effect(EVlobUpdate(self.id, self.write_trust_seed, self.version + 1, encrypted_blob))
        current_block_ids = yield self.get_blocks()
        for block_id in previous_block_ids:
            if block_id not in current_block_ids:
                try:
                    yield Effect(EBlockDelete(block_id))
                except BlockNotFound:
                    pass
        self.dirty = True

    @do
    def truncate(self, length):
        previous_block_ids = yield self.get_blocks()
        matching_blocks = yield self._find_matching_blocks(length, 0)
        blob = []
        blob += matching_blocks['included_blocks']
        new_blocks = yield self._build_file_blocks(matching_blocks['post_included_data'])
        blob.append(new_blocks)
        blob = ejson_dumps(blob)
        blob = blob.encode()
        encrypted_blob = self.encryptor.encrypt(blob)
        encrypted_blob = to_jsonb64(encrypted_blob)
        yield Effect(EVlobUpdate(self.id, self.write_trust_seed, self.version + 1, encrypted_blob))
        current_block_ids = yield self.get_blocks()
        for block_id in previous_block_ids:
            if block_id not in current_block_ids:
                try:
                    yield Effect(EBlockDelete(block_id))
                except BlockNotFound:
                    pass
        self.dirty = True

    @do
    def stat(self):
        version = self.get_version()
        vlob = yield Effect(EVlobRead(self.id, self.read_trust_seed, version))
        encrypted_blob = vlob['blob']
        encrypted_blob = from_jsonb64(encrypted_blob)
        blob = self.encryptor.decrypt(encrypted_blob)
        blob = ejson_loads(blob.decode())
        block_stat = {'creation_date': '2012-01-01T00:00:00'}  # TODO real date
        size = 0
        for blocks_and_key in blob:
            for block in blocks_and_key['blocks']:
                size += block['size']
        # TODO: don't provide atime field if we don't know it?
        return {
            'id': self.id,
            'type': 'file',
            'created': block_stat['creation_date'],
            'updated': block_stat['creation_date'],
            'size': size,
            'version': vlob['version']
        }

    # def history(self):
    #     # TODO ?
    #     pass

    @do
    def restore(self, version=None):
        if version is None:
            version = self.get_version() - 1
        if version < 1 or version >= self.get_version():
            raise FileError('bad_version', 'Bad version number.')
        yield self.discard()
        vlob = yield Effect(EVlobRead(self.id, self.read_trust_seed, version))
        yield Effect(EVlobUpdate(self.id,
                                 self.write_trust_seed,
                                 self.version + 1,
                                 vlob['blob']))
        self.dirty = True

    @do
    def reencrypt(self):
        version = self.get_version()
        old_vlob = yield Effect(EVlobRead(self.id, self.read_trust_seed, version))
        old_blob = old_vlob['blob']
        old_encrypted_blob = from_jsonb64(old_blob)
        new_blob = self.encryptor.decrypt(old_encrypted_blob)
        self.encryptor = generate_sym_key()
        new_encrypted_blob = self.encryptor.encrypt(new_blob)
        new_encrypted_blob = to_jsonb64(new_encrypted_blob)
        new_vlob = yield Effect(EVlobCreate(new_encrypted_blob))
        self.id = new_vlob['id']
        self.read_trust_seed = new_vlob['read_trust_seed']
        self.write_trust_seed = new_vlob['write_trust_seed']
        self.dirty = True

    @do
    def commit(self):
        block_ids = yield self.get_blocks()
        for block_id in block_ids:
            yield Effect(EBlockSynchronize(block_id))
        new_vlob = yield Effect(EVlobSynchronize(self.id))
        if new_vlob:
            if new_vlob is not True:
                self.id = new_vlob['id']
                self.read_trust_seed = new_vlob['read_trust_seed']
                self.write_trust_seed = new_vlob['write_trust_seed']
                new_vlob = self.get_vlob()
            self.version += 1
        self.dirty = False
        return new_vlob

    @do
    def discard(self):
        already_synchronized = False
        block_ids = yield self.get_blocks()
        for block_id in block_ids:
            try:
                yield Effect(EBlockDelete(block_id))
            except BlockNotFound:
                already_synchronized = True
        try:
            yield Effect(EVlobDelete(self.id))
        except VlobNotFound:
            already_synchronized = True
        self.dirty = False
        return not already_synchronized

    @do
    def _build_file_blocks(self, data):
        # Create chunks
        chunk_size = 4096  # TODO modify size
        chunks = [data[i:i + chunk_size] for i in range(0, len(data), chunk_size)]
        # Force a chunk even if the data is empty
        if not chunks:
            chunks = [b'']
        encryptor = generate_sym_key()
        blocks = []
        for chunk in chunks:
            cypher_chunk = encryptor.encrypt(chunk)
            cypher_chunk = to_jsonb64(cypher_chunk)
            block_id = yield Effect(EBlockCreate(cypher_chunk))
            blocks.append({'block': block_id,
                           'digest': digest(chunk),
                           'size': len(chunk)})
        # New vlob atom
        block_key = to_jsonb64(encryptor.key)
        blob = {'blocks': blocks,
                'key': block_key}
        self.dirty = True
        return blob

    @do
    def _find_matching_blocks(self, size=None, offset=0):
        if size is None:
            size = sys.maxsize
        pre_excluded_blocks = []
        post_excluded_blocks = []
        version = self.get_version()
        vlob = yield Effect(EVlobRead(self.id, self.read_trust_seed, version))
        blob = vlob['blob']
        encrypted_blob = from_jsonb64(blob)
        blob = self.encryptor.decrypt(encrypted_blob)
        blob = ejson_loads(blob.decode())
        pre_excluded_blocks = []
        included_blocks = []
        post_excluded_blocks = []
        cursor = 0
        pre_excluded_data = b''
        pre_included_data = b''
        post_included_data = b''
        post_excluded_data = b''
        for blocks_and_key in blob:
            block_key = blocks_and_key['key']
            decoded_block_key = from_jsonb64(block_key)
            encryptor = load_sym_key(decoded_block_key)
            for block_properties in blocks_and_key['blocks']:
                cursor += block_properties['size']
                if cursor <= offset:
                    if len(pre_excluded_blocks) and pre_excluded_blocks[-1]['key'] == block_key:
                        pre_excluded_blocks[-1]['blocks'].append(block_properties)
                    else:
                        pre_excluded_blocks.append({'blocks': [block_properties], 'key': block_key})
                elif cursor > offset and cursor - block_properties['size'] < offset:
                    delta = cursor - offset
                    block = yield Effect(EBlockRead(block_properties['block']))
                    content = from_jsonb64(block['content'])
                    block_data = encryptor.decrypt(content)
                    pre_excluded_data = block_data[:-delta]
                    pre_included_data = block_data[-delta:][:size]
                    if size < len(block_data[-delta:]):
                        post_excluded_data = block_data[-delta:][size:]
                elif cursor > offset and cursor <= offset + size:
                    if len(included_blocks) and included_blocks[-1]['key'] == block_key:
                        included_blocks[-1]['blocks'].append(block_properties)
                    else:
                        included_blocks.append({'blocks': [block_properties], 'key': block_key})
                elif cursor > offset + size and cursor - block_properties['size'] < offset + size:
                    delta = offset + size - (cursor - block_properties['size'])
                    block = yield Effect(EBlockRead(block_properties['block']))
                    content = from_jsonb64(block['content'])
                    block_data = encryptor.decrypt(content)
                    post_included_data = block_data[:delta]
                    post_excluded_data = block_data[delta:]
                else:
                    if len(post_excluded_blocks) and post_excluded_blocks[-1]['key'] == block_key:
                        post_excluded_blocks[-1]['blocks'].append(block_properties)
                    else:
                        post_excluded_blocks.append({'blocks': [block_properties],
                                                     'key': block_key})
        return {
            'pre_excluded_blocks': pre_excluded_blocks,
            'pre_excluded_data': pre_excluded_data,
            'pre_included_data': pre_included_data,
            'included_blocks': included_blocks,
            'post_included_data': post_included_data,
            'post_excluded_data': post_excluded_data,
            'post_excluded_blocks': post_excluded_blocks
        }

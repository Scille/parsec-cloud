from base64 import decodebytes, encodebytes
import json
import sys

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes

from parsec.crypto import generate_sym_key, load_sym_key


class File:

    @classmethod
    async def create(cls, synchronizer):
        self = File()
        self.synchronizer = synchronizer
        blob = [await self._build_file_blocks(b'')]
        blob = json.dumps(blob)
        blob = blob.encode()
        self.encryptor = generate_sym_key()
        encrypted_blob = self.encryptor.encrypt(blob)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        vlob = await self.synchronizer.vlob_create(encrypted_blob)
        self.id = vlob['id']
        self.read_trust_seed = vlob['read_trust_seed']
        self.write_trust_seed = vlob['write_trust_seed']
        self.version = 0
        return self

    @classmethod
    async def load(cls, synchronizer, id, key, read_trust_seed, write_trust_seed, version=None):
        self = File()
        self.synchronizer = synchronizer
        self.id = id
        self.read_trust_seed = read_trust_seed
        self.write_trust_seed = write_trust_seed
        self.encryptor = load_sym_key(decodebytes(key.encode()))
        vlob = await self.synchronizer.vlob_read(self.id, self.read_trust_seed, version)
        self.version = vlob['version']
        return self

    async def get_vlob(self):
        vlob = {}
        vlob['id'] = self.id
        vlob['read_trust_seed'] = self.read_trust_seed
        vlob['write_trust_seed'] = self.write_trust_seed
        vlob['key'] = encodebytes(self.encryptor.key).decode()
        return vlob

    async def get_blocks(self):
        vlob = await self.synchronizer.vlob_read(self.id, self.read_trust_seed, self.version)
        encrypted_blob = decodebytes(vlob['blob'].encode())
        blob = self.encryptor.decrypt(encrypted_blob)
        blob = json.loads(blob.decode())
        block_ids = []
        for block_and_key in blob:
            for block in block_and_key['blocks']:
                block_ids.append(block['block'])
        return block_ids

    async def get_version(self):
        return self.version

    async def read(self, size=None, offset=0):
        vlob = await self.synchronizer.vlob_read(id=self.id,
                                                 version=self.version,
                                                 trust_seed=self.read_trust_seed)
        encrypted_blob = decodebytes(vlob['blob'].encode())
        blob = self.encryptor.decrypt(encrypted_blob)
        blob = json.loads(blob.decode())
        # Get data
        matching_blocks = await self._find_matching_blocks(size, offset)
        data = b''
        data += decodebytes(matching_blocks['pre_included_data'].encode())
        for blocks_and_key in matching_blocks['included_blocks']:
            block_key = blocks_and_key['key']
            decoded_block_key = decodebytes(block_key.encode())
            encryptor = load_sym_key(decoded_block_key)
            for block_properties in blocks_and_key['blocks']:
                block = await self.synchronizer.block_read(block_properties['block'])
                # Decrypt
                block_content = decodebytes(block['content'].encode())
                chunk_data = encryptor.decrypt(block_content)
                # Check integrity
                digest = hashes.Hash(hashes.SHA512(), backend=openssl)
                digest.update(chunk_data)
                new_digest = digest.finalize()
                assert new_digest == decodebytes(block_properties['digest'].encode())
                data += chunk_data
        data += decodebytes(matching_blocks['post_included_data'].encode())
        data = encodebytes(data).decode()
        return data

    async def write(self, data, offset):
        previous_blocks = await self._find_matching_blocks()
        previous_blocks_ids = []
        for blocks_and_key in previous_blocks['included_blocks']:
            for block_properties in blocks_and_key['blocks']:
                previous_blocks_ids.append(block_properties['block'])
        data = decodebytes(data.encode())
        matching_blocks = await self._find_matching_blocks(len(data), offset)
        new_data = decodebytes(matching_blocks['pre_excluded_data'].encode())
        new_data += data
        new_data += decodebytes(matching_blocks['post_excluded_data'].encode())
        new_data = encodebytes(new_data).decode()
        blob = []
        blob += matching_blocks['pre_excluded_blocks']
        blob.append(await self._build_file_blocks(new_data))
        blob += matching_blocks['post_excluded_blocks']
        blob = json.dumps(blob)
        blob = blob.encode()
        encrypted_blob = self.encryptor.encrypt(blob)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        await self.synchronizer.vlob_update(self.id,
                                            self.version + 1,
                                            self.write_trust_seed,
                                            encrypted_blob)
        current_blocks = await self._find_matching_blocks()
        current_blocks_ids = []
        for blocks_and_key in current_blocks['included_blocks']:
            for block_properties in blocks_and_key['blocks']:
                current_blocks_ids.append(block_properties['block'])
        for block_id in previous_blocks_ids:
            if block_id not in current_blocks_ids:
                await self.synchronizer.block_delete(block_id)

    async def truncate(self, length):
        previous_blocks = await self._find_matching_blocks()
        previous_blocks_ids = []
        for blocks_and_key in previous_blocks['included_blocks']:
            for block_properties in blocks_and_key['blocks']:
                previous_blocks_ids.append(block_properties['block'])
        matching_blocks = await self._find_matching_blocks(length, 0)
        blob = []
        blob += matching_blocks['included_blocks']
        blob.append(await self._build_file_blocks(matching_blocks['post_included_data']))
        blob = json.dumps(blob)
        blob = blob.encode()
        encrypted_blob = self.encryptor.encrypt(blob)
        encrypted_blob = encodebytes(encrypted_blob).decode()
        await self.synchronizer.vlob_update(self.id,
                                            self.version + 1,
                                            self.write_trust_seed,
                                            encrypted_blob)
        current_blocks = await self._find_matching_blocks()
        current_blocks_ids = []
        for blocks_and_key in current_blocks['included_blocks']:
            for block_properties in blocks_and_key['blocks']:
                current_blocks_ids.append(block_properties['block'])
        for block_id in previous_blocks_ids:
            if block_id not in current_blocks_ids:
                await self.synchronizer.block_delete(block_id)

    # async def stat(self):
    #     # TODO ?
    #     pass

    # async def history(self):
    #     # TODO ?
    #     pass

    # async def restore(self):
    #     # TODO ?
    #     pass

    # async def reencrypt(self):
    #     # TODO ?
    #     pass

    # async def commit(self):
    #     vlob = await self.synchronizer.vlob_read(id=self.id,
    #                                                      version=self.version,
    #                                                      trust_seed=self.read_trust_seed)
    #     encrypted_blob = decodebytes(vlob['blob'].encode())
    #     blob = self.encryptor.decrypt(encrypted_blob)
    #     blob = json.loads(blob.decode())
    #     block_ids = []
    #     for block_and_key in blob:
    #         for block in block_and_key['blocks']:
    #             block_ids.append(block['block'])
    #     for block_id in block_ids:
    #         try:
    #             await self.synchronizer.synchronize(block_id)
    #         except Exception:  # TODO change type
    #             pass
    #     await self.synchronizer.synchronize(self.id)

    # async def discard(self):
    #     vlob = await self.synchronizer.vlob_read(id=self.id,
    #                                                      version=self.version,
    #                                                      trust_seed=self.read_trust_seed)
    #     blob = vlob['blob']
    #     encrypted_blob = decodebytes(blob.encode())
    #     blob = self.encryptor.decrypt(encrypted_blob)
    #     blob = json.loads(blob.decode())
    #     block_ids = []
    #     for block_and_key in blob:
    #         for block in block_and_key['blocks']:
    #             block_ids.append(block['block'])
    #     for block_id in block_ids:
    #         try:
    #             await self.synchronizer.block_delete(block_id)
    #         except Exception:  # TODO change type
    #             pass
    #     await self.synchronizer.block_delete(self.id)

    async def _build_file_blocks(self, data):
        if isinstance(data, str):
            data = data.encode()
        data = decodebytes(data)
        # Create chunks
        chunk_size = 4096  # TODO modify size
        chunks = [data[i:i + chunk_size] for i in range(0, len(data), chunk_size)]
        # Force a chunk even if the data is empty
        if not chunks:
            chunks = [b'']
        encryptor = generate_sym_key()
        blocks = []
        for chunk in chunks:
            # Digest
            digest = hashes.Hash(hashes.SHA512(), backend=openssl)
            digest.update(chunk)
            chunk_digest = digest.finalize()  # TODO replace with hexdigest ?
            chunk_digest = encodebytes(chunk_digest).decode()
            # Encrypt block
            cypher_chunk = encryptor.encrypt(chunk)
            # Store block
            cypher_chunk = encodebytes(cypher_chunk).decode()
            block_id = await self.synchronizer.block_create(content=cypher_chunk)
            blocks.append({'block': block_id,
                           'digest': chunk_digest,
                           'size': len(chunk)})
        # New vlob atom
        block_key = encodebytes(encryptor.key).decode()
        blob = {'blocks': blocks,
                'key': block_key}
        return blob

    async def _find_matching_blocks(self, size=None, offset=0):
        if size is None:
            size = sys.maxsize
        pre_excluded_blocks = []
        post_excluded_blocks = []
        vlob = await self.synchronizer.vlob_read(self.id,
                                                 self.read_trust_seed,
                                                 self.version)
        blob = vlob['blob']
        encrypted_blob = decodebytes(blob.encode())
        blob = self.encryptor.decrypt(encrypted_blob)
        blob = json.loads(blob.decode())
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
            decoded_block_key = decodebytes(block_key.encode())
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
                    block = await self.synchronizer.block_read(block_properties['block'])
                    content = decodebytes(block['content'].encode())
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
                    block = await self.synchronizer.block_read(block_properties['block'])
                    content = decodebytes(block['content'].encode())
                    block_data = encryptor.decrypt(content)
                    post_included_data = block_data[:delta]
                    post_excluded_data = block_data[delta:]
                else:
                    if len(post_excluded_blocks) and post_excluded_blocks[-1]['key'] == block_key:
                        post_excluded_blocks[-1]['blocks'].append(block_properties)
                    else:
                        post_excluded_blocks.append({'blocks': [block_properties],
                                                     'key': block_key})
        pre_included_data = encodebytes(pre_included_data).decode()
        pre_excluded_data = encodebytes(pre_excluded_data).decode()
        post_included_data = encodebytes(post_included_data).decode()
        post_excluded_data = encodebytes(post_excluded_data).decode()
        return {
            'pre_excluded_blocks': pre_excluded_blocks,
            'pre_excluded_data': pre_excluded_data,
            'pre_included_data': pre_included_data,
            'included_blocks': included_blocks,
            'post_included_data': post_included_data,
            'post_excluded_data': post_excluded_data,
            'post_excluded_blocks': post_excluded_blocks
        }

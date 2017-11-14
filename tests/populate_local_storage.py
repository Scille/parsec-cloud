import json
from nacl.public import Box
from nacl.secret import SecretBox

from parsec.utils import to_jsonb64


def populate_backend():
    pass


def populate_local_storage_cls(user, mocked_local_storage_cls):
    """
    Generated tree:
    /
    /empty_dir/ <= empty directory
    /dir/ <= directory
    /dir/up_to_date.txt <= regular file
    /dir/modified.txt <= regular file with local modifications
    /dir/new.txt <== placeholder file
    """
    store = mocked_local_storage_cls.test_storage

    # Hold my beer...

    # /dir/up_to_date.txt - Blocks
    up_to_date_txt_block_1_id = '505b0bef5dd44763abc9eac03c765bc3'
    up_to_date_txt_block_1_key = b'\xec\x1d\x84\x80\x05\x18\xb0\x8a\x1d\x81\xe0\xdb\xe5%wx\x9f\x7f\x01\xa6\x8f#>\xc5]\xae|\xfd\x1d\xc22\x05'
    store.blocks[up_to_date_txt_block_1_id] = SecretBox(up_to_date_txt_block_1_key).encrypt(b'Hello from ')
    up_to_date_txt_block_2_id = '0187fa3fc8a5480cbb3ef9df5dd2b7e9'
    up_to_date_txt_block_2_key = b'\xae\x85y\xdd:\xae\xa6\xf2\xdf\xce#U\x17\xffa\xde\x19\x1d\xa7\x84[\xb8\x92{$6\xf9\xc4\x8b\xbcT\x14'
    store.blocks[up_to_date_txt_block_2_id] = SecretBox(up_to_date_txt_block_2_key).encrypt(b'up_to_date.txt !')

    # /dir/up_to_date.txt - File manifest

    up_to_date_txt_id = '1a08acb35bc64ee6acff58b09e1ac939'
    up_to_date_txt_key = b'0\xba\x9fY\xd1\xb4D\x93\r\xf6\xa7[\xe8\xaa\xf9\xeea\xb8\x01\x98\xc1~im}C\xfa\xde\\\xe6\xa1-'
    up_to_date_txt_fms = {
        'version': 2,
        'created': '2017-12-02T12:30:30+00:00',
        'updated': '2017-12-02T12:30:45+00:00',
        'blocks': [
            {'id': up_to_date_txt_block_1_id, 'key': to_jsonb64(up_to_date_txt_block_1_key), 'offset': 0, 'size': 11},
            {'id': up_to_date_txt_block_2_id, 'key': to_jsonb64(up_to_date_txt_block_2_key), 'offset': 11, 'size': 16},
        ],
        'size': 27
    }
    store.file_manifests[up_to_date_txt_id] = SecretBox(up_to_date_txt_key).encrypt(json.dumps(up_to_date_txt_fms).encode())

    # /dir/up_to_date.txt - No dirty blocks (the file is up to date...)
    # /dir/up_to_date.txt - No dirty file manifest (the file is up to date...)

    # /dir/modified.txt - Blocks

    modified_txt_block_1_id = '973a198b344d403888472e17b610a43e'
    modified_txt_block_1_key = b'\xc7|\xd7+\xe5\xfbv\xd2\x8c0\xea\r\xff{;2\x0f\xb8s-H\xfd\xfb\xd4\xa157\x86\xde<3\xaa'
    store.blocks[modified_txt_block_1_id] = SecretBox(modified_txt_block_1_key).encrypt(b'This is version 1.')

    # /dir/modified.txt - File manifest
    # This file manifest shoudl be shadowed by the dirty file manifest

    modified_txt_id = 'ba3d58e140ca44bc91cd53745961397d'
    modified_txt_key = b'0\xba\x9fY\xd1\xb4D\x93\r\xf6\xa7[\xe8\xaa\xf9\xeea\xb8\x01\x98\xc1~im}C\xfa\xde\\\xe6\xa1-'
    modified_txt_fms = {
        'version': 2,
        'created': '2017-12-02T12:50:30+00:00',
        'updated': '2017-12-02T12:50:45+00:00',
        'blocks': [
            {'id': modified_txt_block_1_id, 'key': to_jsonb64(modified_txt_block_1_key), 'offset': 0, 'size': 18},
        ],
        'size': 18
    }
    store.file_manifests[modified_txt_id] = SecretBox(modified_txt_key).encrypt(json.dumps(modified_txt_fms).encode())

    # /dir/modified.txt - Dirty blocks

    modified_txt_dirty_block_1_id = 'a38646aabf264f4fb9db0f636c4999a7'
    modified_txt_dirty_block_1_key = b'=\x04\xc8\x1d\xb6\xb2\x0c\xbf\xaf\xee\x04%zk\x12\xa4\xed\xda\\\xf5\x95\xa1\xf6\x99\x965G|\xca;\x8e\x05'
    store.dirty_blocks[modified_txt_dirty_block_1_id] = SecretBox(modified_txt_dirty_block_1_key).encrypt(b'SPARTAAAA !')

    # /dir/modified.txt - Dirty file manifest

    modified_txt_dirty_fm = {
        'version': 2,
        'created': '2017-12-02T12:50:30+00:00',
        'updated': '2017-12-02T12:51:00+00:00',
        'blocks': [
            {'id': modified_txt_block_1_id, 'key': to_jsonb64(modified_txt_block_1_key), 'offset': 0, 'size': 18},
        ],
        'dirty_blocks': [
            {'id': modified_txt_dirty_block_1_id, 'key': to_jsonb64(modified_txt_dirty_block_1_key), 'offset': 8, 'size': 11}
        ],
        'size': 19
    }
    store.dirty_file_manifests[modified_txt_id] = SecretBox(modified_txt_key).encrypt(json.dumps(modified_txt_dirty_fm).encode())

    # /dir/new.txt - No blocks (given the file is a placeholder so far)
    # /dir/new.txt - No file manifest (given the file is a placeholder so far)
    # /dir/new.txt - Dirty blocks

    new_txt_dirty_block_1_id = 'faa4e1068dad47b4a758a73102478388'
    new_txt_dirty_block_1_key = b'\xab\xcfn\xc8*\xe8|\xc42\xf2\xfao\x1b\xc1Xm\xb4\xb9JBe\x9a1W\r(\xcc\xbd1\x12RB'
    store.dirty_blocks[new_txt_dirty_block_1_id] = SecretBox(new_txt_dirty_block_1_key).encrypt(b'Welcome to ')

    new_txt_dirty_block_2_id = '4c5b4338a47c462098d6c98856f5bf56'
    new_txt_dirty_block_2_key = b'\xcb\x1c\xe4\x80\x8d\xca\rl?z\xa4\x82J7\xc5\xd5\xed5^\xb6\x05\x8cR;A\xbd\xb1 \xbd\xc2?\xe9'
    store.dirty_blocks[new_txt_dirty_block_2_id] = SecretBox(new_txt_dirty_block_2_key).encrypt(b'the new file."')

    # /dir/new.txt - No dirty file manifest (you know the reason...)
    # /dir/new.txt - Placeholder file manifest

    new_txt_placeholder_id = '3ca6cb2ba8a9446f8508296b7a8c3ed4'
    new_txt_placeholder_key = b'"\x08"Q\xfbc\xa3 \xf9\xde\xbf\xc3\x07?\x9a\xa6V\xcet\x0c\xa1C\xf2\xa06\xa1\xc9 \xbf\xf6t\xbb'
    new_txt_placeholder_fm = {
        'version': 0,
        'created': '2017-12-02T12:50:30+00:00',
        'updated': '2017-12-02T12:51:00+00:00',
        'blocks': [
        ],
        'dirty_blocks': [
            {'id': new_txt_dirty_block_1_id, 'key': to_jsonb64(new_txt_dirty_block_1_key), 'offset': 0, 'size': 11},
            {'id': new_txt_dirty_block_2_id, 'key': to_jsonb64(new_txt_dirty_block_2_key), 'offset': 11, 'size': 13}
        ],
        'size': 24
    }
    store.placeholder_file_manifests[new_txt_placeholder_id] = SecretBox(new_txt_placeholder_key).encrypt(json.dumps(new_txt_placeholder_fm).encode())

    # Finally, create the dirty user manifest

    local_user_manifest = {
        'base_version': 3,
        'is_dirty': True,
        'file_placeholders': [new_txt_placeholder_id],
        'tree': {
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': {
                'empty_dir': {
                    'type': 'folder',
                    'created': '2017-12-02T12:29:03+00:00',
                    'children': {
                    },
                },
                'dir': {
                    'type': 'folder',
                    'created': '2017-12-02T12:30:23+00:00',
                    'children': {

                        'new.txt': {
                            'type': 'placeholder_file',
                            'id': new_txt_placeholder_id,
                            'key': to_jsonb64(new_txt_placeholder_key)
                        },
                        'up_to_date.txt': {
                            'type': 'file',
                            'id': up_to_date_txt_id,
                            'read_trust_seed': '<rts>',
                            'write_trust_seed': '<wts>',
                            'key': to_jsonb64(up_to_date_txt_key)
                        },
                        'modified.txt': {
                            'type': 'file',
                            'id': modified_txt_id,
                            'read_trust_seed': '<rts>',
                            'write_trust_seed': '<wts>',
                            'key': to_jsonb64(modified_txt_key)
                        }

                    }
                }
            }
        }
    }
    box = Box(user.privkey, user.pubkey)
    store.local_user_manifest = box.encrypt(json.dumps(local_user_manifest).encode())

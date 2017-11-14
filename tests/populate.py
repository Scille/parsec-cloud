import json
from copy import deepcopy
from nacl.public import Box
from nacl.secret import SecretBox

from parsec.utils import to_jsonb64


_cache = {}


def populate_factory(user):
    if user.id not in _cache:
        _cache[user.id] = _populate_factory(user)
    return deepcopy(_cache[user.id])


def _populate_factory(user):
    """
    Generated tree:
    /
    /empty_dir/ <= empty directory
    /dir/ <= directory
    /dir/up_to_date.txt <= regular file, present in the core
    /dir/modified.txt <= regular file with local modifications
    /dir/non_local.txt <= regular file, only stored in backend
    """
    data = {
        'backend': {
            'vlobs': {
                # <id>: {'rts': <str>, 'wts': <str>, 'blobs': [<bytes>, ...]}
            },
            'user_vlobs': {
                # <userid>: [<bytes>, ...]
            },
            'blockstore': {
                    # <id>: <bytes>
            }
        },
        'core': {
            'local_user_manifest': None,  # <bytes>
            'dirty_blocks': {
                # <id>: <bytes>
            },
            'blocks': {
                # <id>: <bytes>
            },
            'file_manifests': {
                # <id>: <bytes>
            },
            'dirty_file_manifests': {
                # <id>: <bytes>
            },
            'placeholder_file_manifests': {
                # <id>: <bytes>
            }
        }

    }

    # Hold my beer...

    # /dir/up_to_date.txt - Blocks
    up_to_date_txt_block_1_id = '505b0bef5dd44763abc9eac03c765bc3'
    up_to_date_txt_block_1_key = b'\xec\x1d\x84\x80\x05\x18\xb0\x8a\x1d\x81\xe0\xdb\xe5%wx\x9f\x7f\x01\xa6\x8f#>\xc5]\xae|\xfd\x1d\xc22\x05'
    up_to_date_txt_block_1_blob = SecretBox(up_to_date_txt_block_1_key).encrypt(b'Hello from ')
    data['backend']['blockstore'][up_to_date_txt_block_1_id] = up_to_date_txt_block_1_blob
    data['core']['blocks'][up_to_date_txt_block_1_id] = up_to_date_txt_block_1_blob
    up_to_date_txt_block_2_id = '0187fa3fc8a5480cbb3ef9df5dd2b7e9'
    up_to_date_txt_block_2_key = b'\xae\x85y\xdd:\xae\xa6\xf2\xdf\xce#U\x17\xffa\xde\x19\x1d\xa7\x84[\xb8\x92{$6\xf9\xc4\x8b\xbcT\x14'
    up_to_date_txt_block_2_blob = SecretBox(up_to_date_txt_block_2_key).encrypt(b'up_to_date.txt !')
    data['backend']['blockstore'][up_to_date_txt_block_2_id] = up_to_date_txt_block_2_blob
    data['core']['blocks'][up_to_date_txt_block_2_id] = up_to_date_txt_block_2_blob

    # /dir/up_to_date.txt - File manifest

    up_to_date_txt_id = '1a08acb35bc64ee6acff58b09e1ac939'
    up_to_date_txt_rts = 'c84bf72d941f4a249fe6754033f9214c'
    up_to_date_txt_wts = '03a2902ed1af4e15a60aa4223f8d9453'
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
    up_to_date_txt_fm_blob = SecretBox(up_to_date_txt_key).encrypt(json.dumps(up_to_date_txt_fms).encode())
    data['backend']['vlobs'][up_to_date_txt_id] = {
        'blobs': [up_to_date_txt_fm_blob],
        'rts': up_to_date_txt_rts,
        'wts': up_to_date_txt_wts
    }
    data['core']['file_manifests'][up_to_date_txt_id] = up_to_date_txt_fm_blob

    # /dir/up_to_date.txt - No dirty blocks (the file is up to date...)
    # /dir/up_to_date.txt - No dirty file manifest (the file is up to date...)

    # /dir/non_local.txt - Blocks
    non_local_txt_block_1_id = '74ab15c511734fed86163944586e721a'
    non_local_txt_block_1_key = b'\xec\x1d\x84\x80\x05\x18\xb0\x8a\x1d\x81\xe0\xdb\xe5%wx\x9f\x7f\x01\xa6\x8f#>\xc5]\xae|\xfd\x1d\xc22\x05'
    non_local_txt_block_1_blob = SecretBox(non_local_txt_block_1_key).encrypt(b'Hello from ')
    data['backend']['blockstore'][non_local_txt_block_1_id] = non_local_txt_block_1_blob
    data['core']['blocks'][non_local_txt_block_1_id] = non_local_txt_block_1_blob
    non_local_txt_block_2_id = 'bc9d482e76f54b21bf96532272defc43'
    non_local_txt_block_2_key = b'\xae\x85y\xdd:\xae\xa6\xf2\xdf\xce#U\x17\xffa\xde\x19\x1d\xa7\x84[\xb8\x92{$6\xf9\xc4\x8b\xbcT\x14'
    non_local_txt_block_2_blob = SecretBox(non_local_txt_block_2_key).encrypt(b'non_local.txt !')
    data['backend']['blockstore'][non_local_txt_block_2_id] = non_local_txt_block_2_blob
    data['core']['blocks'][non_local_txt_block_2_id] = non_local_txt_block_2_blob

    # /dir/non_local.txt - File manifest

    non_local_txt_id = '3c304cbdecb34cdb9920ffd6ee139cbc'
    non_local_txt_rts = '95f04f9836704a1582309b21923a0ec2'
    non_local_txt_wts = '839069b2089b496cbc1888c43a30edf4'
    non_local_txt_key = b'0\xba\x9fY\xd1\xb4D\x93\r\xf6\xa7[\xe8\xaa\xf9\xeea\xb8\x01\x98\xc1~im}C\xfa\xde\\\xe6\xa1-'
    non_local_txt_fms = {
        'version': 1,
        'created': '2017-12-02T12:38:30+00:00',
        'updated': '2017-12-02T12:38:45+00:00',
        'blocks': [
            {'id': non_local_txt_block_1_id, 'key': to_jsonb64(non_local_txt_block_1_key), 'offset': 0, 'size': 11},
            {'id': non_local_txt_block_2_id, 'key': to_jsonb64(non_local_txt_block_2_key), 'offset': 11, 'size': 16},
        ],
        'size': 27
    }
    non_local_txt_fm_blob = SecretBox(non_local_txt_key).encrypt(json.dumps(non_local_txt_fms).encode())
    data['backend']['vlobs'][non_local_txt_id] = {
        'blobs': [non_local_txt_fm_blob],
        'rts': non_local_txt_rts,
        'wts': non_local_txt_wts
    }

    # /dir/non_local.txt - No dirty blocks (the file is not in the core)
    # /dir/non_local.txt - No dirty file manifest (the file is not in the core)

    # /dir/modified.txt - Blocks

    modified_txt_block_1_id = '973a198b344d403888472e17b610a43e'
    modified_txt_block_1_key = b'\xc7|\xd7+\xe5\xfbv\xd2\x8c0\xea\r\xff{;2\x0f\xb8s-H\xfd\xfb\xd4\xa157\x86\xde<3\xaa'
    modified_txt_block_1_blob = SecretBox(modified_txt_block_1_key).encrypt(b'This is version 1.')
    data['backend']['blockstore'][modified_txt_block_1_id] = modified_txt_block_1_blob
    data['core']['blocks'][modified_txt_block_1_id] = modified_txt_block_1_blob

    # /dir/modified.txt - File manifest
    # This file manifest should be shadowed by the dirty file manifest in the core

    modified_txt_id = 'ba3d58e140ca44bc91cd53745961397d'
    modified_txt_rts = 'a8340d5831a44592a7941d7aa1d5c187'
    modified_txt_wts = '68ad5d1c3b9e4192834e888cd773ff18'
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
    modified_txt_fm_blob = SecretBox(modified_txt_key).encrypt(json.dumps(modified_txt_fms).encode())
    data['backend']['vlobs'][modified_txt_id] = {
        'blobs': [modified_txt_fm_blob],
        'rts': modified_txt_rts,
        'wts': modified_txt_wts
    }
    data['core']['file_manifests'][modified_txt_id] = modified_txt_fm_blob

    # /dir/modified.txt - Dirty blocks

    modified_txt_dirty_block_1_id = 'a38646aabf264f4fb9db0f636c4999a7'
    modified_txt_dirty_block_1_key = b'=\x04\xc8\x1d\xb6\xb2\x0c\xbf\xaf\xee\x04%zk\x12\xa4\xed\xda\\\xf5\x95\xa1\xf6\x99\x965G|\xca;\x8e\x05'
    modified_txt_dirty_block_1_blob = SecretBox(modified_txt_dirty_block_1_key).encrypt(b'SPARTAAAA !')
    data['core']['dirty_blocks'][modified_txt_dirty_block_1_id] = modified_txt_dirty_block_1_blob

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
    modified_txt_dirty_fm_blob = SecretBox(modified_txt_key).encrypt(json.dumps(modified_txt_dirty_fm).encode())
    data['core']['dirty_file_manifests'][modified_txt_id] = modified_txt_dirty_fm_blob

    # /dir/new.txt - No blocks (given the file is a placeholder so far)
    # /dir/new.txt - No file manifest (given the file is a placeholder so far)
    # /dir/new.txt - Dirty blocks

    new_txt_dirty_block_1_id = 'faa4e1068dad47b4a758a73102478388'
    new_txt_dirty_block_1_key = b'\xab\xcfn\xc8*\xe8|\xc42\xf2\xfao\x1b\xc1Xm\xb4\xb9JBe\x9a1W\r(\xcc\xbd1\x12RB'
    new_txt_dirty_block_1_blob = SecretBox(new_txt_dirty_block_1_key).encrypt(b'Welcome to ')
    data['core']['dirty_blocks'][new_txt_dirty_block_1_id] = new_txt_dirty_block_1_blob

    new_txt_dirty_block_2_id = '4c5b4338a47c462098d6c98856f5bf56'
    new_txt_dirty_block_2_key = b'\xcb\x1c\xe4\x80\x8d\xca\rl?z\xa4\x82J7\xc5\xd5\xed5^\xb6\x05\x8cR;A\xbd\xb1 \xbd\xc2?\xe9'
    new_txt_dirty_block_2_blob = SecretBox(new_txt_dirty_block_2_key).encrypt(b'the new file."')
    data['core']['dirty_blocks'][new_txt_dirty_block_2_id] = new_txt_dirty_block_2_blob

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
    new_txt_placeholder_fm_blob = SecretBox(new_txt_placeholder_key).encrypt(json.dumps(new_txt_placeholder_fm).encode())
    data['core']['placeholder_file_manifests'][new_txt_placeholder_id] = new_txt_placeholder_fm_blob

    # Now user manifest stored in the backend
    userbox = Box(user.privkey, user.pubkey)

    user_manifest_v1 = {
        'type': 'folder',
        'created': '2017-12-02T12:30:23+00:00',
        'children': {}
    }
    user_manifest_v2 = {
        'type': 'folder',
        'created': '2017-12-02T12:30:23+00:00',
        'children': {
            'dir': {
                'type': 'folder',
                'created': '2017-12-02T12:30:23+00:00',
                'children': {
                    'up_to_date.txt': {
                        'type': 'file',
                        'id': up_to_date_txt_id,
                        'read_trust_seed': up_to_date_txt_rts,
                        'write_trust_seed': up_to_date_txt_wts,
                        'key': to_jsonb64(up_to_date_txt_key)
                    }
                }
            }
        }
    }
    user_manifest_v3 = {
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

                    'non_local.txt': {
                        'type': 'file',
                        'id': non_local_txt_id,
                        'read_trust_seed': non_local_txt_rts,
                        'write_trust_seed': non_local_txt_wts,
                        'key': to_jsonb64(up_to_date_txt_key)
                    },
                    'up_to_date.txt': {
                        'type': 'file',
                        'id': up_to_date_txt_id,
                        'read_trust_seed': up_to_date_txt_rts,
                        'write_trust_seed': up_to_date_txt_wts,
                        'key': to_jsonb64(up_to_date_txt_key)
                    },
                    'modified.txt': {
                        'type': 'file',
                        'id': modified_txt_id,
                        'read_trust_seed': modified_txt_rts,
                        'write_trust_seed': modified_txt_wts,
                        'key': to_jsonb64(modified_txt_key)
                    }

                }
            }
        }
    }
    user_manifest_v1_blob = userbox.encrypt(json.dumps(user_manifest_v1).encode())
    user_manifest_v2_blob = userbox.encrypt(json.dumps(user_manifest_v2).encode())
    user_manifest_v3_blob = userbox.encrypt(json.dumps(user_manifest_v3).encode())
    data['backend']['user_vlobs'][user.id] = [user_manifest_v1_blob, user_manifest_v2_blob, user_manifest_v3_blob]

    # Finally, create the local user manifest
    local_user_manifest = {
        'base_version': 3,
        'is_dirty': True,
        'file_placeholders': [new_txt_placeholder_id],
        'tree': deepcopy(user_manifest_v3)
    }
    local_user_manifest['tree']['children']['dir']['children']['new.txt'] = {
        'type': 'placeholder_file',
        'id': new_txt_placeholder_id,
        'key': to_jsonb64(new_txt_placeholder_key)
    }
    local_user_manifest_blob = userbox.encrypt(json.dumps(local_user_manifest).encode())
    data['core']['local_user_manifest'] = local_user_manifest_blob

    return data


async def populate_backend(user, backend):
    data = populate_factory(user)

    # Useful to keep this here to allow tests to retrieve ids
    backend.test_populate_data = data['backend']

    for vlob_id, vlob in data['backend']['vlobs'].items():
        await backend.vlob.create(vlob_id, vlob['rts'], vlob['wts'], vlob['blobs'][0])
        for version, blob in enumerate(vlob['blobs'][1:], 2):
            await backend.vlob.update(vlob_id, vlob['wts'], version, blob)

    for user_id, blobs in data['backend']['user_vlobs'].items():
        for version, blob in enumerate(blobs, 1):
            await backend.user_vlob.update(user_id, version, blob)

    for block_id, block in data['backend']['blockstore'].items():
        await backend.blockstore.post(block_id, block)


def populate_local_storage_cls(user, mocked_local_storage_cls):
    data = populate_factory(user)
    store = mocked_local_storage_cls.test_storage

    store.local_user_manifest = data['core']['local_user_manifest']
    store.dirty_blocks = data['core']['dirty_blocks']
    store.blocks = data['core']['blocks']
    store.file_manifests = data['core']['file_manifests']
    store.dirty_file_manifests = data['core']['dirty_file_manifests']
    store.placeholder_file_manifests = data['core']['placeholder_file_manifests']

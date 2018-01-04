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
    /dir/modified.txt <= regular file with local modifications in core
    /dir/non_local.txt <= regular file, only stored in backend
    /dir/new.txt <= placeholder file, only stored in core
    """
    data = {
        # Useful for simple introspection of data
        'clear': {
            'user_manifests': {
                # <userid>: [<dict>, ...]
            },
            'folder_manifests': {
                # <id>: [<dict>, ...]
            },
            'file_manifests': {
                # <id>: [<dict>, ...]
            },
            'local_user_manifests': {
                # <id>: (<need_sync>, <dict>)
            },
            'local_folder_manifests': {
                # <id>: (<need_sync>, <dict>)
            },
            'local_file_manifests': {
                # <id>: (<need_sync>, <dict>)
            }
        },
        # Data stored in the backend
        'backend': {
            'vlobs': {
                # <id>: {'rts': <str>, 'wts': <str>, 'blobs': [<bytes>, ...]}
            },
            'user_vlobs': {
                # <userid>: [<bytes>, ...]
            },
            'blocks': {
                    # <id>: <bytes>
            }
        },
        # Data stored in the core
        'core': {
            'local_user_manifest': (True, None),  # (<need_sync>, <bytes>)
            'local_manifests': {
                # <id>: <dict>
            },
            'dirty_blocks': {
                # <id>: <bytes>
            },
            'blocks': {
                # <id>: <bytes>
            }
        }

    }

    # Hold my beer...

    ### File manifests ###

    # /dir/up_to_date.txt - Blocks
    up_to_date_txt_block_1_id = '505b0bef5dd44763abc9eac03c765bc3'
    up_to_date_txt_block_1_key = b'\xec\x1d\x84\x80\x05\x18\xb0\x8a\x1d\x81\xe0\xdb\xe5%wx\x9f\x7f\x01\xa6\x8f#>\xc5]\xae|\xfd\x1d\xc22\x05'
    up_to_date_txt_block_1_blob = SecretBox(up_to_date_txt_block_1_key).encrypt(b'Hello from ')
    data['backend']['blocks'][up_to_date_txt_block_1_id] = up_to_date_txt_block_1_blob
    data['core']['blocks'][up_to_date_txt_block_1_id] = up_to_date_txt_block_1_blob
    up_to_date_txt_block_2_id = '0187fa3fc8a5480cbb3ef9df5dd2b7e9'
    up_to_date_txt_block_2_key = b'\xae\x85y\xdd:\xae\xa6\xf2\xdf\xce#U\x17\xffa\xde\x19\x1d\xa7\x84[\xb8\x92{$6\xf9\xc4\x8b\xbcT\x14'
    up_to_date_txt_block_2_blob = SecretBox(up_to_date_txt_block_2_key).encrypt(b'up_to_date.txt !')
    data['backend']['blocks'][up_to_date_txt_block_2_id] = up_to_date_txt_block_2_blob
    data['core']['blocks'][up_to_date_txt_block_2_id] = up_to_date_txt_block_2_blob

    # /dir/up_to_date.txt - File manifest

    up_to_date_txt_id = '1a08acb35bc64ee6acff58b09e1ac939'
    up_to_date_txt_rts = 'c84bf72d941f4a249fe6754033f9214c'
    up_to_date_txt_wts = '03a2902ed1af4e15a60aa4223f8d9453'
    up_to_date_txt_key = b'0\xba\x9fY\xd1\xb4D\x93\r\xf6\xa7[\xe8\xaa\xf9\xeea\xb8\x01\x98\xc1~im}C\xfa\xde\\\xe6\xa1-'
    up_to_date_txt_fm_v1 = {
        'format': 1,
        'type': 'folder_manifest',
        'version': 1,
        'created': '2017-12-02T12:30:30+00:00',
        'updated': '2017-12-02T12:30:40+00:00',
        'blocks': [
            {'id': up_to_date_txt_block_1_id, 'key': to_jsonb64(up_to_date_txt_block_1_key), 'offset': 0, 'size': 11},
        ],
        'size': 11
    }
    up_to_date_txt_fm_v2 = {
        'format': 1,
        'type': 'folder_manifest',
        'version': 2,
        'created': '2017-12-02T12:30:30+00:00',
        'updated': '2017-12-02T12:30:45+00:00',
        'blocks': [
            {'id': up_to_date_txt_block_1_id, 'key': to_jsonb64(up_to_date_txt_block_1_key), 'offset': 0, 'size': 11},
            {'id': up_to_date_txt_block_2_id, 'key': to_jsonb64(up_to_date_txt_block_2_key), 'offset': 11, 'size': 16},
        ],
        'size': 27
    }
    up_to_date_txt_fm_v1_blob = SecretBox(up_to_date_txt_key).encrypt(json.dumps(up_to_date_txt_fm_v1).encode())
    up_to_date_txt_fm_v2_blob = SecretBox(up_to_date_txt_key).encrypt(json.dumps(up_to_date_txt_fm_v2).encode())
    data['backend']['vlobs'][up_to_date_txt_id] = {
        'blobs': [up_to_date_txt_fm_v1_blob, up_to_date_txt_fm_v2_blob],
        'rts': up_to_date_txt_rts,
        'wts': up_to_date_txt_wts
    }
    data['clear']['file_manifests'][up_to_date_txt_id] = [up_to_date_txt_fm_v1, up_to_date_txt_fm_v2]

    # /dir/up_to_date.txt - No dirty blocks (the file is up to date...)

    # /dir/up_to_date.txt - Local file manifest
    up_to_date_txt_lfm = {
        'format': 1,
        'type': 'local_file_manifest',
        'base_version': 2,
        'need_sync': False,
        'created': '2017-12-02T12:30:30+00:00',
        'updated': '2017-12-02T12:30:45+00:00',
        'blocks': [
            {'id': up_to_date_txt_block_1_id, 'key': to_jsonb64(up_to_date_txt_block_1_key), 'offset': 0, 'size': 11},
            {'id': up_to_date_txt_block_2_id, 'key': to_jsonb64(up_to_date_txt_block_2_key), 'offset': 11, 'size': 16},
        ],
        'dirty_blocks': [
        ],
        'size': 27
    }
    up_to_date_txt_lfm_blob = SecretBox(up_to_date_txt_key).encrypt(json.dumps(up_to_date_txt_lfm).encode())
    data['core']['local_manifests'][up_to_date_txt_id] = up_to_date_txt_lfm_blob
    data['clear']['local_file_manifests'][up_to_date_txt_id] = up_to_date_txt_lfm

    # /dir/non_local.txt - Blocks
    non_local_txt_block_1_id = '74ab15c511734fed86163944586e721a'
    non_local_txt_block_1_key = b'\xec\x1d\x84\x80\x05\x18\xb0\x8a\x1d\x81\xe0\xdb\xe5%wx\x9f\x7f\x01\xa6\x8f#>\xc5]\xae|\xfd\x1d\xc22\x05'
    non_local_txt_block_1_blob = SecretBox(non_local_txt_block_1_key).encrypt(b'Hello from ')
    data['backend']['blocks'][non_local_txt_block_1_id] = non_local_txt_block_1_blob
    data['core']['blocks'][non_local_txt_block_1_id] = non_local_txt_block_1_blob
    non_local_txt_block_2_id = 'bc9d482e76f54b21bf96532272defc43'
    non_local_txt_block_2_key = b'\xae\x85y\xdd:\xae\xa6\xf2\xdf\xce#U\x17\xffa\xde\x19\x1d\xa7\x84[\xb8\x92{$6\xf9\xc4\x8b\xbcT\x14'
    non_local_txt_block_2_blob = SecretBox(non_local_txt_block_2_key).encrypt(b'non_local.txt !')
    data['backend']['blocks'][non_local_txt_block_2_id] = non_local_txt_block_2_blob
    data['core']['blocks'][non_local_txt_block_2_id] = non_local_txt_block_2_blob

    # /dir/non_local.txt - File manifest

    non_local_txt_id = '3c304cbdecb34cdb9920ffd6ee139cbc'
    non_local_txt_rts = '95f04f9836704a1582309b21923a0ec2'
    non_local_txt_wts = '839069b2089b496cbc1888c43a30edf4'
    non_local_txt_key = b'0\xba\x9fY\xd1\xb4D\x93\r\xf6\xa7[\xe8\xaa\xf9\xeea\xb8\x01\x98\xc1~im}C\xfa\xde\\\xe6\xa1-'
    non_local_txt_fm = {
        'format': 1,
        'type': 'file_manifest',
        'version': 1,
        'created': '2017-12-02T12:38:30+00:00',
        'updated': '2017-12-02T12:38:45+00:00',
        'blocks': [
            {'id': non_local_txt_block_1_id, 'key': to_jsonb64(non_local_txt_block_1_key), 'offset': 0, 'size': 11},
            {'id': non_local_txt_block_2_id, 'key': to_jsonb64(non_local_txt_block_2_key), 'offset': 11, 'size': 16},
        ],
        'size': 27
    }
    non_local_txt_fm_blob = SecretBox(non_local_txt_key).encrypt(json.dumps(non_local_txt_fm).encode())
    data['backend']['vlobs'][non_local_txt_id] = {
        'blobs': [non_local_txt_fm_blob],
        'rts': non_local_txt_rts,
        'wts': non_local_txt_wts
    }
    data['clear']['file_manifests'][up_to_date_txt_id] = [non_local_txt_fm]

    # /dir/non_local.txt - No dirty blocks (the file is not in the core)
    # /dir/non_local.txt - No local file manifest (the file is not in the core)

    # /dir/modified.txt - Blocks

    modified_txt_block_1_id = '973a198b344d403888472e17b610a43e'
    modified_txt_block_1_key = b'\xc7|\xd7+\xe5\xfbv\xd2\x8c0\xea\r\xff{;2\x0f\xb8s-H\xfd\xfb\xd4\xa157\x86\xde<3\xaa'
    modified_txt_block_1_blob = SecretBox(modified_txt_block_1_key).encrypt(b'This is version 1.')
    data['backend']['blocks'][modified_txt_block_1_id] = modified_txt_block_1_blob
    data['core']['blocks'][modified_txt_block_1_id] = modified_txt_block_1_blob

    modified_txt_block_2_id = 'cd0396c3c59a453d9fe3253a8c443252'
    modified_txt_block_2_key = b"\xdaT'jU\x0cra}\x02=\xfe\x05\xdbf\xbf\x86\xa3@F\xc6?\x1d\xadg%\xdb\x146\x91n\xde"
    modified_txt_block_2_blob = SecretBox(modified_txt_block_2_key).encrypt(b'This is version 2.')
    data['backend']['blocks'][modified_txt_block_2_id] = modified_txt_block_2_blob
    data['core']['blocks'][modified_txt_block_2_id] = modified_txt_block_2_blob

    # /dir/modified.txt - File manifest
    # This file manifest should be shadowed by the local file manifest in the core

    modified_txt_id = 'ba3d58e140ca44bc91cd53745961397d'
    modified_txt_rts = 'a8340d5831a44592a7941d7aa1d5c187'
    modified_txt_wts = '68ad5d1c3b9e4192834e888cd773ff18'
    modified_txt_key = b'0\xba\x9fY\xd1\xb4D\x93\r\xf6\xa7[\xe8\xaa\xf9\xeea\xb8\x01\x98\xc1~im}C\xfa\xde\\\xe6\xa1-'
    modified_txt_fm_v1 = {
        'format': 1,
        'type': 'file_manifest',
        'version': 1,
        'created': '2017-12-02T12:50:30+00:00',
        'updated': '2017-12-02T12:50:40+00:00',
        'blocks': [
            {'id': modified_txt_block_1_id, 'key': to_jsonb64(modified_txt_block_1_key), 'offset': 0, 'size': 18},
        ],
        'size': 18
    }
    modified_txt_fm_v2 = {
        'format': 1,
        'type': 'file_manifest',
        'version': 2,
        'created': '2017-12-02T12:50:30+00:00',
        'updated': '2017-12-02T12:50:45+00:00',
        'blocks': [
            {'id': modified_txt_block_2_id, 'key': to_jsonb64(modified_txt_block_2_key), 'offset': 0, 'size': 18},
        ],
        'size': 18
    }
    modified_txt_fm_v1_blob = SecretBox(modified_txt_key).encrypt(json.dumps(modified_txt_fm_v1).encode())
    modified_txt_fm_v2_blob = SecretBox(modified_txt_key).encrypt(json.dumps(modified_txt_fm_v2).encode())
    data['backend']['vlobs'][modified_txt_id] = {
        'blobs': [modified_txt_fm_v1_blob, modified_txt_fm_v2_blob],
        'rts': modified_txt_rts,
        'wts': modified_txt_wts
    }
    data['clear']['file_manifests'][modified_txt_id] = [modified_txt_fm_v1, modified_txt_fm_v2]
    data['core']['local_manifests'][modified_txt_id] = modified_txt_fm_v2_blob

    # /dir/modified.txt - Dirty blocks

    modified_txt_dirty_block_1_id = 'a38646aabf264f4fb9db0f636c4999a7'
    modified_txt_dirty_block_1_key = b'=\x04\xc8\x1d\xb6\xb2\x0c\xbf\xaf\xee\x04%zk\x12\xa4\xed\xda\\\xf5\x95\xa1\xf6\x99\x965G|\xca;\x8e\x05'
    modified_txt_dirty_block_1_blob = SecretBox(modified_txt_dirty_block_1_key).encrypt(b'SPARTAAAA !')
    data['core']['dirty_blocks'][modified_txt_dirty_block_1_id] = modified_txt_dirty_block_1_blob

    # /dir/modified.txt - Local file manifest
    modified_txt_lfm = {
        'format': 1,
        'type': 'local_file_manifest',
        'need_sync': True,
        'base_version': 2,
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
    modified_txt_lfm_blob = SecretBox(modified_txt_key).encrypt(json.dumps(modified_txt_lfm).encode())
    data['core']['local_manifests'][modified_txt_id] = modified_txt_lfm_blob
    data['clear']['local_file_manifests'][modified_txt_id] = modified_txt_lfm

    # /dir/new.txt - No blocks (given the file is a placeholder so far)
    # /dir/new.txt - No file manifest (given the file is a placeholder so far)
    # /dir/new.txt - Dirty blocks

    new_txt_dirty_block_1_id = 'faa4e1068dad47b4a758a73102478388'
    new_txt_dirty_block_1_key = b'\xab\xcfn\xc8*\xe8|\xc42\xf2\xfao\x1b\xc1Xm\xb4\xb9JBe\x9a1W\r(\xcc\xbd1\x12RB'
    new_txt_dirty_block_1_blob = SecretBox(new_txt_dirty_block_1_key).encrypt(b'Welcome to ')
    data['core']['dirty_blocks'][new_txt_dirty_block_1_id] = new_txt_dirty_block_1_blob

    new_txt_dirty_block_2_id = '4c5b4338a47c462098d6c98856f5bf56'
    new_txt_dirty_block_2_key = b'\xcb\x1c\xe4\x80\x8d\xca\rl?z\xa4\x82J7\xc5\xd5\xed5^\xb6\x05\x8cR;A\xbd\xb1 \xbd\xc2?\xe9'
    new_txt_dirty_block_2_blob = SecretBox(new_txt_dirty_block_2_key).encrypt(b'the new file.')
    data['core']['dirty_blocks'][new_txt_dirty_block_2_id] = new_txt_dirty_block_2_blob

    # /dir/new.txt - Local file manifest
    new_txt_placeholder_id = '3ca6cb2ba8a9446f8508296b7a8c3ed4'
    new_txt_key = b'"\x08"Q\xfbc\xa3 \xf9\xde\xbf\xc3\x07?\x9a\xa6V\xcet\x0c\xa1C\xf2\xa06\xa1\xc9 \xbf\xf6t\xbb'
    new_txt_lfm = {
        'format': 1,
        'type': 'local_file_manifest',
        'need_sync': True,
        'base_version': 0,
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
    new_txt_lfm_blob = SecretBox(new_txt_key).encrypt(json.dumps(new_txt_lfm).encode())
    data['core']['local_manifests'][new_txt_placeholder_id] = new_txt_lfm_blob
    data['clear']['local_file_manifests'][new_txt_placeholder_id] = new_txt_lfm

    ### Folder manifests ###

    # /dir/ - Folder manifest

    dir_id = '9405ac34949644bb847528bf3911e16d'
    dir_rts = 'a75c2bd100634a84b8ff124a361336c8'
    dir_wts = '45bf72b4181b47cdaac0ca64a5b62d98'
    dir_key = b"\xb5\xf7'd\x85\x17U\xc4\x9b\x94\xdb\n\x1a\xd2mD\xe18\xff_F\xfc(\r\x8a\xc7\xec7}\xeb\x00\xb1"
    dir_fm_v1 = {
        'format': 1,
        'type': 'folder_manifest',
        'version': 1,
        'created': '2017-12-01T12:50:30+00:00',
        'updated': '2017-12-01T12:50:30+00:00',
        'children': {
            'up_to_date.txt': {
                'id': up_to_date_txt_id,
                'rts': up_to_date_txt_rts,
                'wts': up_to_date_txt_wts,
                'key': to_jsonb64(up_to_date_txt_key)
            },
            'modified.txt': {
                'id': modified_txt_id,
                'rts': modified_txt_rts,
                'wts': modified_txt_wts,
                'key': to_jsonb64(modified_txt_key)
            }
        }
    }
    dir_fm_v2 = {
        'format': 1,
        'type': 'folder_manifest',
        'version': 2,
        'created': '2017-12-01T12:50:30+00:00',
        'updated': '2017-12-01T12:50:55+00:00',
        'children': {
            'up_to_date.txt': {
                'id': up_to_date_txt_id,
                'rts': up_to_date_txt_rts,
                'wts': up_to_date_txt_wts,
                'key': to_jsonb64(up_to_date_txt_key)
            },
            'modified.txt': {
                'id': modified_txt_id,
                'rts': modified_txt_rts,
                'wts': modified_txt_wts,
                'key': to_jsonb64(modified_txt_key)
            },
            'non_local.txt': {
                'id': non_local_txt_id,
                'rts': non_local_txt_rts,
                'wts': non_local_txt_wts,
                'key': to_jsonb64(non_local_txt_key)
            }
        }
    }
    dir_fm_v1_blob = SecretBox(dir_key).encrypt(json.dumps(dir_fm_v1).encode())
    dir_fm_v2_blob = SecretBox(dir_key).encrypt(json.dumps(dir_fm_v2).encode())
    data['backend']['vlobs'][dir_id] = {
        'blobs': [dir_fm_v1_blob, dir_fm_v2_blob],
        'rts': dir_rts,
        'wts': dir_wts
    }
    data['clear']['folder_manifests'][dir_id] = [dir_fm_v1, dir_fm_v2]

    # /dir/ - Local folder manifest

    dir_lfm = {
        'format': 1,
        'type': 'local_folder_manifest',
        'need_sync': True,
        'base_version': 1,  # Note the local manifest is out of date !
        'created': '2017-12-01T12:50:30+00:00',
        'updated': '2017-12-02T12:50:56+00:00',
        'children': {
            'up_to_date.txt': {
                'type': 'vlob',
                'id': up_to_date_txt_id,
                'rts': up_to_date_txt_rts,
                'wts': up_to_date_txt_wts,
                'key': to_jsonb64(up_to_date_txt_key)
            },
            'modified.txt': {
                'type': 'vlob',
                'id': modified_txt_id,
                'rts': modified_txt_rts,
                'wts': modified_txt_wts,
                'key': to_jsonb64(modified_txt_key)
            },
            'new.txt': {
                'type': 'placeholder',
                'id': new_txt_placeholder_id,
                'key': to_jsonb64(new_txt_key)
            }
        }
    }
    dir_lfm_blob = SecretBox(dir_key).encrypt(json.dumps(dir_lfm).encode())
    data['core']['local_manifests'][dir_id] = dir_lfm_blob
    data['clear']['local_folder_manifests'][dir_id] = dir_lfm

    # /empty_dir/ - Folder manifest

    empty_dir_id = '81134b43bae146f78cea2d9f55eba0d8'
    empty_dir_rts = '75da877593ad47248ed8667f1f7d4f6d'
    empty_dir_wts = 'b96d7b6800084071a8fe0f1d1eb04e74'
    empty_dir_key = b'\x0e\xdd@\xd3\xb9\x1e\xf5d5Kz\xadib>\xa0yt%\x9aw\xf0/\x99\n\x1b\xde\xeb\x85\x98\xf7^'
    empty_dir_fm_v1 = {
        'format': 1,
        'type': 'folder_manifest',
        'version': 1,
        'created': '2017-12-01T12:50:30+00:00',
        'updated': '2017-12-01T12:50:30+00:00',
        'children': {
        }
    }
    empty_dir_fm_v1_blob = SecretBox(empty_dir_key).encrypt(json.dumps(empty_dir_fm_v1).encode())
    data['backend']['vlobs'][empty_dir_id] = {
        'blobs': [empty_dir_fm_v1_blob],
        'rts': empty_dir_rts,
        'wts': empty_dir_wts
    }
    data['clear']['folder_manifests'][empty_dir_id] = [empty_dir_fm_v1]

    # /empty_dir/ - Local folder manifest
    empty_dir_lfm = {
        'format': 1,
        'type': 'local_folder_manifest',
        'need_sync': False,
        'base_version': 1,
        'created': '2017-12-01T12:50:30+00:00',
        'updated': '2017-12-01T12:50:30+00:00',
        'children': {
        }
    }
    empty_dir_lfm_blob = SecretBox(empty_dir_key).encrypt(json.dumps(empty_dir_lfm).encode())
    data['core']['local_manifests'][empty_dir_id] = empty_dir_lfm_blob
    data['clear']['local_folder_manifests'][empty_dir_id] = empty_dir_lfm

    # Carry on ! Almost there !

    userbox = Box(user.privkey, user.pubkey)

    # / - User manifest (storing root folder data)

    user_manifest_v1 = {
        'format': 1,
        'type': 'user_manifest',
        'version': 1,
        'created': '2017-12-01T12:50:30+00:00',
        'updated': '2017-12-01T12:50:30+00:00',
        'children': {
        }
    }
    user_manifest_v2 = {
        **user_manifest_v1,
        'version': 2,
        'updated': '2017-12-02T12:50:30+00:00',
        'children': {
            'empty_dir': {
                'id': empty_dir_id,
                'rts': empty_dir_rts,
                'wts': empty_dir_wts,
                'key': to_jsonb64(empty_dir_key)
            }
        }
    }
    user_manifest_v3 = {
        **user_manifest_v1,
        'version': 3,
        'updated': '2017-12-03T12:50:30+00:00',
        'children': {
            'empty_dir': {
                'id': empty_dir_id,
                'rts': empty_dir_rts,
                'wts': empty_dir_wts,
                'key': to_jsonb64(empty_dir_key)
            },
            'dir': {
                'id': dir_id,
                'rts': dir_rts,
                'wts': dir_wts,
                'key': to_jsonb64(dir_key)
            }
        }
    }
    user_manifest_v1_blob = userbox.encrypt(json.dumps(user_manifest_v1).encode())
    user_manifest_v2_blob = userbox.encrypt(json.dumps(user_manifest_v2).encode())
    user_manifest_v3_blob = userbox.encrypt(json.dumps(user_manifest_v3).encode())
    data['backend']['user_vlobs'][user.id] = [user_manifest_v1_blob, user_manifest_v2_blob, user_manifest_v3_blob]
    data['clear']['user_manifests'][user.id] = [user_manifest_v1, user_manifest_v2, user_manifest_v3]

    # / - Local user manifest

    local_user_manifest = {
        'format': 1,
        'type': 'local_user_manifest',
        'need_sync': False,
        'base_version': 3,
        'created': '2017-12-01T12:50:30+00:00',
        'updated': '2017-12-03T12:50:30+00:00',
        'children': {
            'empty_dir': {
                'type': 'vlob',
                'id': empty_dir_id,
                'rts': empty_dir_rts,
                'wts': empty_dir_wts,
                'key': to_jsonb64(empty_dir_key)
            },
            'dir': {
                'type': 'vlob',
                'id': dir_id,
                'rts': dir_rts,
                'wts': dir_wts,
                'key': to_jsonb64(dir_key)
            }
        }
    }
    local_user_manifest_blob = userbox.encrypt(json.dumps(local_user_manifest).encode())
    data['core']['local_user_manifest'] = local_user_manifest_blob
    data['clear']['local_user_manifests'][user.id] = local_user_manifest

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

    for block_id, block in data['backend']['blocks'].items():
        await backend.blockstore.post(block_id, block)


def populate_local_storage_cls(user, mocked_local_storage_cls):
    data = populate_factory(user)
    store = mocked_local_storage_cls.test_storage

    store.dirty_blocks = data['core']['dirty_blocks']
    store.blocks = data['core']['blocks']
    store.local_user_manifest = data['core']['local_user_manifest']
    store.local_manifests = data['core']['local_manifests']


def populate_core(core, user):
    data = populate_factory(user)
    store = core.fs.manifests_manager._local_storage

    store.dirty_blocks = data['core']['dirty_blocks']
    store.blocks = data['core']['blocks']
    store.local_user_manifest = data['core']['local_user_manifest']
    store.local_manifests = data['core']['local_manifests']

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from uuid import UUID

from parsec._parsec import SecretKey, HashDigest
from protocol.utils import *
from parsec.api.protocol import *
from parsec.api.data import *
from parsec.core.types import *

################### LocalUserManifest ##################


lum = LocalUserManifest(
    need_sync=True,
    updated=NOW,
    base=UserManifest(
        author=ALICE.device_id,
        timestamp=NOW,
        id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
        version=42,
        created=NOW,
        updated=NOW,
        last_processed_message=3,
        workspaces=(
            WorkspaceEntry(
                name=EntryName("wksp1"),
                id=EntryID.from_hex("b82954f1138b4d719b7f5bd78915d20f"),
                key=SecretKey(
                    unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
                ),
                encryption_revision=2,
                encrypted_on=NOW,
                role_cached_on=NOW,
                role=RealmRole.OWNER,
            ),
        ),
    ),
    last_processed_message=4,
    workspaces=(
        WorkspaceEntry(
            name=EntryName("wksp1"),
            id=EntryID.from_hex("b82954f1138b4d719b7f5bd78915d20f"),
            key=SecretKey(
                unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
            ),
            encryption_revision=2,
            encrypted_on=NOW,
            role_cached_on=NOW,
            role=RealmRole.OWNER,
        ),
        WorkspaceEntry(
            name=EntryName("wksp2"),
            id=EntryID.from_hex("d7e3af6a03e1414db0f4682901e9aa4b"),
            key=SecretKey(
                unhexlify("c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc")
            ),
            encryption_revision=1,
            encrypted_on=NOW,
            role_cached_on=NOW,
            role=None,
        ),
    ),
    speculative=False,
).dump_and_encrypt(key=KEY)
display("local user manifest", lum, [KEY])

lum_synced = LocalUserManifest(
    need_sync=False,
    updated=NOW,
    base=UserManifest(
        author=ALICE.device_id,
        timestamp=NOW,
        id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
        version=42,
        created=NOW,
        updated=NOW,
        last_processed_message=3,
        workspaces=(
            WorkspaceEntry(
                name=EntryName("wksp1"),
                id=EntryID.from_hex("b82954f1138b4d719b7f5bd78915d20f"),
                key=SecretKey(
                    unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
                ),
                encryption_revision=2,
                encrypted_on=NOW,
                role_cached_on=NOW,
                role=RealmRole.OWNER,
            ),
        ),
    ),
    last_processed_message=3,
    workspaces=(
        WorkspaceEntry(
            name=EntryName("wksp1"),
            id=EntryID.from_hex("b82954f1138b4d719b7f5bd78915d20f"),
            key=SecretKey(
                unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
            ),
            encryption_revision=2,
            encrypted_on=NOW,
            role_cached_on=NOW,
            role=RealmRole.OWNER,
        ),
    ),
    speculative=False,
).dump_and_encrypt(key=KEY)
display("local user manifest synced", lum_synced, [KEY])

lum_speculative = LocalUserManifest(
    need_sync=True,
    updated=NOW,
    base=UserManifest(
        author=ALICE.device_id,
        timestamp=NOW,
        id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
        version=0,
        created=NOW,
        updated=NOW,
        last_processed_message=0,
        workspaces=(),
    ),
    last_processed_message=0,
    workspaces=(),
    speculative=True,
).dump_and_encrypt(key=KEY)
display("local user manifest speculative", lum_speculative, [KEY])

raw_legacy_no_speculative_field = {
    "type": "local_user_manifest",
    "need_sync": False,
    "updated": NOW,
    "base": {
        "type": "user_manifest",
        "author": ALICE.device_id.str,
        "timestamp": NOW,
        "id": UUID("87c6b5fd3b454c94bab51d6af1c6930b"),
        "version": 0,
        "created": NOW,
        "updated": NOW,
        "last_processed_message": 0,
        "workspaces": [],
    },
    "last_processed_message": 0,
    "workspaces": [],
}
lum_legacy_no_speculative_field = KEY.encrypt(packb(raw_legacy_no_speculative_field))
LocalUserManifest.decrypt_and_load(
    lum_legacy_no_speculative_field, key=KEY
)  # Make sure data is valid
display(
    "local user manifest legacy no speculative field",
    lum_legacy_no_speculative_field,
    [KEY],
)


################### LocalWorkspaceManifest ##################


wm = LocalWorkspaceManifest(
    base=WorkspaceManifest(
        author=ALICE.device_id,
        timestamp=NOW,
        id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
        version=42,
        created=NOW,
        updated=NOW,
        children={EntryName("wksp1"): EntryID.from_hex("b82954f1138b4d719b7f5bd78915d20f")},
    ),
    need_sync=True,
    updated=NOW,
    children={EntryName("wksp2"): EntryID.from_hex("d7e3af6a03e1414db0f4682901e9aa4b")},
    local_confinement_points={EntryID.from_hex("d7e3af6a03e1414db0f4682901e9aa4b")},
    remote_confinement_points={EntryID.from_hex("b82954f1138b4d719b7f5bd78915d20f")},
    speculative=False,
).dump_and_encrypt(key=KEY)
display("workspace manifest", wm, [KEY])

wm_speculative = LocalWorkspaceManifest(
    base=WorkspaceManifest(
        author=ALICE.device_id,
        timestamp=NOW,
        id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
        version=0,
        created=NOW,
        updated=NOW,
        children={},
    ),
    need_sync=True,
    updated=NOW,
    children={},
    local_confinement_points=set(),
    remote_confinement_points=set(),
    speculative=True,
).dump_and_encrypt(key=KEY)
display("workspace manifest speculative", wm_speculative, [KEY])

raw_wm_legacy_no_confinment_and_speculative_fields = {
    "type": "local_workspace_manifest",
    "base": {
        "type": "workspace_manifest",
        "author": ALICE.device_id.str,
        "timestamp": NOW,
        "id": UUID("87c6b5fd3b454c94bab51d6af1c6930b"),
        "version": 42,
        "created": NOW,
        "updated": NOW,
        "children": {"wksp2": UUID("d7e3af6a03e1414db0f4682901e9aa4b")},
    },
    "need_sync": False,
    "updated": NOW,
    "children": {"wksp2": UUID("d7e3af6a03e1414db0f4682901e9aa4b")},
}
wm_legacy_no_confinment_and_speculative_fields = KEY.encrypt(
    packb(raw_wm_legacy_no_confinment_and_speculative_fields)
)
LocalWorkspaceManifest.decrypt_and_load(wm_legacy_no_confinment_and_speculative_fields, KEY)
display(
    "workspace manifest legacy no confinment and speculative fields",
    wm_legacy_no_confinment_and_speculative_fields,
    [KEY],
)


################### LocalFolderManifest ##################


fm = LocalFolderManifest(
    base=FolderManifest(
        author=ALICE.device_id,
        timestamp=NOW,
        id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
        parent=EntryID.from_hex("07748fbf67a646428427865fd730bf3e"),
        version=42,
        created=NOW,
        updated=NOW,
        children={EntryName("wksp1"): EntryID.from_hex("b82954f1138b4d719b7f5bd78915d20f")},
    ),
    need_sync=True,
    updated=NOW,
    children={EntryName("wksp2"): EntryID.from_hex("d7e3af6a03e1414db0f4682901e9aa4b")},
    local_confinement_points={EntryID.from_hex("d7e3af6a03e1414db0f4682901e9aa4b")},
    remote_confinement_points={EntryID.from_hex("b82954f1138b4d719b7f5bd78915d20f")},
).dump_and_encrypt(key=KEY)
display("folder manifest", fm, [KEY])

raw_fm_legacy_no_confinment_fields = {
    "type": "local_folder_manifest",
    "base": {
        "type": "folder_manifest",
        "author": ALICE.device_id.str,
        "timestamp": NOW,
        "id": UUID("87c6b5fd3b454c94bab51d6af1c6930b"),
        "parent": UUID("07748fbf67a646428427865fd730bf3e"),
        "version": 42,
        "created": NOW,
        "updated": NOW,
        "children": {"wksp2": UUID("d7e3af6a03e1414db0f4682901e9aa4b")},
    },
    "need_sync": False,
    "updated": NOW,
    "children": {"wksp2": UUID("d7e3af6a03e1414db0f4682901e9aa4b")},
}
fm_legacy_no_confinment_fields = KEY.encrypt(packb(raw_fm_legacy_no_confinment_fields))
LocalFolderManifest.decrypt_and_load(fm_legacy_no_confinment_fields, KEY)
display("folder manifest legacy no confinment fields", fm_legacy_no_confinment_fields, [KEY])


# ################### LocalFileManifest ##################


fm = LocalFileManifest(
    need_sync=True,
    updated=NOW,
    size=500,
    blocksize=512,
    blocks=(
        (
            Chunk(
                id=ChunkID.from_hex("ad67b6b5b9ad4653bf8e2b405bb6115f"),
                start=0,
                stop=250,
                raw_offset=0,
                raw_size=512,
                access=BlockAccess(
                    id=BlockID.from_hex("b82954f1138b4d719b7f5bd78915d20f"),
                    key=SecretKey(
                        unhexlify(
                            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
                        )
                    ),
                    offset=0,
                    size=512,
                    digest=HashDigest(
                        unhexlify(
                            "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                        )
                    ),
                ),
            ),
            Chunk(
                id=ChunkID.from_hex("2f99258022a94555b3109e81d34bdf97"),
                start=0,
                stop=250,
                raw_offset=250,
                raw_size=250,
                access=None,
            ),
        ),
    ),
    base=FileManifest(
        author=ALICE.device_id,
        timestamp=NOW,
        id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
        parent=EntryID.from_hex("07748fbf67a646428427865fd730bf3e"),
        version=42,
        created=NOW,
        updated=NOW,
        size=700,
        blocksize=512,
        blocks=[
            BlockAccess(
                id=BlockID.from_hex("b82954f1138b4d719b7f5bd78915d20f"),
                key=SecretKey(
                    unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
                ),
                offset=0,
                size=512,
                digest=HashDigest(
                    unhexlify("076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560")
                ),
            ),
            BlockAccess(
                id=BlockID.from_hex("d7e3af6a03e1414db0f4682901e9aa4b"),
                key=SecretKey(
                    unhexlify("c21ed3aae92c648cb1b6df8be149ebc872247db0dbd37686ff2d075e2d7505cc")
                ),
                offset=512,
                size=188,
                digest=HashDigest(
                    unhexlify("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
                ),
            ),
        ],
    ),
).dump_and_encrypt(key=KEY)
display("file manifest", fm, [KEY])


fm_invalid_blocksize = LocalFileManifest(
    need_sync=True,
    updated=NOW,
    size=500,
    blocksize=2,
    blocks=[],
    base=FileManifest(
        author=ALICE.device_id,
        timestamp=NOW,
        id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
        parent=EntryID.from_hex("07748fbf67a646428427865fd730bf3e"),
        version=42,
        created=NOW,
        updated=NOW,
        size=700,
        blocksize=512,
        blocks=[],
    ),
).dump_and_encrypt(key=KEY)
display("file manifest invalid blocksize", fm_invalid_blocksize, [KEY])

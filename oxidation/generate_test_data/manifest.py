# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa
# cspell: ignore fdrm

from protocol.utils import *
from parsec._parsec import SecretKey, HashDigest
from parsec.api.protocol import *
from parsec.api.data import *


um = UserManifest(
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
).dump_sign_and_encrypt(author_signkey=ALICE.signing_key, key=KEY)
display("user manifest", um, [KEY, ALICE.verify_key, "zip"])

wm = WorkspaceManifest(
    author=ALICE.device_id,
    timestamp=NOW,
    id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
    version=42,
    created=NOW,
    updated=NOW,
    children={
        EntryName("wksp1"): EntryID.from_hex("b82954f1138b4d719b7f5bd78915d20f"),
        EntryName("wksp2"): EntryID.from_hex("d7e3af6a03e1414db0f4682901e9aa4b"),
    },
).dump_sign_and_encrypt(author_signkey=ALICE.signing_key, key=KEY)
display("workspace manifest", wm, [KEY, ALICE.verify_key, "zip"])

fdrm = FolderManifest(
    author=ALICE.device_id,
    timestamp=NOW,
    id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
    parent=EntryID.from_hex("07748fbf67a646428427865fd730bf3e"),
    version=42,
    created=NOW,
    updated=NOW,
    children={
        EntryName("wksp1"): EntryID.from_hex("b82954f1138b4d719b7f5bd78915d20f"),
        EntryName("wksp2"): EntryID.from_hex("d7e3af6a03e1414db0f4682901e9aa4b"),
    },
).dump_sign_and_encrypt(author_signkey=ALICE.signing_key, key=KEY)
display("folder manifest", fdrm, [KEY, ALICE.verify_key, "zip"])

film = FileManifest(
    author=ALICE.device_id,
    timestamp=NOW,
    id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
    parent=EntryID.from_hex("07748fbf67a646428427865fd730bf3e"),
    version=42,
    created=NOW,
    updated=NOW,
    size=700,
    blocksize=512,
    blocks=(
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
    ),
).dump_sign_and_encrypt(author_signkey=ALICE.signing_key, key=KEY)
display("file manifest", film, [KEY, ALICE.verify_key, "zip"])

film_invalid_blocksize = FileManifest(
    author=ALICE.device_id,
    timestamp=NOW,
    id=EntryID.from_hex("87c6b5fd3b454c94bab51d6af1c6930b"),
    parent=EntryID.from_hex("07748fbf67a646428427865fd730bf3e"),
    version=42,
    created=NOW,
    updated=NOW,
    size=700,
    blocksize=2,
    blocks=[],
).dump_sign_and_encrypt(author_signkey=ALICE.signing_key, key=KEY)
display(
    "file manifest invalid blocksize",
    film_invalid_blocksize,
    [KEY, ALICE.verify_key, "zip"],
)

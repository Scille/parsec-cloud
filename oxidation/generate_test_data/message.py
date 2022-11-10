# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from protocol.utils import *
from parsec.api.data import *


sgmc = SharingGrantedMessageContent(
    author=ALICE.device_id,
    timestamp=NOW,
    name=EntryName("wksp1"),
    id=EntryID.from_str("87c6b5fd3b454c94bab51d6af1c6930b"),
    encryption_revision=3,
    encrypted_on=NOW,
    key=KEY,
).dump_sign_and_encrypt_for(author_signkey=ALICE.signing_key, recipient_pubkey=BOB.public_key)
display("message sharing_granted", sgmc, [BOB.private_key, ALICE.verify_key, "zip"])

srctmc = SharingReencryptedMessageContent(
    author=ALICE.device_id,
    timestamp=NOW,
    name=EntryName("wksp1"),
    id=EntryID.from_str("87c6b5fd3b454c94bab51d6af1c6930b"),
    encryption_revision=3,
    encrypted_on=NOW,
    key=KEY,
).dump_sign_and_encrypt_for(author_signkey=ALICE.signing_key, recipient_pubkey=BOB.public_key)
display("message sharing_reencrypted", sgmc, [BOB.private_key, ALICE.verify_key, "zip"])

srvkmc = SharingRevokedMessageContent(
    author=ALICE.device_id,
    timestamp=NOW,
    id=EntryID.from_str("87c6b5fd3b454c94bab51d6af1c6930b"),
).dump_sign_and_encrypt_for(author_signkey=ALICE.signing_key, recipient_pubkey=BOB.public_key)
display("message sharing_revoked", sgmc, [BOB.private_key, ALICE.verify_key, "zip"])

pmc = PingMessageContent(
    author=ALICE.device_id, timestamp=NOW, ping="foo"
).dump_sign_and_encrypt_for(author_signkey=ALICE.signing_key, recipient_pubkey=BOB.public_key)
display("message ping", sgmc, [BOB.private_key, ALICE.verify_key, "zip"])

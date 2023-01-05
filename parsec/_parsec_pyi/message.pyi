# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec_pyi.crypto import PrivateKey, PublicKey, SecretKey, SigningKey, VerifyKey
from parsec._parsec_pyi.ids import DeviceID, EntryID
from parsec._parsec_pyi.manifest import EntryName
from parsec._parsec_pyi.time import DateTime

class MessageContent:
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @classmethod
    def decrypt_verify_and_load_for(
        cls,
        ciphered: bytes,
        recipient_privkey: PrivateKey,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
        expected_timestamp: DateTime,
    ) -> MessageContent: ...
    def dump_sign_and_encrypt_for(
        self,
        author_signkey: SigningKey,
        recipient_pubkey: PublicKey,
    ) -> bytes: ...

class SharingGrantedMessageContent(MessageContent):
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        name: EntryName,
        id: EntryID,
        encryption_revision: int,
        encrypted_on: DateTime,
        key: SecretKey,
    ) -> None: ...
    @property
    def name(self) -> EntryName: ...
    @property
    def id(self) -> EntryID: ...
    @property
    def encryption_revision(self) -> int: ...
    @property
    def encrypted_on(self) -> DateTime: ...
    @property
    def key(self) -> SecretKey: ...

class SharingReencryptedMessageContent(MessageContent):
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        name: EntryName,
        id: EntryID,
        encryption_revision: int,
        encrypted_on: DateTime,
        key: SecretKey,
    ) -> None: ...
    @property
    def name(self) -> EntryName: ...
    @property
    def id(self) -> EntryID: ...
    @property
    def encryption_revision(self) -> int: ...
    @property
    def encrypted_on(self) -> DateTime: ...
    @property
    def key(self) -> SecretKey: ...

class SharingRevokedMessageContent(MessageContent):
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        id: EntryID,
    ) -> None: ...
    @property
    def id(self) -> EntryID: ...

class PingMessageContent(MessageContent):
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        ping: str,
    ) -> None: ...
    @property
    def ping(self) -> str: ...

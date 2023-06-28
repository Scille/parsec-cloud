# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec_pyi.crypto import PrivateKey, PublicKey, SigningKey, VerifyKey
from parsec._parsec_pyi.ids import DeviceID, UserID
from parsec._parsec_pyi.time import DateTime

class ShamirRecoveryBriefCertificate:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        threshold: int,
        per_recipient_shares: dict[UserID, int],
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def threshold(self) -> int: ...
    @property
    def per_recipient_shares(self) -> dict[UserID, int]: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
    ) -> ShamirRecoveryBriefCertificate: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, raw: bytes) -> ShamirRecoveryBriefCertificate: ...

class ShamirRecoveryShareData:
    def __init__(self, reveal_token_share: bytes, data_key_share: bytes) -> None: ...
    @property
    def reveal_token_share(self) -> bytes: ...
    @property
    def data_key_share(self) -> bytes: ...
    @classmethod
    def decrypt_verify_and_load_for(
        cls,
        ciphered: bytes,
        recipient_privkey: PrivateKey,
        author_verify_key: VerifyKey,
    ) -> ShamirRecoveryShareData: ...
    def dump_sign_and_encrypt_for(
        self,
        author_signkey: SigningKey,
        recipient_pubkey: PublicKey,
    ) -> bytes: ...

class ShamirRecoveryCommunicatedData:
    def __init__(self, reveal_token_share: bytes, data_key_share: bytes) -> None: ...
    @property
    def reveal_token_share(self) -> bytes: ...
    @property
    def data_key_share(self) -> bytes: ...
    def dump(self) -> bytes: ...
    @classmethod
    def load(cls, raw: bytes) -> ShamirRecoveryCommunicatedData: ...

class ShamirRecoveryShareCertificate:
    def __init__(
        self,
        author: DeviceID,
        timestamp: DateTime,
        recipient: UserID,
        ciphered_share: bytes,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def recipient(self) -> UserID: ...
    @property
    def ciphered_share(self) -> bytes: ...
    @classmethod
    def verify_and_load(
        cls,
        signed: bytes,
        author_verify_key: VerifyKey,
        expected_author: DeviceID,
    ) -> ShamirRecoveryShareCertificate: ...
    def dump_and_sign(self, author_signkey: SigningKey) -> bytes: ...
    @classmethod
    def unsecure_load(cls, raw: bytes) -> ShamirRecoveryShareCertificate: ...

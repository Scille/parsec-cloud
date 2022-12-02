from __future__ import annotations

from typing import List, Sequence, Tuple

from parsec._parsec_pyi.certif import DeviceCertificate, RevokedUserCertificate, UserCertificate
from parsec._parsec_pyi.crypto import VerifyKey
from parsec._parsec_pyi.ids import DeviceID, UserID
from parsec._parsec_pyi.protocol import Trustchain
from parsec._parsec_pyi.time import DateTime, TimeProvider

class TrustchainContext:
    def __init__(
        self,
        root_verify_key: VerifyKey,
        time_provider: TimeProvider,
        cache_validity: int,
    ) -> None: ...
    @property
    def cache_validity(self) -> int: ...
    def invalidate_user_cache(self, user_id: UserID) -> None: ...
    def get_user(self, user_id: UserID) -> UserCertificate | None: ...
    def get_revoked_user(self, user_id: UserID) -> RevokedUserCertificate | None: ...
    def get_device(self, device_id: DeviceID) -> DeviceCertificate | None: ...
    def load_user_and_devices(
        self,
        trustchain: Trustchain,
        user_certif: bytes,
        revoked_user_certif: bytes | None = None,
        devices_certifs: Sequence[bytes] = (),
        expected_user_id: UserID | None = None,
    ) -> Tuple[UserCertificate, RevokedUserCertificate | None, List[DeviceCertificate]]: ...
    def load_trustchain(
        self,
        users: Sequence[bytes] = (),
        revoked_users: Sequence[bytes] = (),
        devices: Sequence[bytes] = (),
    ) -> Tuple[List[UserCertificate], List[RevokedUserCertificate], List[DeviceCertificate],]: ...

class TrustchainError:
    @property
    def path(self) -> str: ...
    @property
    def exc(self) -> str: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def device_id(self) -> DeviceID: ...
    @property
    def verified_timestamp(self) -> DateTime: ...
    @property
    def user_timestamp(self) -> DateTime: ...
    @property
    def expected(self) -> UserID: ...
    @property
    def got(self) -> UserID: ...

class TrustchainErrorException(BaseException, TrustchainError): ...

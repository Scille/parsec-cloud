from typing import List, Optional, Sequence, Tuple
from pendulum import DateTime
from parsec.api.data.certif import DeviceCertificateContent, RevokedUserCertificateContent, UserCertificateContent
from parsec.api.protocol.types import DeviceID, UserID
from parsec.crypto import VerifyKey


class TrustchainContext:
    def __init__(self, root_verify_key: VerifyKey, cache_validity: int) -> None: ...

    @property
    def cache_validity(self) -> int: ...

    def invalidate_user_cache(self, user_id: UserID) -> None: ...
    def get_user(self, user_id: UserID, now: DateTime = None) -> Optional[UserCertificateContent]: ...
    def get_revoked_user(self, user_id: UserID, now: DateTime = None) -> Optional[RevokedUserCertificateContent]: ...
    def get_device(self, device_id: DeviceID, now: DateTime = None) -> Optional[DeviceCertificateContent]: ...

    def load_user_and_devices(
        self,
        trustchain: dict,
        user_certif: bytes,
        revoked_user_certif: Optional[bytes] = None,
        devices_certifs: Sequence[bytes] = (),
        expected_user_id: UserID = None,
    ) -> Tuple[
        UserCertificateContent,
        Optional[RevokedUserCertificateContent],
        List[DeviceCertificateContent],
    ]: ...

    def load_trustchain(
        self,
        users: Sequence[bytes] = (),
        revoked_users: Sequence[bytes] = (),
        devices: Sequence[bytes] = (),
        now: DateTime = None,
    ) -> Tuple[
        List[UserCertificateContent],
        List[RevokedUserCertificateContent],
        List[DeviceCertificateContent],
    ]: ...

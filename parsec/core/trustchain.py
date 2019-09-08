from typing import Tuple, List, Optional
from pendulum import Pendulum, now as pendulum_now

from parsec.crypto import VerifyKey
from parsec.api.protocol import UserID
from parsec.api.data import (
    DataError,
    UserCertificateContent,
    RevokedUserCertificateContent,
    DeviceCertificateContent,
)


class TrustchainError(Exception):
    pass


class TrustchainContext:
    def __init__(self, root_verify_key: VerifyKey, cache_validity: int):
        self.root_verify_key = root_verify_key
        self.cache_validity = cache_validity
        self._users_cache = {}
        self._devices_cache = {}
        self._revoked_users_cache = {}

    def get_user(self, user_id: UserID, now: Pendulum = None) -> Optional[UserCertificateContent]:
        now = now or pendulum_now()
        try:
            cached_on, verified_user = self._users_cache[user_id]
            if (now - cached_on).total_seconds() < self.cache_validity:
                return verified_user
        except KeyError:
            pass
        return None

    def get_revoked_user(
        self, user_id: UserID, now: Pendulum = None
    ) -> Optional[RevokedUserCertificateContent]:
        now = now or pendulum_now()
        try:
            cached_on, verified_revoked_user = self._revoked_users_cache[user_id]
            if (now - cached_on).total_seconds() < self.cache_validity:
                return verified_revoked_user
        except KeyError:
            pass
        return None

    def get_device(
        self, device_id: UserID, now: Pendulum = None
    ) -> Optional[DeviceCertificateContent]:
        now = now or pendulum_now()
        try:
            cached_on, verified_device = self._devices_cache[device_id]
            if (now - cached_on).total_seconds() < self.cache_validity:
                return verified_device
        except KeyError:
            pass
        return None

    def load_user_and_devices(
        self,
        trustchain: dict,
        user_certif: bytes,
        revoked_user_certif: Optional[bytes] = None,
        devices_certifs: List[bytes] = (),
        expected_user_id: UserID = None,
    ) -> Tuple[
        UserCertificateContent,
        Optional[RevokedUserCertificateContent],
        List[DeviceCertificateContent],
    ]:
        now = pendulum_now()
        self.load_trustchain(**trustchain, now=now)

        (verified_user,), _, _ = self.load_trustchain(users=(user_certif,), now=now)
        expected_user_id = expected_user_id or verified_user.user_id
        if verified_user.user_id != expected_user_id:
            raise TrustchainError(
                f"Expected certificate from `{expected_user_id}` but got `{verified_user.user_id}`"
            )

        if revoked_user_certif:
            _, (verified_revoked_user,), _ = self.load_trustchain(
                revoked_users=(revoked_user_certif,), now=now
            )
            if verified_revoked_user.user_id != expected_user_id:
                raise TrustchainError(
                    f"Expected certificate from `{expected_user_id}` but got `{verified_revoked_user.user_id}`"
                )
        else:
            verified_revoked_user = None

        _, _, verified_devices = self.load_trustchain(devices=devices_certifs, now=now)
        for verified_device in verified_devices:
            if verified_device.device_id.user_id != expected_user_id:
                raise TrustchainError(
                    f"Expected certificate from `{expected_user_id}` but got `{verified_device.device_id}`"
                )

        return verified_user, verified_revoked_user, verified_devices

    def load_trustchain(
        self,
        users: List[bytes] = (),
        revoked_users: List[bytes] = (),
        devices: List[bytes] = (),
        now: Pendulum = None,
    ):
        now = now or pendulum_now()

        unverified_users = {}
        unverified_devices = {}
        unverified_revoked_users = {}

        try:
            for certif in devices:
                unverified_device = DeviceCertificateContent.unsecure_load(certif)
                unverified_devices[unverified_device.device_id] = (unverified_device, certif)
            for certif in users:
                unverified_user = UserCertificateContent.unsecure_load(certif)
                unverified_users[unverified_user.user_id] = (unverified_user, certif)
            for certif in revoked_users:
                unverified_revoked_user = RevokedUserCertificateContent.unsecure_load(certif)
                unverified_revoked_users[unverified_revoked_user.user_id] = (
                    unverified_revoked_user,
                    certif,
                )

        except DataError as exc:
            raise TrustchainError(f"Invalid certificate: {exc}") from exc

        def _verify_created_by_root(certif, certif_cls, certif_path):
            try:
                return certif_cls.verify_and_load(
                    certif, author_verify_key=self.root_verify_key, expected_author=None
                )

            except DataError as exc:
                raise TrustchainError(
                    f"{certif_path} <-sign- <Root Key>: Invalid certificate: {exc}"
                ) from exc

        def _verify_created_by_device(certif, certif_cls, author_id, certif_path):
            path = f"{certif_path} <-sign- {author_id}"
            author_device = _verify_device(author_id, path)
            try:
                verified = certif_cls.verify_and_load(
                    certif,
                    author_verify_key=author_device.verify_key,
                    expected_author=author_device.device_id,
                )

            except DataError as exc:
                raise TrustchainError(f"{path}: Invalid certificate: {exc}") from exc

            # Also make sure author is admin and wasn't revoked at creation time
            verified_user_author = _verify_user(author_device.device_id.user_id, path)
            if not verified_user_author.is_admin:
                raise TrustchainError(
                    f"{path}:  Invalid signature given {verified_user_author.user_id} is not admin"
                )
            verified_revoked_user_author = _verify_revoked_user(
                author_device.device_id.user_id, path
            )
            if (
                verified_revoked_user_author
                and verified.timestamp > verified_revoked_user_author.timestamp
            ):
                raise TrustchainError(
                    f"{path}: Signature ({verified.timestamp}) is posterior "
                    f"to user revocation {verified_revoked_user_author.timestamp})"
                )

            return verified

        def _verify_device(device_id, path):
            # Check if certificate is already verified in cache
            verified_device = self.get_device(device_id, now)
            if verified_device:
                return verified_device

            try:
                unverified_device, certif = unverified_devices[device_id]
            except KeyError:
                raise TrustchainError(f"{path}: Missing device certificate for {device_id}")

            if unverified_device.author is None:
                verified_device = _verify_created_by_root(certif, DeviceCertificateContent, path)
            else:
                verified_device = _verify_created_by_device(
                    certif, DeviceCertificateContent, unverified_device.author, path
                )

            # Populate the cache and return
            self._devices_cache[verified_device.device_id] = (now, verified_device)
            return verified_device

        def _verify_user(user_id, path):
            # Check if certificate is already verified in cache
            verified_user = self.get_user(user_id, now)
            if verified_user:
                return verified_user

            try:
                unverified_user, certif = unverified_users[user_id]
            except KeyError:
                raise TrustchainError(f"{path}: Missing user certificate for {user_id}")

            if unverified_user.author is None:
                verified_user = _verify_created_by_root(certif, UserCertificateContent, path)
            else:
                verified_user = _verify_created_by_device(
                    certif, UserCertificateContent, unverified_user.author, path
                )

            # Populate the cache and return
            self._users_cache[verified_user.user_id] = (now, verified_user)
            return verified_user

        def _verify_revoked_user(user_id, path):
            # Check if certificate is already verified in cache
            verified_revoked_user = self.get_revoked_user(user_id, now)
            if verified_revoked_user:
                return verified_revoked_user

            try:
                unverified_revoked_user, certif = unverified_revoked_users[user_id]
            except KeyError:
                return None

            if unverified_revoked_user.author is None:
                verified_revoked_user = _verify_created_by_root(
                    certif, RevokedUserCertificateContent, path
                )
            else:
                verified_revoked_user = _verify_created_by_device(
                    certif, RevokedUserCertificateContent, unverified_revoked_user.author, path
                )

            # Populate the cache and return
            self._revoked_users_cache[verified_revoked_user.user_id] = (now, verified_revoked_user)
            return verified_revoked_user

        unverified_devices = [
            _verify_device(unverified_device.device_id, f"{unverified_device.device_id}")
            for unverified_device, _ in unverified_devices.values()
        ]
        unverified_users = [
            _verify_user(unverified_user.user_id, f"{unverified_user.user_id}")
            for unverified_user, _ in unverified_users.values()
        ]
        unverified_revoked_users = [
            _verify_revoked_user(
                unverified_revoked_user.user_id, f"{unverified_revoked_user.user_id}"
            )
            for unverified_revoked_user, _ in unverified_revoked_users.values()
        ]
        return unverified_users, unverified_revoked_users, unverified_devices

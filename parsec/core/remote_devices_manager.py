# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import TYPE_CHECKING, List, Tuple

from parsec._parsec import AuthenticatedCmds as RsBackendAuthenticatedCmds
from parsec._parsec import (
    TimeProvider,
    TrustchainContext,
    TrustchainErrorException,
    UserGetRepNotFound,
    UserGetRepOk,
    VerifyKey,
)
from parsec.api.data import DeviceCertificate, RevokedUserCertificate, UserCertificate
from parsec.api.protocol import DeviceID, UserID
from parsec.core.backend_connection import BackendConnectionError, BackendNotAvailable

if TYPE_CHECKING:
    from parsec.core.backend_connection import BackendAuthenticatedCmds

DEFAULT_CACHE_VALIDITY = 60 * 60  # 3600 seconds, 1 hour


class RemoteDevicesManagerError(Exception):
    pass


class RemoteDevicesManagerBackendOfflineError(RemoteDevicesManagerError):
    pass


class RemoteDevicesManagerValidationError(RemoteDevicesManagerError):
    pass


class RemoteDevicesManagerPackingError(RemoteDevicesManagerError):
    pass


class RemoteDevicesManagerNotFoundError(RemoteDevicesManagerError):
    pass


class RemoteDevicesManagerUserNotFoundError(RemoteDevicesManagerNotFoundError):
    pass


class RemoteDevicesManagerDeviceNotFoundError(RemoteDevicesManagerNotFoundError):
    pass


class RemoteDevicesManagerInvalidTrustchainError(RemoteDevicesManagerError):
    pass


class RemoteDevicesManager:
    """
    Fetch users&devices from backend, verify their trustchain and keep
    a cache of them for a limited duration.
    """

    def __init__(
        self,
        backend_cmds: BackendAuthenticatedCmds | RsBackendAuthenticatedCmds,
        root_verify_key: VerifyKey,
        time_provider: TimeProvider,
        cache_validity: int = DEFAULT_CACHE_VALIDITY,
    ):
        self._backend_cmds = backend_cmds
        self._trustchain_ctx = TrustchainContext(root_verify_key, time_provider, cache_validity)

    @property
    def cache_validity(self) -> int:
        return self._trustchain_ctx.cache_validity

    def invalidate_user_cache(self, user_id: UserID) -> None:
        self._trustchain_ctx.invalidate_user_cache(user_id)

    async def get_user(
        self, user_id: UserID, no_cache: bool = False
    ) -> Tuple[UserCertificate, RevokedUserCertificate | None]:
        """
        Raises:
            RemoteDevicesManagerError
            RemoteDevicesManagerBackendOfflineError
            RemoteDevicesManagerUserNotFoundError
            RemoteDevicesManagerInvalidTrustchainError
        """
        try:
            verified_user = None if no_cache else self._trustchain_ctx.get_user(user_id)
            verified_revoked_user = (
                None if no_cache else self._trustchain_ctx.get_revoked_user(user_id)
            )
        except TrustchainErrorException as exc:
            raise RemoteDevicesManagerInvalidTrustchainError(exc) from exc
        if not verified_user:
            verified_user, verified_revoked_user, _ = await self.get_user_and_devices(
                user_id, no_cache=True
            )
        return verified_user, verified_revoked_user

    async def get_device(self, device_id: DeviceID, no_cache: bool = False) -> DeviceCertificate:
        """
        Raises:
            RemoteDevicesManagerError
            RemoteDevicesManagerBackendOfflineError
            RemoteDevicesManagerUserNotFoundError
            RemoteDevicesManagerDeviceNotFoundError
            RemoteDevicesManagerInvalidTrustchainError
        """
        try:
            verified_device = None if no_cache else self._trustchain_ctx.get_device(device_id)
        except TrustchainErrorException as exc:
            raise RemoteDevicesManagerInvalidTrustchainError(exc) from exc
        if not verified_device:
            _, _, verified_devices = await self.get_user_and_devices(
                device_id.user_id, no_cache=True
            )
            try:
                verified_device = next(vd for vd in verified_devices if vd.device_id == device_id)

            except StopIteration:
                raise RemoteDevicesManagerDeviceNotFoundError(
                    f"User `{device_id.user_id.str}` doesn't have a device `{device_id.str}`"
                )
        return verified_device

    async def get_user_and_devices(
        self, user_id: UserID, no_cache: bool = False
    ) -> Tuple[UserCertificate, RevokedUserCertificate | None, List[DeviceCertificate]]:
        """
        Note: unlike `get_user` and `get_device`, this method don't rely on cache
        considering only part of the devices to retrieve could be in cache.
        Raises:
            RemoteDevicesManagerError
            RemoteDevicesManagerBackendOfflineError
            RemoteDevicesManagerUserNotFoundError
            RemoteDevicesManagerInvalidTrustchainError
        """
        try:
            rep = await self._backend_cmds.user_get(user_id)
        except BackendNotAvailable as exc:
            raise RemoteDevicesManagerBackendOfflineError(
                f"User `{user_id.str}` is not in local cache and we are offline."
            ) from exc
        except BackendConnectionError as exc:
            raise RemoteDevicesManagerError(
                f"Failed to fetch user `{user_id.str}` from the backend: {exc}"
            ) from exc

        if isinstance(rep, UserGetRepNotFound):
            raise RemoteDevicesManagerUserNotFoundError(
                f"User `{user_id.str}` doesn't exist in backend"
            )
        elif not isinstance(rep, UserGetRepOk):
            raise RemoteDevicesManagerError(f"Cannot fetch user {user_id}: {rep}")

        try:
            return self._trustchain_ctx.load_user_and_devices(
                trustchain=rep.trustchain,
                user_certif=rep.user_certificate,
                revoked_user_certif=rep.revoked_user_certificate,
                devices_certifs=rep.device_certificates,
                expected_user_id=user_id,
            )
        except TrustchainErrorException as exc:
            raise RemoteDevicesManagerInvalidTrustchainError(exc) from exc

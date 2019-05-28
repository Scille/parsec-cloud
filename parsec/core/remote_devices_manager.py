# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Tuple
import pendulum

from parsec.types import DeviceID, UserID
from parsec.crypto import (
    CryptoError,
    VerifyKey,
    verify_user_certificate,
    verify_device_certificate,
    verify_revoked_device_certificate,
    unsecure_read_user_certificate,
    unsecure_read_device_certificate,
    unsecure_read_revoked_device_certificate,
)
from parsec.core.backend_connection import (
    BackendCmdsPool,
    BackendAnonymousCmds,
    BackendConnectionError,
    BackendNotAvailable,
    BackendCmdsNotFound,
)
from parsec.core.types import (
    UnverifiedRemoteDevice,
    UnverifiedRemoteUser,
    VerifiedRemoteDevice,
    VerifiedRemoteUser,
)


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


class RemoteDevicesManagerInvalidTrustchainError(RemoteDevicesManagerError):
    pass


DEFAULT_CACHE_VALIDITY = 60 * 60  # 1h


def _verify_devices(root_verify_key, *uv_devices):
    """
    Raises:
        RemoteDevicesManagerInvalidTrustchainError
    """
    verified_devices = {}

    # First convert to VerifiedRemoteDevice to easily access metadata
    # (obviously those VerifiedRemoteDevice are not verified at all so far !)
    all_devices = {}
    for uv_device in uv_devices:
        try:
            d_certif = unsecure_read_device_certificate(uv_device.device_certificate)

        except CryptoError as exc:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"Invalid format for device certificate: {exc}"
            ) from exc

        params = {
            "fetched_on": uv_device.fetched_on,
            "device_id": d_certif.device_id,
            "verify_key": d_certif.verify_key,
            "device_certificate": uv_device.device_certificate,
            "certified_by": d_certif.certified_by,
            "certified_on": d_certif.certified_on,
            "revoked_device_certificate": uv_device.revoked_device_certificate,
        }
        if uv_device.revoked_device_certificate:
            try:
                r_certif = unsecure_read_revoked_device_certificate(
                    uv_device.revoked_device_certificate
                )

            except CryptoError as exc:
                raise RemoteDevicesManagerInvalidTrustchainError(
                    f"Invalid format for revoked device certificate: {exc}"
                ) from exc

            if r_certif.device_id != d_certif.device_id:
                raise RemoteDevicesManagerInvalidTrustchainError(
                    f"Mismatch device_id in creation (`{d_certif.device_id}`)"
                    f" and revocation (`{r_certif.device_id}`) certificates"
                )

            params["revoked_by"] = r_certif.certified_by
            params["revoked_on"] = r_certif.certified_on

        d_certif = VerifiedRemoteDevice(**params)
        all_devices[d_certif.device_id] = (d_certif, uv_device)

    def _recursive_verify_device(device_id, path):
        try:
            return verified_devices[device_id]
        except KeyError:
            pass

        try:
            d_certif, uv_device = all_devices[device_id]

        except KeyError:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"{path}: Device not provided by backend"
            )

        # Verify user certif
        if d_certif.certified_by is None:
            # Certified by root
            certifier_verify_key = root_verify_key
            certifier_revoked_on = None
            sub_path = f"{path} <-create- <Root Key>"

        else:
            sub_path = f"{path} <-create- `{d_certif.certified_by}`"
            certifier = _recursive_verify_device(d_certif.certified_by, sub_path)
            certifier_verify_key = certifier.verify_key
            certifier_revoked_on = certifier.revoked_on

        try:
            verify_device_certificate(
                uv_device.device_certificate, d_certif.certified_by, certifier_verify_key
            )

        except CryptoError as exc:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"{sub_path}: invalid certificate: {exc}"
            ) from exc

        if certifier_revoked_on and d_certif.certified_on > certifier_revoked_on:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"{sub_path}: Signature ({d_certif.certified_on}) is "
                f"posterior to device revocation {certifier_revoked_on})"
            )

        # Verify user revoke certif if any
        if d_certif.revoked_by:
            sub_path = f"{path} <-revoke- `{d_certif.revoked_by}`"
            revoker = _recursive_verify_device(d_certif.revoked_by, sub_path)
            try:
                verify_revoked_device_certificate(
                    uv_device.revoked_device_certificate, d_certif.revoked_by, revoker.verify_key
                )

            except CryptoError as exc:
                raise RemoteDevicesManagerInvalidTrustchainError(
                    f"{sub_path}: invalid certificate: {exc}"
                ) from exc

            if revoker.revoked_on and d_certif.revoked_on > revoker.revoked_on:
                raise RemoteDevicesManagerInvalidTrustchainError(
                    f"{sub_path}: Signature ({d_certif.revoked_on}) is "
                    f"posterior to device revocation {revoker.revoked_on})"
                )

        # All set ! This device is valid ;-)
        verified_devices[d_certif.device_id] = d_certif
        return d_certif

    for d_certif, _ in all_devices.values():
        # verify each device here
        _recursive_verify_device(d_certif.device_id, f"`{d_certif.device_id}`")

    return verified_devices


def _verify_user(root_verify_key, uv_user, verified_devices):
    """
    Raises:
        RemoteDevicesManagerInvalidTrustchainError
    """
    try:
        u_certif = unsecure_read_user_certificate(uv_user.user_certificate)

    except CryptoError as exc:
        raise RemoteDevicesManagerInvalidTrustchainError(
            f"Invalid format for user certificate: {exc}"
        ) from exc

    # `_load_devices` must be called before `_load_user` for this to work
    if u_certif.certified_by is None:
        # Certified by root
        certifier_verify_key = root_verify_key
        certifier_revoked_on = None
        sub_path = f"`{u_certif.user_id}` <-create- <Root Key>"

    else:
        sub_path = f"`{u_certif.user_id}` <-create- `{u_certif.certified_by}`"
        try:
            certifier = verified_devices[u_certif.certified_by]

        except KeyError:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"{sub_path}: Device not provided by backend"
            )
        certifier_verify_key = certifier.verify_key
        certifier_revoked_on = certifier.revoked_on

    try:
        verify_user_certificate(
            uv_user.user_certificate, u_certif.certified_by, certifier_verify_key
        )

    except CryptoError as exc:
        raise RemoteDevicesManagerInvalidTrustchainError(
            f"{sub_path}: invalid certificate: {exc}"
        ) from exc

    if certifier_revoked_on and u_certif.certified_on > certifier_revoked_on:
        raise RemoteDevicesManagerInvalidTrustchainError(
            f"{sub_path}: Signature ({u_certif.certified_on}) is posterior "
            f"to device revocation {certifier_revoked_on})"
        )

    return VerifiedRemoteUser(
        fetched_on=uv_user.fetched_on,
        user_id=u_certif.user_id,
        public_key=u_certif.public_key,
        user_certificate=uv_user.user_certificate,
        certified_by=u_certif.certified_by,
        certified_on=u_certif.certified_on,
        is_admin=u_certif.is_admin,
    )


class RemoteDevicesManager:
    """
    Fetch users&devices from backend, verify their trustchain and keep
    a cache of them for a limited duration.
    """

    def __init__(
        self,
        backend_cmds: BackendCmdsPool,
        root_verify_key: VerifyKey,
        cache_validity: int = DEFAULT_CACHE_VALIDITY,
    ):
        self._backend_cmds = backend_cmds
        self._devices = {}
        self._users = {}
        self.root_verify_key = root_verify_key
        self.cache_validity = cache_validity

    async def get_user(self, user_id: UserID) -> VerifiedRemoteUser:
        """
        Raises:
            RemoteDevicesManagerError
            RemoteDevicesManagerBackendOfflineError
            RemoteDevicesManagerNotFoundError
            RemoteDevicesManagerInvalidTrustchainError
        """
        try:
            verified_user = self._users[user_id]
            now = pendulum.now()
            if (now - verified_user.fetched_on).total_seconds() < self.cache_validity:
                return verified_user

        except KeyError:
            pass

        verified_user, _ = await self.get_user_and_devices(user_id)
        return verified_user

    async def get_user_and_devices(
        self, user_id: UserID
    ) -> Tuple[VerifiedRemoteUser, Tuple[VerifiedRemoteDevice]]:
        """
        Note: unlike `get_user` and `get_device`, this method don't rely on cache
        considering only part of the devices to retreive could be in cache.
        Raises:
            RemoteDevicesManagerError
            RemoteDevicesManagerBackendOfflineError
            RemoteDevicesManagerNotFoundError
            RemoteDevicesManagerInvalidTrustchainError
        """
        try:
            uv_user, uv_devices, trustchain = await self._backend_cmds.user_get(user_id)

        except BackendNotAvailable as exc:
            raise RemoteDevicesManagerBackendOfflineError(
                f"User `{user_id}` is not in local cache and we are offline."
            ) from exc

        except BackendCmdsNotFound as exc:
            raise RemoteDevicesManagerNotFoundError(
                f"User `{user_id}` doesn't exist in backend"
            ) from exc

        except BackendConnectionError as exc:
            raise RemoteDevicesManagerError(
                f"Failed to fetch user `{user_id}` from the backend: {exc}"
            ) from exc

        all_verified_devices = self._load_devices(*uv_devices, *trustchain)
        verified_devices = [vd for vd in all_verified_devices if vd.device_id.user_id == user_id]
        verified_user = self._load_user(uv_user)
        if verified_user.user_id != user_id:
            raise RemoteDevicesManagerError(
                f"Backend returned user `{verified_user.user_id}` while we "
                f"were asking for `{user_id}`"
            )
        return verified_user, verified_devices

    async def get_device(self, device_id: DeviceID) -> VerifiedRemoteDevice:
        """
        Raises:
            RemoteDevicesManagerError
            RemoteDevicesManagerBackendOfflineError
            RemoteDevicesManagerNotFoundError
            RemoteDevicesManagerInvalidTrustchainError
        """
        try:
            verified_device = self._devices[device_id]
            now = pendulum.now()
            if (now - verified_device.fetched_on).total_seconds() < self.cache_validity:
                return verified_device

        except KeyError:
            pass

        try:
            uv_user, uv_devices, trustchain = await self._backend_cmds.user_get(device_id.user_id)

        except BackendNotAvailable as exc:
            raise RemoteDevicesManagerBackendOfflineError(
                f"Device `{device_id}` is not in local cache and we are offline."
            ) from exc

        except BackendCmdsNotFound as exc:
            raise RemoteDevicesManagerNotFoundError(
                f"User `{device_id.user_id}` doesn't exist in backend"
            ) from exc

        except BackendConnectionError as exc:
            raise RemoteDevicesManagerError(
                f"Failed to fetch user `{device_id.user_id}` from the backend: {exc}"
            ) from exc

        verified_devices = self._load_devices(*uv_devices, *trustchain)
        try:
            verified_device = next(vd for vd in verified_devices if vd.device_id == device_id)

        except StopIteration:
            raise RemoteDevicesManagerNotFoundError(
                f"User `{device_id.user_id}` doesn't have a device `{device_id}`"
            )

        # Also update the user cache given it was provided anyway
        self._load_user(uv_user)
        return verified_device

    def _load_devices(
        self, *uv_devices: Tuple[UnverifiedRemoteDevice]
    ) -> Tuple[VerifiedRemoteDevice]:
        """
        Raises:
            RemoteDevicesManagerInvalidTrustchainError
        """
        verified_devices = _verify_devices(self.root_verify_key, *uv_devices)
        self._devices.update(verified_devices)
        return tuple(verified_devices.values())

    def _load_user(self, uv_user: UnverifiedRemoteUser) -> VerifiedRemoteUser:
        """
        Raises:
            RemoteDevicesManagerInvalidTrustchainError
        """
        verified_user = _verify_user(self.root_verify_key, uv_user, self._devices)
        self._users[verified_user.user_id] = verified_user
        return verified_user


async def get_device_invitation_creator(
    backend_cmds: BackendAnonymousCmds, root_verify_key: VerifyKey, new_device_id: DeviceID
) -> Tuple[VerifiedRemoteDevice, VerifiedRemoteUser]:
    """
    Raises:
        RemoteDevicesManagerError
        RemoteDevicesManagerBackendOfflineError
        RemoteDevicesManagerNotFoundError
        RemoteDevicesManagerInvalidTrustchainError
    """
    try:
        uv_device, uv_user, trustchain = await backend_cmds.device_get_invitation_creator(
            new_device_id
        )

    except BackendNotAvailable as exc:
        raise RemoteDevicesManagerBackendOfflineError(*exc.args) from exc

    except BackendCmdsNotFound as exc:
        raise RemoteDevicesManagerNotFoundError(
            f"User `{new_device_id}` doesn't exist in backend"
        ) from exc

    except BackendConnectionError as exc:
        raise RemoteDevicesManagerError(
            "Failed to fetch invitation creator for device "
            f"`{new_device_id}` from the backend: {exc}"
        ) from exc

    verified_devices = _verify_devices(root_verify_key, uv_device, *trustchain)
    return (
        verified_devices[unsecure_read_device_certificate(uv_device.device_certificate).device_id],
        _verify_user(root_verify_key, uv_user, verified_devices),
    )


async def get_user_invitation_creator(
    backend_cmds: BackendAnonymousCmds, root_verify_key: VerifyKey, new_user_id: DeviceID
) -> Tuple[VerifiedRemoteDevice, VerifiedRemoteUser]:
    """
    Raises:
        RemoteDevicesManagerError
        RemoteDevicesManagerBackendOfflineError
        RemoteDevicesManagerNotFoundError
        RemoteDevicesManagerInvalidTrustchainError
    """
    try:
        uv_device, uv_user, trustchain = await backend_cmds.user_get_invitation_creator(new_user_id)

    except BackendNotAvailable as exc:
        raise RemoteDevicesManagerBackendOfflineError(*exc.args) from exc

    except BackendCmdsNotFound as exc:
        raise RemoteDevicesManagerNotFoundError(
            f"User `{new_user_id}` doesn't exist in backend"
        ) from exc

    except BackendConnectionError as exc:
        raise RemoteDevicesManagerError(
            "Failed to fetch invitation creator for user "
            f"`{new_user_id}` from the backend: {exc}"
        ) from exc

    verified_devices = _verify_devices(root_verify_key, uv_device, *trustchain)
    return (
        verified_devices[unsecure_read_device_certificate(uv_device.device_certificate).device_id],
        _verify_user(root_verify_key, uv_user, verified_devices),
    )

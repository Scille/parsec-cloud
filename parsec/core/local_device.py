# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import attr
import trio
from enum import Enum
from pathlib import Path, PurePath
from hashlib import sha256
from typing import Callable, List, Optional, Iterator, Dict, Type, Tuple
from importlib import import_module

from parsec.serde import BaseSchema, fields, MsgpackSerializer, OneOfSchema
from parsec.crypto import (
    SecretKey,
    SigningKey,
    PrivateKey,
    CryptoError,
    derivate_secret_key_from_password,
    generate_recovery_passphrase,
    derivate_secret_key_from_recovery_passphrase,
)
from parsec.api.protocol import (
    OrganizationID,
    DeviceID,
    HumanHandle,
    HumanHandleField,
    OrganizationIDField,
    DeviceIDField,
    DeviceLabel,
    DeviceLabelField,
)
from parsec.api.data import DataError, UserProfile
from parsec.core.types import EntryID, LocalDevice, BackendOrganizationAddr


# .keys files are not supposed to leave the parsec configuration folder,
# so it's ok to have such a common suffix
DEVICE_FILE_SUFFIX = ".keys"
# .psrk stands for ps(parsec)r(recovery)k(key)
RECOVERY_DEVICE_FILE_SUFFIX = ".psrk"


class LocalDeviceError(Exception):
    pass


class LocalDeviceCryptoError(LocalDeviceError):
    pass


class LocalDeviceNotFoundError(LocalDeviceError):
    pass


class LocalDeviceAlreadyExistsError(LocalDeviceError):
    pass


class LocalDeviceValidationError(LocalDeviceError):
    pass


class LocalDevicePackingError(LocalDeviceError):
    pass


class DeviceFileType(Enum):
    PASSWORD = "password"
    SMARTCARD = "smartcard"
    RECOVERY = "recovery"


class LegacyDeviceFileSchema(BaseSchema):
    """Schema for legacy device files where the filename contains complementary information."""

    type = fields.EnumCheckedConstant(DeviceFileType.PASSWORD, required=True)
    salt = fields.Bytes(required=True)
    ciphertext = fields.Bytes(required=True)

    # Since human_handle/device_label has been introduced, device_id is
    # redacted (i.e. user_id and device_name are 2 random uuids), hence
    # those fields have been added to the device file so the login page in
    # the GUI can use them to provide useful information.
    human_handle = HumanHandleField(allow_none=True, missing=None)
    device_label = DeviceLabelField(allow_none=True, missing=None)


class BaseDeviceFileSchema(BaseSchema):
    """Schema for device files that does not rely on the filename for complementary information."""

    ciphertext = fields.Bytes(required=True)

    # Override those fields to make them required (although `None` is still valid)
    human_handle = HumanHandleField(required=True, allow_none=True)
    device_label = DeviceLabelField(required=True, allow_none=True)

    # Store device ID, organization ID and slug in the device file
    # For legacy versions, this information is available in the file name
    device_id = DeviceIDField(required=True)
    organization_id = OrganizationIDField(required=True)
    slug = fields.String(required=True)


class PasswordDeviceFileSchema(BaseDeviceFileSchema):
    type = fields.EnumCheckedConstant(DeviceFileType.PASSWORD, required=True)
    salt = fields.Bytes(required=True)


class RecoveryDeviceFileSchema(BaseDeviceFileSchema):
    type = fields.EnumCheckedConstant(DeviceFileType.RECOVERY, required=True)


class SmartcardDeviceFileSchema(BaseDeviceFileSchema):
    type = fields.EnumCheckedConstant(DeviceFileType.SMARTCARD, required=True)
    encrypted_key = fields.Bytes(required=True)
    certificate_id = fields.String(required=True)
    certificate_sha1 = fields.Bytes(allow_none=True, missing=None)


class DeviceFileSchema(OneOfSchema):
    type_field = "type"

    @property
    def type_schemas(  # type: ignore[override]
        self
    ) -> Dict[DeviceFileType, Type[OneOfSchema]]:
        return {
            DeviceFileType.PASSWORD: PasswordDeviceFileSchema,
            DeviceFileType.RECOVERY: RecoveryDeviceFileSchema,
            DeviceFileType.SMARTCARD: SmartcardDeviceFileSchema,
        }

    def get_obj_type(self, obj: Dict[str, object]) -> object:
        return obj["type"]


legacy_key_file_serializer = MsgpackSerializer(
    LegacyDeviceFileSchema,
    validation_exc=LocalDeviceValidationError,
    packing_exc=LocalDevicePackingError,
)

key_file_serializer = MsgpackSerializer(
    DeviceFileSchema, validation_exc=LocalDeviceValidationError, packing_exc=LocalDevicePackingError
)


def generate_new_device(
    organization_addr: BackendOrganizationAddr,
    device_id: Optional[DeviceID] = None,
    profile: UserProfile = UserProfile.STANDARD,
    human_handle: Optional[HumanHandle] = None,
    device_label: Optional[DeviceLabel] = None,
    signing_key: Optional[SigningKey] = None,
    private_key: Optional[PrivateKey] = None,
) -> LocalDevice:
    return LocalDevice(
        organization_addr=organization_addr,
        device_id=device_id or DeviceID.new(),
        device_label=device_label,
        human_handle=human_handle,
        signing_key=signing_key or SigningKey.generate(),
        private_key=private_key or PrivateKey.generate(),
        profile=profile,
        user_manifest_id=EntryID.new(),
        user_manifest_key=SecretKey.generate(),
        local_symkey=SecretKey.generate(),
    )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class AvailableDevice:
    key_file_path: Path
    organization_id: OrganizationID
    device_id: DeviceID
    human_handle: Optional[HumanHandle]
    device_label: Optional[DeviceLabel]
    slug: str
    type: DeviceFileType

    @property
    def user_display(self) -> str:
        return str(self.human_handle or self.device_id.user_id)

    @property
    def short_user_display(self) -> str:
        return str(self.human_handle.label if self.human_handle else self.device_id.user_id)

    @property
    def device_display(self) -> str:
        return str(self.device_label if self.device_label else self.device_id.device_name)

    @property
    def slughash(self) -> str:
        return sha256(self.slug.encode()).hexdigest()


def get_available_device(config_dir: Path, device: LocalDevice) -> AvailableDevice:
    for available_device in _iter_available_devices(config_dir):
        if available_device.slug == device.slug:
            return available_device
    raise FileNotFoundError


def get_key_file(config_dir: Path, device: LocalDevice) -> Path:
    return get_available_device(config_dir, device).key_file_path


def get_default_key_file(config_dir: Path, device: LocalDevice) -> Path:
    """Return the default keyfile path for a given device.

    Note that the filename does not carry any intrinsic meaning.
    Here, we simply use the slughash to avoid name collision.
    """
    return get_devices_dir(config_dir) / f"{device.slughash}{DEVICE_FILE_SUFFIX}"


def get_recovery_device_file_name(recovery_device: LocalDevice) -> str:
    return f"parsec-recovery-{recovery_device.organization_id}-{recovery_device.short_user_display}{RECOVERY_DEVICE_FILE_SUFFIX}"


def get_devices_dir(config_dir: Path) -> Path:
    return config_dir / "devices"


def _load_legacy_device_file(key_file_path: Path) -> Optional[AvailableDevice]:
    # For the legacy device files, the slug is contained in the device filename
    slug = key_file_path.stem

    try:
        organization_id, device_id = LocalDevice.load_slug(slug)
    except ValueError:
        # Not a valid slug, ignore this file
        return None

    try:
        data = legacy_key_file_serializer.loads(key_file_path.read_bytes())

    except (FileNotFoundError, LocalDeviceError):
        # Not a valid device file, ignore this file
        return None

    return AvailableDevice(
        key_file_path=key_file_path,
        organization_id=organization_id,
        device_id=device_id,
        human_handle=data["human_handle"],
        device_label=data["device_label"],
        slug=slug,
        type=data["type"],
    )


def load_device_file(key_file_path: Path) -> Optional[AvailableDevice]:
    try:
        data = key_file_serializer.loads(key_file_path.read_bytes())

    except (FileNotFoundError, LocalDeviceError):
        # Not a valid device file, ignore this
        return None

    return AvailableDevice(
        key_file_path=key_file_path,
        organization_id=data["organization_id"],
        device_id=data["device_id"],
        human_handle=data["human_handle"],
        device_label=data["device_label"],
        slug=data["slug"],
        type=data["type"],
    )


def _iter_available_devices(config_dir: Path) -> Iterator[AvailableDevice]:
    # Set of seen slugs
    seen = set()
    # Consider `.keys` files in devices directory and subdirectories
    key_file_paths = list(get_devices_dir(config_dir).rglob(f"*{DEVICE_FILE_SUFFIX}"))
    # Sort paths so the discovery order is deterministic
    # In the case of duplicate files, that means only the first discovered device is considered
    for key_file_path in sorted(key_file_paths):
        # Load the device file
        device = load_device_file(key_file_path)
        # Try with the legacy deserializer if necessary
        if device is None:
            device = _load_legacy_device_file(key_file_path)
        # Ignore invalid files
        if device is None:
            continue
        # Ignore duplicate files
        if device.slug in seen:
            continue
        # Yield
        seen.add(device.slug)
        yield device


def list_available_devices(config_dir: Path) -> List[AvailableDevice]:
    return list(_iter_available_devices(config_dir))


def load_device_with_password(key_file: Path, password: str) -> LocalDevice:
    """
    Raises:
        LocalDeviceError
        LocalDeviceNotFoundError
        LocalDeviceCryptoError
        LocalDeviceValidationError
        LocalDevicePackingError
    """

    def _decrypt_ciphertext(data: dict) -> bytes:
        if data["type"] != DeviceFileType.PASSWORD:
            raise LocalDeviceValidationError("Not a password-protected device file")

        try:
            key, _ = derivate_secret_key_from_password(password, data["salt"])
            return key.decrypt(data["ciphertext"])
        except CryptoError as exc:
            raise LocalDeviceCryptoError(str(exc)) from exc

    return _load_device(key_file, _decrypt_ciphertext)


def _load_device(key_file: Path, decrypt_ciphertext: Callable[[dict], bytes]) -> LocalDevice:
    try:
        ciphertext = key_file.read_bytes()
    except OSError as exc:
        raise LocalDeviceNotFoundError(f"Config file `{key_file}` is missing") from exc

    try:
        data = key_file_serializer.loads(ciphertext)
    except LocalDeviceError:
        data = legacy_key_file_serializer.loads(ciphertext)

    plaintext = decrypt_ciphertext(data)

    try:
        return LocalDevice.load(plaintext)

    except DataError as exc:
        raise LocalDeviceValidationError(f"Cannot load local device: {exc}") from exc


def save_device_with_password_in_config(
    config_dir: Path, device: LocalDevice, password: str
) -> Path:
    """
    Raises:
        LocalDeviceError
        LocalDeviceValidationError
        LocalDevicePackingError
    """
    key_file = get_default_key_file(config_dir, device)
    # Why do we use `force=True` here ?
    # Key file name is per-device unique (given it contains the device slughash),
    # hence there is no risk to overwrite another device.
    # So if we are overwritting a key file it could be by:
    # - the same device object, hence overwritting has no effect
    # - a device object with same slughash but different device/user keys
    #   This would mean the device enrollment has been replayed (which is
    #   not possible in theory, but could occur in case of a rollback in the
    #   Parsec server), in this case the old device object is now invalid
    #   and it's a good thing to replace it.
    save_device_with_password(key_file, device, password, force=True)
    return key_file


def save_device_with_password(
    key_file: Path, device: LocalDevice, password: str, force: bool
) -> None:
    """
    Raises:
        LocalDeviceError
        LocalDeviceValidationError
        LocalDevicePackingError
    """

    def _encrypt_dump(cleartext: bytes) -> Tuple[DeviceFileType, bytes, dict]:
        try:
            key, salt = derivate_secret_key_from_password(password)
            ciphertext = key.encrypt(cleartext)

        except CryptoError as exc:
            raise LocalDeviceValidationError(f"Cannot dump local device: {exc}") from exc

        extra_args = {"salt": salt}
        return DeviceFileType.PASSWORD, ciphertext, extra_args

    _save_device(key_file, device, force, _encrypt_dump)


def _save_device(
    key_file: Path,
    device: LocalDevice,
    force: bool,
    encrypt_dump: Callable[[bytes], Tuple[DeviceFileType, bytes, dict]],
) -> None:
    assert key_file.suffix == DEVICE_FILE_SUFFIX

    if key_file.exists() and not force:
        raise LocalDeviceAlreadyExistsError(f"Device key file `{key_file}` already exists")

    cleartext = device.dump()
    type, ciphertext, extra_args = encrypt_dump(cleartext)
    key_file_content = key_file_serializer.dumps(
        {
            "type": type,
            **extra_args,
            "ciphertext": ciphertext,
            "human_handle": device.human_handle,
            "device_label": device.device_label,
            "organization_id": device.organization_id,
            "device_id": device.device_id,
            "slug": device.slug,
        }
    )

    try:
        key_file.parent.mkdir(mode=0o700, exist_ok=True, parents=True)
        key_file.write_bytes(key_file_content)

    except OSError as exc:
        raise LocalDeviceError(f"Cannot save {key_file}: {exc}") from exc


def change_device_password(key_file: Path, old_password: str, new_password: str) -> None:
    """
        LocalDeviceError
        LocalDeviceNotFoundError
        LocalDeviceCryptoError
        LocalDeviceValidationError
        LocalDevicePackingError
    """
    device = load_device_with_password(key_file, password=old_password)
    save_device_with_password(key_file, device, password=new_password, force=True)


async def load_recovery_device(key_file: PurePath, passphrase: str) -> LocalDevice:
    """
    Raises:
        LocalDeviceError
        LocalDeviceNotFoundError
        LocalDeviceCryptoError
        LocalDeviceValidationError
        LocalDevicePackingError
    """
    key_file = trio.Path(key_file)
    try:
        ciphertext = await key_file.read_bytes()
    except OSError as exc:
        raise LocalDeviceNotFoundError(f"Recovery file `{key_file}` is missing") from exc

    try:
        data = key_file_serializer.loads(ciphertext)
    except LocalDevicePackingError as exc:
        raise LocalDeviceValidationError("Not a device recovery file") from exc

    if data["type"] != DeviceFileType.RECOVERY:
        raise LocalDeviceValidationError("Not a device recovery file")

    try:
        key = derivate_secret_key_from_recovery_passphrase(passphrase)
    except ValueError as exc:
        # Not really a crypto operation, but it is more coherent for the caller
        raise LocalDeviceCryptoError("Invalid passphrase") from exc

    try:
        plaintext = key.decrypt(data["ciphertext"])
    except CryptoError as exc:
        raise LocalDeviceCryptoError(str(exc)) from exc

    try:
        return LocalDevice.load(plaintext)

    except DataError as exc:
        raise LocalDeviceValidationError(f"Cannot load local device: {exc}") from exc


async def save_recovery_device(key_file: PurePath, device: LocalDevice, force: bool = False) -> str:
    """
    Return the recovery passphrase
    """
    assert key_file.suffix == RECOVERY_DEVICE_FILE_SUFFIX
    key_file = trio.Path(key_file)

    if await key_file.exists() and not force:
        raise LocalDeviceAlreadyExistsError(f"Device key file `{key_file}` already exists")

    passphrase, key = generate_recovery_passphrase()

    try:
        ciphertext = key.encrypt(device.dump())

    except (CryptoError, DataError) as exc:
        raise LocalDeviceValidationError(f"Cannot dump local device: {exc}") from exc

    key_file_content = key_file_serializer.dumps(
        {
            "type": DeviceFileType.RECOVERY,
            "ciphertext": ciphertext,
            "human_handle": device.human_handle,
            "device_label": device.device_label,
            "organization_id": device.organization_id,
            "device_id": device.device_id,
            "slug": device.slug,
        }
    )

    try:
        await key_file.parent.mkdir(mode=0o700, exist_ok=True, parents=True)
        await key_file.write_bytes(key_file_content)

    except OSError as exc:
        raise LocalDeviceError(f"Cannot save {key_file}: {exc}") from exc

    return passphrase


def _load_smartcard_extension():
    try:
        return import_module("parsec_ext.smartcard")
    except ModuleNotFoundError as exc:
        raise ModuleNotFoundError("Parsec smartcard extension not available") from exc


def is_smartcard_extension_available() -> bool:
    try:
        _load_smartcard_extension()
        return True
    except ModuleNotFoundError:
        return False


def load_device_with_smartcard(key_file: Path) -> LocalDevice:
    """
        LocalDeviceError
        LocalDeviceNotFoundError
        LocalDeviceCryptoError
        LocalDeviceValidationError
        LocalDevicePackingError
    """
    return _load_smartcard_extension().load_device_with_smartcard(key_file)


def save_device_with_smartcard_in_config(config_dir: Path, device: LocalDevice) -> Path:
    """
    Raises:
        LocalDeviceError
        LocalDeviceNotFoundError
        LocalDeviceCryptoError
        LocalDeviceValidationError
        LocalDevicePackingError
    """
    key_file = get_default_key_file(config_dir, device)
    # Why do we use `force=True` here ?
    # Key file name is per-device unique (given it contains the device slughash),
    # hence there is no risk to overwrite another device.
    # So if we are overwritting a key file it could be by:
    # - the same device object, hence overwritting has no effect
    # - a device object with same slughash but different device/user keys
    #   This would mean the device enrollment has been replayed (which is
    #   not possible in theory, but could occur in case of a rollback in the
    #   Parsec server), in this case the old device object is now invalid
    #   and it's a good thing to replace it.
    _load_smartcard_extension().save_device_with_smartcard(key_file, device, force=True)
    return key_file


def save_device_with_smartcard(key_file: Path, device: LocalDevice, force: bool) -> None:
    """
        LocalDeviceError
        LocalDeviceNotFoundError
        LocalDeviceCryptoError
        LocalDeviceValidationError
        LocalDevicePackingError
    """
    _load_smartcard_extension().save_device_with_smartcard(key_file, device, force)

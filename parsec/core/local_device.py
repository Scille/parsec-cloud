# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import attr
from pathlib import Path
from hashlib import sha256
from typing import List, Optional, Iterator

from parsec.serde import BaseSchema, fields, MsgpackSerializer
from parsec.crypto import (
    SecretKey,
    SigningKey,
    PrivateKey,
    CryptoError,
    derivate_secret_key_from_password,
)
from parsec.api.protocol import (
    OrganizationID,
    DeviceID,
    HumanHandle,
    HumanHandleField,
    OrganizationIDField,
    DeviceIDField,
)
from parsec.api.data import DataError, UserProfile
from parsec.core.types import EntryID, LocalDevice, BackendOrganizationAddr


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


class LegacyDeviceFileSchema(BaseSchema):
    """Schema for legacy device files where the filename contains complementary information."""

    type = fields.CheckedConstant("password", required=True)
    salt = fields.Bytes(required=True)
    ciphertext = fields.Bytes(required=True)

    # Since human_handle/device_label has been introduced, device_id is
    # redacted (i.e. user_id and device_name are 2 random uuids), hence
    # those fields have been added to the device file so the login page in
    # the GUI can use them to provide useful information.
    human_handle = HumanHandleField(allow_none=True, missing=None)
    device_label = fields.String(allow_none=True, missing=None)


class DeviceFileSchema(LegacyDeviceFileSchema):
    """Schema for device files that does not rely on the filename for complementary information."""

    # Override those fields to make them required (although `None` is still valid)
    human_handle = HumanHandleField(required=True, allow_none=True)
    device_label = fields.String(required=True, allow_none=True)

    # Store device ID, organization ID and slug in the device file
    # For legacy versions, this information is available in the file name
    device_id = DeviceIDField(required=True)
    organization_id = OrganizationIDField(required=True)
    slug = fields.String(required=True)


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
    device_label: Optional[str] = None,
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


def get_key_file(config_dir: Path, device: LocalDevice) -> Path:
    for available_device in _iter_available_devices(config_dir):
        if available_device.slug == device.slug:
            return available_device.key_file_path
    raise FileNotFoundError


def get_default_key_file(config_dir: Path, device: LocalDevice) -> Path:
    """Return the default keyfile path for a given device.

    Note that the filename does not carry any intrinsic meaning.
    Here, we simply use the slughash to avoid name collision.
    """
    return get_devices_dir(config_dir) / f"{device.slughash}.keys"


def get_devices_dir(config_dir: Path) -> Path:
    return config_dir / "devices"


@attr.s(slots=True, frozen=True, auto_attribs=True)
class AvailableDevice:
    key_file_path: Path
    organization_id: OrganizationID
    device_id: DeviceID
    human_handle: Optional[HumanHandle]
    device_label: Optional[str]
    slug: str

    @property
    def user_display(self) -> str:
        return str(self.human_handle or self.device_id.user_id)

    @property
    def short_user_display(self) -> str:
        return str(self.human_handle.label if self.human_handle else self.device_id.user_id)

    @property
    def device_display(self) -> str:
        return self.device_label or str(self.device_id.device_name)

    @property
    def slughash(self) -> str:
        return sha256(self.slug.encode()).hexdigest()


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
    )


def load_device_file(key_file_path: Path) -> Optional[AvailableDevice]:
    try:
        data = key_file_serializer.loads(key_file_path.read_bytes())
    except (FileNotFoundError, LocalDeviceError):
        # Not a valid device file, ignore this folder
        return None

    return AvailableDevice(
        key_file_path=key_file_path,
        organization_id=data["organization_id"],
        device_id=data["device_id"],
        human_handle=data["human_handle"],
        device_label=data["device_label"],
        slug=data["slug"],
    )


def _iter_available_devices(config_dir: Path) -> Iterator[AvailableDevice]:
    # Set of seen slugs
    seen = set()
    # Consider `.keys` files in devices directory and subdirectories
    key_file_paths = list(get_devices_dir(config_dir).rglob("*.keys"))
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
        LocalDeviceNotFoundError
        LocalDeviceCryptoError
        LocalDeviceValidationError
        LocalDevicePackingError
    """
    try:
        ciphertext = key_file.read_bytes()
    except OSError as exc:
        raise LocalDeviceNotFoundError(f"Config file `{key_file}` is missing") from exc

    try:
        data = key_file_serializer.loads(ciphertext)
    except LocalDeviceError:
        data = legacy_key_file_serializer.loads(ciphertext)

    try:
        key, _ = derivate_secret_key_from_password(password, data["salt"])
        plaintext = key.decrypt(data["ciphertext"])
    except CryptoError as exc:
        raise LocalDeviceCryptoError(str(exc)) from exc

    try:
        return LocalDevice.load(plaintext)

    except DataError as exc:
        raise LocalDeviceValidationError(f"Cannot load local device: {exc}") from exc


def save_device_with_password(config_dir: Path, device: LocalDevice, password: str) -> Path:
    """
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
    _save_device_with_password(key_file, device, password, force=True)
    return key_file


def _save_device_with_password(
    key_file: Path, device: LocalDevice, password: str, force: bool
) -> None:
    if key_file.exists() and not force:
        raise LocalDeviceAlreadyExistsError(f"Device key file `{key_file}` already exists")

    try:
        key, salt = derivate_secret_key_from_password(password)
        ciphertext = key.encrypt(device.dump())

    except (CryptoError, DataError) as exc:
        raise LocalDeviceValidationError(f"Cannot dump local device: {exc}") from exc

    key_file_content = key_file_serializer.dumps(
        {
            "salt": salt,
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
    _save_device_with_password(key_file, device, password=new_password, force=True)

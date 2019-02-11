import attr
import pendulum

from parsec.types import DeviceID, DeviceName, UserID
from parsec.trustchain import (
    unsecure_certified_device_extract_verify_key,
    unsecure_certified_user_extract_public_key,
)
from parsec.crypto import PublicKey, VerifyKey


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class RemoteDevice:

    device_id: DeviceID
    certified_device: bytes
    device_certifier: DeviceID

    created_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)
    revocated_on: pendulum.Pendulum = None
    certified_revocation: bytes = None
    revocation_certifier: DeviceID = None

    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def device_name(self) -> DeviceName:
        return self.device_id.device_name

    @property
    def user_id(self) -> UserID:
        return self.device_id.user_id

    @property
    def verify_key(self) -> VerifyKey:
        return unsecure_certified_device_extract_verify_key(self.certified_device)


class RemoteDevicesMapping:
    """
    Basically a frozen dict.
    """

    __slots__ = ("_read_only_mapping",)

    def __init__(self, *devices: RemoteDevice):
        self._read_only_mapping = {d.device_name: d for d in devices}

    def __repr__(self):
        return f"{self.__class__.__name__}({self._read_only_mapping!r})"

    def __getitem__(self, key):
        return self._read_only_mapping[key]

    def items(self):
        return self._read_only_mapping.items()

    def keys(self):
        return self._read_only_mapping.keys()

    def values(self):
        return self._read_only_mapping.values()

    def __iter__(self):
        return self._read_only_mapping.__iter__()

    def __in__(self, key):
        return self._read_only_mapping.__in__(key)


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class RemoteUser:

    user_id: UserID
    certified_user: bytes
    user_certifier: DeviceID
    devices: RemoteDevicesMapping = attr.ib(factory=RemoteDevicesMapping)

    created_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)

    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def public_key(self) -> PublicKey:
        return unsecure_certified_user_extract_public_key(self.certified_user)

    def is_revocated(self) -> bool:
        return all(bool(d.revocated_on) for d in self.devices.values())

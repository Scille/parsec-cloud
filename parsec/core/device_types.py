import attr
import pendulum

from parsec.trustchain import (
    unsecure_certified_device_extract_verify_key,
    unsecure_certified_user_extract_public_key,
)


@attr.s(slots=True, frozen=True, repr=False)
class RemoteDevice:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def device_name(self):
        return self.device_id.device_name

    @property
    def user_id(self):
        return self.device_id.user_id

    @property
    def verify_key(self):
        return unsecure_certified_device_extract_verify_key(self.certified_device)

    device_id = attr.ib()
    certified_device = attr.ib()
    device_certifier = attr.ib()

    created_on = attr.ib(factory=pendulum.now)
    revocated_on = attr.ib(default=None)
    certified_revocation = attr.ib(default=None)
    revocation_certifier = attr.ib(default=None)


@attr.s(slots=True, frozen=True, repr=False)
class LocalDevice:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def device_name(self):
        return self.device_id.device_name

    @property
    def user_id(self):
        return self.device_id.user_id

    @property
    def verify_key(self):
        return unsecure_certified_device_extract_verify_key(self.certified_device)

    device_id = attr.ib()
    certified_device = attr.ib()
    device_certifier = attr.ib()
    signing_key = attr.ib()
    private_key = attr.ib()

    created_on = attr.ib(factory=pendulum.now)
    revocated_on = attr.ib(default=None)
    certified_revocation = attr.ib(default=None)
    revocation_certifier = attr.ib(default=None)


class DevicesMapping:
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


@attr.s(slots=True, frozen=True, repr=False)
class RemoteUser:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def public_key(self):
        return unsecure_certified_user_extract_public_key(self.certified_user)

    user_id = attr.ib()
    certified_user = attr.ib()
    user_certifier = attr.ib()
    devices = attr.ib(factory=DevicesMapping)

    created_on = attr.ib(factory=pendulum.now)
    revocated_on = attr.ib(default=None)
    certified_revocation = attr.ib(default=None)
    revocation_certifier = attr.ib(default=None)

import re
from urllib.parse import urlsplit, urlunsplit, parse_qs

from parsec.crypto_types import VerifyKey, export_root_verify_key, import_root_verify_key


__all__ = (
    "OrganizationID",
    "BackendOrganizationBootstrapAddr",
    "BackendOrganizationAddr",
    "UserID",
    "DeviceName",
    "DeviceID",
)


class OrganizationID(str):
    __slots__ = ()
    regex = re.compile(r"^\w{1,32}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw):
            raise ValueError("Invalid organization ID")

    def __repr__(self):
        return f"<OrganizationID {super().__repr__()}>"


class BackendOrganizationAddr(str):
    __slots__ = ("_root_verify_key", "_organization")

    def __init__(self, raw: str):
        if not isinstance(raw, str):
            raise ValueError("Invalid user backend domain address.")

        parsed = urlsplit(raw)
        self._organization = OrganizationID(parsed.path[1:])

        query = parse_qs(parsed.query)
        try:
            self._root_verify_key = import_root_verify_key(query["rvk"][0])

        except (KeyError, IndexError) as exc:
            raise ValueError("Backend domain address must contains `rvk` params.") from exc

    def get_organization(self) -> OrganizationID:
        return self._organization

    def get_root_verify_key(self) -> VerifyKey:
        return self._root_verify_key


class BackendOrganizationBootstrapAddr(str):
    __slots__ = ("_bootstrap_token", "_organization")

    @classmethod
    def build(
        cls, backend_addr: str, name: str, bootstrap_token: str
    ) -> "BackendOrganizationBootstrapAddr":
        scheme, netloc, _, _, fragment = urlsplit(backend_addr)
        query = f"bootstrap-token={bootstrap_token}"
        return cls(urlunsplit((scheme, netloc, name, query, fragment)))

    def __init__(self, raw: str):
        if not isinstance(raw, str):
            raise ValueError("Invalid user backend domain address.")

        parsed = urlsplit(raw)

        self._organization = OrganizationID(parsed.path[1:])

        query = parse_qs(parsed.query)
        try:
            self._bootstrap_token = query["bootstrap-token"][0]

        except (KeyError, IndexError) as exc:
            raise ValueError(
                "Backend domain address must contains a `bootstrap-token` param."
            ) from exc

    def get_organization(self) -> OrganizationID:
        return self._organization

    def get_bootstrap_token(self) -> str:
        return self._bootstrap_token

    def generate_organization_addr(self, root_verify_key: VerifyKey) -> BackendOrganizationAddr:
        scheme, netloc, path, _, fragment = urlsplit(self)
        query = f"rvk={export_root_verify_key(root_verify_key)}"
        return BackendOrganizationAddr(urlunsplit((scheme, netloc, path, query, fragment)))


class UserID(str):
    __slots__ = ()
    regex = re.compile(r"^\w{1,32}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw):
            raise ValueError("Invalid user ID")

    def __repr__(self):
        return f"<UserID {super().__repr__()}>"


class DeviceName(str):
    __slots__ = ()
    regex = re.compile(r"^\w{1,32}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw):
            raise ValueError("Invalid device name")

    def __repr__(self):
        return f"<DeviceName {super().__repr__()}>"


class DeviceID(str):
    __slots__ = ()
    regex = re.compile(r"^\w{1,32}@\w{1,32}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw):
            raise ValueError("Invalid device ID")

    def __repr__(self):
        return f"<DeviceID {super().__repr__()}>"

    @property
    def user_id(self) -> UserID:
        return UserID(self.split("@")[0])

    @property
    def device_name(self) -> DeviceName:
        return DeviceName(self.split("@")[1])

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
from urllib.parse import parse_qs, urlsplit, urlunsplit

from parsec.crypto_types import VerifyKey, export_root_verify_key, import_root_verify_key

__all__ = (
    "OrganizationID",
    "BackendOrganizationBootstrapAddr",
    "BackendOrganizationAddr",
    "UserID",
    "DeviceName",
    "DeviceID",
)


class FrozenDict(dict):
    def __repr__(self):
        return f"{self.__class__.__name__}({dict.__repr__(self)})"

    def _ro_guard(*args, **kwargs):
        raise AttributeError("FrozenDict doesn't allow modifications")

    __setitem__ = _ro_guard
    __delitem__ = _ro_guard
    pop = _ro_guard
    clear = _ro_guard
    popitem = _ro_guard
    setdefault = _ro_guard
    update = _ro_guard

    def evolve(self, **data):
        return FrozenDict(**self, **data)


class OrganizationID(str):
    __slots__ = ()
    regex = re.compile(r"^\w{1,32}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw):
            raise ValueError("Invalid organization ID")

    def __repr__(self):
        return f"<OrganizationID {super().__repr__()}>"


class BackendAddr(str):
    __slots__ = ("_split", "_use_ssl")

    def __init__(self, raw: str):
        if not isinstance(raw, str):
            raise ValueError("Invalid backend address.")

        self._split = urlsplit(raw)
        self._parse_path(self._split.path)
        if self._split.query:
            params = parse_qs(self._split.query, keep_blank_values=True, strict_parsing=True)
        else:
            params = {}
        self._parse_and_consume_params(params)
        if params:
            raise ValueError(f"Unknown params: {','.join(params)}")

        if self._split.scheme != "parsec":
            raise ValueError("Backend addr must start with `parsec://`")

    def _parse_path(self, path):
        if path not in ("", "/"):
            raise ValueError("Backend addr cannot have path")

    def _parse_and_consume_params(self, params):
        value = params.pop("no_ssl", ("false",))
        if len(value) != 1:
            raise ValueError("Multiple values for param `no_ssl`")
        value = value[0].lower()
        # param is no_ssl, but we store use_ssl (so invert the values)
        if value == "false":
            self._use_ssl = True
        elif value == "true":
            self._use_ssl = False
        else:
            raise ValueError("Invalid `no_ssl` param value (must be true or false)")

    @property
    def use_ssl(self):
        return self._use_ssl

    @property
    def scheme(self):
        return self._split.scheme

    @property
    def hostname(self):
        return self._split.hostname

    @property
    def port(self):
        port = self._split.port
        if port:
            return port
        else:
            return 443 if self._use_ssl else 80


class BackendOrganizationAddr(BackendAddr):
    __slots__ = ("_root_verify_key", "_organization_id")

    @classmethod
    def build(
        cls, backend_addr: str, name: str, root_verify_key: VerifyKey
    ) -> "BackendOrganizationAddr":
        scheme, netloc, base_name, base_query, fragment = urlsplit(backend_addr)
        rvk = export_root_verify_key(root_verify_key)
        query = f"{base_query}&rvk={rvk}" if base_query else f"rvk={rvk}"
        name = f"{base_name}/{name}" if base_name else name
        return cls(urlunsplit((scheme, netloc, name, query, fragment)))

    def _parse_and_consume_params(self, params):
        super()._parse_and_consume_params(params)
        value = params.pop("rvk", ())
        if len(value) != 1:
            raise ValueError("Missing mandatory `rvk` param")
        try:
            self._root_verify_key = import_root_verify_key(value[0])
        except ValueError as exc:
            raise ValueError("Invalid `rvk` param value") from exc

    def _parse_path(self, path):
        self._organization_id = OrganizationID(path[1:])

    @property
    def organization_id(self) -> OrganizationID:
        return self._organization_id

    @property
    def root_verify_key(self) -> VerifyKey:
        return self._root_verify_key


class BackendOrganizationBootstrapAddr(BackendAddr):
    __slots__ = ("_bootstrap_token", "_organization_id")

    @classmethod
    def build(
        cls, backend_addr: str, name: str, bootstrap_token: str
    ) -> "BackendOrganizationBootstrapAddr":
        scheme, netloc, base_name, base_query, fragment = urlsplit(backend_addr)
        query = (
            f"{base_query}&bootstrap-token={bootstrap_token}"
            if base_query
            else f"bootstrap-token={bootstrap_token}"
        )
        name = f"{base_name}/{name}" if base_name else name
        return cls(urlunsplit((scheme, netloc, name, query, fragment)))

    def __init__(self, raw: str):
        super().__init__(raw)

        self._organization_id = OrganizationID(self._split.path[1:])

        query = parse_qs(self._split.query)
        try:
            self._bootstrap_token = query["bootstrap-token"][0]

        except (KeyError, IndexError) as exc:
            raise ValueError(
                "Backend domain address must contains a `bootstrap-token` param."
            ) from exc

    def _parse_and_consume_params(self, params):
        super()._parse_and_consume_params(params)
        value = params.pop("bootstrap-token", ())
        if len(value) != 1:
            raise ValueError("Missing mandatory `bootstrap-token` param")
        self._bootstrap_token = value[0]

    def _parse_path(self, path):
        self._organization_id = OrganizationID(path[1:])

    @property
    def organization_id(self) -> OrganizationID:
        return self._organization_id

    @property
    def bootstrap_token(self) -> str:
        return self._bootstrap_token

    def generate_organization_addr(self, root_verify_key: VerifyKey) -> BackendOrganizationAddr:
        scheme, netloc, _, _, fragment = urlsplit(self)
        query = "no_ssl=true" if not self.use_ssl else ""
        backend_addr = urlunsplit((scheme, netloc, "", query, fragment))
        return BackendOrganizationAddr.build(backend_addr, self.organization_id, root_verify_key)


class UserID(str):
    __slots__ = ()
    regex = re.compile(r"^\w{1,32}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw):
            raise ValueError("Invalid user name")

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

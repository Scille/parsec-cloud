# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Tuple, Optional, TYPE_CHECKING
from urllib.parse import urlsplit, urlunsplit, parse_qs, quote_plus, unquote_plus, urlencode
from marshmallow import ValidationError

from parsec.serde import fields
from parsec.crypto import (
    VerifyKey,
    export_root_verify_key,
    import_root_verify_key,
    binary_urlsafe_decode,
    binary_urlsafe_encode,
)
from parsec.api.protocol import OrganizationID, InvitationType, InvitationToken
from parsec.api.data import EntryID


PARSEC_SCHEME = "parsec"


class BackendAddr:
    """
    Represent the URL to reach a backend
    (e.g. ``parsec://parsec.example.com/``)
    """

    __slots__ = ("_hostname", "_port", "_use_ssl", "_custom_port", "_netloc")

    def __eq__(self, other):
        if isinstance(other, BackendAddr):
            return self.to_url() == other.to_url()
        else:
            return False

    def __hash__(self):
        return hash(self.to_url())

    def __init__(self, hostname: str, port: Optional[int] = None, use_ssl=True):
        assert not hostname.startswith("parsec://")
        self._hostname = hostname
        self._port, self._custom_port = self._parse_port(port, use_ssl)
        self._use_ssl = use_ssl
        if self._custom_port:
            self._netloc = f"{self._hostname}:{self._custom_port}"
        else:
            self._netloc = self._hostname

    @staticmethod
    def _parse_port(port, use_ssl) -> Tuple[int, Optional[int]]:
        """Returns port to use and port to add in netloc if needed"""
        default_port = 443 if use_ssl else 80
        if port not in (None, default_port):
            return port, port
        else:
            return default_port, None

    def __str__(self):
        return self.to_url()

    def __repr__(self):
        return f"{type(self).__name__}(url={self.to_url()})"

    @classmethod
    def from_url(cls, url: str, *, allow_http_redirection: bool = False):
        """
        Use `allow_http_redirection` to accept backend redirection URL,
        for instance `http://example.com/redirect/myOrg?token=123` will be
        converted into `parsec://example.com/myOrg?token=123&no_ssl=True`.
        """
        trim = url.strip()
        split = urlsplit(trim)
        path = unquote_plus(split.path)

        if split.query:
            # Note `parse_qs` takes care of percent-encoding
            params = parse_qs(
                split.query,
                keep_blank_values=True,
                strict_parsing=True,
                encoding="utf-8",
                errors="strict",
            )
        else:
            params = {}

        if allow_http_redirection and split.scheme in ("http", "https"):
            # `no_ssl` is defined by http/https scheme and shouldn't be
            # overwritten by the query part of the url
            params["no_ssl"] = ["true" if split.scheme == "http" else "false"]
            # Remove the `/redirect/` path prefix
            split_path = path.split("/", 2)
            if split_path[:2] != ["", "redirect"]:
                raise ValueError("HTTP to Parsec redirection URL must have a `/redirect/...` path")
            try:
                path = f"/{split_path[2]}"
            except IndexError:
                path = ""

        elif split.scheme != PARSEC_SCHEME:
            raise ValueError(f"Must start with `{PARSEC_SCHEME}://`")

        if not split.hostname:
            raise ValueError("Missing mandatory hostname")

        kwargs = {
            **cls._from_url_parse_and_consume_params(params),
            **cls._from_url_parse_path(path),
        }

        return cls(hostname=split.hostname, port=split.port, **kwargs)

    @classmethod
    def _from_url_parse_path(cls, path):
        if path not in ("", "/"):
            raise ValueError("Cannot have path")
        return {}

    @classmethod
    def _from_url_parse_and_consume_params(cls, params):
        value = params.pop("no_ssl", ("false",))
        if len(value) != 1:
            raise ValueError("Multiple values for param `no_ssl`")
        value = value[0].lower()
        # param is no_ssl, but we store use_ssl (so invert the values)
        if value == "false":
            return {"use_ssl": True}
        elif value == "true":
            return {"use_ssl": False}
        else:
            raise ValueError("Invalid `no_ssl` param value (must be true or false)")

    def to_http_domain_url(self, path: str = "") -> str:
        if self._use_ssl:
            scheme = "https"
        else:
            scheme = "http"

        return urlunsplit((scheme, self._netloc, path, None, None))

    def to_url(self) -> str:
        query = urlencode(self._to_url_get_params())
        return urlunsplit(
            (PARSEC_SCHEME, self._netloc, quote_plus(self._to_url_get_path()), query, None)
        )

    def _to_url_get_path(self):
        return ""

    def _to_url_get_params(self):
        # Return a list to easily manage the order of params
        return [("no_ssl", "true")] if not self._use_ssl else []

    def get_backend_addr(self):
        # Compatibility with Rust where inheritance doesn't exist
        return self

    @property
    def hostname(self):
        return self._hostname

    @property
    def port(self):
        return self._port

    @property
    def netloc(self):
        return self._netloc

    @property
    def use_ssl(self):
        return self._use_ssl


_PyBackendAddr = BackendAddr
if not TYPE_CHECKING:
    try:
        from libparsec.types import BackendAddr as _RsBackendAddr
    except ImportError:
        pass
    else:
        BackendAddr = _RsBackendAddr


class BackendOrganizationAddr(_PyBackendAddr):
    """
    Represent the URL to access an organization within a backend
    (e.g. ``parsec://parsec.example.com/MyOrg?rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss``)
    """

    __slots__ = ("_root_verify_key", "_organization_id")

    def __init__(self, organization_id: OrganizationID, root_verify_key: VerifyKey, **kwargs):
        super().__init__(**kwargs)
        self._organization_id = organization_id
        self._root_verify_key = root_verify_key

    @classmethod
    def _from_url_parse_path(cls, path):
        return {"organization_id": OrganizationID(path[1:])}

    @classmethod
    def _from_url_parse_and_consume_params(cls, params):
        kwargs = super()._from_url_parse_and_consume_params(params)

        value = params.pop("rvk", ())
        if len(value) != 1:
            raise ValueError("Missing mandatory `rvk` param")
        try:
            kwargs["root_verify_key"] = import_root_verify_key(value[0])
        except ValueError as exc:
            raise ValueError("Invalid `rvk` param value") from exc

        return kwargs

    def _to_url_get_path(self):
        return str(self.organization_id)

    def _to_url_get_params(self):
        return [
            *super()._to_url_get_params(),
            ("rvk", export_root_verify_key(self.root_verify_key)),
        ]

    @property
    def organization_id(self) -> OrganizationID:
        return self._organization_id

    @property
    def root_verify_key(self) -> VerifyKey:
        return self._root_verify_key

    @classmethod
    def build(
        cls, backend_addr: BackendAddr, organization_id: OrganizationID, root_verify_key: VerifyKey
    ) -> "BackendOrganizationAddr":
        return cls(
            hostname=backend_addr.hostname,
            port=backend_addr.port,
            use_ssl=backend_addr.use_ssl,
            organization_id=organization_id,
            root_verify_key=root_verify_key,
        )


_PyBackendOrganizationAddr = BackendOrganizationAddr
if not TYPE_CHECKING:
    try:
        from libparsec.types import BackendOrganizationAddr as _RsBackendOrganizationAddr
    except ImportError:
        pass
    else:
        BackendOrganizationAddr = _RsBackendOrganizationAddr


class BackendActionAddr(_PyBackendAddr):
    __slots__ = ()

    @classmethod
    def from_url(cls, url: str, **kwargs):
        if cls is not _PyBackendActionAddr:
            return _PyBackendAddr.from_url.__func__(cls, url, **kwargs)

        else:
            for type in (
                _PyBackendOrganizationBootstrapAddr,
                _PyBackendOrganizationFileLinkAddr,
                _PyBackendInvitationAddr,
                _PyBackendPkiEnrollmentAddr,
            ):
                try:
                    return _PyBackendAddr.from_url.__func__(type, url, **kwargs)
                except ValueError:
                    pass

            raise ValueError("Invalid URL format")


_PyBackendActionAddr = BackendActionAddr
if not TYPE_CHECKING:
    try:
        from libparsec.types import BackendActionAddr as _RsBackendActionAddr
    except ImportError:
        pass
    else:
        BackendActionAddr = _RsBackendActionAddr


class BackendOrganizationBootstrapAddr(_PyBackendActionAddr):
    """
    Represent the URL to bootstrap an organization within a backend
    (e.g. ``parsec://parsec.example.com/my_org?action=bootstrap_organization&token=1234ABCD``)
    """

    __slots__ = ("_organization_id", "_token")

    def __init__(self, organization_id: OrganizationID, token: Optional[str], **kwargs):
        super().__init__(**kwargs)
        self._organization_id = organization_id
        self._token = token

    @classmethod
    def _from_url_parse_path(cls, path):
        return {"organization_id": OrganizationID(path[1:])}

    @classmethod
    def _from_url_parse_and_consume_params(cls, params):
        kwargs = super()._from_url_parse_and_consume_params(params)

        value = params.pop("action", ())
        if len(value) != 1:
            raise ValueError("Missing mandatory `action` param")
        if value[0] != "bootstrap_organization":
            raise ValueError("Expected `action=bootstrap_organization` param value")

        value = params.pop("token", ())
        if len(value) > 1:
            raise ValueError("Multiple values for param `token`")
        elif value and value[0]:
            kwargs["token"] = value[0]
        else:
            kwargs["token"] = None

        return kwargs

    def _to_url_get_path(self):
        return str(self.organization_id)

    def _to_url_get_params(self):
        params = [("action", "bootstrap_organization")]
        if self._token:
            params.append(("token", self._token))
        return [*params, *super()._to_url_get_params()]

    @classmethod
    def build(
        cls, backend_addr: BackendAddr, organization_id: OrganizationID, token: Optional[str] = None
    ) -> "BackendOrganizationBootstrapAddr":
        return cls(
            hostname=backend_addr.hostname,
            port=backend_addr.port,
            use_ssl=backend_addr.use_ssl,
            organization_id=organization_id,
            token=token,
        )

    def generate_organization_addr(self, root_verify_key: VerifyKey) -> BackendOrganizationAddr:
        return _PyBackendOrganizationAddr.build(
            backend_addr=self, organization_id=self.organization_id, root_verify_key=root_verify_key
        )

    @property
    def organization_id(self) -> OrganizationID:
        return self._organization_id

    @property
    def token(self) -> str:
        # For legacy reasons, token must always be provided, hence default
        # token is the empty one (which is used for spontaneous organization
        # bootstrap without prior organization creation)
        return self._token if self._token is not None else ""


_PyBackendOrganizationBootstrapAddr = BackendOrganizationBootstrapAddr
if not TYPE_CHECKING:
    try:
        from libparsec.types import (
            BackendOrganizationBootstrapAddr as _RsBackendOrganizationBootstrapAddr,
        )
    except ImportError:
        pass
    else:
        BackendOrganizationBootstrapAddr = _RsBackendOrganizationBootstrapAddr


class BackendOrganizationFileLinkAddr(_PyBackendActionAddr):
    """
    Represent the URL to share a file link
    (e.g. ``parsec://parsec.example.com/my_org?action=file_link&workspace_id=xx&path=yy``)
    """

    __slots__ = ("_organization_id", "_workspace_id", "_encrypted_path")

    def __init__(
        self,
        organization_id: OrganizationID,
        workspace_id: EntryID,
        encrypted_path: bytes,
        **kwargs,
    ):
        super().__init__(**kwargs)
        self._organization_id = organization_id
        self._workspace_id = workspace_id
        self._encrypted_path = encrypted_path

    @classmethod
    def _from_url_parse_path(cls, path):
        return {"organization_id": OrganizationID(path[1:])}

    @classmethod
    def _from_url_parse_and_consume_params(cls, params):
        kwargs = super()._from_url_parse_and_consume_params(params)

        value = params.pop("action", ())
        if len(value) != 1:
            raise ValueError("Missing mandatory `action` param")
        if value[0] != "file_link":
            raise ValueError("Expected `action=file_link` param value")

        value = params.pop("workspace_id", ())
        if len(value) != 1:
            raise ValueError("Missing mandatory `workspace_id` param")
        try:
            kwargs["workspace_id"] = EntryID.from_hex(value[0])
        except ValueError as exc:
            raise ValueError("Invalid `workspace_id` param value") from exc

        value = params.pop("path", ())
        if len(value) != 1:
            raise ValueError("Missing mandatory `path` param")
        try:
            kwargs["encrypted_path"] = binary_urlsafe_decode(value[0])
        except ValueError as exc:
            raise ValueError("Invalid `path` param value") from exc

        return kwargs

    def _to_url_get_path(self):
        return str(self.organization_id)

    def _to_url_get_params(self):
        params = [
            ("action", "file_link"),
            ("workspace_id", self._workspace_id.hex),
            ("path", binary_urlsafe_encode(self._encrypted_path)),
        ]
        return [*params, *super()._to_url_get_params()]

    @classmethod
    def build(
        cls,
        organization_addr: BackendOrganizationAddr,
        workspace_id: EntryID,
        encrypted_path: bytes,
    ) -> "BackendOrganizationFileLinkAddr":
        return cls(
            hostname=organization_addr.hostname,
            port=organization_addr.port,
            use_ssl=organization_addr.use_ssl,
            organization_id=organization_addr.organization_id,
            workspace_id=workspace_id,
            encrypted_path=encrypted_path,
        )

    @property
    def organization_id(self) -> OrganizationID:
        return self._organization_id

    @property
    def workspace_id(self) -> EntryID:
        return self._workspace_id

    @property
    def encrypted_path(self) -> bytes:
        return self._encrypted_path


_PyBackendOrganizationFileLinkAddr = BackendOrganizationFileLinkAddr
if not TYPE_CHECKING:
    try:
        from libparsec.types import (
            BackendOrganizationFileLinkAddr as _RsBackendOrganizationFileLinkAddr,
        )
    except ImportError:
        pass
    else:
        BackendOrganizationFileLinkAddr = _RsBackendOrganizationFileLinkAddr


class BackendOrganizationAddrField(fields.Field):
    def _deserialize(self, value, attr, data):
        try:
            return BackendOrganizationAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(self, value, attr, data):
        if value is None:
            return None

        return value.to_url()


class BackendAddrField(fields.Field):
    def _deserialize(self, value, attr, data):
        try:
            return BackendAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(self, value, attr, data):
        if value is None:
            return None

        return value.to_url()


class BackendInvitationAddr(_PyBackendActionAddr):
    """
    Represent the URL to invite a user or a device
    (e.g. ``parsec://parsec.example.com/my_org?action=claim_user&token=3a50b191122b480ebb113b10216ef343``)
    """

    __slots__ = ("_organization_id", "_invitation_type", "_token")

    def __init__(
        self,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
        **kwargs,
    ):
        if token is None:
            raise TypeError("Missing required argument: 'token'")
        super().__init__(**kwargs)
        self._organization_id = organization_id
        self._invitation_type = invitation_type
        self._token = token

    @classmethod
    def _from_url_parse_path(cls, path):
        return {"organization_id": OrganizationID(path[1:])}

    @classmethod
    def _from_url_parse_and_consume_params(cls, params):
        kwargs = super()._from_url_parse_and_consume_params(params)

        value = params.pop("action", ())
        if len(value) != 1:
            raise ValueError("Missing mandatory `action` param")
        if value[0] == "claim_user":
            kwargs["invitation_type"] = InvitationType.USER
        elif value[0] == "claim_device":
            kwargs["invitation_type"] = InvitationType.DEVICE
        else:
            raise ValueError("Expected `action=claim_user` or `action=claim_device` param value")

        value = params.pop("token", ())
        if len(value) != 1:
            raise ValueError("Missing mandatory `token` param")
        try:
            kwargs["token"] = InvitationToken.from_hex(value[0])
        except ValueError:
            raise ValueError("Invalid `token` param value")

        return kwargs

    def _to_url_get_path(self):
        return str(self.organization_id)

    def _to_url_get_params(self):
        action = {InvitationType.USER: "claim_user", InvitationType.DEVICE: "claim_device"}[
            self._invitation_type
        ]
        if self._token is None:
            return [("action", action), *super()._to_url_get_params()]
        return [("action", action), ("token", self._token.hex), *super()._to_url_get_params()]

    def to_http_redirection_url(self) -> str:
        # Skipping no_ssl param because it is already in the scheme
        query = urlencode({k: v for k, v in self._to_url_get_params() if k != "no_ssl"})
        path = "/redirect/" + quote_plus(self._to_url_get_path())
        if self._use_ssl:
            scheme = "https"
        else:
            scheme = "http"

        return urlunsplit((scheme, self._netloc, path, query, None))

    @classmethod
    def build(
        cls,
        backend_addr: BackendAddr,
        organization_id: OrganizationID,
        invitation_type: InvitationType,
        token: InvitationToken,
    ) -> "BackendInvitationAddr":
        return cls(
            hostname=backend_addr.hostname,
            port=backend_addr.port,
            use_ssl=backend_addr.use_ssl,
            organization_id=organization_id,
            invitation_type=invitation_type,
            token=token,
        )

    def generate_organization_addr(self, root_verify_key: VerifyKey) -> _PyBackendOrganizationAddr:
        return _PyBackendOrganizationAddr.build(
            backend_addr=self, organization_id=self.organization_id, root_verify_key=root_verify_key
        )

    def generate_backend_addr(self) -> _PyBackendAddr:
        return _PyBackendAddr(self.hostname, self.port, self.use_ssl)

    @property
    def organization_id(self) -> OrganizationID:
        return self._organization_id

    @property
    def invitation_type(self) -> InvitationType:
        return self._invitation_type

    @property
    def token(self) -> InvitationToken:
        return self._token


_PyBackendInvitationAddr = BackendInvitationAddr
if not TYPE_CHECKING:
    try:
        from libparsec.types import BackendInvitationAddr as _RsBackendInvitationAddr
    except ImportError:
        pass
    else:
        BackendInvitationAddr = _RsBackendInvitationAddr


class BackendPkiEnrollmentAddr(_PyBackendActionAddr):
    """
    Represent the URL used to reach an organization to request a PKI-based enrollment
    (e.g. ``parsec://parsec.example.com/my_org?action=pki_enrollment``)
    """

    __slots__ = ("_organization_id",)

    def __init__(self, organization_id: OrganizationID, **kwargs):
        super().__init__(**kwargs)
        self._organization_id = organization_id

    @classmethod
    def _from_url_parse_path(cls, path):
        return {"organization_id": OrganizationID(path[1:])}

    @classmethod
    def _from_url_parse_and_consume_params(cls, params):
        kwargs = super()._from_url_parse_and_consume_params(params)

        value = params.pop("action", ())
        if len(value) != 1:
            raise ValueError("Missing mandatory `action` param")
        if value[0] != "pki_enrollment":
            raise ValueError("Expected `action=pki_enrollment` param value")

        return kwargs

    def _to_url_get_path(self):
        return str(self.organization_id)

    def _to_url_get_params(self):
        return [("action", "pki_enrollment"), *super()._to_url_get_params()]

    def to_http_redirection_url(self) -> str:
        # Skipping no_ssl param because it is already in the scheme
        query = urlencode({k: v for k, v in self._to_url_get_params() if k != "no_ssl"})
        path = "/redirect/" + quote_plus(self._to_url_get_path())
        if self._use_ssl:
            scheme = "https"
        else:
            scheme = "http"

        return urlunsplit((scheme, self._netloc, path, query, None))

    @classmethod
    def build(
        cls, backend_addr: BackendAddr, organization_id: OrganizationID
    ) -> "BackendInvitationAddr":
        return cls(
            hostname=backend_addr.hostname,
            port=backend_addr.port,
            use_ssl=backend_addr.use_ssl,
            organization_id=organization_id,
        )

    def generate_organization_addr(self, root_verify_key: VerifyKey) -> _PyBackendOrganizationAddr:
        return _PyBackendOrganizationAddr.build(
            backend_addr=self, organization_id=self.organization_id, root_verify_key=root_verify_key
        )

    def generate_backend_addr(self) -> _PyBackendAddr:
        return _PyBackendAddr(self.hostname, self.port, self.use_ssl)

    @property
    def organization_id(self) -> OrganizationID:
        return self._organization_id


_PyBackendPkiEnrollmentAddr = BackendPkiEnrollmentAddr
if not TYPE_CHECKING:
    try:
        from libparsec.types import BackendPkiEnrollmentAddr as _RsBackendPkiEnrollmentAddr
    except ImportError:
        pass
    else:
        BackendPkiEnrollmentAddr = _RsBackendPkiEnrollmentAddr


class BackendPkiEnrollmentAddrField(fields.Field):
    def _deserialize(self, value, attr, data):
        try:
            return BackendPkiEnrollmentAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(self, value, attr, data):
        if value is None:
            return None

        return value.to_url()

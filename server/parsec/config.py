# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import logging
from dataclasses import dataclass, field, fields
from enum import Enum
from typing import TYPE_CHECKING, Literal
from urllib.parse import urlparse, urlunparse

from jinja2.environment import Environment

from parsec._parsec import ActiveUsersLimit, DateTime, EmailAddress, ParsecAddr, SecretKey
from parsec.email_rate_limit import EmailRateLimit

if TYPE_CHECKING:
    from parsec.components.memory.organization import MemoryOrganization, OrganizationID
    from parsec.components.organization import TosLocale, TosUrl


def hide_password(url: str) -> str:
    parsed = urlparse(url)
    if parsed.password is None:
        return url
    args = list(parsed)
    args[1] = f"{parsed.username}:***@{parsed.hostname}:{parsed.port}"
    return urlunparse(args)


class BaseDatabaseConfig:
    # Overloaded by children
    type: Literal["POSTGRESQL", "MOCKED"]

    def set_min_max_connections(self, min_connections: int, max_connections: int) -> None:
        raise NotImplementedError

    def is_mocked(self) -> bool:
        raise NotImplementedError


@dataclass(slots=True)
class PostgreSQLDatabaseConfig(BaseDatabaseConfig):
    type = "POSTGRESQL"

    url: str
    min_connections: int
    max_connections: int

    def set_min_max_connections(self, min_connections: int, max_connections: int) -> None:
        self.min_connections = min_connections
        self.max_connections = max_connections

    def is_mocked(self) -> bool:
        return False

    def __str__(self) -> str:
        url = hide_password(self.url)
        return f"{self.__class__.__name__}(url={url}, min_connections={self.min_connections}, max_connections={self.max_connections})"

    __repr__ = __str__


@dataclass(slots=True)
class MockedDatabaseConfig(BaseDatabaseConfig):
    type = "MOCKED"

    def set_min_max_connections(self, min_connections: int, max_connections: int) -> None:
        pass

    def is_mocked(self) -> bool:
        return True


class BaseBlockStoreConfig:
    # Overloaded by children
    type: Literal["RAID0", "RAID1", "RAID5", "S3", "SWIFT", "POSTGRESQL", "MOCKED", "DISABLED"]


@dataclass(slots=True)
class RAID0BlockStoreConfig(BaseBlockStoreConfig):
    type = "RAID0"

    blockstores: list[BaseBlockStoreConfig]


@dataclass(slots=True)
class RAID1BlockStoreConfig(BaseBlockStoreConfig):
    type = "RAID1"

    blockstores: list[BaseBlockStoreConfig]
    partial_create_ok: bool = False


@dataclass(slots=True)
class RAID5BlockStoreConfig(BaseBlockStoreConfig):
    type = "RAID5"

    blockstores: list[BaseBlockStoreConfig]
    partial_create_ok: bool = False


@dataclass(slots=True)
class S3BlockStoreConfig(BaseBlockStoreConfig):
    type = "S3"

    s3_endpoint_url: str | None
    s3_region: str
    s3_bucket: str
    s3_key: str
    s3_secret: str

    def __str__(self) -> str:
        # Do not show the secret in the logs
        return f"{self.__class__.__name__}(s3_endpoint_url={self.s3_endpoint_url}, s3_region={self.s3_region}, s3_bucket={self.s3_bucket}, s3_key={self.s3_key})"

    __repr__ = __str__


@dataclass(slots=True)
class SWIFTBlockStoreConfig(BaseBlockStoreConfig):
    type = "SWIFT"

    swift_authurl: str
    swift_tenant: str
    swift_container: str
    swift_user: str
    swift_password: str

    def __str__(self) -> str:
        # Do not show the password in the logs
        return f"{self.__class__.__name__}(swift_authurl={self.swift_authurl}, swift_tenant={self.swift_tenant}, swift_container={self.swift_container}, swift_user={self.swift_user})"

    __repr__ = __str__


@dataclass(slots=True)
class PostgreSQLBlockStoreConfig(BaseBlockStoreConfig):
    type = "POSTGRESQL"


@dataclass(slots=True)
class MockedBlockStoreConfig(BaseBlockStoreConfig):
    type = "MOCKED"


@dataclass(slots=True)
class DisabledBlockStoreConfig(BaseBlockStoreConfig):
    type = "DISABLED"


@dataclass(slots=True)
class SmtpEmailConfig:
    type = "SMTP"

    host: str
    port: int
    host_user: str | None
    host_password: str | None
    use_ssl: bool
    use_tls: bool
    sender: EmailAddress

    def __str__(self) -> str:
        # Do not show the password in the logs
        return f"{self.__class__.__name__}(sender={self.sender}, host={self.host}, port={self.port}, use_ssl={self.use_ssl}, use_tls={self.use_tls})"

    __repr__ = __str__


@dataclass(slots=True)
class MockedSentEmail:
    sender: EmailAddress
    recipient: EmailAddress
    timestamp: DateTime
    body: str


@dataclass(slots=True)
class MockedEmailConfig:
    type = "MOCKED"

    sender: EmailAddress
    # Emails sent, ordered from oldest to newest.
    # Keeping the emails is needed for testbed server's `GET /testbed/mailbox/{email}` route.
    sent_emails: list[MockedSentEmail] = field(default_factory=list)


EmailConfig = SmtpEmailConfig | MockedEmailConfig


class LogLevel(Enum):
    DEBUG = logging.DEBUG
    INFO = logging.INFO
    WARNING = logging.WARNING
    ERROR = logging.ERROR
    CRITICAL = logging.CRITICAL


class AllowedClientAgent(Enum):
    """
    Client agent allowed to connect to an organization.

    - `NATIVE_ONLY`: Only desktop client is allowed
    - `NATIVE_OR_WEB`: Desktop and web clients are allowed
    """

    # Note the enum value is used in the PostgreSQL queries, so modifying it
    # impacts the datamodel !
    NATIVE_ONLY = "NATIVE_ONLY"
    NATIVE_OR_WEB = "NATIVE_OR_WEB"


class AccountVaultStrategy(Enum):
    """
    The account vault allows the user to store sensitive data (e.g. device keys)
    server-side.

    This typically allows simplified authentication on a new machine (i.e. no need
    for device-to-device enrollment) at a the cost of slightly less security (since
    the server can try to brute-force the user's password to gain access to his vault).

    Note this configuration is purely advisory since only the client can decrypt his
    user's vault content, and hence it would be pointless to try to enforce it on the
    server side.
    """

    # Note the enum value is used in the PostgreSQL queries, so modifying it
    # impacts the datamodel !
    ALLOWED = "ALLOWED"
    FORBIDDEN = "FORBIDDEN"


@dataclass(slots=True, kw_only=True)
class BackendConfig:
    debug: bool
    jinja_env: Environment
    db_config: BaseDatabaseConfig
    blockstore_config: BaseBlockStoreConfig
    email_config: SmtpEmailConfig | MockedEmailConfig

    # URL of the server to use when generating redirect URLs.
    # This is currently used for two things:
    # - For invitation URL in emails
    # - In the redirect API (e.g. `GET /redirect/FOO` -> `302 <server_addr>/FOO`)
    server_addr: ParsecAddr

    # Bearer token used to authenticate the administration API
    administration_token: str

    # Minimal wait time (in seconds) between two email having the same recipient or initiator IP address
    email_rate_limit_cooldown_delay: int
    # Maximum number of mail send per hour for any given recipient or initiator IP address
    # Note setting it to `0` disable the check.
    email_rate_limit_max_per_hour: int
    # Small hack here: since the rate limit system is purely in-memory and configured
    # only by fields from this class, we instantiate here to simplify passing it
    # to the different components needing it.
    email_rate_limit: EmailRateLimit = field(init=False)

    # Random value used to make unpredictable (but still stable & realistic) the password
    # algorithm configuration returned for non-existing accounts, hence preventing an
    # attacker from using it as an oracle to determine if a given email has an account.
    fake_account_password_algorithm_seed: SecretKey

    # Amount of time (in seconds) before a keep alive message is sent to an SSE
    # connection. Set to `None` to disable keep alive messages.
    sse_keepalive: int | None = None

    # Comma separated list of IP Addresses, IP Networks, or literals (e.g. UNIX Socket path) to trust with proxy headers
    # Use "*" to trust all proxies. If not provided, the gunicorn/uvicorn `FORWARDED_ALLOW_IPS`
    # environment variable is used, defaulting to trusting only localhost if absent.
    proxy_trusted_addresses: str | None = None

    organization_bootstrap_webhook_url: str | None = None
    organization_spontaneous_bootstrap: bool = False
    organization_initial_active_users_limit: ActiveUsersLimit = field(
        default_factory=lambda: ActiveUsersLimit.NO_LIMIT
    )
    organization_initial_user_profile_outsider_allowed: bool = True
    # Note minimum archiving period must be a positive value !
    organization_initial_minimum_archiving_period: int = 2592000  # seconds (i.e 30 days)
    organization_initial_tos: dict[TosLocale, TosUrl] | None = None
    organization_initial_allowed_client_agent: AllowedClientAgent = AllowedClientAgent.NATIVE_OR_WEB
    organization_initial_account_vault_strategy: AccountVaultStrategy = AccountVaultStrategy.ALLOWED

    # Number of SSE events kept in memory to allow client to catch up on reconnection
    sse_events_cache_size: int = 1024
    backend_mocked_data: dict[OrganizationID, MemoryOrganization] | None = None

    def logging_kwargs(self) -> dict[str, str]:
        """
        Provide a safe dictionary for logging the backend configuration.
        """
        kwargs = {field.name: repr(getattr(self, field.name)) for field in fields(self)}
        kwargs["administration_token"] = "***"
        kwargs["fake_account_password_algorithm_seed"] = "***"
        return kwargs

    def __post_init__(self):
        # Sanity checks
        assert self.sse_keepalive is None or self.sse_keepalive >= 0, self.sse_keepalive
        assert self.organization_initial_minimum_archiving_period >= 0, (
            self.organization_initial_minimum_archiving_period
        )
        assert self.email_rate_limit_cooldown_delay >= 0, self.email_rate_limit_cooldown_delay
        assert self.email_rate_limit_max_per_hour >= 0, self.email_rate_limit_max_per_hour

        self.email_rate_limit = EmailRateLimit(
            cooldown_delay=self.email_rate_limit_cooldown_delay,
            max_per_hour=self.email_rate_limit_max_per_hour,
        )

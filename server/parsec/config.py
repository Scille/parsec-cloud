# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass, field

from parsec._parsec import ActiveUsersLimit, BackendAddr


class BaseBlockStoreConfig:
    # Overloaded by children
    type: str


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


@dataclass(slots=True)
class SWIFTBlockStoreConfig(BaseBlockStoreConfig):
    type = "SWIFT"

    swift_authurl: str
    swift_tenant: str
    swift_container: str
    swift_user: str
    swift_password: str


@dataclass(slots=True)
class PostgreSQLBlockStoreConfig(BaseBlockStoreConfig):
    type = "POSTGRESQL"


@dataclass(slots=True)
class MockedBlockStoreConfig(BaseBlockStoreConfig):
    type = "MOCKED"


@dataclass(slots=True)
class SmtpEmailConfig:
    type = "SMTP"

    host: str
    port: int
    host_user: str | None
    host_password: str | None
    use_ssl: bool
    use_tls: bool
    sender: str

    def __str__(self) -> str:
        return f"{self.__class__.__name__}(sender={self.sender}, host={self.host}, port={self.port}, use_ssl={self.use_ssl})"


@dataclass(slots=True)
class MockedEmailConfig:
    type = "MOCKED"

    sender: str
    tmpdir: str

    def __str__(self) -> str:
        return f"{self.__class__.__name__}(sender={self.sender}, tmpdir={self.tmpdir})"


EmailConfig = SmtpEmailConfig | MockedEmailConfig


@dataclass(slots=True)
class BackendConfig:
    administration_token: str

    db_url: str
    db_min_connections: int
    db_max_connections: int
    sse_keepalive: float  # Set to `math.inf` if disabled

    blockstore_config: BaseBlockStoreConfig

    email_config: SmtpEmailConfig | MockedEmailConfig
    forward_proto_enforce_https: tuple[str, str] | None
    backend_addr: BackendAddr | None

    debug: bool

    organization_bootstrap_webhook_url: str | None = None
    organization_spontaneous_bootstrap: bool = False
    organization_initial_active_users_limit: ActiveUsersLimit = field(
        default_factory=lambda: ActiveUsersLimit.NO_LIMIT
    )
    organization_initial_user_profile_outsider_allowed: bool = True

    # Number of SSE events kept in memory to allow client to catch up on reconnection
    sse_events_cache_size: int = 1024

    @property
    def db_type(self) -> str:
        if self.db_url.upper() == "MOCKED":
            return "MOCKED"
        else:
            return "POSTGRESQL"

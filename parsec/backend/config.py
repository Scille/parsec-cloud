# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import attr
from typing import List, Tuple, Union

from parsec._parsec import BackendAddr


class BaseBlockStoreConfig:
    # Overloaded by children
    @property
    def type(self) -> str:
        raise NotImplementedError


@attr.s(frozen=True, auto_attribs=True)
class RAID0BlockStoreConfig(BaseBlockStoreConfig):
    type = "RAID0"

    blockstores: List[BaseBlockStoreConfig]


@attr.s(frozen=True, auto_attribs=True)
class RAID1BlockStoreConfig(BaseBlockStoreConfig):
    type = "RAID1"

    blockstores: List[BaseBlockStoreConfig]
    partial_create_ok: bool = False


@attr.s(frozen=True, auto_attribs=True)
class RAID5BlockStoreConfig(BaseBlockStoreConfig):
    type = "RAID5"

    blockstores: List[BaseBlockStoreConfig]
    partial_create_ok: bool = False


@attr.s(frozen=True, auto_attribs=True)
class S3BlockStoreConfig(BaseBlockStoreConfig):
    type = "S3"

    s3_endpoint_url: str | None
    s3_region: str
    s3_bucket: str
    s3_key: str
    s3_secret: str


@attr.s(frozen=True, auto_attribs=True)
class SWIFTBlockStoreConfig(BaseBlockStoreConfig):
    type = "SWIFT"

    swift_authurl: str
    swift_tenant: str
    swift_container: str
    swift_user: str
    swift_password: str


@attr.s(frozen=True, auto_attribs=True)
class PostgreSQLBlockStoreConfig(BaseBlockStoreConfig):
    type = "POSTGRESQL"


@attr.s(frozen=True, auto_attribs=True)
class MockedBlockStoreConfig(BaseBlockStoreConfig):
    type = "MOCKED"


@attr.s(slots=True, frozen=True, auto_attribs=True)
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


@attr.s(slots=True, frozen=True, auto_attribs=True)
class MockedEmailConfig:
    type = "MOCKED"

    sender: str
    tmpdir: str

    def __str__(self) -> str:
        return f"{self.__class__.__name__}(sender={self.sender}, tmpdir={self.tmpdir})"


EmailConfig = Union[SmtpEmailConfig, MockedEmailConfig]


@attr.s(slots=True, frozen=True, auto_attribs=True)
class BackendConfig:
    administration_token: str

    db_url: str
    db_min_connections: int
    db_max_connections: int

    blockstore_config: BaseBlockStoreConfig

    email_config: Union[SmtpEmailConfig, MockedEmailConfig]
    forward_proto_enforce_https: Tuple[str, str] | None
    backend_addr: BackendAddr | None

    debug: bool

    organization_bootstrap_webhook_url: str | None = None
    organization_spontaneous_bootstrap: bool = False
    organization_initial_active_users_limit: int | None = None
    organization_initial_user_profile_outsider_allowed: bool = True

    @property
    def db_type(self) -> str:
        if self.db_url.upper() == "MOCKED":
            return "MOCKED"
        else:
            return "POSTGRESQL"

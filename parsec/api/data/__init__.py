# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.api.data.base import (
    DataError,
    BaseSignedDataSchema,
    SignedDataMeta,
    BaseData,
    BaseSchema,
)
from parsec.api.data.certif import (
    UserCertificateContent,
    DeviceCertificateContent,
    RevokedDeviceCertificateContent,
    RealmRoleCertificateContent,
)
from parsec.api.data.message import (
    MessageContent,
    SharingGrantedMessageContent,
    SharingReencryptedMessageContent,
    SharingRevokedMessageContent,
    PingMessageContent,
)
from parsec.api.data.manifest import (
    BlockAccess,
    WorkspaceEntry,
    UserManifest,
    WorkspaceManifest,
    FolderManifest,
    FileManifest,
)


__api_data_version__ = (1, 0)


__all__ = (
    "__api_data_version__",
    "DataError",
    "BaseSignedDataSchema",
    "SignedDataMeta",
    "BaseData",
    "BaseSchema",
    # Certifs
    "UserCertificateContent",
    "DeviceCertificateContent",
    "RevokedDeviceCertificateContent",
    "RealmRoleCertificateContent",
    # Messages
    "MessageContent",
    "SharingGrantedMessageContent",
    "SharingReencryptedMessageContent",
    "SharingRevokedMessageContent",
    "PingMessageContent",
    # Manifests
    "BlockAccess",
    "WorkspaceEntry",
    "UserManifest",
    "WorkspaceManifest",
    "FolderManifest",
    "FileManifest",
)

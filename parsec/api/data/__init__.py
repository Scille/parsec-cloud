# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.api.data.base import (
    DataError,
    BaseData,
    BaseAPIData,
    BaseSchema,
    BaseSignedData,
    BaseAPISignedData,
    BaseSignedDataSchema,
    BaseLocalData,
)
from parsec.api.data.certif import (
    UserCertificateContent,
    DeviceCertificateContent,
    RevokedDeviceCertificateContent,
    RealmRoleCertificateContent,
)
from parsec.api.data.invite_claim import (
    UserClaimContent,
    DeviceClaimContent,
    DeviceClaimAnswerContent,
)
from parsec.api.data.message import (
    MessageContent,
    SharingGrantedMessageContent,
    SharingReencryptedMessageContent,
    SharingRevokedMessageContent,
    PingMessageContent,
)
from parsec.api.data.manifest import (
    BlockID,
    BlockIDField,
    BlockAccess,
    WorkspaceEntry,
    Manifest,
    UserManifest,
    WorkspaceManifest,
    FolderManifest,
    FileManifest,
)


__api_data_version__ = (1, 0)


__all__ = (
    "__api_data_version__",
    # Base
    "DataError",
    "BaseData",
    "BaseAPIData",
    "BaseSchema",
    "BaseSignedData",
    "BaseAPISignedData",
    "BaseSignedDataSchema",
    "BaseLocalData",
    # Certifs
    "UserCertificateContent",
    "DeviceCertificateContent",
    "RevokedDeviceCertificateContent",
    "RealmRoleCertificateContent",
    # Invite&Claim
    "UserClaimContent",
    "DeviceClaimContent",
    "DeviceClaimAnswerContent",
    # Messages
    "MessageContent",
    "SharingGrantedMessageContent",
    "SharingReencryptedMessageContent",
    "SharingRevokedMessageContent",
    "PingMessageContent",
    # Manifests
    "BlockID",
    "BlockIDField",
    "BlockAccess",
    "WorkspaceEntry",
    "Manifest",
    "UserManifest",
    "WorkspaceManifest",
    "FolderManifest",
    "FileManifest",
)

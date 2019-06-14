# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class FSError(Exception):
    pass


class FSValidationError(FSError):
    pass


class FSPackingError(FSError):
    pass


class FSWorkspaceNotFoundError(FSError):
    pass


class FSBackendOfflineError(FSError):
    pass


class FSSharingNotAllowedError(FSError):
    pass


class FSRemoteSyncError(FSError):
    pass


class FSRemoteManifestNotFound(FSError):
    pass


class FSNoSynchronizationRequired(FSError):
    pass


class FSWorkspaceNoAccess(FSError):
    pass


class FSWorkspaceInMaintenance(FSError):
    pass


class FSWorkspaceNotInMaintenance(FSError):
    pass


class FSMaintenanceNotAllowedError(FSError):
    pass

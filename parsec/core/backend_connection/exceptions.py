# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class BackendConnectionError(Exception):
    pass


class BackendNotAvailable(BackendConnectionError):
    pass


class BackendIncompatibleVersion(BackendNotAvailable):
    pass


class BackendNotAvailableRVKMismatch(BackendNotAvailable):
    pass


class BackendHandshakeError(BackendConnectionError):
    pass


class BackendHandshakeAPIVersionError(BackendHandshakeError):
    pass


class BackendHandshakeRVKMismatchError(BackendHandshakeError):
    pass


class BackendDeviceRevokedError(BackendHandshakeError):
    pass


class BackendCmdsInvalidRequest(BackendConnectionError):
    pass


class BackendCmdsInvalidResponse(BackendConnectionError):
    pass


class BackendCmdsBadResponse(BackendConnectionError):
    def __repr__(self):
        return f"<{type(self).__name__}({self.status})>"

    def __str__(self):
        return f"Backend error `{self.status}`: {self.reason}"

    @property
    def status(self):
        return self.args[0]["status"]

    @property
    def reason(self):
        try:
            return self.args[0]["reason"]
        except KeyError:
            return f"Error code `{self.status}` returned"


class BackendCmdsError(BackendCmdsBadResponse):
    status = "error"


class BackendCmdsAlreadyExists(BackendCmdsBadResponse):
    status = "already_exists"


class BackendCmdsRoleAlreadyGranted(BackendCmdsBadResponse):
    status = "already_granted"


class BackendCmdsNotAllowed(BackendCmdsBadResponse):
    status = "not_allowed"


class BackendCmdsBadVersion(BackendCmdsBadResponse):
    status = "bad_version"


class BackendCmdsBadTimestamp(BackendCmdsBadResponse):
    status = "bad_timestamp"


class BackendCmdsNotFound(BackendCmdsBadResponse):
    status = "not_found"


class BackendCmdsDenied(BackendCmdsBadResponse):
    status = "denied"


class BackendCmdsBadUserId(BackendCmdsBadResponse):
    status = "bad_user_id"


class BackendCmdsInvalidCertification(BackendCmdsBadResponse):
    status = "invalid_certification"


class BackendCmdsInvalidData(BackendCmdsBadResponse):
    status = "invalid_data"


class BackendCmdsAlreadyBootstrapped(BackendCmdsBadResponse):
    status = "already_bootstrapped"


class BackendCmdsCancelled(BackendCmdsBadResponse):
    status = "cancelled"


class BackendCmdsNoEvents(BackendCmdsBadResponse):
    status = "no_events"


class BackendCmdsTimeout(BackendCmdsBadResponse):
    status = "timeout"


class BackendCmdsBadMessage(BackendCmdsBadResponse):
    status = "bad_message"


class BackendCmdsBadEncryptionRevision(BackendCmdsBadResponse):
    status = "bad_encryption_revision"


class BackendCmdsInMaintenance(BackendCmdsBadResponse):
    status = "in_maintenance"


class BackendCmdsMaintenanceError(BackendCmdsBadResponse):
    status = "maintenance_error"


class BackendCmdsParticipantsMismatchError(BackendCmdsBadResponse):
    status = "participants_mismatch"


STATUS_TO_EXC_CLS = {
    exc_cls.status: exc_cls
    for exc_cls in (
        BackendCmdsBadResponse,
        BackendCmdsAlreadyExists,
        BackendCmdsRoleAlreadyGranted,
        BackendCmdsNotAllowed,
        BackendCmdsBadVersion,
        BackendCmdsNotFound,
        BackendCmdsDenied,
        BackendCmdsBadUserId,
        BackendCmdsInvalidCertification,
        BackendCmdsInvalidData,
        BackendCmdsAlreadyBootstrapped,
        BackendCmdsCancelled,
        BackendCmdsNoEvents,
        BackendCmdsTimeout,
        BackendCmdsBadMessage,
        BackendCmdsInMaintenance,
        BackendCmdsMaintenanceError,
        BackendCmdsParticipantsMismatchError,
    )
}


def raise_on_bad_response(msg):
    status = msg["status"]
    if status != "ok":
        exc_cls = STATUS_TO_EXC_CLS.get(status, BackendCmdsBadResponse)
        raise exc_cls(msg)

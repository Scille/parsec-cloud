# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from enum import IntEnum


class AdvisoryLock(IntEnum):
    """
    Advisory lock must be taken for certain operations to avoid concurrency issue.

    - Invitation creation: Only one active invitation is allowed per email, this is something
      that cannot be enforced purely in PostgreSQL with unique constraint (given previous
      invitations merely got a deleted flag set).
    - PKI enrollment creation: Only one active PKI enrollment is allowed per x509
      certificate, this is something that cannot be enforced in PostgreSQL with unique
      constraint (given previous PKI enrollment are kept with just an update on their
      state field from `submitted` to `accepted/rejected/cancelled`).

    Note this enum is an `IntEnum`, as the integer value will be used as main discriminant
    key when using `PG_ADVISORY_XACT_LOCK` in SQL query. Hence each advisory lock must
    have a different integer value!
    """

    InvitationCreation = 66
    PKIEnrollmentCreation = 67
    AsyncEnrollmentCreation = 68

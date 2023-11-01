# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import DateTime, EnrollmentID

class PkiEnrollmentListItem:
    def __init__(
        self,
        enrollment_id: EnrollmentID,
        submit_payload: bytes,
        submit_payload_signature: bytes,
        submitted_on: DateTime,
        submitter_der_x509_certificate: bytes,
    ) -> None: ...
    @property
    def enrollment_id(self) -> EnrollmentID: ...
    @property
    def submit_payload(self) -> bytes: ...
    @property
    def submit_payload_signature(self) -> bytes: ...
    @property
    def submitted_on(self) -> DateTime: ...
    @property
    def submitter_der_x509_certificate(self) -> bytes: ...

class Req:
    def __init__(
        self,
    ) -> None: ...
    def dump(self) -> bytes: ...

class Rep:
    @staticmethod
    def load(raw: bytes) -> Rep: ...
    def dump(self) -> bytes: ...

class RepUnknownStatus(Rep):
    def __init__(self, status: str, reason: str | None) -> None: ...
    @property
    def status(self) -> str: ...
    @property
    def reason(self) -> str | None: ...

class RepOk(Rep):
    def __init__(self, enrollments: list[PkiEnrollmentListItem]) -> None: ...
    @property
    def enrollments(self) -> list[PkiEnrollmentListItem]: ...

class RepAuthorNotAllowed(Rep):
    def __init__(
        self,
    ) -> None: ...

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Iterable, List, Optional, Dict
from pathlib import Path
from uuid import UUID
from pendulum import DateTime
import attr
from parsec.core.types.backend_address import (
    BackendPkiEnrollmentAddr,
    BackendPkiEnrollmentAddrField,
)
from parsec.core.types.base import BaseLocalData

from parsec.serde import BaseSchema, fields, post_load
from parsec.api.data import DataError
from parsec.api.data import PkiEnrollmentSubmitPayload
from parsec.core.local_device import (
    MsgpackSerializer,
    LocalDeviceValidationError,
    LocalDeviceError,
    LocalDevicePackingError,
)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class X509Certificate:
    class SCHEMA_CLS(BaseSchema):
        type = fields.CheckedConstant("x509_certifcate", required=True)
        issuer = fields.Dict(required=True)
        subject = fields.Dict(required=True)
        der_x509_certificate = fields.Bytes(required=True)
        certificate_sha1 = fields.Bytes(required=True)
        certificate_id = fields.String(required=True)

        @post_load
        def make_obj(self, data):
            data.pop("type", None)
            return X509Certificate(**data)

    issuer: Dict[str, str]
    subject: Dict[str, str]
    der_x509_certificate: bytes
    certificate_sha1: bytes
    certificate_id: str

    @property
    def subject_common_name(self) -> Optional[str]:
        return self.subject.get("common_name")

    @property
    def subject_email_address(self) -> Optional[str]:
        return self.subject.get("email_address")

    @property
    def issuer_common_name(self) -> Optional[str]:
        return self.issuer.get("common_name")


class PendingDeviceKeys(BaseSchema):
    type = fields.CheckedConstant("requested_device_keys", required=True)
    private_key = fields.PrivateKey(required=True)
    signing_key = fields.SigningKey(required=True)


pending_device_keys_serializer = MsgpackSerializer(
    PendingDeviceKeys,
    validation_exc=LocalDeviceValidationError,
    packing_exc=LocalDevicePackingError,
)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class LocalPendingEnrollment(BaseLocalData):

    DIRECTORY_NAME = "enrollment_requests"

    class SCHEMA_CLS(BaseSchema):
        type = fields.CheckedConstant("local_pending_enrollment", required=True)
        x509_certificate = fields.Nested(X509Certificate.SCHEMA_CLS, required=True)
        addr = BackendPkiEnrollmentAddrField(required=True)
        submitted_on = fields.DateTime(required=True)
        enrollment_id = fields.UUID()
        submit_payload = fields.Nested(PkiEnrollmentSubmitPayload.SCHEMA_CLS, required=True)
        encrypted_key = fields.Bytes(required=True)
        ciphertext = fields.Bytes(required=True)  # An encrypted PendingDeviceKeys

        @post_load
        def make_obj(self, data):
            data.pop("type", None)
            return LocalPendingEnrollment(**data)

    x509_certificate: X509Certificate
    addr: BackendPkiEnrollmentAddr
    submitted_on: DateTime
    enrollment_id: UUID
    submit_payload: PkiEnrollmentSubmitPayload
    encrypted_key: bytes
    ciphertext: bytes

    def get_path(self, config_dir: Path) -> Path:
        return config_dir / self.DIRECTORY_NAME / self.enrollment_id.hex

    def save(self, config_dir: Path) -> Path:
        path = self.get_path(config_dir)
        try:
            path.parent.mkdir(mode=0o700, exist_ok=True, parents=True)
            path.write_bytes(self.dump())
        except OSError as exc:
            raise LocalDeviceError(f"Cannot save {path}: {exc}") from exc
        return path

    @classmethod
    def iter_path(cls, config_dir: Path) -> Iterable[Path]:
        parent = config_dir / cls.DIRECTORY_NAME
        if not parent.exists():
            return
        for path in parent.iterdir():
            yield path

    @classmethod
    def load_from_path(cls, path: Path):
        data = path.read_bytes()
        try:
            return cls.load(data)
        except DataError as exc:
            raise LocalDeviceValidationError(
                f"Cannot load local enrollment request: {exc}"
            ) from exc

    @classmethod
    def load_from_enrollment_id(
        cls, config_dir: Path, enrollment_id: UUID
    ) -> "LocalPendingEnrollment":
        data = (config_dir / cls.DIRECTORY_NAME / enrollment_id.hex).read_bytes()
        try:
            return cls.load(data)
        except DataError as exc:
            raise LocalDeviceValidationError(
                f"Cannot load local enrollment request: {exc}"
            ) from exc

    @classmethod
    def list(cls, config_dir: Path) -> List["LocalPendingEnrollment"]:
        result = []
        for path in cls.iter_path(config_dir):
            try:
                result.append(cls.load(path.read_bytes()))
            # Ignore invalid or missing files
            except (DataError, OSError) as e:
                print(repr(e))
                pass
        return result

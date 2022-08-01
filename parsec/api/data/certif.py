# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec._parsec import DateTime
from typing import Any, Dict, Literal

from parsec.sequester_crypto import SequesterVerifyKeyDer, SequesterEncryptionKeyDer
from parsec.serde import fields, post_load
from parsec.api.protocol import (
    SequesterServiceID,
    SequesterServiceIDField,
)
from parsec.api.data.base import (
    BaseAPIData,
    BaseAPISignedData,
    BaseSignedDataSchema,
)
import attr

from parsec.serde.schema import BaseSchema

@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class SequesterAuthorityCertificate(BaseAPISignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("sequester_authority_certificate", required=True)
        # Override author field to always uses None given this certificate can only be signed by the root key
        author = fields.CheckedConstant(
            None, required=True, allow_none=True
        )  # Constant None fields required to be allowed to be None !
        verify_key_der = fields.SequesterVerifyKeyDerField(required=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "SequesterAuthorityCertificate":
            data.pop("type")
            return SequesterAuthorityCertificate(**data)

    # Override author field to always uses None given this certificate can only be signed by the root key
    author: Literal[None]  # type: ignore[assignment]
    verify_key_der: SequesterVerifyKeyDer


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class SequesterServiceCertificate(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        # No author field here given we are signed by the sequester authority
        type = fields.CheckedConstant("sequester_service_certificate", required=True)
        timestamp = fields.DateTime(required=True)
        service_id = SequesterServiceIDField(required=True)
        service_label = fields.String(required=True)
        encryption_key_der = fields.SequesterEncryptionKeyDerField(required=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "SequesterServiceCertificate":
            data.pop("type")
            return SequesterServiceCertificate(**data)

    timestamp: DateTime
    service_id: SequesterServiceID
    service_label: str
    encryption_key_der: SequesterEncryptionKeyDer

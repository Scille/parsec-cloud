# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import re
from typing import Optional, Tuple, List, Dict, Any, TYPE_CHECKING
from random import randint, shuffle

from parsec.crypto import VerifyKey, PublicKey, PrivateKey, SecretKey
from parsec.serde import fields, post_load
from parsec.api.protocol import (
    DeviceID,
    DeviceIDField,
    HumanHandle,
    HumanHandleField,
    DeviceLabel,
    DeviceLabelField,
    StrBased,
)
from parsec.api.data.base import BaseAPIData, BaseSchema
from parsec.api.data.entry import EntryID, EntryIDField
from parsec.api.data.certif import UserProfile, UserProfileField
import attr


class SASCode(StrBased):
    MAX_BYTE_SIZE = 4
    SYMBOLS = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789"
    REGEX = re.compile(rf"^[{SYMBOLS}]{{{MAX_BYTE_SIZE}}}$")

    @classmethod
    def from_int(cls, num: int) -> "SASCode":
        if num < 0:
            raise ValueError("Provided integer is negative")
        result = ""
        for _ in range(cls.MAX_BYTE_SIZE):
            result += cls.SYMBOLS[num % len(cls.SYMBOLS)]
            num //= len(cls.SYMBOLS)
        if num != 0:
            raise ValueError("Provided integer is too large")
        return cls(result)


_PySASCode = SASCode
if not TYPE_CHECKING:
    try:
        from libparsec.types import SASCode as _RsSASCode
    except:
        pass
    else:
        SASCode = _RsSASCode


def generate_sas_codes(
    claimer_nonce: bytes, greeter_nonce: bytes, shared_secret_key: SecretKey
) -> Tuple[SASCode, SASCode]:
    # Computes combined HMAC
    combined_nonce = claimer_nonce + greeter_nonce
    # Digest size of 5 bytes so we can split it beween two 20bits SAS
    combined_hmac = shared_secret_key.hmac(combined_nonce, digest_size=5)

    hmac_as_int = int.from_bytes(combined_hmac, "big")
    # Big endian number extracted from bits [0, 20[
    claimer_sas = hmac_as_int % 2 ** 20
    # Big endian number extracted from bits [20, 40[
    greeter_sas = (hmac_as_int >> 20) % 2 ** 20

    return SASCode.from_int(claimer_sas), SASCode.from_int(greeter_sas)


_Py_generate_sas_codes = generate_sas_codes
if not TYPE_CHECKING:
    try:
        from libparsec.types import generate_sas_codes as _Rs_generate_sas_codes
    except:
        pass
    else:
        generate_sas_codes = _Rs_generate_sas_codes


def generate_sas_code_candidates(valid_sas: SASCode, size: int) -> List[SASCode]:
    candidates = {valid_sas}
    while len(candidates) < size:
        candidates.add(SASCode.from_int(randint(0, 2 ** 20 - 1)))
    ordered_candidates = list(candidates)
    shuffle(ordered_candidates)
    return ordered_candidates


_Py_generate_sas_code_candidates = generate_sas_code_candidates
if not TYPE_CHECKING:
    try:
        from libparsec.types import generate_sas_code_candidates as _Rs_generate_sas_code_candidates
    except:
        pass
    else:
        generate_sas_code_candidates = _Rs_generate_sas_code_candidates


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class InviteUserData(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        type = fields.CheckedConstant("invite_user_data", required=True)
        # Claimer ask for device_label/human_handle, but greeter has final word on this
        requested_device_label = DeviceLabelField(required=True, allow_none=True)
        requested_human_handle = HumanHandleField(required=True, allow_none=True)
        # Note claiming user also imply creating a first device
        public_key = fields.PublicKey(required=True)
        verify_key = fields.VerifyKey(required=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "InviteUserData":  # type: ignore[misc]

            data.pop("type")
            return InviteUserData(**data)

    requested_device_label: Optional[DeviceLabel]
    requested_human_handle: Optional[HumanHandle]
    public_key: PublicKey
    verify_key: VerifyKey


_PyInviteUserData = InviteUserData
if not TYPE_CHECKING:
    try:
        from libparsec.types import InviteUserData as _RsInviteUserData
    except:
        pass
    else:
        InviteUserData = _RsInviteUserData


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class InviteUserConfirmation(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        type = fields.CheckedConstant("invite_user_confirmation", required=True)
        device_id = DeviceIDField(required=True)
        device_label = DeviceLabelField(required=True, allow_none=True)
        human_handle = HumanHandleField(required=True, allow_none=True)
        profile = UserProfileField(required=True)
        root_verify_key = fields.VerifyKey(required=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "InviteUserConfirmation":  # type: ignore[misc]
            data.pop("type")
            return InviteUserConfirmation(**data)

    device_id: DeviceID
    device_label: Optional[DeviceLabel]
    human_handle: Optional[HumanHandle]
    profile: UserProfile
    root_verify_key: VerifyKey


_PyInviteUserConfirmation = InviteUserConfirmation
if not TYPE_CHECKING:
    try:
        from libparsec.types import InviteUserConfirmation as _RsInviteUserConfirmation
    except:
        pass
    else:
        InviteUserConfirmation = _RsInviteUserConfirmation


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class InviteDeviceData(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        type = fields.CheckedConstant("invite_device_data", required=True)
        # Claimer ask for device_label, but greeter has final word on this
        requested_device_label = DeviceLabelField(required=True, allow_none=True)
        verify_key = fields.VerifyKey(required=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "InviteDeviceData":  # type: ignore[misc]
            data.pop("type")
            return InviteDeviceData(**data)

    requested_device_label: Optional[DeviceLabel]
    verify_key: VerifyKey


_PyInviteDeviceData = InviteDeviceData
if not TYPE_CHECKING:
    try:
        from libparsec.types import InviteDeviceData as _RsInviteDeviceData
    except:
        pass
    else:
        InviteDeviceData = _RsInviteDeviceData


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class InviteDeviceConfirmation(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        type = fields.CheckedConstant("invite_device_confirmation", required=True)
        device_id = DeviceIDField(required=True)
        device_label = DeviceLabelField(required=True, allow_none=True)
        human_handle = HumanHandleField(required=True, allow_none=True)
        profile = UserProfileField(required=True)
        private_key = fields.PrivateKey(required=True)
        user_manifest_id = EntryIDField(required=True)
        user_manifest_key = fields.SecretKey(required=True)
        root_verify_key = fields.VerifyKey(required=True)

        @post_load
        def make_obj(  # type: ignore[misc]
            self, data: Dict[str, Any]
        ) -> "InviteDeviceConfirmation":
            data.pop("type")
            return InviteDeviceConfirmation(**data)

    device_id: DeviceID
    device_label: Optional[DeviceLabel]
    human_handle: Optional[HumanHandle]
    profile: UserProfile
    private_key: PrivateKey
    user_manifest_id: EntryID
    user_manifest_key: SecretKey
    root_verify_key: VerifyKey


_PyInviteDeviceConfirmation = InviteDeviceConfirmation
if not TYPE_CHECKING:
    try:
        from libparsec.types import InviteDeviceConfirmation as _RsInviteDeviceConfirmation
    except:
        pass
    else:
        InviteDeviceConfirmation = _RsInviteDeviceConfirmation

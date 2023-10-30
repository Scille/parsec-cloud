# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from .common import Ref


def validate_entry_name(raw: Ref[str]) -> bool:
    raise NotImplementedError()


def validate_path(raw: Ref[str]) -> bool:
    raise NotImplementedError()


def validate_human_handle_label(raw: Ref[str]) -> bool:
    raise NotImplementedError()


def validate_email(raw: Ref[str]) -> bool:
    raise NotImplementedError()


def validate_device_label(raw: Ref[str]) -> bool:
    raise NotImplementedError()


def validate_invitation_token(raw: Ref[str]) -> bool:
    raise NotImplementedError()


def validate_organization_id(raw: Ref[str]) -> bool:
    raise NotImplementedError()

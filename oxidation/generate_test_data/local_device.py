# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from uuid import UUID
from protocol.utils import *

from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *
from parsec.core.types import *
from parsec.core.local_device import *

ld = KEY.encrypt(ALICE.dump())
display("local device (ADMIN)", ld, [KEY])

ld_as_standard = LocalDevice(
    organization_addr=ALICE.organization_addr,
    device_id=ALICE.device_id,
    device_label=ALICE.device_label,
    human_handle=ALICE.human_handle,
    signing_key=ALICE.signing_key,
    private_key=ALICE.private_key,
    profile=UserProfile.STANDARD,
    user_manifest_id=ALICE.user_manifest_id,
    user_manifest_key=ALICE.user_manifest_key,
    local_symkey=ALICE.local_symkey,
)
ld_as_standard = KEY.encrypt(ld_as_standard.dump())
display("local device (STANDARD)", ld_as_standard, [KEY])

ld_as_outsider = LocalDevice(
    organization_addr=ALICE.organization_addr,
    device_id=ALICE.device_id,
    device_label=ALICE.device_label,
    human_handle=ALICE.human_handle,
    signing_key=ALICE.signing_key,
    private_key=ALICE.private_key,
    profile=UserProfile.OUTSIDER,
    user_manifest_id=ALICE.user_manifest_id,
    user_manifest_key=ALICE.user_manifest_key,
    local_symkey=ALICE.local_symkey,
)
ld_as_outsider = KEY.encrypt(ld_as_outsider.dump())
display("local device (OUTSIDER)", ld_as_outsider, [KEY])

ld_no_device_label_no_human_handle = LocalDevice(
    organization_addr=ALICE.organization_addr,
    device_id=ALICE.device_id,
    device_label=None,
    human_handle=None,
    signing_key=ALICE.signing_key,
    private_key=ALICE.private_key,
    profile=ALICE.profile,
    user_manifest_id=ALICE.user_manifest_id,
    user_manifest_key=ALICE.user_manifest_key,
    local_symkey=ALICE.local_symkey,
)
ld_no_device_label_no_human_handle = KEY.encrypt(ld_no_device_label_no_human_handle.dump())
display("local device no device_label/human_handle", ld_no_device_label_no_human_handle, [KEY])

ld_legagacy_admin = {
    "organization_addr": ALICE.organization_addr.to_url(),
    "device_id": str(ALICE.device_id),
    "signing_key": ALICE.signing_key.encode(),
    "private_key": ALICE.private_key.encode(),
    "is_admin": True,
    "user_manifest_id": UUID(ALICE.user_manifest_id.hex),
    "user_manifest_key": bytes(ALICE.user_manifest_key.secret),
    "local_symkey": bytes(ALICE.local_symkey.secret),
}
ld_legagacy_admin = KEY.encrypt(packb(ld_legagacy_admin))
display("local device legacy is_admin_field (ADMIN)", ld_legagacy_admin, [KEY])

ld_legagacy_not_admin = {
    "organization_addr": ALICE.organization_addr.to_url(),
    "device_id": str(ALICE.device_id),
    "signing_key": ALICE.signing_key.encode(),
    "private_key": ALICE.private_key.encode(),
    "is_admin": False,
    "user_manifest_id": UUID(ALICE.user_manifest_id.hex),
    "user_manifest_key": bytes(ALICE.user_manifest_key.secret),
    "local_symkey": bytes(ALICE.local_symkey.secret),
}
ld_legagacy_not_admin = KEY.encrypt(packb(ld_legagacy_not_admin))
display("local device legacy is_admin_field (STANARD)", ld_legagacy_not_admin, [KEY])

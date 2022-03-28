# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
# flake8: noqa

from protocol.utils import *

from parsec.api.protocol import *
from parsec.api.data import *


uc = UserCertificateContent(
    author=ALICE.device_id,
    timestamp=NOW,
    user_id=BOB.user_id,
    human_handle=BOB.human_handle,
    public_key=BOB.public_key,
    profile=UserProfile.STANDARD,
).dump_and_sign(ALICE.signing_key)
display("user certificate", uc, [ALICE.verify_key, "zip"])

redacted_uc = UserCertificateContent(
    author=ALICE.device_id,
    timestamp=NOW,
    user_id=BOB.user_id,
    human_handle=None,
    public_key=BOB.public_key,
    profile=UserProfile.STANDARD,
).dump_and_sign(ALICE.signing_key)
display("redacted user certificate", redacted_uc, [ALICE.verify_key, "zip"])

raw_legacy_certif = {
    "type": "user_certificate",
    "author": str(ALICE.device_id),
    "timestamp": NOW,
    "user_id": str(BOB.user_id),
    "public_key": BOB.public_key.encode(),
    "is_admin": True,
}
uc_old = ALICE.signing_key.sign(zlib.compress(packb(raw_legacy_certif)))
UserCertificateContent.verify_and_load(
    uc_old,
    author_verify_key=ALICE.verify_key,
    expected_author=ALICE.device_id,
    expected_user=BOB.user_id,
    expected_human_handle=None,
)
display(
    "user certificate legacy is_admin field and no human_handle_field",
    uc_old,
    [ALICE.verify_key, "zip"],
)

dc = DeviceCertificateContent(
    author=ALICE.device_id,
    timestamp=NOW,
    device_id=BOB.device_id,
    device_label=BOB.device_label,
    verify_key=BOB.verify_key,
).dump_and_sign(ALICE.signing_key)
display("device certificate", dc, [ALICE.verify_key, "zip"])

redacted_dc = DeviceCertificateContent(
    author=ALICE.device_id,
    timestamp=NOW,
    device_id=BOB.device_id,
    device_label=None,
    verify_key=BOB.verify_key,
).dump_and_sign(ALICE.signing_key)
display("redacted device certificate", redacted_dc, [ALICE.verify_key, "zip"])

raw_legacy_certif = {
    "type": "device_certificate",
    "author": str(ALICE.device_id),
    "timestamp": NOW,
    "device_id": str(BOB.device_id),
    "verify_key": BOB.verify_key.encode(),
}
dc_old = ALICE.signing_key.sign(zlib.compress(packb(raw_legacy_certif)))
DeviceCertificateContent.verify_and_load(
    dc_old,
    author_verify_key=ALICE.verify_key,
    expected_author=ALICE.device_id,
    expected_device=BOB.device_id,
)
display("device certificate legacy no device_label field", dc_old, [ALICE.verify_key, "zip"])

ruc = RevokedUserCertificateContent(
    author=ALICE.device_id, timestamp=NOW, user_id=BOB.user_id
).dump_and_sign(ALICE.signing_key)
display("revoked user certificate", ruc, [ALICE.verify_key, "zip"])

rrc = RealmRoleCertificateContent(
    author=ALICE.device_id,
    timestamp=NOW,
    realm_id=RealmID.from_hex("4486e7cf02d747bd9126679ba58e0474"),
    user_id=BOB.user_id,
    role=RealmRole.OWNER,
).dump_and_sign(ALICE.signing_key)
display("realm role certificate", rrc, [ALICE.verify_key, "zip"])

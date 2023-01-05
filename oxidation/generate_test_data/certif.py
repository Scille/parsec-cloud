# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from protocol.utils import *

from parsec._parsec import SequesterVerifyKeyDer, SequesterPublicKeyDer
from parsec.api.protocol import *
from parsec.api.data import *


uc = UserCertificate(
    author=ALICE.device_id,
    timestamp=NOW,
    user_id=BOB.user_id,
    human_handle=BOB.human_handle,
    public_key=BOB.public_key,
    profile=UserProfile.STANDARD,
).dump_and_sign(ALICE.signing_key)
display("user certificate", uc, [ALICE.verify_key, "zip"])

redacted_uc = UserCertificate(
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
    "author": ALICE.device_id.str,
    "timestamp": NOW,
    "user_id": BOB.user_id.str,
    "public_key": BOB.public_key.encode(),
    "is_admin": True,
}
uc_old = ALICE.signing_key.sign(zlib.compress(packb(raw_legacy_certif)))
UserCertificate.verify_and_load(
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

dc = DeviceCertificate(
    author=ALICE.device_id,
    timestamp=NOW,
    device_id=BOB.device_id,
    device_label=BOB.device_label,
    verify_key=BOB.verify_key,
).dump_and_sign(ALICE.signing_key)
display("device certificate", dc, [ALICE.verify_key, "zip"])

redacted_dc = DeviceCertificate(
    author=ALICE.device_id,
    timestamp=NOW,
    device_id=BOB.device_id,
    device_label=None,
    verify_key=BOB.verify_key,
).dump_and_sign(ALICE.signing_key)
display("redacted device certificate", redacted_dc, [ALICE.verify_key, "zip"])

raw_legacy_certif = {
    "type": "device_certificate",
    "author": ALICE.device_id.str,
    "timestamp": NOW,
    "device_id": BOB.device_id.str,
    "verify_key": BOB.verify_key.encode(),
}
dc_old = ALICE.signing_key.sign(zlib.compress(packb(raw_legacy_certif)))
DeviceCertificate.verify_and_load(
    dc_old,
    author_verify_key=ALICE.verify_key,
    expected_author=ALICE.device_id,
    expected_device=BOB.device_id,
)
display("device certificate legacy no device_label field", dc_old, [ALICE.verify_key, "zip"])

ruc = RevokedUserCertificate(
    author=ALICE.device_id, timestamp=NOW, user_id=BOB.user_id
).dump_and_sign(ALICE.signing_key)
display("revoked user certificate", ruc, [ALICE.verify_key, "zip"])

rrc = RealmRoleCertificate(
    author=ALICE.device_id,
    timestamp=NOW,
    realm_id=RealmID.from_hex("4486e7cf02d747bd9126679ba58e0474"),
    user_id=BOB.user_id,
    role=RealmRole.OWNER,
).dump_and_sign(ALICE.signing_key)
display("realm role certificate", rrc, [ALICE.verify_key, "zip"])

sac = SequesterAuthorityCertificate(
    timestamp=NOW,
    verify_key_der=SequesterVerifyKeyDer(
        bytes.fromhex(
            "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689b069f3"
            "f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045d75b584c"
            "1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b"
            "087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207ebfd020301"
            "0001"
        )
    ),
).dump_and_sign(ALICE.signing_key)
display("sequester authority certificate", sac, [ALICE.verify_key, "zip"])

ssc = SequesterServiceCertificate(
    timestamp=NOW,
    encryption_key_der=SequesterPublicKeyDer(
        bytes.fromhex(
            "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689b069f3"
            "f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a3045d75b584c"
            "1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b"
            "087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207ebfd020301"
            "0001"
        )
    ),
    service_id=SequesterServiceID.from_hex("b5eb565343c442b3a26be44573813ff0"),
    service_label="foo",
).dump()
display("sequester service certificate", ssc, ["zip"])

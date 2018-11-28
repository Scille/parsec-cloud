from typing import Dict

from parsec.types import DeviceID
from parsec.crypto import (
    sign_and_add_meta,
    decode_signedmeta,
    unsecure_extract_msg_from_signed,
    SigningKey,
    CryptoError,
)


class TrustChainError(Exception):
    pass


class MissingSignatureError(TrustChainError):
    pass


def extract_signatory(payload: bytes) -> DeviceID:
    signatory_id, _ = decode_signedmeta(payload)
    return signatory_id


def verify_trust_chain(leaf: bytes, nodes: bytes):
    pass


ROOT_DEVICE_ID = DeviceID("root@root")

#     already_checked = set()
#     curr_signed_data
#      = leaf
#     while True:
#         curr_signature_author, curr_signed_data = decode_signedmeta(curr)
#         try:
#             curr = nodes[curr_signature_author]
#         except KeyError:
#             raise MissingSignatureError(f'Device ID {curr_signature_author} required but not provided.')


#
# device_manifest =
#   device_id
#   device_verify_key
#   timestamp
#
# signed_device_manifest =
#   authority_signing_key(device_manifest)
#   authority_id
#   backend-generated timestamp


def certify(device_id: DeviceID, device_signkey: SigningKey, payload: bytes) -> bytes:
    return sign_and_add_meta(device_id, device_signkey, payload)


def unsecure_extract_payload_from_certified_data(certified_payload: bytes):
    _, signed_data = decode_signedmeta(certified_payload)
    return unsecure_extract_msg_from_signed(signed_data)


__all__ = (
    "CryptoError",
    "TrustChainError",
    "MissingSignatureError",
    "verify_trust_chain",
    "certify",
)

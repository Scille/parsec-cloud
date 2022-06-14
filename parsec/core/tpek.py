# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from pathlib import Path
from uuid import uuid4
import pendulum
from parsec.core.types.backend_address import BackendTpekServiceAddr
from parsec.tpek_crypto import load_der_private_key, sign_tpek
from parsec.api.data.tpek import TpekDerServiceEncryptionKey
from parsec.core.backend_connection.anonymous import (
    tpek_register_service as cmd_tpek_register_service,
)


async def register_tpek_service(
    addr: BackendTpekServiceAddr, tpek_signing_key: Path, tpek_encryption_key, service_type
):
    tpek_der_signing_key = load_der_private_key(tpek_signing_key)
    tpek_der_encryption_private_key = load_der_private_key(tpek_encryption_key)
    tpek_der_encryption_public_key = tpek_der_encryption_private_key.public_key

    tpek_der_payload = TpekDerServiceEncryptionKey(
        encryption_key=tpek_der_encryption_public_key, timestamp=pendulum.now()
    ).dump()
    tpek_der_payload_signature = sign_tpek(tpek_der_signing_key, tpek_der_payload)

    service_id = uuid4()
    rep = await cmd_tpek_register_service(
        addr=addr,
        service_type=service_type,
        service_id=service_id,
        tpek_der_payload=tpek_der_payload,
        tpek_der_payload_signature=tpek_der_payload_signature,
    )
    assert rep

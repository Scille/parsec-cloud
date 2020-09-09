# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.api.protocol import APIV1_HandshakeType, sgx_hello_world_serializer
from parsec.backend.utils import catch_protocol_errors, api


class BaseSgxComponent:
    def __init__(self):
        pass

    @api("sgx_hello_world", handshake_types=[APIV1_HandshakeType.ADMINISTRATION])
    @catch_protocol_errors
    async def api_sgx_hello_world(self, client_ctx, msg):
        msg = sgx_hello_world_serializer.req_load(msg)
        await self.hello_world()
        return sgx_hello_world_serializer.rep_dump({"status": "ok"})

    async def hello_world(self):
        raise NotImplementedError()

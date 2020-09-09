# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = ("sgx_hello_world_serializer",)


class SgxHelloWorldReqSchema(BaseReqSchema):
    pass


class SgxHelloWorldRepSchema(BaseRepSchema):
    pass


sgx_hello_world_serializer = CmdSerializer(SgxHelloWorldReqSchema, SgxHelloWorldRepSchema)

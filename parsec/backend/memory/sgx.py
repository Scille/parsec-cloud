# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.backend.sgx import BaseSgxComponent

from sys import stderr


class MemorySgxComponent(BaseSgxComponent):
    def __init__(self, send_event, *args, **kwargs):
        self._send_event = send_event

    async def hello_world(self):
        print("Hello World from Enclave !", file=stderr)

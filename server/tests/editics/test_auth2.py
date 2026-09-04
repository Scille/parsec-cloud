# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from __future__ import annotations

from parsec._parsec import VlobID
from tests.common import (
    CoolorgRpcClients,
    # assert_oo_shape,
    # load_captured_session,
    # SimulatedEditicsClient,
    EditicsJSRuntime,
)


async def test_foo(
    coolorg: CoolorgRpcClients,
    editics_js_runtime: EditicsJSRuntime,
):
    async with editics_js_runtime.new_client(
        who=coolorg.alice,
        realm_id=coolorg.wksp1_id,
        document_id=VlobID.new(),
    ) as alice_editics_client:
        assert (
            await alice_editics_client.inject_oo_client_event(
                {"type": "isSaveLock", "syncChangesIndex": 0}
            )
            is None
        )

        assert await alice_editics_client.listen_oo_server_event() == {
            "type": "saveLock",
            "saveLock": False,
        }

    # async with coolorg.alice.join_editics_session(
    #     realm_id=coolorg.wksp1_id,
    #     document_id=VlobID.new(),
    #     vlob_version=1,
    #     editor_type=1,
    # ) as alice_editics:
    # alice_editics = SimulatedEditicsClient()

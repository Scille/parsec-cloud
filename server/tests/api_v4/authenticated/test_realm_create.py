# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import DateTime, RealmRole, RealmRoleCertificate, VlobID, authenticated_cmds
from parsec.events import EventRealmCertificate
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_realm_create_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    timestamp = DateTime.now()
    wksp_id = VlobID.new()
    certif = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=timestamp,
        realm_id=wksp_id,
        role=RealmRole.OWNER,
        user_id=coolorg.alice.device_id.user_id,
    )

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.realm_create(
            realm_role_certificate=certif.dump_and_sign(coolorg.alice.signing_key)
        )
        assert rep == authenticated_cmds.v4.realm_create.RepOk()
        await spy.wait_event_occurred(
            EventRealmCertificate(
                organization_id=coolorg.organization_id,
                timestamp=timestamp,
                realm_id=wksp_id,
                user_id=coolorg.alice.device_id.user_id,
                role_removed=False,
            )
        )


# async def test_authenticated_realm_create_already_exists(
#     coolorg: CoolorgRpcClients, backend: Backend
# ) -> None:
#     block_id = BlockID.new()
#     block = b"<block content>"

#     await backend.block.create(
#         coolorg.organization_id, coolorg.alice.device_id, block_id, coolorg.wksp1_id, block
#     )

#     rep = await coolorg.alice.block_create(block_id, coolorg.wksp1_id, block)
#     assert rep == authenticated_cmds.v4.block_create.RepAlreadyExists()

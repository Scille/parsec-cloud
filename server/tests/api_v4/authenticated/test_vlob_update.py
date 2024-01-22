# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

from parsec._parsec import DateTime, VlobID, authenticated_cmds
from parsec.events import EventVlob
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_vlob_update_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    vlob_id = VlobID.new()
    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)

    v1_blob = b"<block content 1>"
    v1_timestamp = DateTime.now()
    outcome = await backend.vlob.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        vlob_id=vlob_id,
        key_index=1,
        blob=v1_blob,
        timestamp=v1_timestamp,
        sequester_blob=None,
    )
    assert outcome is None, outcome

    with backend.event_bus.spy() as spy:
        v2_blob = b"<block content 2>"
        v2_timestamp = DateTime.now()
        rep = await coolorg.alice.vlob_update(
            vlob_id=vlob_id,
            key_index=1,
            version=2,
            blob=v2_blob,
            timestamp=v2_timestamp,
            sequester_blob=None,
        )
        assert rep == authenticated_cmds.v4.vlob_update.RepOk()

        await spy.wait_event_occurred(
            EventVlob(
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                realm_id=coolorg.wksp1_id,
                timestamp=v2_timestamp,
                vlob_id=vlob_id,
                version=2,
                blob=v2_blob,
                last_common_certificate_timestamp=DateTime(2000, 1, 6),
                last_realm_certificate_timestamp=DateTime(2000, 1, 12),
            )
        )

    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == {
        **initial_dump,
        vlob_id: [
            (coolorg.alice.device_id, ANY, coolorg.wksp1_id, v1_blob),
            (coolorg.alice.device_id, ANY, coolorg.wksp1_id, v2_blob),
        ],
    }


# TODO: check that blob bigger than EVENT_VLOB_MAX_BLOB_SIZE doesn't get in the event
# TODO: checking bad key index should be done on both unknown and valid but old key key index

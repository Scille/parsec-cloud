# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import DateTime, VlobID, authenticated_cmds, testbed
from parsec.events import EventVlob
from tests.common import Backend, CoolorgRpcClients


@pytest.mark.parametrize("key_index", (0, 1))
async def test_authenticated_vlob_create_ok(
    key_index: int, coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    vlob_id = VlobID.new()
    initial_dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)

    if key_index == 0:
        assert isinstance(coolorg.alice.event, testbed.TestbedEventBootstrapOrganization)
        realm_id = coolorg.alice.event.first_user_user_realm_id
        last_realm_certificate_timestamp = DateTime(2000, 1, 14)
    else:
        realm_id = coolorg.wksp1_id
        last_realm_certificate_timestamp = DateTime(2000, 1, 12)

    v1_blob = b"<block content>"
    v1_timestamp = DateTime.now()
    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.vlob_create(
            realm_id=realm_id,
            vlob_id=vlob_id,
            key_index=key_index,
            timestamp=v1_timestamp,
            blob=v1_blob,
            sequester_blob=None,
        )
        assert rep == authenticated_cmds.v4.vlob_create.RepOk()

        await spy.wait_event_occurred(
            EventVlob(
                organization_id=coolorg.organization_id,
                author=coolorg.alice.device_id,
                realm_id=realm_id,
                timestamp=v1_timestamp,
                vlob_id=vlob_id,
                version=1,
                blob=v1_blob,
                last_common_certificate_timestamp=DateTime(2000, 1, 6),
                last_realm_certificate_timestamp=last_realm_certificate_timestamp,
            )
        )

    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    assert dump == {
        **initial_dump,
        vlob_id: [(coolorg.alice.device_id, ANY, realm_id, v1_blob)],
    }


# TODO: check that blob bigger than EVENT_VLOB_MAX_BLOB_SIZE doesn't get in the event

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import SigningKey
from tests.common import MinimalorgRpcClients


@pytest.mark.parametrize("initial_good_auth", (False, True))
async def test_events_listen_auth_then_not_allowed(
    initial_good_auth: bool,
    minimalorg: MinimalorgRpcClients,
) -> None:
    if initial_good_auth:
        # First authentication goes fine (which setups authentication cache)...
        async with minimalorg.alice.events_listen() as alice_sse:
            await alice_sse.next_event()

    # ...no we force alice to use the wrong signing key...
    minimalorg.alice.signing_key = SigningKey.generate()

    # ...which cause authentication failure

    response = await minimalorg.alice.raw_sse_connection()
    assert response.status_code == 403, response.content


# TODO: Here put generic tests on the `/authenticated/<raw_organization_id>/events` route:
# TODO: - test keepalive (must be done by starting an actual uvicorn server)
# TODO: - test `Last-Event-ID`
# TODO: - test close on user revoked
# TODO: - test close on backpressure (too many events pilling up)
# TODO: - test bad accept type

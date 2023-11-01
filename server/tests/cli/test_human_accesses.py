# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from tests.cli.common import cli_invoke_in_thread, cli_with_running_backend_testbed


@pytest.mark.xfail(reason="must correct the trio.serve_tcp")
@pytest.mark.postgresql
async def test_human_accesses(backend, alice, postgresql_url):
    async with cli_with_running_backend_testbed(backend, alice) as (_, alice):
        cmd = (
            "human_accesses"
            f" --db {postgresql_url} --db-min-connections 1 --db-max-connections 2"
            f" --organization {alice.organization_id.str}"
        )

        result = await cli_invoke_in_thread(cmd)
        assert result.exit_code == 0
        assert result.output.startswith("Found 3 result(s)")

        # Also test with filter

        result = await cli_invoke_in_thread(f"{cmd} --filter alice")
        assert result.exit_code == 0
        assert result.output.startswith("Found 1 result(s)")

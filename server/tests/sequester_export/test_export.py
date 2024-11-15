# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import sqlite3
from pathlib import Path

from parsec._parsec import (
    DateTime,
)
from parsec.realm_export import export_realm
from tests.common import Backend, CoolorgRpcClients


async def test_export_ok(
    coolorg: CoolorgRpcClients, backend: Backend, tmp_path: Path
):
    output_db_path = tmp_path / "output.sqlite"

    await export_realm(
        backend=backend,
        organization_id=coolorg.organization_id,
        realm_id=coolorg.wksp1_id,
        output_db_path=output_db_path,
        snapshot_timestamp=DateTime.now(),
        on_progress=lambda x: None,
    )

    assert output_db_path.is_file()
    con = sqlite3.connect(output_db_path)


# async def test_re_export_is_noop(
#     coolorg: CoolorgRpcClients, backend: Backend, tmp_path: Path
# ):
#     output_db_path = tmp_path / "output.sqlite"

#     await export_realm(
#         backend=backend,
#         organization_id=coolorg.organization_id,
#         realm_id=coolorg.wksp1_id,
#         output_db_path=output_db_path,
#         on_progress=lambda x: None,
#     )

#     await export_realm(
#         backend=backend,
#         organization_id=coolorg.organization_id,
#         realm_id=coolorg.wksp1_id,
#         output_db_path=output_db_path,
#         on_progress=lambda x: None,
#     )


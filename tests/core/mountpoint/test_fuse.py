# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import pytest


@pytest.mark.linux  # win32 doesn't allow to remove an opened file
def test_delete_then_close_file(mountpoint_service):
    async def _bootstrap(fs, mountpoint_manager):
        await fs.file_create(f"/w/foo.txt")

    mountpoint_service.start()
    mountpoint_service.execute(_bootstrap)

    w_path = mountpoint_service.get_default_workspace_mountpoint()
    foo_path = w_path / "foo.txt"
    fd = os.open(foo_path, os.O_RDWR)
    os.unlink(foo_path)
    os.close(fd)

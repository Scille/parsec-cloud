# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.core.types import FsPath


def _day(d):
    return Pendulum(2000, 1, d)


@pytest.mark.trio
async def test_versions_existing_file_no_remove_minimal_synced(alice_workspace, alice):
    versions = await alice_workspace.versions(FsPath("/files/renamed"), skip_minimal_sync=False)

    assert len(versions) == 6

    # Moved /files/content to /files/renamed on day 5, moved it again later
    assert versions[0][1:] == (
        3,
        _day(6),
        _day(7),
        alice.device_id,
        _day(5),
        False,
        5,
        FsPath("/files/content"),
        FsPath("/files/renamed_again"),
    )

    # Created a new file with the same name on day 8
    assert versions[1][1:] == (1, _day(8), _day(8), alice.device_id, _day(8), False, 0, None, None)

    # Wrote on it. Moved it again on day 9 as we renamed /files to /moved
    assert versions[2][1:] == (
        2,
        _day(8),
        _day(9),
        alice.device_id,
        _day(8),
        False,
        6,
        None,
        FsPath("/moved/renamed"),
    )

    # And moved back /moved to /files on day 11, /files/renamed is deleted on day 12
    assert versions[3][1:] == (
        2,
        _day(11),
        _day(12),
        alice.device_id,
        _day(8),
        False,
        6,
        FsPath("/moved/renamed"),
        None,
    )

    # Created a file, again, but didn't write
    assert versions[4][1:] == (
        1,
        _day(13),
        _day(14),
        alice.device_id,
        _day(13),
        False,
        0,
        None,
        None,
    )
    # Used "touch" method again, but on a created file. Wrote on it. Didn't delete since then
    assert versions[5][1:3] == (2, _day(14))
    assert Pendulum.now().add(hours=-1) < versions[5][3] < Pendulum.now()
    assert versions[5][4:] == (alice.device_id, _day(14), False, 5, None, None)


@pytest.mark.trio
async def test_versions_existing_file_remove_minimal_synced(alice_workspace, alice):
    versions = await alice_workspace.versions(FsPath("/files/renamed"))

    assert len(versions) == 5

    # Moved /files/content to /files/renamed on day 5, moved it again later
    assert versions[0][1:] == (
        3,
        _day(6),
        _day(7),
        alice.device_id,
        _day(5),
        False,
        5,
        FsPath("/files/content"),
        FsPath("/files/renamed_again"),
    )
    # Created a new file with the same name on day 8
    # This entry is deleted as we only get the one obtained by writing on it in our list
    # This is the entry where we wrote on it
    # Moved it again on day 9 as we renamed /files to /moved
    assert versions[1][1:] == (
        2,
        _day(8),
        _day(9),
        alice.device_id,
        _day(8),
        False,
        6,
        None,
        FsPath("/moved/renamed"),
    )
    # And moved back /moved to /files on day 11, /files/renamed is deleted on day 12
    assert versions[2][1:] == (
        2,
        _day(11),
        _day(12),
        alice.device_id,
        _day(8),
        False,
        6,
        FsPath("/moved/renamed"),
        None,
    )
    # Created a file, again, but didn't write
    assert versions[3][1:] == (
        1,
        _day(13),
        _day(14),
        alice.device_id,
        _day(13),
        False,
        0,
        None,
        None,
    )
    # Used "touch" method again, but on a created file. Wrote on it. Didn't delete since then
    assert versions[4][1:3] == (2, _day(14))
    assert Pendulum.now().add(hours=-1) < versions[4][3] < Pendulum.now()
    assert versions[4][4:] == (alice.device_id, _day(14), False, 5, None, None)


@pytest.mark.trio
@pytest.mark.parametrize("skip_minimal_sync", (False, True))
async def test_versions_non_existing_file_remove_minimal_synced(
    alice_workspace, alice, skip_minimal_sync
):
    versions = await alice_workspace.versions(
        FsPath("/moved/renamed"), skip_minimal_sync=skip_minimal_sync
    )
    assert len(versions) == 1

    assert versions[0][1:] == (
        2,
        _day(9),
        _day(11),
        alice.device_id,
        _day(8),
        False,
        6,
        FsPath("/files/renamed"),
        FsPath("/files/renamed"),
    )


@pytest.mark.trio
@pytest.mark.parametrize("skip_minimal_sync", (False, True))
async def test_versions_existing_directory(alice_workspace, alice, skip_minimal_sync):
    versions = await alice_workspace.versions(FsPath("/files"), skip_minimal_sync=skip_minimal_sync)
    assert len(versions) == 8
    assert versions[0][1:] == (
        1,
        _day(4),
        _day(4),
        alice.device_id,
        _day(4),
        True,
        None,
        None,
        None,
    )
    assert versions[1][1:] == (
        2,
        _day(4),
        _day(6),
        alice.device_id,
        _day(4),
        True,
        None,
        None,
        None,
    )
    assert versions[2][1:] == (
        3,
        _day(6),
        _day(7),
        alice.device_id,
        _day(6),
        True,
        None,
        None,
        None,
    )
    assert versions[3][1:] == (
        4,
        _day(7),
        _day(8),
        alice.device_id,
        _day(7),
        True,
        None,
        None,
        None,
    )
    assert versions[4][1:] == (
        5,
        _day(8),
        _day(9),
        alice.device_id,
        _day(8),
        True,
        None,
        None,
        FsPath("/moved"),
    )
    assert versions[5][1:] == (
        6,
        _day(11),
        _day(12),
        alice.device_id,
        _day(10),
        True,
        None,
        FsPath("/moved"),
        None,
    )
    assert versions[6][1:] == (
        7,
        _day(12),
        _day(13),
        alice.device_id,
        _day(12),
        True,
        None,
        None,
        None,
    )
    assert versions[7][1:3] == (8, _day(13))
    assert Pendulum.now().add(hours=-1) < versions[7][3] < Pendulum.now()
    assert versions[7][4:] == (alice.device_id, _day(13), True, None, None, None)


@pytest.mark.trio
async def test_version_non_existing_directory(alice_workspace, alice):
    versions = await alice_workspace.versions(FsPath("/moved"))
    assert len(versions) == 2
    assert versions[0][1:] == (
        5,
        _day(9),
        _day(10),
        alice.device_id,
        _day(8),
        True,
        None,
        FsPath("/files"),
        None,
    )
    assert versions[1][1:] == (
        6,
        _day(10),
        _day(11),
        alice.device_id,
        _day(10),
        True,
        None,
        None,
        FsPath("/files"),
    )

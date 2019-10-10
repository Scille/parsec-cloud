# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.core.types import FsPath


@pytest.mark.trio
async def test_versions_existing_file(alice_workspace, alice):
    versions = await alice_workspace.versions(FsPath("/files/renamed_content"))
    versions_list = list(versions.items())

    assert len(versions_list) == 6

    # Moved /files/content to /files/renamed_content on day 5, moved it again later
    assert versions_list[0][0][1:] == (3, Pendulum(2000, 1, 6), Pendulum(2000, 1, 7))
    assert versions_list[0][1] == (
        (alice.device_id, Pendulum(2000, 1, 5)),
        FsPath("/files/content"),
        FsPath("/files/renamed_again_content"),
    )

    # Created a new file with the same name on day 8
    assert versions_list[1][0][1:] == (1, Pendulum(2000, 1, 8), Pendulum(2000, 1, 8))
    assert versions_list[1][1] == ((alice.device_id, Pendulum(2000, 1, 8)), None, None)

    # Wrote on it. Moved it again on day 9 as we renamed /files to /moved_files
    assert versions_list[2][0][1:] == (2, Pendulum(2000, 1, 8), Pendulum(2000, 1, 9))
    assert versions_list[2][1] == (
        (alice.device_id, Pendulum(2000, 1, 8)),
        None,
        FsPath("/moved_files/renamed_content"),
    )

    # And moved back /moved_files to /files on day 11, /files/renamed_content is deleted on day 12
    assert versions_list[3][0][1:] == (2, Pendulum(2000, 1, 11), Pendulum(2000, 1, 12))
    assert versions_list[3][1] == (
        (alice.device_id, Pendulum(2000, 1, 8)),
        FsPath("/moved_files/renamed_content"),
        None,
    )

    # Created a file, again, but didn't write
    assert versions_list[4][0][1:] == (1, Pendulum(2000, 1, 13), Pendulum(2000, 1, 14))
    assert versions_list[4][1] == ((alice.device_id, Pendulum(2000, 1, 13)), None, None)

    # Used "touch" method again, but on a created file. Wrote on it. Didn't delete since then
    assert versions_list[5][0][1:3] == (2, Pendulum(2000, 1, 14))
    assert Pendulum.now().add(hours=-1) < versions_list[5][0][3] < Pendulum.now()
    assert versions_list[5][1] == ((alice.device_id, Pendulum(2000, 1, 14)), None, None)


@pytest.mark.trio
async def test_versions_non_existing_file(alice_workspace, alice):
    versions = await alice_workspace.versions(FsPath("/moved_files/renamed_content"))
    versions_list = list(versions.items())

    assert len(versions_list) == 2

    assert versions_list[0][0][1:] == (2, Pendulum(2000, 1, 9), Pendulum(2000, 1, 10))
    assert versions_list[0][1] == (
        (alice.device_id, Pendulum(2000, 1, 8)),
        FsPath("/files/renamed_content"),
        None,
    )

    assert versions_list[1][0][1:] == (2, Pendulum(2000, 1, 10), Pendulum(2000, 1, 11))
    assert versions_list[1][1] == (
        (alice.device_id, Pendulum(2000, 1, 8)),
        None,
        FsPath("/files/renamed_content"),
    )


@pytest.mark.trio
async def test_versions_existing_directory(alice_workspace, alice):
    versions = await alice_workspace.versions(FsPath("/files"))
    versions_list = list(versions.items())

    assert len(versions_list) == 8

    assert versions_list[0][0][1:] == (1, Pendulum(2000, 1, 4), Pendulum(2000, 1, 4))
    assert versions_list[0][1] == ((alice.device_id, Pendulum(2000, 1, 4)), None, None)

    assert versions_list[1][0][1:] == (2, Pendulum(2000, 1, 4), Pendulum(2000, 1, 6))
    assert versions_list[1][1] == ((alice.device_id, Pendulum(2000, 1, 4)), None, None)

    assert versions_list[2][0][1:] == (3, Pendulum(2000, 1, 6), Pendulum(2000, 1, 7))
    assert versions_list[2][1] == ((alice.device_id, Pendulum(2000, 1, 6)), None, None)

    assert versions_list[3][0][1:] == (4, Pendulum(2000, 1, 7), Pendulum(2000, 1, 8))
    assert versions_list[3][1] == ((alice.device_id, Pendulum(2000, 1, 7)), None, None)

    assert versions_list[4][0][1:] == (5, Pendulum(2000, 1, 8), Pendulum(2000, 1, 9))
    assert versions_list[4][1] == (
        (alice.device_id, Pendulum(2000, 1, 8)),
        None,
        FsPath("/moved_files"),
    )

    assert versions_list[5][0][1:] == (6, Pendulum(2000, 1, 11), Pendulum(2000, 1, 12))
    assert versions_list[5][1] == (
        (alice.device_id, Pendulum(2000, 1, 10)),
        FsPath("/moved_files"),
        None,
    )

    assert versions_list[6][0][1:] == (7, Pendulum(2000, 1, 12), Pendulum(2000, 1, 13))
    assert versions_list[6][1] == ((alice.device_id, Pendulum(2000, 1, 12)), None, None)

    assert versions_list[7][0][1:3] == (8, Pendulum(2000, 1, 13))
    assert Pendulum.now().add(hours=-1) < versions_list[7][0][3] < Pendulum.now()
    assert versions_list[7][1] == ((alice.device_id, Pendulum(2000, 1, 13)), None, None)


@pytest.mark.trio
async def test_version_non_existing_directory(alice_workspace, alice):
    versions = await alice_workspace.versions(FsPath("/moved_files"))
    versions_list = list(versions.items())

    assert len(versions_list) == 2

    assert versions_list[0][0][1:] == (5, Pendulum(2000, 1, 9), Pendulum(2000, 1, 10))
    assert versions_list[0][1] == ((alice.device_id, Pendulum(2000, 1, 8)), FsPath("/files"), None)

    assert versions_list[1][0][1:] == (6, Pendulum(2000, 1, 10), Pendulum(2000, 1, 11))
    assert versions_list[1][1] == ((alice.device_id, Pendulum(2000, 1, 10)), None, FsPath("/files"))

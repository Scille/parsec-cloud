# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import sys

import pytest

if sys.platform != "win32":
    pytest.skip("Windows only", allow_module_level=True)

from winfspy.tests.winfstest import test_winfs as winfstest

# Constants

TEST_MODULES = winfstest.TEST_MODULES


# Fixtures

process_runner = winfstest.process_runner


@pytest.fixture
def file_system_path(mountpoint_service):
    return mountpoint_service.wpath


# Tests


@pytest.mark.slow
@pytest.mark.mountpoint
@pytest.mark.parametrize("test_module_path", TEST_MODULES, ids=[path.name for path in TEST_MODULES])
def test_winfstest(test_module_path, file_system_path, process_runner):

    # File attributes are not supported at the moment
    if "CreateFile_Attributes" in test_module_path.name:
        pytest.xfail()

    # Does not pass because of strict attribute testing
    if "CreateRemoveDirectory" in test_module_path.name:
        pytest.xfail()

    # Read-only attribute is required for the MoveFile test case
    if "MoveFile" in test_module_path.name:
        pytest.xfail()

    # Setting file attributes is not supported at the moment
    if "SetGetFileAttributes" in test_module_path.name:
        pytest.xfail()

    # Setting file times is not supported at the moment
    if "FileTime" in test_module_path.name:
        pytest.xfail()

    # Setting security attributes is not supported at the moment
    if "GetSetSecurity" in test_module_path.name:
        pytest.xfail()

    # Run winfstest with the parsec mountpoint
    winfstest.test_winfs(test_module_path, file_system_path, process_runner)

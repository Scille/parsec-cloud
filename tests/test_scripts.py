# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import os


@pytest.mark.skipif(os.name != "posix", reason="Test for linux")
def test_run_test_env_linux():
    os.chdir(os.path.dirname(__file__))
    assert os.system('bash -c "source scripts/run_testenv.sh"') == 0


@pytest.mark.skipif(os.name != "nt", reason="Test for windows")
def test_run_test_env_windows():
    os.chdir(os.path.dirname(__file__))
    assert os.system("source scripts/run_testenv.bat") == 0

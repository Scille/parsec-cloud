# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import os
import pathlib
import re
import subprocess
import sys
import tempfile

import psutil
import pytest


def kill_local_backend(backend_port=6888):
    pattern = f"parsec.* backend.* run.* -P {backend_port}"
    for proc in psutil.process_iter():
        if "python" in proc.name():
            arguments = " ".join(proc.cmdline())
            if re.search(pattern, arguments):
                proc.kill()


@pytest.fixture
def run_testenv():
    try:
        # Source the run_testenv script and echo the testenv path
        base_dir = os.path.dirname(__file__)
        if sys.platform == "win32":
            fd, bat_script = tempfile.mkstemp(suffix=".bat")
            with open(fd, "w") as f:
                f.write(f"call {base_dir}\\scripts\\run_testenv.bat\r\necho %APPDATA%")
            output = subprocess.check_output(str(bat_script))
            os.unlink(bat_script)
        else:
            output = subprocess.check_output(
                f"source {base_dir}/scripts/run_testenv.sh && echo $XDG_CONFIG_HOME",
                shell=True,
                executable="bash",
            )

        # Retrieve the testenv path
        testenv_path = pathlib.Path(output.splitlines()[-1].decode())
        if sys.platform == "win32":
            data_path = testenv_path / "parsec" / "data"
            config_path = testenv_path / "parsec" / "config"
        else:
            testenv_path = testenv_path.parent
            data_path = testenv_path / "share" / "parsec"
            config_path = testenv_path / "config" / "parsec"

        # Make sure the corresponding directories exist
        assert testenv_path.exists()
        assert data_path.exists()
        assert config_path.exists()

        yield config_path

    # Make sure we don't leave a backend running as it messes up with the CI
    finally:
        kill_local_backend()

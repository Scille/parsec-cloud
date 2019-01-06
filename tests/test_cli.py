import pytest
import re
import os
from pathlib import Path
from contextlib import contextmanager
from time import sleep
import subprocess


CWD = Path(__file__).parent.parent


def _run(cmd):
    print(f"========= RUN {cmd} ==============")
    env = {**os.environ.copy(), "DEBUG": "true"}
    ret = subprocess.run(
        cmd.split(), stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=CWD, env=env
    )
    print(ret.stdout.decode())
    print(ret.stderr.decode())
    print(f"========= DONE {ret.returncode} ==============")
    assert ret.returncode == 0
    return ret


class LivingStream:
    def __init__(self, stream):
        self.stream = stream
        self._already_read_data = ""

    def rewind(self):
        pass

    def read(self):
        new_data_size = len(self.stream.peek())
        new_data = self.stream.read(new_data_size).decode()
        self._already_read_data += new_data
        return self._already_read_data


@contextmanager
def _running(cmd, wait_for=None):
    env = {**os.environ.copy(), "DEBUG": "true"}
    p = subprocess.Popen(
        cmd.split(),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        stdin=subprocess.PIPE,
        cwd=CWD,
        env=env,
    )
    p.live_stdout = LivingStream(p.stdout)
    p.live_stderr = LivingStream(p.stderr)
    try:
        if wait_for:
            out = ""
            for _ in range(10):
                out = p.live_stdout.read()[len(out) :]
                if out:
                    print(out, end="")
                if wait_for in out:
                    break
                sleep(0.1)
            else:
                raise RuntimeError(f"Command took too much time to start")

        yield p

    finally:
        print(p.live_stdout.read())
        print(p.live_stderr.read())
        p.kill()
        p.wait()


def test_init_backend(postgresql_url, unused_tcp_port):
    p = _run(f"python -m parsec.cli backend init --force --db {postgresql_url}")

    # Already initialized db is ok
    p = _run(f"python -m parsec.cli backend init --db {postgresql_url}")
    assert b"Database already initialized" in p.stdout

    # Test backend can run
    with _running(
        (
            "python -m parsec.cli backend run --blockstore=POSTGRESQL "
            f"--store={postgresql_url} --port={unused_tcp_port}"
        ),
        wait_for="Starting Parsec Backend",
    ):
        pass


@pytest.mark.slow
def test_full_run(unused_tcp_port, tmpdir):
    org = "org42"
    alice1 = "alice@pc1"
    alice2 = "alice@pc2"
    bob1 = "bob@pc1"
    password = "P@ssw0rd."

    print("######## START BACKEND #########")
    with _running(
        f"python -m parsec.cli backend run --port={unused_tcp_port}",
        wait_for="Starting Parsec Backend",
    ):

        print("####### Create organization #######")
        p = _run(
            "python -m parsec.cli core create_organization "
            f"{org} --addr=ws://localhost:{unused_tcp_port}"
        )
        url = re.search(
            r"^Bootstrap organization url: (.*)$", p.stdout.decode(), re.MULTILINE
        ).group(1)

        print("####### Bootstrap organization #######")
        p = _run(
            "python -m parsec.cli core bootstrap_organization "
            f"{alice1} --addr={url} --config-dir={tmpdir} --password={password}"
        )

        print("####### Create another user #######")
        with _running(
            "python -m parsec.cli core invite_user "
            f"--config-dir={tmpdir} --device={alice1} --password={password} bob",
            wait_for="Invitation token:",
        ) as p:
            stdout = p.live_stdout.read()
            url = re.search(r"^Backend url: (.*)$", stdout, re.MULTILINE).group(1)
            token = re.search(r"^Invitation token: (.*)$", stdout, re.MULTILINE).group(1)

            _run(
                "python -m parsec.cli core claim_user "
                f"--config-dir={tmpdir} --addr={url} --token={token} "
                f"--password={password} {bob1}"
            )

        print("####### Create another device #######")
        with _running(
            "python -m parsec.cli core invite_device "
            f"--config-dir={tmpdir} --device={alice1} --password={password} pc2",
            wait_for="Invitation token:",
        ) as p:
            stdout = p.live_stdout.read()
            url = re.search(r"^Backend url: (.*)$", stdout, re.MULTILINE).group(1)
            token = re.search(r"^Invitation token: (.*)$", stdout, re.MULTILINE).group(1)

            _run(
                "python -m parsec.cli core claim_device "
                f"--config-dir={tmpdir} --addr={url} --token={token} "
                f"--password={password} {alice2}"
            )

        print("####### List users #######")
        p = _run(f"python -m parsec.cli core list_devices --config-dir={tmpdir}")
        stdout = p.stdout.decode()
        assert alice1 in stdout
        assert alice2 in stdout
        assert bob1 in stdout

        print("####### New users can communicate with backend #######")
        _run(
            "python -m parsec.cli core create_workspace wksp1 "
            f"--config-dir={tmpdir} --device={bob1} --password={password}"
        )
        _run(
            "python -m parsec.cli core create_workspace wksp2 "
            f"--config-dir={tmpdir} --device={alice2} --password={password}"
        )

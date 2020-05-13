# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
import os

try:
    import fcntl
except ModuleNotFoundError:  # Not available on Windows
    pass
import sys
import subprocess
from time import sleep
from pathlib import Path
from contextlib import contextmanager
from unittest.mock import ANY, MagicMock, patch, mock_open

import attr
import pytest
import trustme
from click.testing import CliRunner
from async_generator import asynccontextmanager

import parsec
from parsec.cli import cli


CWD = Path(__file__).parent.parent


def test_version():
    runner = CliRunner()
    result = runner.invoke(cli, ["--version"])
    assert result.exit_code == 0
    assert f"parsec, version {parsec.__version__}\n" in result.output


def test_share_workspace(tmpdir, alice, bob):
    # As usual Windows path require a big hack...
    config_dir = tmpdir.strpath.replace("\\", "\\\\")
    # Mocking
    factory_mock = MagicMock()
    share_mock = MagicMock()

    @asynccontextmanager
    async def logged_core_factory(*args, **kwargs):
        yield factory_mock(*args, **kwargs)

    async def share(*args, **kwargs):
        return share_mock(*args, **kwargs)

    factory_mock.return_value.user_fs.workspace_share = share

    with patch("parsec.core.cli.utils.list_available_devices") as list_mock:
        with patch("parsec.core.cli.utils.load_device_with_password") as load_mock:
            with patch("parsec.core.cli.share_workspace.logged_core_factory", logged_core_factory):
                list_mock.return_value = [
                    (bob.organization_id, bob.device_id, "password", MagicMock())
                ]
                runner = CliRunner()
                args = (
                    "core share_workspace --password bla "
                    f"--device={bob.device_id} --config-dir={config_dir} "
                    f"ws1 {alice.user_id}"
                )
                result = runner.invoke(cli, args)

    print(result.output)
    assert result.exit_code == 0
    assert result.output == ""

    list_mock.assert_called_once_with(ANY)
    load_mock.assert_called_once_with(ANY, "bla")
    factory_mock.assert_called_once_with(ANY, load_mock.return_value)
    share_mock.assert_called_once_with("/ws1", alice.user_id)


def _short_cmd(cmd):
    if len(cmd) < 40:
        return cmd
    else:
        return f"{cmd[:40]}…"


def _run(cmd, env={}, timeout=20.0, capture=True):
    print(f"========= RUN {cmd} ==============")
    env = {**os.environ.copy(), "DEBUG": "true", **env}
    cooked_cmd = ("python -m parsec.cli " + cmd).split()
    kwargs = {}
    if capture:
        kwargs["stdout"] = kwargs["stderr"] = subprocess.PIPE
    ret = subprocess.run(cooked_cmd, cwd=CWD, env=env, timeout=timeout, **kwargs)
    if capture:
        print(ret.stdout.decode(), file=sys.stdout)
        print(ret.stderr.decode(), file=sys.stderr)
    print(f"========= DONE {ret.returncode} ({_short_cmd(cmd)}) ==============")
    assert ret.returncode == 0
    return ret


class LivingStream:
    def __init__(self, stream):
        self.stream = stream
        self._already_read_data = ""

    def read(self):
        new_data_size = len(self.stream.peek())
        new_data = self.stream.read1(new_data_size).decode()
        self._already_read_data += new_data
        return self._already_read_data


@contextmanager
def _running(cmd, wait_for=None, env={}):
    env = {**os.environ.copy(), "DEBUG": "true", **env}
    cooked_cmd = ("python -m parsec.cli " + cmd).split()
    p = subprocess.Popen(
        cooked_cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        stdin=subprocess.PIPE,
        cwd=CWD,
        env=env,
    )
    p.live_stdout = LivingStream(p.stdout)
    p.live_stderr = LivingStream(p.stderr)
    # Turn stdin non-blocking
    fl = fcntl.fcntl(p.stdin, fcntl.F_GETFL)
    fcntl.fcntl(p.stdin, fcntl.F_SETFL, fl | os.O_NONBLOCK)
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
        print(f"**************************** TERM to {p.pid} ({_short_cmd(cmd)})")
        p.terminate()
        p.wait()
        print(p.live_stdout.read())
        print(p.live_stderr.read())


@pytest.mark.skipif(os.name == "nt", reason="Hard to test on Windows...")
@patch("parsec.backend.postgresql.migrations", "toto")
def test_migrate_backend(postgresql_url, unused_tcp_port, tmpdir):
    with patch("parsec.backend.cli.migration._sorted_file_migrations") as _sorted_file_migrations:
        with patch("builtins.open", mock_open(read_data="SELECT current_database();")):
            _sorted_file_migrations.return_value = [
                (100001, "migration1", "100001_migration1.sql"),
                (100002, "migration2", "100002_migration2.sql"),
            ]
            runner = CliRunner()
            dry_run_args = f"backend migrate --db {postgresql_url} --dry-run"
            result = runner.invoke(cli, dry_run_args)
            assert "100001_migration1.sql [ ]" in result.output
            assert "100002_migration2.sql [ ]" in result.output

            apply_args = f"backend migrate --db {postgresql_url}"
            runner = CliRunner()
            result = runner.invoke(cli, apply_args)
            assert "100001_migration1.sql [X]" in result.output
            assert "100002_migration2.sql [X]" in result.output

            _sorted_file_migrations.return_value.append(
                (100003, "migration3", "100003_migration3.sql")
            )

            result = runner.invoke(cli, dry_run_args)
            assert "100001_migration1.sql [X]" in result.output
            assert "100002_migration2.sql [X]" in result.output
            assert "100003_migration3.sql [ ]" in result.output

            result = runner.invoke(cli, apply_args)
            assert "100001_migration1.sql [X]" in result.output
            assert "100002_migration2.sql [X]" in result.output
            assert "100003_migration3.sql [X]" in result.output


@pytest.fixture(params=(False, True), ids=("no_ssl", "ssl"))
def ssl_conf(request):
    @attr.s
    class SSLConf:
        @property
        def use_ssl(self):
            return bool(self.backend_opts)

        client_env = attr.ib(factory=dict)
        backend_opts = attr.ib(default="")

    if not request.param:
        yield SSLConf()
    else:
        ca = trustme.CA()
        server_cert = ca.issue_cert("localhost")
        with ca.cert_pem.tempfile() as ca_certfile, server_cert.cert_chain_pems[
            0
        ].tempfile() as server_certfile, server_cert.private_key_pem.tempfile() as server_keyfile:

            yield SSLConf(
                backend_opts=f" --ssl-keyfile={server_keyfile} --ssl-certfile={server_certfile} ",
                client_env={"SSL_CAFILE": ca_certfile},
            )


@pytest.mark.slow
@pytest.mark.skipif(os.name == "nt", reason="Hard to test on Windows...")
def test_apiv1_full_run(alice, alice2, bob, unused_tcp_port, tmpdir, ssl_conf):
    # As usual Windows path require a big hack...
    config_dir = tmpdir.strpath.replace("\\", "\\\\")
    org = alice.organization_id
    alice1_slug = f"{alice.organization_id}:{alice.device_id}"
    alice2_slug = f"{alice2.organization_id}:{alice2.device_id}"
    bob1_slug = f"{bob.organization_id}:{bob.device_id}"
    alice1 = alice.device_id
    alice2 = alice2.device_id
    bob1 = bob.device_id
    password = "P@ssw0rd."
    administration_token = "9e57754ddfe62f7f8780edc0"

    print("######## START BACKEND #########")
    with _running(
        (
            f"backend run --db=MOCKED --blockstore=MOCKED"
            f" --administration-token={administration_token}"
            f" --port={unused_tcp_port}"
            f" {ssl_conf.backend_opts}"
        ),
        wait_for="Starting Parsec Backend",
    ):

        print("####### Create organization #######")
        admin_url = f"parsec://localhost:{unused_tcp_port}"
        if not ssl_conf.use_ssl:
            admin_url += "?no_ssl=true"

        p = _run(
            "core create_organization "
            f"{org} --addr={admin_url} "
            f"--administration-token={administration_token}",
            env=ssl_conf.client_env,
        )
        url = re.search(
            r"^Bootstrap organization url: (.*)$", p.stdout.decode(), re.MULTILINE
        ).group(1)

        print("####### Bootstrap organization #######")
        _run(
            "core bootstrap_organization "
            f"{alice1} --addr={url} --config-dir={config_dir} --password={password}",
            env=ssl_conf.client_env,
        )

        print("####### Create another user #######")
        with _running(
            "core apiv1 invite_user "
            f"--config-dir={config_dir} --device={alice1_slug} "
            f"--password={password} {bob1.user_id}",
            wait_for="token:",
            env=ssl_conf.client_env,
        ) as p:
            stdout = p.live_stdout.read()
            url = re.search(r"^url: (.*)$", stdout, re.MULTILINE).group(1)
            token = re.search(r"^token: (.*)$", stdout, re.MULTILINE).group(1)

            _run(
                "core apiv1 claim_user "
                f"--config-dir={config_dir} --addr={url} --token={token} "
                f"--password={password} {bob1.device_name}",
                env=ssl_conf.client_env,
            )

        print("####### Create another device #######")
        with _running(
            "core apiv1 invite_device "
            f"--config-dir={config_dir} --device={alice1_slug} --password={password}"
            f" {alice2.device_name}",
            wait_for="token:",
            env=ssl_conf.client_env,
        ) as p:
            stdout = p.live_stdout.read()
            url = re.search(r"^url: (.*)$", stdout, re.MULTILINE).group(1)
            token = re.search(r"^token: (.*)$", stdout, re.MULTILINE).group(1)

            _run(
                "core apiv1 claim_device "
                f"--config-dir={config_dir} --addr={url} --token={token} "
                f"--password={password}",
                env=ssl_conf.client_env,
            )

        print("####### List users #######")
        p = _run(f"core list_devices --config-dir={config_dir}", env=ssl_conf.client_env)
        stdout = p.stdout.decode()
        assert alice1 in stdout
        assert alice2 in stdout
        assert bob1 in stdout

        print("####### New users can communicate with backend #######")
        _run(
            "core create_workspace wksp1 "
            f"--config-dir={config_dir} --device={bob1_slug} --password={password}",
            env=ssl_conf.client_env,
        )
        _run(
            "core create_workspace wksp2 "
            f"--config-dir={config_dir} --device={alice2_slug} --password={password}",
            env=ssl_conf.client_env,
        )

        print("####### Stats organization #######")
        _run(
            "core stats_organization "
            f"{org} --addr={admin_url} "
            f"--administration-token={administration_token}",
            env=ssl_conf.client_env,
        )

        print("####### Status organization #######")
        _run(
            "core status_organization "
            f"{org} --addr={admin_url} "
            f"--administration-token={administration_token}",
            env=ssl_conf.client_env,
        )


@pytest.mark.slow
@pytest.mark.skipif(os.name == "nt", reason="Hard to test on Windows...")
def test_full_run(alice, alice2, bob, unused_tcp_port, tmpdir, ssl_conf):
    # As usual Windows path require a big hack...
    config_dir = tmpdir.strpath.replace("\\", "\\\\")
    org = alice.organization_id
    alice1_slug = f"{alice.organization_id}:{alice.device_id}"
    alice2_slug = f"{alice2.organization_id}:{alice2.device_id}"
    bob1_slug = f"{bob.organization_id}:{bob.device_id}"
    alice1 = alice.device_id
    alice2 = alice2.device_id
    bob1 = bob.device_id
    password = "P@ssw0rd."
    administration_token = "9e57754ddfe62f7f8780edc0"

    print("######## START BACKEND #########")
    with _running(
        (
            f"backend run --db=MOCKED --blockstore=MOCKED"
            f" --administration-token={administration_token}"
            f" --port={unused_tcp_port}"
            f" {ssl_conf.backend_opts}"
        ),
        wait_for="Starting Parsec Backend",
    ):

        print("####### Create organization #######")
        admin_url = f"parsec://localhost:{unused_tcp_port}"
        if not ssl_conf.use_ssl:
            admin_url += "?no_ssl=true"

        p = _run(
            "core create_organization "
            f"{org} --addr={admin_url} "
            f"--administration-token={administration_token}",
            env=ssl_conf.client_env,
        )
        url = re.search(
            r"^Bootstrap organization url: (.*)$", p.stdout.decode(), re.MULTILINE
        ).group(1)

        print("####### Bootstrap organization #######")
        _run(
            "core bootstrap_organization "
            f"{alice1} --addr={url} --config-dir={config_dir} --password={password}",
            env=ssl_conf.client_env,
        )

        print("####### Stats organization #######")
        _run(
            "core stats_organization "
            f"{org} --addr={admin_url} "
            f"--administration-token={administration_token}",
            env=ssl_conf.client_env,
        )

        print("####### Status organization #######")
        _run(
            "core status_organization "
            f"{org} --addr={admin_url} "
            f"--administration-token={administration_token}",
            env=ssl_conf.client_env,
        )

        print("####### Create user&device invitations #######")
        p = _run(
            "core invite_user "
            f"--config-dir={config_dir} --device={alice1_slug} "
            f"--password={password} bob@example.com",
            env=ssl_conf.client_env,
        )
        user_invitation_url = re.search(r"^url: (.*)$", p.stdout.decode(), re.MULTILINE).group(1)
        user_invitation_token = re.search(r"token=([^&]+)", user_invitation_url).group(1)

        p = _run(
            "core invite_device "
            f"--config-dir={config_dir} --device={alice1_slug} "
            f"--password={password}",
            env=ssl_conf.client_env,
        )
        device_invitation_url = re.search(r"^url: (.*)$", p.stdout.decode(), re.MULTILINE).group(1)
        device_invitation_token = re.search(r"token=([^&]+)", device_invitation_url).group(1)

        print("####### List invitations #######")

        p = _run(
            "core list_invitations "
            f"--config-dir={config_dir} --device={alice1_slug} "
            f"--password={password}",
            env=ssl_conf.client_env,
        )
        stdout = p.stdout.decode()
        assert device_invitation_token in stdout
        assert user_invitation_token in stdout

        def _wait_for(p, wait_txt):
            for _ in range(10):
                sleep(0.1)
                stdout = p.live_stdout.read()
                if wait_txt in stdout:
                    return stdout
            else:
                raise AssertionError("Too slow")

        print("####### Claim user invitation #######")
        with _running(
            "core claim_invitation "
            f"--config-dir={config_dir} --password={password} {user_invitation_url} ",
            env=ssl_conf.client_env,
        ) as p_claimer:
            with _running(
                "core greet_invitation "
                f"--config-dir={config_dir} --device={alice1_slug} "
                f"--password={password} {user_invitation_token}",
                env=ssl_conf.client_env,
            ) as p_greeter:

                greeter_code = None

                print("~~~ Retreive greeter code ~~~")
                stdout_greeter = _wait_for(p_greeter, "Code to provide to claimer: ")
                greeter_code = re.search(
                    r"Code to provide to claimer: (.*)$", stdout_greeter, re.MULTILINE
                ).group(1)

                print("~~~ Provide greeter code ~~~")
                stdout_claimer = _wait_for(p_claimer, "Select code provided by greeter")
                assert greeter_code in stdout_claimer
                code_index = re.search(
                    f"([0-9]) - {greeter_code}", stdout_claimer, re.MULTILINE
                ).group(1)

                p_claimer.stdin.write(f"{code_index}\n".encode())
                p_claimer.stdin.flush()

                claimer_code = None

                print("~~~ Retreive claimer code ~~~")
                stdout_claimer = _wait_for(p_claimer, "Code to provide to greeter: ")
                claimer_code = re.search(
                    r"Code to provide to greeter: (.*)$", stdout_claimer, re.MULTILINE
                ).group(1)

                print("~~~ Provide claimer code ~~~")
                stdout_greeter = _wait_for(p_greeter, "Select code provided by claimer")
                assert claimer_code in stdout_greeter
                code_index = re.search(
                    f"([0-9]) - {claimer_code}", stdout_greeter, re.MULTILINE
                ).group(1)

                p_greeter.stdin.write(f"{code_index}\n".encode())
                p_greeter.stdin.flush()

                print("~~~ Claimer fill info ~~~")
                stdout_claimer = _wait_for(p_claimer, "User fullname")
                p_claimer.stdin.write(b"John Doe\n")
                p_claimer.stdin.flush()

                stdout_claimer = _wait_for(p_claimer, "Device ID")
                p_claimer.stdin.write(b"john@dev1\n")
                p_claimer.stdin.flush()

                print("~~~ Greeter validate info ~~~")
                stdout_greeter = _wait_for(p_greeter, "New user label [John Doe]:")
                p_greeter.stdin.write(b"Bob Doe\n")
                p_greeter.stdin.flush()

                stdout_greeter = _wait_for(p_greeter, "New user email [bob@example.com]:")
                p_greeter.stdin.write(b"bob.doe@example.com\n")
                p_greeter.stdin.flush()

                stdout_greeter = _wait_for(p_greeter, "New user device ID [john@dev1]:")
                p_greeter.stdin.write(f"{bob1}\n".encode())
                p_greeter.stdin.flush()

                stdout_greeter = _wait_for(p_greeter, "New user is admin ?")
                p_greeter.stdin.write(b"y\n")
                p_greeter.stdin.flush()

                print("~~~ Greeter finish invitation ~~~")
                _wait_for(p_greeter, "Creating the user in the backend")
                p_greeter.wait()

                print("~~~ Claimer finish invitation ~~~")
                p_claimer.wait()

        print("####### Claim device invitation #######")
        with _running(
            "core claim_invitation "
            f"--config-dir={config_dir} --password={password} {device_invitation_url} ",
            env=ssl_conf.client_env,
        ) as p_claimer:
            with _running(
                "core greet_invitation "
                f"--config-dir={config_dir} --device={alice1_slug} "
                f"--password={password} {device_invitation_token}",
                env=ssl_conf.client_env,
            ) as p_greeter:

                greeter_code = None

                print("~~~ Retreive greeter code ~~~")
                stdout_greeter = _wait_for(p_greeter, "Code to provide to claimer: ")
                greeter_code = re.search(
                    r"Code to provide to claimer: (.*)$", stdout_greeter, re.MULTILINE
                ).group(1)

                print("~~~ Provide greeter code ~~~")
                stdout_claimer = _wait_for(p_claimer, "Select code provided by greeter")
                assert greeter_code in stdout_claimer
                code_index = re.search(
                    f"([0-9]) - {greeter_code}", stdout_claimer, re.MULTILINE
                ).group(1)

                p_claimer.stdin.write(f"{code_index}\n".encode())
                p_claimer.stdin.flush()

                claimer_code = None

                print("~~~ Retreive claimer code ~~~")
                stdout_claimer = _wait_for(p_claimer, "Code to provide to greeter: ")
                claimer_code = re.search(
                    r"Code to provide to greeter: (.*)$", stdout_claimer, re.MULTILINE
                ).group(1)

                print("~~~ Provide claimer code ~~~")
                stdout_greeter = _wait_for(p_greeter, "Select code provided by claimer")
                assert claimer_code in stdout_greeter
                code_index = re.search(
                    f"([0-9]) - {claimer_code}", stdout_greeter, re.MULTILINE
                ).group(1)

                p_greeter.stdin.write(f"{code_index}\n".encode())
                p_greeter.stdin.flush()

                print("~~~ Claimer fill info ~~~")
                stdout_claimer = _wait_for(p_claimer, "Device name")
                p_claimer.stdin.write(b"devB\n")
                p_claimer.stdin.flush()

                print("~~~ Greeter validate info ~~~")
                stdout_greeter = _wait_for(p_greeter, "New device name [devB]:")
                p_greeter.stdin.write(f"{alice2.device_name}\n".encode())
                p_greeter.stdin.flush()

                print("~~~ Greeter finish invitation ~~~")
                _wait_for(p_greeter, "Creating the device in the backend")
                p_greeter.wait()

                print("~~~ Claimer finish invitation ~~~")
                p_claimer.wait()

        print("####### List users #######")
        p = _run(f"core list_devices --config-dir={config_dir}", env=ssl_conf.client_env)
        stdout = p.stdout.decode()
        assert alice1 in stdout
        assert alice2 in stdout
        assert bob1 in stdout

        print("####### New users can communicate with backend #######")
        _run(
            "core create_workspace wksp1 "
            f"--config-dir={config_dir} --device={bob1_slug} --password={password}",
            env=ssl_conf.client_env,
        )
        _run(
            "core create_workspace wksp2 "
            f"--config-dir={config_dir} --device={alice2_slug} --password={password}",
            env=ssl_conf.client_env,
        )


@pytest.mark.gui
@pytest.mark.slow
@pytest.mark.parametrize(
    "env",
    [
        pytest.param({}, id="Standard environement"),
        pytest.param(
            {"WINFSP_LIBRARY_PATH": "nope"},
            id="Wrong winfsp library path",
            marks=pytest.mark.skipif(os.name != "nt", reason="Windows only"),
        ),
        pytest.param(
            {"WINFSP_DEBUG_PATH": "nope"},
            id="Wrong winfsp binary path",
            marks=pytest.mark.skipif(os.name != "nt", reason="Windows only"),
        ),
    ],
)
def test_gui_with_diagnose_option(env):
    _run(f"core gui --diagnose", env=env, capture=False)

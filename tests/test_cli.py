# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import re
import os
import trio
from uuid import UUID
import click
from pathlib import Path
from functools import partial
import oscrypto.asymmetric

try:
    import fcntl
except ModuleNotFoundError:  # Not available on Windows
    pass
import sys
import subprocess
from time import sleep
from contextlib import contextmanager, asynccontextmanager
from unittest.mock import ANY, patch
import attr
import pytest
import trustme
from click.testing import CliRunner, Result as CliResult

from parsec import __version__ as parsec_version
from parsec._parsec import DateTime
from parsec.cli import cli
from parsec.cli_utils import ParsecDateTimeClickType
from parsec.api.protocol import RealmID
from parsec.backend.postgresql import MigrationItem
from parsec.backend.sequester import (
    SequesterServiceAlreadyDisabledError,
    SequesterServiceAlreadyEnabledError,
    SequesterServiceType,
)
from parsec.core.types import BackendAddr, EntryID
from parsec.core.local_device import save_device_with_password_in_config
from parsec.core.types import (
    LocalDevice,
    UserInfo,
    BackendOrganizationAddr,
    BackendPkiEnrollmentAddr,
)
from parsec.core.cli.share_workspace import WORKSPACE_ROLE_CHOICES

from tests.common import (
    AsyncMock,
    real_clock_timeout,
    asgi_app_handle_client_factory,
    customize_fixtures,
    sequester_service_factory,
)


CWD = Path(__file__).parent.parent
# Starting parsec cli as a new subprocess can be very slow (typically a couple
# of seconds on my beafy machine !), so we use an unusaly large value here to
# avoid issues in the CI.
SUBPROCESS_TIMEOUT = 30


@pytest.fixture(params=WORKSPACE_ROLE_CHOICES.keys())
def cli_workspace_role(request):
    expected_role = WORKSPACE_ROLE_CHOICES[request.param]
    return request.param, expected_role


def test_version():
    runner = CliRunner()
    result = runner.invoke(cli, ["--version"])
    assert result.exit_code == 0
    assert f"parsec, version {parsec_version}\n" in result.output


def test_datetime_parsing():
    parser = ParsecDateTimeClickType()
    dt = DateTime(2000, 1, 1, 12, 30, 59)
    assert parser.convert(dt, None, None) is dt
    assert parser.convert("2000-01-01T12:30:59Z", None, None) == dt
    assert parser.convert("2000-01-01T12:30:59.123Z", None, None) == dt.add(microseconds=123000)
    assert parser.convert("2000-01-01T12:30:59.123000Z", None, None) == dt.add(microseconds=123000)
    assert parser.convert("2000-01-01T12:30:59.123456Z", None, None) == dt.add(microseconds=123456)
    assert parser.convert("2000-01-01", None, None) == DateTime(2000, 1, 1)

    with pytest.raises(click.exceptions.BadParameter) as exc:
        parser.convert("dummy", None, None)
    assert (
        str(exc.value)
        == "`dummy` must be a RFC3339 date/datetime (e.g. `2000-01-01` or `2000-01-01T00:00:00Z`)"
    )

    with pytest.raises(click.exceptions.BadParameter):
        parser.convert("2000-01-01Z", None, None)

    # Timezone naive is not allowed
    with pytest.raises(click.exceptions.BadParameter) as exc:
        parser.convert("2000-01-01T12:30:59.123456", None, None)
    with pytest.raises(click.exceptions.BadParameter):
        parser.convert("2000-01-01T12:30:59", None, None)


def test_share_workspace(tmp_path, monkeypatch, alice, bob, cli_workspace_role):
    config_dir = tmp_path / "config"
    # Mocking
    factory_mock = AsyncMock()
    workspace_role, expected_workspace_role = cli_workspace_role

    @asynccontextmanager
    async def logged_core_factory(*args, **kwargs):
        yield factory_mock(*args, **kwargs)

    password = "S3cr3t"
    save_device_with_password_in_config(config_dir, bob, password)
    alice_info = [
        UserInfo(
            user_id=alice.user_id,
            human_handle=alice.human_handle,
            profile=alice.profile,
            created_on=alice.timestamp(),
            revoked_on=None,
        )
    ]

    def _run_cli_share_workspace_test(args, expected_error_code, use_recipiant):
        factory_mock.reset_mock()

        workspace_id = EntryID.new()

        factory_mock.return_value.user_fs.workspace_share.is_async = True
        factory_mock.return_value.find_humans.is_async = True
        factory_mock.return_value.find_humans.return_value = (alice_info, 1)
        factory_mock.return_value.find_workspace_from_name.return_value.id = workspace_id

        monkeypatch.setattr(
            "parsec.core.cli.share_workspace.logged_core_factory", logged_core_factory
        )
        runner = CliRunner()
        result = runner.invoke(cli, args)

        assert result.exit_code == expected_error_code
        factory_mock.assert_called_once_with(ANY, bob)
        factory_mock.return_value.user_fs.workspace_share.assert_called_once_with(
            workspace_id, alice.user_id, expected_workspace_role
        )
        if use_recipiant:
            factory_mock.return_value.find_humans.assert_called_once()
        else:
            factory_mock.return_value.find_humans.assert_not_called()

    default_args = (
        f"core share_workspace --password {password} "
        f"--device={bob.slughash} --config-dir={config_dir.as_posix()} "
        f"--role={workspace_role} "
        f"--workspace-name=ws1 "
    )

    # Test with user-id
    args = default_args + f"--user-id={alice.user_id.str}"
    _run_cli_share_workspace_test(args, 0, False)
    # Test with recipiant
    args = default_args + f"--recipiant=alice@example.com"
    _run_cli_share_workspace_test(args, 0, True)


def test_reencrypt_workspace(tmp_path, monkeypatch, alice, bob):
    config_dir = tmp_path / "config"
    # Mocking
    factory_mock = AsyncMock()

    workspace_id = EntryID.new()

    @asynccontextmanager
    async def logged_core_factory(*args, **kwargs):
        yield factory_mock(*args, **kwargs)

    password = "S3cr3t"
    save_device_with_password_in_config(config_dir, bob, password)

    def _run_cli_reencrypt_workspace_test(args, expected_error_code, from_beginning):
        factory_mock.reset_mock()

        reenc_needs_mock = AsyncMock()
        reenc_needs_mock.need_reencryption = True
        reenc_needs_mock.reencryption_already_in_progress = not from_beginning

        workspace_fs_mock = AsyncMock()
        workspace_fs_mock.get_reencryption_need.return_value = reenc_needs_mock
        workspace_fs_mock.get_reencryption_need.is_async = True

        job_mock = AsyncMock()
        job_mock.do_one_batch.return_value = (100, 100)
        job_mock.do_one_batch.is_async = True

        factory_mock.return_value.user_fs.get_workspace.return_value = workspace_fs_mock
        if from_beginning:
            factory_mock.return_value.user_fs.workspace_start_reencryption.return_value = job_mock
            factory_mock.return_value.user_fs.workspace_start_reencryption.is_async = True
        else:
            factory_mock.return_value.user_fs.workspace_continue_reencryption.return_value = (
                job_mock
            )
            factory_mock.return_value.user_fs.workspace_continue_reencryption.is_async = True
        factory_mock.return_value.find_workspace_from_name.return_value.id = workspace_id

        monkeypatch.setattr(
            "parsec.core.cli.reencrypt_workspace.logged_core_factory", logged_core_factory
        )
        runner = CliRunner()
        result = runner.invoke(cli, args)

        assert result.exit_code == expected_error_code
        factory_mock.assert_called_once_with(ANY, bob)
        factory_mock.return_value.user_fs.get_workspace.assert_called_once_with(workspace_id)
        workspace_fs_mock.get_reencryption_need.assert_called_once()
        if from_beginning:
            factory_mock.return_value.user_fs.workspace_start_reencryption.assert_called_once_with(
                workspace_id
            )
        else:
            factory_mock.return_value.user_fs.workspace_continue_reencryption.assert_called_once_with(
                workspace_id
            )
        job_mock.do_one_batch.assert_called_once()

    args = (
        f"core reencrypt_workspace --password {password} "
        f"--device={bob.slughash} --config-dir={config_dir.as_posix()} "
        f"--workspace-name=ws1 "
    )

    _run_cli_reencrypt_workspace_test(args, 0, from_beginning=True)
    _run_cli_reencrypt_workspace_test(args, 0, from_beginning=False)


def _short_cmd(cmd):
    if len(cmd) < 40:
        return cmd
    else:
        return f"{cmd[:40]}…"


def _run(cmd, env={}, timeout=SUBPROCESS_TIMEOUT, capture=True):
    print(f"========= RUN {cmd} ==============")
    env = {**os.environ.copy(), "DEBUG": "true", **env}
    cooked_cmd = (sys.executable + " -m parsec.cli " + cmd).split()
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
        new_data = self.stream.read()
        if new_data is not None:
            self._already_read_data += new_data.decode()
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
    p.wait_for = partial(_wait_for, p)
    p.wait_for_regex = partial(_wait_for_regex, p)
    p.live_stdout = LivingStream(p.stdout)
    p.live_stderr = LivingStream(p.stderr)
    # Turn stdin/stdout/stderr non-blocking
    for stream in [p.stdin, p.stdout, p.stderr]:
        fl = fcntl.fcntl(stream, fcntl.F_GETFL)
        fcntl.fcntl(stream, fcntl.F_SETFL, fl | os.O_NONBLOCK)
    try:
        if wait_for:
            out = ""
            for _ in range(SUBPROCESS_TIMEOUT * 10):  # 10ms sleep steps
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


def _wait_for(p, wait_txt):
    for _ in range(SUBPROCESS_TIMEOUT * 10):  # 10ms sleep steps
        sleep(0.1)
        stdout = p.live_stdout.read()
        if wait_txt in stdout:
            return stdout
    else:
        raise AssertionError("Too slow")


def _wait_for_regex(p, regex, stderr=False):
    for _ in range(SUBPROCESS_TIMEOUT * 10):  # 10ms sleep steps
        sleep(0.1)
        if stderr:
            output = p.live_stderr.read()
        else:
            output = p.live_stdout.read()
        match = re.search(regex, output, re.MULTILINE)
        if match:
            return match
    else:
        raise AssertionError("Too slow")


@pytest.mark.postgresql
@pytest.mark.skipif(sys.platform == "win32", reason="Hard to test on Windows...")
def test_migrate_backend(postgresql_url, unused_tcp_port):
    sql = "SELECT current_database();"  # Dummy migration content
    dry_run_args = f"backend migrate --db {postgresql_url} --dry-run"
    apply_args = f"backend migrate --db {postgresql_url}"

    with patch("parsec.backend.cli.migration.retrieve_migrations") as retrieve_migrations:
        retrieve_migrations.return_value = [
            MigrationItem(
                idx=100001, name="migration1", file_name="100001_migration1.sql", sql=sql
            ),
            MigrationItem(
                idx=100002, name="migration2", file_name="100002_migration2.sql", sql=sql
            ),
        ]
        runner = CliRunner()
        result = runner.invoke(cli, dry_run_args)
        assert "100001_migration1.sql ✔" in result.output
        assert "100002_migration2.sql ✔" in result.output

        runner = CliRunner()
        result = runner.invoke(cli, apply_args)
        assert "100001_migration1.sql ✔" in result.output
        assert "100002_migration2.sql ✔" in result.output

        retrieve_migrations.return_value.append(
            MigrationItem(idx=100003, name="migration3", file_name="100003_migration3.sql", sql=sql)
        )

        result = runner.invoke(cli, dry_run_args)
        assert "100001_migration1.sql (already applied)" in result.output
        assert "100002_migration2.sql (already applied)" in result.output
        assert "100003_migration3.sql ✔" in result.output

        result = runner.invoke(cli, apply_args)
        assert "100001_migration1.sql (already applied)" in result.output
        assert "100002_migration2.sql (already applied)" in result.output
        assert "100003_migration3.sql ✔" in result.output

        result = runner.invoke(cli, apply_args)
        assert "100001_migration1.sql (already applied)" in result.output
        assert "100002_migration2.sql (already applied)" in result.output
        assert "100003_migration3.sql (already applied)" in result.output


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
        server_cert = ca.issue_cert("127.0.0.1")
        with ca.cert_pem.tempfile() as ca_certfile, server_cert.cert_chain_pems[
            0
        ].tempfile() as server_certfile, server_cert.private_key_pem.tempfile() as server_keyfile:

            yield SSLConf(
                backend_opts=f" --ssl-keyfile={server_keyfile} --ssl-certfile={server_certfile} ",
                # SSL_CERT_FILE is the env var used by default by ssl.SSLContext
                # TODO: replace those by proper params in the api ?
                client_env={"SSL_CAFILE": ca_certfile, "SSL_CERT_FILE": ca_certfile},
            )


@pytest.mark.slow
@pytest.mark.skipif(sys.platform == "win32", reason="Hard to test on Windows...")
def test_full_run(coolorg, unused_tcp_port, tmp_path, ssl_conf):
    config_dir = tmp_path / "config"
    config_dir.mkdir()

    org = coolorg.organization_id

    alice_human_handle_label = "Alice"
    alice_human_handle_email = "alice@example.com"
    bob_human_handle_label = "Bob"
    bob_human_handle_email = "bob@example.com"
    alice1_device_label = "PC1"
    alice2_device_label = "PC2"
    alice3_device_label = "PC3"
    bob_device_label = "Desktop"

    password = "P@ssw0rd."
    administration_token = "9e57754ddfe62f7f8780edc0"

    # Cannot use `click.CliRunner` in this test given it doesn't support
    # concurrent run of commands :'(

    print("######## START BACKEND #########")
    with _running(
        (
            f"backend run --db=MOCKED --blockstore=MOCKED"
            f" --administration-token={administration_token}"
            f" --port=0"
            f" --backend-addr=parsec://127.0.0.1:{unused_tcp_port}"
            f" --email-host=MOCKED"
            f" --log-level=INFO"
            f" {ssl_conf.backend_opts}"
        ),
        wait_for="Starting Parsec Backend",
    ) as bp:
        # Given we set port to 0, it is going to be picked random by hypercorn
        # and printed in the logs
        backend_port = bp.wait_for_regex(
            r"Running on https?://127.0.0.1:([0-9]+)", stderr=True
        ).group(1)

        backend_url = f"parsec://127.0.0.1:{backend_port}"
        if not ssl_conf.use_ssl:
            backend_url += "?no_ssl=true"

        print("####### Create organization #######")

        p = _run(
            "core create_organization "
            f"{org.str} --addr={backend_url} "
            f"--administration-token={administration_token}",
            env=ssl_conf.client_env,
        )
        url = re.search(
            r"^Bootstrap organization url: (.*)$", p.stdout.decode(), re.MULTILINE
        ).group(1)

        print("####### Bootstrap organization #######")
        with _running(
            "core bootstrap_organization "
            f"{url} --config-dir={config_dir.as_posix()} --password={password}",
            env=ssl_conf.client_env,
            wait_for="User fullname:",
        ) as p:
            print("~~~ Bootstrap user fullname ~~~")
            p.stdin.write(f"{alice_human_handle_label}\n".encode())
            p.stdin.flush()
            print("~~~ Bootstrap user email ~~~")
            p.wait_for("User email:")
            p.stdin.write(f"{alice_human_handle_email}\n".encode())
            p.stdin.flush()
            print("~~~ Bootstrap device label ~~~")
            p.wait_for("Device label [")
            p.stdin.write(f"{alice1_device_label}\n".encode())
            p.stdin.flush()
            print("~~~ Bootstrap finish invitation ~~~")
            match = p.wait_for_regex(r"Saving device ([a-zA-Z0-9]+)")
            alice1_slughash = match.group(1)
            p.wait()

        print("####### Stats organization #######")
        _run(
            "core stats_organization "
            f"{org.str} --addr={backend_url} "
            f"--administration-token={administration_token}",
            env=ssl_conf.client_env,
        )

        print("####### Status organization #######")
        _run(
            "core status_organization "
            f"{org.str} --addr={backend_url} "
            f"--administration-token={administration_token}",
            env=ssl_conf.client_env,
        )

        print("####### Create user&device invitations #######")
        p = _run(
            "core invite_user "
            f"--config-dir={config_dir.as_posix()} --device={alice1_slughash} "
            f"--password={password} bob@example.com",
            env=ssl_conf.client_env,
        )
        user_invitation_url = re.search(r"^url: (.*)$", p.stdout.decode(), re.MULTILINE).group(1)
        user_invitation_token = re.search(r"token=([^&]+)", user_invitation_url).group(1)

        p = _run(
            "core invite_device "
            f"--config-dir={config_dir.as_posix()} --device={alice1_slughash} "
            f"--password={password}",
            env=ssl_conf.client_env,
        )
        device_invitation_url = re.search(r"^url: (.*)$", p.stdout.decode(), re.MULTILINE).group(1)
        device_invitation_token = re.search(r"token=([^&]+)", device_invitation_url).group(1)

        print("####### Cancel invitation #######")

        p = _run(
            "core invite_user "
            f"--config-dir={config_dir.as_posix()} --device={alice1_slughash} "
            f"--password={password} zack@example.com",
            env=ssl_conf.client_env,
        )
        to_cancel_invitation_url = re.search(r"^url: (.*)$", p.stdout.decode(), re.MULTILINE).group(
            1
        )
        p = _run(
            "core cancel_invitation "
            f"--config-dir={config_dir.as_posix()} --device={alice1_slughash} "
            f"--password={password} {to_cancel_invitation_url}",
            env=ssl_conf.client_env,
        )

        print("####### List invitations #######")

        p = _run(
            "core list_invitations "
            f"--config-dir={config_dir.as_posix()} --device={alice1_slughash} "
            f"--password={password}",
            env=ssl_conf.client_env,
        )
        stdout = p.stdout.decode()
        assert device_invitation_token in stdout
        assert user_invitation_token in stdout

        print("####### Claim user invitation #######")
        with _running(
            "core claim_invitation "
            f"--config-dir={config_dir.as_posix()} --password={password} {user_invitation_url} ",
            env=ssl_conf.client_env,
        ) as p_claimer:
            with _running(
                "core greet_invitation "
                f"--config-dir={config_dir.as_posix()} --device={alice1_slughash} "
                f"--password={password} {user_invitation_token}",
                env=ssl_conf.client_env,
            ) as p_greeter:

                greeter_code = None

                print("~~~ Retrieve greeter code ~~~")
                match = p_greeter.wait_for_regex(r"Code to provide to claimer: (.*)$")
                greeter_code = match.group(1)

                print("~~~ Provide greeter code ~~~")
                stdout_claimer = p_claimer.wait_for("Select code provided by greeter")
                assert greeter_code in stdout_claimer
                code_index = re.search(
                    f"([0-9]) - {greeter_code}", stdout_claimer, re.MULTILINE
                ).group(1)

                p_claimer.stdin.write(f"{code_index}\n".encode())
                p_claimer.stdin.flush()

                claimer_code = None

                print("~~~ Retrieve claimer code ~~~")
                match = p_claimer.wait_for_regex(r"Code to provide to greeter: (.*)$")
                claimer_code = match.group(1)

                print("~~~ Provide claimer code ~~~")
                stdout_greeter = p_greeter.wait_for("Select code provided by claimer")
                assert claimer_code in stdout_greeter
                code_index = re.search(
                    f"([0-9]) - {claimer_code}", stdout_greeter, re.MULTILINE
                ).group(1)

                p_greeter.stdin.write(f"{code_index}\n".encode())
                p_greeter.stdin.flush()

                print("~~~ Claimer fill info ~~~")
                stdout_claimer = p_claimer.wait_for("User fullname")
                p_claimer.stdin.write(b"John Doe\n")
                p_claimer.stdin.flush()

                stdout_claimer = p_claimer.wait_for("Device label")
                p_claimer.stdin.write(b"D3vIce1\n")
                p_claimer.stdin.flush()

                print("~~~ Greeter validate info ~~~")
                stdout_greeter = p_greeter.wait_for("New user label [John Doe]:")
                p_greeter.stdin.write(f"{bob_human_handle_label}\n".encode())
                p_greeter.stdin.flush()

                stdout_greeter = p_greeter.wait_for("New user email [bob@example.com]:")
                p_greeter.stdin.write(f"{bob_human_handle_email}\n".encode())
                p_greeter.stdin.flush()

                stdout_greeter = p_greeter.wait_for("New user device label [D3vIce1]:")
                p_greeter.stdin.write(f"{bob_device_label}\n".encode())
                p_greeter.stdin.flush()

                stdout_greeter = p_greeter.wait_for("New user profile (0, 1, 2) [1]:")
                p_greeter.stdin.write(b"1\n")
                p_greeter.stdin.flush()

                print("~~~ Greeter finish invitation ~~~")
                p_greeter.wait()

                print("~~~ Claimer finish invitation ~~~")
                match = p_claimer.wait_for_regex(r"Saving device ([a-zA-Z0-9]+)")
                bob1_slughash = match.group(1)
                p_claimer.wait()

        print("####### Claim device invitation #######")
        with _running(
            "core claim_invitation "
            f"--config-dir={config_dir.as_posix()} --password={password} {device_invitation_url} ",
            env=ssl_conf.client_env,
        ) as p_claimer:
            with _running(
                "core greet_invitation "
                f"--config-dir={config_dir.as_posix()} --device={alice1_slughash} "
                f"--password={password} {device_invitation_url}",
                env=ssl_conf.client_env,
            ) as p_greeter:

                greeter_code = None

                print("~~~ Retrieve greeter code ~~~")
                match = p_greeter.wait_for_regex(r"Code to provide to claimer: (.*)$")
                greeter_code = match.group(1)

                print("~~~ Provide greeter code ~~~")
                stdout_claimer = p_claimer.wait_for("Select code provided by greeter")
                assert greeter_code in stdout_claimer
                code_index = re.search(
                    f"([0-9]) - {greeter_code}", stdout_claimer, re.MULTILINE
                ).group(1)

                p_claimer.stdin.write(f"{code_index}\n".encode())
                p_claimer.stdin.flush()

                claimer_code = None

                print("~~~ Retrieve claimer code ~~~")
                match = p_claimer.wait_for_regex(r"Code to provide to greeter: (.*)$")
                claimer_code = match.group(1)

                print("~~~ Provide claimer code ~~~")
                stdout_greeter = p_greeter.wait_for("Select code provided by claimer")
                assert claimer_code in stdout_greeter
                code_index = re.search(
                    rf"([0-9]) - {claimer_code}", stdout_greeter, re.MULTILINE
                ).group(1)
                p_greeter.stdin.write(f"{code_index}\n".encode())
                p_greeter.stdin.flush()

                print("~~~ Claimer fill info ~~~")
                stdout_claimer = p_claimer.wait_for("Device label [")
                p_claimer.stdin.write(b"DeviceB\n")
                p_claimer.stdin.flush()

                print("~~~ Greeter validate info ~~~")
                stdout_greeter = p_greeter.wait_for("New device label [DeviceB]:")
                p_greeter.stdin.write(f"{alice2_device_label}\n".encode())
                p_greeter.stdin.flush()

                print("~~~ Greeter finish invitation ~~~")
                p_greeter.wait()

                print("~~~ Claimer finish invitation ~~~")
                match = p_claimer.wait_for_regex(r"Saving device ([a-zA-Z0-9]+)")
                alice2_slughash = match.group(1)
                p_claimer.wait()

        print("####### List users #######")
        p = _run(f"core list_devices --config-dir={config_dir.as_posix()}", env=ssl_conf.client_env)
        stdout = p.stdout.decode()
        assert alice1_slughash[:3] in stdout
        assert (
            f"{org.str}: {alice_human_handle_label} <{alice_human_handle_email}> @ {alice1_device_label}"
            in stdout
        )
        assert alice2_slughash[:3] in stdout
        assert (
            f"{org.str}: {alice_human_handle_label} <{alice_human_handle_email}> @ {alice2_device_label}"
            in stdout
        )
        assert bob1_slughash[:3] in stdout
        assert (
            f"{org.str}: {bob_human_handle_label} <{bob_human_handle_email}> @ {bob_device_label}"
            in stdout
        )

        print("####### New users can communicate with backend #######")
        _run(
            "core create_workspace wksp1 "
            f"--config-dir={config_dir.as_posix()} --device={bob1_slughash} --password={password}",
            env=ssl_conf.client_env,
        )
        _run(
            "core create_workspace wksp2 "
            f"--config-dir={config_dir.as_posix()} --device={alice2_slughash} --password={password}",
            env=ssl_conf.client_env,
        )

        print("####### Recovery device #######")
        recovery_dir = tmp_path / "recovery"
        recovery_dir.mkdir()
        p = _run(
            f"core export_recovery_device --output={recovery_dir.as_posix()} "
            f"--config-dir={config_dir.as_posix()} --device={alice1_slughash} --password={password}",
            env=ssl_conf.client_env,
        )
        stdout = p.stdout.decode()
        # Retrieve passphrase
        match = re.search(r"Save the recovery passphrase in a safe place: ([a-zA-Z0-9\-]+)", stdout)
        passphrase = match.group(1)
        # Retrieve recovery file
        recovery_file = next(recovery_dir.glob("*.psrk"))
        # Do the import
        p = _run(
            f"core import_recovery_device {recovery_file.as_posix()} "
            f"--passphrase={passphrase} --device-label={alice3_device_label} "
            f"--config-dir={config_dir.as_posix()} --password={password}",
            env=ssl_conf.client_env,
        )
        stdout = p.stdout.decode()
        match = re.search(r"Saving device ([a-zA-Z0-9]+)", stdout)
        alice3_slughash = match.group(1)

        print("####### Recovered device can communicate with backend #######")
        _run(
            "core create_workspace wksp3 "
            f"--config-dir={config_dir.as_posix()} --device={alice3_slughash} --password={password}",
            env=ssl_conf.client_env,
        )


# This test has been detected as flaky.
# Using re-runs is a valid temporary solutions but the problem should be investigated in the future.
@pytest.mark.gui
@pytest.mark.slow
@pytest.mark.flaky(reruns=1)
@pytest.mark.parametrize(
    "env",
    [
        pytest.param({}, id="Standard environement"),
        pytest.param(
            {"WINFSP_LIBRARY_PATH": "nope"},
            id="Wrong winfsp library path",
            marks=pytest.mark.skipif(sys.platform != "win32", reason="Windows only"),
        ),
        pytest.param(
            {"WINFSP_DEBUG_PATH": "nope"},
            id="Wrong winfsp binary path",
            marks=pytest.mark.skipif(sys.platform != "win32", reason="Windows only"),
        ),
    ],
)
def test_gui_with_diagnose_option(env):
    _run(f"core gui --diagnose", env=env, capture=False)


@pytest.fixture
def no_parsec_extension():
    saved_parsec_ext = sys.modules.get("parsec_ext")
    sys.modules["parsec_ext"] = None
    yield
    if saved_parsec_ext is None:
        del sys.modules["parsec_ext"]
    else:
        sys.modules["parsec_ext"] = saved_parsec_ext


def test_pki_enrollment_not_available(tmp_path, alice, no_parsec_extension):
    # First need to have alice device on the disk
    config_dir = tmp_path / "config"
    alice_password = "S3cr3t"
    save_device_with_password_in_config(config_dir, alice, alice_password)

    # Now Run the cli
    runner = CliRunner()
    for cmd in [
        f"core pki_enrollment_submit --config-dir={config_dir.as_posix()} parsec://parsec.example.com/my_org?action=pki_enrollment",
        f"core pki_enrollment_poll --config-dir={config_dir.as_posix()}",
        f"core pki_enrollment_review_pendings --config-dir={config_dir.as_posix()} --device {alice.slughash} --password {alice_password}",
    ]:
        result = runner.invoke(cli, cmd)
        assert result.exit_code == 1
        assert "Error: Parsec smartcard extension not available" in result.output


@asynccontextmanager
async def cli_with_running_backend_testbed(backend_asgi_app, *devices):
    # The cli commands are going to run in a separate thread with they own trio loop,
    # hence we should not share memory channel / events between trio loops otherwise
    # unexpected errors will occur !
    async with trio.open_service_nursery() as nursery:
        listeners = await nursery.start(
            partial(
                trio.serve_tcp,
                asgi_app_handle_client_factory(backend_asgi_app),
                port=0,
                host="127.0.0.1",
            )
        )
        _, port, *_ = listeners[0].socket.getsockname()
        backend_addr = BackendAddr("127.0.0.1", port, use_ssl=False)

        # Now the local device point to an invalid backend address, must fix this
        def _correct_local_device_backend_addr(device):
            organization_addr = BackendOrganizationAddr.build(
                backend_addr,
                organization_id=device.organization_addr.organization_id,
                root_verify_key=device.organization_addr.root_verify_key,
            )
            return LocalDevice(
                organization_addr=organization_addr,
                device_id=device.device_id,
                device_label=device.device_label,
                human_handle=device.human_handle,
                signing_key=device.signing_key,
                private_key=device.private_key,
                profile=device.profile,
                user_manifest_id=device.user_manifest_id,
                user_manifest_key=device.user_manifest_key,
                local_symkey=device.local_symkey,
            )

        yield (backend_addr,) + tuple(_correct_local_device_backend_addr(d) for d in devices)

        nursery.cancel_scope.cancel()


# This test has been detected as flaky.
# Using re-runs is a valid temporary solutions but the problem should be investigated in the future.
@pytest.mark.trio
@pytest.mark.real_tcp
@pytest.mark.flaky(reruns=1)
async def test_pki_enrollment(tmp_path, mocked_parsec_ext_smartcard, backend_asgi_app, alice):
    async with cli_with_running_backend_testbed(backend_asgi_app, alice) as (backend_addr, alice):
        # First, save the local device needed for pki_enrollment_review_pendings command
        config_dir = tmp_path / "config"
        alice_password = "S3cr3t"
        save_device_with_password_in_config(config_dir, alice, alice_password)

        runner = CliRunner()

        async def _cli_invoke_in_thread(cmd: str):
            # We must run the command from another thread given it will create it own trio loop
            async with real_clock_timeout():
                # Pass DEBUG environment variable for better output on crash
                return await trio.to_thread.run_sync(
                    lambda: runner.invoke(cli, cmd, env={"DEBUG": "1"})
                )

        # Now we should have a new local device !
        result = await _cli_invoke_in_thread(
            f"core list_devices --config-dir={config_dir.as_posix()}"
        )

        async def run_review_pendings(extra_args: str = "", check_result: bool = True):
            result = await _cli_invoke_in_thread(
                f"core pki_enrollment_review_pendings --config-dir={config_dir.as_posix()} --device {alice.slughash} --password {alice_password} {extra_args}"
            )

            if not check_result:
                return result
            if result.exception:
                raise AssertionError(
                    f"CliRunner raise an exception: {result.exception}"
                ) from result.exception
            assert (
                result.exit_code == 0
            ), f"Bad exit_code: {result.exit_code}\nOutput: {result.output}"

            first_line, *other_lines = result.output.splitlines()
            match = re.match(r"^Found ([0-9]+) pending enrollment\(s\):", first_line)
            assert match
            enrollments_count = int(match.group(1))
            # Just retrieve the enrollment ID
            enrollments = []
            for line in other_lines:
                match = re.match(r"^Pending enrollment ([a-f0-9]+)", line)
                if match:
                    enrollments.append(match.group(1))
            assert len(enrollments) == enrollments_count
            return enrollments

        addr = BackendPkiEnrollmentAddr.build(backend_addr, organization_id=alice.organization_id)

        async def run_submit(extra_args: str = "", check_result: bool = True):
            result = await _cli_invoke_in_thread(
                f"core pki_enrollment_submit --config-dir={config_dir.as_posix()} {addr.to_url()} --device-label PC1 {extra_args}"
            )
            if not check_result:
                return result
            if result.exception:
                raise result.exception
            assert (
                result.exit_code == 0
            ), f"Bad exit_code: {result.exit_code}\nOutput: {result.output}"
            match = re.match(
                r"PKI enrollment ([a-f0-9]+) submitted", result.output.splitlines()[-1]
            )
            assert match
            return UUID(match.group(1))

        async def run_poll(extra_args: str = "", check_result: bool = True):
            result = await _cli_invoke_in_thread(
                f"core pki_enrollment_poll --config-dir={config_dir.as_posix()} --password S3cr3t {extra_args}"
            )
            if not check_result:
                return result
            if result.exception:
                raise AssertionError(
                    f"CliRunner raise an exception: {result.exception}"
                ) from result.exception
            assert (
                result.exit_code == 0
            ), f"Bad exit_code: {result.exit_code}\nOutput: {result.output}"

            first_line, *other_lines = result.output.splitlines()
            match = re.match(r"^Found ([0-9]+) pending enrollment\(s\):", first_line)
            assert match
            enrollments_count = int(match.group(1))
            # Just retrieve the enrollment ID
            enrollments = []
            for line in other_lines:
                match = re.match(r"^Pending enrollment ([a-f0-9]+)", line)
                if match:
                    enrollments.append(match.group(1))
            assert len(enrollments) == enrollments_count
            return enrollments

        # Time for testing !

        # List with no enrollments
        assert await run_review_pendings(extra_args="--list-only") == []

        # Poll with no local enrollments
        assert await run_poll() == []

        # New enrollment
        enrollment_id1 = await run_submit()
        assert await run_poll() == [enrollment_id1.hex[:3]]
        # Poll doesn't modify the pending enrollment
        assert await run_poll() == [enrollment_id1.hex[:3]]

        # List new enrollment
        assert await run_review_pendings(extra_args="--list-only") == [enrollment_id1.hex[:3]]

        # Try to reply enrollment without force
        result = await run_submit(check_result=False)
        assert result.exit_code == 1
        assert (
            f"The certificate `{mocked_parsec_ext_smartcard.default_x509_certificate.certificate_sha1.hex()}` has already been submitted"
            in result.output
        )
        assert await run_review_pendings(extra_args="--list-only") == [
            enrollment_id1.hex[:3]
        ]  # No change

        # Actually reply enrollment with force
        enrollment_id3 = await run_submit(extra_args="--force")
        assert await run_review_pendings(extra_args="--list-only") == [enrollment_id3.hex[:3]]

        # Reject enrollment
        await run_review_pendings(extra_args=f"--reject {enrollment_id3.hex}")
        assert await run_review_pendings(extra_args="--list-only") == []

        # Accept enrollment
        enrollment_id4 = await run_submit()
        await run_review_pendings(
            extra_args=f"--accept {enrollment_id4.hex} --pki-extra-trust-root {mocked_parsec_ext_smartcard.default_trust_root_path}"
        )
        assert await run_review_pendings(extra_args="--list-only") == []

        # It is no longer possible to do another enrollment with the same certificate (until the user is revoked)
        result = await run_submit(check_result=False)
        assert (
            f"The certificate `{mocked_parsec_ext_smartcard.default_x509_certificate.certificate_sha1.hex()}` has already been enrolled"
            in result.output
        )

        # Reject/Accept not possible against unknown/cancelled/accepted enrollments
        for extra_args in (
            # Unknown
            f"--reject e499f9aed05e4287875a177909d62d90",
            f"--accept e499f9aed05e4287875a177909d62d90",
            # Already Cancelled
            f"--reject {enrollment_id1.hex[:3]}",
            f"--accept {enrollment_id1.hex[:3]}",
            # Already Rejected
            f"--reject {enrollment_id3.hex[:3]}",
            f"--accept {enrollment_id3.hex[:3]}",
            # Already Accepted
            f"--reject {enrollment_id4.hex[:3]}",
            f"--accept {enrollment_id4.hex[:3]}",
        ):
            result = await run_review_pendings(extra_args=extra_args, check_result=False)
            assert result.exit_code == 1
            assert "Additional --accept/--reject elements not used" in result.output

        # Poll to handle the accepted enrollments, and discard the non-pendings ones
        ids = await run_poll(
            extra_args=f"--finalize {enrollment_id4.hex[:3]} --pki-extra-trust-root {mocked_parsec_ext_smartcard.default_trust_root_path}"
        )
        assert set(ids) == {enrollment_id1.hex[:3], enrollment_id3.hex[:3], enrollment_id4.hex[:3]}

        # Now we should have a new local device !
        result = await _cli_invoke_in_thread(
            f"core list_devices --config-dir={config_dir.as_posix()}"
        )
        print(result.output)
        assert result.exit_code == 0
        assert result.output.startswith("Found 2 device(s)")
        assert "CoolOrg: John Doe <john@example.com> @ PC1" in result.output

        # And all the enrollments should have been taken care of
        assert await run_poll() == []


def _setup_sequester_key_paths(tmp_path, coolorg):
    key_path = tmp_path / "keys"
    key_path.mkdir()
    service_key_path = key_path / "service.pem"
    authority_key_path = key_path / "authority.pem"
    authority_pubkey_path = key_path / "authority_pub.pem"
    service = sequester_service_factory("Test Service", coolorg.sequester_authority)
    service_key = service.encryption_key
    authority_key = coolorg.sequester_authority.signing_key
    service_key_path.write_bytes(oscrypto.asymmetric.dump_public_key(service_key))
    authority_key_path.write_bytes(
        oscrypto.asymmetric.dump_private_key(authority_key, passphrase=None)
    )
    authority_pubkey_path.write_bytes(
        oscrypto.asymmetric.dump_public_key(coolorg.sequester_authority.verify_key)
    )
    return authority_key_path, authority_pubkey_path, service_key_path


async def _cli_invoke_in_thread(runner: CliRunner, cmd: str, input: str = None):
    # We must run the command from another thread given it will create it own trio loop
    async with real_clock_timeout():
        # Pass DEBUG environment variable for better output on crash
        return await trio.to_thread.run_sync(
            lambda: runner.invoke(cli, cmd, input=input, env={"DEBUG": "1"})
        )


@pytest.mark.trio
@pytest.mark.postgresql
async def test_human_accesses(backend, alice, postgresql_url):
    async with cli_with_running_backend_testbed(backend, alice) as (backend_addr, alice):
        runner = CliRunner()

        cmd = f"backend human_accesses --db {postgresql_url} --db-min-connections 1 --db-max-connections 2 --organization {alice.organization_id.str}"

        result = await _cli_invoke_in_thread(runner, cmd)
        assert result.exit_code == 0
        assert result.output.startswith("Found 3 result(s)")

        # Also test with filter

        result = await _cli_invoke_in_thread(runner, f"{cmd} --filter alice")
        assert result.exit_code == 0
        assert result.output.startswith("Found 1 result(s)")


@pytest.mark.trio
@pytest.mark.postgresql
@customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_sequester(tmp_path, backend, coolorg, alice, postgresql_url):
    async with cli_with_running_backend_testbed(backend, alice) as (backend_addr, alice):
        runner = CliRunner()

        common_args = f"--db {postgresql_url} --db-min-connections 1 --db-max-connections 2 --organization {alice.organization_id.str}"

        async def run_list_services() -> CliResult:
            result = await _cli_invoke_in_thread(
                runner, f"backend sequester list_services {common_args}"
            )
            assert result.exit_code == 0
            return result

        async def generate_service_certificate(
            service_key_path: Path,
            authority_key_path: Path,
            service_label: str,
            output: Path,
            check_result: bool = True,
        ) -> CliResult:
            result = await _cli_invoke_in_thread(
                runner,
                f"backend sequester generate_service_certificate --service-label {service_label} --service-public-key {service_key_path} --authority-private-key {authority_key_path} --output {output}",
            )
            if check_result:
                assert result.exit_code == 0
            return result

        async def import_service_certificate(
            service_certificate_path: Path,
            extra_args: str = "",
            check_result: bool = True,
        ) -> CliResult:
            result = await _cli_invoke_in_thread(
                runner,
                f"backend sequester import_service_certificate {common_args} --service-certificate {service_certificate_path} {extra_args}",
            )
            if check_result:
                assert result.exit_code == 0
            return result

        async def create_service(
            service_key_path: Path,
            authority_key_path: Path,
            service_label: str,
            extra_args: str = "",
            check_result: bool = True,
        ) -> CliResult:
            result = await _cli_invoke_in_thread(
                runner,
                f"backend sequester create_service {common_args} --service-public-key {service_key_path} --authority-private-key {authority_key_path} --service-label {service_label} {extra_args}",
            )
            if check_result:
                assert result.exit_code == 0
            return result

        async def disable_service(service_id: str) -> CliResult:
            return await _cli_invoke_in_thread(
                runner,
                f"backend sequester update_service {common_args} --disable --service {service_id}",
            )

        async def enable_service(service_id: str) -> CliResult:
            return await _cli_invoke_in_thread(
                runner,
                f"backend sequester update_service {common_args} --enable --service {service_id}",
            )

        async def export_service(service_id: str, realm: RealmID, path: str) -> CliResult:
            return await _cli_invoke_in_thread(
                runner,
                f"backend sequester export_realm {common_args} --service {service_id} --realm {realm.str} --output {path} -b MOCKED",
            )

        # Assert no service configured
        result = await run_list_services()
        assert result.output == "Found 0 sequester service(s)\n"

        # Create service
        authority_key_path, authority_pubkey_path, service_key_path = _setup_sequester_key_paths(
            tmp_path, coolorg
        )
        service_label = "TestService"
        result = await create_service(service_key_path, authority_key_path, service_label)

        # List services
        result = await run_list_services()
        assert result.output.startswith("Found 1 sequester service(s)\n")
        assert service_label in result.output
        assert "Disabled on" not in result.output

        # Disable service
        match = re.search(r"Service TestService \(id: ([0-9a-f]+)\)", result.output, re.MULTILINE)
        assert match
        service_id = match.group(1)
        result = await disable_service(service_id)
        assert result.exit_code == 0
        result = await run_list_services()
        assert result.output.startswith("Found 1 sequester service(s)\n")
        assert service_label in result.output
        assert "Disabled on" in result.output

        # Service already disabled
        result = await disable_service(service_id)
        assert result.exit_code == 1
        assert isinstance(result.exception, SequesterServiceAlreadyDisabledError)

        # Re enable service
        result = await enable_service(service_id)
        assert result.exit_code == 0
        result = await run_list_services()
        assert result.output.startswith("Found 1 sequester service(s)\n")
        assert service_label in result.output
        assert "Disabled on" not in result.output

        # Service already enabled
        result = await enable_service(service_id)
        assert result.exit_code == 1
        assert isinstance(result.exception, SequesterServiceAlreadyEnabledError)

        # Export realm
        realms = await backend.realm.get_realms_for_user(alice.organization_id, alice.user_id)
        realm_id = list(realms.keys())[0]
        output_dir = tmp_path / "output"
        output_dir.mkdir()
        result = await export_service(service_id, realm_id, output_dir)
        files = list(output_dir.iterdir())
        assert len(files) == 1
        assert files[0].name.endswith(f"parsec-sequester-export-realm-{realm_id.str}.sqlite")

        # Create service using generate/import commands
        service_certif_pem = tmp_path / "certif.pem"
        service2_label = "TestService2"
        await generate_service_certificate(
            service_key_path, authority_key_path, service2_label, service_certif_pem
        )
        assert (
            "-----BEGIN PARSEC SEQUESTER SERVICE CERTIFICATE-----" in service_certif_pem.read_text()
        )
        await import_service_certificate(
            service_certif_pem,
        )

        # Import invalid sequester service certificate
        modify_index = len("-----BEGIN PARSEC SEQUESTER SERVICE CERTIFICATE-----\n") + 1
        pem_content = bytearray(service_certif_pem.read_bytes())
        pem_content[modify_index] = 0 if pem_content[modify_index] != 0 else 1
        service_certif_pem.write_bytes(pem_content)
        result = await import_service_certificate(
            service_certif_pem,
            check_result=False,
        )
        assert result.exit_code == 1

        # Create webhook service
        result = await create_service(
            service_key_path,
            authority_key_path,
            "WebhookService",
            extra_args="--service-type webhook --webhook-url http://nowhere.lost",
        )
        services = await backend.sequester.get_organization_services(alice.organization_id)
        assert services[-1].service_type == SequesterServiceType.WEBHOOK
        assert services[-1].webhook_url

        # Create webhook service but forget webook URL
        result = await create_service(
            service_key_path,
            authority_key_path,
            "BadWebhookService",
            extra_args="--service-type webhook",
            check_result=False,
        )
        assert result.exit_code == 1

        # Create non-webhook service but provide a webook URL
        result = await create_service(
            service_key_path,
            authority_key_path,
            "BadStorageService",
            extra_args="--service-type storage --webhook-url https://nowhere.lost",
            check_result=False,
        )
        assert result.exit_code == 1


@pytest.mark.trio
@pytest.mark.skipif(
    sys.platform == "win32", reason="Key files paths are expected to be in linux posix format"
)
@customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_bootstrap_sequester(coolorg, tmp_path, backend, running_backend):
    org = "TheOne"
    config_dir = tmp_path / "config"
    config_dir.mkdir()
    device_label = "device_label"
    human_label = "human_label"
    human_email = "human@email.com"
    password = "P@ssw0rd."
    runner = CliRunner()

    _, _, public_key = _setup_sequester_key_paths(tmp_path=tmp_path, coolorg=coolorg)

    async def create_organization():
        result = await _cli_invoke_in_thread(
            runner,
            f"core create_organization {org} --addr={running_backend.addr.to_url()}  --administration-token={backend.config.administration_token}",
        )
        assert result.exit_code == 0
        url = re.search(r"^Bootstrap organization url: (.*)$", result.output, re.MULTILINE).group(1)
        return url

    async def bootstrap_organization(url):
        result = await _cli_invoke_in_thread(
            runner,
            f"core bootstrap_organization  {url} --password={password} --device-label={device_label} --human-label={human_label} --human-email={human_email} --config-dir={config_dir.as_posix()} --sequester-verify-key={public_key}",
            input="y",
        )
        assert result.exit_code == 0
        return result

    url = await create_organization()
    result = await bootstrap_organization(url)
    assert "Saving device" in result.output

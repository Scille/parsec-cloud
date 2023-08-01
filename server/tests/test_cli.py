# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import os
import re
from functools import partial
from pathlib import Path

import click
import trio

try:
    import fcntl
except ModuleNotFoundError:  # Not available on Windows
    pass
import subprocess
import sys
from contextlib import asynccontextmanager, contextmanager
from time import sleep
from unittest.mock import patch

import attr
import pytest
import trustme
from click.testing import CliRunner
from click.testing import Result as CliResult

from parsec import __version__ as parsec_version
from parsec._parsec import (
    BackendAddr,
    BackendOrganizationAddr,
    DateTime,
)
from parsec.api.protocol import RealmID
from parsec.backend.postgresql import MigrationItem
from parsec.backend.sequester import (
    SequesterServiceAlreadyDisabledError,
    SequesterServiceAlreadyEnabledError,
    SequesterServiceType,
)
from parsec.cli import cli
from parsec.cli_utils import ParsecDateTimeClickType
from tests.common import (
    LocalDevice,
    asgi_app_handle_client_factory,
    customize_fixtures,
    real_clock_timeout,
    sequester_service_factory,
)

CWD = Path(__file__).parent.parent
# Starting parsec cli as a new subprocess can be very slow (typically a couple
# of seconds on my beafy machine !), so we use an unusually large value here to
# avoid issues in the CI.
SUBPROCESS_TIMEOUT = 30


@pytest.mark.parametrize(
    "args",
    (
        ["--version"],
        ["backend", "--version"],
        ["backend", "run", "--version"],
        ["backend", "sequester", "--version"],
        ["backend", "sequester", "list_services", "--version"],
    ),
    ids=[
        "root",
        "backend",
        "backend_run",
        "backend_sequester",
        "backend_sequester_list_services",
    ],
)
def test_version(args: list[str]):
    runner = CliRunner()
    result = runner.invoke(cli, args)
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
    ):
        pass


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


def _setup_sequester_key_paths(tmp_path, coolorg):
    key_path = tmp_path / "keys"
    key_path.mkdir()
    service_key_path = key_path / "service.pem"
    authority_key_path = key_path / "authority.pem"
    authority_pubkey_path = key_path / "authority_pub.pem"
    service = sequester_service_factory("Test Service", coolorg.sequester_authority)
    service_key = service.encryption_key
    authority_key = coolorg.sequester_authority.signing_key
    service_key_path.write_text(service_key.dump_pem())
    authority_key_path.write_text(authority_key.dump_pem())
    authority_pubkey_path.write_text(coolorg.sequester_authority.verify_key.dump_pem())
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
    async with cli_with_running_backend_testbed(backend, alice) as (_backend_addr, alice):
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
    async with cli_with_running_backend_testbed(backend, alice) as (_backend_addr, alice):
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
                f"backend sequester export_realm {common_args} --service {service_id} --realm {realm.hex} --output {path} -b MOCKED",
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
        assert files[0].name.endswith(f"parsec-sequester-export-realm-{realm_id.hex}.sqlite")

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

        # Create webhook service but forget webhook URL
        result = await create_service(
            service_key_path,
            authority_key_path,
            "BadWebhookService",
            extra_args="--service-type webhook",
            check_result=False,
        )
        assert result.exit_code == 1

        # Create non-webhook service but provide a webhook URL
        result = await create_service(
            service_key_path,
            authority_key_path,
            "BadStorageService",
            extra_args="--service-type storage --webhook-url https://nowhere.lost",
            check_result=False,
        )
        assert result.exit_code == 1

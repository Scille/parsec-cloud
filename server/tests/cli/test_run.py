# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import sys
from collections.abc import Generator
from dataclasses import dataclass
from pathlib import Path
from ssl import PROTOCOL_TLS_CLIENT, SSLContext
from time import sleep
from urllib.error import URLError
from urllib.request import urlopen

import pytest
import trustme

from tests.cli.common import cli_running
from tests.common.client import CoolorgRpcClients


@dataclass
class SSLConf:
    @property
    def use_ssl(self):
        return bool(self.backend_opts)

    backend_opts: str = ""
    ca_certfile: Path | None = None


@pytest.fixture(params=(False, True), ids=("no_ssl", "ssl"))
def ssl_conf(request: pytest.FixtureRequest) -> Generator[SSLConf, None, None]:
    if not request.param:
        yield SSLConf()
    else:
        ca = trustme.CA()
        server_cert = ca.issue_cert("127.0.0.1")
        with (
            ca.cert_pem.tempfile() as ca_certfile,
            server_cert.cert_chain_pems[0].tempfile() as server_certfile,
            server_cert.private_key_pem.tempfile() as server_keyfile,
        ):
            yield SSLConf(
                backend_opts=f" --ssl-keyfile={server_keyfile} --ssl-certfile={server_certfile} ",
                ca_certfile=Path(ca_certfile),
            )


@pytest.mark.skipif(sys.platform == "win32", reason="Hard to test on Windows...")
def test_run(coolorg: CoolorgRpcClients, unused_tcp_port: int, tmp_path: Path, ssl_conf: SSLConf):
    config_dir = tmp_path / "config"
    config_dir.mkdir()

    administration_token = "9e57754ddfe62f7f8780edc0"

    # Cannot use `click.CliRunner` in this test given it doesn't support
    # concurrent run of commands :'(

    print("######## START SERVER #########")
    with cli_running(
        (
            f"run --db=MOCKED --blockstore=MOCKED"
            f" --administration-token={administration_token}"
            f" --port={unused_tcp_port}"
            f" --server-addr=parsec3://127.0.0.1:{unused_tcp_port}"
            f" --email-host=MOCKED"
            f" --log-level=INFO"
            f" --fake-account-password-algorithm-seed=F4k3"
            f" {ssl_conf.backend_opts}"
        ),
        wait_for="Starting Parsec server",
    ):
        if ssl_conf.use_ssl:
            scheme = "https"
            client_ssl_context = SSLContext(protocol=PROTOCOL_TLS_CLIENT)
            client_ssl_context.load_verify_locations(ssl_conf.ca_certfile)
        else:
            scheme = "http"
            client_ssl_context = None

        retries = 0
        while True:
            try:
                rep = urlopen(
                    f"{scheme}://127.0.0.1:{unused_tcp_port}/", context=client_ssl_context
                )
            except URLError:
                # Connection refused might be due to the server not quiet ready yet...
                retries += 1
                if retries > 10:
                    raise
                sleep(0.1)
                continue

            assert rep.status == 200
            break

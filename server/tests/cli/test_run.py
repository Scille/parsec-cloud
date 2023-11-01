# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import sys
from dataclasses import dataclass, field

import pytest
import trustme

from tests.cli.common import cli_running


@pytest.fixture(params=(False, True), ids=("no_ssl", "ssl"))
def ssl_conf(request: pytest.FixtureRequest):
    @dataclass
    class SSLConf:
        @property
        def use_ssl(self):
            return bool(self.backend_opts)

        client_env: dict = field(default_factory=dict)
        backend_opts: str = ""

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
def test_run(coolorg, unused_tcp_port, tmp_path, ssl_conf):
    config_dir = tmp_path / "config"
    config_dir.mkdir()

    administration_token = "9e57754ddfe62f7f8780edc0"

    # Cannot use `click.CliRunner` in this test given it doesn't support
    # concurrent run of commands :'(

    print("######## START BACKEND #########")
    with cli_running(
        (
            f"run --db=MOCKED --blockstore=MOCKED"
            f" --administration-token={administration_token}"
            f" --port=0"
            f" --backend-addr=parsec://127.0.0.1:{unused_tcp_port}"
            f" --email-host=MOCKED"
            f" --log-level=INFO"
            f" {ssl_conf.backend_opts}"
        ),
        wait_for="Starting Parsec Backend",
    ):
        # TODO: send a request to ensure the server is correctly listening
        pass

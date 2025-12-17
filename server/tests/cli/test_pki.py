# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import os
from pathlib import Path

import click
from click.testing import CliRunner

from parsec._parsec import TrustAnchor
from parsec.cli.pki import pki_server_options
from tests.common.pki import TestPki


@click.command
@pki_server_options
def mini_app(trusted_x509_root_dir: list[TrustAnchor]):
    click.echo(f"Found {len(trusted_x509_root_dir)} anchors")


def test_trust_anchor_opt(test_pki: TestPki):
    runner = CliRunner(catch_exceptions=False)
    with runner.isolated_filesystem():
        TRUST_ANCHOR_DIR = Path("pki")
        os.mkdir(TRUST_ANCHOR_DIR)
        for root in test_pki.root.values():
            os.symlink(root.certificate.path, TRUST_ANCHOR_DIR / root.certificate.path.name)
        result = runner.invoke(mini_app, ["--trusted-x509-root-dir", str(TRUST_ANCHOR_DIR)])
        assert result.exit_code == 0
        assert result.stdout == f"Found {len(test_pki.root)} anchors\n"


def test_not_trust_anchor():
    runner = CliRunner(catch_exceptions=False)
    with runner.isolated_filesystem():
        os.mkdir("empty_dir")
        result = runner.invoke(mini_app, ["--trusted-x509-root-dir", "empty_dir"])
        assert result.exit_code == 0
        assert result.stdout == "Found 0 anchors\n"

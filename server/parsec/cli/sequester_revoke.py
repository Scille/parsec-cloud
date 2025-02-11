# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
import textwrap
from base64 import b64decode, b64encode
from pathlib import Path
from typing import Any

import click

from parsec._parsec import (
    CryptoError,
    DateTime,
    OrganizationID,
    SequesterRevokedServiceCertificate,
    SequesterServiceID,
    SequesterSigningKeyDer,
    SequesterVerifyKeyDer,
)
from parsec.ballpark import RequireGreaterTimestamp
from parsec.cli.options import db_server_options, debug_config_options, logging_config_options
from parsec.cli.testbed import if_testbed_available
from parsec.cli.utils import cli_exception_handler, start_backend
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.sequester import (
    SequesterRevokeServiceStoreBadOutcome,
    SequesterRevokeServiceValidateBadOutcome,
)
from parsec.config import BaseDatabaseConfig, DisabledBlockStoreConfig, LogLevel

SEQUESTER_SERVICE_REVOCATION_CERTIFICATE_PEM_HEADER = (
    "-----BEGIN PARSEC SEQUESTER SERVICE REVOCATION CERTIFICATE-----"
)
SEQUESTER_SERVICE_REVOCATION_CERTIFICATE_PEM_FOOTER = (
    "-----END PARSEC SEQUESTER SERVICE REVOCATION CERTIFICATE-----"
)


def _dump_sequester_service_revocation_certificate_pem(
    certificate: SequesterRevokedServiceCertificate,
    authority_signing_key: SequesterSigningKeyDer,
) -> str:
    signed = authority_signing_key.sign(certificate.dump())
    return "\n".join(
        (
            SEQUESTER_SERVICE_REVOCATION_CERTIFICATE_PEM_HEADER,
            *textwrap.wrap(b64encode(signed).decode(), width=64),
            SEQUESTER_SERVICE_REVOCATION_CERTIFICATE_PEM_FOOTER,
            "",
        )
    )


def _load_sequester_service_revocation_certificate_pem(
    pem: str, authority_verify_key: SequesterVerifyKeyDer
) -> tuple[SequesterRevokedServiceCertificate, bytes]:
    err_msg = "Not a valid Parsec sequester service revocation certificate PEM file"
    try:
        header, *content, footer = pem.strip().splitlines()
    except ValueError as exc:
        raise ValueError(err_msg) from exc

    if header != SEQUESTER_SERVICE_REVOCATION_CERTIFICATE_PEM_HEADER:
        raise ValueError(
            f"{err_msg}: missing `{SEQUESTER_SERVICE_REVOCATION_CERTIFICATE_PEM_HEADER}` header"
        )
    if footer != SEQUESTER_SERVICE_REVOCATION_CERTIFICATE_PEM_FOOTER:
        raise ValueError(
            f"{err_msg}: missing `{SEQUESTER_SERVICE_REVOCATION_CERTIFICATE_PEM_FOOTER}` footer"
        )

    try:
        signed = b64decode("".join(content))
        return (
            SequesterRevokedServiceCertificate.load(authority_verify_key.verify(signed)),
            signed,
        )
    except (ValueError, CryptoError) as exc:
        raise ValueError(f"{err_msg}: invalid body ({exc})") from exc


class SequesterBackendCliError(Exception):
    pass


class GenerateServiceRevocationCertificateDevOption(click.Option):
    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        if value:
            import os
            import tempfile

            from parsec._parsec import testbed

            template_content = testbed.test_get_testbed_template("sequestered")
            assert template_content is not None
            event = template_content.events[0]
            assert isinstance(event, testbed.TestbedEventBootstrapOrganization)
            assert event.sequester_authority_signing_key is not None

            # Note this file is not deleted when the application ends, this is considered
            # okay since it is only used for niche testing purpose.
            file_fd, file_path = tempfile.mkstemp()
            os.write(file_fd, event.sequester_authority_signing_key.dump_pem().encode("utf8"))
            os.close(file_fd)

            for key, value in (
                ("debug", True),
                ("authority_private_key", file_path),
            ):
                if key not in opts:
                    opts[key] = value

        return value, args


@click.command(short_help="Generate a certificate for a new sequester service")
@click.option(
    "--service-id",
    type=SequesterServiceID.from_hex,
    required=True,
)
@click.option(
    "--authority-private-key",
    help="File containing the private authority key use. Used to sign the encryption key.",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
    required=True,
)
@click.argument("output", required=False)
# Add --debug & --version
@debug_config_options
@if_testbed_available(
    click.option(
        "--dev",
        cls=GenerateServiceRevocationCertificateDevOption,
        is_flag=True,
        is_eager=True,
        help=(
            "Equivalent to `--debug --authority-private-key=<tempfile of sequestered testbed template's authority key>`"
        ),
    )
)
def generate_service_revocation_certificate(
    service_id: SequesterServiceID,
    authority_private_key: Path,
    output: str | None,
    debug: bool,
    dev: bool = False,
) -> None:
    output = output or str(Path.cwd())

    with cli_exception_handler(debug):
        # 1) Load key files

        authority_key = SequesterSigningKeyDer.load_pem(authority_private_key.read_text())

        # 2) Generate certificate

        timestamp = DateTime.now()
        certificate = SequesterRevokedServiceCertificate(
            timestamp=timestamp,
            service_id=service_id,
        )

        # 3) Write the certificate as PEM in output file

        pem_content = _dump_sequester_service_revocation_certificate_pem(
            certificate=certificate,
            authority_signing_key=authority_key,
        )

        cooked_output = Path(output)
        if cooked_output.is_dir():
            output_file = (
                cooked_output
                / f"sequester_service_revocation_certificate-{service_id.hex}-{timestamp.to_rfc3339()}.pem"
            )
        else:
            output_file = cooked_output
        output_file.write_bytes(pem_content.encode("utf8"))

        display_service = f"(sequester service ID: {click.style(service_id.hex, fg='yellow')}, timestamp: {click.style(timestamp, fg='yellow')})"
        display_file = click.style(output_file, fg="green")
        click.echo(
            f"Sequester service revocation certificate {display_service} exported in {display_file}"
        )
        click.echo(
            f"Use {click.style('parsec sequester revoke_service', fg='yellow')} command to add it to an organization"
        )


class RevokeServiceDevOption(click.Option):
    def handle_parse_result(
        self, ctx: click.Context, opts: Any, args: list[str]
    ) -> tuple[Any, list[str]]:
        value, args = super().handle_parse_result(ctx, opts, args)
        if value:
            for key, value in (
                ("debug", True),
                ("db", "MOCKED"),
                ("with_testbed", "sequestered"),
                ("organization", "SequesteredOrgTemplate"),
            ):
                if key not in opts:
                    opts[key] = value

        return value, args


@click.command(short_help="Create a new sequester service from its existing certificate")
@click.option(
    "--service-revocation-certificate",
    help="File containing the sequester service certificate (previously generated by `parsec sequester generate_service_revocation_certificate` command).",
    type=click.File("r", encoding="utf8"),
    required=True,
    metavar="CERTIFICATE.pem",
)
@click.option(
    "--organization",
    type=OrganizationID,
    help="Organization ID where to register the service",
    required=True,
)
@db_server_options
# Add --log-level/--log-format/--log-file
@logging_config_options(default_log_level="INFO")
# Add --debug & --version
@debug_config_options
@if_testbed_available(
    click.option("--with-testbed", help="Start by populating with a testbed template")
)
@if_testbed_available(
    click.option(
        "--dev",
        cls=RevokeServiceDevOption,
        is_flag=True,
        is_eager=True,
        help=(
            "Equivalent to `--debug --db=MOCKED --with-testbed=sequestered --organization SequesteredOrgTemplate`"
        ),
    )
)
def revoke_service(
    service_revocation_certificate: click.utils.LazyFile,
    organization: OrganizationID,
    db: BaseDatabaseConfig,
    db_max_connections: int,
    db_min_connections: int,
    log_level: LogLevel,
    log_format: str,
    log_file: str | None,
    debug: bool,
    with_testbed: str | None = None,
    dev: bool = False,
) -> None:
    debug = False
    with cli_exception_handler(debug):
        service_revocation_certificate_pem = service_revocation_certificate.read()

        asyncio.run(
            _revoke_service(
                db_config=db,
                debug=debug,
                with_testbed=with_testbed,
                organization_id=organization,
                service_revocation_certificate_pem=service_revocation_certificate_pem,
            )
        )
        click.echo(click.style("Service revoked", fg="green"))


async def _revoke_service(
    db_config: BaseDatabaseConfig,
    debug: bool,
    with_testbed: str | None,
    organization_id: OrganizationID,
    service_revocation_certificate_pem: str,
) -> None:
    # Can use a dummy blockstore config since we are not going to query it
    blockstore_config = DisabledBlockStoreConfig()

    async with start_backend(
        db_config=db_config,
        blockstore_config=blockstore_config,
        debug=debug,
        populate_with_template=with_testbed,
    ) as backend:
        # 1) Retrieve the organization and check it is compatible

        outcome = await backend.organization.get(organization_id)
        match outcome:
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                raise RuntimeError("Organization doesn't exist")

        if not org.is_bootstrapped:
            raise RuntimeError("Organization is not bootstrapped")

        if not org.is_sequestered:
            raise RuntimeError("Organization is not sequestered")

        # 2) Validate the certificate

        assert org.sequester_authority_verify_key_der is not None

        (
            revoked_service_cooked,
            revoked_service_raw,
        ) = _load_sequester_service_revocation_certificate_pem(
            pem=service_revocation_certificate_pem,
            authority_verify_key=org.sequester_authority_verify_key_der,
        )

        # 3) Insert the certificate

        outcome = await backend.sequester.revoke_service(
            now=DateTime.now(),
            organization_id=organization_id,
            revoked_service_certificate=revoked_service_raw,
        )

        match outcome:
            case SequesterRevokedServiceCertificate():
                pass

            case RequireGreaterTimestamp() as err:
                raise RuntimeError(
                    f"Cannot import certificate since its timestamp (`{revoked_service_cooked.timestamp}`) is older "
                    f"than the most recent sequester certificate already on the on server (`{err.strictly_greater_than}`)"
                )

            case SequesterRevokeServiceStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                raise RuntimeError("Organization doesn't exist")

            case SequesterRevokeServiceStoreBadOutcome.SEQUESTER_DISABLED:
                raise RuntimeError("Organization is not sequestered")

            case SequesterRevokeServiceStoreBadOutcome.SEQUESTER_SERVICE_NOT_FOUND:
                raise RuntimeError("Sequester service not found")

            case SequesterRevokeServiceStoreBadOutcome.SEQUESTER_SERVICE_ALREADY_REVOKED:
                raise RuntimeError("Sequester service already revoked")

            # Should never occur since we have already checked the validity at step 2
            case SequesterRevokeServiceValidateBadOutcome.INVALID_CERTIFICATE:
                assert False

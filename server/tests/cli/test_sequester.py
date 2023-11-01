# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import re
from pathlib import Path

import pytest
from click.testing import CliRunner
from click.testing import Result as CliResult

from parsec._parsec import (
    VlobID,
)
from parsec.components.sequester import (
    SequesterServiceType,
)
from tests.cli.common import cli_invoke_in_thread, cli_with_running_backend_testbed

# TODO
# from tests.common import (
#     customize_fixtures,
#     sequester_service_factory,
# )


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


@pytest.mark.xfail(reason="customize_fixture is not implemented")
@pytest.mark.asyncio
@pytest.mark.postgresql
# @customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_sequester(tmp_path, backend, coolorg, alice, postgresql_url):
    async with cli_with_running_backend_testbed(backend, alice) as (_backend_addr, alice):
        runner = CliRunner()

        common_args = f"--db {postgresql_url} --db-min-connections 1 --db-max-connections 2 --organization {alice.organization_id.str}"

        async def run_list_services() -> CliResult:
            result = await cli_invoke_in_thread(runner, f"sequester list_services {common_args}")
            assert result.exit_code == 0
            return result

        async def generate_service_certificate(
            service_key_path: Path,
            authority_key_path: Path,
            service_label: str,
            output: Path,
            check_result: bool = True,
        ) -> CliResult:
            result = await cli_invoke_in_thread(
                runner,
                f"sequester generate_service_certificate --service-label {service_label} --service-public-key {service_key_path} --authority-private-key {authority_key_path} --output {output}",
            )
            if check_result:
                assert result.exit_code == 0
            return result

        async def import_service_certificate(
            service_certificate_path: Path,
            extra_args: str = "",
            check_result: bool = True,
        ) -> CliResult:
            result = await cli_invoke_in_thread(
                runner,
                f"sequester import_service_certificate {common_args} --service-certificate {service_certificate_path} {extra_args}",
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
            result = await cli_invoke_in_thread(
                runner,
                f"sequester create_service {common_args} --service-public-key {service_key_path} --authority-private-key {authority_key_path} --service-label {service_label} {extra_args}",
            )
            if check_result:
                assert result.exit_code == 0
            return result

        async def disable_service(service_id: str) -> CliResult:
            return await cli_invoke_in_thread(
                runner,
                f"sequester update_service {common_args} --disable --service {service_id}",
            )

        async def enable_service(service_id: str) -> CliResult:
            return await cli_invoke_in_thread(
                runner,
                f"sequester update_service {common_args} --enable --service {service_id}",
            )

        async def export_service(service_id: str, realm: VlobID, path: str) -> CliResult:
            return await cli_invoke_in_thread(
                runner,
                f"sequester export_realm {common_args} --service {service_id} --realm {realm.hex} --output {path} -b MOCKED",
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
        realm_id = next(iter(realms.keys()))
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

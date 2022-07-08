# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest


from tests.common import OrganizationFullData, customize_fixtures, sequester_service_factory

from tests.test_cli import _running
import oscrypto.asymmetric


@pytest.mark.trio
@pytest.mark.postgresql
@customize_fixtures(backend_not_populated=False)
@customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_sequester_backend_cli(
    postgresql_url, coolorg: OrganizationFullData, running_backend, backend, tmp_path
):
    common_args = f"--db {postgresql_url} --db-min-connections 1 --db-max-connections 2 --organization {coolorg.organization_id}"
    with _running(f"backend list_services {common_args}") as list_services:
        output = list_services.stdout.readlines()
    assert output == [b"No service configured\n"]

    key_path = tmp_path / "keys"
    key_path.mkdir()
    service_key_path = key_path / "service.pem"
    authority_key_path = key_path / "authority.pem"

    service = sequester_service_factory("Test Service", coolorg.sequester_authority)

    service_label = "TestService"
    service_key = service.encryption_key
    authority_key = coolorg.sequester_authority.signing_key
    service_key_path.write_bytes(oscrypto.asymmetric.dump_public_key(service_key))
    authority_key_path.write_bytes(
        oscrypto.asymmetric.dump_private_key(authority_key, passphrase=None)
    )

    with _running(
        f"backend create_service {common_args} --service-public-key {service_key_path} --authority-private-key {authority_key_path} --service-label {service_label}"
    ) as register_service:
        print(register_service.stdout.read())

    common_args = f"--db {postgresql_url} --db-min-connections 1 --db-max-connections 2 --organization {coolorg.organization_id}"
    with _running(f"backend list_services {common_args}") as list_services:
        output = list_services.stdout.readlines()
    print(output, type(output[0]), output[0][0], output[0][:4])
    print(str(output))

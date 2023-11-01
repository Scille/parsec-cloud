# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# TODO: test organization_already_bootstrapped is not returned if a invalid token is provided
import pytest

from parsec._parsec import BootstrapToken, DateTime, OrganizationID, anonymous_cmds
from parsec._parsec import testbed as tb
from tests.common import AnonymousRpcClient, AsyncClient, Backend, TestbedBackend


@pytest.mark.parametrize("kind", ("standard", "sequestered", "spontaneous"))
async def test_anonymous_organization_bootstrap_ok(
    kind: str,
    testbed: TestbedBackend,
    backend: Backend,
    client: AsyncClient,
    ballpark_always_ok: None,
) -> None:
    match kind:
        case "standard":
            spontaneous = False
            sequestered = False
        case "sequestered":
            spontaneous = False
            sequestered = True
        case "spontaneous":
            spontaneous = True
            sequestered = False
        case unknown:
            assert False, unknown

    # 1) Create the organization to bootstrap
    organization_id = OrganizationID("NewOrg")
    if spontaneous:
        bootstrap_token = None
        backend.config.organization_spontaneous_bootstrap = True
    else:
        bootstrap_token = await backend.organization.create(now=DateTime.now(), id=organization_id)
        assert isinstance(bootstrap_token, BootstrapToken), bootstrap_token

    # 2) To do the bootstrap we need certificates, so steal them from another organization
    _, _, coolorg_content = await testbed.get_template("coolorg")
    bootstrap_event = coolorg_content.events[0]
    assert isinstance(bootstrap_event, tb.TestbedEventBootstrapOrganization)
    user_certificate = bootstrap_event.first_user_raw_certificate
    redacted_user_certificate = bootstrap_event.first_user_raw_redacted_certificate
    device_certificate = bootstrap_event.first_user_first_device_raw_certificate
    redacted_device_certificate = bootstrap_event.first_user_first_device_raw_redacted_certificate
    root_verify_key = bootstrap_event.root_signing_key.verify_key
    if sequestered:
        pytest.xfail(reason="TODO: no sequestered template so far !")
    else:
        sequester_authority_certificate = None

    rpc = AnonymousRpcClient(client, organization_id)

    rep = await rpc.organization_bootstrap(
        bootstrap_token=bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    )

    assert rep == anonymous_cmds.v4.organization_bootstrap.RepOk()

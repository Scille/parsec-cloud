# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections.abc import Awaitable
from dataclasses import dataclass
from typing import Protocol

import pytest

from parsec._parsec import BootstrapToken, DateTime, OrganizationID, anonymous_cmds
from parsec._parsec import testbed as tb
from tests.common import AnonymousRpcClient, AsyncClient, Backend, TestbedBackend


@pytest.fixture
def organization_id() -> OrganizationID:
    return OrganizationID("NewOrg")


@pytest.fixture
def anonymous_client(client: AsyncClient, organization_id: OrganizationID) -> AnonymousRpcClient:
    return AnonymousRpcClient(client, organization_id)


@dataclass
class BackendPreConfig:
    spontaneous: bool
    sequestered: bool


@dataclass
class BackendConfig:
    spontaneous: bool
    sequestered: bool
    bootstrap_token: BootstrapToken | None


class ConfigureBackend(Protocol):
    def __call__(self, organization_id: OrganizationID) -> Awaitable[BackendConfig]:
        ...


@pytest.fixture(
    params=[
        pytest.param(BackendPreConfig(spontaneous=False, sequestered=False), id="standard"),
        pytest.param(BackendPreConfig(spontaneous=False, sequestered=True), id="sequestered"),
        pytest.param(BackendPreConfig(spontaneous=True, sequestered=False), id="spontaneous"),
    ]
)
def backend_bootstrap_config(backend: Backend, request: pytest.FixtureRequest) -> ConfigureBackend:
    config: BackendPreConfig = request.param

    async def _backend_bootstrap_config(organization_id: OrganizationID) -> BackendConfig:
        """
        Configure the backend to either allow spontaneous bootstrapping or not.
        When bootstrapping is not spontaneous, a bootstrap token is created and returned.
        """
        if config.spontaneous:
            bootstrap_token = None
            backend.config.organization_spontaneous_bootstrap = True
        else:
            bootstrap_token = await backend.organization.create(
                now=DateTime.now(), id=organization_id
            )
            assert isinstance(bootstrap_token, BootstrapToken), bootstrap_token
        return BackendConfig(
            spontaneous=config.spontaneous,
            sequestered=config.sequestered,
            bootstrap_token=bootstrap_token,
        )

    return _backend_bootstrap_config


@pytest.mark.usefixtures("ballpark_always_ok")
async def test_ok(
    organization_id: OrganizationID,
    backend_bootstrap_config: ConfigureBackend,
    testbed: TestbedBackend,
    anonymous_client: AnonymousRpcClient,
) -> None:
    config = await backend_bootstrap_config(organization_id)

    # 1) To do the bootstrap we need certificates, so steal them from another organization
    _, _, coolorg_content = await testbed.get_template("coolorg")
    bootstrap_event = coolorg_content.events[0]
    assert isinstance(bootstrap_event, tb.TestbedEventBootstrapOrganization)
    user_certificate = bootstrap_event.first_user_raw_certificate
    redacted_user_certificate = bootstrap_event.first_user_raw_redacted_certificate
    device_certificate = bootstrap_event.first_user_first_device_raw_certificate
    redacted_device_certificate = bootstrap_event.first_user_first_device_raw_redacted_certificate
    root_verify_key = bootstrap_event.root_signing_key.verify_key
    if config.sequestered:
        pytest.xfail(reason="TODO: no sequestered template so far !")
    else:
        sequester_authority_certificate = None

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=config.bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    )

    assert rep == anonymous_cmds.v4.organization_bootstrap.RepOk()


@pytest.mark.usefixtures("ballpark_always_ok")
async def test_organization_already_bootstrapped(
    organization_id: OrganizationID,
    backend_bootstrap_config: ConfigureBackend,
    testbed: TestbedBackend,
    anonymous_client: AnonymousRpcClient,
) -> None:
    config = await backend_bootstrap_config(organization_id)

    # 1) To do the bootstrap we need certificates, so steal them from another organization
    _, _, coolorg_content = await testbed.get_template("coolorg")
    bootstrap_event = coolorg_content.events[0]
    assert isinstance(bootstrap_event, tb.TestbedEventBootstrapOrganization)
    user_certificate = bootstrap_event.first_user_raw_certificate
    redacted_user_certificate = bootstrap_event.first_user_raw_redacted_certificate
    device_certificate = bootstrap_event.first_user_first_device_raw_certificate
    redacted_device_certificate = bootstrap_event.first_user_first_device_raw_redacted_certificate
    root_verify_key = bootstrap_event.root_signing_key.verify_key
    if config.sequestered:
        pytest.xfail(reason="TODO: no sequestered template so far !")
    else:
        sequester_authority_certificate = None

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=config.bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    )

    assert rep == anonymous_cmds.v4.organization_bootstrap.RepOk()

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=config.bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    )

    assert rep == anonymous_cmds.v4.organization_bootstrap.RepOrganizationAlreadyBootstrapped()


@pytest.mark.usefixtures("ballpark_always_ok")
async def test_invalid_bootstrap_token(
    organization_id: OrganizationID,
    backend_bootstrap_config: ConfigureBackend,
    testbed: TestbedBackend,
    anonymous_client: AnonymousRpcClient,
) -> None:
    config = await backend_bootstrap_config(organization_id)

    # 1) To do the bootstrap we need certificates, so steal them from another organization
    _, _, coolorg_content = await testbed.get_template("coolorg")
    bootstrap_event = coolorg_content.events[0]
    assert isinstance(bootstrap_event, tb.TestbedEventBootstrapOrganization)
    user_certificate = bootstrap_event.first_user_raw_certificate
    redacted_user_certificate = bootstrap_event.first_user_raw_redacted_certificate
    device_certificate = bootstrap_event.first_user_first_device_raw_certificate
    redacted_device_certificate = bootstrap_event.first_user_first_device_raw_redacted_certificate
    root_verify_key = bootstrap_event.root_signing_key.verify_key
    if config.sequestered:
        pytest.xfail(reason="TODO: no sequestered template so far !")
    else:
        sequester_authority_certificate = None

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=BootstrapToken.new(),
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    )

    assert rep == anonymous_cmds.v4.organization_bootstrap.RepInvalidBootstrapToken()

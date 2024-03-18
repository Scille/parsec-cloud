# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections.abc import Awaitable
from dataclasses import dataclass
from typing import Protocol

import pytest

from parsec._parsec import (
    BootstrapToken,
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    HumanHandle,
    OrganizationID,
    PrivateKey,
    SequesterAuthorityCertificate,
    SequesterSigningKeyDer,
    SigningKey,
    UserCertificate,
    UserProfile,
    anonymous_cmds,
)
from parsec._parsec import testbed as tb
from parsec.ballpark import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET
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
    def __call__(self, organization_id: OrganizationID) -> Awaitable[BackendConfig]: ...


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


@dataclass
class OverrideCertifData:
    user: bytes | None = None
    device: bytes | None = None
    abr_user: bytes | None = None
    abr_device: bytes | None = None
    sequester: bytes | None = None


@pytest.mark.parametrize(
    "override_certif_data",
    [
        pytest.param(OverrideCertifData(user=b"foobar"), id="user_certificate"),
        pytest.param(OverrideCertifData(device=b"foobar"), id="device_certificate"),
        pytest.param(OverrideCertifData(abr_user=b"foobar"), id="redacted_user_certificate"),
        pytest.param(OverrideCertifData(abr_device=b"foobar"), id="redacted_device_certificate"),
        pytest.param(OverrideCertifData(sequester=b"foobar"), id="sequester_certificate"),
    ],
)
@pytest.mark.usefixtures("ballpark_always_ok")
async def test_invalid_certificate(
    override_certif_data: OverrideCertifData,
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
    user_certificate = override_certif_data.user or bootstrap_event.first_user_raw_certificate
    redacted_user_certificate = (
        override_certif_data.abr_user or bootstrap_event.first_user_raw_redacted_certificate
    )
    device_certificate = (
        override_certif_data.device or bootstrap_event.first_user_first_device_raw_certificate
    )
    redacted_device_certificate = (
        override_certif_data.abr_device
        or bootstrap_event.first_user_first_device_raw_redacted_certificate
    )
    root_verify_key = bootstrap_event.root_signing_key.verify_key
    if override_certif_data.sequester:
        sequester_authority_certificate = override_certif_data.sequester
    else:
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

    assert rep == anonymous_cmds.v4.organization_bootstrap.RepInvalidCertificate()


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


@pytest.mark.parametrize(
    [
        "user_certif_late",
        "device_certif_late",
        "sequester_auth_certif_late",
    ],
    [
        # User and device certificate should have the same timestamp
        pytest.param(True, True, False, id="user_n_device_certificate_late"),
        pytest.param(False, False, True, id="sequester_certificate_late"),
    ],
)
@pytest.mark.parametrize(
    "ballpark_offset",
    [
        pytest.param(BALLPARK_CLIENT_EARLY_OFFSET + 5, id="early"),
        pytest.param(-BALLPARK_CLIENT_LATE_OFFSET - 5, id="late"),
    ],
)
async def test_timestamp_out_of_ballpark(
    user_certif_late: bool,
    device_certif_late: bool,
    sequester_auth_certif_late: bool,
    ballpark_offset: int,
    organization_id: OrganizationID,
    backend_bootstrap_config: ConfigureBackend,
    anonymous_client: AnonymousRpcClient,
) -> None:
    config = await backend_bootstrap_config(organization_id)

    now = DateTime.now()
    late = now.add(seconds=ballpark_offset)

    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key

    device_id = DeviceID.new()
    user_id = device_id.user_id
    user_priv_key = PrivateKey.generate()
    user_human_handle = HumanHandle("foo@bar.com", str(user_id))
    device_label = DeviceLabel("device label")

    user_certificate = UserCertificate(
        author=None,
        timestamp=late if user_certif_late else now,
        user_id=user_id,
        human_handle=user_human_handle,
        public_key=user_priv_key.public_key,
        profile=UserProfile.ADMIN,
    )
    redacted_user_certificate = UserCertificate(
        author=user_certificate.author,
        timestamp=user_certificate.timestamp,
        user_id=user_certificate.user_id,
        human_handle=None,
        public_key=user_certificate.public_key,
        profile=user_certificate.profile,
    )
    device_certificate = DeviceCertificate(
        author=device_id,
        timestamp=late if device_certif_late else now,
        device_id=device_id,
        device_label=device_label,
        verify_key=root_verify_key,
    )
    redacted_device_certificate = DeviceCertificate(
        author=device_certificate.author,
        timestamp=device_certificate.timestamp,
        device_id=device_certificate.device_id,
        device_label=None,
        verify_key=device_certificate.verify_key,
    )

    _, sequester_verify_key = SequesterSigningKeyDer.generate_pair(1024)

    sequester_authority_certificate = SequesterAuthorityCertificate(
        timestamp=late if sequester_auth_certif_late else now,
        verify_key_der=sequester_verify_key,
    )

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=config.bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate.dump_and_sign(root_signing_key),
        device_certificate=device_certificate.dump_and_sign(root_signing_key),
        redacted_user_certificate=redacted_user_certificate.dump_and_sign(root_signing_key),
        redacted_device_certificate=redacted_device_certificate.dump_and_sign(root_signing_key),
        sequester_authority_certificate=sequester_authority_certificate.dump_and_sign(
            root_signing_key
        )
        if sequester_authority_certificate
        else None,
    )

    assert isinstance(rep, anonymous_cmds.v4.organization_bootstrap.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == BALLPARK_CLIENT_EARLY_OFFSET
    assert rep.ballpark_client_late_offset == BALLPARK_CLIENT_LATE_OFFSET
    assert rep.client_timestamp == late
    # We don't compare the server timestamp as it's not deterministic, should we freeze time ?

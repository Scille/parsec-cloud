# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections.abc import Awaitable
from dataclasses import dataclass
from typing import Protocol
from unittest.mock import ANY

import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    BootstrapToken,
    DateTime,
    DeviceLabel,
    EmailAddress,
    HumanHandle,
    OrganizationID,
    SequesterAuthorityCertificate,
    SequesterSigningKeyDer,
    SigningKey,
    UserProfile,
    anonymous_cmds,
)
from parsec._parsec import testbed as tb
from parsec.ballpark import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET
from parsec.components.organization import TermsOfService
from tests.common import (
    AnonymousRpcClient,
    AsyncClient,
    Backend,
    TestbedBackend,
    next_organization_id,
)
from tests.common.utils import generate_new_device_certificates, generate_new_user_certificates


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
            # Note we also configure an TOS here, this is to make sure this has no impact
            # on organization bootstrap (as it should be only checked  on user authentication).
            bootstrap_token = await backend.organization.create(
                now=DateTime.now(),
                id=organization_id,
                tos={"cn_HK": "https://parsec.invalid/tos_cn.pdf"},
            )
            assert isinstance(bootstrap_token, BootstrapToken), bootstrap_token
        return BackendConfig(
            spontaneous=config.spontaneous,
            sequestered=config.sequestered,
            bootstrap_token=bootstrap_token,
        )

    return _backend_bootstrap_config


@pytest.mark.usefixtures("ballpark_always_ok")
async def test_anonymous_organization_bootstrap_ok(
    backend_bootstrap_config: ConfigureBackend,
    client: AsyncClient,
    backend: Backend,
    testbed: TestbedBackend,
) -> None:
    organization_id = next_organization_id(prefix="NewOrg")
    anonymous_client = AnonymousRpcClient(client, organization_id)

    config = await backend_bootstrap_config(organization_id)

    # 1) To do the bootstrap we need certificates, so steal them from another organization
    if config.sequestered:
        _, _, testbed_template_content = await testbed.get_template("sequestered")
    else:
        _, _, testbed_template_content = await testbed.get_template("coolorg")
    bootstrap_event = testbed_template_content.events[0]
    assert isinstance(bootstrap_event, tb.TestbedEventBootstrapOrganization)
    user_certificate = bootstrap_event.first_user_raw_certificate
    redacted_user_certificate = bootstrap_event.first_user_raw_redacted_certificate
    device_certificate = bootstrap_event.first_user_first_device_raw_certificate
    redacted_device_certificate = bootstrap_event.first_user_first_device_raw_redacted_certificate
    root_verify_key = bootstrap_event.root_signing_key.verify_key
    sequester_authority_certificate = bootstrap_event.sequester_authority_raw_certificate
    if config.spontaneous:
        backend.config.organization_initial_active_users_limit = ActiveUsersLimit.limited_to(3)
        backend.config.organization_initial_user_profile_outsider_allowed = False
        backend.config.organization_initial_minimum_archiving_period = 42
        backend.config.organization_initial_tos = {"cn_HK": "https://parsec.invalid/tos_cn.pdf"}

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=config.bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    )

    assert rep == anonymous_cmds.latest.organization_bootstrap.RepOk()

    # Ensure the default config has been used to configure the organization
    # when spontaneously created.
    if config.spontaneous:
        orgs = await backend.organization.test_dump_organizations()
        org = orgs[organization_id]
        assert org.active_users_limit == backend.config.organization_initial_active_users_limit
        assert (
            org.user_profile_outsider_allowed
            == backend.config.organization_initial_user_profile_outsider_allowed
        )
        assert (
            org.minimum_archiving_period
            == backend.config.organization_initial_minimum_archiving_period
        )
        assert backend.config.organization_initial_tos is not None  # Sanity check to please typing
        assert org.tos == TermsOfService(
            updated_on=ANY, per_locale_urls=backend.config.organization_initial_tos
        )


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
async def test_anonymous_organization_bootstrap_invalid_certificate(
    override_certif_data: OverrideCertifData,
    backend_bootstrap_config: ConfigureBackend,
    client: AsyncClient,
    testbed: TestbedBackend,
) -> None:
    organization_id = next_organization_id(prefix="NewOrg")
    anonymous_client = AnonymousRpcClient(client, organization_id)

    config = await backend_bootstrap_config(organization_id)

    # 1) To do the bootstrap we need certificates, so steal them from another organization
    if config.sequestered:
        _, _, testbed_template_content = await testbed.get_template("sequestered")
    else:
        _, _, testbed_template_content = await testbed.get_template("coolorg")
    bootstrap_event = testbed_template_content.events[0]
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
        sequester_authority_certificate = bootstrap_event.sequester_authority_raw_certificate

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=config.bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    )

    assert rep == anonymous_cmds.latest.organization_bootstrap.RepInvalidCertificate()


@pytest.mark.usefixtures("ballpark_always_ok")
async def test_anonymous_organization_bootstrap_organization_already_bootstrapped(
    backend_bootstrap_config: ConfigureBackend,
    client: AsyncClient,
    testbed: TestbedBackend,
) -> None:
    organization_id = next_organization_id(prefix="NewOrg")
    anonymous_client = AnonymousRpcClient(client, organization_id)

    config = await backend_bootstrap_config(organization_id)

    # 1) To do the bootstrap we need certificates, so steal them from another organization
    if config.sequestered:
        _, _, testbed_template_content = await testbed.get_template("sequestered")
    else:
        _, _, testbed_template_content = await testbed.get_template("coolorg")
    bootstrap_event = testbed_template_content.events[0]
    assert isinstance(bootstrap_event, tb.TestbedEventBootstrapOrganization)
    user_certificate = bootstrap_event.first_user_raw_certificate
    redacted_user_certificate = bootstrap_event.first_user_raw_redacted_certificate
    device_certificate = bootstrap_event.first_user_first_device_raw_certificate
    redacted_device_certificate = bootstrap_event.first_user_first_device_raw_redacted_certificate
    root_verify_key = bootstrap_event.root_signing_key.verify_key
    sequester_authority_certificate = bootstrap_event.sequester_authority_raw_certificate

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=config.bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    )

    assert rep == anonymous_cmds.latest.organization_bootstrap.RepOk()

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=config.bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    )

    assert rep == anonymous_cmds.latest.organization_bootstrap.RepOrganizationAlreadyBootstrapped()


@pytest.mark.usefixtures("ballpark_always_ok")
async def test_anonymous_organization_bootstrap_invalid_bootstrap_token(
    backend_bootstrap_config: ConfigureBackend,
    client: AsyncClient,
    testbed: TestbedBackend,
) -> None:
    organization_id = next_organization_id(prefix="NewOrg")
    anonymous_client = AnonymousRpcClient(client, organization_id)

    config = await backend_bootstrap_config(organization_id)

    # 1) To do the bootstrap we need certificates, so steal them from another organization
    if config.sequestered:
        _, _, testbed_template_content = await testbed.get_template("sequestered")
    else:
        _, _, testbed_template_content = await testbed.get_template("coolorg")
    bootstrap_event = testbed_template_content.events[0]
    assert isinstance(bootstrap_event, tb.TestbedEventBootstrapOrganization)
    user_certificate = bootstrap_event.first_user_raw_certificate
    redacted_user_certificate = bootstrap_event.first_user_raw_redacted_certificate
    device_certificate = bootstrap_event.first_user_first_device_raw_certificate
    redacted_device_certificate = bootstrap_event.first_user_first_device_raw_redacted_certificate
    root_verify_key = bootstrap_event.root_signing_key.verify_key
    sequester_authority_certificate = bootstrap_event.sequester_authority_raw_certificate

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=BootstrapToken.new(),
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    )

    assert rep == anonymous_cmds.latest.organization_bootstrap.RepInvalidBootstrapToken()


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
async def test_anonymous_organization_bootstrap_timestamp_out_of_ballpark(
    user_certif_late: bool,
    device_certif_late: bool,
    sequester_auth_certif_late: bool,
    ballpark_offset: int,
    backend_bootstrap_config: ConfigureBackend,
    client: AsyncClient,
) -> None:
    organization_id = next_organization_id(prefix="NewOrg")
    anonymous_client = AnonymousRpcClient(client, organization_id)

    config = await backend_bootstrap_config(organization_id)

    now = DateTime.now()
    late = now.add(seconds=ballpark_offset)

    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key

    user_certificates = generate_new_user_certificates(
        timestamp=late if user_certif_late else now,
        human_handle=HumanHandle(EmailAddress("mike@example.invalid"), "Mike"),
        profile=UserProfile.ADMIN,
        author_signing_key=root_signing_key,
    )

    device_certificates = generate_new_device_certificates(
        timestamp=late if device_certif_late else now,
        user_id=user_certificates.certificate.user_id,
        device_label=DeviceLabel("device label"),
        author_signing_key=root_signing_key,
    )

    _, sequester_verify_key = SequesterSigningKeyDer.generate_pair(1024)

    sequester_authority_certificate = SequesterAuthorityCertificate(
        timestamp=late if sequester_auth_certif_late else now,
        verify_key_der=sequester_verify_key,
    )

    rep = await anonymous_client.organization_bootstrap(
        bootstrap_token=config.bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificates.signed_certificate,
        device_certificate=device_certificates.signed_certificate,
        redacted_user_certificate=user_certificates.signed_redacted_certificate,
        redacted_device_certificate=device_certificates.signed_redacted_certificate,
        sequester_authority_certificate=sequester_authority_certificate.dump_and_sign(
            root_signing_key
        )
        if sequester_authority_certificate
        else None,
    )

    assert isinstance(rep, anonymous_cmds.latest.organization_bootstrap.RepTimestampOutOfBallpark)
    assert rep.ballpark_client_early_offset == BALLPARK_CLIENT_EARLY_OFFSET
    assert rep.ballpark_client_late_offset == BALLPARK_CLIENT_LATE_OFFSET
    assert rep.client_timestamp == late
    # We don't compare the server timestamp as it's not deterministic, should we freeze time ?


def test_anonymous_organization_bootstrap_http_common_errors() -> None:
    # Nothing to do here: the organization doesn't exist yet so nothing can be missing or expired !
    # (this test is only present to please `test_each_cmd_req_rep_has_dedicated_test`)
    pass

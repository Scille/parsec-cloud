# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import AsyncGenerator, Awaitable, Callable, Coroutine

import pytest

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SigningKey,
    SigningKeyAlgorithm,
    UserProfile,
    UserUpdateCertificate,
    VlobID,
)
from parsec.components.user import UserInfo
from tests.common import Backend, CoolorgRpcClients, RpcTransportError


@pytest.fixture(params=("common_certificate", "realm_certificate", "shamir_certificate", "vlob"))
def alice_generated_data(
    request: pytest.FixtureRequest,
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> Callable[[DateTime], Awaitable[None]]:
    """
    Parametrized fixture to generate all the different types of data a user can create.

    This is to be used when testing APIs that change the user capabilities (e.g.
    `realm_unshare`) to ensure the change cannot be done if its timestamp is older
    than data previously created by the user.

    Note, while being part of the realm's data, blocks are not present here. This
    is simply because blocks don't contain timestamp information.
    """
    match request.param:
        case "common_certificate":

            async def _alice_generated_data(now: DateTime) -> None:
                device_id = DeviceID.new()
                verify_key = SigningKey.generate().verify_key

                device_certif = DeviceCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    user_id=coolorg.alice.user_id,
                    device_id=device_id,
                    device_label=DeviceLabel("New device"),
                    verify_key=verify_key,
                    algorithm=SigningKeyAlgorithm.ED25519,
                ).dump_and_sign(coolorg.alice.signing_key)

                redacted_device_certif = DeviceCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    user_id=coolorg.alice.user_id,
                    device_id=device_id,
                    device_label=None,
                    verify_key=verify_key,
                    algorithm=SigningKeyAlgorithm.ED25519,
                ).dump_and_sign(coolorg.alice.signing_key)

                outcome = await backend.user.create_device(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    device_certificate=device_certif,
                    redacted_device_certificate=redacted_device_certif,
                )
                assert isinstance(outcome, DeviceCertificate)

        case "realm_certificate":

            async def _alice_generated_data(now: DateTime) -> None:
                realm_role_certif = RealmRoleCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    realm_id=VlobID.new(),
                    user_id=coolorg.alice.user_id,
                    role=RealmRole.OWNER,
                ).dump_and_sign(coolorg.alice.signing_key)

                outcome = await backend.realm.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    realm_role_certificate=realm_role_certif,
                )
                assert isinstance(outcome, RealmRoleCertificate)

        case "shamir_certificate":
            # TODO: implement this once Shamir support is done !
            pytest.skip(reason="shamir not implemented")

        case "vlob":

            async def _alice_generated_data(now: DateTime) -> None:
                outcome = await backend.vlob.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    realm_id=coolorg.wksp1_id,
                    vlob_id=VlobID.new(),
                    key_index=1,
                    timestamp=now,
                    blob=b"<vlob data>",
                )
                assert outcome is None

        case unknown:
            assert False, unknown

    return _alice_generated_data


@pytest.fixture(params=("common_certificate", "realm_certificate", "vlob"))
def alice_generated_realm_wksp1_data(
    request: pytest.FixtureRequest,
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> Callable[[DateTime], Awaitable[None]]:
    """
    Parametrized fixture to generate all the different types of data a user can create
    in a realm.

    This is to be used when testing APIs that change the user capabilities (e.g.
    `realm_unshare`) to ensure the change cannot be done if its timestamp is older
    than data previously created by the user.

    Note, while being part of the realm's data, blocks are not present here. This
    is simply because blocks don't contain timestamp information.
    """
    match request.param:
        case "common_certificate":

            async def _alice_generated_realm_data(now: DateTime) -> None:
                device_id = DeviceID.new()
                verify_key = SigningKey.generate().verify_key

                device_certif = DeviceCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    user_id=coolorg.alice.user_id,
                    device_id=device_id,
                    device_label=DeviceLabel("New device"),
                    verify_key=verify_key,
                    algorithm=SigningKeyAlgorithm.ED25519,
                ).dump_and_sign(coolorg.alice.signing_key)

                redacted_device_certif = DeviceCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    user_id=coolorg.alice.user_id,
                    device_id=device_id,
                    device_label=None,
                    verify_key=verify_key,
                    algorithm=SigningKeyAlgorithm.ED25519,
                ).dump_and_sign(coolorg.alice.signing_key)

                outcome = await backend.user.create_device(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    device_certificate=device_certif,
                    redacted_device_certificate=redacted_device_certif,
                )
                assert isinstance(outcome, DeviceCertificate), outcome

        case "realm_certificate":

            async def _alice_generated_realm_data(now: DateTime) -> None:
                realm_name_certificate = RealmNameCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=now,
                    realm_id=coolorg.wksp1_id,
                    key_index=1,
                    encrypted_name=b"<dummy encrypted name>",
                ).dump_and_sign(coolorg.alice.signing_key)

                outcome = await backend.realm.rename(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    realm_name_certificate=realm_name_certificate,
                    initial_name_or_fail=False,
                )
                assert isinstance(outcome, RealmNameCertificate), outcome

        case "vlob":

            async def _alice_generated_realm_data(now: DateTime) -> None:
                outcome = await backend.vlob.create(
                    organization_id=coolorg.organization_id,
                    now=now,
                    author=coolorg.alice.device_id,
                    realm_id=coolorg.wksp1_id,
                    vlob_id=VlobID.new(),
                    key_index=1,
                    timestamp=now,
                    blob=b"<vlob data>",
                )
                assert outcome is None, outcome

        case unknown:
            assert False, unknown

    return _alice_generated_realm_data


HttpCommonErrorsTesterDoCallback = Callable[[], Coroutine[None, None, None]]
HttpCommonErrorsTester = Callable[[HttpCommonErrorsTesterDoCallback], Coroutine[None, None, None]]


# TODO: not sure how to test "organization_not_found"
@pytest.fixture(params=("organization_expired", "author_revoked", "author_frozen"))
async def authenticated_http_common_errors_tester(
    request: pytest.FixtureRequest, coolorg: CoolorgRpcClients, backend: Backend
) -> AsyncGenerator[HttpCommonErrorsTester, None]:
    tester_called = False

    async def _authenticated_http_common_errors_tester(do: HttpCommonErrorsTesterDoCallback):
        nonlocal tester_called
        tester_called = True
        match request.param:
            case "organization_expired":
                outcome = await backend.organization.update(
                    id=coolorg.organization_id, is_expired=True
                )
                assert outcome is None

                expected_http_status = 460

            case "author_revoked":
                # Must first promote Bob to ADMIN...

                bob_user_update_certificate = UserUpdateCertificate(
                    author=coolorg.alice.device_id,
                    timestamp=DateTime.now(),
                    user_id=coolorg.bob.user_id,
                    new_profile=UserProfile.ADMIN,
                ).dump_and_sign(coolorg.alice.signing_key)

                outcome = await backend.user.update_user(
                    organization_id=coolorg.organization_id,
                    now=DateTime.now(),
                    author=coolorg.alice.device_id,
                    author_verify_key=coolorg.alice.signing_key.verify_key,
                    user_update_certificate=bob_user_update_certificate,
                )
                assert isinstance(outcome, UserUpdateCertificate)

                # ...to be able to revoke Alice

                certif = RevokedUserCertificate(
                    author=coolorg.bob.device_id,
                    user_id=coolorg.alice.user_id,
                    timestamp=DateTime.now(),
                )
                outcome = await backend.user.revoke_user(
                    now=DateTime.now(),
                    organization_id=coolorg.organization_id,
                    author=coolorg.bob.device_id,
                    author_verify_key=coolorg.bob.signing_key.verify_key,
                    revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
                )
                assert isinstance(outcome, RevokedUserCertificate)

                expected_http_status = 461

            case "author_frozen":
                outcome = await backend.user.freeze_user(
                    organization_id=coolorg.organization_id,
                    user_id=coolorg.alice.user_id,
                    user_email=None,
                    frozen=True,
                )
                assert isinstance(outcome, UserInfo)

                expected_http_status = 462

            case unknown:
                assert False, unknown

        try:
            await do()
            assert False, f"{do!r} was expected to raise an `RpcTransportError` exception !"
        except RpcTransportError as err:
            assert err.rep.status_code == expected_http_status

    yield _authenticated_http_common_errors_tester

    assert tester_called


# TODO: not sure how to test "organization_not_found" & "invitation_not_found"
@pytest.fixture(params=("organization_expired", "invitation_already_used"))
async def invited_http_common_errors_tester(
    request: pytest.FixtureRequest, coolorg: CoolorgRpcClients, backend: Backend
) -> AsyncGenerator[HttpCommonErrorsTester, None]:
    tester_called = False

    async def _invited_http_common_errors_tester(do: HttpCommonErrorsTesterDoCallback):
        nonlocal tester_called
        tester_called = True

        match request.param:
            case "organization_expired":
                outcome = await backend.organization.update(
                    id=coolorg.organization_id, is_expired=True
                )
                assert outcome is None

                expected_http_status = 460

            case "invitation_already_used":
                outcome = await backend.invite.cancel(
                    now=DateTime.now(),
                    organization_id=coolorg.organization_id,
                    author=coolorg.alice.device_id,
                    token=coolorg.invited_alice_dev3.token,
                )
                assert outcome is None

                expected_http_status = 410

            case unknown:
                assert False, unknown

        try:
            await do()
            assert False, f"{do!r} was expected to raise an `RpcTransportError` exception !"
        except RpcTransportError as err:
            assert err.rep.status_code == expected_http_status, err

    yield _invited_http_common_errors_tester

    assert tester_called


# TODO: not sure how to test "organization_not_found"
@pytest.fixture(params=("organization_expired",))
async def anonymous_http_common_errors_tester(
    request: pytest.FixtureRequest, coolorg: CoolorgRpcClients, backend: Backend
) -> AsyncGenerator[HttpCommonErrorsTester, None]:
    tester_called = False

    async def _anonymous_http_common_errors_tester(do: HttpCommonErrorsTesterDoCallback):
        nonlocal tester_called
        tester_called = True

        match request.param:
            case "organization_expired":
                outcome = await backend.organization.update(
                    id=coolorg.organization_id, is_expired=True
                )
                assert outcome is None

                expected_http_status = 460

            case unknown:
                assert False, unknown

        try:
            await do()
            assert False, f"{do!r} was expected to raise an `RpcTransportError` exception !"
        except RpcTransportError as err:
            assert err.rep.status_code == expected_http_status, err

    yield _anonymous_http_common_errors_tester

    assert tester_called

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
    UserID,
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


async def alice_gives_profile(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    recipient: UserID,
    new_profile: UserProfile | None,
    now: DateTime | None = None,
) -> RevokedUserCertificate | UserUpdateCertificate:
    now = now or DateTime.now()
    if new_profile is None:
        certif = RevokedUserCertificate(
            author=coolorg.alice.device_id,
            timestamp=now,
            user_id=recipient,
        )

        outcome = await backend.user.revoke_user(
            now=now,
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            author_verify_key=coolorg.alice.signing_key.verify_key,
            revoked_user_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        )
        assert isinstance(outcome, RevokedUserCertificate)

    else:
        certif = UserUpdateCertificate(
            author=coolorg.alice.device_id,
            timestamp=now,
            user_id=recipient,
            new_profile=new_profile,
        )

        outcome = await backend.user.update_user(
            now=now,
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            author_verify_key=coolorg.alice.signing_key.verify_key,
            user_update_certificate=certif.dump_and_sign(coolorg.alice.signing_key),
        )
        assert isinstance(outcome, UserUpdateCertificate)

    return certif


async def bob_becomes_admin(
    coolorg: CoolorgRpcClients, backend: Backend
) -> tuple[UserUpdateCertificate, bytes]:
    # Bob becomes ADMIN...

    t0 = DateTime.now()
    certif0 = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        timestamp=t0,
        user_id=coolorg.bob.user_id,
        new_profile=UserProfile.ADMIN,
    )
    raw_certif0 = certif0.dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.user.update_user(
        now=t0,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        user_update_certificate=raw_certif0,
    )
    assert isinstance(outcome, UserUpdateCertificate)
    return certif0, raw_certif0


async def bob_becomes_admin_and_changes_alice(
    coolorg: CoolorgRpcClients, backend: Backend, new_alice_profile: UserProfile | None
) -> tuple[
    tuple[UserUpdateCertificate, bytes],
    tuple[RevokedUserCertificate | UserUpdateCertificate, bytes],
]:
    (certif0, raw_certif0) = await bob_becomes_admin(coolorg, backend)
    # ...then change Alice's profile (or revoke her) !

    t1 = DateTime.now()
    if new_alice_profile is None:
        certif1 = RevokedUserCertificate(
            author=coolorg.bob.device_id,
            timestamp=t1,
            user_id=coolorg.alice.user_id,
        )
        raw_certif1 = certif1.dump_and_sign(coolorg.alice.signing_key)

        outcome = await backend.user.revoke_user(
            now=t1,
            organization_id=coolorg.organization_id,
            author=coolorg.bob.device_id,
            author_verify_key=coolorg.alice.signing_key.verify_key,
            revoked_user_certificate=raw_certif1,
        )
        assert isinstance(outcome, RevokedUserCertificate)

    else:
        certif1 = UserUpdateCertificate(
            author=coolorg.bob.device_id,
            timestamp=t1,
            user_id=coolorg.alice.user_id,
            new_profile=new_alice_profile,
        )
        raw_certif1 = certif1.dump_and_sign(coolorg.alice.signing_key)

        outcome = await backend.user.update_user(
            now=t1,
            organization_id=coolorg.organization_id,
            author=coolorg.bob.device_id,
            author_verify_key=coolorg.alice.signing_key.verify_key,
            user_update_certificate=raw_certif1,
        )
        assert isinstance(outcome, UserUpdateCertificate)

    return (certif0, raw_certif0), (certif1, raw_certif1)


async def wksp1_alice_gives_role(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    recipient: UserID,
    new_role: RealmRole | None,
    now: DateTime | None = None,
) -> tuple[RealmRoleCertificate, bytes]:
    now = now or DateTime.now()
    if new_role is None:
        certif = RealmRoleCertificate(
            author=coolorg.alice.device_id,
            timestamp=now,
            realm_id=coolorg.wksp1_id,
            role=None,
            user_id=recipient,
        )
        raw_certif = certif.dump_and_sign(coolorg.alice.signing_key)

        outcome = await backend.realm.unshare(
            now=now,
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            author_verify_key=coolorg.bob.signing_key.verify_key,
            realm_role_certificate=raw_certif,
        )
        assert isinstance(outcome, RealmRoleCertificate)

    else:
        certif = RealmRoleCertificate(
            author=coolorg.alice.device_id,
            timestamp=now,
            realm_id=coolorg.wksp1_id,
            role=new_role,
            user_id=recipient,
        )
        raw_certif = certif.dump_and_sign(coolorg.alice.signing_key)

        outcome = await backend.realm.share(
            now=now,
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            author_verify_key=coolorg.bob.signing_key.verify_key,
            realm_role_certificate=raw_certif,
            key_index=1,
            recipient_keys_bundle_access=b"<dummy key bundle access>",
        )
        assert isinstance(outcome, RealmRoleCertificate)

    return (certif, raw_certif)


async def wksp1_bob_becomes_owner_and_changes_alice(
    coolorg: CoolorgRpcClients, backend: Backend, new_alice_role: RealmRole | None
) -> tuple[tuple[RealmRoleCertificate, bytes], tuple[RealmRoleCertificate, bytes]]:
    # Bob becomes OWNER...

    t0 = DateTime.now()
    certif0 = RealmRoleCertificate(
        author=coolorg.alice.device_id,
        timestamp=t0,
        realm_id=coolorg.wksp1_id,
        role=RealmRole.OWNER,
        user_id=coolorg.bob.user_id,
    )
    raw_certif0 = certif0.dump_and_sign(coolorg.alice.signing_key)

    outcome = await backend.realm.share(
        now=t0,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        author_verify_key=coolorg.alice.signing_key.verify_key,
        realm_role_certificate=raw_certif0,
        key_index=1,
        recipient_keys_bundle_access=b"<dummy key bundle access>",
    )
    assert isinstance(outcome, RealmRoleCertificate)

    # ...then change Alice's access !

    t1 = DateTime.now()
    if new_alice_role is None:
        certif1 = RealmRoleCertificate(
            author=coolorg.bob.device_id,
            timestamp=t1,
            realm_id=coolorg.wksp1_id,
            role=None,
            user_id=coolorg.alice.user_id,
        )
        raw_certif1 = certif1.dump_and_sign(coolorg.alice.signing_key)

        outcome = await backend.realm.unshare(
            now=t1,
            organization_id=coolorg.organization_id,
            author=coolorg.bob.device_id,
            author_verify_key=coolorg.bob.signing_key.verify_key,
            realm_role_certificate=raw_certif1,
        )
        assert isinstance(outcome, RealmRoleCertificate)

    else:
        certif1 = RealmRoleCertificate(
            author=coolorg.bob.device_id,
            timestamp=t1,
            realm_id=coolorg.wksp1_id,
            role=new_alice_role,
            user_id=coolorg.alice.user_id,
        )
        raw_certif1 = certif1.dump_and_sign(coolorg.alice.signing_key)

        outcome = await backend.realm.share(
            now=t1,
            organization_id=coolorg.organization_id,
            author=coolorg.bob.device_id,
            author_verify_key=coolorg.bob.signing_key.verify_key,
            realm_role_certificate=raw_certif1,
            key_index=1,
            recipient_keys_bundle_access=b"<dummy key bundle access>",
        )
        assert isinstance(outcome, RealmRoleCertificate)

    return (certif0, raw_certif0), (certif1, raw_certif1)


type HttpCommonErrorsTesterDoCallback = Callable[[], Coroutine[None, None, None]]
type HttpCommonErrorsTester = Callable[
    [HttpCommonErrorsTesterDoCallback], Coroutine[None, None, None]
]


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


def generate_realm_role_certificate(
    coolorg: CoolorgRpcClients,
    user_id: UserID,
    role: RealmRole | None,
    author: DeviceID | None = None,
    timestamp: DateTime | None = None,
    realm_id: VlobID | None = None,
) -> RealmRoleCertificate:
    return RealmRoleCertificate(
        author=author if author is not None else coolorg.alice.device_id,
        timestamp=timestamp if timestamp is not None else DateTime.now(),
        realm_id=realm_id if realm_id is not None else coolorg.wksp1_id,
        user_id=user_id,
        role=role,
    )

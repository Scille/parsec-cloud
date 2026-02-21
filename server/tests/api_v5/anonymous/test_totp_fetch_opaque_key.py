# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import (
    DateTime,
    SecretKey,
    TOTPOpaqueKeyID,
    anonymous_cmds,
)
from parsec.components.totp import TOTPFetchOpaqueKeyBadOutcome, compute_totp_one_time_password
from parsec.components.user import UserInfo
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    bob_becomes_admin_and_changes_alice,
)


async def _init_totp_and_create_opaque_key(
    org: MinimalorgRpcClients | CoolorgRpcClients, backend: Backend
) -> tuple[str, TOTPOpaqueKeyID, SecretKey]:
    totp_secret = await backend.totp.setup_get_secret(
        organization_id=org.organization_id,
        author=org.alice.device_id,
    )
    assert isinstance(totp_secret, bytes)

    one_time_password = compute_totp_one_time_password(DateTime.now(), totp_secret)

    outcome = await backend.totp.setup_confirm(
        organization_id=org.organization_id,
        author=org.alice.device_id,
        one_time_password=one_time_password,
    )
    assert outcome is None

    outcome = await backend.totp.create_opaque_key(
        organization_id=org.organization_id,
        author=org.alice.device_id,
    )
    assert isinstance(outcome, tuple)
    return one_time_password, *outcome


async def test_anonymous_totp_fetch_opaque_key_ok(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    one_time_password, opaque_key_id, opaque_key = await _init_totp_and_create_opaque_key(
        minimalorg, backend
    )

    rep = await minimalorg.anonymous.totp_fetch_opaque_key(
        user_id=minimalorg.alice.user_id,
        opaque_key_id=opaque_key_id,
        one_time_password=one_time_password,
    )
    assert isinstance(rep, anonymous_cmds.latest.totp_fetch_opaque_key.RepOk)
    assert rep.opaque_key == opaque_key


async def test_anonymous_totp_fetch_opaque_key_invalid_one_time_password(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    one_time_password, opaque_key_id, _ = await _init_totp_and_create_opaque_key(coolorg, backend)

    # Invalid one-time-password

    rep = await coolorg.anonymous.totp_fetch_opaque_key(
        user_id=coolorg.alice.user_id,
        opaque_key_id=opaque_key_id,
        one_time_password="00",  # Always wrong given our one-time-password is supposed to be 6 digits
    )
    assert rep == anonymous_cmds.latest.totp_fetch_opaque_key.RepInvalidOneTimePassword()

    # Unknown opaque key ID

    rep = await coolorg.anonymous.totp_fetch_opaque_key(
        user_id=coolorg.alice.user_id,
        opaque_key_id=TOTPOpaqueKeyID.new(),
        one_time_password="000000",
    )
    assert rep == anonymous_cmds.latest.totp_fetch_opaque_key.RepInvalidOneTimePassword()

    # Mismatch between user ID and existing opaque key's owner

    rep = await coolorg.anonymous.totp_fetch_opaque_key(
        user_id=coolorg.bob.user_id,  # Opaque key is only accessible to Alice !
        opaque_key_id=opaque_key_id,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_fetch_opaque_key.RepInvalidOneTimePassword()

    # Frozen user

    outcome = await backend.user.freeze_user(
        organization_id=coolorg.organization_id,
        user_id=coolorg.alice.user_id,
        user_email=None,
        frozen=True,
    )
    assert isinstance(outcome, UserInfo)

    rep = await coolorg.anonymous.totp_fetch_opaque_key(
        user_id=coolorg.alice.user_id,
        opaque_key_id=opaque_key_id,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_fetch_opaque_key.RepInvalidOneTimePassword()

    outcome = await backend.user.freeze_user(
        organization_id=coolorg.organization_id,
        user_id=coolorg.alice.user_id,
        user_email=None,
        frozen=False,
    )
    assert isinstance(outcome, UserInfo)

    # Revoked user

    await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)

    rep = await coolorg.anonymous.totp_fetch_opaque_key(
        user_id=coolorg.alice.user_id,
        opaque_key_id=opaque_key_id,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_fetch_opaque_key.RepInvalidOneTimePassword()


async def test_anonymous_totp_fetch_opaque_key_setup_not_completed(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    totp_secret = await backend.totp.setup_get_secret(
        organization_id=minimalorg.organization_id,
        author=minimalorg.alice.device_id,
    )
    assert isinstance(totp_secret, bytes)

    one_time_password = compute_totp_one_time_password(DateTime.now(), totp_secret)

    outcome = await backend.totp.create_opaque_key(
        organization_id=minimalorg.organization_id,
        author=minimalorg.alice.device_id,
    )
    assert isinstance(outcome, tuple)
    opaque_key_id, _ = outcome

    # The one-time-password is valid but we still get a `invalid_one_time_password`
    # since the TOTP setup hasn't been confirmed.
    # This is important to ensure the TOTP secret is no longer freely accessible
    # once it is used to protect keys.
    rep = await minimalorg.anonymous.totp_fetch_opaque_key(
        user_id=minimalorg.alice.user_id,
        opaque_key_id=opaque_key_id,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_fetch_opaque_key.RepInvalidOneTimePassword()


async def test_anonymous_totp_fetch_opaque_key_setup_reset(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    one_time_password, opaque_key_id, _ = await _init_totp_and_create_opaque_key(
        minimalorg, backend
    )

    outcome = await backend.totp.reset(
        organization_id=minimalorg.organization_id, user_id=minimalorg.alice.user_id
    )
    assert isinstance(outcome, tuple)

    # Once the setup reset, the previous TOTP secret should no longer be valid,
    # and hence its one-time-password should be rejected.
    rep = await minimalorg.anonymous.totp_fetch_opaque_key(
        user_id=minimalorg.alice.user_id,
        opaque_key_id=opaque_key_id,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_fetch_opaque_key.RepInvalidOneTimePassword()


async def test_anonymous_totp_fetch_opaque_key_throttled(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    one_time_password, opaque_key_id1, _ = await _init_totp_and_create_opaque_key(
        minimalorg, backend
    )
    outcome = await backend.totp.create_opaque_key(
        organization_id=minimalorg.organization_id,
        author=minimalorg.alice.device_id,
    )
    assert isinstance(outcome, tuple)
    opaque_key_id2, _ = outcome

    # Simulate multiple failed attemps to trigger throttling

    now = DateTime.now()

    outcome = await backend.totp.fetch_opaque_key(
        now=now,
        organization_id=minimalorg.organization_id,
        user_id=minimalorg.alice.user_id,
        opaque_key_id=opaque_key_id1,
        one_time_password="00",
    )
    assert outcome == TOTPFetchOpaqueKeyBadOutcome.INVALID_ONE_TIME_PASSWORD

    outcome = await backend.totp.fetch_opaque_key(
        now=now.add(seconds=2),
        organization_id=minimalorg.organization_id,
        user_id=minimalorg.alice.user_id,
        opaque_key_id=opaque_key_id1,
        one_time_password="00",
    )
    assert outcome == TOTPFetchOpaqueKeyBadOutcome.INVALID_ONE_TIME_PASSWORD

    outcome = await backend.totp.fetch_opaque_key(
        now=now.add(seconds=5),
        organization_id=minimalorg.organization_id,
        user_id=minimalorg.alice.user_id,
        opaque_key_id=opaque_key_id1,
        one_time_password="00",
    )
    assert outcome == TOTPFetchOpaqueKeyBadOutcome.INVALID_ONE_TIME_PASSWORD

    # Now do the regular API call, this should show get throttled...
    rep = await minimalorg.anonymous.totp_fetch_opaque_key(
        user_id=minimalorg.alice.user_id,
        opaque_key_id=opaque_key_id1,
        one_time_password=one_time_password,  # Doesn't matter that we have the correct one-time-password
    )
    assert rep == anonymous_cmds.latest.totp_fetch_opaque_key.RepThrottled(
        wait_until=now.add(seconds=3**2)
    )

    # ...however accessing a different opaque key is fine

    rep = await minimalorg.anonymous.totp_fetch_opaque_key(
        user_id=minimalorg.alice.user_id,
        opaque_key_id=opaque_key_id2,
        one_time_password="00",
    )
    assert rep == anonymous_cmds.latest.totp_fetch_opaque_key.RepInvalidOneTimePassword()


async def test_anonymous_totp_fetch_opaque_key_http_common_errors(
    coolorg: CoolorgRpcClients,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.anonymous.totp_fetch_opaque_key(
            user_id=coolorg.alice.user_id,
            opaque_key_id=TOTPOpaqueKeyID.new(),
            one_time_password="000000",
        )

    await anonymous_http_common_errors_tester(do)

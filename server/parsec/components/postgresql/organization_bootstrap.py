# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    BootstrapToken,
    DateTime,
    DeviceCertificate,
    OrganizationID,
    SequesterAuthorityCertificate,
    UserCertificate,
    VerifyKey,
)
from parsec.ballpark import TimestampOutOfBallpark
from parsec.components.organization import (
    OrganizationBootstrapStoreBadOutcome,
    OrganizationBootstrapValidateBadOutcome,
    organization_bootstrap_validate,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_lock_common_write_and_fetch_checks = Q("""
WITH my_organization AS (
    SELECT
        _id,
        is_expired,
        bootstrap_token,
        (root_verify_key IS NOT NULL) AS already_bootstrapped,
        -- Use `IS DISTINCT FROM` clause to handle comparison with NULL values
        (bootstrap_token IS DISTINCT FROM $bootstrap_token) AS invalid_bootstrap_token
    FROM organization
    WHERE
        organization_id = $organization_id
    LIMIT 1
),

-- Common topic write lock must occur ASAP
-- This is to protect us against concurrent bootstrap
my_locked_common_topic AS (
    SELECT last_timestamp
    FROM common_topic
    WHERE organization = (SELECT _id FROM my_organization)
    LIMIT 1
    FOR UPDATE
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT is_expired FROM my_organization) AS organization_expired,
    (SELECT already_bootstrapped FROM my_organization) AS organization_already_bootstrapped,
    (SELECT invalid_bootstrap_token FROM my_organization) AS organization_invalid_bootstrap_token
""")

_q_update_organization = Q("""
WITH new_human AS (
    INSERT INTO human (organization, email, label)
    VALUES (
        $organization_internal_id,
        $email,
        $label
    )
    RETURNING _id
),

new_user AS (
    INSERT INTO user_ (
        organization,
        user_id,
        initial_profile,
        current_profile,
        user_certificate,
        redacted_user_certificate,
        user_certifier,
        created_on,
        human
    )
    VALUES (
        $organization_internal_id,
        $user_id,
        $initial_profile,
        $initial_profile,
        $user_certificate,
        $redacted_user_certificate,
        NULL,
        $bootstrapped_on,
        (SELECT _id FROM new_human)
    )
    RETURNING _id
),

new_device AS (
    INSERT INTO device (
        organization,
        user_,
        device_id,
        device_label,
        verify_key,
        device_certificate,
        redacted_device_certificate,
        device_certifier,
        created_on
    )
    (
        SELECT
            $organization_internal_id,
            new_user._id,
            $device_id,
            $device_label,
            $verify_key,
            $device_certificate,
            $redacted_device_certificate,
            NULL,
            $bootstrapped_on
        FROM new_user
    )
    RETURNING _id
),

updated_organization AS (
    UPDATE organization
    SET
        root_verify_key = $root_verify_key,
        sequester_authority_certificate=$sequester_authority_certificate,
        sequester_authority_verify_key_der=$sequester_authority_verify_key_der,
        _bootstrapped_on = $bootstrapped_on
    WHERE
        _id = $organization_internal_id
    RETURNING TRUE
),

updated_common_topic AS (
    UPDATE common_topic
    SET last_timestamp = $bootstrapped_on
    WHERE
        organization = $organization_internal_id
        -- Sanity check
        AND last_timestamp < $bootstrapped_on
    RETURNING TRUE
)

SELECT
    COALESCE((SELECT * FROM updated_organization), FALSE) AS update_organization_ok,
    COALESCE((SELECT * FROM updated_common_topic), FALSE) AS update_common_topic_ok
""")


async def organization_bootstrap(
    conn: AsyncpgConnection,
    id: OrganizationID,
    now: DateTime,
    bootstrap_token: BootstrapToken | None,
    root_verify_key: VerifyKey,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
    sequester_authority_certificate: bytes | None,
) -> (
    tuple[UserCertificate, DeviceCertificate, SequesterAuthorityCertificate | None]
    | OrganizationBootstrapValidateBadOutcome
    | OrganizationBootstrapStoreBadOutcome
    | TimestampOutOfBallpark
):
    # 1) Validate certificates

    match organization_bootstrap_validate(
        now=now,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
        sequester_authority_certificate=sequester_authority_certificate,
    ):
        case (user_certif, device_certif, sequester_certif):
            pass
        case error:
            return error

    # 2) Write lock common topic, check and start updating

    sequester_authority_verify_key_der = (
        None if sequester_certif is None else sequester_certif.verify_key_der.dump()
    )

    row = await conn.fetchrow(
        *_q_lock_common_write_and_fetch_checks(
            organization_id=id.str,
            bootstrap_token=None if bootstrap_token is None else bootstrap_token.hex,
        )
    )
    assert row is not None

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return OrganizationBootstrapStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, unknown

    match row["organization_expired"]:
        case False:
            pass
        case True:
            return OrganizationBootstrapStoreBadOutcome.ORGANIZATION_EXPIRED
        case unknown:
            assert False, unknown

    match row["organization_already_bootstrapped"]:
        case False:
            pass
        case True:
            return OrganizationBootstrapStoreBadOutcome.ORGANIZATION_ALREADY_BOOTSTRAPPED
        case unknown:
            assert False, unknown

    match row["organization_invalid_bootstrap_token"]:
        case False:
            pass
        case True:
            return OrganizationBootstrapStoreBadOutcome.INVALID_BOOTSTRAP_TOKEN
        case unknown:
            assert False, unknown

    # 3) All checks are good, now we do the insertions

    row = await conn.fetchrow(
        *_q_update_organization(
            organization_internal_id=organization_internal_id,
            root_verify_key=root_verify_key.encode(),
            sequester_authority_certificate=sequester_authority_certificate,
            sequester_authority_verify_key_der=sequester_authority_verify_key_der,
            bootstrapped_on=user_certif.timestamp,
            email=user_certif.human_handle.email,
            label=user_certif.human_handle.label,
            user_id=user_certif.user_id,
            initial_profile=user_certif.profile.str,
            user_certificate=user_certificate,
            redacted_user_certificate=redacted_user_certificate,
            device_id=device_certif.device_id,
            device_label=device_certif.device_label.str,
            verify_key=device_certif.verify_key.encode(),
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
    )
    assert row is not None

    match row["update_organization_ok"]:
        case True:
            pass
        case unknown:
            assert False, unknown

    match row["update_common_topic_ok"]:
        case True:
            pass
        case unknown:
            assert False, unknown

    return (user_certif, device_certif, sequester_certif)

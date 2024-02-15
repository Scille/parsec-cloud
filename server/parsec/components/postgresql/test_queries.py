# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec.components.postgresql.utils import (
    Q,
    q_human,
    q_human_internal_id,
    q_organization_internal_id,
    q_user_internal_id,
)

# q_test_drop_organization_from_organization_table
# q_test_drop_organization_from_user_table
# q_test_drop_organization_from_device_table
# q_test_duplicate_organization_from_organization_table
# q_test_duplicate_organization_from_user_table
# q_test_duplicate_organization_from_device_table

q_test_drop_organization_from_organization_table = Q(
    """
DELETE FROM organization
WHERE organization_id = $organization_id
"""
)

q_test_drop_organization_from_human_table = Q(
    f"""
DELETE FROM human
WHERE organization = { q_organization_internal_id("$organization_id") }
"""
)

q_test_drop_organization_from_user_table = Q(
    f"""
DELETE FROM user_
WHERE organization = { q_organization_internal_id("$organization_id") }
"""
)

q_test_drop_organization_from_device_table = Q(
    f"""
DELETE FROM device
WHERE organization = { q_organization_internal_id("$organization_id") }
"""
)

q_test_duplicate_organization_from_organization_table = Q(
    """
INSERT INTO organization (
    organization_id,
    bootstrap_token,
    root_verify_key,
    _expired_on,
    user_profile_outsider_allowed,
    active_users_limit,
    is_expired,
    _bootstrapped_on,
    _created_on,
    sequester_authority_certificate,
    sequester_authority_verify_key_der
)
SELECT
    $target_id,
    bootstrap_token,
    root_verify_key,
    _expired_on,
    user_profile_outsider_allowed,
    active_users_limit,
    is_expired,
    _bootstrapped_on,
    _created_on,
    sequester_authority_certificate,
    sequester_authority_verify_key_der
FROM organization
WHERE organization_id = $source_id
"""
)

q_test_duplicate_organization_from_human_table = Q(
    f"""
INSERT INTO human (
    organization,
    email,
    label
)
SELECT
    { q_organization_internal_id("$target_id") },
    email,
    label
FROM human
WHERE organization = { q_organization_internal_id("$source_id") }
"""
)

q_test_duplicate_organization_from_user_table = Q(
    f"""
INSERT INTO user_ (
    organization,
    user_id,
    user_certificate,
    user_certifier,
    created_on,
    revoked_on,
    revoked_user_certificate,
    revoked_user_certifier,
    human,
    redacted_user_certificate,
    profile
)
SELECT
    { q_organization_internal_id("$target_id") },
    user_id,
    user_certificate,
    user_certifier,
    created_on,
    revoked_on,
    revoked_user_certificate,
    revoked_user_certifier,
    { q_human_internal_id(organization_id="$target_id", email=q_human(_id="user_.human", select="email")) },
    redacted_user_certificate,
    profile
FROM user_
WHERE organization = { q_organization_internal_id("$source_id") }
"""
)

q_test_duplicate_organization_from_device_table = Q(
    f"""
INSERT INTO device (
    organization,
    user_,
    device_id,
    device_certificate,
    device_certifier,
    created_on,
    redacted_device_certificate,
    device_label
)
SELECT
    { q_organization_internal_id("$target_id") },
    { q_user_internal_id(organization_id="$target_id", user_id="user_id") },
    device_id,
    device_certificate,
    device_certifier,
    created_on,
    redacted_device_certificate,
    device_label
FROM device
WHERE organization = { q_organization_internal_id("$source_id") }
"""
)  # TODO: The certifier is wrong here. It should be the new organization's certifier, not the old one.

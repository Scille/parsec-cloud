# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec.components.postgresql.utils import (
    Q,
    q_device,
    q_device_internal_id,
    q_human,
    q_human_internal_id,
    q_organization_internal_id,
    q_realm,
    q_realm_internal_id,
    q_user,
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

q_test_drop_organization_from_invitation_table = Q(
    f"""
DELETE FROM invitation
WHERE organization = { q_organization_internal_id("$organization_id") }
"""
)

q_test_drop_organization_from_realm_table = Q(
    f"""
DELETE FROM realm
WHERE organization = { q_organization_internal_id("$organization_id") }
"""
)

q_test_drop_organization_from_realm_user_role_table = Q(
    f"""
DELETE FROM realm_user_role
WHERE realm in (
    SELECT _id
    FROM realm
    WHERE organization = { q_organization_internal_id("$organization_id") }
)
"""
)

q_test_drop_organization_from_vlob_atom_table = Q(
    f"""
DELETE FROM vlob_atom
WHERE realm in (
    SELECT _id
    FROM realm
    WHERE organization = { q_organization_internal_id("$organization_id") }
)
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
    sequester_authority_verify_key_der,
    minimum_archiving_period
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
    sequester_authority_verify_key_der,
    minimum_archiving_period
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
    { q_user_internal_id(organization_id="$target_id", user_id=q_user(_id="device.user_", select="user_id")) },
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


q_test_duplicate_organization_from_invitation_table = Q(
    f"""
INSERT INTO invitation (
    organization,
    token,
    type,
    author,
    claimer_email,
    created_on,
    deleted_on,
    deleted_reason
)
SELECT
    { q_organization_internal_id("$target_id") },
    token,
    type,
    { q_user_internal_id(organization_id="$target_id", user_id=q_user(_id="invitation.author", select="user_id")) },
    claimer_email,
    created_on,
    deleted_on,
    deleted_reason
FROM invitation
WHERE organization = { q_organization_internal_id("$source_id") }
"""
)


q_test_duplicate_organization_from_realm_table = Q(
    f"""
INSERT INTO realm (
    organization,
    realm_id,
    key_index,
    created_on
)
SELECT
    { q_organization_internal_id("$target_id") },
    realm_id,
    key_index,
    created_on
FROM realm
WHERE organization = { q_organization_internal_id("$source_id") }
"""
)

q_test_duplicate_organization_from_realm_user_role_table = Q(
    f"""
INSERT INTO realm_user_role (
    realm,
    user_,
    role,
    certificate,
    certified_by,
    certified_on
)
SELECT
    { q_realm_internal_id(organization_id="$target_id", realm_id=q_realm(_id="realm_user_role.realm", select="realm_id")) },
    { q_user_internal_id(organization_id="$target_id", user_id=q_user(_id="realm_user_role.user_", select="user_id")) },
    role,
    certificate,
    certified_by,
    certified_on
FROM realm_user_role
INNER JOIN realm ON realm._id = realm_user_role.realm
WHERE realm.organization = { q_organization_internal_id("$source_id") }
"""
)


q_test_duplicate_organization_from_vlob_atom_table = Q(
    f"""
INSERT INTO vlob_atom (
    realm,
    key_index,
    vlob_id,
    version,
    blob,
    size,
    author,
    created_on
)
SELECT
    { q_realm_internal_id(organization_id="$target_id", realm_id=q_realm(_id="vlob_atom.realm", select="realm_id")) },
    vlob_atom.key_index,
    vlob_atom.vlob_id,
    vlob_atom.version,
    vlob_atom.blob,
    vlob_atom.size,
    { q_device_internal_id(organization_id="$target_id", device_id=q_device(_id="vlob_atom.author", select="device_id")) },
    vlob_atom.created_on
FROM vlob_atom
INNER JOIN realm ON realm._id = vlob_atom.realm
WHERE realm.organization = { q_organization_internal_id("$source_id") }
"""
)

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec.components.postgresql.utils import (
    Q,
    q_device,
    q_human,
    q_invitation,
    q_organization_internal_id,
    q_realm,
    q_sequester_service,
    q_user,
)

q_test_drop_organization = Q(
    """
WITH deleted_organizations AS (
    DELETE FROM organization
    WHERE organization_id = $organization_id
    RETURNING _id
),
deleted_sequester_service AS (
    DELETE FROM sequester_service
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
),
deleted_human AS (
    DELETE FROM human
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
),
deleted_users AS (
    DELETE FROM user_
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
),
deleted_devices AS (
    DELETE FROM device
    WHERE user_ in (select * from deleted_users)
    RETURNING _id
),
deleted_profiles AS (
    DELETE FROM profile
    WHERE user_ in (select * from deleted_users)
    RETURNING _id
),
deleted_invitations AS (
    DELETE FROM invitation
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
),
deleted_realms AS (
    DELETE FROM realm
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
),
deleted_realm_user_roles AS (
    DELETE FROM realm_user_role
    WHERE realm in (select * from deleted_realms)
    RETURNING _id
),
deleted_vlob_atoms AS (
    DELETE FROM vlob_atom
    WHERE realm in (select * from deleted_realms)
    RETURNING _id
),
deleted_blocks AS (
    DELETE FROM block
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id, block_id
),
deleted_realm_keys_bundle AS (
    DELETE FROM realm_keys_bundle
    WHERE realm in (select * from deleted_realms)
    RETURNING _id
),
deleted_realm_keys_bundle_access AS (
    DELETE FROM realm_keys_bundle_access
    WHERE realm in (select * from deleted_realms)
    RETURNING _id
),
deleted_realm_sequester_keys_bundle_access AS (
    DELETE FROM realm_sequester_keys_bundle_access
    WHERE realm_keys_bundle in (select * from deleted_realm_keys_bundle)
    RETURNING _id
),
deleted_realm_names AS (
    DELETE FROM realm_name
    WHERE realm in (select * from deleted_realms)
    RETURNING _id
),
deleted_realm_vlob_updates AS (
    DELETE FROM realm_vlob_update
    WHERE realm in (select * from deleted_realms)
    RETURNING _id
),
deleted_block_datas AS (
    DELETE FROM block_data
    WHERE organization_id = $organization_id
    RETURNING _id
),
deleted_topics_common AS (
    DELETE FROM common_topic
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
),
deleted_topics_sequester AS (
    DELETE FROM sequester_topic
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
),
deleted_topics_shamir_recovery AS (
    DELETE FROM shamir_recovery_topic
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
),
deleted_topics_realm AS (
    DELETE FROM realm_topic
    WHERE organization in (select * from deleted_organizations)
    OR realm in (select * from deleted_realms)
    RETURNING _id
),
deleted_greeting_sessions AS (
    DELETE FROM greeting_session
    WHERE invitation in (select * from deleted_invitations)
    RETURNING _id
),
deleted_greeting_attempts AS (
    DELETE FROM greeting_attempt
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
),
deleted_greeting_steps AS (
    DELETE FROM greeting_step
    WHERE greeting_attempt in (select * from deleted_greeting_attempts)
    RETURNING _id
),
deleted_shamir_recovery_setups AS (
    DELETE FROM shamir_recovery_setup
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
),
deleted_shamir_recovery_shares AS (
    DELETE FROM shamir_recovery_share
    WHERE organization in (select * from deleted_organizations)
    RETURNING _id
)
SELECT 1
"""
)


q_test_duplicate_organization = Q(
    f"""
WITH new_organization_ids AS (
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
    RETURNING _id
),
new_sequester_services AS (
    INSERT INTO sequester_service (
        organization,
        service_id,
        service_certificate,
        service_label,
        created_on,
        disabled_on,
        webhook_url,
        service_type,
        revoked_on,
        sequester_revoked_service_certificate
    )
    SELECT
        (select * from new_organization_ids),
        service_id,
        service_certificate,
        service_label,
        created_on,
        disabled_on,
        webhook_url,
        service_type,
        revoked_on,
        sequester_revoked_service_certificate
    FROM sequester_service
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id, service_id
),
new_human_ids AS (
    INSERT INTO human (
        organization,
        email,
        label
    )
    SELECT
        (select * from new_organization_ids),
        email,
        label
    FROM human
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id, email
),
new_users AS (
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
        current_profile,
        initial_profile
    )
    SELECT
        (select * from new_organization_ids),
        user_id,
        user_certificate,
        user_certifier,
        created_on,
        revoked_on,
        revoked_user_certificate,
        revoked_user_certifier,
        (
            SELECT _id FROM new_human_ids
            WHERE email = {q_human(_id="user_.human", select="human.email")}
        ),
        redacted_user_certificate,
        current_profile,
        initial_profile
    FROM user_
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id, user_id
),
new_devices AS (
    INSERT INTO device (
        organization,
        user_,
        device_id,
        verify_key,
        device_certificate,
        device_certifier,
        created_on,
        redacted_device_certificate,
        device_label
    )
    SELECT
        (select * from new_organization_ids),
        (
            SELECT _id FROM new_users
            WHERE user_id = {q_user(_id="device.user_", select="user_id")}
        ),
        device_id,
        verify_key,
        device_certificate,
        device_certifier,
        created_on,
        redacted_device_certificate,
        device_label
    FROM device
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id, device_id
),
patched_user_certifiers AS (
    UPDATE user_
    SET user_certifier = (
        SELECT _id FROM new_devices
        WHERE device_id = {q_device(_id="user_certifier", select="device_id")}
    )
    WHERE user_certifier IS NOT NULL
    RETURNING _id
),
patched_revoked_user_certifiers AS (
    UPDATE user_
    SET revoked_user_certifier = (
        SELECT _id FROM new_devices
        WHERE device_id = {q_device(_id="revoked_user_certifier", select="device_id")}
    )
    RETURNING _id
),
patched_device_certifiers AS (
    UPDATE device
    SET device_certifier = (
        SELECT _id FROM new_devices
        WHERE device_id = {q_device(_id="device_certifier", select="device_id")}
    )
    RETURNING _id
),
new_profiles AS (
    INSERT INTO profile (
        user_,
        profile,
        profile_certificate,
        certified_by,
        certified_on
    )
    SELECT
        (
            SELECT _id FROM new_users
            WHERE user_id = {q_user(_id="profile.user_", select="user_id")}
        ),
        profile,
        profile_certificate,
        (
            SELECT _id FROM new_devices
            WHERE device_id = {q_device(_id="profile.certified_by", select="device_id")}
        ),
        certified_on
    FROM profile
    INNER JOIN user_ ON profile.user_ = user_._id
    WHERE user_.organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_shamir_recovery_setups AS (
    INSERT INTO shamir_recovery_setup (
        organization,
        user_,
        brief_certificate,
        reveal_token,
        threshold,
        shares,
        ciphered_data,
        created_on,
        deleted_on,
        deletion_certificate
    )
    SELECT
        (select * from new_organization_ids),
        (
            SELECT _id FROM new_users
            WHERE user_id = {q_user(_id="shamir_recovery_setup.user_", select="user_id")}
        ),
        brief_certificate,
        reveal_token,
        threshold,
        shares,
        ciphered_data,
        created_on,
        deleted_on,
        deletion_certificate
    FROM shamir_recovery_setup
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id, reveal_token
),
new_shamir_recovery_shares AS (
    INSERT INTO shamir_recovery_share (
        organization,
        shamir_recovery,
        recipient,
        share_certificate,
        shares
    )
    SELECT
        (select * from new_organization_ids),
        (
            SELECT _id FROM new_shamir_recovery_setups
            WHERE reveal_token = (
                SELECT reveal_token FROM shamir_recovery_setup
                WHERE shamir_recovery_setup._id = shamir_recovery_share.shamir_recovery
            )
        ),
        (
            SELECT _id FROM new_users
            WHERE user_id = {q_user(_id="shamir_recovery_share.recipient", select="user_id")}
        ),
        share_certificate,
        shares
    FROM shamir_recovery_share
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_invitations AS (
    INSERT INTO invitation (
        organization,
        token,
        type,
        created_by_device,
        created_by_service_label,
        user_invitation_claimer_email,
        device_invitation_claimer,
        created_on,
        deleted_on,
        deleted_reason,
        shamir_recovery
    )
    SELECT
        (select * from new_organization_ids),
        token,
        type,
        (
            SELECT _id FROM new_devices
            WHERE device_id = {q_device(_id="invitation.created_by_device", select="device_id")}
        ),
        created_by_service_label,
        user_invitation_claimer_email,
        (
            SELECT _id FROM new_users
            WHERE user_id = {q_user(_id="invitation.device_invitation_claimer", select="user_id")}
        ),
        created_on,
        deleted_on,
        deleted_reason,
        (
            SELECT _id FROM new_shamir_recovery_setups
            WHERE reveal_token = (
                SELECT reveal_token FROM shamir_recovery_setup
                WHERE shamir_recovery_setup._id = invitation.shamir_recovery
            )
        )
    FROM invitation
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id, token
),
new_realms AS (
    INSERT INTO realm (
        organization,
        realm_id,
        key_index,
        created_on
    )
    SELECT
        (select * from new_organization_ids),
        realm_id,
        key_index,
        created_on
    FROM realm
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id, realm_id
),
new_realm_user_roles AS (
    INSERT INTO realm_user_role (
        realm,
        user_,
        role,
        certificate,
        certified_by,
        certified_on
    )
    SELECT
        (
            SELECT _id FROM new_realms
            WHERE realm_id = {q_realm(_id="realm_user_role.realm", select="realm_id")}
        ),
        (
            SELECT _id FROM new_users
            WHERE user_id = {q_user(_id="realm_user_role.user_", select="user_id")}
        ),
        role,
        certificate,
        (
            SELECT _id FROM new_devices
            WHERE device_id = {q_device(_id="realm_user_role.certified_by", select="device_id")}
        ),
        certified_on
    FROM realm_user_role
    INNER JOIN realm ON realm._id = realm_user_role.realm
    WHERE realm.organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_vlob_atoms AS (
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
        (
            SELECT _id FROM new_realms
            WHERE realm_id = {q_realm(_id="vlob_atom.realm", select="realm_id")}
        ),
        vlob_atom.key_index,
        vlob_atom.vlob_id,
        vlob_atom.version,
        vlob_atom.blob,
        vlob_atom.size,
        (
            SELECT _id FROM new_devices
            WHERE device_id = {q_device(_id="vlob_atom.author", select="device_id")}
        ),
        vlob_atom.created_on
    FROM vlob_atom
    INNER JOIN realm ON realm._id = vlob_atom.realm
    WHERE realm.organization = {q_organization_internal_id("$source_id")}
    returning _id, vlob_id, version
),
new_blocks AS (
    INSERT INTO block (
        organization,
        block_id,
        realm,
        author,
        size,
        created_on,
        deleted_on,
        key_index
    )
    SELECT
        (select * from new_organization_ids),
        block_id,
        (
            SELECT _id FROM new_realms
            WHERE realm_id = {q_realm(_id="block.realm", select="realm_id")}
        ),
        (
            SELECT _id FROM new_devices
            WHERE device_id = {q_device(_id="block.author", select="device_id")}
        ),
        size,
        created_on,
        deleted_on,
        key_index
    FROM block
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_realm_keys_bundle AS (
    INSERT INTO realm_keys_bundle (
        realm,
        key_index,
        realm_key_rotation_certificate,
        certified_by,
        certified_on,
        key_canary,
        keys_bundle
    )
    SELECT
        (
            SELECT _id FROM new_realms
            WHERE realm_id = {q_realm(_id="realm_keys_bundle.realm", select="realm_id")}
        ),
        realm_keys_bundle.key_index,
        realm_key_rotation_certificate,
        (
            SELECT _id FROM new_devices
            WHERE device_id = {q_device(_id="realm_keys_bundle.certified_by", select="device_id")}
        ),
        certified_on,
        key_canary,
        keys_bundle
    FROM realm_keys_bundle
    INNER JOIN realm ON realm._id = realm_keys_bundle.realm
    WHERE realm.organization = {q_organization_internal_id("$source_id")}
    RETURNING _id, realm, key_index
),
new_realm_keys_bundle_access AS (
    INSERT INTO realm_keys_bundle_access (
        realm,
        user_,
        realm_keys_bundle,
        access,
        from_sharing
    )
    SELECT
        (
            SELECT _id FROM new_realms
            WHERE realm_id = {q_realm(_id="realm_keys_bundle_access.realm", select="realm_id")}
        ),
        (
            SELECT _id FROM new_users
            WHERE user_id = {q_user(_id="realm_keys_bundle_access.user_", select="user_id")}
        ),
        (
            SELECT _id FROM new_realm_keys_bundle
            WHERE realm = (
                SELECT _id FROM new_realms
                WHERE realm_id = {q_realm(_id="realm_keys_bundle_access.realm", select="realm_id")}
            )
            AND key_index = (
                SELECT key_index FROM realm_keys_bundle
                WHERE _id = realm_keys_bundle
            )
        ),
        access,
        (
            SELECT _id FROM realm_user_role
            WHERE certificate = (
                SELECT certificate FROM realm_user_role
                WHERE _id = from_sharing
            )
        )
    FROM realm_keys_bundle_access
    INNER JOIN realm ON realm._id = realm_keys_bundle_access.realm
    WHERE realm.organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_realm_sequester_keys_bundle_access AS (
    INSERT INTO realm_sequester_keys_bundle_access (
        realm,
        sequester_service,
        realm_keys_bundle,
        access
    )
    SELECT
        (
            SELECT _id FROM new_realms
            WHERE realm_id = {q_realm(_id="realm_sequester_keys_bundle_access.realm", select="realm_id")}
        ),
        (
            SELECT _id FROM new_sequester_services
            WHERE service_id = {q_sequester_service(_id="realm_sequester_keys_bundle_access.sequester_service", select="service_id")}
        ),
        (
            SELECT _id FROM new_realm_keys_bundle
            WHERE realm = (
                SELECT _id FROM new_realms
                WHERE realm_id = {q_realm(_id="realm_sequester_keys_bundle_access.realm", select="realm_id")}
            )
            AND key_index = (
                SELECT key_index FROM realm_keys_bundle
                WHERE _id = realm_keys_bundle
            )
        ),
        access
    FROM realm_sequester_keys_bundle_access
    INNER JOIN realm ON realm._id = realm_sequester_keys_bundle_access.realm
    WHERE realm.organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_realm_names AS (
    INSERT INTO realm_name (
        realm,
        realm_name_certificate,
        certified_by,
        certified_on
    )
    SELECT
        (
            SELECT _id FROM new_realms
            WHERE realm_id = {q_realm(_id="realm_name.realm", select="realm_id")}
        ),
        realm_name_certificate,
        (
            SELECT _id FROM new_devices
            WHERE device_id = {q_device(_id="realm_name.certified_by", select="device_id")}
        ),
        certified_on
    FROM realm_name
    INNER JOIN realm ON realm._id = realm_name.realm
    WHERE realm.organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_realm_vlob_updates AS (
    INSERT INTO realm_vlob_update (
        realm,
        index,
        vlob_atom
    )
    SELECT
        (
            SELECT _id FROM new_realms
            WHERE realm_id = {q_realm(_id="realm_vlob_update.realm", select="realm_id")}
        ),
        index,
        (
            SELECT _id FROM new_vlob_atoms
            WHERE vlob_id = (
                SELECT vlob_id FROM vlob_atom
                WHERE _id = realm_vlob_update.vlob_atom
            )
            AND version = (
                SELECT version FROM vlob_atom
                WHERE _id = realm_vlob_update.vlob_atom
            )
        )
    FROM realm_vlob_update
    INNER JOIN realm ON realm._id = realm_vlob_update.realm
    WHERE realm.organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_block_data AS (
    INSERT INTO block_data (
        organization_id,
        block_id,
        data
    )
    SELECT
        $target_id,
        block_id,
        data
    FROM block_data
    WHERE organization_id = $source_id
    RETURNING _id
),
new_topics_common AS (
    INSERT INTO common_topic (
        organization,
        last_timestamp
    )
    SELECT
        (select * from new_organization_ids),
        last_timestamp
    FROM common_topic
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_topics_sequester AS (
    INSERT INTO sequester_topic (
        organization,
        last_timestamp
    )
    SELECT
        (select * from new_organization_ids),
        last_timestamp
    FROM sequester_topic
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_topics_shamir_recovery AS (
    INSERT INTO shamir_recovery_topic (
        organization,
        last_timestamp
    )
    SELECT
        (select * from new_organization_ids),
        last_timestamp
    FROM shamir_recovery_topic
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_topics_realm AS (
    INSERT INTO realm_topic (
        organization,
        realm,
        last_timestamp
    )
    SELECT
        (select * from new_organization_ids),
        (
            SELECT _id FROM new_realms
            WHERE realm_id = {q_realm(_id="realm_topic.realm", select="realm_id")}
        ),
        last_timestamp
    FROM realm_topic
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
),
new_greeting_sessions AS (
    INSERT INTO greeting_session (
        invitation,
        greeter
    )
    SELECT
        (
            SELECT _id FROM new_invitations
            WHERE token = {q_invitation(_id="greeting_session.invitation", select="token")}
        ),
        (
            SELECT _id FROM new_users
            WHERE user_id = {q_user(_id="greeting_session.greeter", select="user_id")}
        )
    FROM greeting_session
    INNER JOIN invitation ON invitation._id = greeting_session.invitation
    WHERE invitation.organization = {q_organization_internal_id("$source_id")}
    RETURNING _id, invitation, greeter
),
new_greeting_attempts AS (
    INSERT INTO greeting_attempt (
        organization,
        greeting_attempt_id,
        greeting_session,
        claimer_joined,
        greeter_joined,
        cancelled_reason,
        cancelled_on,
        cancelled_by
    )
    SELECT
        (select * from new_organization_ids),
        greeting_attempt_id,
        (
            SELECT new_greeting_sessions._id FROM new_greeting_sessions
            INNER JOIN new_invitations ON new_invitations._id = new_greeting_sessions.invitation
            INNER JOIN new_users ON new_users._id = new_greeting_sessions.greeter
            WHERE new_invitations.token = {q_invitation(_id="greeting_session.invitation", select="token")}
            AND new_users.user_id = {q_user(_id="greeting_session.greeter", select="user_id")}
        ),
        claimer_joined,
        greeter_joined,
        cancelled_reason,
        cancelled_on,
        cancelled_by
    FROM greeting_attempt
    INNER JOIN greeting_session ON greeting_session._id = greeting_attempt.greeting_session
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id, greeting_attempt_id
),
new_greeting_steps AS (
    INSERT INTO greeting_step (
        greeting_attempt,
        step,
        greeter_data,
        claimer_data
    )
    SELECT
        (
            SELECT _id FROM new_greeting_attempts
            WHERE greeting_attempt_id = (
                SELECT greeting_attempt_id FROM greeting_attempt
                WHERE greeting_attempt._id = greeting_step.greeting_attempt
            )
        ),
        step,
        greeter_data,
        claimer_data
    FROM greeting_step
    INNER JOIN greeting_attempt ON greeting_attempt._id = greeting_step.greeting_attempt
    WHERE organization = {q_organization_internal_id("$source_id")}
    RETURNING _id
)
SELECT 1
"""
)

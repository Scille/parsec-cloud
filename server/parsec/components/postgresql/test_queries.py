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
WITH
deleted_organizations AS (  --noqa: ST03
    DELETE FROM organization
    WHERE organization_id = $organization_id
    RETURNING _id
),

deleted_sequester_service AS (  --noqa: ST03
    DELETE FROM sequester_service
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_human AS (  --noqa: ST03
    DELETE FROM human
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_users AS (  --noqa: ST03
    DELETE FROM user_
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_devices AS (  --noqa: ST03
    DELETE FROM device
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_profiles AS (  --noqa: ST03
    DELETE FROM profile
    WHERE user_ IN (SELECT * FROM deleted_users)
    RETURNING _id
),

deleted_invitations AS (  --noqa: ST03
    DELETE FROM invitation
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_realms AS (  --noqa: ST03
    DELETE FROM realm
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_realm_user_roles AS (  --noqa: ST03
    DELETE FROM realm_user_role
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_vlob_atoms AS (  --noqa: ST03
    DELETE FROM vlob_atom
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_blocks AS (  --noqa: ST03
    DELETE FROM block
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id, block_id
),

deleted_realm_keys_bundle AS (  --noqa: ST03
    DELETE FROM realm_keys_bundle
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_realm_keys_bundle_access AS (  --noqa: ST03
    DELETE FROM realm_keys_bundle_access
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_realm_sequester_keys_bundle_access AS (  --noqa: ST03
    DELETE FROM realm_sequester_keys_bundle_access
    WHERE realm_keys_bundle IN (SELECT * FROM deleted_realm_keys_bundle)
    RETURNING _id
),

deleted_realm_names AS (  --noqa: ST03
    DELETE FROM realm_name
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_realm_vlob_updates AS (  --noqa: ST03
    DELETE FROM realm_vlob_update
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_block_datas AS (  --noqa: ST03
    DELETE FROM block_data
    WHERE organization_id = $organization_id
    RETURNING _id
),

deleted_topics_common AS (  --noqa: ST03
    DELETE FROM common_topic
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_topics_sequester AS (  --noqa: ST03
    DELETE FROM sequester_topic
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_topics_shamir_recovery AS (  --noqa: ST03
    DELETE FROM shamir_recovery_topic
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_topics_realm AS (  --noqa: ST03
    DELETE FROM realm_topic
    WHERE
        organization IN (SELECT * FROM deleted_organizations)
        OR realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_greeting_sessions AS (  --noqa: ST03
    DELETE FROM greeting_session
    WHERE invitation IN (SELECT * FROM deleted_invitations)
    RETURNING _id
),

deleted_greeting_attempts AS (  --noqa: ST03
    DELETE FROM greeting_attempt
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_greeting_steps AS (  --noqa: ST03
    DELETE FROM greeting_step
    WHERE greeting_attempt IN (SELECT * FROM deleted_greeting_attempts)
    RETURNING _id
),

deleted_shamir_recovery_setups AS (  --noqa: ST03
    DELETE FROM shamir_recovery_setup
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_shamir_recovery_shares AS (  --noqa: ST03
    DELETE FROM shamir_recovery_share
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
)

SELECT 1
"""
)


q_test_duplicate_organization = Q(
    f"""
WITH
new_organization_ids AS (  -- noqa: ST03
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
        minimum_archiving_period,
        allowed_client_agent,
        account_vault_strategy
    )
    SELECT
        $target_id AS organization_id,
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
        minimum_archiving_period,
        allowed_client_agent,
        account_vault_strategy
    FROM organization
    WHERE organization_id = $source_id
    RETURNING _id
),

new_sequester_services AS (  -- noqa: ST03
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
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
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
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    -- Order by _id so that the new items are inserted in the same order than
    -- the original ones. This is not strictly necessary, but it ensures that
    -- a row with a smaller `_id` also has an older `created_on` (without the
    -- order by clause, this can randomly no longer be the case depending on
    -- how PostgreSQL decide to store rows...).
    ORDER BY _id
    RETURNING _id, service_id
),

new_human_ids AS (  -- noqa: ST03
    INSERT INTO human (
        organization,
        email,
        label
    )
    SELECT
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        email,
        label
    FROM human
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id, email
),

new_users AS (  -- noqa: ST03
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
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        user_id,
        user_certificate,
        user_certifier,
        created_on,
        revoked_on,
        revoked_user_certificate,
        revoked_user_certifier,
        (
            SELECT new_human_ids._id FROM new_human_ids
            WHERE new_human_ids.email = {q_human(_id="user_.human", select="email")}  -- noqa: LT05,LT14
        ) AS human,
        redacted_user_certificate,
        current_profile,
        initial_profile
    FROM user_
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id, user_id
),

new_devices AS (  -- noqa: ST03
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
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        (
            SELECT new_users._id FROM new_users
            WHERE new_users.user_id = {q_user(_id="device.user_", select="user_id")}  -- noqa: LT05,LT14
        ) AS user_,
        device_id,
        verify_key,
        device_certificate,
        device_certifier,
        created_on,
        redacted_device_certificate,
        device_label
    FROM device
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id, device_id
),

patched_user_certifiers AS (  -- noqa: ST03
    UPDATE user_
    SET
        user_certifier = (
            SELECT _id
            FROM new_devices
            WHERE device_id = {q_device(_id="user_.user_certifier", select="device_id")}  -- noqa: LT05,LT14
        )
    WHERE
        organization = (SELECT new_organization_ids._id FROM new_organization_ids)
        AND user_certifier IS NOT NULL
    RETURNING _id
),

patched_revoked_user_certifiers AS (  -- noqa: ST03
    UPDATE user_
    SET
        revoked_user_certifier = (
            SELECT _id
            FROM new_devices
            WHERE device_id = {q_device(_id="user_.revoked_user_certifier", select="device_id")}  -- noqa: LT05,LT14
        )
    WHERE
        organization = (SELECT new_organization_ids._id FROM new_organization_ids)
        AND revoked_user_certifier IS NOT NULL
    RETURNING _id
),

patched_device_certifiers AS (  -- noqa: ST03
    UPDATE device
    SET
        device_certifier = (
            SELECT _id
            FROM new_devices
            WHERE device_id = {q_device(_id="device.device_certifier", select="device_id")}  -- noqa: LT05,LT14
        )
    WHERE
        organization = (SELECT new_organization_ids._id FROM new_organization_ids)
        AND device_certifier IS NOT NULL
    RETURNING _id
),

new_profiles AS (  -- noqa: ST03
    INSERT INTO profile (
        user_,
        profile,
        profile_certificate,
        certified_by,
        certified_on
    )
    SELECT
        (
            SELECT new_users._id
            FROM new_users
            WHERE new_users.user_id = user_.user_id
        ) AS user_,
        profile.profile,
        profile.profile_certificate,
        (
            SELECT new_devices._id
            FROM new_devices
            WHERE new_devices.device_id = {q_device(_id="profile.certified_by", select="device_id")}  -- noqa: LT05,LT14
        ) AS certified_by,
        profile.certified_on
    FROM profile
    INNER JOIN user_ ON profile.user_ = user_._id
    WHERE user_.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY profile._id
    RETURNING _id
),

new_shamir_recovery_setups AS (  -- noqa: ST03
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
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        (
            SELECT new_users._id FROM new_users
            WHERE new_users.user_id = {q_user(_id="shamir_recovery_setup.user_", select="user_id")}  -- noqa: LT05,LT14
        ) AS user_,
        brief_certificate,
        reveal_token,
        threshold,
        shares,
        ciphered_data,
        created_on,
        deleted_on,
        deletion_certificate
    FROM shamir_recovery_setup
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id, reveal_token
),

new_shamir_recovery_shares AS (  -- noqa: ST03
    INSERT INTO shamir_recovery_share (
        organization,
        shamir_recovery,
        recipient,
        share_certificate,
        shares
    )
    SELECT
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        (
            SELECT new_shamir_recovery_setups._id
            FROM new_shamir_recovery_setups
            WHERE new_shamir_recovery_setups.reveal_token = (
                SELECT shamir_recovery_setup.reveal_token
                FROM shamir_recovery_setup
                WHERE shamir_recovery_setup._id = shamir_recovery_share.shamir_recovery
            )
        ) AS shamir_recovery,
        (
            SELECT new_users._id
            FROM new_users
            WHERE new_users.user_id = {q_user(_id="shamir_recovery_share.recipient", select="user_id")}  -- noqa: LT05,LT14
        ) AS recipient,
        share_certificate,
        shares
    FROM shamir_recovery_share
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id
),

new_invitations AS (  -- noqa: ST03
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
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        token,
        type,
        (
            SELECT new_devices._id
            FROM new_devices
            WHERE new_devices.device_id = {q_device(_id="invitation.created_by_device", select="device_id")}  -- noqa: LT05,LT14
        ) AS created_by_device,
        created_by_service_label,
        user_invitation_claimer_email,
        (
            SELECT new_users._id
            FROM new_users
            WHERE new_users.user_id = {q_user(_id="invitation.device_invitation_claimer", select="user_id")}  -- noqa: LT05,LT14
        ) AS device_invitation_claimer,
        created_on,
        deleted_on,
        deleted_reason,
        (
            SELECT new_shamir_recovery_setups._id
            FROM new_shamir_recovery_setups
            WHERE new_shamir_recovery_setups.reveal_token = (
                SELECT shamir_recovery_setup.reveal_token
                FROM shamir_recovery_setup
                WHERE shamir_recovery_setup._id = invitation.shamir_recovery
            )
        ) AS shamir_recovery
    FROM invitation
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id, token
),

new_realms AS (  -- noqa: ST03
    INSERT INTO realm (
        organization,
        realm_id,
        key_index,
        created_on
    )
    SELECT
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        realm_id,
        key_index,
        created_on
    FROM realm
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id, realm_id
),

new_realm_user_roles AS (  -- noqa: ST03
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
            SELECT new_realms._id
            FROM new_realms
            WHERE new_realms.realm_id = realm.realm_id
        ) AS realm,
        (
            SELECT new_users._id
            FROM new_users
            WHERE new_users.user_id = {q_user(_id="realm_user_role.user_", select="user_id")}  -- noqa: LT05,LT14
        ) AS user_,
        realm_user_role.role,
        realm_user_role.certificate,
        (
            SELECT new_devices._id
            FROM new_devices
            WHERE new_devices.device_id = {q_device(_id="realm_user_role.certified_by", select="device_id")}  -- noqa: LT05,LT14
        ) AS certified_by,
        realm_user_role.certified_on
    FROM realm_user_role
    INNER JOIN realm ON realm_user_role.realm = realm._id
    WHERE realm.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY realm_user_role._id
    RETURNING _id
),

new_vlob_atoms AS (  -- noqa: ST03
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
            SELECT new_realms._id
            FROM new_realms
            WHERE new_realms.realm_id = realm.realm_id
        ) AS realm,
        vlob_atom.key_index,
        vlob_atom.vlob_id,
        vlob_atom.version,
        vlob_atom.blob,
        vlob_atom.size,
        (
            SELECT new_devices._id
            FROM new_devices
            WHERE new_devices.device_id = {q_device(_id="vlob_atom.author", select="device_id")}  -- noqa: LT05,LT14
        ) AS author,
        vlob_atom.created_on
    FROM vlob_atom
    INNER JOIN realm ON vlob_atom.realm = realm._id
    WHERE realm.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY vlob_atom._id
    RETURNING _id, vlob_id, version
),

new_blocks AS (  -- noqa: ST03
    INSERT INTO block (
        block_id,
        realm,
        author,
        size,
        created_on,
        deleted_on,
        key_index
    )
    SELECT
        block.block_id,
        (
            SELECT new_realms._id
            FROM new_realms
            WHERE new_realms.realm_id = realm.realm_id
        ) AS realm,
        (
            SELECT new_devices._id
            FROM new_devices
            WHERE new_devices.device_id = {q_device(_id="block.author", select="device_id")}  -- noqa: LT05,LT14
        ) AS author,
        block.size,
        block.created_on,
        block.deleted_on,
        block.key_index
    FROM block
    INNER JOIN realm ON block.realm = realm._id
    WHERE realm.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY block._id
    RETURNING block._id
),

new_realm_keys_bundle AS (  -- noqa: ST03
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
            SELECT new_realms._id
            FROM new_realms
            WHERE new_realms.realm_id = realm.realm_id
        ) AS realm,
        realm_keys_bundle.key_index,
        realm_keys_bundle.realm_key_rotation_certificate,
        (
            SELECT new_devices._id
            FROM new_devices
            WHERE new_devices.device_id = {q_device(_id="realm_keys_bundle.certified_by", select="device_id")}  -- noqa: LT05,LT14
        ) AS certified_by,
        realm_keys_bundle.certified_on,
        realm_keys_bundle.key_canary,
        realm_keys_bundle.keys_bundle
    FROM realm_keys_bundle
    INNER JOIN realm ON realm_keys_bundle.realm = realm._id
    WHERE realm.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY realm_keys_bundle._id
    RETURNING _id, realm, key_index
),

new_realm_keys_bundle_access AS (  -- noqa: ST03
    INSERT INTO realm_keys_bundle_access (
        realm,
        user_,
        realm_keys_bundle,
        access,
        from_sharing
    )
    SELECT
        (
            SELECT new_realms._id
            FROM new_realms
            WHERE new_realms.realm_id = realm.realm_id
        ) AS realm,
        (
            SELECT new_users._id
            FROM new_users
            WHERE new_users.user_id = {q_user(_id="realm_keys_bundle_access.user_", select="user_id")}  -- noqa: LT05,LT14
        ) AS user_,
        (
            SELECT new_realm_keys_bundle._id
            FROM new_realm_keys_bundle
            WHERE
                new_realm_keys_bundle.key_index = (
                    SELECT realm_keys_bundle.key_index
                    FROM realm_keys_bundle
                    WHERE realm_keys_bundle._id = realm_keys_bundle_access.realm_keys_bundle
                )
        ) AS realm_keys_bundle,
        realm_keys_bundle_access.access,
        (
            SELECT realm_user_role._id
            FROM realm_user_role
            WHERE
                realm_user_role.realm = realm._id
                AND realm_user_role.certificate = (
                    SELECT rur2.certificate
                    FROM realm_user_role AS rur2
                    WHERE rur2._id = realm_keys_bundle_access.from_sharing
                )
        ) AS from_sharing
    FROM realm_keys_bundle_access
    INNER JOIN realm ON realm_keys_bundle_access.realm = realm._id
    WHERE realm.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY realm_keys_bundle_access._id
    RETURNING _id
),

new_realm_sequester_keys_bundle_access AS (  -- noqa: ST03
    INSERT INTO realm_sequester_keys_bundle_access (
        realm,
        sequester_service,
        realm_keys_bundle,
        access
    )
    SELECT
        (
            SELECT new_realms._id
            FROM new_realms
            WHERE new_realms.realm_id = realm.realm_id
        ) AS realm,
        (
            SELECT new_sequester_services._id
            FROM new_sequester_services
            WHERE new_sequester_services.service_id = {q_sequester_service(_id="realm_sequester_keys_bundle_access.sequester_service", select="service_id")}  -- noqa: LT05,LT14
        ) AS sequester_service,
        (
            SELECT new_realm_keys_bundle._id
            FROM new_realm_keys_bundle
            WHERE new_realm_keys_bundle.realm = (
                SELECT new_realms._id
                FROM new_realms
                WHERE new_realms.realm_id = realm.realm_id
            )
            AND new_realm_keys_bundle.key_index = (
                SELECT realm_keys_bundle.key_index
                FROM realm_keys_bundle
                WHERE realm_keys_bundle._id = realm_sequester_keys_bundle_access.realm_keys_bundle
            )
        ) AS realm_keys_bundle,
        realm_sequester_keys_bundle_access.access
    FROM realm_sequester_keys_bundle_access
    INNER JOIN realm ON realm_sequester_keys_bundle_access.realm = realm._id
    WHERE realm.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY realm_sequester_keys_bundle_access._id
    RETURNING _id
),

new_realm_names AS (  -- noqa: ST03
    INSERT INTO realm_name (
        realm,
        realm_name_certificate,
        certified_by,
        certified_on
    )
    SELECT
        (
            SELECT new_realms._id
            FROM new_realms
            WHERE new_realms.realm_id = {q_realm(_id="realm_name.realm", select="realm_id")}  -- noqa: LT05,LT14
        ) AS realm,
        realm_name.realm_name_certificate,
        (
            SELECT new_devices._id
            FROM new_devices
            WHERE new_devices.device_id = {q_device(_id="realm_name.certified_by", select="device_id")}  -- noqa: LT05,LT14
        ) AS certified_by,
        realm_name.certified_on
    FROM realm_name
    INNER JOIN realm ON realm_name.realm = realm._id
    WHERE realm.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY realm_name._id
    RETURNING _id
),

new_realm_vlob_updates AS (  -- noqa: ST03
    INSERT INTO realm_vlob_update (
        realm,
        index,
        vlob_atom
    )
    SELECT
        (
            SELECT new_realms._id FROM new_realms
            WHERE new_realms.realm_id = {q_realm(_id="realm_vlob_update.realm", select="realm_id")}  -- noqa: LT05,LT14
        ) AS realm,
        realm_vlob_update.index,
        (
            SELECT new_vlob_atoms._id
            FROM new_vlob_atoms
            WHERE
                new_vlob_atoms.vlob_id = (
                    SELECT vlob_atom.vlob_id
                    FROM vlob_atom
                    WHERE vlob_atom._id = realm_vlob_update.vlob_atom
                )
                AND new_vlob_atoms.version = (
                    SELECT vlob_atom.version
                    FROM vlob_atom
                    WHERE vlob_atom._id = realm_vlob_update.vlob_atom
                )
        ) AS vlob_atom
    FROM realm_vlob_update
    INNER JOIN realm ON realm_vlob_update.realm = realm._id
    WHERE realm.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY realm_vlob_update._id
    RETURNING _id
),

new_block_data AS (  -- noqa: ST03
    INSERT INTO block_data (
        organization_id,
        block_id,
        data
    )
    SELECT
        $target_id AS organization_id,
        block_id,
        data
    FROM block_data
    WHERE organization_id = $source_id
    ORDER BY _id
    RETURNING _id
),

new_topics_common AS (  -- noqa: ST03
    INSERT INTO common_topic (
        organization,
        last_timestamp
    )
    SELECT
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        last_timestamp
    FROM common_topic
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id
),

new_topics_sequester AS (  -- noqa: ST03
    INSERT INTO sequester_topic (
        organization,
        last_timestamp
    )
    SELECT
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        last_timestamp
    FROM sequester_topic
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id
),

new_topics_shamir_recovery AS (  -- noqa: ST03
    INSERT INTO shamir_recovery_topic (
        organization,
        last_timestamp
    )
    SELECT
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        last_timestamp
    FROM shamir_recovery_topic
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id
),

new_topics_realm AS (  -- noqa: ST03
    INSERT INTO realm_topic (
        organization,
        realm,
        last_timestamp
    )
    SELECT
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        (
            SELECT new_realms._id
            FROM new_realms
            WHERE new_realms.realm_id = {q_realm(_id="realm_topic.realm", select="realm_id")}  -- noqa: LT05,LT14
        ) AS realm,
        last_timestamp
    FROM realm_topic
    WHERE organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY _id
    RETURNING _id
),

new_greeting_sessions AS (  -- noqa: ST03
    INSERT INTO greeting_session (
        invitation,
        greeter
    )
    SELECT
        (
            SELECT new_invitations._id
            FROM new_invitations
            WHERE new_invitations.token = {q_invitation(_id="greeting_session.invitation", select="token")}  -- noqa: LT05,LT14
        ) AS invitation,
        (
            SELECT new_users._id
            FROM new_users
            WHERE new_users.user_id = {q_user(_id="greeting_session.greeter", select="user_id")}  -- noqa: LT05,LT14
        ) AS greeter
    FROM greeting_session
    INNER JOIN invitation ON greeting_session.invitation = invitation._id
    WHERE invitation.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY greeting_session._id
    RETURNING _id, invitation, greeter
),

new_greeting_attempts AS (  -- noqa: ST03
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
        (SELECT new_organization_ids._id FROM new_organization_ids) AS organization,
        greeting_attempt.greeting_attempt_id,
        (
            SELECT new_greeting_sessions._id FROM new_greeting_sessions
            INNER JOIN new_invitations ON new_greeting_sessions.invitation = new_invitations._id
            INNER JOIN new_users ON new_greeting_sessions.greeter = new_users._id
            WHERE
                new_invitations.token = {q_invitation(_id="greeting_session.invitation", select="token")}  -- noqa: LT05,LT14
                AND new_users.user_id = {q_user(_id="greeting_session.greeter", select="user_id")}  -- noqa: LT05,LT14
        ) AS greeting_session,
        greeting_attempt.claimer_joined,
        greeting_attempt.greeter_joined,
        greeting_attempt.cancelled_reason,
        greeting_attempt.cancelled_on,
        greeting_attempt.cancelled_by
    FROM greeting_attempt
    INNER JOIN greeting_session ON greeting_attempt.greeting_session = greeting_session._id
    WHERE
        greeting_attempt.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY greeting_attempt._id
    RETURNING _id, greeting_attempt_id
),

new_greeting_steps AS (  -- noqa: ST03
    INSERT INTO greeting_step (
        greeting_attempt,
        step,
        greeter_data,
        claimer_data
    )
    SELECT
        (
            SELECT new_greeting_attempts._id
            FROM new_greeting_attempts
            WHERE new_greeting_attempts.greeting_attempt_id = greeting_attempt.greeting_attempt_id
        ) AS greeting_attempt,
        greeting_step.step,
        greeting_step.greeter_data,
        greeting_step.claimer_data
    FROM greeting_step
    INNER JOIN greeting_attempt ON greeting_step.greeting_attempt = greeting_attempt._id
    WHERE
        greeting_attempt.organization = {q_organization_internal_id("$source_id")}  -- noqa: LT05,LT14
    ORDER BY greeting_step._id
    RETURNING _id
)

SELECT 1
"""
)

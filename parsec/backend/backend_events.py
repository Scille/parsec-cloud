# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class BackendEvents:
    """ Backend internal events"""

    # Device
    device_claimed = "device.claimed"
    device_created = "device.created"
    device_invitation_cancelled = "device.invitation.cancelled"
    # Invitation
    invite_status_changed = "invite.status_changed"
    invite_conduit_updated = "invite.conduit_updated"
    # User
    user_claimed = "user.claimed"
    user_created = "user.created"
    user_revoked = "user.revoked"
    user_invitation_cancelled = "user.invitation.cancelled"
    # Realm
    realm_vlobs_updated = "realm.vlobs_updated"
    realm_roles_updated = "realm.roles_updated"
    realm_maintenance_started = "realm.maintenance_started"
    realm_maintenance_finished = "realm.maintenance_finished"
    # Others
    message_received = "message.received"
    pinged = "pinged"

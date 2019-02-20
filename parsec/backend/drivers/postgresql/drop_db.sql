-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

DROP FUNCTION IF EXISTS get_organization_internal_id(VARCHAR);
DROP FUNCTION IF EXISTS get_user_internal_id(orgid VARCHAR, userid VARCHAR);
DROP FUNCTION IF EXISTS get_device_internal_id(orgid VARCHAR, deviceid VARCHAR);
DROP FUNCTION IF EXISTS get_device_id(deviceinternalid INTEGER);

DROP TABLE IF EXISTS
    organizations,

    users,
    devices,

    user_invitations,
    device_invitations,

    messages,
    vlobs,
    beacons,
    vlobs_per_beacon,

    blockstore
CASCADE;

DROP TYPE IF EXISTS
    USER_INVITATION_STATUS,
    DEVICE_INVITATION_STATUS,
    DEVICE_CONF_TRY_STATUS
CASCADE;

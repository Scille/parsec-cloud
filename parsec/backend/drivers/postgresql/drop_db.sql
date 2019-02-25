-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

DROP FUNCTION IF EXISTS get_organization_internal_id(VARCHAR);
DROP FUNCTION IF EXISTS get_user_internal_id(orgid VARCHAR, userid VARCHAR);
DROP FUNCTION IF EXISTS get_user_id(deviceinternalid INTEGER);
DROP FUNCTION IF EXISTS get_device_internal_id(orgid VARCHAR, deviceid VARCHAR);
DROP FUNCTION IF EXISTS get_device_id(deviceinternalid INTEGER);
DROP FUNCTION IF EXISTS user_can_administrate_beacon(userinternalid INTEGER, beaconinternalid INTEGER);
DROP FUNCTION IF EXISTS user_can_read_beacon(userinternalid INTEGER, beaconinternalid INTEGER);
DROP FUNCTION IF EXISTS user_can_write_beacon(userinternalid INTEGER, beaconinternalid INTEGER);
DROP FUNCTION IF EXISTS get_beacon_internal_id(orgid VARCHAR, beaconid UUID);
DROP FUNCTION IF EXISTS get_vlob_internal_id(orgid VARCHAR, vlobid UUID);
DROP FUNCTION IF EXISTS get_vlob_id(vlobinternalid INTEGER);


DROP TABLE IF EXISTS
    organizations,

    users,
    devices,

    user_invitations,
    device_invitations,

    messages,

    beacons,
    beacon_accesses,
    vlobs,
    vlob_atoms,
    beacon_vlob_atom_updates,

    blockstore
CASCADE;

DROP TYPE IF EXISTS
    USER_INVITATION_STATUS,
    DEVICE_INVITATION_STATUS,
    DEVICE_CONF_TRY_STATUS
CASCADE;

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

################### EventsListen ##################

serializer = events_listen_serializer

serialized = serializer.req_dumps({"cmd": "events_listen", "wait": False})
serializer.req_loads(serialized)
display("events_listen_req", serialized, [])

serialized = serializer.rep_dumps({"event": APIEvent.PINGED, "ping": "foobar"})
serializer.rep_loads(serialized)
display("events_listen_rep_pinged", serialized, [])

serialized = serializer.rep_dumps({"event": APIEvent.MESSAGE_RECEIVED, "index": 0})
serializer.rep_loads(serialized)
display("events_listen_rep_message_received", serialized, [])

serialized = serializer.rep_dumps(
    {
        "event": APIEvent.INVITE_STATUS_CHANGED,
        "invitation_status": InvitationStatus.IDLE,
        "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
    }
)
serializer.rep_loads(serialized)
display("events_listen_rep_invite_status_changed", serialized, [])

serialized = serializer.rep_dumps(
    {
        "event": APIEvent.REALM_MAINTENANCE_FINISHED,
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 0,
    }
)
serializer.rep_loads(serialized)
display("events_listen_rep_realm_maintenance_finished", serialized, [])

serialized = serializer.rep_dumps(
    {
        "event": APIEvent.REALM_MAINTENANCE_STARTED,
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 0,
    }
)
serializer.rep_loads(serialized)
display("events_listen_rep_realm_maintenance_started", serialized, [])

serialized = serializer.rep_dumps(
    {
        "event": APIEvent.REALM_VLOBS_UPDATED,
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "checkpoint": 0,
        "src_id": VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
        "src_version": 0,
    }
)
serializer.rep_loads(serialized)
display("events_listen_rep_realm_vlobs_updated", serialized, [])

serialized = serializer.rep_dumps(
    {
        "event": APIEvent.REALM_ROLES_UPDATED,
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "role": RealmRole.OWNER,
    }
)
serializer.rep_loads(serialized)
display("events_listen_rep_realm_roles_updated", serialized, [])

serialized = serializer.rep_dumps({"status": "cancelled", "reason": "foobar"})
serializer.rep_loads(serialized)
display("events_listen_rep_cancelled", serialized, [])

serialized = serializer.rep_dumps({"status": "no_events"})
serializer.rep_loads(serialized)
display("events_listen_rep_no_events", serialized, [])

################### EventsSubscribe ##################

serializer = events_subscribe_serializer

serialized = serializer.req_dumps({"cmd": "events_subscribe"})
serializer.req_loads(serialized)
display("events_subscribe_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("events_subscribe_rep", serialized, [])

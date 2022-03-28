# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
# flake8: noqa

from pendulum import datetime
from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

################### OrganizationStats ##################

serializer = organization_stats_serializer

serialized = serializer.req_dumps({"cmd": "organization_stats"})
serializer.req_loads(serialized)
display("organization_stats_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "data_size": 8,
        "metadata_size": 8,
        "realms": 1,
        "users": 1,
        "active_users": 1,
        "users_per_profile_detail": [{"profile": UserProfile.ADMIN, "active": 1, "revoked": 0}],
    }
)
serializer.rep_loads(serialized)
display("organization_stats_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed", "reason": "foobar"})
serializer.rep_loads(serialized)
display("organization_stats_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("organization_stats_rep_not_found", serialized, [])

################### OrganizationConfig ##################

serializer = organization_config_serializer

serialized = serializer.req_dumps({"cmd": "organization_config"})
serializer.req_loads(serialized)
display("organization_config_req", serialized, [])

serialized = serializer.rep_dumps({"user_profile_outsider_allowed": False, "active_users_limit": 1})
serializer.rep_loads(serialized)
display("organization_config_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("organization_config_rep_not_found", serialized, [])

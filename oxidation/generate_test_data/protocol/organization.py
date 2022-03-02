# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
# flake8: noqa

from pendulum import datetime
from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

################### OrganizationBootstrap ##################

serializer = apiv1_organization_bootstrap_serializer
serialized = serializer.req_dumps(
    {
        "cmd": "api_v1_organization_bootstrap",
        "bootstrap_token": "foobar",
        "root_verify_key": VerifyKey(
            unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        ),
        "user_certificate": b"foobar",
        "device_certificate": b"foobar",
        "redacted_user_certificate": b"foobar",
        "redacted_device_certificate": b"foobar",
    }
)
serializer.req_loads(serialized)
display("api_v1_organization_bootstrap_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("api_v1_organization_bootstrap_rep", serialized, [])

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

################### OrganizationConfig ##################

serializer = organization_config_serializer

serialized = serializer.req_dumps({"cmd": "organization_config"})
serializer.req_loads(serialized)
display("organization_config_req", serialized, [])

serialized = serializer.rep_dumps({"user_profile_outsider_allowed": False, "active_users_limit": 1})
serializer.rep_loads(serialized)
display("organization_config_rep", serialized, [])

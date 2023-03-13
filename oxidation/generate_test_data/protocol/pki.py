# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from utils import *

from parsec._parsec import DateTime, EnrollmentID
from parsec.api.protocol import *
from parsec.api.data import *

################### Pki submit ##################
serializer = pki_enrollment_submit_serializer
serialized = serializer.req_dumps(
    {
        "cmd": "pki_enrollment_submit",
        "enrollment_id": EnrollmentID.new(),
        "force": False,
        "submit_payload": b"dummy",
        "submit_payload_signature": b"dummy",
        "submitter_der_x509_certificate": b"dummy",
        "submitter_der_x509_certificate_email": "mail@mail.com",
    }
)
serializer.req_loads(serialized)
display("pki enrollment submit req", serialized, [])
serialized = serializer.rep_dumps({"status": "ok", "submitted_on": DateTime.now()})
serializer.rep_loads(serialized)
display("pki enrollment submit rep", serialized, [])

################### Pki info ##################
serializer = pki_enrollment_info_serializer
serialized = serializer.req_dumps(
    {
        "cmd": "pki_enrollment_info",
        "enrollment_id": EnrollmentID.new(),
    }
)
serializer.req_loads(serialized)
display("pki enrollment info req", serialized, [])

# Accepted
serialized = serializer.rep_dumps(
    {
        "status": "ok",
        "accept_payload": b"dummy",
        "accept_payload_signature": b"dummy",
        "accepted_on": DateTime.now(),
        "accepter_der_x509_certificate": b"dummy",
        "enrollment_status": "ACCEPTED",
        "submitted_on": DateTime.now(),
    }
)
serializer.rep_loads(serialized)
display("pki enrollment info accepted rep", serialized, [])

# Cancelled
serialized = serializer.rep_dumps(
    {
        "status": "ok",
        "cancelled_on": DateTime.now(),
        "enrollment_status": "CANCELLED",
        "submitted_on": DateTime.now(),
    }
)
serializer.rep_loads(serialized)
display("pki enrollment info cancelled rep", serialized, [])

# Rejected
serialized = serializer.rep_dumps(
    {
        "status": "ok",
        "rejected_on": DateTime.now(),
        "enrollment_status": "REJECTED",
        "submitted_on": DateTime.now(),
    }
)
serializer.rep_loads(serialized)
display("pki enrollment info rejected rep", serialized, [])

# Submitted
serialized = serializer.rep_dumps(
    {
        "status": "ok",
        "rejected_on": DateTime.now(),
        "enrollment_status": "SUBMITTED",
        "submitted_on": DateTime.now(),
    }
)
serializer.rep_loads(serialized)
display("pki enrollment info submitted rep", serialized, [])

################### Pki accept ##################
serializer = pki_enrollment_accept_serializer
uuid = EnrollmentID.new()
serialized = serializer.req_dumps(
    {
        "cmd": "pki_enrollment_accept",
        "enrollment_id": uuid,
        "accepter_der_x509_certificate": b"<accepter_der_x509_certificate>",
        "accept_payload_signature": b"<signature>",
        "accept_payload": b"<dummy>",
        "user_certificate": b"<dummy>",
        "device_certificate": b"<dummy>",
        "redacted_user_certificate": b"<dummy>",
        "redacted_device_certificate": b"<dummy>",
    }
)
serializer.req_loads(serialized)
display("pki enrollment accept req", serialized, [])
serialized = serializer.rep_dumps({"status": "ok"})
serializer.rep_loads(serialized)
display("pki enrollment accept rep", serialized, [])

################### Pki list ##################
serializer = pki_enrollment_list_serializer
uuid = EnrollmentID.new()
serialized = serializer.req_dumps({"cmd": "pki_enrollment_list"})
serializer.req_loads(serialized)
display("pki enrollment list req", serialized, [])
serialized = serializer.rep_dumps(
    {
        "enrollments": [
            {
                "enrollment_id": uuid,
                "submit_payload": b"<dummy>",
                "submitter_der_x509_certificate": b"<x509 certif>",
                "submit_payload_signature": b"<signature>",
                "submitted_on": DateTime.from_rfc3339("2022-11-16T10:36:23.390001Z"),
            }
        ],
        "status": "ok",
    }
)
display("pki enrollment list rep", serialized, [])
serialized = serializer.rep_dumps(
    {
        "enrollments": [],
        "status": "ok",
    }
)
serializer.rep_loads(serialized)
display("pki enrollment empty list ok rep", serialized, [])
serialized = serializer.rep_dumps(
    {
        "reason": "oof",
        "status": "not_allowed",
    }
)
serializer.rep_loads(serialized)
display("pki enrollment empty list ok rep", serialized, [])

################### Pki reject ##################
serializer = pki_enrollment_reject_serializer
uuid = EnrollmentID.new()
serialized = serializer.req_dumps({"cmd": "pki_enrollment_reject", "enrollment_id": uuid})
serializer.req_loads(serialized)
display("pki enrollment reject req", serialized, [])
serialized = serializer.rep_dumps({"status": "ok"})
serializer.rep_loads(serialized)
display("pki enrollment reject rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed", "reason": "oof"})
serializer.rep_loads(serialized)
display("pki enrollment reject rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "oof"})
serializer.rep_loads(serialized)
display("pki enrollment reject rep", serialized, [])

serialized = serializer.rep_dumps({"status": "no_longer_available", "reason": "oof"})
serializer.rep_loads(serialized)
display("pki enrollment reject rep", serialized, [])

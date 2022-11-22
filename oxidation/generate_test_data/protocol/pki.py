# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from utils import *

from parsec._parsec import DateTime
from parsec.crypto import *
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

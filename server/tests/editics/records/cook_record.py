# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# The record is a capture from OnlyOffice's demo page, hence the wrong names/ids:
# - John Smith is Alice
# - Kate Cage is Bob
# - F89d8069ba2b is Alice's device ID
# - 78e1e841 is Bob's device ID
# - 78e1e8411 & F89d8069ba2b2 (i.e. participant IDs) gets replaced by a random
#   UUID (since they are expected to change for every new client session)
# - SessionId gets replaced by the participant ID (since in Parsec editics session
#   and participant are two concepts merged together)
#
# Note in the record there is `connectionId` with the same value than `sessionId`,
# but we don't touch it. This is because `connectionId` correspond to the websocket
# connection ID (i.e. transport layer), and the `sessionId` just take this ID for
# a higher layer concept (i.e. client session that survives transport disconnect).
# In any way, `connectionId` is a field never used in the OnlyOffice client,
# the OnlyOffice server is just leaking internal data here.

import re
import sys

ALICE_PARTICIPANT_ID = "4a51295bf2d04b47b6269a8e33b120e0"
BOB_PARTICIPANT_ID = "26dbd085b4c548a48fed5577eecd9709"
BY_NAME = {"John Smith": ALICE_PARTICIPANT_ID, "Kate Cage": BOB_PARTICIPANT_ID}
# Event headings look like `### 06:22:59.876   <-  Kate Cage       cursor`
HEADING_RE = re.compile(r"^### .+?(John Smith|Kate Cage)")

lines = []
participant_id = None  # UUID of the participant that emitted the event being read
for line in open(sys.argv[1]).read().splitlines():
    if heading := HEADING_RE.match(line):
        participant_id = BY_NAME[heading.group(1)]
    line = line.replace("John Smith", "Alice").replace("Kate Cage", "Bob")
    # `userId` + `indexUser` composite ids
    line = line.replace("78e1e8411", ALICE_PARTICIPANT_ID).replace(
        "F89d8069ba2b2", BOB_PARTICIPANT_ID
    )
    if participant_id:
        # `sessionId` is the participant id in the editics protocol, but is not
        # distinguishable between Alice and Bob in the capture, hence the heading lookup.
        line = re.sub(r'"sessionId": "\w+"', f'"sessionId": "{participant_id}"', line)
    # `userId` is the device id in our case
    line = line.replace("F89d8069ba2b", "de10a11cec0010000000000000000000")
    line = line.replace("78e1e841", "de10808c001000000000000000000000")
    lines.append(line)

open(sys.argv[1], "w").write("\n".join(lines) + "\n")

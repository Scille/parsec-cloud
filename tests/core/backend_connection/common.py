# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec.api.protocol import APIV1_ANONYMOUS_CMDS, AUTHENTICATED_CMDS, INVITED_CMDS

ALL_CMDS = AUTHENTICATED_CMDS | INVITED_CMDS | APIV1_ANONYMOUS_CMDS

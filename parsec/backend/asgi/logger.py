# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import base64
import binascii
from typing import TYPE_CHECKING, Mapping

from hypercorn.logging import AccessLogAtoms, Logger

if TYPE_CHECKING:
    from hypercorn.typing import ResponseSummary, WWWScope


class ParsecLogger(Logger):
    def atoms(
        self, request: "WWWScope", response: "ResponseSummary", request_time: float
    ) -> Mapping[str, str]:
        """Create and return an access log atoms dictionary.
        This can be overridden and customised if desired. It should
        return a mapping between an access log format key and a value.
        """
        access_log = AccessLogAtoms(request, response, request_time)
        if "{author}i" in access_log:
            decoded = self._try_decode_base64(access_log["{author}i"])
            if decoded:
                access_log["author"] = decoded

        return access_log

    def _try_decode_base64(self, value: str) -> str | None:
        try:
            return base64.b64decode(value, validate=True).decode()
        except (binascii.Error, UnicodeDecodeError):
            return None

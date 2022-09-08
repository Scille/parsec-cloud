# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from hypercorn.logging import Logger, AccessLogAtoms
from typing import TYPE_CHECKING, Mapping, Iterable, Optional, Tuple, Union

import base64
import binascii

if TYPE_CHECKING:
    from hypercorn.config import Config
    from hypercorn.typing import ResponseSummary, WWWScope


class ParsecLogger(Logger):
    def __init__(self, config: "Config") -> None:
        super().__init__(config)

    def atoms(
        self, request: "WWWScope", response: "ResponseSummary", request_time: float
    ) -> Mapping[str, str]:
        """Create and return an access log atoms dictionary.
        This can be overidden and customised if desired. It should
        return a mapping between an access log format key and a value.
        """
        access_log = AccessLogAtoms(request, response, request_time)
        id_value = self._find_author_header(request["headers"])

        id = self._try_decode_base64(id_value)
        access_log[b"Author".decode("latin1").lower()] = id

        return access_log

    def _find_author_header(self, headers: Iterable[Tuple[bytes, bytes]]) -> Optional[bytes]:
        for (name, value) in headers:
            # latin1 (ISO-8859-1) is standard http charset
            if name.decode("latin1") == "Author":
                return value

        return None

    def _try_decode_base64(self, value: Union[bytes, None]) -> Optional[str]:
        DEFAULT_MESSAGE = "anonymous or invalid"
        if value is None:
            return DEFAULT_MESSAGE

        try:
            return base64.b64decode(value.decode(), validate=True).decode()
        except binascii.Error:
            return DEFAULT_MESSAGE
        except UnicodeDecodeError:
            return DEFAULT_MESSAGE

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from collections import namedtuple


class ApiVersion(namedtuple("ApiVersion", "version revision")):
    def __str__(self) -> str:
        return f"{self.version}.{self.revision}"


API_V1_VERSION = ApiVersion(version=1, revision=3)
API_V2_VERSION = ApiVersion(version=2, revision=7)
API_V3_VERSION = ApiVersion(version=3, revision=0)
API_V4_VERSION = ApiVersion(version=4, revision=0)
API_VERSION = API_V4_VERSION

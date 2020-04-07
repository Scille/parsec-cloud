# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from collections import namedtuple


class ApiVersion(namedtuple("ApiVersion", "version revision")):
    def __str__(self):
        return f"{self.version}.{self.revision}"


API_V1_VERSION = ApiVersion(version=1, revision=2)
API_V2_VERSION = ApiVersion(version=2, revision=0)
API_VERSION = API_V2_VERSION

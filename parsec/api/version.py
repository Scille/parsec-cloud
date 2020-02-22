# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from collections import namedtuple


class ApiVersion(namedtuple("ApiVersion", "version revision")):
    def __str__(self):
        return f"{self.version}.{self.revision}"


API_VERSION = ApiVersion(version=1, revision=1)

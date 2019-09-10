# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from collections import namedtuple

ApiVersion = namedtuple("ApiVersion", "version revision")
API_VERSION = ApiVersion(version=1, revision=0)

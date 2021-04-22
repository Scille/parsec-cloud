# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import os
import sys
from parsec.cli import cli

os.environ["SENTRY_URL"] = "https://863e60bbef39406896d2b7a5dbd491bb@sentry.io/1212848"
os.environ["PREFERRED_ORG_CREATION_BACKEND_ADDR"] = "parsec://saas.parsec.cloud"
os.makedirs(os.path.expandvars("%APPDATA%\\\\parsec"), exist_ok=True)

cli(args=["core", "gui", *sys.argv[1:]])

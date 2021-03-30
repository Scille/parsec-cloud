# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import sys
import os

from parsec.cli import cli
from click import core

# This prevents click from raising encoding errors at startup
core._verify_python3_env = lambda: None

os.environ["SENTRY_URL"] = "https://863e60bbef39406896d2b7a5dbd491bb@sentry.io/1212848"
os.environ["PREFERRED_ORG_CREATION_BACKEND_ADDR"] = "parsec://saas.parsec.cloud/"

cli(args=["core", "gui", *sys.argv[1:]])

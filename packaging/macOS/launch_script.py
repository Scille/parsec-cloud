# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import parsec.cli
import sys
import os
import locale

os.environ["SENTRY_URL"] = "https://863e60bbef39406896d2b7a5dbd491bb@sentry.io/1212848"
os.environ["PREFERRED_ORG_CREATION_BACKEND_ADDR"] = "parsec://saas.parsec.cloud/"

locale.setlocale(locale.LC_ALL, "")

sys.argv = [sys.argv[0], "core", "gui", *sys.argv[1:]]
parsec.cli.cli.main()

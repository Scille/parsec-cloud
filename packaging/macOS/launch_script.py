# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import parsec.cli
import sys
import os
import locale

os.environ["SENTRY_URL"] = "https://863e60bbef39406896d2b7a5dbd491bb@sentry.io/1212848"
os.environ["PREFERRED_ORG_CREATION_BACKEND_ADDR"] = "parsec://saas.parsec.cloud/"

# Setting QT_MAC_WANTS_LAYER makes Qt work properly on MacOS 11.X
# without having to use another version.
os.environ["QT_MAC_WANTS_LAYER"] = "1"

# The locale values appear to be None by default when launching the app from the
# Finder. If not set, Click will report an encoding error and the app won't launch.
locale.setlocale(locale.LC_ALL, "")

sys.argv = [sys.argv[0], "core", "gui", *sys.argv[1:]]
parsec.cli.cli.main()

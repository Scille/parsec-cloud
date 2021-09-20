# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import os
import sys

# Emulate an access to the python interpreter though a "-m" option
# This way, `sys.executable` can reliably be used to run parsec helper modules
# in subprocesses. This is done before too many dependencies are imported
if len(sys.argv) == 3 and sys.argv[1] == "-m" and sys.argv[2].startswith("parsec._file_dialog"):
    # Import this module explicitely so pyinstaller know it has to ship it
    from parsec._file_dialog import main

    main()
    sys.exit()

from parsec.cli import cli

os.environ["SENTRY_URL"] = "https://863e60bbef39406896d2b7a5dbd491bb@sentry.io/1212848"
os.environ["PREFERRED_ORG_CREATION_BACKEND_ADDR"] = "parsec://saas.parsec.cloud"
os.makedirs(os.path.expandvars("%APPDATA%\\\\parsec"), exist_ok=True)

cli(args=["core", "gui", *sys.argv[1:]])

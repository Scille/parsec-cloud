# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import sys
import os

# Emulate an access to the python interpreter though a "-m" option
# This way, `sys.executable` can reliably be used to run parsec helper modules
# in subprocesses. This is done before too many dependencies are imported
if len(sys.argv) == 3 and sys.argv[1] == "-m" and sys.argv[2].startswith("parsec._file_dialog"):
    # Import this module explicitely so pyinstaller know it has to ship it
    from parsec._file_dialog import main

    main()
    sys.exit()

from parsec.cli import cli
from click import core

core._verify_python3_env = lambda: None
# This prevents click from raising encoding errors at startup.
# It would be better to wrap the original _verify_python3_env so that we could
# log an error when it raises an exception instead of silence it. This would
# allow us to have more insight of when those errors occurs (OS version,
# locales used etc.)
# However the crash occurs before we initialize sentry logging, so it would be a
# mess to catch the exception, save it on the side, then retrieve and log it
# after sentry initialization...
# On top of that, we don't really care about unicode in click here (we are
# expecting to receive no arguments from argv or a single url which is should
# not contain unicode, similarly we output ascii logs to stdout but nobody read
# them anyway when starting the application this way)

os.environ["SENTRY_URL"] = "https://863e60bbef39406896d2b7a5dbd491bb@sentry.io/1212848"
os.environ["PREFERRED_ORG_CREATION_BACKEND_ADDR"] = "parsec://saas.parsec.cloud/"

cli(args=["core", "gui", *sys.argv[1:]])

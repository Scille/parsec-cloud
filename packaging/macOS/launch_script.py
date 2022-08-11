# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import sys
import os
import multiprocessing

# Enable freeze support for supporting the multiprocessing module
# This is useful for running qt dialogs in subprocesses.
# We do this before even importing third parties in order to increase performance.
multiprocessing.freeze_support()


from parsec.cli import cli


os.environ[
    "PARSEC_SENTRY_DSN"
] = "https://863e60bbef39406896d2b7a5dbd491bb@o155936.ingest.sentry.io/1212848"
os.environ["PREFERRED_ORG_CREATION_BACKEND_ADDR"] = "parsec://saas.parsec.cloud/"

cli(args=["core", "gui", *sys.argv[1:]])

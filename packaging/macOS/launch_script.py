# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import parsec.cli
import sys

sys.argv = [sys.argv[0], "core", "gui", *sys.argv[1:]]
parsec.cli.cli.main()

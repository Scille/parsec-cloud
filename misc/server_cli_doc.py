# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import re
from pathlib import Path

PROJECT_DIR = Path(__file__).parent.parent
CLI_RUN_SRC_FILE = PROJECT_DIR / "server/parsec/cli/run.py"
HOSTING_DOC_FILE = PROJECT_DIR / "docs/HOSTING.md"


if __name__ == "__main__":
    cli_run_src = CLI_RUN_SRC_FILE.read_text(encoding="utf8")
    all_env_vars = re.findall(r"\"(PARSEC_\w+)\"", cli_run_src, re.MULTILINE)

    hosting_src = HOSTING_DOC_FILE.read_text(encoding="utf8")
    missing_docs = [env_var for env_var in all_env_vars if env_var not in hosting_src]

    if missing_docs:
        raise SystemExit(
            f"`docs/HOSTING.md` is missing documentation for environ variable(s): {', '.join(missing_docs)}"
        )

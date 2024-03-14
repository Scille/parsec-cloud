# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import os
import sys
from typing import Any, Sequence

from parsec.cli import cli

# Add support for PARSEC_CMD_ARGS env var

vanilla_cli_main = cli.main


def patched_cli_main(args: Sequence[str] | None = None, **kwargs: Any) -> Any:
    args_list = list(args) if args is not None else sys.argv[1:]

    raw_extra_args = os.environ.get("PARSEC_CMD_ARGS", "")
    args_list += [os.path.expandvars(x) for x in raw_extra_args.split()]

    return vanilla_cli_main(args=args_list[:], **kwargs)


# [pyright] here we want to use the patched main instead of the one provided by `BaseCommand`
# Note: We could use monkeypatch instead :thinking:
cli.main = patched_cli_main  # pyright: ignore[reportAttributeAccessIssue]


if __name__ == "__main__":
    cli()

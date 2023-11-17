# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

# cspell:words oneline

"""
PRs must contain a newsfragment that reference an opened issue
"""

import argparse
import json
from concurrent.futures import ThreadPoolExecutor
from itertools import repeat
from pathlib import Path
from subprocess import run
from typing import Optional
from urllib.request import HTTPError, Request, urlopen

# This list should be keep up to date with `misc/releaser.py::FRAGMENT_TYPES.keys()`
VALID_TYPE = ["feature", "bugfix", "doc", "removal", "api", "misc", "empty"]


def check_newsfragment(fragment: Path, base: str) -> Optional[bool]:
    fragment_name = fragment.name

    # If file never existed in `base`, consider as a new newsfragment
    # Cannot just git diff against `base` branch here given newsfragments
    # removed in master will be considered as new items in our branch
    # --exit-code makes the command exit with 0 if there are changes
    cmd_args = ["git", "log", base, "--exit-code", "--oneline", "--", str(fragment)]
    print(f"[{fragment_name}] Checking news fragment on base {base} with `{' '.join(cmd_args)}`")
    ret = run(cmd_args, capture_output=True)

    if ret.returncode == 0:
        print(f"[{fragment_name}] Found new newsfragment")

        id, type, *_ = fragment_name.split(".")

        if type not in VALID_TYPE:
            print(
                f"[{fragment_name}] Not a valid fragment type `{type}` (expected one of {VALID_TYPE})"
            )
            return False

        # For more information on github api for issues:
        # see https://docs.github.com/en/rest/issues/issues#get-an-issue
        url = f"https://api.github.com/repos/Scille/parsec-cloud/issues/{id}"
        req = Request(
            method="GET",
            url=url,
            headers={"Accept": "application/vnd.github.v3+json"},
        )
        try:
            response = urlopen(req)
        except HTTPError as exc:
            print(
                f"[{fragment_name}] fragment ID doesn't correspond to an issue! (On <{url}> HTTP error {exc.code} {exc.reason})"
            )
            return False
        else:
            # Sanity check: try to deserialize the response.
            data = json.loads(response.read())
            print(f"[{fragment_name}] issue#{id} => {data}")
            return True
    else:
        return None


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--base", help="The base branch to compare to", default="origin/master", type=str
    )
    args = parser.parse_args()

    with ThreadPoolExecutor() as pool:
        ret = list(
            filter(
                lambda value: value is not None,
                pool.map(
                    check_newsfragment,
                    Path("newsfragments").glob("*.rst"),
                    repeat("origin/" + args.base),
                ),
            )
        )

    if len(ret) == 0:
        raise SystemExit("No new newsfragment found")

    if True not in ret:
        raise SystemExit("No valid newsfragments found")

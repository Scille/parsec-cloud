# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

"""
PRs must contain a newsfragment that reference an opened issue
"""

import argparse
import json
import re
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from subprocess import run
from typing import Optional
from urllib.request import HTTPError, Request, urlopen

# If file never existed in master, consider as a new newsfragment
# Cannot just git diff against master branch here given newsfragments
# removed in master will be considered as new items in our branch
# --exit-code makes the command exit with 0 if there are changes
BASE_CMD = "git log origin/master --exit-code --".split()

# Ignore master given we compare against it !
# Also ignore release branch (e.g. `2.11`) that don't have to create newsfragments
IGNORED_BRANCHES_PATTERN = r"(master|[0-9]+\.[0-9]+)"

# This list should be keep up to date with `misc/releaser.py::FRAGMENT_TYPES.keys()`
VALID_TYPE = ["feature", "bugfix", "doc", "removal", "api", "misc", "empty"]


def check_newsfragment(fragment: Path) -> Optional[bool]:
    fragment_name = fragment.name
    cmd_args = [*BASE_CMD, str(fragment)]
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
                f"[{fragment_name}] fragment ID doesn't correspond to an issue ! (On <{url}> HTTP error {exc.code} {exc.reason})"
            )
        else:
            data = json.loads(response.read())
            print(f"[{fragment_name}] issue#{id} => {data}")
            if "pull_request" in data:
                print(
                    f"[{fragment_name}] fragment ID correspond to a pull request instead of an issue !"
                )
            else:
                if data["state"] == "open":
                    return True
                else:
                    print(f"[{fragment_name}] fragment ID correspond to a closed issue !")
        return False
    else:
        return None


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("branch_name", help="current branch name", type=str)
    args = parser.parse_args()

    if re.match(IGNORED_BRANCHES_PATTERN, args.branch_name):
        print("Release branch detected, ignoring newsfragment checks")
        raise SystemExit(0)

    with ThreadPoolExecutor() as pool:
        ret = list(
            filter(
                lambda value: value is not None,
                pool.map(check_newsfragment, Path("newsfragments").glob("*.rst")),
            )
        )

    if len(ret) == 0:
        raise SystemExit("No new newsfragment found")

    if True not in ret:
        raise SystemExit("No valid newsfragments found")

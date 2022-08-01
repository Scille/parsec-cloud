# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

"""
PRs must contain a newsfragment that reference an opened issue
"""

import re
import json
import argparse
from pathlib import Path
from subprocess import run
from urllib.request import urlopen, Request, HTTPError
from concurrent.futures import ThreadPoolExecutor


# If file never existed in master, consider as a new newsfragment
# Cannot just git diff against master branch here given newsfragments
# removed in master will be considered as new items in our branch
# --exit-code makes the command exit with 0 if there are changes
BASE_CMD = "git log origin/master --exit-code --".split()

# Ignore master given we compare against it !
# Also ignore release branch (e.g. `2.11`) that don't have to create newsfragments
IGNORED_BRANCHES_PATTERN = r"(master|[0-9]+\.[0-9]+)"


def check_newsfragment(fragment):
    cmd_args = [*BASE_CMD, fragment]
    ret = run(cmd_args, capture_output=True)
    if ret.returncode == 0:
        print(f"Found new newsfragment {fragment.name}")

        id, *_ = fragment.name.split(".")
        req = Request(
            method="GET",
            url=f"https://api.github.com/repos/Scille/parsec-cloud/issues/{id}",
            headers={"Accept": "application/vnd.github.v3+json"},
        )
        try:
            ret = urlopen(req)
        except HTTPError:
            raise SystemExit("New newsfragment ID doesn't correspond to an issue !")
        data = json.loads(ret.read())
        if "pull_request" in data:
            raise SystemExit(
                "New newsfragement ID correspond to a pull request instead of an issue !"
            )
        if data["state"] != "open":
            raise SystemExit("New newsfragement ID correspond to a closed issue")
        return True
    return False


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("branch_name", help="current branch name", type=str)
    args = parser.parse_args()

    if re.match(IGNORED_BRANCHES_PATTERN, args.branch_name):
        print("Release branch detected, ignoring newsfragment checks")
        raise SystemExit(0)

    with ThreadPoolExecutor() as pool:
        ret = pool.map(check_newsfragment, Path("newsfragments").glob("*.rst"))

    if True not in ret:
        raise SystemExit("No new newsfragment found")

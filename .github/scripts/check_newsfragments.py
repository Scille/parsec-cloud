# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
This scripts checks that all <issue>.<type>.rst files in the newsfragments
directory are valid:

- <type> corresponds to a valid type
- <issue> corresponds to an existing issue or pull request
"""

import argparse
import json
import os
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from urllib.request import HTTPError, Request, urlopen

# This list should be keep up to date with `misc/releaser.py::FRAGMENT_TYPES.keys()`
VALID_TYPES = ["feature", "bugfix", "doc", "removal", "api", "misc", "empty"]
HEADERS: dict[str, str] = {
    "Accept": "application/vnd.github.v3+json",
    "X-GitHub-Api-Version": "2022-11-28",
    **(
        {"Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}"}
        if "GITHUB_TOKEN" in os.environ
        else {}
    ),
}


def check_newsfragment(fragment: Path) -> bool | None:
    fragment_name = fragment.name

    # Check news fragment TYPE
    issue, type, *_ = fragment_name.split(".")
    if type not in VALID_TYPES:
        print(f"[{fragment_name}] Type `{type}` is not valid (expected one of {VALID_TYPES})")
        return False

    # Check news fragment ISSUE number
    #
    # NOTE: GitHub's REST API considers every pull request an issue, but not
    # every issue is a pull request. For this reason, "Issues" endpoints may
    # return both issues and pull requests in the response.
    # See https://docs.github.com/en/rest/issues/issues#get-an-issue
    url = f"https://api.github.com/repos/Scille/parsec-cloud/issues/{issue}"
    req = Request(
        method="GET",
        url=url,
        headers=HEADERS,
    )
    try:
        response = urlopen(req)
    except HTTPError as exc:
        print(
            f"[{fragment_name}] Issue `#{issue}` was not found (on <{url}> HTTP error {exc.code} {exc.reason})"
        )
        return False
    else:
        # Sanity check: try to deserialize the response
        data = json.loads(response.read())
        print(f"[{fragment_name}] Issue #{issue} => {data['title']}")
        return True


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    args = parser.parse_args()

    print("Checking news fragments...")

    with ThreadPoolExecutor() as pool:
        ret = list(
            filter(
                lambda value: value is not None,
                pool.map(
                    check_newsfragment,
                    Path("newsfragments").glob("*.*.rst"),  # <issue>.<type>.rst
                ),
            )
        )

    if True not in ret:
        raise SystemExit("No valid news fragments found")

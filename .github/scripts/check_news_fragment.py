# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

# PRs must contain a newsfragment that reference an opened issue
import re
import sys
import json
from pathlib import Path
from subprocess import run
from urllib.request import urlopen, Request, HTTPError
from concurrent.futures import ThreadPoolExecutor
import logging

logging.basicConfig(level=logging.INFO)
ROOT_LOGGER = logging.getLogger()

# If file never existed in master, consider as a new newsfragment
# Cannot just git diff against master branch here given newsfragments
# removed in master will be considered as new items in our branch
# --exit-code makes the command exit with 0 if there are changes
BASE_CMD = "git log origin/master --exit-code --".split()


def parse_args():
    import argparse

    parser = argparse.ArgumentParser("check_news_fragment")
    parser.add_argument("branch_name", help="current branch name", type=str)
    return parser.parse_args()


args = parse_args()
BRANCH_NAME = args.branch_name

# Regex pattern to match for release branch.
# Release branch don't have to create a news fragment
RELEASE_BRANCH_NAME_PATTERN = r"[0-9]+.[0-9]+"

ROOT_LOGGER.info(f"current branch {BRANCH_NAME}")
if re.match(RELEASE_BRANCH_NAME_PATTERN, BRANCH_NAME) is not None:
    print("Release branch detected, ignoring newsfragment checks")
    sys.exit(0)


def check_newsfragment(fragment):
    log = ROOT_LOGGER.getChild(str(fragment.name))

    cmd_args = [*BASE_CMD, fragment]
    ret = run(cmd_args, capture_output=True)
    log.info("checking fragment {}".format(fragment.name))

    log.info("stdout: {}".format(ret.stdout))
    log.info("stderr: {}".format(ret.stderr))
    if ret.returncode == 0:
        log.info(f"Found new newsfragment {fragment.name}")

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


with ThreadPoolExecutor() as pool:
    ret = pool.map(check_newsfragment, Path("newsfragments").glob("*.rst"))

if True not in ret:
    raise SystemExit("No new newsfragment found")

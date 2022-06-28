# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

# PRs must contain a newsfragment that reference an opened issue
from pathlib import Path
from subprocess import run
from urllib.request import urlopen, Request, HTTPError
from concurrent.futures import ThreadPoolExecutor
import json
import logging

logging.basicConfig(level=logging.INFO)
root_logger = logging.getLogger()

# If file never existed in master, consider as a new newsfragment
# Cannot just git diff against master branch here given newsfragments
# removed in master will be considered as new items in our branch
# --exit-code makes the command exit with 0 if there are changes
basecmd = "git log origin/master --exit-code --".split()


def check_newsfragment(fragment):
    log = root_logger.getChild(str(fragment.name))

    cmd_args = [*basecmd, fragment]
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

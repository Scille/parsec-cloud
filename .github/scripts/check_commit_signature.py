# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

# Commit signature is not mandatory for the repo (yet ?), however we
# want to make sure the signed commits are actually valid.
# Indeed, it seems the signature process breaks from time to time,
# see for instance commit 1e1535b010050c025d85b685e96e08db5e9cd7bb
# that is considered invalid, but has been generated 3mn after the valid
# 2df2c661a4774097615b7f66078207b15a56316b on the same computer with
# the same key...)
import json
import re
from urllib.request import urlopen

match = re.match(r"refs/pull/([0-9]+)/merge", "$(Build.SourceBranch)")

if match:
    pr_id = match.group(1)
    r = urlopen(f"https://api.github.com/repos/Scille/parsec-cloud/pulls/{pr_id}/commits")
    bad_commits = [
        c["sha"] for c in json.load(r) if c["commit"]["verification"]["reason"] == "invalid"
    ]

    if bad_commits:
        raise SystemExit(f"""Invalid signatures in commits: {', '.join(bad_commits)}""")

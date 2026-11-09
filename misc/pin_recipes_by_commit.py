#!/usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# Check/fix that external recipes are pinned by commit hash (instead of by
# version tag, which an attacker could overwrite) and carry a `# <tag>`
# comment so the pinned version stays readable.
#
# Targets:
# - `pre-commit`: the `rev:` of the remote repos in `.pre-commit-config.yaml`
# - `github-actions`: the `uses:` of the Github actions in `.github/{workflows,actions}/*.yml`

from __future__ import annotations

import argparse
import re
import subprocess
import tempfile
from collections.abc import Callable
from functools import cache
from pathlib import Path

PROJECT_DIR = Path(__file__).parent.parent.resolve()
PC_PRE_COMMIT_CONFIG = PROJECT_DIR / ".pre-commit-config.yaml"
GA_DIRS = (PROJECT_DIR / ".github" / "workflows", PROJECT_DIR / ".github" / "actions")

# A commit hash, i.e. 40 hexadecimal characters
COMMIT_HASH_RE = re.compile(r"[0-9a-f]{40}")
# pre-commit: `- repo: <url>` followed by `    rev: <ref>`
PC_REPO_RE = re.compile(r"^\s*-\s*repo:\s*(?P<url>\S+)\s*$")
PC_REV_RE = re.compile(r"^(?P<prefix>\s*rev:\s)(?P<ref>.+?)\s*$")
# Github actions: `uses: <action>@<ref>`, with an optional leading `- ` (list item)
GA_USES_RE = re.compile(r"^(?P<prefix>[\s-]*uses:\s)(?P<spec>\S+)(?P<rest>.*)$")


def split_ref_comment(text: str) -> tuple[str, str | None]:
    """Split `<ref> [# <comment>]` into its two parts."""
    ref, _, comment = text.partition("#")
    return ref.strip(), comment.strip() or None


@cache
def git_ls_remote(url: str, *patterns: str) -> tuple[tuple[str, str], ...]:
    """Query a remote repository, returning a list of (hash, refname)."""
    try:
        output = subprocess.check_output(
            # `ls-remote` never uses the local repository, so run it from a neutral
            # directory to not be bothered by the local git repository state
            ["git", "ls-remote", url, *patterns],
            text=True,
            stderr=subprocess.PIPE,
            cwd=tempfile.gettempdir(),
        )
    except subprocess.CalledProcessError as exc:
        raise RuntimeError(f"cannot query {url}: {exc.stderr.strip()}") from exc
    return tuple(
        (commit, refname)
        for line in output.splitlines()
        for commit, refname in (line.split(maxsplit=1),)
    )


def resolve_commit(url: str, tag: str) -> str:
    """Return the commit hash pointed by the given tag."""
    refs = git_ls_remote(url, f"refs/tags/{tag}", f"refs/tags/{tag}^{{}}")
    # Annotated tags come with an extra `^{}` entry pointing to the actual commit
    for commit, refname in refs:
        if refname.endswith("^{}"):
            return commit
    if not refs:
        raise RuntimeError(f"cannot find tag `{tag}` in {url}")
    return refs[0][0]


def find_tag(url: str, commit: str) -> str:
    """Return a tag pointing to the given commit."""
    tag = None
    for c, refname in git_ls_remote(url, "refs/tags/*"):
        if c == commit:
            # Keep the last match: ref names are sorted, so it is the most precise tag
            tag = refname.removeprefix("refs/tags/").removesuffix("^{}")
    if tag is None:
        raise RuntimeError(f"cannot find a tag for commit {commit} in {url}")
    return tag


def pin_recipe(url: str, ref: str, comment: str | None) -> tuple[str, str]:
    """Return the `<commit hash>` and `<tag>` a recipe should be pinned with."""
    if COMMIT_HASH_RE.fullmatch(ref):
        if comment:
            return ref, comment  # Already properly pinned
        # Pinned by commit, but missing the expected `<tag>` comment
        return ref, find_tag(url, ref)
    # Pinned by tag: resolve the commit hash it points to
    return resolve_commit(url, ref), ref


def process_recipe(
    path: Path,
    lineno: int,
    line: str,
    url: str,
    ref: str,
    comment: str | None,
    render: Callable[[str, str], str],
    check: bool,
) -> tuple[str | None, bool]:
    """Check/fix a single recipe line, returning the new line (if fixed) and its status."""
    location = f"{path.relative_to(PROJECT_DIR)}:{lineno}"
    try:
        new_ref, new_comment = pin_recipe(url, ref, comment)
    except RuntimeError as exc:
        print(f"ERROR {location}: {exc}")
        return None, False
    new_line = render(new_ref, new_comment)
    if new_line == line:
        return None, True  # Already properly pinned
    if check:
        print(f"ERROR {location}: is `{line.strip()}`, should be `{new_line.strip()}`")
        return None, False
    print(f"FIXED {location}: `{line.strip()}` -> `{new_line.strip()}`")
    return new_line, True


def process_pre_commit(check: bool) -> bool:
    lines = PC_PRE_COMMIT_CONFIG.read_text().splitlines()
    url = None
    ok = True
    for lineno, line in enumerate(lines, 1):
        if match := PC_REPO_RE.match(line):
            # Only remote repos have a `rev:` field, local ones are not concerned
            url = match.group("url") if match.group("url").startswith("http") else None
        elif url and (match := PC_REV_RE.match(line)):
            repo_url, url = url, None
            prefix = match.group("prefix")
            ref, comment = split_ref_comment(match.group("ref"))
            new_line, line_ok = process_recipe(
                PC_PRE_COMMIT_CONFIG,
                lineno,
                line,
                repo_url,
                ref,
                comment,
                lambda ref, comment: f"{prefix}{ref} # {comment}",
                check,
            )
            ok &= line_ok
            if new_line is not None:
                lines[lineno - 1] = new_line
    if not check:
        PC_PRE_COMMIT_CONFIG.write_text("\n".join(lines) + "\n")
    return ok


def process_ci(check: bool) -> bool:
    ok = True
    for path in sorted(p for dir in GA_DIRS for p in dir.rglob("*.yml")):
        lines = path.read_text().splitlines()
        changed = False
        for lineno, line in enumerate(lines, 1):
            match = GA_USES_RE.match(line)
            if not match:
                continue
            spec = match.group("spec")
            # Local actions and docker images are not pinned, skip them
            action, sep, ref = spec.rpartition("@")
            if not sep or spec.startswith(("./", "docker://")):
                continue
            url = f"https://github.com/{'/'.join(action.split('/')[:2])}"
            prefix = match.group("prefix")
            ref, comment = split_ref_comment(ref + match.group("rest"))
            new_line, line_ok = process_recipe(
                path,
                lineno,
                line,
                url,
                ref,
                comment,
                lambda ref, comment: f"{prefix}{action}@{ref} # {comment}",
                check,
            )
            ok &= line_ok
            if new_line is not None:
                lines[lineno - 1] = new_line
                changed = True
        if changed and not check:
            path.write_text("\n".join(lines) + "\n")
    return ok


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Check/fix that recipes are pinned by commit hash "
        "(instead of by version tag, which could be overwritten by an attacker)."
    )
    parser.add_argument(
        "target",
        choices=("pre-commit", "pc", "github-actions", "ga"),
        help="Which recipes to check/fix",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Only report problems, don't fix them",
    )
    args = parser.parse_args()

    match args.target:
        case "pre-commit" | "pc":
            ok = process_pre_commit(args.check)
        case "github-actions" | "ga":
            ok = process_ci(args.check)
        case unknown:
            assert False, unknown

    if not ok:
        raise SystemExit(1)

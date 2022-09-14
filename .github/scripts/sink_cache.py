# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

# CLI to Delete Github Action Cache.
#
# This script remove Cache entries a specified Github repository.
# This allow future workflow to run on clear cache data (since they'll have to recreate them)
import argparse
from dataclasses import dataclass
from datetime import datetime
import json
import logging
import subprocess
from typing import Any, Dict, Optional
from logging import basicConfig, getLogger

basicConfig(level=logging.INFO)
logger = getLogger()


def naturalize(bytes_size: int) -> str:
    """
    Transform a size in bytes into an human readable format
    >>> naturalize(14050948082)
    13.1 GiB
    >>> naturalize(142964341)
    136.3 MiB
    """
    normalized_size = bytes_size / 2**20
    if normalized_size < 1000.0:
        return f"{normalized_size:.1f} MiB"
    else:
        normalized_size = bytes_size / 2**30
        return f"{normalized_size:.1f} GiB"


def gh_api(url: str, method: Optional[str] = "GET"):
    extra_args = [f"--header='Accept: application/json'"]
    if method is not None:
        extra_args.append(f"--method={method}")
    cmd_args = ["gh", "api", *extra_args, url]
    logger.debug(f"Will execute cmd `{' '.join(cmd_args)}`")
    ret = subprocess.check_output(cmd_args)
    return json.loads(ret)


@dataclass
class CacheEntry:
    id: int
    key: str
    ref: str
    version: str
    last_accessed_at: datetime
    created_at: datetime
    size_in_bytes: int

    def from_dict(dict: Dict[str, Any]) -> "CacheEntries":
        # Datetime format for `2019-01-24T22:45:36.000Z``

        def get_iso_datetime(raw: str) -> datetime:
            """
            Transform the raw date-time string onto a datetime object.
            Currently the api provide a date-time with the following format:

            - YYYY-MM-DDTHH:MM:SS.fffffffffZ
            """
            from re import subn

            log = logger.getChild(get_iso_datetime.__name__)
            # `fromisoformat` don't support the `Z` timezone to specify the UTC timezone, so we replace it.
            raw_without_z, _ = subn(r"Z$", "+00:00", raw, count=1)
            # `fromisoformat` don't support nanosecond precision, so we need to remove the nanosecond bits.
            raw_without_nano, _ = subn(r"(?:(.\d{3,3})(?:\d+))", r"\1", raw_without_z, count=1)

            log.debug(f"raw={raw}, without_z={raw_without_z}, without_nano={raw_without_nano}")

            dt = datetime.fromisoformat(raw_without_nano)
            return dt

        last_accessed_at = get_iso_datetime(dict["last_accessed_at"])
        create_at = get_iso_datetime(dict["created_at"])

        return CacheEntry(
            id=int(dict["id"]),
            key=dict["key"],
            ref=dict["ref"],
            version=dict["version"],
            last_accessed_at=last_accessed_at,
            created_at=create_at,
            size_in_bytes=int(dict["size_in_bytes"]),
        )


@dataclass
class CacheEntries:
    total_count: int
    actions_caches: Dict[int, CacheEntry]

    def __repr__(self) -> str:
        return f"{type(self).__name__}({self.total_count}, {self.actions_caches:r})"

    @staticmethod
    def empty() -> "CacheEntries":
        return CacheEntries(-1, {})

    def need_to_download_more(self) -> bool:
        return self.total_count - 1 != len(self.actions_caches)

    def from_dict(raw_dict: Dict[str, Any]) -> "CacheEntries":
        return CacheEntries(0, {}).update_from_dict(raw_dict, unique_cache=True)

    def update_from_dict(
        self, raw_dict: Dict[str, Any], unique_cache: bool = False
    ) -> "CacheEntries":
        self.total_count = int(raw_dict["total_count"])

        for action_cache in raw_dict["actions_caches"]:
            entry = CacheEntry.from_dict(action_cache)
            if unique_cache and entry.id in self.actions_caches:
                raise RuntimeError(f"Duplicate Cache ID {entry.id}")
            self.actions_caches[entry.id] = entry

        return self

    def __add__(self, x: "CacheEntries") -> "CacheEntries":
        new_entries = self.empty()

        new_entries.total_count = max(self.total_count, x.total_count)
        new_entries.actions_caches = self.actions_caches | x.actions_caches
        return new_entries


def get_cache_entries(owner: str, repo: str, ref: Optional[str]) -> CacheEntries:
    """
    Documentation: https://docs.github.com/en/rest/actions/cache#list-github-actions-caches-for-a-repository
    """
    log = logger.getChild(get_cache_entries.__name__)
    page_counter = 1
    cache_entries = CacheEntries.empty()
    base_url = f"/repos/{owner}/{repo}/actions/caches?"
    if ref is not None:
        base_url += f"ref={ref}&"
    while True:
        raw_cache_entries = gh_api(f"{base_url}page={page_counter}")
        entries = CacheEntries.from_dict(raw_cache_entries)
        log.info(f"Found {len(entries.actions_caches)} entries at page {page_counter}")
        if len(entries.actions_caches) == 0:
            break
        # Merge with main CacheEntries
        cache_entries = cache_entries + entries
        # Increase page counter
        page_counter += 1

    return cache_entries


def remove_cache_entries(
    entries: CacheEntries, owner: str, repo: str, dry: bool, ref: Optional[str]
):
    log = logger.getChild(remove_cache_entries.__name__)
    base_url = f"/repos/{owner}/{repo}/actions/caches?"
    if ref is not None:
        base_url += f"ref={ref}&"
    for (entry_id, entry) in entries.actions_caches.items():
        log.info(
            f"Will remove entry {entry.key}"
            + (f" at {ref}" if ref is not None else "")
            + " ({} or {} bytes)".format(naturalize(entry.size_in_bytes), entry.size_in_bytes)
        )
        if not dry:
            try:
                return_val = gh_api(f"{base_url}key={entry.key}", method="DELETE")
            except subprocess.CalledProcessError as cpe:
                log.error(f"Error while removing cache {entry.key}: {cpe}")
            else:
                log.debug(f"res={return_val}")


def remove_caches_from_repository(owner: str, repo: str, dry: bool, ref: Optional[str]):
    logger.info(f"Will remove caches from {owner}/{repo}" + (f"/{ref}" if ref is not None else ""))
    entries = get_cache_entries(owner, repo, ref)
    if entries.actions_caches:
        remove_cache_entries(entries, owner, repo, dry, ref)
    else:
        logger.info("No cache to delete !")
    total_removed_size = sum(entry.size_in_bytes for entry in entries.actions_caches.values())

    logger.info(
        "Total cache size {} ({} bytes)".format(naturalize(total_removed_size), total_removed_size)
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="sink-cache",
        description="\n".join(
            [
                "Remote cache entries from a specified Github repository.",
                "Will use Github CLI under the hood.",
                "",
                "The value `owner` & `repo` are located on the URL of a github repository (`https://github.com/{owner}/{repo}`)",
            ]
        ),
    )
    parser.add_argument("owner", help="The owner of the repository")
    parser.add_argument("repo", help="The Github repository name")
    parser.add_argument(
        "--ref",
        required=False,
        help="Remove the cache from a specific Github reference, can be `refs/heads/<branch name>` or `refs/pull/<number>/merge`",
    )
    parser.add_argument("--dry", action="store_true")
    args = parser.parse_args()

    remove_caches_from_repository(owner=args.owner, repo=args.repo, dry=args.dry, ref=args.ref)

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""
For a given Parsec version, this script outputs the `<channel>` value intended to be used in the `--release=<channel>` option for `snapcraft upload`.

- A channel is formed with <track>/<risk-level>/<branch>
- Parsec currently has 4 tracks: latest, v2, v3, nigthly
- The <branch> component is not used

E.g.

$ python misc/snapcraft_releases.py 2.0.0
v2/stable
$ python misc/snapcraft_releases.py 3.0.0-b.0
v3/beta,latest/beta
$ python misc/snapcraft_releases.py 3.0.0-b.0 --nightly
v3/edge,latest/edge,nightly/stable
"""

import logging
from argparse import ArgumentParser
from enum import StrEnum

from releaser import (  # type: ignore (pyright struggles with this when run from server folder)
    Version,
)

log = logging.getLogger(__name__)


class Track(StrEnum):
    Latest = "latest"
    V2 = "v2"
    V3 = "v3"
    Nightly = "nightly"


class RiskLevel(StrEnum):
    Stable = "stable"
    Candidate = "candidate"
    Beta = "beta"
    Edge = "edge"


ALL_RISK_LEVEL = (RiskLevel.Stable, RiskLevel.Candidate, RiskLevel.Beta, RiskLevel.Edge)


def get_tracks_for_version(version: Version) -> tuple[Track, ...]:
    match version.major:
        case 2:
            return (Track.V2,)
        case 3:
            return (Track.V3, Track.Latest)
        case _:
            raise ValueError(f"Unsupported major version: {version.major}")


assert get_tracks_for_version(Version.parse("2.0.0")) == (Track.V2,)
assert get_tracks_for_version(Version.parse("3.0.0")) == (Track.V3, Track.Latest)


def get_risk_level_for_version(version: Version) -> RiskLevel:
    if version.prerelease:
        PRERELEASE_TO_RISK = {
            "a": RiskLevel.Edge,
            "b": RiskLevel.Beta,
            "rc": RiskLevel.Candidate,
        }
        for pre, chan in PRERELEASE_TO_RISK.items():
            if version.prerelease.startswith(pre):
                return chan
        raise ValueError(f"Unsupported prerelease: {version.prerelease}")

    if version.dev is not None:
        return RiskLevel.Edge

    return RiskLevel.Stable


assert get_risk_level_for_version(Version.parse("3.0.0")) == RiskLevel.Stable
assert get_risk_level_for_version(Version.parse("3.0.0-a.0")) == RiskLevel.Edge
assert get_risk_level_for_version(Version.parse("3.0.0-b.0")) == RiskLevel.Beta
assert get_risk_level_for_version(Version.parse("3.0.0-rc.0")) == RiskLevel.Candidate
assert get_risk_level_for_version(Version.parse("3.0.0-dev.0")) == RiskLevel.Edge

if __name__ == "__main__":
    parser = ArgumentParser(
        "snapcraft_releases",
        description="""
    Output as many snapcraft's `<channel>` values based on the given Parsec version.
    """,
        epilog="See https://snapcraft.io/docs/channels",
    )

    parser.add_argument(
        "version",
        help="The Parsec version to determine the channels for.",
        type=Version.parse,
    )
    parser.add_argument("--nightly", help="Use nightly track", action="store_true")

    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO)

    version: Version = args.version
    log.info(f"Version: {version!r}")

    tracks_for_version = get_tracks_for_version(version)
    log.info(f"Tracks for version {version}: {', '.join(tracks_for_version)}")

    release_channels: list[str] = []
    if args.nightly:
        # Update channels "<tracks>/edge"
        release_channels.extend(map(lambda track: f"{track}/{RiskLevel.Edge}", tracks_for_version))
        # Update channels "nightly/stable"
        release_channels.append(f"{Track.Nightly}/{RiskLevel.Stable}")
    else:
        risk = get_risk_level_for_version(version)
        release_channels.extend(map(lambda track: f"{track}/{risk}", tracks_for_version))

    log.debug(" ".join(release_channels))
    print(*release_channels, sep=",")

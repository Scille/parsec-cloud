# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass

from parsec._parsec import DateTime
from parsec.logging import get_logger
from parsec.types import BadOutcome

__all__ = ("timestamps_in_the_ballpark", "TimestampOutOfBallpark")


logger = get_logger()


@dataclass(slots=True)
class RequireGreaterTimestamp(BadOutcome):
    strictly_greater_than: DateTime


# Those values are used by the server to determine whether the client
# operates in an acceptable time window. A client early offset of
# 300 seconds means that a timestamp provided by the client cannot be
# higher than 300 seconds after the server current time. A client late
# offset of 320 seconds means that a timestamp provided by the client
# cannot be lower than 320 seconds before the server current time.
#
# Those values used to be higher (30 minutes), but an argument for
# decreasing this value is that client clocks are usually either
# fully synchronized (with NTP) or fully desynchronized (with an
# incorrect configuration). It's possible for a client clock to
# slowly drift over time if it lost access to an NTP but drifting
# an entire minute would take weeks or months.
#
# However, we observed that such a case could occur, where a client's
# clock was 72 seconds late while the offset was 50/70 seconds. The
# cause being setting the clock manually, which can easily cause drift
# of 1 or 2 minutes. Since it's impossible to predict such behavior,
# the offset has been changed from 50/70 to 300/320 seconds, which
# seems good enough to prevent such cases and is Windows' standard
# tolerance for clock drifting.
#
# Note that those values also have to take into account the fact
# that the timestamps being compared are not produced at the same
# moment. Typically:
# - The client generates a timestamp
# - The client generates a request including this timestamp
# - The client sends this request to the server over the network
# - The server receives and processes the request
# - The server compares the timestamp to its current time
# The worst case scenario would be a slow client machine, a large
# request, a slow network connection and a busy server. Even
# in this scenario, a 10 seconds time difference is hardly
# imaginable on a properly functioning system.
#
# This is an argument for making this comparison asymmetrical: with no
# clock drift between client and server, communication latency makes data
# arriving to the server always in the past. Hence we should be more
# forgiving of data in the past than in the future !
#
# A more radical check would be to not accept more than 10 seconds
# delay and 10 seconds shifting, yielding a 10/20 seconds window
# (10 seconds in advance or 20 seconds late). This would effectively
# reduce the previous 50/70 seconds time window by a factor of 4.
# This however seems unrealistic as the 50/70 window turned out
# too narrow.

BALLPARK_CLIENT_EARLY_OFFSET = 300  # seconds
BALLPARK_CLIENT_LATE_OFFSET = 320  # seconds
BALLPARK_ALWAYS_OK = False  # Useful for disabling ballpark checks in the tests


@dataclass(slots=True)
class TimestampOutOfBallpark(BadOutcome):
    client_timestamp: DateTime
    server_timestamp: DateTime
    ballpark_client_early_offset: int = BALLPARK_CLIENT_EARLY_OFFSET
    ballpark_client_late_offset: int = BALLPARK_CLIENT_LATE_OFFSET


def timestamps_in_the_ballpark(
    client: DateTime,
    server: DateTime,
    ballpark_client_early_offset: int = BALLPARK_CLIENT_EARLY_OFFSET,
    ballpark_client_late_offset: int = BALLPARK_CLIENT_LATE_OFFSET,
) -> None | TimestampOutOfBallpark:
    """
    Useful to compare signed message timestamp with the one stored by the server.

    Returns `None` if the timestamps are in the ballpark.
    """
    seconds = server - client
    if -ballpark_client_early_offset < seconds < ballpark_client_late_offset:
        return None
    else:
        if BALLPARK_ALWAYS_OK:
            return None
        return TimestampOutOfBallpark(
            client_timestamp=client,
            server_timestamp=server,
            ballpark_client_early_offset=ballpark_client_early_offset,
            ballpark_client_late_offset=ballpark_client_late_offset,
        )

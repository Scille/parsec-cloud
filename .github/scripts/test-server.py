# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import time
from urllib.request import Request, urlopen


def can_connect_to_testbed(r: Request) -> bool:
    MAX_RETRIES = 10
    DURATION_BETWEEN_RETRIES = 1

    for i in range(MAX_RETRIES):
        try:
            urlopen(r)
        except Exception as exc:
            print(f"Try {i + 1}/{MAX_RETRIES}: {exc}")
            time.sleep(DURATION_BETWEEN_RETRIES)
            continue
        else:
            return True
    return False


if __name__ == "__main__":
    r = Request("http://127.0.0.1:6777/", method="GET")
    print(f"Will connect to testbed server on: {r.get_full_url()}")

    if can_connect_to_testbed(r):
        print("Success")
    else:
        raise SystemExit("Cannot connect to Parsec server :(")

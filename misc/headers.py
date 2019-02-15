#!/usr/bin/env python
# -*- coding: utf-8 -*-

import sys
import pathlib
import re
import argparse

HEADER = "# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS\n\n"
HEADER_RE = re.compile(
    r"^# Parsec Cloud \(https://parsec.cloud\) Copyright \(c\) AGPLv3 2019 Scille SAS$"
)


def get_files():
    SCANS = ["parsec", "tests"]

    for scan in SCANS:
        path = pathlib.Path(scan)
        if path.is_dir():
            for f in pathlib.Path(path).glob("**/*.py"):
                yield f
        elif path.is_file():
            yield path


def check_headers():
    ret = 0
    for f in get_files():
        with open(f, "r") as fd:
            if not HEADER_RE.match(fd.readline()[:-1]):
                print("Missing header", f)
                ret = 1
    return ret


def add_headers():
    for f in get_files():
        data = None
        with open(f, "r") as fd:
            first_line = fd.readline()[:-1]
            if HEADER_RE.match(first_line):
                continue
            data = fd.read()
        with open(f, "w") as fd:
            fd.write(HEADER)
            fd.write(first_line)
            fd.write("\n")
            fd.write(data)
    return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("cmd", choices=["check", "add"])

    args = parser.parse_args()
    if args.cmd == "check":
        sys.exit(check_headers())
    else:
        sys.exit(add_headers())

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import argparse
import subprocess

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("what", nargs="+", choices=["all", "agnostic", "platform", "bindings"])
    args = parser.parse_args()

    ret = subprocess.check_output(
        "cargo tree --workspace --quiet --depth 0".split(), encoding="utf8"
    )

    platform_crates = []
    bindings_crates = []
    agnostic_crates = []
    for line in ret.splitlines():
        if not line.startswith("libparsec"):
            continue

        crate_name = line.split(" ", 1)[0]

        if crate_name.startswith("libparsec_platform_"):
            platform_crates.append(crate_name)
        elif crate_name.startswith("libparsec_bindings_"):
            bindings_crates.append(crate_name)
        else:
            agnostic_crates.append(crate_name)

    crates = []
    what = set(args.what)

    if what & {"all", "agnostic"}:
        crates += agnostic_crates

    if what & {"all", "platform"}:
        crates += platform_crates

    if what & {"all", "bindings"}:
        crates += bindings_crates

    print(" ".join([f"-p {crate}" for crate in crates]))

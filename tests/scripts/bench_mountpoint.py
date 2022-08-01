#! /usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS


import signal
from pathlib import Path
from time import sleep
from tempfile import mkdtemp
from subprocess import run, Popen, PIPE
from contextlib import contextmanager


PORT = 6778
ORGNAME = "Org42"
TOKEN = "CCDCC27B6108438D99EF8AF5E847C3BB"
DEVICE = "alice@dev1"
PASSWORD = "P@ssw0rd."

PARSEC_CLI = "python -m parsec.cli"
PARSEC_PROFILE_CLI = "python -m cProfile -o bench.prof -m parsec.cli"


def run_cmd(cmd):
    print(f"---> {cmd}")
    out = run(cmd.split(), capture_output=True)
    if out.returncode != 0:
        print(out.stdout.decode())
        print(out.stderr.decode())
        raise RuntimeError(f"Error during command `{cmd}`")
    return out


@contextmanager
def keep_running_cmd(cmd):
    print(f"===> {cmd}")
    process = Popen(cmd.split(), stdout=PIPE, stderr=PIPE)
    sleep(0.2)
    if process.poll():
        print(process.stdout.read().decode())
        print(process.stderr.read().decode())
        raise RuntimeError(f"Command `{cmd}` has stopped with status code {process.returncode}")
    try:
        yield
    finally:
        if not process.poll():
            process.send_signal(signal.SIGINT)
            process.wait()
        if process.returncode != 0:
            print(process.stdout.read().decode())
            print(process.stderr.read().decode())
            raise RuntimeError(f"Command `{cmd}` return status code {process.returncode}")


def main():
    workdir = Path(mkdtemp(prefix="parsec-bench-"))
    print(f"Workdir: {workdir}")
    confdir = workdir / "core"
    mountdir = workdir / "mountpoint"
    confdir.mkdir(exist_ok=True)
    mountdir.mkdir(exist_ok=True)

    # Start backend & create organization
    with keep_running_cmd(f"{PARSEC_CLI} backend run --port={PORT}"):
        backend_addr = f"parsec://127.0.0.1:{PORT}?no_ssl=true"

        out = run_cmd(
            f"{PARSEC_CLI} core create_organization {ORGNAME}"
            f" --addr={backend_addr} --administration-token={TOKEN}"
        )

        boostrap_addr = out.stdout.decode().split("Bootstrap organization url: ")[-1].strip()
        out = run_cmd(
            f"{PARSEC_CLI} core bootstrap_organization {DEVICE}"
            f" --addr={boostrap_addr} --config-dir={confdir} --password={PASSWORD}"
        )

        out = run_cmd(
            f"{PARSEC_CLI} core create_workspace w1"
            f" --config-dir={confdir} --device={DEVICE} --password={PASSWORD}"
        )

        with keep_running_cmd(
            f"{PARSEC_PROFILE_CLI} core run -l INFO"
            f" --device={DEVICE} --password={PASSWORD} --mountpoint={mountdir} --config-dir={confdir}"
        ):

            # Wait for mountpoint to be ready
            w1dir = mountdir / "w1"
            for _ in range(10):
                sleep(0.1)
                if w1dir.exists():
                    break
            else:
                RuntimeError("Parsec failed to mount workspace")

            # Create 100 MB file
            file = workdir / "sample"
            file.write_bytes(bytearray(100 * 1024 * 1024))

            try:
                # Copy it into the workspace
                print("********** starting bench ***********")
                run(f"time pv {file} > {mountdir}/w1/sample", shell=True)
                print("********** bench done ***********")
            finally:
                file.unlink()


if __name__ == "__main__":
    main()

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import os
import re
from functools import partial
from pathlib import Path
from typing import IO

try:
    import fcntl
except ModuleNotFoundError:  # Not available on Windows
    pass
import subprocess
import sys
from contextlib import asynccontextmanager, contextmanager
from time import sleep

import anyio
from click.testing import CliRunner, Result

from parsec._parsec import (
    BackendAddr,
    BackendOrganizationAddr,
)
from parsec.cli import cli

# TODO
# from tests.common import (
#     LocalDevice,
#     asgi_app_handle_client_factory,
# )


CWD = Path(__file__).parent.parent
# Starting parsec cli as a new subprocess can be very slow (typically a couple
# of seconds on my beafy machine !), so we use an unusually large value here to
# avoid issues in the CI.
SUBPROCESS_TIMEOUT = 30


def _short_cmd(cmd: str) -> str:
    if len(cmd) < 40:
        return cmd
    else:
        return f"{cmd[:40]}…"


def cli_run(
    cmd: str, env: dict[str, str] = {}, timeout: float = SUBPROCESS_TIMEOUT, capture: bool = True
):
    print(f"========= RUN {cmd} ==============")
    env = {**os.environ.copy(), "DEBUG": "true", **env}
    cooked_cmd = (sys.executable + " -m parsec.cli " + cmd).split()
    kwargs = {}
    if capture:
        kwargs["stdout"] = kwargs["stderr"] = subprocess.PIPE
    ret = subprocess.run(cooked_cmd, cwd=CWD, env=env, timeout=timeout, **kwargs)
    if capture:
        print(ret.stdout.decode(), file=sys.stdout)
        print(ret.stderr.decode(), file=sys.stderr)
    print(f"========= DONE {ret.returncode} ({_short_cmd(cmd)}) ==============")
    assert ret.returncode == 0
    return ret


class LivingStream:
    def __init__(self, stream: IO[bytes]):
        self.stream = stream
        self._already_read_data = ""

    def read(self) -> str:
        new_data = self.stream.read()
        if new_data is not None:
            self._already_read_data += new_data.decode()
        return self._already_read_data


@contextmanager
def cli_running(cmd: str, wait_for: str | None = None, env: dict[str, str] = {}):
    env = {**os.environ.copy(), "DEBUG": "true", **env}
    cooked_cmd = ("python -m parsec.cli " + cmd).split()
    p = subprocess.Popen(
        cooked_cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        stdin=subprocess.PIPE,
        cwd=CWD,
        env=env,
    )
    p.wait_for = partial(_wait_for, p)  # type: ignore
    p.wait_for_regex = partial(_wait_for_regex, p)  # type: ignore
    p.live_stdout = LivingStream(p.stdout)  # type: ignore
    p.live_stderr = LivingStream(p.stderr)  # type: ignore
    # Turn stdin/stdout/stderr non-blocking
    for stream in [p.stdin, p.stdout, p.stderr]:
        fl = fcntl.fcntl(stream, fcntl.F_GETFL)
        fcntl.fcntl(stream, fcntl.F_SETFL, fl | os.O_NONBLOCK)
    try:
        if wait_for:
            out = ""
            for _ in range(SUBPROCESS_TIMEOUT * 10):  # 10ms sleep steps
                out = p.live_stdout.read()[len(out) :]
                if out:
                    print(out, end="")
                if wait_for in out:
                    break
                sleep(0.1)
            else:
                raise RuntimeError("Command took too much time to start")

        yield p

    finally:
        print(f"**************************** TERM to {p.pid} ({_short_cmd(cmd)})")
        p.terminate()
        p.wait()
        print(p.live_stdout.read())
        print(p.live_stderr.read())


def _wait_for(p: subprocess.Popen, wait_txt: str) -> str:
    for _ in range(SUBPROCESS_TIMEOUT * 10):  # 10ms sleep steps
        sleep(0.1)
        stdout = p.live_stdout.read()
        if wait_txt in stdout:
            return stdout
    else:
        raise AssertionError("Too slow")


def _wait_for_regex(p: subprocess.Popen, regex: str, stderr: bool = False) -> re.Match[str]:
    for _ in range(SUBPROCESS_TIMEOUT * 10):  # 10ms sleep steps
        sleep(0.1)
        if stderr:
            output = p.live_stderr.read()
        else:
            output = p.live_stdout.read()
        match = re.search(regex, output, re.MULTILINE)
        if match:
            return match
    else:
        raise AssertionError("Too slow")


@asynccontextmanager
async def cli_with_running_backend_testbed(backend_asgi_app, *devices):
    # The cli commands are going to run in a separate thread with they own trio loop,
    # hence we should not share memory channel / events between trio loops otherwise
    # unexpected errors will occur !
    async with anyio.create_task_group() as task_group:
        listeners = await task_group.start(
            partial(
                # TODO !
                trio.serve_tcp,
                asgi_app_handle_client_factory(backend_asgi_app),
                port=0,
                host="127.0.0.1",
            )
        )
        _, port, *_ = listeners[0].socket.getsockname()
        backend_addr = BackendAddr("127.0.0.1", port, use_ssl=False)

        # Now the local device point to an invalid backend address, must fix this
        def _correct_local_device_backend_addr(device):
            organization_addr = BackendOrganizationAddr.build(
                backend_addr,
                organization_id=device.organization_addr.organization_id,
                root_verify_key=device.organization_addr.root_verify_key,
            )
            return LocalDevice(
                organization_addr=organization_addr,
                device_id=device.device_id,
                device_label=device.device_label,
                human_handle=device.human_handle,
                signing_key=device.signing_key,
                private_key=device.private_key,
                profile=device.profile,
                user_realm_id=device.user_realm_id,
                user_realm_key=device.user_realm_key,
                local_symkey=device.local_symkey,
            )

        yield (backend_addr, *(_correct_local_device_backend_addr(d) for d in devices))

        task_group.cancel_scope.cancel()


async def cli_invoke_in_thread(cmd: str, input: str | None = None) -> Result:
    runner = CliRunner()
    # We must run the command from another thread given it will create it own asyncio loop
    # Pass DEBUG environment variable for better output on crash
    return await anyio.to_thread.run_sync(
        lambda: runner.invoke(cli, cmd, input=input, env={"DEBUG": "1"})
    )

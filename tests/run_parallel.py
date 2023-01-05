#! /usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

"""
Why this an not just pytest_xdist ?

The issue is pytest_xdist is based on the `execnet` library for dispatching
tests. This is what allow pytest_xdist to run tests across different machines,
but is a bit overkill if you only want to run tests locally.

However the main issue is that `execnet` doesn't run the actual pytest code
in the main thread of the newly created process which is an issue on macOS.

On top of that I suspect the forking mechanism of pytest_xdist to keep sharing
too much things between processes (given the fork occurs after pytest has been
started). This is probably not the case, but I'm currently a very desperate
man considering how unstable the CI is right now...

So the idea is simple here:
- Each child starts pytest with the `--slice-test` option, so parent doesn't even
  have to do the test dispatching. The drawback is we cannot do work stealing if
  a job lags behind because it got most of the slow tests.
- We share as few thing as possible between parent and child process (i.e. a single
  queue to inform what is going in the child).
"""

import multiprocessing
import queue
import sys
from collections import defaultdict
from typing import List

import pytest

COLOR_END = "\033[0m"
COLOR_RED = "\033[91m"
COLOR_GREEN = "\033[92m"
COLOR_YELLOW = "\033[93m"


class TestStatusReportPlugin:
    def __init__(self, job_id: int, report_queue: multiprocessing.Queue):
        self.job_id = job_id
        self.report_queue = report_queue

    def sendevent(self, name, **kwargs):
        self.report_queue.put((self.job_id, name, kwargs))

    @pytest.hookimpl
    def pytest_internalerror(self, excrepr):
        formatted_error = str(excrepr)
        self.sendevent("internal_error", formatted_error=formatted_error)

    @pytest.hookimpl
    def pytest_collection_finish(self, session: pytest.Session):
        self.sendevent("collection_finish", total_tests_count=len(session.items))

    @pytest.hookimpl
    def pytest_sessionstart(self, session):
        self.sendevent("workerready")

    @pytest.hookimpl
    def pytest_sessionfinish(self, exitstatus):
        self.sendevent("workerfinished", exitstatus=exitstatus)

    @pytest.hookimpl
    def pytest_runtest_logreport(self, report: pytest.TestReport):
        # Ignore tests skipped for being out of the job's tests slice
        if ("test_out_of_slice", True) in report.user_properties:
            return
        self.sendevent(
            "testreport",
            nodeid=report.nodeid,
            outcome=report.outcome,
            when=report.when,
            longrepr=report.longrepr,
            duration=report.duration,
        )

    @pytest.hookimpl
    def pytest_warning_recorded(self, warning_message, when, nodeid, location):
        self.sendevent(
            "warning_recorded",
            warning_message_data=str(warning_message),
            when=when,
            nodeid=nodeid,
            location=location,
        )


def _run_pytest(job_index, args, plugins):
    import os
    import sys

    # Stdout is shared with parent process, so we must disable it to keep it readable
    sys.stdout = open(os.devnull, "w")
    try:
        pytest.main(args, plugins)

    except BaseException as exc:
        import traceback

        tb_formatted = traceback.format_exception(exc, exc, exc.__traceback__)
        msg = "unexpected exception from pytest:\n" + "\n".join(tb_formatted)
        # Print on stderr so we're sure the stacktrace will appear in the logs
        # even if parent process fail to process the event when send
        print(
            f">>> [gw{job_index}] {COLOR_RED}CRASH !!!{COLOR_END} {msg}",
            file=sys.stderr,
            flush=True,
        )
        plugins[0].report_queue.put((job_index, "unexpected_exception", {"msg": msg}))

        if not isinstance(exc, Exception):
            raise exc


if __name__ == "__main__":
    verbose = any(True for x in sys.argv if x == "--verbose" or x.startswith("-v"))
    fast_fail = "-x" in sys.argv
    try:
        index = sys.argv.index("-n")
        parallelism = sys.argv[index + 1]
        if parallelism == "auto":
            parallelism = multiprocessing.cpu_count()
        parallelism = int(parallelism)
    except (IndexError, ValueError):
        raise SystemExit(f"usage: {sys.argv[0]} -n auto tests")

    args = sys.argv[1:index] + sys.argv[index + 2 :]

    multiprocessing.set_start_method("spawn")
    print("==== Running in parallel ===")
    report_queue = multiprocessing.Queue()
    jobs: List[multiprocessing.Process] = []
    for job_index in range(parallelism):
        job_args = [f"--slice-tests={job_index + 1}/{parallelism}", *args]
        print(f"pytest {' '.join(job_args)}")
        plugins = [TestStatusReportPlugin(job_index, report_queue)]
        job = multiprocessing.Process(target=_run_pytest, args=[job_index, job_args, plugins])
        jobs.append(job)

    jobs_status: List[str] = []
    for job in jobs:
        job.start()
        jobs_status.append("started")

    total_tests_count = None
    tests_started = {}
    tests_has_failed = False

    def _set_test_has_failed():
        global tests_has_failed
        tests_has_failed = True
        if fast_fail:
            raise KeyboardInterrupt()

    # Use a default dict here, so setting the error never overwrite a previously
    # set error (this is suppose not to happen, but if it does we really want to know !)
    job_crashes = defaultdict(lambda: "")

    def _percent_display(event_params):
        percent_color = COLOR_RED if tests_has_failed else COLOR_GREEN
        percent = len(tests_started) * 100 / total_tests_count
        return f"{percent_color}[{int(percent)}%]{COLOR_END}"

    try:
        while True:
            try:
                job_index, event_name, event_params = report_queue.get(timeout=1)
            except queue.Empty:
                if all(not job.is_alive() for job in jobs):
                    break

            else:

                if event_name == "unexpected_exception":
                    # The event has already be printed by the child process,
                    # only store the event for final recap
                    job_crashes[
                        job_index
                    ] += (
                        f">>> [gw{job_index}] {COLOR_RED}CRASH !!!{COLOR_END} {event_params['msg']}"
                    )
                    _set_test_has_failed()

                elif event_name == "internal_error":
                    msg = f">>> [gw{job_index}] {COLOR_RED}CRASH !!!{COLOR_END} Pytest internal error:\n{event_params['formatted_error']}"
                    print(msg, flush=True)
                    job_crashes[job_index] += msg
                    _set_test_has_failed()

                elif event_name == "collection_finish":
                    total_tests_count = event_params["total_tests_count"]

                elif event_name == "workerready":
                    jobs_status[job_index] = "ready"

                elif event_name == "workerfinished":
                    jobs_status[job_index] = "finished"
                    print(
                        f">>> [gw{job_index}] pytest job has finished with status {COLOR_RED if event_params['exitstatus'] else COLOR_GREEN}{event_params['exitstatus']}{COLOR_END}"
                    )
                    if event_params["exitstatus"] != 0:
                        _set_test_has_failed()
                    if all(x == "finished" for x in jobs_status):
                        break

                elif event_name == "testreport":
                    percent_color = COLOR_RED if tests_has_failed else COLOR_GREEN
                    tests_started.setdefault(event_params["nodeid"], event_params["outcome"])
                    base = (
                        f"[gw{job_index}] {_percent_display(event_params)} {event_params['nodeid']}"
                    )
                    if event_params["when"] == "setup":
                        if event_params["outcome"] == "skipped":
                            outcome = f"{COLOR_YELLOW}SKIPPED{COLOR_END}"
                            print(f"{base} {outcome}")
                        elif event_params["outcome"] == "passed":
                            outcome = "..."
                            print(f"{base} {outcome}")
                        else:
                            outcome = f"{COLOR_RED}Error !!!{COLOR_END}\n{event_params['longrepr']}"
                            print(f"{base} {outcome}")
                            _set_test_has_failed()

                    elif event_params["when"] == "call":
                        if tests_started.get(event_params["nodeid"]) == "skipped":
                            continue
                        if event_params["outcome"] not in ("skipped", "passed"):
                            outcome = f"{COLOR_RED}Error !!!{COLOR_END}\n{event_params['longrepr']}"
                            tests_started[event_params["nodeid"]] = ("error", outcome)
                            print(f"{base} {outcome}")
                            _set_test_has_failed()

                    elif event_params["when"] == "teardown":
                        if tests_started.get(event_params["nodeid"]) == "skipped":
                            continue
                        # Teardown is never in `skipped` state
                        if event_params["outcome"] == "passed":
                            if event_params["duration"] > 1:
                                outcome = (
                                    f"{COLOR_GREEN}PASSED{COLOR_END} ({event_params['duration']}s)"
                                )
                            else:
                                outcome = f"{COLOR_GREEN}PASSED{COLOR_END}"
                            print(f"{base} {outcome}")
                        else:
                            _set_test_has_failed()
                            outcome = f"{COLOR_RED}Error !!!{COLOR_END}\n{event_params['longrepr']}"
                            print(f"{base} {outcome}")

                elif event_name == "warning_recorded":
                    base = f">>> [gw{job_index}] {_percent_display(event_params)} {event_params['nodeid']}"
                    print(
                        f"{base} {COLOR_YELLOW}Warning during {event_params['when']} step{COLOR_END}: {event_params['warning_message_data']}"
                    )

    except KeyboardInterrupt:
        print("^C hit, terminating jobs...")
        for job in jobs:
            job.terminate()

    else:
        if any(job_status != "finished" for job_status in jobs_status):
            tests_has_failed = True  # Just to be sure
            print(
                ">>> Some jobs died before pytest has finished :'(\n"
                + "  \n".join(
                    f"[gw{job_index}] last status {COLOR_GREEN if job_status == 'finished' else COLOR_RED}{job_status}{COLOR_END}"
                    for job_index, job_status in enumerate(jobs_status)
                ),
                flush=True,
            )

    for job_index, job in enumerate(jobs):
        crash_msg = job_crashes.get(job_index)
        if crash_msg:
            print(f">>> [gw{job_index}] {COLOR_RED}CRASH !!!{COLOR_END} {crash_msg}", flush=True)
        try:
            job.join(timeout=5)
        except TimeoutError:
            print(f"Job gw{job_index} takes too long to join...")

    raise SystemExit(1 if tests_has_failed else 0)

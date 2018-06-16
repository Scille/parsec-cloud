import pytest
import trio
import queue
import shutil
from copy import deepcopy
from functools import partial
from contextlib import contextmanager
from pathlib import Path
from hypothesis.stateful import run_state_machine_as_test, RuleBasedStateMachine


class ThreadToTrioCommunicator:
    def __init__(self, portal, timeout=None):
        self.timeout = timeout
        self.portal = portal
        self.queue = queue.Queue()
        self.trio_queue = trio.Queue(1)

    def send(self, msg):
        self.portal.run(self.trio_queue.put, msg)
        ret = self.queue.get(timeout=self.timeout)
        if isinstance(ret, Exception):
            raise ret

        return ret

    async def trio_recv(self):
        ret = await self.trio_queue.get()
        return ret

    async def trio_respond(self, msg):
        self.queue.put(msg)

    def close(self):
        # Avoid deadlock if somebody is waiting on the other end
        self.queue.put(RuntimeError("Communicator has closed while something was still listening"))


@contextmanager
def open_communicator(portal):
    communicator = ThreadToTrioCommunicator(portal)
    try:
        yield communicator

    except Exception as exc:
        # Pass the exception to the listening part, to have the current
        # hypothesis rule crash correctly
        communicator.queue.put(exc)
        raise

    finally:
        communicator.close()


@pytest.fixture
async def portal():
    return trio.BlockingTrioPortal()


@pytest.fixture
async def TrioDriverRuleBasedStateMachine(nursery, portal, loghandler, hypothesis_settings):
    class TrioDriverRuleBasedStateMachine(RuleBasedStateMachine):
        _portal = portal
        _nursery = nursery
        _running = trio.Lock()

        @classmethod
        async def run_test(cls):
            await trio.run_sync_in_worker_thread(
                partial(run_state_machine_as_test, cls, settings=hypothesis_settings)
            )

        async def trio_runner(self, task_status):
            raise NotImplementedError()

        @property
        def communicator(self):
            assert self._communicator
            return self._communicator

        async def _trio_runner(self, *, task_status=trio.TASK_STATUS_IGNORED):
            print("=====================================================")
            # We need to hijack `task_status.started` callback because error
            # handling of trio_runner coroutine depends of it (see below).
            task_started = False
            vanilla_task_status_started = task_status.started

            def task_status_started_hook(ret=None):
                nonlocal task_started
                task_started = True
                vanilla_task_status_started(ret)

            task_status.started = task_status_started_hook

            # Drop previous run logs, preventing flooding stdout
            loghandler.records.clear()
            try:
                # This lock is to make sure the hypothesis thread doesn't start
                # another `_trio_runner` coroutine while this one hasn't done
                # it teardown yet.
                async with self._running:
                    with trio.open_cancel_scope() as self._trio_runner_cancel_scope:
                        with open_communicator(self._portal) as self._communicator:
                            await self.trio_runner(task_status)
            except Exception as exc:
                if not task_started:
                    # If the crash occurs during the init phase, hypothesis
                    # thread is synchrone with this coroutine so raising the
                    # exception here will have the expected effect.
                    raise

        def __init__(self, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self._trio_runner_crash = None
            self._portal.run(self._nursery.start, self._trio_runner)

        def teardown(self):
            self._trio_runner_cancel_scope.cancel()

    return TrioDriverRuleBasedStateMachine


@pytest.fixture
def oracle_fs_factory(tmpdir):
    class OracleFS:
        def __init__(self, base_path):
            self.base = Path(base_path)
            self.base.mkdir(parents=True)
            self.root = self.base / "root"
            self.root.mkdir()
            self.base.chmod(0o500)  # Root oracle can no longer be removed this way
            self.entries_stats = {}
            self._register_stat(self.root, "folder")

        def _cook_path(self, path):
            assert path[0] == "/"
            return self.root / path[1:]

        def _register_stat(self, path, type):
            self.entries_stats[path] = {
                "type": type,
                "base_version": 0,
                "is_placeholder": True,
                "need_sync": True,
            }

        def dump(self):
            content = []

            def _recursive_dump(path):
                for child in path.iterdir():
                    entry_name = self._relative_path(child)
                    if child.is_dir():
                        _recursive_dump(child)
                        entry_name += "/"
                    stat = self.entries_stats[child]
                    content.append((entry_name, stat["base_version"], stat["need_sync"]))

            _recursive_dump(self.root)
            return sorted(content)

        def copy(self):
            new_oracle = _oracle_fs_factory()
            for path, stat in self.entries_stats.items():
                new_path = new_oracle.root / path.relative_to(self.root)
                new_oracle.entries_stats[new_path] = stat.copy()
            # copytree requires the target folder doesn't exist
            new_oracle.base.chmod(0o700)
            new_oracle.root.rmdir()
            shutil.copytree(self.root, new_oracle.root)
            new_oracle.base.chmod(0o500)
            return new_oracle

        def create_file(self, path):
            path = self._cook_path(path)
            try:
                path.touch(exist_ok=False)
            except OSError as exc:
                return "invalid_path"
            self._register_stat(path, "file")
            self.entries_stats[path.parent]["need_sync"] = True
            return "ok"

        def create_folder(self, path):
            path = self._cook_path(path)
            try:
                path.mkdir(exist_ok=False)
            except OSError as exc:
                return "invalid_path"
            self._register_stat(path, "folder")
            self.entries_stats[path.parent]["need_sync"] = True
            return "ok"

        def unlink(self, path):
            path = self._cook_path(path)
            try:
                path.unlink()
            except OSError as exc:
                return "invalid_path"
            del self.entries_stats[path]
            self.entries_stats[path.parent]["need_sync"] = True
            return "ok"

        def rmdir(self, path):
            path = self._cook_path(path)
            try:
                path.rmdir()
            except OSError as exc:
                return "invalid_path"
            self._delete_stats(path)
            self.entries_stats[path.parent]["need_sync"] = True
            return "ok"

        def delete(self, path):
            cooked_path = self._cook_path(path)
            if cooked_path.is_file():
                return self.unlink(path)
            else:
                return self.rmdir(path)

        def _move_stats(self, oldpath, newpath):
            new_stats = {}
            for canditate_path, candidate_stat in self.entries_stats.items():
                try:
                    new_candidate_path = newpath / canditate_path.relative_to(oldpath)
                except ValueError:
                    # Candidate is not a child of oldpath
                    new_stats[canditate_path] = candidate_stat
                    continue
                new_stats[new_candidate_path] = candidate_stat
            self.entries_stats = new_stats

        def _delete_stats(self, oldpath):
            new_stats = {}
            for canditate_path, candidate_stat in self.entries_stats.items():
                try:
                    canditate_path.relative_to(oldpath)
                except ValueError:
                    # Candidate is not a child of oldpath
                    new_stats[canditate_path] = candidate_stat
            self.entries_stats = new_stats

        def move(self, src, dst):
            src = self._cook_path(src)
            dst = self._cook_path(dst)
            try:
                src.rename(str(dst))
            except OSError as exc:
                return "invalid_path"
            if src != dst:
                # TODO: currently move create a new access only for
                # the moved entry (and not it children)...
                self.entries_stats[dst] = {
                    "type": self.entries_stats.pop(src)["type"],
                    "base_version": 0,
                    "is_placeholder": True,
                    "need_sync": True,
                }
                self._move_stats(src, dst)
                self.entries_stats[dst.parent]["need_sync"] = True
                self.entries_stats[src.parent]["need_sync"] = True
            return "ok"

        def sync(self, path, *, sync_cb=lambda path, stat: None):
            path = self._cook_path(path)
            if path.exists():
                self._backward_recursive_sync(path, sync_cb)
                self._recursive_children_sync(path, sync_cb)
                return "ok"
            return "invalid_path"

        def _relative_path(self, path):
            path = str(path.relative_to(self.root))
            return "/" if path == "." else f"/{path}"

        def _recursive_children_sync(self, path, sync_cb):
            if path.is_dir():
                for child in path.iterdir():
                    child_stat = self.entries_stats[child]
                    if child_stat["need_sync"]:
                        child_stat["need_sync"] = False
                        child_stat["is_placeholder"] = False
                        child_stat["base_version"] += 1
                        sync_cb(self._relative_path(child), child_stat)
                    self._recursive_children_sync(child, sync_cb)

        def _backward_recursive_sync(self, path, sync_cb):
            stat = self.entries_stats[path]
            if not stat["need_sync"]:
                return

            def _recursive_sync_placeholder_parent(path):
                parent_stat = self.entries_stats[path.parent]
                parent_is_placeholder = parent_stat["is_placeholder"]

                parent_stat["base_version"] += 1
                parent_stat["is_placeholder"] = False
                # Parent got a minimal sync: if other placeholder children are
                # present they won't be synchronized
                for otherchild in path.parent.iterdir():
                    if otherchild == path:
                        continue
                    if self.entries_stats[otherchild]["is_placeholder"]:
                        parent_stat["need_sync"] = True
                else:
                    parent_stat["need_sync"] = False
                sync_cb(self._relative_path(path.parent), parent_stat)

                if parent_is_placeholder:
                    _recursive_sync_placeholder_parent(path.parent)

            # If path is a placeholder, synchronizing it means we must
            # synchronize it parents first

            is_placeholder = stat["is_placeholder"]
            stat["need_sync"] = False
            stat["is_placeholder"] = False
            stat["base_version"] += 1
            sync_cb(self._relative_path(path), stat)

            if path == self.root:
                return

            if is_placeholder:
                _recursive_sync_placeholder_parent(path)

        def stat(self, path):
            path = self._cook_path(path)
            if path.exists():
                return {"status": "ok", **self.entries_stats[path]}
            else:
                return {"status": "invalid_path"}

    count = 0

    def _oracle_fs_factory():
        nonlocal count
        count += 1
        return OracleFS(Path(tmpdir / f"oracle_fs-{count}"))

    return _oracle_fs_factory

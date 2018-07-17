import pytest
import attr
import trio
import shutil
from pathlib import Path
from hypothesis_trio.stateful import TrioRuleBasedStateMachine, initialize, Bundle


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


@attr.s
class StartedCore:
    core = attr.ib()
    sock = attr.ib()
    need_stop = attr.ib()
    stopped = attr.ib()

    async def stop(self):
        self.need_stop.set()
        await self.stopped.wait()

    async def core_cmd(self, msg):
        await self.sock.send(msg)
        return await self.sock.recv()


@attr.s
class StartedBackend:
    backend = attr.ib()
    server = attr.ib()
    need_stop = attr.ib()
    stopped = attr.ib()

    async def stop(self):
        self.need_stop.set()
        await self.stopped.wait()


@pytest.fixture
def BaseParsecStateMachine(
    server_factory, backend_addr, backend_factory, core_factory, core_sock_factory, device_factory
):
    class BaseParsecStateMachine(TrioRuleBasedStateMachine):
        async def start_core(self, device, backend_addr=backend_addr):
            async def _start(*, task_status=trio.TASK_STATUS_IGNORED):
                async with trio.open_nursery() as nursery:
                    stopped = trio.Event()
                    need_stop = trio.Event()
                    core = await core_factory(
                        devices=[device],
                        config={"auto_sync": False, "backend_addr": backend_addr},
                        nursery=nursery,
                    )
                    try:
                        await core.login(device)
                        sock = core_sock_factory(core, nursery=nursery)
                        task_status.started(StartedCore(core, sock, need_stop, stopped))
                        await need_stop.wait()
                        await core.teardown()
                        nursery.cancel_scope.cancel()
                    finally:
                        stopped.set()

            return await self.get_root_nursery().start(_start)

        async def start_backend(self, devices, backend_addr=backend_addr):
            async def _start(*, task_status=trio.TASK_STATUS_IGNORED):
                async with trio.open_nursery() as nursery:
                    stopped = trio.Event()
                    need_stop = trio.Event()
                    backend = await backend_factory(devices=devices, nursery=nursery)
                    try:
                        server = server_factory(backend.handle_client, nursery=nursery)
                        task_status.started(StartedBackend(backend, server, need_stop, stopped))
                        await need_stop.wait()
                        await backend.teardown()
                        nursery.cancel_scope.cancel()
                    finally:
                        stopped.set()

            return await self.get_root_nursery().start(_start)

    return BaseParsecStateMachine


@pytest.fixture
def BaseCoreAloneStateMachine(BaseParsecStateMachine, device_factory):
    class BaseCoreAloneStateMachine(BaseParsecStateMachine):
        Cores = Bundle("core")

        def __init__(self, **kwargs):
            super().__init__(**kwargs)
            self.device = device_factory()
            self.core_info = None

        @initialize(target=Cores)
        async def _init_core(self):
            await self.restart_core()
            return self.core_info.core

        async def restart_core(self, reset_local_db=False):
            await self.teardown()

            if reset_local_db:
                self.device.local_db._data.clear()

            self.core_info = await self.start_core(self.device)

        async def teardown(self):
            if self.core_info:
                await self.core_info.stop()
                self.core_info = None

        async def core_cmd(self, msg):
            return await self.core_info.core_cmd(msg)

    return BaseCoreAloneStateMachine


@pytest.fixture
def BaseCoreWithBackendStateMachine(BaseParsecStateMachine, device_factory):
    class BaseCoreWithBackendStateMachine(BaseParsecStateMachine):
        Cores = Bundle("core")
        Backends = Bundle("backend")

        def __init__(self, **kwargs):
            super().__init__(**kwargs)
            self.device = device_factory()
            self.core_info = None
            self.backend_info = None

        @initialize(target=Backends)
        async def _init_backend(self):
            self.backend_info = await self.start_backend(devices=[self.device])
            return self.backend_info.backend

        @initialize(target=Cores, backend=Backends)
        async def _init_core(self, backend):
            await self.restart_core()
            return self.core_info.core

        async def restart_core(self, reset_local_db=False):
            if self.core_info:
                await self.core_info.stop()

            if reset_local_db:
                self.device.local_db._data.clear()

            self.core_info = await self.start_core(
                self.device, backend_addr=self.backend_info.server.addr
            )

        async def teardown(self):
            if self.core_info:
                await self.core_info.stop()
            if self.backend_info:
                await self.backend_info.stop()

        async def core_cmd(self, msg):
            return await self.core_info.core_cmd(msg)

    return BaseCoreWithBackendStateMachine

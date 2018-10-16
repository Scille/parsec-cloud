import os
import trio
import time
import multiprocessing
from queue import Empty
from uuid import uuid4
from pathlib import Path
from fuse import FUSE

from parsec.core.mountpoint.operations import FuseOperations
from parsec.core.mountpoint.exceptions import MountpointConfigurationError


multiprocessing.freeze_support()


def _bootstrap_mountpoint(mountpoint):
    if os.name == "posix":
        # On POSIX systems, mounting target must exists
        mountpoint.mkdir(exist_ok=True, parents=True)
        initial_st_dev = mountpoint.stat().st_dev
    else:
        # On Windows, only parent's mounting target must exists
        mountpoint.parent.mkdir(exist_ok=True, parents=True)
        if mountpoint.exists():
            raise MountpointConfigurationError(
                f"Mountpoint `{mountpoint.absolute()}` must not exists on windows"
            )
        initial_st_dev = None

    return initial_st_dev


def _run_fuse_process(fs_access_client, abs_mountpoint, fuse_config):
    try:
        fuse_operations = FuseOperations(fs_access_client, abs_mountpoint)
        FUSE(fuse_operations, abs_mountpoint, foreground=True, **fuse_config)
    except BaseException as exc:
        # Cannot just send the exception object given it could be
        # non-pickle
        # TODO: traceback.format_exc(exc) crash on trio.MultiError
        fs_access_client.client_crash(str(exc))
        raise


async def run_fuse_in_process(
    fs, mountpoint: Path, fuse_config: dict, *, task_status=trio.TASK_STATUS_IGNORED
):
    abs_mountpoint = str(mountpoint.absolute())
    fs_access = ProcessFSAccess(fs)

    initial_st_dev = _bootstrap_mountpoint(mountpoint)

    fuse_process = multiprocessing.Process(
        target=_run_fuse_process, args=(fs_access.get_client(), abs_mountpoint, fuse_config)
    )

    fuse_runner_stopped = trio.Event()
    try:
        async with trio.open_nursery() as nursery:

            async def _stop_fuse_runner():
                nursery.cancel_scope.cancel()
                await fuse_runner_stopped.wait()
                # await _stop_fuse_process(mountpoint, fuse_process)

            nursery.start_soon(
                _wait_for_fuse_ready,
                mountpoint,
                initial_st_dev,
                lambda: task_status.started(_stop_fuse_runner),
            )

            fuse_process.start()
            await fs_access.run_server()

    finally:
        await _stop_fuse_process(mountpoint, fuse_process)
        fuse_runner_stopped.set()


async def _wait_for_fuse_ready(mountpoint, initial_st_dev, started_cb):

    # Polling until fuse is ready
    # Note given python fs api is blocking, we must run it inside a thread
    # to avoid blocking the trio loop and ending up in a deadlock

    need_stop = False

    def _wait_for_fuse_ready_thread():
        while not need_stop:
            # print('wait for fuse...')
            time.sleep(0.1)
            try:
                if mountpoint.stat().st_dev != initial_st_dev:
                    print("fuse ready !")
                    break
            except FileNotFoundError:
                pass
        print("fusee crashed ?")

    try:
        await trio.run_sync_in_worker_thread(_wait_for_fuse_ready_thread, cancellable=True)
    finally:
        need_stop = True
    started_cb()


async def _stop_fuse_process(mountpoint, fuse_process):

    # Ask for dummy file just to force a fuse operation that will
    # process the SIGTERM signal.
    # Note given python fs api is blocking, we must run it inside a thread
    # to avoid blocking the trio loop and ending up in a deadlock

    def _stop_fuse():
        fuse_process.terminate()
        # TODO: useful ?
        while True:
            try:
                (mountpoint / "__shutdown_fuse__").exists()
            except OSError:
                pass
            print("Try to close fuse..")
            fuse_process.join()
            if not fuse_process.is_alive():
                print("SUCCESS !")
                break

    if fuse_process.is_alive():
        await trio.run_sync_in_worker_thread(_stop_fuse)

    if os.name == "posix":
        try:
            mountpoint.rmdir()
        except OSError:
            pass


class ProcessFSAccess:
    def __init__(self, fs):
        self.fs = fs
        self._stopping = multiprocessing.Event()
        self._req_queue = multiprocessing.JoinableQueue()
        self._rep_queue = multiprocessing.Queue()

    def get_client(self):
        return ProcessFSAccessClient(self._stopping, self._req_queue, self._rep_queue)

    async def run_server(self):
        async with trio.open_nursery() as nursery:
            print("[server] waiting for req...")
            try:
                while True:

                    def _get_req():
                        while True:
                            try:
                                req = self._req_queue.get(timeout=1)
                                break
                            except Empty:
                                if self._stopping.is_set():
                                    return
                                continue
                        if req[1] == "noop":
                            self._req_queue.task_done()
                        elif req[1] == "crash":
                            self._req_queue.task_done()
                            raise RuntimeError(f"Client has crashed: {req[2][0]}")
                        return req

                    req_id, req_cmd, req_args = await trio.run_sync_in_worker_thread(
                        _get_req, cancellable=True
                    )
                    print("[server] recv", req_id, req_cmd, req_args)
                    nursery.start_soon(self._process_req, req_id, req_cmd, req_args)

            except BaseException:
                self._stopping.set()
                # Wait for the current request to finish to avoid deadlock
                # in the fuse process
                await trio.run_sync_in_worker_thread(self._req_queue.join)
                raise

    async def _process_req(self, req_id, req_cmd, req_args):
        try:
            print("process", req_id, req_cmd, req_args)
            if req_cmd == "file_create":

                async def _do(path):
                    await self.fs.file_create(path)
                    return await self.fs.file_fd_open(path)

                rep = await _do(*req_args)

            elif req_cmd == "file_fd_seek_and_read":

                async def _do(fh, size, offset):
                    await self.fs.file_fd_seek(fh, offset)
                    return await self.fs.file_fd_read(fh, size)

                rep = await _do(*req_args)

            elif req_cmd == "file_fd_seek_and_write":

                async def _do(fh, data, offset):
                    await self.fs.file_fd_seek(fh, offset)
                    return await self.fs.file_fd_write(fh, data)

                rep = await _do(*req_args)

            else:
                rep = await getattr(self.fs, req_cmd)(*req_args)

            self._rep_queue.put((req_id, rep))
            self._req_queue.task_done()
            print("[server] done with", req_id, req_cmd, req_args, rep)

        except Exception as exc:
            self._rep_queue.put((req_id, exc))
            self._req_queue.task_done()
            print("[server] done with", req_id, req_cmd, req_args, exc)

        except BaseException as exc:
            self._rep_queue.put((req_id, exc))
            self._req_queue.task_done()
            raise


class ProcessFSAccessClient:
    def __init__(self, stopping, req_queue, rep_queue):
        self._stopping = stopping
        self._req_queue = req_queue
        self._rep_queue = rep_queue

    def _send_req(self, req_cmd, *req_args):
        if self._stopping.is_set():
            raise RuntimeError("Server is stopping")
        req_id = uuid4()
        print("[client] send", req_id, req_cmd, req_args)
        self._req_queue.put((req_id, req_cmd, req_args))
        while True:
            rep_req_id, rep_val = self._rep_queue.get()
            print("[client] recv", rep_req_id, rep_val)
            if rep_req_id != req_id:
                # Given multiple client threads share the same rep_queue, it's
                # possible we didn't get our own request's answer...
                self._rep_queue.put((rep_req_id, rep_val))
                continue
            print("[client] good answer for", req_id, rep_val)
            if isinstance(rep_val, Exception):
                raise rep_val
            else:
                return rep_val

    def client_crash(self, exc):
        self._req_queue.put((0, "crash", (exc,)))

    def stat(self, path):
        return self._send_req("stat", path)

    def delete(self, path):
        return self._send_req("delete", path)

    def move(self, src, dst):
        return self._send_req("move", src, dst)

    def file_create(self, path):
        return self._send_req("file_create", path)

    def folder_create(self, path):
        return self._send_req("folder_create", path)

    def file_truncate(self, path, length):
        return self._send_req("file_truncate", path, length)

    def file_fd_open(self, path):
        return self._send_req("file_fd_open", path)

    def file_fd_close(self, fh):
        return self._send_req("file_fd_close", fh)

    def file_fd_seek(self, fh, offset):
        return self._send_req("file_fd_seek", fh, offset)

    def file_fd_read(self, fh, size):
        return self._send_req("file_fd_read", fh, size)

    def file_fd_seek_and_read(self, fh, size, offset):
        return self._send_req("file_fd_seek_and_read", fh, size, offset)

    def file_fd_seek_and_write(self, fh, data, offset):
        return self._send_req("file_fd_seek_and_write", fh, data, offset)

    def file_fd_write(self, fh, data):
        return self._send_req("file_fd_write", fh, data)

    def file_fd_truncate(self, fh, length):
        return self._send_req("file_fd_truncate", fh, length)

    def file_fd_flush(self, fh):
        return self._send_req("file_fd_flush", fh)

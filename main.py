import signal
from multiprocessing import Process

from parsec2.vfs import VFSClient
from parsec2.vfs.mock import VFSMock
from parsec2.ui.fuse import FuseUI


def main(mock_path, mountpoint):
    vfs_server = VFSMock(mock_path)
    vfs_client = VFSClient(vfs_server.cmd_dispach)
    fuseui = FuseUI(mountpoint, vfs_client)

    fuseui.start()

    # p1 = Process(target=vfs_server.start)
    # p2 = Process(target=fuseui.start)
    # p1.start()
    # p2.start()

    # def graceful_exit():
    #     p1.terminate()
    #     p2.terminate()

    # signal.signal(signal.SIGINT, graceful_exit)
    # signal.signal(signal.SIGTERM, graceful_exit)
    # p1.join()
    # p2.join()


if __name__ == '__main__':
    main('/home/emmanuel/projects/parsec-cloud/tmp/mocked', '/home/emmanuel/projects/parsec-cloud/tmp/mounted')

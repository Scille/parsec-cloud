import sys
import asyncio
import asyncssh
import zmq
import zmq.asyncio

from ..abstract import BaseServer
from ..vfs import BaseVFSClient, VFSFileNotFoundError
from ..vfs.vfs_pb2 import Stat


class MySFTPServer(asyncssh.SFTPServer):
    def __init__(self, conn):
        root = '/tmp/sftp/' + conn.get_extra_info('username')
        os.makedirs(root, exist_ok=True)
        super().__init__(conn, chroot=root)


class SFTPUIServer(BaseServer):
    def __init__(self, vfs: BaseVFSClient, host: str='', port: int=8022):
        self.host = host
        self.port = port
        self.vfs = vfs

    def start(self):
        loop = asyncio.get_event_loop()
        loop.run_until_complete(self._start_server())
        loop.run_forever()


    async def _start_server(self):
        # await asyncssh.listen(self.host, self.port, server_host_keys=['ssh_host_key'],
        #                       authorized_client_keys='ssh_user_ca',
        #                       sftp_factory=MySFTPServer)
        await asyncssh.listen(self.host, self.port, server_host_keys=['ssh_host_key'],
                              authorized_client_keys='/home/emmanuel/.ssh/id_rsa.pub',
                              sftp_factory=True)

    # async def _recv_and_process(self):
    #     sock = ctx.socket(zmq.PULL)
    #     sock.bind("%s:%s" % (self.host, self.port))
    #     msg = await sock.recv_multipart() # waits for msg to be ready
    #     reply = await async_process(msg)
    #     yield from sock.send_multipart(reply)

    # ctx = zmq.asyncio.Context()
    # loop = zmq.asyncio.ZMQEventLoop()
    # asyncio.set_event_loop(loop)


    def stop(self):
        raise NotImplementedError()

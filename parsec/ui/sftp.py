import os
from io import IOBase
from stat import S_IRWXU, S_IRWXG, S_IRWXO, S_IFDIR, S_IFREG
import asyncio
import asyncssh
from asyncssh.constants import (
    FXF_READ, FXF_WRITE, FXF_APPEND, FXF_CREAT, FXF_TRUNC, FXF_EXCL,
    FX_NO_SUCH_FILE, FX_OP_UNSUPPORTED)
from asyncssh import SFTPServer, SFTPError, SFTPAttrs

from ..abstract import BaseServer
from ..vfs import BaseVFSClient, VFSFileNotFoundError
from ..vfs.vfs_pb2 import Stat


class SFTPUIServer(BaseServer):
    def __init__(self, vfs: BaseVFSClient, host: str='', port: int=8022,
                 host_key: str='host_key', authorized_client_keys: str='client_keys.pub'):
        self.host = host
        self.port = port
        self._vfs = vfs
        self._host_key = host_key
        self._authorized_client_keys = authorized_client_keys

    def start(self):
        loop = asyncio.get_event_loop()
        loop.run_until_complete(self._start_server())
        loop.run_forever()

    async def _start_server(self):
        await asyncssh.listen(self.host, self.port, server_host_keys=[self._host_key],
                              authorized_client_keys=self._authorized_client_keys,
                              sftp_factory=lambda c: ParsecSFTPServer(c, self._vfs))

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


class FileObj(IOBase):
    def __init__(self, vfs, path, seek=0, append=False, read=False, write=False):
        self._vfs = vfs
        self._path = path
        self._offset = seek
        self._append = append
        self._can_read = read
        self._can_write = write
        self._content = None
        self._need_flush = False

    def _get_content(self, force=False):
        if not self._content or force:
            self._content = self._vfs.read_file(self._path).content
        return self._content

    def seek(self, offset):
        self._offset = offset

    def read(self, size):
        if not self._can_read:
            raise OSError("[Errno 9] Bad file descriptor")
        content = self._get_content()
        return content[self._offset:self._offset + size]

    def write(self, data):
        if not self._can_write:
            raise OSError("[Errno 9] Bad file descriptor")
        if self._offset == 0 and not self._append:
            content = b''
        else:
            content = self._get_content()
        offset = len(content) if self._append else self._offset
        self._content = content[:offset] + data
        self._need_flush = True

    def flush(self):
        if self._need_flush:
            self._vfs.write_file(self._path, self._content)
            self._need_flush = False

    def close(self):
        self.flush()


class ParsecSFTPServer(SFTPServer):

    def __init__(self, conn, vfs):
        self._vfs = vfs
        super().__init__(conn)

    def open(self, path, pflags, attrs):
        """Open a file to serve to a remote client

           This method returns a file object which can be used to read
           and write data and get and set file attributes.

           The possible open mode flags and their meanings are:

             ========== ======================================================
             Mode       Description
             ========== ======================================================
             FXF_READ   Open the file for reading. If neither FXF_READ nor
                        FXF_WRITE are set, this is the default.
             FXF_WRITE  Open the file for writing. If both this and FXF_READ
                        are set, open the file for both reading and writing.
             FXF_APPEND Force writes to append data to the end of the file
                        regardless of seek position.
             FXF_CREAT  Create the file if it doesn't exist. Without this,
                        attempts to open a non-existent file will fail.
             FXF_TRUNC  Truncate the file to zero length if it already exists.
             FXF_EXCL   Return an error when trying to open a file which
                        already exists.
             ========== ======================================================

           The attrs argument is used to set initial attributes of the
           file if it needs to be created. Otherwise, this argument is
           ignored.

           :param bytes path:
               The name of the file to open
           :param int pflags:
               The access mode to use for the file (see above)
           :param attrs:
               File attributes to use if the file needs to be created
           :type attrs: :class:`SFTPAttrs`

           :returns: A file object to use to access the file

           :raises: :exc:`SFTPError` to return an error to the client

        """
        append = bool(pflags & FXF_APPEND)
        try:
            response = self._vfs.stat(path)
            if pflags & FXF_EXCL:
                raise FileExistsError("[Errno 17] File exists: '%s'" % path)
        except VFSFileNotFoundError:
            if pflags & FXF_CREAT:
                self._vfs.create_file(path)
                return FileObj(self._vfs, write=True, path=path, append=append)
            else:
                raise FileNotFoundError("[Errno 2] No such file or directory: '%s'" % path)

        seek = 0 if pflags & FXF_TRUNC else response.stat.size

        if pflags & FXF_CREAT and response.stat.type == Stat.DIRECTORY:
            raise IsADirectoryError("[Errno 21] Is a directory: '%s'" % path)

        return FileObj(self._vfs, path=path, seek=seek, append=append,
                       read=pflags & FXF_READ, write=pflags & FXF_WRITE)

    def lstat(self, path):
        """Get attributes of a file, directory, or symlink

           This method queries the attributes of a file, directory,
           or symlink. Unlike :meth:`stat`, this method should
           return the attributes of a symlink itself rather than
           the target of that link.

           :param bytes path:
               The path of the file, directory, or link to get attributes for

           :returns: An :class:`SFTPAttrs` or an os.stat_result containing
                     the file attributes

           :raises: :exc:`SFTPError` to return an error to the client

        """
        try:
            stat = self._vfs.stat(path).stat
        except VFSFileNotFoundError:
            raise SFTPError(FX_NO_SUCH_FILE, 'No such file')
        mod = S_IFDIR if stat.type == Stat.DIRECTORY else S_IFREG
        return SFTPAttrs(**{
            'size': stat.size,
            'uid': os.getuid(),
            'gid': os.getgid(),
            'permissions': mod | S_IRWXU | S_IRWXG | S_IRWXO,
            'atime': stat.atime,
            'mtime': stat.mtime
        })

    def fstat(self, file_obj):
        """Get attributes of an open file

           :param file file_obj:
               The file to get attributes for

           :returns: An :class:`SFTPAttrs` or an os.stat_result containing
                     the file attributes

           :raises: :exc:`SFTPError` to return an error to the client

        """
        return self.lstat(file_obj.path)

    def setstat(self, path, attrs):
        """Set attributes of a file or directory

           This method sets attributes of a file or directory. If
           the path provided is a symbolic link, the attributes
           should be set on the target of the link. A subset of the
           fields in ``attrs`` can be initialized and only those
           attributes should be changed.

           :param bytes path:
               The path of the remote file or directory to set attributes for
           :param attrs:
               File attributes to set
           :type attrs: :class:`SFTPAttrs`

           :raises: :exc:`SFTPError` to return an error to the client

        """
        raise SFTPError(FX_OP_UNSUPPORTED, 'Not implemented')

    def fsetstat(self, file_obj, attrs):
        """Set attributes of an open file

           :param attrs:
               File attributes to set on the file
           :type attrs: :class:`SFTPAttrs`

           :raises: :exc:`SFTPError` to return an error to the client

        """
        raise SFTPError(FX_OP_UNSUPPORTED, 'Not implemented')

    def listdir(self, path):
        """List the contents of a directory

           :param bytes path:
               The path of the directory to open

           :returns: A list of names of files in the directory

           :raises: :exc:`SFTPError` to return an error to the client

        """
        try:
            return [b'.', b'..'] + [d.encode() for d in self._vfs.list_dir(path).list_dir]
        except VFSFileNotFoundError:
            raise SFTPError(FX_NO_SUCH_FILE, 'No such directory')

    def remove(self, path):
        """Remove a file or symbolic link

           :param bytes path:
               The path of the file or link to remove

           :raises: :exc:`SFTPError` to return an error to the client

        """
        try:
            self._vfs.delete_file(path)
        except VFSFileNotFoundError:
            raise SFTPError(FX_NO_SUCH_FILE, 'No such file')

    def mkdir(self, path, attrs):
        """Create a directory with the specified attributes

           :param bytes path:
               The path of where the new directory should be created
           :param attrs:
               The file attributes to use when creating the directory
           :type attrs: :class:`SFTPAttrs`

           :raises: :exc:`SFTPError` to return an error to the client

        """
        self._vfs.make_dir(path)

    def rmdir(self, path):
        """Remove a directory

           :param bytes path:
               The path of the directory to remove

           :raises: :exc:`SFTPError` to return an error to the client

        """
        try:
            self._vfs.remove_dir(path)
        except VFSFileNotFoundError:
            raise SFTPError(FX_NO_SUCH_FILE, 'No such file')

    def realpath(self, path):
        """Return the canonical version of a path

           :param bytes path:
               The path of the directory to canonicalize

           :returns: bytes containing the canonical path

           :raises: :exc:`SFTPError` to return an error to the client

        """
        return path

    def stat(self, path):
        """Get attributes of a file or directory, following symlinks

           This method queries the attributes of a file or directory.
           If the path provided is a symbolic link, the returned
           attributes should correspond to the target of the link.

           :param bytes path:
               The path of the remote file or directory to get attributes for

           :returns: An :class:`SFTPAttrs` or an os.stat_result containing
                     the file attributes

           :raises: :exc:`SFTPError` to return an error to the client

        """
        return self.lstat(path)

    def rename(self, oldpath, newpath):
        """Rename a file, directory, or link

           This method renames a file, directory, or link.

           .. note:: This is a request for the standard SFTP version
                     of rename which will not overwrite the new path
                     if it already exists. The :meth:`posix_rename`
                     method will be called if the client requests the
                     POSIX behavior where an existing instance of the
                     new path is removed before the rename.

           :param bytes oldpath:
               The path of the file, directory, or link to rename
           :param bytes newpath:
               The new name for this file, directory, or link

           :raises: :exc:`SFTPError` to return an error to the client

        """
        raise SFTPError(FX_OP_UNSUPPORTED, 'Not implemented')

    def readlink(self, path):
        """Return the target of a symbolic link

           :param bytes path:
               The path of the symbolic link to follow

           :returns: bytes containing the target path of the link

           :raises: :exc:`SFTPError` to return an error to the client

        """
        raise SFTPError(FX_OP_UNSUPPORTED, 'Not implemented')

    def symlink(self, oldpath, newpath):
        """Create a symbolic link

           :param bytes oldpath:
               The path the link should point to
           :param bytes newpath:
               The path of where to create the symbolic link

           :raises: :exc:`SFTPError` to return an error to the client

        """
        raise SFTPError(FX_OP_UNSUPPORTED, 'Not implemented')

    def posix_rename(self, oldpath, newpath):
        """Rename a file, directory, or link with POSIX semantics

           This method renames a file, directory, or link, removing
           the prior instance of new path if it previously existed.

           :param bytes oldpath:
               The path of the file, directory, or link to rename
           :param bytes newpath:
               The new name for this file, directory, or link

           :raises: :exc:`SFTPError` to return an error to the client

        """
        raise SFTPError(FX_OP_UNSUPPORTED, 'Not implemented')

    def statvfs(self, path):
        """Get attributes of the file system containing a file

           :param bytes path:
               The path of the file system to get attributes for

           :returns: An :class:`SFTPVFSAttrs` or an os.statvfs_result
                     containing the file system attributes

           :raises: :exc:`SFTPError` to return an error to the client

        """
        raise SFTPError(FX_OP_UNSUPPORTED, 'Not implemented')

    def fstatvfs(self, file_obj):
        """Return attributes of the file system containing an open file

           :param file file_obj:
               The open file to get file system attributes for

           :returns: An :class:`SFTPVFSAttrs` or an os.statvfs_result
                     containing the file system attributes

           :raises: :exc:`SFTPError` to return an error to the client

        """
        raise SFTPError(FX_OP_UNSUPPORTED, 'Not implemented')

    def link(self, oldpath, newpath):
        """Create a hard link

           :param bytes oldpath:
               The path of the file the hard link should point to
           :param bytes newpath:
               The path of where to create the hard link

           :raises: :exc:`SFTPError` to return an error to the client

        """
        raise SFTPError(FX_OP_UNSUPPORTED, 'Not implemented')

    def fsync(self, file_obj):
        """Force file data to be written to disk

           :param file file_obj:
               The open file containing the data to flush to disk

           :raises: :exc:`SFTPError` to return an error to the client

        """
        file_obj.flush()

    def exit(self):
        """Shut down this SFTP server"""

        pass

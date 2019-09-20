import os
import socket
from structlog import get_logger
from collections import namedtuple

from PyQt5.QtCore import QObject, pyqtSignal
from PyQt5.QtNetwork import QTcpServer, QHostAddress

from parsec.core.gui.desktop import is_process_running


logger = get_logger()


def get_main_instance_status(pid_file):
    InstanceStatus = namedtuple("InstanceStatus", ("running", "pid", "port"))

    try:
        with open(pid_file) as fd:
            pid, port = [int(x) for x in fd.readline().strip().split(";")]
            logger.debug(f"Info read : pid is {pid}, port is {port}")
    except (ValueError, FileNotFoundError):
        logger.warning("Error while reading the pid file")
        return InstanceStatus(False, None, None)

    if is_process_running(pid):
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            sock.connect(("127.0.0.1", port))
        except socket.error:
            logger.warning("Pid file is present but server is down")
            return InstanceStatus(False, None, None)
        else:
            sock.close()
            return InstanceStatus(True, pid, port)
    else:
        logger.warning("Pid file present but process not running")
        return InstanceStatus(False, None, None)


def send_to_main_instance(port, data):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.connect(("127.0.0.1", port))
        sock.send(data.encode())
        logger.debug("Sending info to main instance")
        sock.close()
        return True
    except socket.error:
        return False


class Daemon(QObject):
    new_instance_required = pyqtSignal(str, str)

    def __init__(self, pid_file):
        super().__init__()
        self.pid_file = pid_file
        self.server = None

    def clean_up(self):
        try:
            os.remove(self.pid_file)
        except FileNotFoundError:
            pass
        if self.server:
            self.server.close()

    def start(self):
        self.server = QTcpServer(self)
        self.server.newConnection.connect(self.on_new_connection)
        port = 1337
        while not self.server.listen(QHostAddress.LocalHost, port):
            port += 1
        logger.debug(f"Server started on port {port}")
        with open(self.pid_file, "w") as fd:
            pid = os.getpid()
            fd.write(f"{pid};{port}")

    def on_new_connection(self):
        sock = self.server.nextPendingConnection()
        if not sock.waitForReadyRead(200):
            logger.debug("Client connected but did not send anything")
            # A new client just figuring out if another instance is running
            return
        # Here we got some data
        data = sock.readAll()
        data_str = data.data().decode()
        cmd, url = data_str.split(" ")
        logger.debug(f"New data received from another instance {cmd} {url}")
        self.new_instance_required.emit(cmd, url)
        sock.close()

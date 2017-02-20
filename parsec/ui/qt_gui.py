import sys
import json
import signal
import pathlib
from os.path import dirname
from PyQt5.QtWidgets import QApplication, QWidget, QSystemTrayIcon, QMenu, QPushButton, QHBoxLayout, QFileDialog, QInputDialog, QLineEdit
from PyQt5.QtGui import QIcon

from ..abstract import BaseServer
from ..crypto import BaseCryptoClient, CryptoError


ICON_PATH = dirname(__file__) + '/../resources/logo.ico'
DRIVE_ICON_PATH = dirname(__file__) + '/../resources/google_drive.ico'
CONFIG_PATH = str(pathlib.Path.home() / '.parsec')


def load_config():
    return json.load(CONFIG_PATH)


def save_config(cfg):
    with open(CONFIG_PATH, 'w') as fd:
        json.dump(cfg, fd)


class SettingsWindow(QWidget):
    def __init__(self, crypto, parent=None):
        super().__init__(parent)
        self.setWindowTitle("Parsec - Settings")
        self.setWindowIcon(QIcon(ICON_PATH))

        self.drive_button = QPushButton(self)
        self.drive_button.setIcon(QIcon(DRIVE_ICON_PATH))
        # self.drive_button.clicked.connect()

        self.crypto = crypto

        self.load_key_button = QPushButton(self)
        self.load_key_button.setText("Load RSA key")

        def load_rsa_key():
            fname = QFileDialog.getOpenFileName(self, 'Select RSA key')
            if not fname[0]:
                return
            text, ok = QInputDialog.getText(self, 'RSA key password', 'Password:', echo=QLineEdit.Password)
            if ok:
                try:
                    with open(fname[0], 'rb') as fd:
                        self.crypto.load_key(pem=fd.read(), passphrase=text.encode('utf-8'))
                except CryptoError:
                    pass

        self.load_key_button.clicked.connect(load_rsa_key)
        # self.load_key_button.clicked.connect(self.file_dialog_rsa_key.show)

        self.layout = QHBoxLayout(self)
        self.layout.addWidget(self.load_key_button)
        self.layout.addWidget(self.drive_button)
        # self.layout.addWidget(self.file_dialog_rsa_key)

    def closeEvent(self, event):
        self.hide()
        event.ignore()

    # @QtCore.pyqtSlot()
    # def on_pushButton_clicked(self):
    #     self.dialogTextBrowser.exec_()


class Systray(QSystemTrayIcon):
    def __init__(self, on_exit, crypto):
        super().__init__(QIcon(ICON_PATH))
        menu = QMenu()
        display_settings_action = menu.addAction("Settings")
        self.settings_window = SettingsWindow(crypto=crypto)
        display_settings_action.triggered.connect(self.settings_window.show)
        exit_action = menu.addAction("Exit")
        exit_action.triggered.connect(on_exit)
        self.setContextMenu(menu)
        self.show()


class QtGUIServer(BaseServer):
    def __init__(self, crypto: BaseCryptoClient):
        self.app = QApplication(sys.argv)
        self.systray = Systray(crypto=crypto, on_exit=self.app.exit)
        self.crypto = crypto

    def start(self):
        # Allow ^C from the Qt application
        signal.signal(signal.SIGINT, signal.SIG_DFL)
        self.app.exec_()

    def stop(self):
        self.app.exit()

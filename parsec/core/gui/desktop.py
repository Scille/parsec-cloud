# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import psutil

from PyQt5.QtCore import QUrl, QFileInfo, QSysInfo, QLocale
from PyQt5.QtGui import QDesktopServices, QGuiApplication, QClipboard

from parsec.api.protocol import DeviceName


def open_file(path):
    QDesktopServices.openUrl(QUrl.fromLocalFile(QFileInfo(path).absoluteFilePath()))


def open_url(url):
    QDesktopServices.openUrl(QUrl(url))


def get_default_device():
    device = QSysInfo.machineHostName()
    if device.lower() == "localhost":
        device = QSysInfo.productType()
    return "".join([c for c in device if DeviceName.regex.match(c)])


def get_locale_language():
    return QLocale.system().name()[:2].lower()


def copy_to_clipboard(text):
    QGuiApplication.clipboard().setText(text, QClipboard.Clipboard)
    QGuiApplication.clipboard().setText(text, QClipboard.Selection)


def is_process_running(pid):
    return psutil.pid_exists(pid)


def parsec_instances_count():
    inst_count = 0
    for proc in psutil.process_iter():
        try:
            if proc.name().lower() in ["parsec", "parsec.exe"] and "backend" not in proc.cmdline():
                inst_count += 1
        except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
            pass
    return inst_count


def add_parsec_protocol_handler():
    with open("/home/max/.local/share/applications/parsec.desktop", "w") as fd:
        fd.write("[Desktop Entry]\n")
        fd.write("Version=1.0\n")
        fd.write("Type=Application\n")
        fd.write("Name=Parsec\n")
        fd.write("Comment=Sovereign Enclave for sharing sensitive data on the cloud\n")
        fd.write("TryExec=snap run parsec\n")
        fd.write("Exec=snap run parsec %F\n")
        fd.write("MimeType=applicatioon/x-parsec;\n")
        fd.write("Actions=Bootstrap;ClaimUser;ClaimDevice\n\n")

        fd.write("[Desktop Action Bootstrap]\n")
        fd.write("Exec=snap run parsec --cmd boostrap\n")
        fd.write("Name=Bootstrap an organisation\n\n")

        fd.write("[Desktop Action ClaimUser]\n")
        fd.write("Exec=snap run parsec --cmd claim-user\n")
        fd.write("Name=Claim a user\n\n")

        fd.write("[Desktop Action ClaimDevice]\n")
        fd.write("Exec=snap run parsec --cmd claim-device\n")
        fd.write("Name=Claim a device\n")


if __name__ == "__main__":
    add_parsec_protocol_handler()

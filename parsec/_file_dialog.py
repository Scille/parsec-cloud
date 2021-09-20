# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

"""
Helper module used by `parsec.core.gui.custom_dialogs.QFileDialogInProcess`
to run the file selection dialog in a subprocess.
"""

import sys
import json


def main():
    from PyQt5.QtWidgets import QApplication, QFileDialog

    # A qapplication needs to be created first
    app = QApplication(sys.argv)  # noqa: F841
    method, args, kwargs = json.load(sys.stdin)
    result = getattr(QFileDialog, method)(None, *args, **kwargs)
    json.dump(result, sys.stdout)


if __name__ == "__main__":
    main()

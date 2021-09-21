# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

"""
Helper module used by `parsec.core.gui.custom_dialogs.QFileDialogInProcess`
to run the file selection dialog in a subprocess.
"""

import sys
import json


def main():
    from PyQt5.QtWidgets import QApplication, QFileDialog

    # For testing purposes
    QFileDialog.getOpenFileName_testing = classmethod(
        lambda cls, parent, arg: (str(arg), "<getOpenFileName>")
    )
    QFileDialog.getOpenFileNames_testing = classmethod(
        lambda cls, parent, arg: ([str(arg)], "<getOpenFileNames>")
    )
    QFileDialog.getSaveFileName_testing = classmethod(
        lambda cls, parent, arg: (str(arg), "<getSaveFileName>")
    )
    QFileDialog.getExistingDirectory_testing = classmethod(
        lambda cls, parent, arg: str(arg) + "/getExistingDirectory"
    )

    # A qapplication needs to be created first
    app = QApplication(sys.argv)  # noqa: F841
    method, args, kwargs = json.load(sys.stdin)
    method += kwargs.pop("test_suffix", "")
    result = getattr(QFileDialog, method)(None, *args, **kwargs)
    json.dump(result, sys.stdout)


if __name__ == "__main__":
    main()

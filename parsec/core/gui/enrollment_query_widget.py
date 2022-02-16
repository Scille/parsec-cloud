# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtWidgets import QWidget

from parsec.core.gui.custom_dialogs import GreyedDialog
from parsec.core.gui.lang import translate

from parsec.core.gui.ui.enrollment_query_widget import Ui_EnrollmentQueryWidget


class EnrollmentQueryWidget(QWidget, Ui_EnrollmentQueryWidget):
    def __init__(self, jobs_ctx, config, addr):
        super().__init__()
        self.setupUi(self)
        self.status = False
        self.jobs_ctx = jobs_ctx
        self.config = config
        self.dialog = None
        self.addr = addr
        self.label_cert_error.setVisible(False)
        self.widget_user_info.setVisible(False)
        self.count = 0
        self.button_select_cert.clicked.connect(self._on_select_cert_clicked)
        self.button_ask_to_join.clicked.connect(self._on_ask_to_join_clicked)

    def _on_ask_to_join_clicked(self):
        self.dialog.accept()

    def _on_select_cert_clicked(self):
        if self.count % 2 == 0:
            self.label_cert_error.setText("This certificate is invalid.")
            self.label_cert_error.setVisible(True)
            self.widget_user_info.setVisible(False)
            self.button_ask_to_join.setEnabled(False)
        else:
            self.label_cert_error.setVisible(False)
            self.line_edit_user_name.setText("Hubert Farnsworth")
            self.line_edit_user_email.setText("hubert.farnsworth@planetexpress.com")
            self.widget_user_info.setVisible(True)
            self.button_ask_to_join.setEnabled(True)
        self.count += 1

    @classmethod
    def show_modal(cls, jobs_ctx, config, addr, parent, on_finished):
        w = cls(jobs_ctx=jobs_ctx, config=config, addr=addr)
        d = GreyedDialog(w, translate("TEXT_ENROLLMENT_QUERY_TITLE"), parent=parent, width=800)
        w.dialog = d

        def _on_finished(result):
            return on_finished()

        d.finished.connect(_on_finished)

        # Unlike exec_, show is asynchronous and works within the main Qt loop
        d.show()
        return w

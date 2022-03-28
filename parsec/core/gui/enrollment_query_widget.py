# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import uuid

from PyQt5.QtWidgets import QWidget

from parsec.crypto import PrivateKey, SigningKey

from parsec.api.protocol.pki import pki_enrollment_request_serializer

from parsec.core.gui.custom_dialogs import GreyedDialog, show_error
from parsec.core.gui.lang import translate
from parsec.core.gui import desktop

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
        self.url = None
        self.enrollment_request = None
        self.certificated_id = None
        self.request_id = None
        self.local_request = None
        self.certificate_sha1 = None
        self.label_cert_error.setVisible(False)
        self.widget_user_info.setVisible(False)
        self.button_ask_to_join.setEnabled(False)
        self.button_select_cert.clicked.connect(self._on_select_cert_clicked)
        self.button_ask_to_join.clicked.connect(self._on_ask_to_join_clicked)

    def _on_ask_to_join_clicked(self):
        self.jobs_ctx.submit_job(None, None, self.make_enrollment_request)

    async def make_enrollment_request(self):
        self.button_ask_to_join.setEnabled(False)

        async def _http_request(url, method="GET", data=None):
            from urllib.request import Request, urlopen
            import trio

            def _do_req():
                req = Request(url=url, method=method, data=data)
                with urlopen(req) as rep:
                    return rep.read()

            return await trio.to_thread.run_sync(_do_req)

        data = pki_enrollment_request_serializer.req_dumps(
            {
                "request": self.enrollment_request,
                "certificate_id": self.certificate_sha1,
                "request_id": self.request_id,
                "force_flag": True,
            }
        )

        await self.local_request.save(self.config.config_dir)
        rep_data = await _http_request(self.url, method="POST", data=data)
        rep = pki_enrollment_request_serializer.rep_loads(rep_data)

        if rep["status"] != "ok":
            await self.local_request.get_path(self.config.config_dir).unlink()
            self.button_ask_to_join.setEnabled(True)
            show_error(self, translate("TEXT_ENROLLMENT_QUERY_FAILED"))
        else:
            self.status = True
            self.dialog.accept()

    async def prepare_enrollment_request(self):
        from parsec_ext import smartcard

        self.enrollment_request = None
        self.certificated_id = None
        self.request_id = None
        self.local_request = None
        self.certificate_sha1 = None
        self.url = None

        self.request_id = uuid.uuid4()
        private_key = PrivateKey.generate()
        signing_key = SigningKey.generate()

        organization_id = self.addr.organization_id
        backend_address = self.addr.generate_backend_addr()
        self.url = self.addr.to_http_domain_url(
            f"/anonymous/pki/{organization_id}/enrollment_request"
        )
        try:
            self.enrollment_request, self.local_request, self.certificate_id, self.certificate_sha1 = await smartcard.prepare_enrollment_request(
                self.request_id,
                private_key,
                signing_key,
                desktop.get_default_device(),
                backend_address,
                organization_id,
            )
            self.widget_user_info.setVisible(True)
            self.label_cert_error.setVisible(True)
            self.line_edit_user_name.setText(self.local_request.human_handle.label)
            self.line_edit_user_email.setText(self.local_request.human_handle.email)
            self.button_ask_to_join.setEnabled(True)
        except:
            import traceback

            traceback.print_exc()
            self.widget_user_info.setVisible(False)
            self.label_cert_error.setText(translate("TEXT_ENROLLMENT_ERROR_LOADING_CERTIFICATE"))
            self.label_cert_error.setVisible(True)
            self.button_ask_to_join.setEnabled(False)

    def _on_select_cert_clicked(self):
        self.jobs_ctx.submit_job(None, None, self.prepare_enrollment_request)

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

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from functools import partial
from structlog import get_logger
from PyQt5.QtCore import QCoreApplication, pyqtSignal, pyqtSlot
from PyQt5.QtWidgets import QMainWindow

from parsec import __version__ as PARSEC_VERSION

from parsec.core.local_device import (
    LocalDeviceError,
    load_device_with_password,
    load_device_with_pkcs11,
)
from parsec.core.config import save_config
from parsec.core.mountpoint import MountpointConfigurationError, MountpointDriverCrash
from parsec.core.backend_connection import BackendHandshakeError, BackendDeviceRevokedError
from parsec.core import logged_core_factory
from parsec.core.gui.lang import translate as _
from parsec.core.gui import telemetry
from parsec.core.gui.trio_thread import QtToTrioJobScheduler, ThreadSafeQtSignal
from parsec.core.gui.login_widget import LoginWidget
from parsec.core.gui.central_widget import CentralWidget
from parsec.core.gui.custom_widgets import ask_question, show_error
from parsec.core.gui.starting_guide_dialog import StartingGuideDialog
from parsec.core.gui.ui.main_window import Ui_MainWindow


logger = get_logger()


async def _do_run_core(config, device, event_bus, qt_on_ready):
    async with logged_core_factory(config=config, device=device, event_bus=event_bus) as core:
        if config.mountpoint_enabled:
            await core.mountpoint_manager.mount_all()
        # Create our own job scheduler allows us to cancel all pending
        # jobs depending on us when we logout
        core_jobs_ctx = QtToTrioJobScheduler()
        async with trio.open_nursery() as nursery:
            await nursery.start(core_jobs_ctx._start)
            qt_on_ready.emit(core, core_jobs_ctx)


class MainWindow(QMainWindow, Ui_MainWindow):
    run_core_success = pyqtSignal()
    run_core_error = pyqtSignal()
    run_core_ready = pyqtSignal(object, object)
    logged_in = pyqtSignal()
    logged_out = pyqtSignal()

    def __init__(self, jobs_ctx, event_bus, config, minimize_on_close: bool = False, **kwargs):
        super().__init__(**kwargs)
        self.setupUi(self)

        self.jobs_ctx = jobs_ctx
        self.event_bus = event_bus
        self.config = config
        self.minimize_on_close = minimize_on_close
        self.force_close = False
        self.need_close = False

        self.core = None
        self.core_jobs_ctx = None
        self.runing_core_job = None

        self.run_core_success.connect(self.on_core_run_done)
        self.run_core_error.connect(self.on_core_run_done)
        self.run_core_ready.connect(self.on_run_core_ready)

        self.login_widget = LoginWidget(jobs_ctx, config, parent=self)
        self.widget_center.layout().addWidget(self.login_widget)
        self.central_widget = CentralWidget(jobs_ctx, event_bus, config, parent=self)
        self.widget_center.layout().addWidget(self.central_widget)

        self.setWindowTitle(
            _(
                "Parsec - Community Edition - {} - Sovereign enclave for sharing "
                "sensitive data on the cloud"
            ).format(PARSEC_VERSION)
        )

        self.central_widget.logout_requested.connect(self.logout)
        self.login_widget.login_with_password_clicked.connect(self.login_with_password)
        self.login_widget.login_with_pkcs11_clicked.connect(self.login_with_pkcs11)

        self.show_login_widget()

    def show_starting_guide(self):
        s = StartingGuideDialog(parent=self)
        s.exec_()

    def showMaximized(self):
        super().showMaximized()
        QCoreApplication.processEvents()
        if self.config.gui_first_launch:
            self.show_starting_guide()
            r = ask_question(
                self,
                _("Error reporting"),
                _(
                    "Do you authorize Parsec to send data when it encounters an error to help "
                    "us improve your experience ?"
                ),
            )
            self.config = self.config.evolve(gui_first_launch=False, telemetry_enabled=r)
            # TODO: send gui.config.changed event instead
            save_config(self.config)
        self.login_widget.core_config = self.config
        telemetry.init(self.config)

    def show_top(self):
        self.show()
        self.raise_()

    @pyqtSlot(object, object)
    def _core_ready(self, core, core_jobs_ctx):
        self.run_core_ready.emit(core, core_jobs_ctx)

    def start_core(self, device):
        assert not self.runing_core_job
        assert not self.core
        assert not self.core_jobs_ctx

        self.runing_core_job = self.jobs_ctx.submit_job(
            ThreadSafeQtSignal(self, "run_core_success"),
            ThreadSafeQtSignal(self, "run_core_error"),
            _do_run_core,
            self.config,
            device,
            self.event_bus,
            ThreadSafeQtSignal(self, "run_core_ready", object, object),
        )

    def on_run_core_ready(self, core, core_jobs_ctx):
        self.core = core
        self.core_jobs_ctx = core_jobs_ctx
        self.central_widget.set_core_attributes(core=core, portal=core_jobs_ctx)
        self.config = self.config.evolve(
            gui_last_device="{}:{}".format(
                self.core.device.organization_addr.organization_id, self.core.device.device_id
            )
        )
        # TODO: send gui.config.changed event instead
        save_config(self.config)
        self.logged_in.emit()

    def on_core_run_done(self):
        assert self.runing_core_job.is_finished()
        self.runing_core_job = None
        self.core_jobs_ctx = None
        self.core = None

        self.central_widget.set_core_attributes(None, None)
        self.logged_out.emit()

    def stop_core(self):
        if self.runing_core_job:
            self.runing_core_job.cancel_and_join()

    def logout(self):
        self.stop_core()
        self.show_login_widget()

    def login_with_password(self, key_file, password):
        try:
            device = load_device_with_password(key_file, password)
            self.start_core(device)
            self.show_central_widget()

        except LocalDeviceError:
            show_error(self, _("Authentication failed."))

        except BackendDeviceRevokedError:
            show_error(self, _("This device has been revoked."))

        except BackendHandshakeError:
            show_error(self, _("User not registered in the backend."))

        except (RuntimeError, MountpointConfigurationError, MountpointDriverCrash):
            show_error(self, _("Mountpoint already in use."))

        except Exception as exc:
            import traceback

            traceback.print_exc()
            show_error(self, _("Can not login"))
            logger.error("Error while trying to log in: {}".format(str(exc)))

    def login_with_pkcs11(self, key_file, pkcs11_pin, pkcs11_key, pkcs11_token):
        try:
            device = load_device_with_pkcs11(
                key_file, token_id=pkcs11_token, key_id=pkcs11_key, pin=pkcs11_pin
            )
            self.start_core(device)
            self.show_central_widget()

        except LocalDeviceError:
            show_error(self, _("Authentication failed."))

        except BackendDeviceRevokedError:
            show_error(self, _("This device has been revoked."))

        except BackendHandshakeError:
            show_error(self, _("User not registered in the backend."))

        except (RuntimeError, MountpointConfigurationError, MountpointDriverCrash):
            show_error(self, _("Mountpoint already in use."))

        except Exception as exc:
            import traceback

            traceback.print_exc()
            show_error(self, _("Can not login."))
            logger.error("Error while trying to log in: {}".format(str(exc)))

    def close_app(self, force=False):
        self.need_close = True
        self.force_close = force
        self.close()

    def closeEvent(self, event):
        if self.minimize_on_close and not self.need_close:
            self.hide()
            event.ignore()

        else:
            if self.config.gui_confirmation_before_close and not self.force_close:
                result = ask_question(self, _("Confirmation"), _("Are you sure you want to quit ?"))
                if not result:
                    event.ignore()
                    return
            msg = _("Parsec is still running.")
            self.jobs_ctx.run_sync(
                partial(self.event_bus.send, "gui.systray.notif", title="Parsec", msg=msg)
            )
            self.stop_core()
            event.accept()

    def show_central_widget(self):
        self._hide_all_widgets()
        self.central_widget.show()
        self.central_widget.reset()

    def show_login_widget(self):
        self._hide_all_widgets()
        self.login_widget.reset()
        self.login_widget.show()

    def _hide_all_widgets(self):
        self.login_widget.hide()
        self.central_widget.hide()

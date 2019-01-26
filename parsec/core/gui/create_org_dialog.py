import os
import pendulum

from PyQt5.QtWidgets import QDialog

import trio

from parsec.backend.config import DEFAULT_ADMINISTRATOR_TOKEN
from parsec.core.backend_connection import (
    backend_administrator_cmds_factory,
    backend_anonymous_cmds_factory,
)
from parsec.types import BackendOrganizationBootstrapAddr, DeviceID, BackendAddr
from parsec.crypto import SigningKey
from parsec.trustchain import certify_user, certify_device
from parsec.core.config import get_default_config_dir
from parsec.core.devices_manager import generate_new_device, save_device_with_password

from parsec.core.gui.custom_widgets import show_error, show_info
from parsec.core.gui.ui.create_org_dialog import Ui_CreateOrgDialog


class CreateOrgDialog(QDialog, Ui_CreateOrgDialog):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setupUi(self)
        self.button_create_org.clicked.connect(self.create_organization)

    def create_organization(self):
        async def _create_org(name, backend_addr, admin_token):
            try:
                async with backend_administrator_cmds_factory(backend_addr, admin_token) as cmds:
                    bootstrap_token = await cmds.organization_create(name)
                org_addr = BackendOrganizationBootstrapAddr.build(
                    backend_addr, name, bootstrap_token
                )
                return org_addr, True
            except:
                return None, False

        async def _bootstrap_org(device_id, org_addr, config_dir, password):
            try:
                root_signing_key = SigningKey.generate()
                root_verify_key = root_signing_key.verify_key
                organization_addr = org_addr.generate_organization_addr(root_verify_key)

                device = generate_new_device(device_id, organization_addr)

                save_device_with_password(config_dir, device, password)

                now = pendulum.now()
                certified_user = certify_user(
                    None, root_signing_key, device.user_id, device.public_key, now
                )
                certified_device = certify_device(
                    None, root_signing_key, device_id, device.verify_key, now
                )

                async with backend_anonymous_cmds_factory(org_addr) as cmds:
                    await cmds.organization_bootstrap(
                        org_addr.organization_id,
                        org_addr.bootstrap_token,
                        root_verify_key,
                        certified_user,
                        certified_device,
                    )
                return True
            except:
                return False

        org_addr, status = trio.run(
            _create_org,
            self.line_edit_org_name.text(),
            BackendAddr(self.line_edit_backend_addr.text()),
            DEFAULT_ADMINISTRATOR_TOKEN,
        )
        if not status:
            show_error(self, "Error while creating the organization.")
            return

        device_id = DeviceID(
            "{}@{}".format(self.line_edit_username.text(), self.line_edit_device.text())
        )
        config_dir = get_default_config_dir(os.environ)
        status = trio.run(
            _bootstrap_org, device_id, org_addr, config_dir, self.line_edit_password.text()
        )
        if not status:
            show_error(self, "Error while creating the organization.")
            return
        show_info(self, "Organization has been created.")
        self.accept()

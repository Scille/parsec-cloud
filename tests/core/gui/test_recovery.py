# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from PyQt5 import QtCore

from parsec.core.local_device import save_device_with_password_in_config
from parsec.core.recovery import (
    create_new_device_from_original,
    generate_recovery_device_name,
    generate_recovery_password,
    generate_passphrase_from_recovery_password,
)

from parsec.core.gui.lang import translate
from parsec.core.gui.device_recovery_export_widget import (
    DeviceRecoveryExportPage1Widget,
    DeviceRecoveryExportPage2Widget,
)
from parsec.core.gui.device_recovery_import_widget import (
    DeviceRecoveryImportPage1Widget,
    DeviceRecoveryImportPage2Widget,
)


@pytest.fixture
def catch_import_recovery_widget(widget_catcher_factory):
    return widget_catcher_factory(
        "parsec.core.gui.device_recovery_import_widget.DeviceRecoveryImportWidget"
    )


@pytest.fixture
def catch_export_recovery_widget(widget_catcher_factory):
    return widget_catcher_factory(
        "parsec.core.gui.device_recovery_export_widget.DeviceRecoveryExportWidget"
    )


@pytest.mark.gui
@pytest.mark.trio
async def test_export_recovery_device(
    gui,
    aqtbot,
    monkeypatch,
    core_config,
    running_backend,
    catch_export_recovery_widget,
    tmp_path,
    alice,
):
    PASSWORD = "P@ssw0rd"
    RECOVERY_KEY_FILE_NAME = "alice.psrk"
    save_device_with_password_in_config(core_config.config_dir, alice, PASSWORD)

    with monkeypatch.context() as m:
        m.setattr(
            "parsec.core.gui.main_window.ask_question",
            lambda *args, **kwargs: translate("ACTION_CREATE_RECOVERY_DEVICE"),
        )
        aqtbot.key_click(gui, "i", QtCore.Qt.ControlModifier, 200)

    exp_w = await catch_export_recovery_widget()
    assert exp_w

    assert isinstance(exp_w.current_page, DeviceRecoveryExportPage1Widget)
    assert not exp_w.button_validate.isEnabled()

    assert exp_w.current_page.combo_devices.count() == 1
    assert exp_w.current_page.combo_devices.currentData() == alice.slug

    exp_w.current_page.label_file_path.setText(str(tmp_path / RECOVERY_KEY_FILE_NAME))

    assert not exp_w.button_validate.isEnabled()

    aqtbot.key_clicks(exp_w.current_page.edit_password, PASSWORD)

    assert exp_w.button_validate.isEnabled()

    async with aqtbot.wait_signal(exp_w.export_success):
        aqtbot.mouse_click(exp_w.button_validate, QtCore.Qt.LeftButton)

    def _page2_shown():
        assert isinstance(exp_w.current_page, DeviceRecoveryExportPage2Widget)

    await aqtbot.wait_until(_page2_shown)

    assert (tmp_path / RECOVERY_KEY_FILE_NAME).is_file()
    assert (tmp_path / RECOVERY_KEY_FILE_NAME).stat().st_size > 0

    assert len(exp_w.current_page.edit_passphrase.toPlainText()) > 0
    assert exp_w.current_page.label_file_path.text() == str(tmp_path / RECOVERY_KEY_FILE_NAME)

    assert exp_w.button_validate.isEnabled()
    aqtbot.mouse_click(exp_w.button_validate, QtCore.Qt.LeftButton)


@pytest.mark.gui
@pytest.mark.trio
async def test_import_recovery_device(
    gui,
    aqtbot,
    monkeypatch,
    core_config,
    running_backend,
    catch_import_recovery_widget,
    autoclose_dialog,
    tmp_path,
    alice,
):
    PASSWORD = "P@ssw0rd"
    NEW_DEVICE_LABEL = "Alice_New_Device"
    RECOVERY_KEY_FILE_NAME = "alice.psrk"
    save_device_with_password_in_config(core_config.config_dir, alice, PASSWORD)

    device_label = generate_recovery_device_name()
    recovery_password = generate_recovery_password()

    await create_new_device_from_original(
        alice,
        device_label,
        recovery_password,
        core_config.config_dir,
        tmp_path / RECOVERY_KEY_FILE_NAME,
    )
    with monkeypatch.context() as m:
        m.setattr(
            "parsec.core.gui.main_window.ask_question",
            lambda *args, **kwargs: translate("ACTION_RECOVER_DEVICE"),
        )
        aqtbot.key_click(gui, "i", QtCore.Qt.ControlModifier, 200)

    imp_w = await catch_import_recovery_widget()
    assert imp_w
    assert isinstance(imp_w.current_page, DeviceRecoveryImportPage1Widget)
    assert not imp_w.button_validate.isEnabled()

    imp_w.current_page.label_key_file.setText(str(tmp_path / RECOVERY_KEY_FILE_NAME))
    assert not imp_w.button_validate.isEnabled()

    aqtbot.key_clicks(imp_w.current_page.edit_passphrase, "abcdef")
    assert not imp_w.button_validate.isEnabled()
    assert imp_w.current_page.label_passphrase_error.text() == translate(
        "TEXT_RECOVERY_INVALID_PASSPHRASE"
    )
    assert imp_w.current_page.label_passphrase_error.isVisible()

    passphrase = generate_passphrase_from_recovery_password(recovery_password)

    imp_w.current_page.edit_passphrase.setText("")
    aqtbot.key_clicks(imp_w.current_page.edit_passphrase, passphrase)

    assert imp_w.button_validate.isEnabled()
    assert not imp_w.current_page.label_passphrase_error.isVisible()

    aqtbot.mouse_click(imp_w.button_validate, QtCore.Qt.LeftButton)

    def _page2_shown():
        assert isinstance(imp_w.current_page, DeviceRecoveryImportPage2Widget)

    await aqtbot.wait_until(_page2_shown)

    assert not imp_w.button_validate.isEnabled()

    imp_w.current_page.line_edit_device.setText("")
    aqtbot.key_clicks(imp_w.current_page.line_edit_device, NEW_DEVICE_LABEL)
    assert not imp_w.button_validate.isEnabled()
    aqtbot.key_clicks(imp_w.current_page.widget_password.line_edit_password, PASSWORD)
    assert not imp_w.button_validate.isEnabled()
    aqtbot.key_clicks(imp_w.current_page.widget_password.line_edit_password_check, PASSWORD)
    assert imp_w.button_validate.isEnabled()

    async with aqtbot.wait_signal(imp_w.create_new_device_success):
        aqtbot.mouse_click(imp_w.button_validate, QtCore.Qt.LeftButton)

    assert autoclose_dialog.dialogs == [("", translate("TEXT_RECOVERY_IMPORT_SUCCESS"))]

    lw = gui.test_get_login_widget()

    accounts_w = lw.widget.layout().itemAt(0).widget()
    assert accounts_w.accounts_widget.layout().count() == 3

    assert any(
        accounts_w.accounts_widget.layout().itemAt(i).widget() is not None
        and accounts_w.accounts_widget.layout().itemAt(i).widget().label_device.text()
        == NEW_DEVICE_LABEL
        for i in range(accounts_w.accounts_widget.layout().count())
    )

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from PyQt5 import QtCore

from pathlib import Path

from parsec.core.local_device import (
    save_device_with_password_in_config,
    get_recovery_device_file_name,
    save_recovery_device,
)
from parsec.core.recovery import generate_recovery_device
from parsec.api.protocol import DeviceLabel

from parsec.core.gui.lang import translate
from parsec.core.gui.device_recovery_export_widget import (
    DeviceRecoveryExportPage1Widget,
    DeviceRecoveryExportPage2Widget,
)
from parsec.core.gui.device_recovery_import_widget import DeviceRecoveryImportPage1Widget
from parsec.core.gui.authentication_choice_widget import AuthenticationChoiceWidget


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
@pytest.mark.parametrize("kind", ["ok", "offline", "already_exists"])
async def test_export_recovery_device(
    gui,
    aqtbot,
    monkeypatch,
    core_config,
    running_backend,
    autoclose_dialog,
    catch_export_recovery_widget,
    tmp_path,
    alice,
    kind,
):
    PASSWORD = "P@ssw0rd"
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

    exp_w.current_page.label_file_path.setText(str(tmp_path))

    exp_w.current_page._check_infos()

    monkeypatch.setattr(
        "parsec.core.gui.device_recovery_export_widget.get_text_input",
        lambda *args, **kwargs: PASSWORD,
    )

    if kind == "ok":
        aqtbot.mouse_click(exp_w.button_validate, QtCore.Qt.LeftButton)

        def _page2_shown():
            assert isinstance(exp_w.current_page, DeviceRecoveryExportPage2Widget)

        await aqtbot.wait_until(_page2_shown)

        recovery_file = Path(exp_w.current_page.label_file_path.text())

        assert recovery_file.is_file()
        assert recovery_file.stat().st_size > 0

        assert len(exp_w.current_page.edit_passphrase.toPlainText()) > 0

        assert exp_w.button_validate.isEnabled()
        aqtbot.mouse_click(exp_w.button_validate, QtCore.Qt.LeftButton)

    elif kind == "offline":
        with running_backend.offline():
            aqtbot.mouse_click(exp_w.button_validate, QtCore.Qt.LeftButton)

            def _error_shown():
                assert autoclose_dialog.dialogs == [
                    ("Error", translate("EXPORT_KEY_BACKEND_OFFLINE"))
                ]

            await aqtbot.wait_until(_error_shown)

    elif kind == "already_exists":
        file_name = get_recovery_device_file_name(alice)
        (tmp_path / file_name).touch()

        aqtbot.mouse_click(exp_w.button_validate, QtCore.Qt.LeftButton)

        def _error_shown():
            assert autoclose_dialog.dialogs == [
                ("Error", translate("TEXT_RECOVERY_DEVICE_FILE_ALREADY_EXISTS"))
            ]

        await aqtbot.wait_until(_error_shown)


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.parametrize("kind", ["ok", "offline"])
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
    kind,
):
    PASSWORD = "P@ssw0rd"
    NEW_DEVICE_LABEL = DeviceLabel("Alice_New_Device")
    save_device_with_password_in_config(core_config.config_dir, alice, PASSWORD)

    recovery_device = await generate_recovery_device(alice)
    file_name = get_recovery_device_file_name(recovery_device)
    file_path = tmp_path / file_name
    passphrase = await save_recovery_device(file_path, recovery_device)

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

    imp_w.current_page.line_edit_device.setText("")

    imp_w.current_page.label_key_file.setText(str(file_path))
    assert not imp_w.button_validate.isEnabled()

    await aqtbot.key_clicks(imp_w.current_page.edit_passphrase, "abcdef")
    assert not imp_w.button_validate.isEnabled()
    await aqtbot.wait_until(
        lambda: imp_w.current_page.label_passphrase_error.text()
        == translate("TEXT_RECOVERY_INVALID_PASSPHRASE")
    )
    assert imp_w.current_page.label_passphrase_error.isVisible()

    imp_w.current_page.edit_passphrase.setText("")
    await aqtbot.key_clicks(imp_w.current_page.edit_passphrase, passphrase)
    await aqtbot.wait_until(lambda: not imp_w.current_page.label_passphrase_error.isVisible())
    assert not imp_w.button_validate.isEnabled()

    await aqtbot.key_clicks(imp_w.current_page.line_edit_device, NEW_DEVICE_LABEL.str)
    await aqtbot.wait_until(imp_w.button_validate.isEnabled)

    if kind == "ok":

        async with aqtbot.wait_signal(imp_w.create_new_device_success):
            aqtbot.mouse_click(imp_w.button_validate, QtCore.Qt.LeftButton)

        def _page2_shown():
            assert isinstance(imp_w.current_page, AuthenticationChoiceWidget)

        await aqtbot.wait_until(_page2_shown)

        assert not imp_w.button_validate.isEnabled()
        await aqtbot.key_clicks(
            imp_w.current_page.main_layout.itemAt(0).widget().line_edit_password, PASSWORD
        )
        assert not imp_w.button_validate.isEnabled()
        await aqtbot.key_clicks(
            imp_w.current_page.main_layout.itemAt(0).widget().line_edit_password_check, PASSWORD
        )
        await aqtbot.wait_until(imp_w.button_validate.isEnabled)

        aqtbot.mouse_click(imp_w.button_validate, QtCore.Qt.LeftButton)
        await aqtbot.wait_until(
            lambda: autoclose_dialog.dialogs == [("", translate("TEXT_RECOVERY_IMPORT_SUCCESS"))]
        )

        lw = gui.test_get_login_widget()

        def _devices_listed():
            assert lw.widget.layout().count() > 0

        await aqtbot.wait_until(_devices_listed)

        accounts_w = lw.widget.layout().itemAt(0).widget()
        assert accounts_w.accounts_widget.layout().count() == 3

        assert any(
            accounts_w.accounts_widget.layout().itemAt(i).widget() is not None
            and accounts_w.accounts_widget.layout().itemAt(i).widget().label_device.text()
            == NEW_DEVICE_LABEL.str
            for i in range(accounts_w.accounts_widget.layout().count())
        )

    elif kind == "offline":

        with running_backend.offline():
            aqtbot.mouse_click(imp_w.button_validate, QtCore.Qt.LeftButton)

            def _error_shown():
                assert autoclose_dialog.dialogs == [
                    ("Error", translate("IMPORT_KEY_BACKEND_OFFLINE"))
                ]

            await aqtbot.wait_until(_error_shown)

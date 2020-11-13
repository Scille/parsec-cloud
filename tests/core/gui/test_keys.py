# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from parsec.core.local_device import save_device_with_password
from parsec.core.gui.keys_widget import KeysWidget, KeyWidget
from parsec.core.local_device import AvailableDevice

from PyQt5 import QtCore


@pytest.fixture
def keys_widget(qtbot, core_config, alice, bob):
    password = "123_is_not_a_valid_password"
    save_device_with_password(core_config.config_dir, alice, password)
    save_device_with_password(core_config.config_dir, bob, password)

    w = KeysWidget(core_config, parent=None)
    qtbot.addWidget(w)
    assert w.scroll_content.layout().count() == 2
    return w


@pytest.mark.gui
def test_keys_list_devices(core_config, alice, bob, keys_widget):
    keys_layout = keys_widget.scroll_content.layout()

    users = [alice, bob]
    for i in range(keys_layout.count()):
        key_w = keys_layout.itemAt(i).widget()
        assert isinstance(key_w, KeyWidget)
        assert isinstance(key_w.device, AvailableDevice)
        assert key_w.label_org.text() in [user.organization_id for user in users]
        assert key_w.label_user.text() in [user.human_handle.label for user in users]
        assert key_w.label_device.text() in [user.device_label for user in users]


@pytest.mark.gui
def test_keys_export(qtbot, core_config, alice, bob, monkeypatch, keys_widget):
    keys_layout = keys_widget.scroll_content.layout()

    tmp_path = core_config.config_dir.joinpath("tmp")
    tmp_path.mkdir()
    monkeypatch.setattr(
        "parsec.core.gui.keys_widget.QFileDialog.getExistingDirectory", lambda x: tmp_path
    )

    key_w = keys_layout.itemAt(0).widget()
    qtbot.mouseClick(key_w.export_button, QtCore.Qt.LeftButton)

    def key_exported():
        assert len(list(tmp_path.glob("*.keys"))) == 1

    qtbot.wait_until(key_exported)


@pytest.mark.gui
def test_keys_import(qtbot, core_config, alice, bob, monkeypatch):
    password = "123_is_not_a_valid_password"
    save_device_with_password(core_config.config_dir, alice, password)

    tmp_path = core_config.config_dir.joinpath("tmp")
    tmp_path.mkdir()
    tmp_path.joinpath("devices").mkdir()

    fake_config = type("fake_config", (), {"config_dir": tmp_path})()

    w = KeysWidget(fake_config, parent=None)
    qtbot.addWidget(w)

    keys_layout = w.scroll_content.layout()
    assert keys_layout.count() == 0

    key_glob = list(core_config.config_dir.joinpath("devices").glob("*.keys"))
    assert len(key_glob) == 1

    monkeypatch.setattr(
        "parsec.core.gui.keys_widget.QFileDialog.getOpenFileName",
        lambda *x, **y: (key_glob[0], None),
    )
    monkeypatch.setattr("parsec.core.gui.keys_widget.ask_question", lambda *x, **y: "ACTION_YES")
    qtbot.mouseClick(w.button_import_key, QtCore.Qt.LeftButton)

    def key_imported():
        assert keys_layout.count() == 1

    qtbot.wait_until(key_imported)

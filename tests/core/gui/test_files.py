# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import os
from pathlib import Path

import pytest
import trio
from PyQt5 import QtCore, QtGui, QtWidgets
from PyQt5.QtGui import QGuiApplication

from parsec._parsec import ChunkID, EntryID, EntryName
from parsec.core.fs import FsPath
from parsec.core.fs.workspacefs.sync_transactions import DEFAULT_BLOCK_SIZE
from parsec.core.gui.file_items import TYPE_DATA_INDEX, FileType
from parsec.core.gui.file_size import get_filesize
from parsec.core.gui.file_table import Column
from parsec.core.gui.files_widget import FilesWidget
from parsec.core.gui.lang import translate as _
from parsec.core.gui.main_window import MainWindow
from parsec.core.types import AnyLocalManifest, WorkspaceRole
from parsec.test_utils import create_inconsistent_workspace


def create_files_widget_testbed(
    monkeypatch: pytest.MonkeyPatch, aqtbot, logged_gui, user_fs, wfs, f_w: FilesWidget, c_w
):
    # === Testbed class is full of helpers ===
    class FilesWidgetTestbed:
        def __init__(self):
            self.logged_gui = logged_gui
            self.user_fs = user_fs
            self.workspace_fs = wfs
            self.files_widget = f_w

        def pwd(self):
            return str(c_w.navigation_bar_widget.get_current_path())

        def ls(self):
            items = []
            for i in range(1, f_w.table_files.rowCount()):
                table_item = f_w.table_files.item(i, 1)
                if table_item.data(TYPE_DATA_INDEX) == FileType.Folder:
                    items.append(table_item.text() + "/")
                elif table_item.data(TYPE_DATA_INDEX) == FileType.Inconsistency:
                    items.append(table_item.text() + "!")
                else:
                    items.append(table_item.text())
            return items

        async def cd(self, path):
            current_path_parts = [p for p in c_w.navigation_bar_widget.paths]
            if path.startswith("/"):
                raise ValueError("Absolute path not supported")
            for name in path.split("/"):
                if name == "." or name == "":
                    continue
                elif name == "..":
                    if not current_path_parts:
                        raise ValueError("Root folder already reached, cannot go up anymore !")
                    f_w.table_files.item_activated.emit(FileType.ParentFolder, "Parent folder")
                    current_path_parts.pop()
                else:
                    f_w.table_files.item_activated.emit(FileType.Folder, name)
                    current_path_parts.append(EntryName(name))

                def _path_reached():
                    fs_path = FsPath("/" + "/".join([part.str for part in current_path_parts]))
                    assert f_w.current_directory == fs_path
                    assert c_w.navigation_bar_widget.get_current_path() == fs_path

                await aqtbot.wait_until(_path_reached)

        def name_to_selection(self, name):
            for i in range(f_w.table_files.rowCount()):
                if f_w.table_files.item(i, 1).text() == name:
                    return [i, i]
            else:
                raise ValueError(f"No item with name {name}")

        async def apply_selection(self, selection, reset_selection_first=True):
            # selection can be string, tuple of (start_line, end_line) or a list of that
            if reset_selection_first:
                await self.reset_selection()
            if isinstance(selection, (list, tuple)):
                ranges = [
                    x if isinstance(x, (list, tuple)) else self.name_to_selection(x)
                    for x in selection
                ]
            else:
                ranges = [self.name_to_selection(selection)]

            def _do_selection():
                for top, bottom in ranges:
                    f_w.table_files.setRangeSelected(
                        QtWidgets.QTableWidgetSelectionRange(top, 0, bottom, 0), True
                    )

            _do_selection()

        async def reset_selection(self):
            f_w.table_files.reset()

        async def copy(self, selection=None):
            if selection is not None:
                await self.apply_selection(selection)
            async with aqtbot.wait_signal(f_w.table_files.copy_clicked):
                aqtbot.key_click(f_w.table_files, "C", modifier=QtCore.Qt.ControlModifier)
            assert f_w.clipboard is not None

        async def cut(self, selection=None):
            if selection is not None:
                await self.apply_selection(selection)
            async with aqtbot.wait_signal(f_w.table_files.cut_clicked):
                aqtbot.key_click(f_w.table_files, "X", modifier=QtCore.Qt.ControlModifier)
            assert f_w.clipboard is not None

        async def paste(self):
            async with aqtbot.wait_signal(f_w.table_files.paste_clicked):
                aqtbot.key_click(f_w.table_files, "V", modifier=QtCore.Qt.ControlModifier)

        async def check_files_view(
            self, path, expected_entries: list[str], workspace_name=EntryName("wksp1")
        ):
            expected_table_files = []
            # Parent dir line brings to workspaces list if we looking into workspace's root
            if path == "/":
                expected_table_files.append(("Workspaces list", FileType.ParentWorkspace))
            else:
                expected_table_files.append(("Parent folder", FileType.ParentFolder))
            # Table contains one line per files + the "parent dir" line
            for name in expected_entries:
                if name.endswith("/"):
                    expected_table_files.append((name[:-1], FileType.Folder))
                elif name.endswith("!"):
                    expected_table_files.append((name[:-1], FileType.Inconsistency))
                else:
                    expected_table_files.append((name, FileType.File))

            def _view_ok():
                # Check title (top part of the GUI)
                assert c_w.navigation_bar_widget.workspace_name == workspace_name
                assert str(c_w.navigation_bar_widget.get_current_path()) == path
                # Now check actual files view
                assert f_w.workspace_fs.get_workspace_name() == workspace_name
                assert f_w.table_files.rowCount() == len(expected_table_files)

                for i, (name, type) in enumerate(expected_table_files):
                    assert f_w.table_files.item(i, 1).text() == name
                    for j in range(5):
                        assert f_w.table_files.item(i, j).data(TYPE_DATA_INDEX) == type

            await aqtbot.wait_until(_view_ok)

        async def create_folder(self, name, wait_until=None):
            monkeypatch.setattr(
                "parsec.core.gui.files_widget.get_text_input", lambda *args, **kwargs: (name)
            )
            aqtbot.mouse_click(f_w.button_create_folder, QtCore.Qt.LeftButton)

            def _folder_created():
                for i in range(f_w.table_files.rowCount()):
                    if f_w.table_files.item(i, 1).text() == name:
                        break
                else:
                    assert False  # New folder not found

            await aqtbot.wait_until(wait_until or _folder_created)

        async def delete(self, selection=None, wait_until=None):
            if selection is not None:
                await self.apply_selection(selection)
            to_delete_count = len(f_w.table_files.selected_files())
            initial_items_count = f_w.table_files.rowCount()
            monkeypatch.setattr(
                "parsec.core.gui.files_widget.ask_question", lambda *args: _("ACTION_FILE_DELETE")
            )
            f_w.table_files.delete_clicked.emit()

            def _item_deleted():
                new_items_count = f_w.table_files.rowCount()
                assert new_items_count + to_delete_count == initial_items_count

            await aqtbot.wait_until(wait_until or _item_deleted)

        async def rename(self, new_name, selection=None, wait_until=None):
            if selection is not None:
                await self.apply_selection(selection)
            to_rename_names = [x.name for x in f_w.table_files.selected_files()]
            monkeypatch.setattr(
                "parsec.core.gui.files_widget.get_text_input", lambda *args, **kwargs: new_name
            )
            f_w.table_files.rename_clicked.emit()

            def _item_renamed():
                rows = f_w.table_files.rowCount()
                assert rows != 0  # If is about to be redrawn
                for i in range(rows):
                    assert f_w.table_files.item(i, 1).text() not in to_rename_names

            await aqtbot.wait_until(wait_until or _item_renamed)

    return FilesWidgetTestbed()


@pytest.fixture
async def files_widget_testbed(monkeypatch, aqtbot, logged_gui: MainWindow):
    c_w = logged_gui.test_get_central_widget()
    w_w = logged_gui.test_get_workspaces_widget()

    w_w.check_hide_unmounted.setChecked(False)

    workspace_name = EntryName("wksp1")

    # Create the workspace
    user_fs = logged_gui.test_get_core().user_fs
    wid = await user_fs.workspace_create(workspace_name)
    wfs = user_fs.get_workspace(wid)

    # Now wait for GUI to take it into account
    def _workspace_available():
        assert w_w.layout_workspaces.count() == 1

    await aqtbot.wait_until(_workspace_available)

    f_w: FilesWidget = await logged_gui.test_switch_to_files_widget(workspace_name)

    return create_files_widget_testbed(monkeypatch, aqtbot, logged_gui, user_fs, wfs, f_w, c_w)


@pytest.mark.gui
@pytest.mark.trio
async def test_file_browsing_and_edit(
    monkeypatch: pytest.MonkeyPatch,
    tmpdir: Path,
    aqtbot,
    autoclose_dialog,
    files_widget_testbed,
):
    # cspell: ignore zdir
    tb = files_widget_testbed
    f_w: FilesWidget = files_widget_testbed.files_widget

    # Populate some files for import
    out_of_parsec_data = Path(tmpdir) / "out_of_parsec_data"
    out_of_parsec_data.mkdir(parents=True)
    (out_of_parsec_data / "file1.txt").touch()
    (out_of_parsec_data / "file2.txt").touch()
    (out_of_parsec_data / "file3").touch()
    (out_of_parsec_data / "dir3/dir31").mkdir(parents=True)
    (out_of_parsec_data / "dir3/dir32").mkdir(parents=True)
    (out_of_parsec_data / "dir3/dir31/file311.txt").touch()

    # Workspace starts empty
    await tb.check_files_view(path="/", expected_entries=[])

    # Now create a folder
    await tb.create_folder("dir1")
    await tb.check_files_view(path="/", expected_entries=["dir1/"])

    # Make sure files list is ordered
    await tb.create_folder("zdir2")  # Keep in mind files and folders should be ordered separately
    await tb.create_folder("dir0")
    await tb.check_files_view(path="/", expected_entries=["dir0/", "dir1/", "zdir2/"])

    # Import files
    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.QDialogInProcess.getOpenFileNames",
        classmethod(
            lambda *args, **kwargs: (
                [
                    out_of_parsec_data / "file1.txt",
                    out_of_parsec_data / "file2.txt",
                    out_of_parsec_data / "file3",
                ],
                True,
            )
        ),
    )
    async with aqtbot.wait_signal(f_w.import_success):
        aqtbot.mouse_click(f_w.button_import_files, QtCore.Qt.LeftButton)
    await tb.check_files_view(
        path="/", expected_entries=["dir0/", "dir1/", "zdir2/", "file1.txt", "file2.txt", "file3"]
    )

    # File with the same name without extension
    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.QDialogInProcess.getOpenFileNames",
        classmethod(lambda *args, **kwargs: ([out_of_parsec_data / "file3"], True)),
    )
    async with aqtbot.wait_signal(f_w.import_success):
        aqtbot.mouse_click(f_w.button_import_files, QtCore.Qt.LeftButton)
    await tb.check_files_view(
        path="/",
        expected_entries=[
            "dir0/",
            "dir1/",
            "zdir2/",
            "file1.txt",
            "file2.txt",
            "file3",
            "file3 (2)",
        ],
    )

    await tb.delete("file3")
    await tb.delete("file3 (2)")
    await tb.check_files_view(
        path="/", expected_entries=["dir0/", "dir1/", "zdir2/", "file1.txt", "file2.txt"]
    )

    # Import a new file with a similar name
    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.QDialogInProcess.getOpenFileNames",
        classmethod(lambda *args, **kwargs: ([out_of_parsec_data / "file1.txt"], True)),
    )
    async with aqtbot.wait_signal(f_w.import_success):
        aqtbot.mouse_click(f_w.button_import_files, QtCore.Qt.LeftButton)
    await tb.check_files_view(
        path="/",
        expected_entries=["dir0/", "dir1/", "zdir2/", "file1 (2).txt", "file1.txt", "file2.txt"],
    )

    await tb.delete(selection=["file1 (2).txt"])
    await tb.check_files_view(
        path="/", expected_entries=["dir0/", "dir1/", "zdir2/", "file1.txt", "file2.txt"]
    )

    # Import folder
    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.QDialogInProcess.getExistingDirectory",
        classmethod(lambda *args, **kwargs: out_of_parsec_data / "dir3"),
    )
    async with aqtbot.wait_signal(f_w.import_success):
        aqtbot.mouse_click(f_w.button_import_folder, QtCore.Qt.LeftButton)
    await tb.check_files_view(
        path="/", expected_entries=["dir0/", "dir1/", "dir3/", "zdir2/", "file1.txt", "file2.txt"]
    )

    # Rename multiple items
    await tb.rename(selection=["dir0", "file1.txt", "file2.txt"], new_name="renamed")
    await tb.check_files_view(
        path="/",
        expected_entries=[
            "dir1/",
            "dir3/",
            "renamed_1/",
            "zdir2/",
            "renamed_2.txt",
            "renamed_3.txt",
        ],
    )

    # Rename folder&file
    await tb.rename(selection="renamed_1", new_name="dir0")
    await tb.rename(selection="renamed_2.txt", new_name="file1.txt")
    await tb.check_files_view(
        path="/",
        expected_entries=["dir0/", "dir1/", "dir3/", "zdir2/", "file1.txt", "renamed_3.txt"],
    )

    # Overwriting a file is ok
    await tb.rename(selection="renamed_3.txt", new_name="file1.txt")
    await tb.check_files_view(
        path="/", expected_entries=["dir0/", "dir1/", "dir3/", "zdir2/", "file1.txt"]
    )
    # Overwriting an empty folder is also ok...
    await tb.rename(selection="dir0", new_name="dir1")
    await tb.check_files_view(path="/", expected_entries=["dir1/", "dir3/", "zdir2/", "file1.txt"])

    # ...but not a non-empty folder !
    def _error_displayed():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs[0][0] == "Error"
        assert autoclose_dialog.dialogs[0][1] == "The file could not be renamed."

    await tb.rename(selection="dir1", new_name="dir3", wait_until=_error_displayed)
    autoclose_dialog.reset()
    await tb.check_files_view(path="/", expected_entries=["dir1/", "dir3/", "zdir2/", "file1.txt"])

    # Overwriting between file and folder is not ok
    def _error_displayed():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs[0][0] == "Error"
        assert autoclose_dialog.dialogs[0][1] == "The file could not be renamed."

    for selection, new_name in (("dir1", "file1.txt"), ("file1.txt", "dir1")):
        await tb.rename(selection=selection, new_name=new_name, wait_until=_error_displayed)
        # Nothing should have changes
        await tb.check_files_view(
            path="/", expected_entries=["dir1/", "dir3/", "zdir2/", "file1.txt"]
        )
        autoclose_dialog.reset()

    # Jump into sub folder /dir3
    await tb.cd("dir3")

    # Create folder in the sub folder
    await tb.create_folder("dir33")
    await tb.check_files_view(path="/dir3", expected_entries=["dir31/", "dir32/", "dir33/"])

    # Retry to create another folder with the same name !
    def _error_displayed():
        assert len(autoclose_dialog.dialogs) == 1
        assert autoclose_dialog.dialogs[0][0] == "Error"
        assert autoclose_dialog.dialogs[0][1] == "A folder with the same name already exists."

    await tb.create_folder("dir33", wait_until=_error_displayed)
    autoclose_dialog.reset()

    # Copy a folder /dir3/dir31
    await tb.copy("dir31")

    # Moving to sub directory /dir3/dir33 and do the paste
    await tb.cd("dir33")
    await tb.paste()
    await tb.check_files_view(path="/dir3/dir33", expected_entries=["dir31/"])

    # Have a look inside the copied folder /dir3/dir33/dir31
    await tb.cd("dir31")
    await tb.check_files_view(path="/dir3/dir33/dir31", expected_entries=["file311.txt"])

    # Cut /dir3/dir33/dir31/file311.txt
    await tb.cut("file311.txt")

    # Go back into the original /dir3/dir31/
    await tb.cd("../..")
    await tb.check_files_view(path="/dir3", expected_entries=["dir31/", "dir32/", "dir33/"])
    await tb.cd("dir31")
    await tb.check_files_view(path="/dir3/dir31", expected_entries=["file311.txt"])

    # Paste /dir3/dir33/dir31/file311.txt as /dir3/dir31/file311 (2).txt
    await tb.paste()
    await tb.check_files_view(
        path="/dir3/dir31", expected_entries=["file311 (2).txt", "file311.txt"]
    )

    # Go back into /dir3 and cut /dir3/dir31
    await tb.cd("..")
    await tb.check_files_view(path="/dir3", expected_entries=["dir31/", "dir32/", "dir33/"])
    await tb.cut("dir31")

    # Jump back into root /, and do the paste
    await tb.cd("..")
    await tb.paste()
    await tb.check_files_view(
        path="/", expected_entries=["dir1/", "dir3/", "dir31/", "zdir2/", "file1.txt"]
    )
    await tb.cd("dir3")
    await tb.check_files_view(path="/dir3", expected_entries=["dir32/", "dir33/"])
    await tb.cd("../dir31")
    await tb.check_files_view(path="/dir31", expected_entries=["file311 (2).txt", "file311.txt"])

    # Jump back into root dir
    await tb.cd("..")

    # Delete empty folder
    await tb.delete("dir1")
    await tb.check_files_view(path="/", expected_entries=["dir3/", "dir31/", "zdir2/", "file1.txt"])

    # Delete non-empty folder
    await tb.delete("dir3")
    await tb.check_files_view(path="/", expected_entries=["dir31/", "zdir2/", "file1.txt"])

    # Delete multiple items
    await tb.delete(selection=["zdir2", "file1.txt"])
    await tb.check_files_view(path="/", expected_entries=["dir31/"])

    # Finally jump again, ending up in workspaces list
    w_w = tb.logged_gui.test_get_workspaces_widget()
    async with aqtbot.wait_exposed(w_w):
        f_w.table_files.item_activated.emit(FileType.ParentWorkspace, "Workspaces list")


@pytest.mark.gui
@pytest.mark.trio
async def test_show_inconsistent_dir(
    aqtbot, autoclose_dialog, files_widget_testbed, running_backend
):
    tb = files_widget_testbed

    # Create a new workspace and make user GUI knows about it
    await tb.logged_gui.test_switch_to_workspaces_widget()
    await create_inconsistent_workspace(user_fs=tb.user_fs, name=EntryName("wksp2"))

    # Now wait for GUI to take it into account
    def _workspace_available():
        assert tb.logged_gui.test_get_workspaces_widget().layout_workspaces.count() == 2

    await aqtbot.wait_until(_workspace_available)

    # Jump into the workspace, should be fine
    await tb.logged_gui.test_switch_to_files_widget(workspace_name=EntryName("wksp2"))
    await tb.check_files_view(
        workspace_name=EntryName("wksp2"), path="/", expected_entries=["rep/"]
    )

    # Now go into the folder containing the `newfail.txt` inconsistent file
    await tb.cd("rep")
    await tb.check_files_view(
        workspace_name=EntryName("wksp2"), path="/rep", expected_entries=["foo.txt", "newfail.txt!"]
    )


# This test has been detected as flaky.
# Using re-runs is a valid temporary solutions but the problem should be investigated in the future.
@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.flaky(reruns=3)
async def test_copy_cut_between_workspaces(aqtbot, autoclose_dialog, files_widget_testbed):
    tb = files_widget_testbed

    # Create a new workspace and make user GUI knows about it
    await tb.logged_gui.test_switch_to_workspaces_widget()
    await tb.user_fs.workspace_create(EntryName("wksp2"))

    # Now wait for GUI to take it into account
    def _workspace_available():
        assert tb.logged_gui.test_get_workspaces_widget().layout_workspaces.count() == 2

    await aqtbot.wait_until(_workspace_available)

    # Populate the first workspace
    await tb.workspace_fs.mkdir("/foo")
    await tb.workspace_fs.touch("/foo/bar.txt")
    await tb.workspace_fs.touch("/foo/spam.txt")
    await tb.check_files_view(path="/", expected_entries=["foo/"])

    # This helps a lot with the consistency of this test but I have no idea why
    # This should be investigated in the future.
    relax = 0.1  # s
    await trio.sleep(relax)

    # 1) Test the copy
    await tb.copy("foo")
    await tb.logged_gui.test_switch_to_files_widget(workspace_name=EntryName("wksp2"))
    await tb.paste()
    await tb.check_files_view(
        workspace_name=EntryName("wksp2"), path="/", expected_entries=["foo/"]
    )

    # This helps a lot with the consistency of this test but I have no idea why
    # This should be investigated in the future.
    await trio.sleep(relax)

    await tb.cd("foo")
    await tb.check_files_view(
        workspace_name=EntryName("wksp2"), path="/foo", expected_entries=["bar.txt", "spam.txt"]
    )

    # This helps a lot with the consistency of this test but I have no idea why
    # This should be investigated in the future.
    await trio.sleep(relax)

    # 2) Test the cut
    await tb.cut(["bar.txt", "spam.txt"])
    await tb.logged_gui.test_switch_to_files_widget(workspace_name=EntryName("wksp1"))
    await tb.paste()
    await tb.check_files_view(path="/", expected_entries=["foo/", "bar.txt", "spam.txt"])
    # Make sure the files are no longer in the initial workspace
    await tb.logged_gui.test_switch_to_files_widget(workspace_name=EntryName("wksp2"))
    await tb.cd("foo")
    await tb.check_files_view(workspace_name=EntryName("wksp2"), path="/foo", expected_entries=[])


@pytest.mark.gui
@pytest.mark.trio
async def test_cut_dir_in_itself(aqtbot, autoclose_dialog, files_widget_testbed):
    tb = files_widget_testbed

    # Populate the workspace
    await tb.workspace_fs.mkdir("/foo")
    await tb.workspace_fs.touch("/foo/bar.txt")
    await tb.check_files_view(path="/", expected_entries=["foo/"])

    await tb.cut("foo")

    # Moving to sub folder and do the paste
    await tb.cd("foo")
    await tb.paste()

    def _paste_failed():
        assert autoclose_dialog.dialogs == [
            ("Error", _("TEXT_FILE_FOLDER_MOVED_INTO_ITSELF_ERROR"))
        ]
        # sub folder shouldn't have changed
        assert tb.ls() == ["bar.txt"]
        assert tb.pwd() == "/foo"

    await aqtbot.wait_until(_paste_failed)

    # Moving back to root, nothing should have changed
    await tb.cd("..")
    await tb.check_files_view(path="/", expected_entries=["foo/"])


@pytest.mark.gui
@pytest.mark.trio
async def test_drag_and_drop(tmpdir, aqtbot, autoclose_dialog, files_widget_testbed):
    tb = files_widget_testbed
    f_w = files_widget_testbed.files_widget

    # Populate some files for import
    file1 = Path(tmpdir) / "file1.txt"
    file1.touch()

    def _import_file():
        mime_data = QtCore.QMimeData()
        mime_data.setUrls([QtCore.QUrl.fromLocalFile(str(file1))])
        drop_event = QtGui.QDropEvent(
            f_w.table_files.pos(),
            QtCore.Qt.MoveAction,
            mime_data,
            QtCore.Qt.LeftButton,
            QtCore.Qt.NoModifier,
        )
        f_w.table_files.dropEvent(drop_event)

    # Good drap&drop

    _import_file()
    await tb.check_files_view(path="/", expected_entries=["file1.txt"])

    # Drap&drop in readonly workspace is not allowed

    # Quick hack to have a read-only workspace ;-)
    f_w.table_files.current_user_role = WorkspaceRole.READER

    _import_file()

    def _import_failed():
        assert autoclose_dialog.dialogs == [("Error", _("TEXT_FILE_DROP_WORKSPACE_IS_READ_ONLY"))]
        assert tb.ls() == ["file1.txt"]
        assert tb.pwd() == "/"

    await aqtbot.wait_until(_import_failed)


@pytest.mark.linux  # Cannot chmod on Windows
@pytest.mark.gui
@pytest.mark.trio
async def test_import_file_permission_denied(
    monkeypatch, tmpdir, aqtbot, autoclose_dialog, files_widget_testbed
):
    tb = files_widget_testbed
    f_w = files_widget_testbed.files_widget

    # Populate some files for import
    out_of_parsec_data = Path(tmpdir) / "out_of_parsec_data"
    out_of_parsec_data.mkdir(parents=True)
    file1 = out_of_parsec_data / "file1.txt"
    file2 = out_of_parsec_data / "file2.txt"
    file1.touch()
    file2.touch()
    # Changing file permissions
    os.chmod(file1, 000)
    try:

        # Try importing one file

        monkeypatch.setattr(
            "parsec.core.gui.custom_dialogs.QDialogInProcess.getOpenFileNames",
            classmethod(lambda *args, **kwargs: ([file1], True)),
        )

        async with aqtbot.wait_signal(f_w.button_import_files.clicked):
            aqtbot.mouse_click(f_w.button_import_files, QtCore.Qt.LeftButton)

        def _import_failed():
            assert autoclose_dialog.dialogs == [
                ("Error", _("TEXT_FILE_IMPORT_ONE_PERMISSION_ERROR"))
            ]
            assert tb.ls() == []
            assert tb.pwd() == "/"

        await aqtbot.wait_until(_import_failed)
        autoclose_dialog.dialogs = []

        # Try importing multiple files

        monkeypatch.setattr(
            "parsec.core.gui.custom_dialogs.QDialogInProcess.getOpenFileNames",
            classmethod(lambda *args, **kwargs: ([file1, file2], True)),
        )

        async with aqtbot.wait_signal(f_w.button_import_files.clicked):
            aqtbot.mouse_click(f_w.button_import_files, QtCore.Qt.LeftButton)

        def _import_error_shown():
            assert autoclose_dialog.dialogs == [
                ("Error", _("TEXT_FILE_IMPORT_MULTIPLE_PERMISSION_ERROR"))
            ]
            assert tb.ls() == ["file2.txt"]
            assert tb.pwd() == "/"

        await aqtbot.wait_until(_import_error_shown)

    finally:
        # Try to reset the permission to avoid errors pytest tmp dir is cleaned
        os.chmod(file1, 755)


@pytest.mark.gui
@pytest.mark.trio
async def test_open_file_failed(monkeypatch, aqtbot, autoclose_dialog, files_widget_testbed):
    tb = files_widget_testbed
    f_w = files_widget_testbed.files_widget

    # Populate the workspace with files
    await tb.workspace_fs.touch("/file1.txt")
    await tb.workspace_fs.touch("/file2.txt")
    await tb.check_files_view(path="/", expected_entries=["file1.txt", "file2.txt"])

    async def _open_files_job(paths):
        return False, paths

    monkeypatch.setattr("parsec.core.gui.files_widget.desktop.open_files_job", _open_files_job)
    monkeypatch.setattr(
        "parsec.core.gui.files_widget.ask_question",
        lambda *args, **kwargs: (_("ACTION_FILE_OPEN_MULTIPLE")),
    )

    # Open the file selected
    await tb.apply_selection("file1.txt")
    f_w.table_files.open_clicked.emit()

    def _open_single_file_error_shown():
        assert autoclose_dialog.dialogs == [
            ("Error", _("TEXT_FILE_OPEN_ERROR_file").format(file="file1.txt"))
        ]

    await aqtbot.wait_until(_open_single_file_error_shown)
    await tb.reset_selection()
    autoclose_dialog.reset()

    # Open a file by double click
    f_w.table_files.item_activated.emit(FileType.File, "file1.txt")
    await aqtbot.wait_until(_open_single_file_error_shown)
    autoclose_dialog.reset()

    # Open multiple files
    await tb.apply_selection(["file1.txt", "file2.txt"])
    f_w.table_files.open_clicked.emit()

    def _open_multiple_files_error_shown():
        assert autoclose_dialog.dialogs == [("Error", _("TEXT_FILE_OPEN_MULTIPLE_ERROR"))]

    await aqtbot.wait_until(_open_multiple_files_error_shown)


@pytest.mark.gui
@pytest.mark.trio
async def test_copy_file_link(aqtbot, autoclose_dialog, files_widget_testbed, snackbar_catcher):
    tb = files_widget_testbed
    f_w = files_widget_testbed.files_widget

    # Populate the workspace
    await tb.workspace_fs.mkdir("/foo")
    await tb.workspace_fs.touch("/foo/bar.txt")
    await tb.check_files_view(path="/", expected_entries=["foo/"])

    await tb.cd("foo")
    await tb.apply_selection("bar.txt")
    snackbar_catcher.reset()

    f_w.table_files.file_path_clicked.emit()

    def _file_link_copied_snackbar():
        assert snackbar_catcher.snackbars == [("INFO", _("TEXT_FILE_LINK_COPIED_TO_CLIPBOARD"))]
        url = QGuiApplication.clipboard().text()
        assert url.startswith("parsec://")

    await aqtbot.wait_until(_file_link_copied_snackbar)


@pytest.mark.gui
@pytest.mark.trio
async def test_use_file_link(aqtbot, autoclose_dialog, files_widget_testbed):
    tb = files_widget_testbed
    f_w: FilesWidget = files_widget_testbed.files_widget

    # Populate the workspace
    await tb.workspace_fs.mkdir("/foo")
    await tb.workspace_fs.touch("/foo/bar.txt")
    await tb.check_files_view(path="/", expected_entries=["foo/"])

    # Create and use file link
    url = f_w.workspace_fs.generate_file_link("/foo/bar.txt")
    tb.logged_gui.add_instance(url.to_url())

    def _selection_on_file():
        assert tb.pwd() == "/foo"
        selected_files = f_w.table_files.selected_files()
        print(f"[{_selection_on_file.__name__}] selected_files={selected_files}")
        assert len(selected_files) == 1
        selected_files[0].name == "bar.txt"
        # No new tab has been created
        assert tb.logged_gui.tab_center.count() == 1

    await aqtbot.wait_until(_selection_on_file)


# Note: other file link tests are in test_main_window.py


@pytest.fixture
def catch_file_status_widget(widget_catcher_factory):
    return widget_catcher_factory("parsec.core.gui.file_status_widget.FileStatusWidget")


@pytest.mark.gui
@pytest.mark.trio
async def test_show_file_status(
    running_backend, aqtbot, autoclose_dialog, files_widget_testbed, catch_file_status_widget
):
    tb = files_widget_testbed
    f_w = files_widget_testbed.files_widget

    # Populate the workspace
    await tb.workspace_fs.mkdir("/foo")
    await tb.workspace_fs.mkdir("/foo/innerfoo")
    await tb.workspace_fs.touch("/foo/bar.txt")
    await tb.check_files_view(path="/", expected_entries=["foo/"])

    await f_w.workspace_fs.sync()

    await tb.cd("foo")

    await tb.check_files_view(path="/foo", expected_entries=["innerfoo/", "bar.txt"])

    # Try to open status with two files selected
    await tb.apply_selection(["bar.txt", "innerfoo"])

    selected_files = f_w.table_files.selected_files()
    assert len(selected_files) == 2
    f_w.show_selected_file_status()

    assert autoclose_dialog.dialogs == [
        ("Error", _("TEXT_FILE_STATUS_MULTIPLE_FILES_SELECTED_ERROR"))
    ]
    autoclose_dialog.reset()

    # Select a single file
    await tb.apply_selection("bar.txt")
    selected_files = f_w.table_files.selected_files()
    assert len(selected_files) == 1

    f_w.show_selected_file_status()
    file_status_w = await catch_file_status_widget()
    assert file_status_w

    def _wait_status_shown():
        assert file_status_w.label_location.text().endswith(str(Path("/foo/bar.txt")))
        assert file_status_w.label_workspace.text() == "wksp1"
        assert file_status_w.label_filetype.text() == "file"
        assert file_status_w.label_size.text() == "0 B"
        assert (
            file_status_w.label_created_on.text() != ""
            and file_status_w.label_created_on.text() != _("TEXT_FILE_INFO_NON_APPLICABLE")
        )
        assert file_status_w.label_created_by.text() == "Boby McBobFace"
        assert (
            file_status_w.label_last_updated_on.text() != ""
            and file_status_w.label_last_updated_on.text() != _("TEXT_FILE_INFO_NON_APPLICABLE")
        )
        assert file_status_w.label_last_updated_by.text() == "Boby McBobFace"
        assert file_status_w.label_availability.text() == _("TEXT_YES")
        assert file_status_w.label_uploaded.text() == _("TEXT_YES")
        assert file_status_w.label_local.text() == "0/0 (100%)"
        assert file_status_w.label_remote.text() == "0/0 (100%)"
        assert file_status_w.label_default_block_size.text() == get_filesize(DEFAULT_BLOCK_SIZE)

    await aqtbot.wait_until(_wait_status_shown)
    file_status_w.dialog.accept()

    # Select a single dir
    await tb.apply_selection("innerfoo")
    selected_files = f_w.table_files.selected_files()
    assert len(selected_files) == 1

    f_w.show_selected_file_status()
    file_status_w = await catch_file_status_widget()
    assert file_status_w

    def _wait_status_shown():
        assert file_status_w.label_location.text().endswith(str(Path("/foo/innerfoo")))
        assert file_status_w.label_workspace.text() == "wksp1"
        assert file_status_w.label_filetype.text() == "folder"
        assert file_status_w.label_size.text() == _("TEXT_FILE_INFO_NON_APPLICABLE")
        assert (
            file_status_w.label_created_on.text() != ""
            and file_status_w.label_created_on.text() != _("TEXT_FILE_INFO_NON_APPLICABLE")
        )
        assert file_status_w.label_created_by.text() == "Boby McBobFace"
        assert (
            file_status_w.label_last_updated_on.text() != ""
            and file_status_w.label_last_updated_on.text() != _("TEXT_FILE_INFO_NON_APPLICABLE")
        )
        assert file_status_w.label_last_updated_by.text() == "Boby McBobFace"
        assert file_status_w.label_availability.text() == _("TEXT_FILE_INFO_NON_APPLICABLE")
        assert file_status_w.label_uploaded.text() == _("TEXT_FILE_INFO_NON_APPLICABLE")
        assert file_status_w.label_local.text() == _("TEXT_FILE_INFO_NON_APPLICABLE")
        assert file_status_w.label_remote.text() == _("TEXT_FILE_INFO_NON_APPLICABLE")
        assert file_status_w.label_default_block_size.text() == _("TEXT_FILE_INFO_NON_APPLICABLE")

    await aqtbot.wait_until(_wait_status_shown)


@pytest.mark.gui
@pytest.mark.trio
async def test_import_file_disk_full(
    monkeypatch: pytest.MonkeyPatch, tmpdir, aqtbot, autoclose_dialog, files_widget_testbed
):
    tb = files_widget_testbed
    f_w: FilesWidget = files_widget_testbed.files_widget

    # Populate some files for import
    out_of_parsec_data = Path(tmpdir) / "out_of_parsec_data"
    out_of_parsec_data.mkdir(parents=True)
    file1 = out_of_parsec_data / "file1.txt"
    file2 = out_of_parsec_data / "file2.txt"
    file1.touch()
    file2.touch()

    @staticmethod
    def rust_impl_set_manifest_patched(
        entry_id: EntryID,
        manifest: AnyLocalManifest,
        cache_only: bool = False,
        removed_ids: set[ChunkID] | None = None,
    ):
        import sqlite3

        from parsec.core.fs.exceptions import FSLocalStorageOperationalError

        try:
            raise sqlite3.OperationalError("database or disk is full")
        except Exception as exc:
            raise FSLocalStorageOperationalError from exc

    # Patch `run_in_thread` to raise an OperationError at the next commit
    monkeypatch.setattr(
        "parsec._parsec.WorkspaceStorage.set_manifest", rust_impl_set_manifest_patched
    )

    monkeypatch.setattr(
        "parsec.core.gui.custom_dialogs.QDialogInProcess.getOpenFileNames",
        classmethod(lambda *args, **kwargs: ([file1], True)),
    )

    async with aqtbot.wait_signal(f_w.button_import_files.clicked):
        aqtbot.mouse_click(f_w.button_import_files, QtCore.Qt.LeftButton)

    def _import_failed():
        assert set(autoclose_dialog.dialogs) == {
            ("Error", _("TEXT_FILE_IMPORT_ONE_ERROR")),
            ("Error", _("TEXT_FILE_IMPORT_LOCAL_STORAGE_ERROR")),
        }
        assert tb.ls() == []
        assert tb.pwd() == "/"

    await aqtbot.wait_until(_import_failed)


@pytest.mark.gui
@pytest.mark.trio
async def test_new_folder_menu(aqtbot, files_widget_testbed, monkeypatch):
    tb = files_widget_testbed
    f_w = files_widget_testbed.files_widget

    await tb.check_files_view(path="/", expected_entries=[])

    monkeypatch.setattr(
        "parsec.core.gui.files_widget.get_text_input", lambda *args, **kwargs: "NewFolder"
    )
    async with aqtbot.wait_signal(f_w.folder_create_success):
        f_w.table_files.new_folder_clicked.emit()

    await tb.check_files_view(path="/", expected_entries=["NewFolder/"])


@pytest.mark.gui
@pytest.mark.trio
async def test_sort_menu(aqtbot, files_widget_testbed, monkeypatch):
    tb = files_widget_testbed
    f_w = files_widget_testbed.files_widget

    await tb.check_files_view(path="/", expected_entries=[])

    # Populate the workspace
    await tb.workspace_fs.mkdir("/h")
    await tb.workspace_fs.mkdir("/n")
    await tb.workspace_fs.mkdir("/w")
    await tb.workspace_fs.touch("/a.txt")
    await tb.workspace_fs.touch("/j.txt")
    await tb.workspace_fs.touch("/v.txt")
    await tb.check_files_view(
        path="/", expected_entries=["h/", "n/", "w/", "a.txt", "j.txt", "v.txt"]
    )

    f_w.table_files.sort_clicked.emit(Column.NAME)

    await tb.check_files_view(
        path="/", expected_entries=["a.txt", "h/", "j.txt", "n/", "v.txt", "w/"]
    )


@pytest.mark.gui
@pytest.mark.trio
async def test_current_folder_status_menu(
    running_backend, aqtbot, files_widget_testbed, catch_file_status_widget
):
    f_w = files_widget_testbed.files_widget

    await f_w.workspace_fs.sync()

    f_w.table_files.show_current_folder_status_clicked.emit()

    file_status_w = await catch_file_status_widget()
    assert file_status_w

    def _wait_status_shown():
        path = file_status_w.core.mountpoint_manager.get_path_in_mountpoint(
            file_status_w.workspace_fs.workspace_id, FsPath("/"), None
        )
        assert file_status_w.label_location.text() == str(path)
        assert file_status_w.label_workspace.text() == "wksp1"
        assert file_status_w.label_filetype.text() == "folder"
        assert file_status_w.label_size.text() == "N/A"
        assert (
            file_status_w.label_created_on.text() != ""
            and file_status_w.label_created_on.text() != _("TEXT_FILE_INFO_NON_APPLICABLE")
        )
        assert file_status_w.label_created_by.text() == "Boby McBobFace"
        assert (
            file_status_w.label_last_updated_on.text() != ""
            and file_status_w.label_last_updated_on.text() != _("TEXT_FILE_INFO_NON_APPLICABLE")
        )
        assert file_status_w.label_last_updated_by.text() == "Boby McBobFace"
        assert file_status_w.label_availability.text() == "N/A"
        assert file_status_w.label_uploaded.text() == "N/A"
        assert file_status_w.label_local.text() == "N/A"
        assert file_status_w.label_remote.text() == "N/A"
        assert file_status_w.label_default_block_size.text() == "N/A"

    await aqtbot.wait_until(_wait_status_shown)

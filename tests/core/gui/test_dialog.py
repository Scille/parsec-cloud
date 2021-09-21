# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from parsec.core.gui.custom_dialogs import QFileDialogInProcess


@pytest.mark.gui
@pytest.mark.trio
async def test_file_dialog_in_process(gui):
    assert QFileDialogInProcess.getOpenFileName(gui, 42, test_suffix="_testing") == [
        "42",
        "<getOpenFileName>",
    ]
    assert QFileDialogInProcess.getOpenFileNames(gui, 42, test_suffix="_testing") == [
        ["42"],
        "<getOpenFileNames>",
    ]
    assert QFileDialogInProcess.getSaveFileName(gui, 42, test_suffix="_testing") == [
        "42",
        "<getSaveFileName>",
    ]
    assert (
        QFileDialogInProcess.getExistingDirectory(gui, 42, test_suffix="_testing")
        == "42/getExistingDirectory"
    )

    with pytest.raises(RuntimeError) as ctx:
        QFileDialogInProcess.getOpenFileName(gui, i_dont_exist=42)
    assert "'i_dont_exist' is not a valid keyword argument" in str(ctx.value)

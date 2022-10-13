# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
import multiprocessing
from parsec.core.gui.custom_dialogs import QDialogInProcess


@pytest.fixture
def close_process_pool():
    if multiprocessing.get_start_method(allow_none=True) is None:
        multiprocessing.set_start_method("spawn")
    assert multiprocessing.get_start_method() == "spawn"
    yield
    for pool in QDialogInProcess.pools.values():
        pool.terminate()
        pool.join()
    QDialogInProcess.pools.clear()


@pytest.mark.gui
@pytest.mark.trio
@pytest.mark.flaky(reruns=3)
async def test_file_dialog_in_process(gui, close_process_pool):
    assert QDialogInProcess.getOpenFileName(gui, "title", dir="dir", testing=True) == (
        "getOpenFileName",
        ("title",),
        {"dir": "dir"},
    )
    assert QDialogInProcess.getOpenFileNames(gui, "title", dir="dir", testing=True) == (
        "getOpenFileNames",
        ("title",),
        {"dir": "dir"},
    )
    assert QDialogInProcess.getSaveFileName(gui, "title", dir="dir", testing=True) == (
        "getSaveFileName",
        ("title",),
        {"dir": "dir"},
    )
    assert QDialogInProcess.getExistingDirectory(gui, "title", dir="dir", testing=True) == (
        "getExistingDirectory",
        ("title",),
        {"dir": "dir"},
    )

    assert QDialogInProcess.print_html(gui, "some_html", testing=True) == (
        "print_html",
        ("some_html",),
        {},
    )

    with pytest.raises(TypeError) as ctx:
        QDialogInProcess.getOpenFileName(gui, i_dont_exist=42)
    assert "'i_dont_exist' is not a valid keyword argument" in str(ctx.value)

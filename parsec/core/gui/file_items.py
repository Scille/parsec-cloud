# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pathlib
from enum import IntEnum
from typing import Any, cast

from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import QTableWidgetItem
from PyQt5.QtGui import QIcon, QPainter, QColor, QPixmap

from parsec.core.gui.lang import translate as _
from parsec.core.gui.custom_widgets import Pixmap

NAME_DATA_INDEX = Qt.UserRole
TYPE_DATA_INDEX = Qt.UserRole + 1
ENTRY_ID_DATA_INDEX = Qt.UserRole + 2
COPY_STATUS_DATA_INDEX = Qt.UserRole + 3


class FileType(IntEnum):
    ParentWorkspace = 1
    ParentFolder = 2
    Folder = 3
    File = 4
    Inconsistency = 5


class CustomTableItem(QTableWidgetItem):
    def __lt__(self, other: object) -> bool:
        if not isinstance(other, CustomTableItem):
            return False

        self_type = self.data(TYPE_DATA_INDEX)
        other_type = other.data(TYPE_DATA_INDEX)
        self_data = self.data(NAME_DATA_INDEX)
        other_data = other.data(NAME_DATA_INDEX)

        if self_type == FileType.ParentWorkspace or self_type == FileType.ParentFolder:
            return False
        elif other_type == FileType.ParentWorkspace or other_type == FileType.ParentFolder:
            return False
        return self_data < other_data


class IconTableItem(CustomTableItem):
    SYNCED_ICON = None
    UNSYNCED_ICON = None
    CONFINED_ICON = None

    def __init__(
        self, is_synced: bool, is_confined: bool, pixmap: str, *args: Any, **kwargs: Any
    ) -> None:
        super().__init__(*args, **kwargs)
        self.source = pixmap
        if not IconTableItem.CONFINED_ICON:
            IconTableItem.CONFINED_ICON = Pixmap(":/icons/images/material/block.svg")
            IconTableItem.CONFINED_ICON.replace_color(QColor(0, 0, 0), QColor(0xBB, 0xBB, 0xBB))
        if not IconTableItem.SYNCED_ICON:
            IconTableItem.SYNCED_ICON = Pixmap(":/icons/images/material/check.svg")
            IconTableItem.SYNCED_ICON.replace_color(QColor(0, 0, 0), QColor(50, 168, 82))
        if not IconTableItem.UNSYNCED_ICON:
            IconTableItem.UNSYNCED_ICON = Pixmap(":/icons/images/material/cached.svg")
            IconTableItem.UNSYNCED_ICON.replace_color(QColor(0, 0, 0), QColor(219, 20, 20))
        self._is_synced = is_synced
        self._is_confined = is_confined
        self.switch_icon()

    def switch_icon(self) -> None:
        icon = self._draw_pixmap(self.source, self.isSelected(), self.is_synced, self.is_confined)
        self.setIcon(QIcon(icon))
        if self.is_confined:
            self.setToolTip(_("TEXT_FILE_ITEM_IS_CONFINED_TOOLTIP"))
        elif self.is_synced:
            self.setToolTip(_("TEXT_FILE_ITEM_IS_SYNCED_TOOLTIP"))
        else:
            self.setToolTip(_("TEXT_FILE_ITEM_IS_NOT_SYNCED_TOOLTIP"))

    def _draw_pixmap(
        self, source: str, selected: bool, synced: bool, confined: bool
    ) -> QPixmap | None:
        color = QColor(0x99, 0x99, 0x99)
        if confined:
            color = QColor(0xDD, 0xDD, 0xDD)
            if selected:
                color = QColor(0xAA, 0xAA, 0xAA)
        elif selected:
            color = QColor(0x33, 0x33, 0x33)
        p = Pixmap(source)
        if p.isNull():
            return None
        p = Pixmap(p.scaled(120, 120, Qt.KeepAspectRatio))
        p.replace_color(QColor(0, 0, 0), color)
        painter = QPainter(p)
        if confined:
            painter.drawPixmap(
                p.width() - 90, 0, 100, 100, cast(QPixmap, IconTableItem.CONFINED_ICON)
            )
        elif synced:
            painter.drawPixmap(
                p.width() - 90, 0, 100, 100, cast(QPixmap, IconTableItem.SYNCED_ICON)
            )
        else:
            painter.drawPixmap(
                p.width() - 90, 0, 100, 100, cast(QPixmap, IconTableItem.UNSYNCED_ICON)
            )
        painter.end()
        return p

    @property
    def is_synced(self) -> bool:
        return self._is_synced

    @is_synced.setter
    def is_synced(self, value: bool) -> None:
        self._is_synced = value
        self.switch_icon()

    @property
    def is_confined(self) -> bool:
        return self._is_confined

    @is_confined.setter
    def is_confined(self, value: bool) -> None:
        self._is_confined = value
        self.switch_icon()


class FileTableItem(IconTableItem):
    EXTENSIONS = {
        ".pdf": "pdf-file-format-symbol",
        ".3dm": "3dm-file-format",
        ".3ds": "3ds-file-format-symbol",
        ".3g2": "3g2-file-format-symbol",
        ".3gp": "3gp-file-format-variant",
        ".7z": "7z-file-format-variant",
        ".7zip": "7z-file-format-variant",
        ".aac": "aac-file-format",
        ".aif": "aif-file-format",
        ".ai": "ai-file-format-symbol",
        ".apk": "apk-file-format",
        ".app": "app-file-format-variant",
        ".asf": "asf-file-format-variant",
        ".asp": "asp-file-format-symbol",
        ".aspx": "aspx-file-format",
        ".axx": "asx-multimedia-file-format",
        ".avi": "avi-file-format-variant",
        ".bak": "bak-file-format-symbol",
        ".bat": "bat-file-format-symbol",
        ".bin": "bin-file-format",
        ".bmp": "bmp-file-format-symbol",
        ".cab": "cab-file-format",
        ".cad": "cad-file-format-symbol",
        ".cdr": "cdr-file-format-symbol",
        ".cer": "cer-file-format",
        ".cfg": "cfg-file-format-symbol",
        ".cfm": "cfm-file-format-symbol",
        ".cgi": "cgi-file-format-symbol",
        ".class": "class-file-format-variant",
        ".com": "com-file-format-symbol",
        ".cpl": "cpl-file-format-variant",
        ".cpp": "cpp-file-format-symbol",
        ".crx": "crx-file-format-symbol",
        ".csr": "csr-file-format",
        ".css": "css-file-format-symbol",
        ".csv": "csv-file-format-symbol",
        ".cue": "cue-file-format-symbol",
        ".cur": "cur-file-format",
        ".dat": "dat-file-format-variant",
        ".dbf": "dbf-file-format-symbol",
        ".db": "db-file-format-variant",
        ".dds": "dds-file-format-symbol",
        ".deb": "debian-file",
        ".dem": "dem-file-format-symbol",
        ".dll": "dll-file-format-variant",
        ".dmg": "dmg-file-format-symbol",
        ".dmp": "dmp-file-format-symbol",
        ".doc": "doc-file-format-symbol",
        ".docx": "docx-file-format",
        ".drv": "drv-file-format-variant",
        ".dtd": "dtd-file-format-extension",
        ".dwg": "dwg-file-format-variant",
        ".dxf": "dxf-file-format-symbol",
        ".elf": "elf-file",
        ".eml": "eml-file",
        ".eps": "eps-file-format-variant",
        ".exe": "exe-file-format-symbol",
        ".sh": "exe-file-format-symbol",
        ".fla": "fla-file-format-variant",
        ".flash": "flash-file-format",
        ".flv": "flv-file-format-symbol",
        ".fnt": "fnt-file-format",
        ".fon": "fon-file-format-symbol",
        ".gam": "gam-file-format-variant",
        ".gbr": "gbr-file-format-extension",
        ".ged": "ged-file-format-symbol",
        ".gif": "gif-file-format",
        ".gpx": "gpx-file-format-variant",
        ".gz": "gz-file-format-symbol",
        ".tgz": "gz-file-format-symbol",
        ".xf": "gz-file-format-symbol",
        ".gzip": "gzip-file-format-variant",
        ".hqz": "hqz-file-format",
        ".html": "html-file-with-code-symbol",
        ".htm": "html-file-with-code-symbol",
        ".epub": "ibooks-file-format-symbol",
        ".mobi": "ibooks-file-format-symbol",
        ".lrf": "ibooks-file-format-symbol",
        ".lrx": "ibooks-file-format-symbol",
        ".chm": "ibooks-file-format-symbol",
        ".fb2": "ibooks-file-format-symbol",
        ".xeb": "ibooks-file-format-symbol",
        ".ceb": "ibooks-file-format-symbol",
        ".ibooks": "ibooks-file-format-symbol",
        ".azw": "ibooks-file-format-symbol",
        ".azw3": "ibooks-file-format-symbol",
        ".kf8": "ibooks-file-format-symbol",
        ".kfx": "ibooks-file-format-symbol",
        ".lit": "ibooks-file-format-symbol",
        ".prc": "ibooks-file-format-symbol",
        ".opf": "ibooks-file-format-symbol",
        ".tr2": "ibooks-file-format-symbol",
        ".tr3": "ibooks-file-format-symbol",
        "icns": "icns-file-format",
        ".ico": "ico-file-format-variant",
        ".ics": "ics-file-format-symbol",
        ".iff": "iff-file-format",
        ".indd": "indd-file-format-variant",
        ".ipa": "ipa-file",
        ".iso": "iso-file-format",
        ".jar": "jar-file-format",
        ".jpg": "jpg-image-file-format",
        ".jpeg": "jpg-image-file-format",
        ".jfif": "jpg-image-file-format",
        ".js": "js-file-format-symbol",
        ".json": "js-file-format-symbol",
        ".jsp": "jsp-file-format-symbol",
        ".key": "key-file-format-variant",
        ".kml": "kml-file-format-variant",
        ".kmz": "kmz-file-format-symbol",
        ".lnk": "lnk-file-format-symbol",
        ".log": "log-file-format",
        ".lua": "lua-file-format-symbol",
        ".m3u": "m3u-file-format",
        ".m4a": "m4a-file-format-symbol",
        ".m4v": "m4v-file-format-variant",
        ".mkv": "m4v-file-format-variant",
        ".dylib": "mach-o-file-format",
        ".bundle": "mach-o-file-format",
        ".max": "max-file-format-variant",
        ".mdb": "mdb-file-format-symbol",
        ".ndf": "mdf-file-format-variant",
        ".mid": "mid-file-format",
        ".midi": "mid-file-format",
        ".mim": "mim-file-format",
        ".mime": "mim-file-format",
        ".mov": "mov-file-format-symbol",
        ".mp3": "mp3-file-format-variant",
        ".mp4": "mp4-file-format-symbol",
        ".mpa": "mpa-file-format",
        ".mpg": "mpg-file-format-variant",
        ".msg": "msg-file",
        ".msi": "msi-file-format-symbol",
        ".nes": "nes-file-variant",
        ".o": "object-file-format",
        ".odb": "odb-file-format-variant",
        ".odc": "odc-file-format-symbol",
        ".odf": "odf-file-format-variant",
        ".odg": "odg-file-format",
        ".odi": "odi-file-format-symbol",
        ".odp": "odp-file-format-symbol",
        ".ods": "ods-file-format-symbol",
        ".odt": "odt-file-format",
        ".odx": "odx-file-format-extension",
        ".ogg": "ogg-file-format-symbol",
        ".otf": "otf-file-format",
        ".pages": "pages-file-format-symbol",
        ".pct": "pct-file-format-symbol",
        ".pdb": "pdb-file-format-variant",
        ".pif": "pif-file-format-variant",
        ".pkg": "pkg-file-format-variant",
        ".pl": "pl-file-format-variant",
        ".png": "png-file-extension-interface-symbol",
        ".pps": "pps-file-format-symbol",
        ".ppt": "ppt-file-format",
        ".pptx": "pptx-file-format-variant",
        ".psd": "psd-file-format-variant",
        ".ps": "ps-file-format",
        ".pub": "pub-file-format-symbol",
        ".py": "python-file-symbol",
        ".ra": "ra-file-format",
        ".rar": "rar-file-format",
        ".raw": "raw-file-format-symbol",
        ".rm": "rm-file-format",
        ".rom": "rom-file",
        ".rpm": "rpm-file-format-symbol",
        ".rss": "rss-file-format-symbol",
        ".rtf": "rtf-icon-format",
        ".sav": "sav-file-format",
        ".sdf": "sdf-file-format",
        ".sitx": "sitx-file-format-variant",
        ".sql": "sql-file-format-symbol",
        ".srt": "srt-file-format-symbol",
        ".svg": "svg-file-format-symbol",
        ".swf": "swf-file-format-symbol",
        ".sys": "sys-file-format",
        ".tar": "tar-file-variant",
        ".tex": "tex-file-format",
        ".tga": "tga-file-format-symbol",
        ".thm": "thm-file-format-symbol",
        ".tiff": "tiff-images-file-extension-symbol-for-interface",
        ".tmp": "tmp-icon-file-format",
        ".torrent": "torrent-file-format",
        ".ttf": "ttf-file-format-symbol",
        ".txt": "txt-text-file-extension-symbol",
        ".in": "txt-text-file-extension-symbol",
        ".uue": "uue-file-format-symbol",
        ".vb": "vb-file-symbol",
        ".vcd": "vcd-file-format-symbol",
        ".vcf": "vcf-file-format-variant",
        ".vob": "vob-file-format-symbol",
        ".wav": "wav-file-format-variant",
        ".wma": "wma-file-format-symbol",
        ".wmv": "wmv-file-format-extension",
        ".wpd": "wpd-file-format-symbol",
        ".wps": "wps-file-format",
        ".wsf": "wsf-file-format-variant",
        ".xhtml": "xhtml-icon-file-format",
        ".xlr": "xlr-file-format-variant",
        ".xls": "xls-file-format-symbol",
        ".xlsx": "xlsx-file-format",
        ".xml": "xml-file-format-variant",
        ".yuv": "yuv-file-format-variant",
        ".zip": "zip-compressed-files-extension",
    }

    def __init__(
        self, is_synced: bool, is_confined: bool, file_name: str, *args: object, **kwargs: object
    ) -> None:
        ext = pathlib.Path(file_name).suffix
        icon = self.EXTENSIONS.get(ext.lower(), "blank-file")
        super().__init__(
            is_synced,
            is_confined,
            f":/file_ext/images/file_formats/{icon}.svg",
            "",
            *args,
            **kwargs,
        )
        self.setData(TYPE_DATA_INDEX, FileType.File)


class FolderTableItem(IconTableItem):
    def __init__(self, is_synced: bool, is_confined: bool, *args: object, **kwargs: object) -> None:
        super().__init__(
            is_synced, is_confined, ":/icons/images/material/folder_open.svg", "", *args, **kwargs
        )
        self.setData(NAME_DATA_INDEX, 0)
        self.setData(TYPE_DATA_INDEX, FileType.Folder)


class InconsistencyTableItem(IconTableItem):
    def __init__(self, is_synced: bool, is_confined: bool, *args: object, **kwargs: object) -> None:
        super().__init__(
            is_synced, is_confined, ":/icons/images/material/warning.svg", "", *args, **kwargs
        )
        self.setData(NAME_DATA_INDEX, 0)
        self.setData(TYPE_DATA_INDEX, FileType.Inconsistency)

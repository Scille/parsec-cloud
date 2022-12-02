# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import gettext
import io

from PyQt5.QtCore import QCoreApplication, QDataStream, QFile, QIODevice, QLocale
from structlog import get_logger

from parsec._parsec import DateTime, LocalDateTime
from parsec.core.config import CoreConfig
from parsec.core.gui.desktop import get_locale_language


LANGUAGES = {"English": "en", "Français": "fr"}

_current_translator = None
_current_locale_language = None

logger = get_logger()


def format_datetime(dt: DateTime | LocalDateTime, full: bool = False, seconds: bool = False) -> str:
    # Handle if it is already a LocalDateTime
    if isinstance(dt, DateTime):
        dt = dt.to_local()
    if _current_locale_language == "en":
        if seconds:
            return dt.format("%d/%m/%Y %H:%M:S")
        return dt.format("%d/%m/%Y %H:%M")
    else:
        if seconds:
            return dt.format("%m/%d/%Y %I:%M:S%p")
        return dt.format("%m/%d/%Y %I:%M%p")


def qt_translate(_: object, string: str) -> str:
    return translate(string)


def translate(string: str) -> str:
    if _current_translator:
        return _current_translator.gettext(string)
    return gettext.gettext(string)


def get_qlocale() -> QLocale:
    assert _current_locale_language is not None
    return QLocale(_current_locale_language)


def switch_language(core_config: CoreConfig, lang_key: str | None = None) -> str | None:
    global _current_translator
    global _current_locale_language

    # Assign our own translation function is valid
    QCoreApplication.translate = qt_translate  # type: ignore[assignment]

    if not lang_key:
        lang_key = core_config.gui_language
    if not lang_key:
        lang_key = get_locale_language()
        logger.info("No language in settings, trying local language", lang_key=lang_key)
    if lang_key not in LANGUAGES.values():
        logger.info("Language unavailable, defaulting to English", lang_key=lang_key)
        lang_key = "en"

    _current_locale_language = lang_key

    rc_file = QFile(f":/translations/translations/parsec_{lang_key}.mo")
    if not rc_file.open(QIODevice.ReadOnly):
        logger.warning("Unable to read the translations for language", lang_key=lang_key)
        return None

    try:
        data_stream = QDataStream(rc_file)
        out_stream = io.BytesIO()
        content = data_stream.readRawData(rc_file.size())
        out_stream.write(content)
        out_stream.seek(0)
        _current_translator = gettext.GNUTranslations(out_stream)
        _current_translator.install()
    except OSError:
        logger.warning("Unable to load the translations for language", lang_key=lang_key)
        return None
    finally:
        rc_file.close()
    return lang_key

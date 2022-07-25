# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import io
import gettext

from structlog import get_logger

from PyQt5.QtCore import QCoreApplication, QIODevice, QFile, QDataStream, QLocale

from parsec.core.gui.desktop import get_locale_language


LANGUAGES = {"English": "en", "Français": "fr"}

_current_translator = None
_current_locale_language = None

logger = get_logger()


def format_datetime(dt, full=False, seconds=False):
    # fmt = "L LT"
    # if seconds:
    #     fmt = "L LTS"
    # if full:
    #     fmt = "LLLL"
    # # The alternative formatter is now the default one since pendulum 2.0.0
    # return dt.in_tz(pendulum.local_timezone()).format(fmt, locale=_current_locale_language)

    # TODO: better support without relying on pytzdata (using strftime ?)
    return str(dt)


def qt_translate(_, string):
    return translate(string)


def translate(string):
    if _current_translator:
        return _current_translator.gettext(string)
    return gettext.gettext(string)


def get_qlocale():
    q = QLocale(_current_locale_language)
    return q


def switch_language(core_config, lang_key=None):
    global _current_translator
    global _current_locale_language

    QCoreApplication.translate = qt_translate

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

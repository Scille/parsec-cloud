from structlog import get_logger

from PyQt5.QtCore import QTranslator, QCoreApplication

from parsec.core.gui.desktop import get_locale_language
from parsec.core.gui import settings


LANGUAGES = {"English": "en", "Français": "fr"}

_current_translator = None

logger = get_logger()


def switch_language(lang_key=None):
    global _current_translator

    if not lang_key:
        lang_key = settings.get_value("global/language")
    if not lang_key:
        lang_key = get_locale_language()
        logger.info("No language in settings, trying local language '{}'".format(lang_key))
    if lang_key not in LANGUAGES.values():
        lang_key = "en"
        logger.warning("Language '{}' unavailable, defaulting to English".format(lang_key))
    translator = QTranslator()
    path = ":/translations/translations/parsec_{}.qm".format(lang_key)
    if not translator.load(path):
        logger.warning("Unable to load the translations for language '{}'".format(lang_key))
        return False
    if not QCoreApplication.installTranslator(translator):
        logger.warning("Failed to install the translator for language '{}'".format(lang_key))
        return False

    QCoreApplication.removeTranslator(_current_translator)
    settings.set_value("global/language", lang_key)
    _current_translator = translator
    QCoreApplication.processEvents()
    return True

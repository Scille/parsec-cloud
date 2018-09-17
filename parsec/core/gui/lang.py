from PyQt5.QtCore import QTranslator, QLocale, QCoreApplication


_LANGUAGES = {"English": "en", "Français": "fr", "Deutsch": "de", "Español": "es", "中文": "zh"}

_current_language = "en"

_current_translator = None


def translate(cls, text):
    return QCoreApplication.translate(cls.__class__.__name__, text)


def switch_to_locale():
    global _current_language
    global _current_translator

    translator = QTranslator()
    locale = QLocale.system().name()[:2].lower()
    tr_file = ":/translations/parsec_{}".format(locale)
    print(
        "Trying to load translation file '{}' for locale {}".format(
            tr_file, QLocale.system().name()
        )
    )
    if not translator.load(tr_file):
        return False
    if QCoreApplication.installTranslator(translator):
        QCoreApplication.removeTranslator(_current_translator)
        _current_translator = translator
        _current_language = locale
        return True
    return False


def switch_to_language_name(lang_name):
    lang_key = _LANGUAGES.get(lang_name)
    return switch_to_language_key(lang_key)


def switch_to_language_key(lang_key):
    global _current_language
    global _current_translator

    if not lang_key:
        return False
    translator = QTranslator()
    translator.load(":/translations/parsec_{}".format(lang_key))
    if QCoreApplication.installTranslator(translator):
        QCoreApplication.removeTranslator(_current_translator)
        _current_language = lang_key
        _current_translator = translator
        return True
    return False


def get_current_language_name():
    for lang_name, lang_key in _LANGUAGES.items():
        if _current_language == lang_key:
            return lang_name
    return "English"


def get_language_name_from_key(lang_key):
    return _LANGUAGES.get(lang_key)


def get_current_language_key():
    return _current_language

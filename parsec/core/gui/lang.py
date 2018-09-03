from PyQt5.QtCore import QTranslator, QLibraryInfo, QLocale, QFile, QCoreApplication


_LANGUAGES = {
    'English': 'en',
    'Français': 'fr',
    'Deutsch': 'de',
    'Español': 'es',
    '中文': 'zh'
}

_CURRENT_LANGUAGE = 'en'


def translate(cls, text):
    return QCoreApplication.translate(cls.__class__.__name__, text)


def switch_to_locale():
    translator = QTranslator()
    locale = QLocale.system().name()[:2]
    translator.load(':/translations/parsec_{}'.format(locale))
    if QCoreApplication.installTranslator(translator):
        _CURRENT_LANGUAGE = locale
        return True
    return False


def switch_to_language_name(lang_name):
    lang_key = _LANGUAGES.get(lang_name)
    return switch_to_language_key(lang_key)


def switch_to_language_key(lang_key):
    if not lang_key:
        return False
    translator = QTranslator()
    translator.load(':/translations/parsec_{}'.format(lang_key))
    if QCoreApplication.installTranslator(translator):
        _CURRENT_LANGUAGE = lang_key
        return True
    return False


def get_current_language_name():
    for lang_name, lang_key in _LANGUAGES.items():
        if _CURRENT_LANGUAGE == lang_key:
            return lang_name
    return 'English'


def get_language_name_from_key(lang_key):
    return _LANGUAGES.get(lang_key)


def get_current_language_key():
    return _CURRENT_LANGUAGE

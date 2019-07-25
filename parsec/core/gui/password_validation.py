# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from zxcvbn import zxcvbn

from parsec.core.gui.lang import translate as _


PASSWORD_CSS = {
    0: "color: white; background-color: rgb(64, 64, 64);",
    1: "color: white; background-color: rgb(194, 51, 51);",
    2: "color: white; background-color: rgb(194, 51, 51);",
    3: "color: white; background-color: rgb(219, 125, 58);",
    4: "color: white; background-color: rgb(55, 130, 65);",
    5: "color: white; background-color: rgb(24, 175, 44);",
}


def get_password_strength_text(password_score):
    PASSWORD_STRENGTH_TEXTS = [
        _("PASSWORD_STR_TOO_SHORT"),
        _("PASSWORD_STR_VERY_WEAK"),
        _("PASSWORD_STR_WEAK"),
        _("PASSWORD_STR_AVERAGE"),
        _("PASSWORD_STR_GOOD"),
        _("PASSWORD_STR_STRONG"),
    ]
    return PASSWORD_STRENGTH_TEXTS[password_score]


def get_password_strength(password):
    if len(password) < 8:
        return 0
    result = zxcvbn(password)
    return result["score"] + 1

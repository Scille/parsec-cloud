# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from zxcvbn import zxcvbn

from parsec.core.gui.lang import translate as _


PASSWORD_STRENGTH_TEXTS = [
    _("Too short"),
    _("Very weak"),
    _("Weak"),
    _("Average"),
    _("Good"),
    _("Strong"),
]

PASSWORD_CSS = {
    0: "color: white; background-color: rgb(64, 64, 64);",
    1: "color: white; background-color: rgb(194, 51, 51);",
    2: "color: white; background-color: rgb(194, 51, 51);",
    3: "color: white; background-color: rgb(219, 125, 58);",
    4: "color: white; background-color: rgb(55, 130, 65);",
}


def get_password_strength(password):
    if len(password) < 8:
        return 0
    result = zxcvbn(password)
    return result["score"] + 1

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

#!/usr/bin/env python

import xml.etree.ElementTree as ET


def get_location(elem):
    loc = elem.find("location")
    return loc.attrib["filename"], int(loc.attrib["line"])


def get_translation_string(elem):
    tr = elem.find("source")
    return tr.text


def extract_qt(fileobj, keywords, comment_tags, options):
    tree = ET.parse(fileobj)
    root = tree.getroot()
    for message in root.iter("message"):
        loc = get_location(message)
        s = get_translation_string(message)
        yield (loc[1], None, s, [])


if __name__ == "__main__":
    with open("parsec/core/gui/tr/parsec_fr.ts") as fd:
        for line, f, s, comment in extract_qt(fd, {}, {}, {}):
            print(line, f, s, comment)

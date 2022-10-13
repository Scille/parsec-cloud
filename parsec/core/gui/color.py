# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import colorsys

from binascii import crc32


class StringToColor:
    def __init__(self, hue, lightness, saturation):
        self.color = (hue, lightness, saturation)

    @property
    def hue(self):
        return self.color[0]

    @property
    def lightness(self):
        return self.color[1]

    @property
    def saturation(self):
        return self.color[2]

    def rgb(self):
        return colorsys.hls_to_rgb(self.color[0], self.color[1], self.color[2])

    def rgb255(self):
        r, g, b = self.rgb()
        return int(255 * r), int(255 * g), int(255 * b)

    def hex(self):
        r, g, b = self.rgb()
        return "#{:x}{:x}{:x}".format(*self.rgb255())

    @classmethod
    def from_string(cls, string):
        hash = crc32(string.encode())
        hue = (hash % 359) / 360
        hash //= 360
        saturation = (hash % 90 + 100) / 240
        hash //= saturation
        lightness = (hash % 50 + 150) / 240
        return cls(hue, lightness, saturation)

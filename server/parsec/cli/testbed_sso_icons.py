# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from base64 import b64decode

# Icons served by the testbed server to mock the SSO providers' icons
# (see `/testbed/mock/sso/icon/{name}` route).
# Both a SVG and a PNG are provided to exercise the different image formats.

PRO_CONNECT_ICON_SVG = b"""\
<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 64 64">
  <rect width="64" height="64" fill="#D71920"/>
  <text x="32" y="47" text-anchor="middle" font-family="DejaVu Sans, Arial, sans-serif" font-size="44" font-weight="700" fill="#FFFFFF">P</text>
</svg>
"""

# cspell: disable
HEXAGONE_ICON_PNG = b64decode(
    "iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAIAAAAlC+aJAAAAxElEQVR4nO3aPQrCQBQAYSOCQjqx"
    "svYcHsDGLkcIeAM729Rpcgdbj2FjayVYWFlapPQA/uEmMDyYrwzsssODkIVkk/I8iGxIH6ArA2gG"
    "0AygGUAzgGYALXzAKG3ZvV7k4zfxRXM7nB7f125X09169vr8eGmX1fXfk4SfgAE0A2gG0AygGUBL"
    "/Bb6ZL+Z97vhT+EnYADNAFrPb6EuN7I04SdgAM0AmgE0A2gG0AygZf5uAzOAZgDNAJoBNANoBtAM"
    "oIUPeALuxhZAOsWsCwAAAABJRU5ErkJggg=="
)
# cspell: enable

# Icon name -> (content, media type)
TESTBED_SSO_ICONS: dict[str, tuple[bytes, str]] = {
    "pro_connect": (PRO_CONNECT_ICON_SVG, "image/svg+xml"),
    "hexagone": (HEXAGONE_ICON_PNG, "image/png"),
}

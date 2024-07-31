# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import platform
import sys

print(f"{sys.platform=}", file=sys.stderr)

if sys.platform == "linux":
    platform_info = platform.freedesktop_os_release()
    # VERSION_ID is not available in all platforms
    release = platform_info.get("VERSION_ID", "unknown")
    os = platform_info["ID"]
elif sys.platform == "darwin":
    platform_info = platform.mac_ver()
    release = platform_info[0]
    os = "macos"
elif sys.platform == "win32":
    platform_info = platform.win32_ver()
    release = platform_info[0]
    os = "windows"
else:
    raise ValueError(f"Unsupported platform {sys.platform}")

print(f"{platform_info=}", file=sys.stderr)
print(f"{os=} {release=}", file=sys.stderr)

id = f"{os}-{release}"
print(f"{id=}", file=sys.stderr)

print(f"platform={sys.platform}")
print(f"{id=}")
print(f"{os=}")
print(f"{release=}")
print(f"arch={platform.machine()}")

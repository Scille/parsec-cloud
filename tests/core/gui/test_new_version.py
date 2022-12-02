# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.core.gui.new_version import Version, do_check_new_version


@pytest.mark.trio
@pytest.mark.parametrize(
    "data",
    [
        # Basic Window 64bits
        {
            "current_version": "1.8.0",
            "platform": "win32",
            "arch": "x86_64",
            "json_releases": ["v1.9.0", "v1.8.0", "v1.7.0"],
            "expected": (
                Version("1.9.0"),
                "https://api.com/releases/v1.9.0/parsec-cloud-v1.9.0-win64-setup.exe",
            ),
        },
        # Basic Window 32bits
        {
            "current_version": "1.8.0",
            "platform": "win32",
            "arch": "i386",
            "json_releases": ["v1.9.0", "v1.8.0", "v1.7.0"],
            "expected": (
                Version("1.9.0"),
                "https://api.com/releases/v1.9.0/parsec-cloud-v1.9.0-win32-setup.exe",
            ),
        },
        # Basic macOS 64bits
        {
            "current_version": "1.8.0",
            "platform": "darwin",
            "arch": "x86_64",
            "json_releases": ["v1.9.0", "v1.8.0", "v1.7.0"],
            "expected": (
                Version("1.9.0"),
                "https://api.com/releases/v1.9.0/parsec-cloud-v1.9.0-macos-amd64.dmg",
            ),
        },
        # No compatible release
        {
            "current_version": "1.8.0",
            "json_releases": [
                {"tag_name": "v1.9.0", "assets": ["macos-amd64.dmg"]},
                {"tag_name": "v1.8.0", "assets": ["macos-amd64.dmg"]},
                {"tag_name": "v1.7.0", "assets": ["macos-amd64.dmg"]},
            ],
            "expected": None,
        },
        # No releases at all
        {"current_version": "1.8.0", "json_releases": [], "expected": None},
        # Current version is the last one
        {
            "current_version": "1.9.0",
            "json_releases": ["v1.9.0", "v1.8.0", "v1.7.0"],
            "expected": None,
        },
        #  Current version is a development one
        {
            "current_version": "1.9.0+dev",
            "json_releases": ["v1.9.0", "v1.8.0", "v1.7.0"],
            "expected": None,
        },
        #  Current version is an old development one
        {
            "current_version": "1.8.0+dev",
            "json_releases": ["v1.9.0", "v1.8.0", "v1.7.0"],
            "expected": (
                Version("1.9.0"),
                "https://api.com/releases/v1.9.0/parsec-cloud-v1.9.0-win64-setup.exe",
            ),
        },
        # Latest version is not compatible with our platform
        {
            "current_version": "1.8.0",
            "platform": "darwin",
            "json_releases": [
                {"tag_name": "v1.9.0", "assets": ["win64-setup.exe"]},
                "v1.8.0",
                "v1.7.0",
            ],
            "expected": None,
        },
        # Latest version is not compatible with our arch
        {
            "current_version": "1.8.0",
            "arch": "x86_64",
            "json_releases": [
                {"tag_name": "v1.9.0", "assets": ["win32-setup.exe"]},
                "v1.8.0",
                "v1.7.0",
            ],
            "expected": None,
        },
        # Latest version is not compatible with our platform, but a newer version is available nevertheless
        {
            "current_version": "1.8.0",
            "platform": "win32",
            "json_releases": [
                "v1.9.1",
                {"tag_name": "v1.9.0", "assets": ["macos-amd64.dmg"]},
                "v1.8.0",
                "v1.7.0",
            ],
            "expected": (
                Version("1.9.1"),
                "https://api.com/releases/v1.9.1/parsec-cloud-v1.9.1-win64-setup.exe",
            ),
        },
        # Current version is not available in the releases for our platform
        {
            "current_version": "1.8.0",
            "platform": "win32",
            "json_releases": [{"tag_name": "v1.8.0", "assets": ["macos-amd64.dmg"]}, "v1.7.0"],
            "expected": None,
        },
        # Ignore prerelease by default...
        {
            "current_version": "1.8.0",
            "json_releases": [{"tag_name": "v1.9.0", "prerelease": True}, "v1.8.0", "v1.7.0"],
            "expected": None,
        },
        # ...but can handle them if needed
        {
            "current_version": "1.8.0",
            "json_releases": [{"tag_name": "v1.9.0", "prerelease": True}, "v1.8.0", "v1.7.0"],
            "allow_prerelease": True,
            "expected": (
                Version("1.9.0"),
                "https://api.com/releases/v1.9.0/parsec-cloud-v1.9.0-win64-setup.exe",
            ),
        },
        # Ignore draft no matter what
        {
            "current_version": "1.8.0",
            "json_releases": [{"tag_name": "v1.9.0", "draft": True}, "v1.8.0", "v1.7.0"],
            "expected": None,
        },
        # Ignore draft no matter what
        {
            "current_version": "1.8.0",
            "json_releases": [{"tag_name": "v1.9.0", "draft": True}, "v1.8.0", "v1.7.0"],
            "allow_prerelease": True,
            "expected": None,
        },
        # Ignore invalid tag names
        {
            "current_version": "1.8.0",
            "json_releases": [{"tag_name": "dummy"}, "v1.8.0"],
            "allow_prerelease": True,
            "expected": None,
        },
        # Invalid tag names
        {
            "current_version": "1.8.0",
            "json_releases": [{"tag_name": "dummy"}, "v1.9.0", "v1.8.0"],
            "allow_prerelease": True,
            "expected": (
                Version("1.9.0"),
                "https://api.com/releases/v1.9.0/parsec-cloud-v1.9.0-win64-setup.exe",
            ),
        },
    ],
)
async def test_check_new_version(monkeypatch, data):
    assert not (
        data.keys()
        - {"current_version", "platform", "arch", "json_releases", "allow_prerelease", "expected"}
    )  # Sanity check

    DEFAULT_PLATFORM = "win32"
    DEFAULT_ARCH = "x86_64"
    DEFAULT_RELEASE_ASSETS = ["win64-setup.exe", "win32-setup.exe", "macos-amd64.dmg"]

    from parsec.core.gui import new_version as new_version_mod

    monkeypatch.setattr(new_version_mod, "__version__", data["current_version"])

    platform = data.get("platform", DEFAULT_PLATFORM)
    arch = data.get("arch", DEFAULT_ARCH)
    monkeypatch.setattr(new_version_mod, "get_platform_and_arch", lambda: (platform, arch))

    def _cook_json_release(json_release):
        if isinstance(json_release, str):
            json_release = {"tag_name": json_release}
        assert not (
            json_release.keys() - {"tag_name", "prerelease", "draft", "assets"}
        )  # Sanity check
        tag_name = json_release["tag_name"]
        cooked_assets = []
        for asset in json_release.get("assets", DEFAULT_RELEASE_ASSETS):
            name = f"parsec-cloud-{tag_name}-{asset}"
            cooked_assets.append(
                {
                    "name": name,
                    "browser_download_url": f"https://api.com/releases/{tag_name}/{name}",
                }
            )
        return {
            "tag_name": tag_name,
            "draft": json_release.get("draft", False),
            "prerelease": json_release.get("prerelease", False),
            "assets": cooked_assets,
        }

    json_releases = [_cook_json_release(json_release) for json_release in data["json_releases"]]

    async def _mocked_fetch_json_releases(api_url):
        return json_releases

    monkeypatch.setattr(new_version_mod, "fetch_json_releases", _mocked_fetch_json_releases)

    result = await do_check_new_version(
        api_url="https://api.com/releases", allow_prerelease=data.get("allow_prerelease", False)
    )
    assert result == data["expected"]


@pytest.mark.trio
async def test_check_new_version_offline(monkeypatch):
    from parsec.core.gui import new_version as new_version_mod

    async def _mocked_new_version_mod(api_url):
        return None

    monkeypatch.setattr(new_version_mod, "fetch_json_releases", _mocked_new_version_mod)

    result = await do_check_new_version(api_url="https://api.com/releases")
    assert result is None


@pytest.mark.trio
@pytest.mark.parametrize(
    "payload",
    [
        {},
        [{}],
        [{"assets": [], "draft": "foo", "prerelease": False, "version": "v1.9.0"}],
        [{"assets": [], "draft": False, "prerelease": False, "version": "<no_a_version>"}],
        [
            {
                "assets": [
                    {
                        "browser_download_url": "https://api.com/releases/v1.9.0/parsec-cloud-v1.9.0-win64-setup.exe",
                        "name": "parsec-cloud-v1.9.0-win64-setup.exe",
                    }
                ],
                "draft": False,
                "prerelease": False,
                "version": "<no_a_version>",
            }
        ],
        [
            {
                "assets": [
                    {
                        # Missing browser_download_url field
                        "name": "parsec-cloud-v1.9.0-win64-setup.exe"
                    }
                ],
                "draft": False,
                "prerelease": False,
                "version": "v1.9.0",
            }
        ],
    ],
)
async def test_check_new_version_bad_api_payload(monkeypatch, caplog, payload):
    from parsec.core.gui import new_version as new_version_mod

    async def _mocked_new_version_mod(api_url):
        return payload

    monkeypatch.setattr(new_version_mod, "fetch_json_releases", _mocked_new_version_mod)

    result = await do_check_new_version(api_url="https://api.com/releases")
    assert result is None
    caplog.assert_occured_once(
        "[error    ] Cannot load releases info from API [parsec.core.gui.new_version]"
    )

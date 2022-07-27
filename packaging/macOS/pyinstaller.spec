# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os

SRC_DIR = os.getenv("SRC_DIR", default="../..")

# Awesome hack to load `__version__`
__version__ = None
exec(open(f"{SRC_DIR}/parsec/_version.py", encoding="utf-8").read())

a = Analysis(
    ['launch_script.py'],
    pathex=[os.path.dirname(os.path.abspath('parsec.spec'))],
    binaries=[],
    datas=[
        (f'{SRC_DIR}/parsec/core/resources/*.ignore', 'parsec/core/resources'),
        (f'{SRC_DIR}/parsec/core/resources/*.icns', 'parsec/core/resources'),
        (f'{SRC_DIR}/parsec/core/resources/default_pattern.ignore', 'parsec/core/resources'),
    ],
    hiddenimports=[],
    hookspath=['my_hooks'],
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=None,
    noarchive=False
)

pyz = PYZ(a.pure, a.zipped_data, cipher=None)

exe = EXE(
    pyz,
    a.scripts,
    [],
    exclude_binaries=True,
    name="parsec-gui",
    icon=f"{SRC_DIR}/parsec/core/resources/parsec.icns",
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    console=False
)

coll = COLLECT(
    exe,
    a.binaries,
    a.zipfiles,
    a.datas,
    strip=False,
    upx=True,
    upx_exclude=[],
    name="parsec"
)

app = BUNDLE(
    coll,
    name='parsec.app',
    icon=f'{SRC_DIR}/parsec/core/resources/parsec.icns',
    bundle_identifier='com.scille.parsec',
    info_plist={
        'NSPrincipalClass': 'NSApplication',
        'NSHighResolutionCapable': True,
        'NSAppleScriptEnabled': False,
        'CFBundleIdentifier': 'com.scille.parsec',
        'CFBundleName': 'Parsec',
        'CFBundleDisplayName': 'Parsec',
        'CFBundleShortVersionString': __version__,
        'CFBundleURLTypes': [
            {
                'CFBundleTypeRole': 'Shell',
                'CFBundleURLName': 'com.scille.parsec',
                'CFBundleURLSchemes': ['parsec']
            }
        ],
        # Encoding is specified here to force UTF-8 when launching app instead of
        # default ascii, which caused issues with click
        'LSEnvironment': {
            'LANG': 'en_US.UTF-8',
            'LC_CTYPE': 'en_US.UTF-8'
        },
    },
)

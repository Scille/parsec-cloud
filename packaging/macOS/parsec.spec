# -*- mode: python ; coding: utf-8 -*-

import os

a = Analysis(['launch_script.py'],
             pathex=[os.path.dirname(os.path.abspath('parsec.spec'))],
             binaries=[],
             datas=[
                ('../../parsec/core/resources/*.ignore', 'parsec/core/resources'),
                ('../../parsec/core/resources/*.icns', 'parsec/core/resources')
                ],
             hiddenimports=[],
             hookspath=['my_hooks'],
             runtime_hooks=[],
             excludes=[],
             win_no_prefer_redirects=False,
             win_private_assemblies=False,
             cipher=None,
             noarchive=False)
pyz = PYZ(a.pure, a.zipped_data,
             cipher=None)
exe = EXE(pyz,
          a.scripts,
          a.binaries,
          a.zipfiles,
          a.datas,
          [],
          name='parsec',
          debug=False,
          bootloader_ignore_signals=False,
          strip=False,
          upx=True,
          console=False )
app = BUNDLE(exe,
             name='parsec.app',
             icon='../../parsec/core/resources/parsec.icns',
             bundle_identifier='fr.scille.parsec',
             info_plist={
                'NSPrincipalClass': 'NSApplication',
                'NSHighResolutionCapable': True,
                'NSAppleScriptEnabled': False,
                'CFBundleIdentifier': 'fr.scille.parsec',
                'CFBundleName': 'parsec',
                'CFBundleDisplayName': 'Parsec',
                'CFBundleURLTypes': [
                   {
                      'CFBundleURLName': 'parsec',
                      'CFBundleURLSchemes': ['parsec']
                   }
                ]
             },
            )

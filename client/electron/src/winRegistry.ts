// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { join } from 'path';
import { electronIsDev } from './utils';

export class WinRegistry {
  private regedit: any = null;
  private APP_GUID: string = '';

  constructor(appGuid: string, appPath: string) {
    this.APP_GUID = appGuid;
    if (process.platform === 'win32') {
      try {
        const reg = require('regedit');
        if (!electronIsDev) {
          const vbsDirectory = join(appPath, '../vbs');
          reg.setExternalVBSLocation(vbsDirectory);
        }
        this.regedit = reg.promisified;
      } catch (error: any) {
        console.error('Failed to init regedit', error);
      }
    }
  }

  async areLongPathsEnabled(): Promise<boolean> {
    try {
      if (process.platform !== 'win32' || !this.regedit) {
        return true;
      }
      const KEY = 'HKLM\\SYSTEM\\CurrentControlSet\\Control\\FileSystem';
      const values = await this.regedit.list([KEY]);
      const longPathValue = values[KEY].values['LongPathsEnabled'];
      if (!longPathValue) {
        console.warn('`LongPathsEnabled` was not found in registry.');
        return false;
      }
      if (longPathValue.type !== 'REG_DWORD') {
        console.warn('`LongPathsEnabled`has incorrect type.');
        return false;
      }
      console.debug(`'${KEY}\\LongPathsEnabled' has value ${longPathValue.value}`);
      return (longPathValue.value as any as number) === 1;
    } catch (e: any) {
      console.error(`Error while trying to obtain 'LongPathsEnabled' value: ${e.toString()}`);
    }
  }

  async addMountpointToQuickAccess(mountpointPath: string, iconPath: string): Promise<void> {
    try {
      if (process.platform !== 'win32' || !this.regedit) {
        return;
      }
      await this.removeMountpointFromQuickAccess();

      const baseKey1 = `HKCU\\Software\\Classes\\CLSID\\{${this.APP_GUID}}`;
      const baseKey2 = `HKCU\\Software\\Classes\\Wow6432Node\\CLSID\\{${this.APP_GUID}}`;
      const systemRoot = process.env.SYSTEMROOT || 'C:\\Windows';

      for (const key of [baseKey1, baseKey2]) {
        await this.regedit.createKey([key]);
        await this.regedit.putValue({
          [key]: {
            AppName: {
              value: 'Parsec',
              type: 'REG_DEFAULT',
            },
            SortOrderIndex: {
              value: 0x42,
              type: 'REG_DWORD',
            },
            'System.IsPinnedToNamespaceTree': {
              value: 0x01,
              type: 'REG_DWORD',
            },
          },
        });
        await this.regedit.createKey([`${key}\\DefaultIcon`]);
        await this.regedit.putValue({
          [`${key}\\DefaultIcon`]: {
            IconPath: {
              value: iconPath,
              type: 'REG_DEFAULT',
            },
          },
        });
        await this.regedit.createKey([`${key}\\InProcServer32`]);
        await this.regedit.putValue({
          [`${key}\\InProcServer32`]: {
            IconPath: {
              value: `${systemRoot}\\system32\\shell32.dll`,
              type: 'REG_DEFAULT',
            },
          },
        });
        await this.regedit.createKey([`${key}\\Instance`]);
        await this.regedit.putValue({
          [`${key}\\Instance`]: {
            CLSID: {
              value: '{0E5AAE11-A475-4c5b-AB00-C66DE400274E}',
              type: 'REG_SZ',
            },
          },
        });
        await this.regedit.createKey([`${key}\\Instance\\InitPropertyBag`]);
        await this.regedit.putValue({
          [`${key}\\Instance\\InitPropertyBag`]: {
            Attributes: {
              value: 0x11,
              type: 'REG_DWORD',
            },
            TargetFolderPath: {
              value: mountpointPath,
              type: 'REG_SZ',
            },
          },
        });

        await this.regedit.createKey([`${key}\\ShellFolder`]);
        await this.regedit.putValue({
          [`${key}\\ShellFolder`]: {
            Attributes: {
              value: 0xf080004d,
              type: 'REG_DWORD',
            },
            FolderValueFlags: {
              value: 0x28,
              type: 'REG_DWORD',
            },
          },
        });
      }

      await this.regedit.createKey(`HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{${this.APP_GUID}}`);
      await this.regedit.putValue({
        [`HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{${this.APP_GUID}}`]: {
          AppName: {
            value: 'Parsec',
            type: 'REG_DEFAULT',
          },
        },
      });

      await this.regedit.putValue({
        'HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\HideDesktopIcons\\NewStartPanel': {
          [`{${this.APP_GUID}}`]: {
            value: 0x1,
            type: 'REG_DWORD',
          },
        },
      });
    } catch (error: any) {
      console.error('Failed to add mountpoint link to sidebar', error);
    }
  }

  async removeMountpointFromQuickAccess(): Promise<void> {
    async function _silentDeleteValues(regedit: any, values: string[]): Promise<void> {
      try {
        await regedit.deleteValue(values);
      } catch (_err) {}
    }

    async function _silentDeleteKeys(regedit: any, keys: string[]): Promise<void> {
      try {
        await regedit.deleteKey(keys);
      } catch (_err) {}
    }

    if (process.platform !== 'win32' || !this.regedit) {
      return;
    }
    const baseKey1 = `HKCU\\Software\\Classes\\CLSID\\{${this.APP_GUID}}`;
    const baseKey2 = `HKCU\\Software\\Classes\\Wow6432Node\\CLSID\\{${this.APP_GUID}}`;

    for (const key of [baseKey1, baseKey2]) {
      await _silentDeleteValues(this.regedit, [
        `${key}\\SortOrderIndex`,
        `${key}\\System.IsPinnedToNamespaceTree`,
        `${key}\\Instance\\CLSID`,
        `${key}\\Instance\\InitPropertyBag\\Attributes`,
        `${key}\\Instance\\InitPropertyBag\\TargetFolderPath`,
        `${key}\\ShellFolder\\Attributes`,
        `${key}\\ShellFolder\\FolderValueFlags`,
        `${key}\\Instance\\InitPropertyBag`,
        `${key}\\Instance`,
      ]);
      await _silentDeleteKeys(this.regedit, [`${key}\\DefaultIcon`, `${key}\\InProcServer32`, `${key}\\ShellFolder`, key]);
    }

    await _silentDeleteValues(this.regedit, [
      `HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{${this.APP_GUID}}`,
    ]);
    await _silentDeleteKeys(this.regedit, [
      `HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Desktop\\NameSpace\\{${this.APP_GUID}}`,
    ]);
    await _silentDeleteValues(this.regedit, [
      `HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\HideDesktopIcons\\NewStartPanel\\{${this.APP_GUID}}`,
    ]);
  }
}

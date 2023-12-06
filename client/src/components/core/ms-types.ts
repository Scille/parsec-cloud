// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export enum MsReportTheme {
  Info = 'ms-info',
  Success = 'ms-success',
  Warning = 'ms-warning',
  Error = 'ms-error',
}

export enum MsModalResult {
  Cancel = 'cancel',
  Confirm = 'confirm',
}

interface MsOption {
  label: string;
  description?: string;
  key: any;
  disabled?: boolean;
}

export function getMsOptionByKey(options: MsOption[], key: any): MsOption | undefined {
  return options.find((option) => {
    return option.key === key;
  });
}

export type MsDropdownOption = MsOption;
export type MsSorterOption = MsOption;

export interface MsSorterLabels {
  asc: string;
  desc: string;
}
export interface MsDropdownChangeEvent {
  option: MsDropdownOption;
}

export interface MsSorterChangeEvent {
  option: MsSorterOption;
  sortByAsc: boolean;
}

export enum MsImageExtension {
  Svg = 'svg',
  Png = 'png',
  Webp = 'webp',
}

export enum MsImages {
  CaretExpand = 'caret-expand',
  Device = 'device',
  FileImport = 'file-import',
  LogoIconGradient = 'logo-icon-gradient',
  LogoRowWhite = 'logo-row-white',
  PasswordLock = 'password-lock',
  SwapArrows = 'swap-arrows',
  WavyCaretUp = 'wavy-caret-up',
}

export interface MsImageResource {
  name: MsImages;
  extensions: Array<MsImageExtension>;
}

export const msImageResourceMap: Map<MsImages, MsImageResource> = new Map([
  [
    MsImages.CaretExpand,
    {
      name: MsImages.CaretExpand,
      extensions: [MsImageExtension.Svg],
    },
  ],
  [
    MsImages.Device,
    {
      name: MsImages.Device,
      extensions: [MsImageExtension.Svg],
    },
  ],
  [
    MsImages.FileImport,
    {
      name: MsImages.FileImport,
      extensions: [MsImageExtension.Svg],
    },
  ],
  [
    MsImages.LogoIconGradient,
    {
      name: MsImages.LogoIconGradient,
      extensions: [MsImageExtension.Svg, MsImageExtension.Webp],
    },
  ],
  [
    MsImages.LogoRowWhite,
    {
      name: MsImages.LogoRowWhite,
      extensions: [MsImageExtension.Svg, MsImageExtension.Webp],
    },
  ],
  [
    MsImages.PasswordLock,
    {
      name: MsImages.PasswordLock,
      extensions: [MsImageExtension.Svg],
    },
  ],
  [
    MsImages.SwapArrows,
    {
      name: MsImages.SwapArrows,
      extensions: [MsImageExtension.Svg],
    },
  ],
  [
    MsImages.WavyCaretUp,
    {
      name: MsImages.WavyCaretUp,
      extensions: [MsImageExtension.Svg],
    },
  ],
]);

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export interface CustomPublishOptions {
  provider: 'custom';
  /** The name of the repository */
  repo: string;
  /** The owner of the repository */
  owner: string;
  /** The machine arch the electron-builder is running on */
  buildMachineArch: string;
}

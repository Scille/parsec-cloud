// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export interface CustomPublishOptions {
  readonly provider: 'custom';
  /** The machine arch the electron-builder is running on */
  readonly buildMachineArch: string;
  nightlyBuild: boolean;
  /**
   * The repository name.
   */
  readonly repo: string;
  /**
   * The owner.
   */
  readonly owner: string;
  /**
   * The host (including the port if need).
   * @default github.com
   */
  readonly host?: string | null;
  /**
   * The channel.
   * @default latest
   */
  readonly channel?: string | null;
  /**
   * The type of release. By default `draft` release will be created.
   *
   * Also you can set release type using environment variable.
   * If `EP_DRAFT`is set to `true` — `draft`, if `EP_PRE_RELEASE`is set to `true` — `prerelease`.
   * @default draft
   */
  releaseType?: 'draft' | 'prerelease' | 'release' | null;
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export interface MsModalConfig {
  title: string,
  subtitle?: string,
  closeButtonEnabled: boolean,
  cancelButton?: {
    disabled: boolean,
    label: string,
    // eslint-disable-next-line @typescript-eslint/ban-types
    onClick?: Function,
  },
  confirmButton?: {
    disabled: boolean,
    label: string,
    // eslint-disable-next-line @typescript-eslint/ban-types
    onClick?: Function,
  },
}

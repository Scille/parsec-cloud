// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Translatable } from 'megashark-lib';

interface MenuAction {
  icon?: string;
  image?: string;
  label: Translatable;
  action: any;
  danger?: boolean;
}

export { MenuAction };

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Query, Routes } from '@/router';
import { Translatable } from 'megashark-lib';

export interface RouterPathNode {
  id: number;
  display?: string;
  title?: Translatable;
  icon?: string;
  popoverIcon?: string;
  route: Routes;
  params?: object;
  query?: Query;
}

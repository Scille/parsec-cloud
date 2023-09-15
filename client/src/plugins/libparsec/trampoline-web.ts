// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/* eslint-disable @typescript-eslint/ban-ts-comment */

// @ts-ignore
// eslint-disable-next-line camelcase
import init_module from 'libparsec_bindings_web';
// @ts-ignore
import * as module from 'libparsec_bindings_web';

export async function LoadWebLibParsecPlugin(): Promise<any> {
  await init_module();
  module.initLogger();
  return module;
}

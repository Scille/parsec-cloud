// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

export async function LoadWebLibParsecPlugin(): Promise<any> {
  throw new Error('Native build is not supposed to load the web version of LibparsecPlugin !');
}

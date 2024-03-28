// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

function fromObject(obj: object): string {
  return btoa(JSON.stringify(obj));
}

function toObject(data: string): object {
  return JSON.parse(atob(data));
}

export const Base64 = {
  fromObject,
  toObject,
};

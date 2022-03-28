// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

export interface LibParsecPlugin {
  submitJob(options: {cmd: string, payload: string}): Promise<{ value: string }>;
}

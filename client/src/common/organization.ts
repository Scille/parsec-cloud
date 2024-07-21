// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export function generateTrialOrganizationName(userEmail: string): string {
  const timestamp = new Date().valueOf().toString();
  const part = userEmail.slice(0, 11).replaceAll(/[^a-zA-Z0-9-_]/g, '_');

  return `trial-${part}-${timestamp}`;
}

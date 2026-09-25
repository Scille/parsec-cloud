// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export enum LocalNetworkAccessPermission {
  Granted = 'granted',
  Prompt = 'prompt',
  Denied = 'denied',
  Unknown = 'unknown',
}

export async function getLocalNetworkAccessPermissions(): Promise<LocalNetworkAccessPermission> {
  try {
    const status = await navigator.permissions.query({ name: 'loopback-network' as PermissionName });
    return status.state as LocalNetworkAccessPermission;
  } catch {
    return LocalNetworkAccessPermission.Unknown;
  }
}

export async function promptLocalNetworkAccessPermissions(): Promise<void> {
  try {
    window.nativeAPI.log('debug', 'Prompting access to local network');
    await fetch('http://127.0.0.1', { targetAddressSpace: 'loopback' } as any);
  } catch {}
}

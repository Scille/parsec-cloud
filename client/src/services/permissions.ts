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

function* generateUrl(protocol: 'http:' | 'https:'): Generator<string> {
  const startingPort = protocol === 'http:' ? 41231 : 41232;
  const endPort = protocol === 'http:' ? 41321 : 41322;

  for (let port = startingPort; port <= endPort; port += 10) {
    yield `${protocol}//127.0.0.1:${port}`;
  }
}

export async function promptLocalNetworkAccessPermissions(): Promise<void> {
  window.nativeAPI.log('debug', 'Prompting access to local network');

  if (window.location.protocol !== 'http:' && window.location.protocol !== 'https:') {
    window.nativeAPI.log('error', `Invalid protocol for location: ${window.location.protocol}`);
    return;
  }

  for (const url of generateUrl(window.location.protocol)) {
    try {
      const req = await fetch(url, { targetAddressSpace: 'loopback' } as any);
      if (req.ok) {
        window.nativeAPI.log('debug', `Found a service running on ${url}`);
        break;
      }
    } catch {}
  }
}

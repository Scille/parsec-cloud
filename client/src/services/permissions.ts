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

  const controller = new AbortController();
  try {
    const url = await Promise.any(
      Array.from(generateUrl(window.location.protocol), async (url) => {
        const resp = await fetch(url, { signal: controller.signal, targetAddressSpace: 'loopback' } as any);
        if (!resp.ok) {
          throw new Error(`Service on ${url} answered with status ${resp.status}`);
        }
        return url;
      }),
    );
    window.nativeAPI.log('debug', `Found a service running on ${url}`);
  } catch {
    window.nativeAPI.log('debug', 'No service found running on localhost');
  } finally {
    controller.abort();
  }
}

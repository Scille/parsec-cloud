// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getServerConfig, isElectron, OpenBaoAuthConfigTag } from '@/parsec';
import { getRouter } from '@/router';
import axios, { AxiosInstance } from 'axios';

enum OpenBaoErrorType {
  HTTPError = 'http-error',
  NetworkError = 'network-error',
  InitError = 'init-error',
  NotAvailable = 'not-available',
  PopupFailed = 'popup-failed',
  DataError = 'invalid-data',
}

interface OpenBaoError {
  httpStatus?: number;
  type: OpenBaoErrorType;
  detail?: string;
  errors?: Array<string>;
}

export interface OpenBaoConnectionInfo {
  userId: string;
  token: string;
  server: string;
  provider: OpenBaoAuthConfigTag;
  secretMountpoint: string;
}

type OpenBaoResult<T> = { ok: true; value: T } | { ok: false; error: OpenBaoError };

export class OpenBaoClient {
  _client: AxiosInstance;
  _userId: string;
  _token: string;
  _server: string;
  _provider: OpenBaoAuthConfigTag;
  _mountpoint: string;
  _secretMountpoint: string;

  constructor(token: string, id: string, server: string, provider: OpenBaoAuthConfigTag, mountpoint: string, secretMountpoint: string) {
    this._server = server;
    this._client = axios.create({
      baseURL: this._server,
      timeout: 5000,
    });
    this._client.defaults.headers.common['X-Vault-Token'] = token;
    this._token = token;
    this._userId = id;
    this._provider = provider;
    this._mountpoint = mountpoint;
    this._secretMountpoint = secretMountpoint;
  }

  getConnectionInfo(): OpenBaoConnectionInfo {
    return {
      userId: this._userId,
      token: this._token,
      server: this._server,
      secretMountpoint: this._secretMountpoint,
      provider: this._provider,
    };
  }

  private async _store(key: string, data: object): Promise<OpenBaoResult<undefined>> {
    try {
      const path = `${this._secretMountpoint}/${this._userId}/${key}`;
      await this._client.post(
        path,
        { data: data },
        {
          validateStatus: (status) => {
            return status === 200;
          },
        },
      );
      return { ok: true, value: undefined };
    } catch (err: any) {
      if (err.response) {
        return {
          ok: false,
          error: { type: OpenBaoErrorType.HTTPError, httpStatus: err.response.status, errors: err.response.data.errors },
        };
      } else {
        return { ok: false, error: { type: OpenBaoErrorType.NetworkError, detail: err.toString() } };
      }
    }
  }

  private async _retrieve(key: string): Promise<OpenBaoResult<any>> {
    try {
      const path = `${this._secretMountpoint}/${this._userId}/${key}`;
      const response = await this._client.get(path, {
        validateStatus: (status) => {
          return status === 200;
        },
      });
      return { ok: true, value: response.data.data.data };
    } catch (err: any) {
      if (err.response) {
        return {
          ok: false,
          error: { type: OpenBaoErrorType.HTTPError, httpStatus: err.response.status, errors: err.response.data.errors },
        };
      } else {
        return { ok: false, error: { type: OpenBaoErrorType.NetworkError, detail: err.toString() } };
      }
    }
  }
}

async function getConnectionUrl(openBaoServer: string, mountpoint: string): Promise<OpenBaoResult<string>> {
  try {
    const callbackPath = getRouter().resolve('/oidc/callback').href;
    let host!: string;
    if (isElectron()) {
      // Fixed host, this is filtered by the SSO. It doesn't have to exist, Electron catches the request.
      // If changed, don't forget to also change it in electron
      host = 'https://callback.parsec.cloud.invalid';
    } else {
      host = window.location.origin;
    }

    const redirect = new URL(callbackPath, host);
    const client = axios.create({ baseURL: openBaoServer, timeout: 5000 });
    // eslint-disable-next-line camelcase
    const resp = await client.post(`/v1/${mountpoint}/oidc/auth_url`, { role: 'default', redirect_uri: redirect.toString() });
    if (!resp.data.data.auth_url) {
      return {
        ok: false,
        error: { type: OpenBaoErrorType.DataError, httpStatus: resp.status, errors: ['Missing `auth_url` in response'] },
      };
    }
    return { ok: true, value: resp.data.data.auth_url };
  } catch (err: any) {
    if (err.response) {
      return { ok: false, error: { type: OpenBaoErrorType.HTTPError, httpStatus: err.response.status, errors: err.response.data.errors } };
    } else {
      return { ok: false, error: { type: OpenBaoErrorType.NetworkError, detail: err.toString() } };
    }
  }
}

async function getToken(
  code: string,
  state: string,
  openBaoServer: string,
  mountpoint: string,
): Promise<OpenBaoResult<{ token: string; id: string }>> {
  try {
    const client = axios.create({ baseURL: openBaoServer, timeout: 5000 });
    const resp = await client.get(`/v1/${mountpoint}/oidc/callback`, { params: { code: code, state: state } });
    if (!resp.data.auth.client_token || !resp.data.auth.entity_id) {
      return {
        ok: false,
        error: { type: OpenBaoErrorType.DataError, httpStatus: resp.status, errors: ['Missing `client_token` or `entity_id` in response'] },
      };
    }
    return { ok: true, value: { token: resp.data.auth.client_token, id: resp.data.auth.entity_id } };
  } catch (err: any) {
    if (err.response) {
      return { ok: false, error: { type: OpenBaoErrorType.HTTPError, httpStatus: err.response.status, errors: err.response.data.errors } };
    } else {
      return { ok: false, error: { type: OpenBaoErrorType.NetworkError, detail: err.toString() } };
    }
  }
}

async function isOpenBaoAvailable(parsecServer: string): Promise<boolean> {
  const config = await getServerConfig(parsecServer);
  return config.ok && config.value.openbao !== null;
}

async function openBaoConnect(
  openBaoServer: string,
  provider: OpenBaoAuthConfigTag,
  mountpoint: string,
  secretMountpoint: string,
): Promise<OpenBaoResult<OpenBaoClient>> {
  const connResult = await getConnectionUrl(openBaoServer, mountpoint);
  if (!connResult.ok) {
    window.electronAPI.log('error', `Failed to retrieve SSO auth url: ${JSON.stringify(connResult.error)}`);
    return connResult;
  }
  let resultPromise!: Promise<{ code: string; state: string }>;
  if (isElectron()) {
    resultPromise = new Promise<{ code: string; state: string }>((resolve, reject) => {
      window.electronAPI.receive('parsec-sso-complete', async (code?: string, state?: string) => {
        if (code && state) {
          resolve({ code, state });
        } else {
          reject();
        }
      });
    });
  } else {
    const bc = new BroadcastChannel('openbao-oidc');
    resultPromise = new Promise<{ code: string; state: string }>((resolve, reject) => {
      bc.onmessage = (event): void => {
        // Set a 5min timeout
        const t = setTimeout(
          () => {
            bc.close();
            reject(new Error('timeout'));
          },
          5 * 60 * 1000,
        );

        if (!event.data.code || !event.data.state) {
          reject();
        } else {
          resolve({ code: event.data.code, state: event.data.state });
        }
        clearTimeout(t);
        bc.close();
      };
    });
  }

  if (!window.electronAPI.openPopup(connResult.value)) {
    window.electronAPI.log('error', 'Failed to open popup');
    return { ok: false, error: { type: OpenBaoErrorType.PopupFailed } };
  }

  try {
    const { code, state } = await resultPromise;
    const result = await getToken(code, state, openBaoServer, mountpoint);
    if (!result.ok) {
      return result;
    }
    const client = new OpenBaoClient(result.value.token, result.value.id, openBaoServer, provider, mountpoint, secretMountpoint);
    return { ok: true, value: client };
  } catch (err: any) {
    return { ok: false, error: { type: OpenBaoErrorType.InitError, detail: err.toString() } };
  }
}

const HandledSSOProviders = [OpenBaoAuthConfigTag.OIDCProConnect];

function isSSOProviderHandled(provider: OpenBaoAuthConfigTag): boolean {
  return HandledSSOProviders.includes(provider);
}

export { isOpenBaoAvailable, isSSOProviderHandled, openBaoConnect, OpenBaoError, OpenBaoErrorType, OpenBaoResult };

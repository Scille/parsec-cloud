// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isElectron } from '@/parsec';
import { getRouter } from '@/router';
import { Env } from '@/services/environment';
import axios, { AxiosInstance } from 'axios';

enum OpenBaoErrorType {
  HTTPError = 'http-error',
  NetworkError = 'network-error',
  InitError = 'init-error',
  NotAvailable = 'not-available',
  PopupFailed = 'popup-failed',
}

interface OpenBaoError {
  httpStatus?: number;
  type: OpenBaoErrorType;
  detail?: string;
  errors?: Array<string>;
}

type OpenBaoResult<T> = { ok: true; value: T } | { ok: false; error: OpenBaoError };

class OpenBaoClient {
  _client: AxiosInstance;
  _userId: string;

  constructor(token: string, id: string) {
    this._client = axios.create({
      baseURL: Env.getOpenBaoServer(),
      timeout: 5000,
    });
    this._client.defaults.headers.common['X-Vault-Token'] = token;
    this._userId = id;
  }

  async store(key: string, data: object): Promise<OpenBaoResult<undefined>> {
    try {
      const path = `/v1/parsec-keys/data/${this._userId}/${key}`;
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

  async retrieve(key: string): Promise<OpenBaoResult<any>> {
    try {
      const path = `/v1/parsec-keys/data/${this._userId}/${key}`;
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

async function getConnectionUrl(): Promise<OpenBaoResult<string>> {
  try {
    const callbackPath = getRouter().resolve('/oidc/callback').path;
    let host!: string;
    if (isElectron()) {
      // Fixed host, this is filtered by the SSO.
      host = 'http://localhost:4200';
    } else {
      host = window.location.origin;
    }

    const redirect = new URL(callbackPath, host);
    const client = axios.create({ baseURL: Env.getOpenBaoServer(), timeout: 5000 });
    // eslint-disable-next-line camelcase
    const resp = await client.post('/v1/auth/oidc/oidc/auth_url', { role: 'default', redirect_uri: redirect.toString() });
    return { ok: true, value: resp.data.data.auth_url };
  } catch (err: any) {
    if (err.response) {
      return { ok: false, error: { type: OpenBaoErrorType.HTTPError, httpStatus: err.response.status, errors: err.response.data.errors } };
    } else {
      return { ok: false, error: { type: OpenBaoErrorType.NetworkError, detail: err.toString() } };
    }
  }
}

async function getToken(code: string, state: string): Promise<OpenBaoResult<{ token: string; id: string }>> {
  try {
    const client = axios.create({ baseURL: Env.getOpenBaoServer(), timeout: 5000 });
    const resp = await client.get('/v1/auth/oidc/oidc/callback', { params: { code: code, state: state } });
    return { ok: true, value: { token: resp.data.auth.client_token, id: resp.data.auth.entity_id } };
  } catch (err: any) {
    if (err.response) {
      return { ok: false, error: { type: OpenBaoErrorType.HTTPError, httpStatus: err.response.status, errors: err.response.data.errors } };
    } else {
      return { ok: false, error: { type: OpenBaoErrorType.NetworkError, detail: err.toString() } };
    }
  }
}

let client: OpenBaoClient | undefined = undefined;

interface _OpenBaoAPI {
  openBaoConnect: () => Promise<OpenBaoResult<OpenBaoClient>>;
  getOpenBaoClient: () => OpenBaoClient | undefined;
  isOpenBaoAvailable(): boolean;
}

function useOpenBao(): _OpenBaoAPI {
  function isAvailable(): boolean {
    return Boolean(Env.getOpenBaoServer());
  }

  async function startProcess(): Promise<OpenBaoResult<OpenBaoClient>> {
    if (!isAvailable()) {
      return { ok: false, error: { type: OpenBaoErrorType.NotAvailable, detail: 'OpenBao server not set' } };
    }
    if (client) {
      return { ok: true, value: client };
    }

    const connResult = await getConnectionUrl();
    if (!connResult.ok) {
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
      const result = await getToken(code, state);
      if (!result.ok) {
        return result;
      }
      client = new OpenBaoClient(result.value.token, result.value.id);
      return { ok: true, value: client };
    } catch (err: any) {
      return { ok: false, error: { type: OpenBaoErrorType.InitError, detail: err.toString() } };
    }
  }

  function getClient(): OpenBaoClient | undefined {
    return client;
  }

  return { getOpenBaoClient: getClient, openBaoConnect: startProcess, isOpenBaoAvailable: isAvailable };
}

export { OpenBaoError, OpenBaoResult, useOpenBao };

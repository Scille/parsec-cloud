// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isWeb } from '@/parsec';
import { Env } from '@/services/environment';
import axios, { AxiosInstance } from 'axios';

const Resources = {
  LogoFull: 'logo-full',
  LogoIcon: 'logo-icon',
  HomeSidebar: 'home-sidebar',
  TranslationEnUs: 'translation-en-us',
  TranslationFrFr: 'translation-fr-fr',
} as const;
type Resources = (typeof Resources)[keyof typeof Resources];

function resolveResource(res: Resources, cacheBustingSuffix: string): string {
  let template: string;
  switch (res) {
    case Resources.LogoFull:
      template = 'logo{cacheBustingSuffix}.svg';
      break;
    case Resources.LogoIcon:
      template = 'logo_icon{cacheBustingSuffix}.svg';
      break;
    case Resources.HomeSidebar:
      template = 'home_sidebar{cacheBustingSuffix}.png';
      break;
    case Resources.TranslationEnUs:
      template = 'custom_en-US{cacheBustingSuffix}.json';
      break;
    case Resources.TranslationFrFr:
      template = 'custom_fr-FR{cacheBustingSuffix}.json';
      break;
    default:
      // Guard to have the compilation fails whenever a new kind of Resource is added
      res satisfies never;
      return '';
  }
  return template.replace('{cacheBustingSuffix}', cacheBustingSuffix);
}

async function convertSVG(buffer: ArrayBuffer): Promise<string> {
  return new TextDecoder('utf-8').decode(buffer);
}

async function convertJSON(buffer: ArrayBuffer): Promise<object> {
  return JSON.parse(new TextDecoder('utf-8').decode(buffer));
}

async function convertBinary(buffer: ArrayBuffer): Promise<Uint8Array> {
  return new Uint8Array(buffer);
}

type convertFunction = (buffer: ArrayBuffer) => Promise<NonNullable<unknown>>;

const ResourceConverters = new Map<Resources, convertFunction>([
  [Resources.LogoIcon, convertSVG],
  [Resources.LogoFull, convertSVG],
  [Resources.HomeSidebar, convertBinary],
  [Resources.TranslationEnUs, convertJSON],
  [Resources.TranslationFrFr, convertJSON],
]);

class ResourcesMap {
  map: Map<Resources, NonNullable<unknown>>;

  constructor() {
    this.map = new Map<Resources, NonNullable<unknown>>();
  }

  get(res: Resources): NonNullable<unknown> | undefined {
    return this.map.get(res);
  }

  set(res: Resources, content: NonNullable<unknown>): void {
    this.map.set(res, content);
  }

  has(res: Resources): boolean {
    return this.map.has(res);
  }

  clear(): void {
    this.map.clear();
  }
}

interface IResourcesProvider {
  load(res: Resources): Promise<NonNullable<unknown>>;
}

class WebResourcesProvider implements IResourcesProvider {
  axiosInstance: AxiosInstance;
  cacheBustingSuffix: string;

  constructor(cacheBustingSuffix: string, path: string = 'custom') {
    let baseURL: string;
    if (import.meta.env.BASE_URL.endsWith('/')) {
      baseURL = `${import.meta.env.BASE_URL ?? ''}${path}`;
    } else {
      baseURL = `${import.meta.env.BASE_URL ?? ''}/${path}`;
    }

    this.axiosInstance = axios.create({
      baseURL,
      timeout: 1000,
    });
    this.cacheBustingSuffix = cacheBustingSuffix;
  }

  async load(res: Resources): Promise<NonNullable<unknown>> {
    const resName = resolveResource(res, this.cacheBustingSuffix);
    const resp = await this.axiosInstance.get(resName, { responseType: 'arraybuffer' });
    if (resp.status !== 200) {
      throw new Error(`HTTP status ${resp.status} ${resp.statusText}`);
    }
    // Returns index.html if resource does not exist
    // Note content-type header may contain charset (e.g. `text/html; charset=utf-8`)
    if (resp.headers['content-type'] && resp.headers['content-type'].startsWith('text/html')) {
      throw new Error('Resource not found');
    }
    const convertF = ResourceConverters.get(res);
    if (!convertF) {
      throw new Error("Don't know how to convert");
    }
    return await convertF(resp.data);
  }
}

class FileSystemResourcesProvider implements IResourcesProvider {
  cacheBustingSuffix: string;

  constructor(cacheBustingSuffix: string) {
    this.cacheBustingSuffix = cacheBustingSuffix;
  }

  async load(res: Resources): Promise<NonNullable<unknown>> {
    const resName = resolveResource(res, this.cacheBustingSuffix);
    const data = await window.electronAPI.readCustomFile(resName);

    if (!data) {
      throw new Error('File not found');
    }
    const convertF = ResourceConverters.get(res);
    if (!convertF) {
      throw new Error("Don't know how to convert");
    }
    return await convertF(data);
  }
}

class _ResourcesManager {
  resources: ResourcesMap;
  provider: WebResourcesProvider | FileSystemResourcesProvider | null;

  constructor() {
    this.resources = new ResourcesMap();
    const [customBrandingEnabled, cacheBustingSuffix] = Env.isCustomBrandingEnabled();
    if (customBrandingEnabled) {
      this.provider = isWeb() ? new WebResourcesProvider(cacheBustingSuffix) : new FileSystemResourcesProvider(cacheBustingSuffix);
    } else {
      this.provider = null;
    }
  }

  async loadAll(): Promise<void> {
    if (this.provider === null) {
      window.electronAPI.log('info', 'Custom branding is not enabled, not loading resources');
      return;
    } else {
      window.electronAPI.log('info', 'Custom branding is enabled, loading resources...');
    }
    const promises: Array<Promise<NonNullable<unknown>>> = [];
    for (const res of Object.values(Resources)) {
      const promise = this.provider.load(res);

      // then/catch to process all queries simultaneously
      promise
        .then((resource) => {
          if (resource !== undefined) {
            this.resources.set(res, resource);
          }
        })
        .catch((reason) => {
          window.electronAPI.log('debug', `Failed to retrieve custom resource ${res}: ${reason.toString()}`);
        });
      promises.push(promise);
    }
    // Waiting for everything to be loaded
    await Promise.allSettled(promises);
  }

  get(res: Resources, defaultValue: any = undefined): NonNullable<unknown> | undefined {
    return this.resources.get(res) ?? defaultValue;
  }
}

class ResourcesManager {
  static manager: _ResourcesManager | undefined;

  static instance(): _ResourcesManager {
    if (!this.manager) {
      this.manager = new _ResourcesManager();
    }
    return this.manager;
  }
}

export { Resources, ResourcesManager };

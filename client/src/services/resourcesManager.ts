// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isWeb } from '@/parsec';
import { Env } from '@/services/environment';
import axios from 'axios';

enum Resources {
  LogoFull = 'logo.svg',
  LogoIcon = 'logo_icon.svg',
  HomeSidebar = 'home_sidebar.png',
  TranslationEnUs = 'custom_en-US.json',
  TranslationFrFr = 'custom_fr-FR.json',
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
  path: string;

  constructor(path: string = '/custom') {
    this.path = path;
  }

  async load(res: Resources): Promise<NonNullable<unknown>> {
    const axiosInstance = axios.create({
      baseURL: this.path,
      timeout: 1000,
    });

    const resp = await axiosInstance.get(res, { responseType: 'arraybuffer' });
    if (resp.status !== 200) {
      throw new Error(`HTTP status ${resp.status} ${resp.statusText}`);
    }
    // Returns index.html if resource does not exist
    if (resp.headers['content-type'] === 'text/html') {
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
  async load(res: Resources): Promise<NonNullable<unknown>> {
    const data = await window.electronAPI.readCustomFile(res);

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
  provider: WebResourcesProvider | FileSystemResourcesProvider;

  constructor() {
    this.resources = new ResourcesMap();
    this.provider = isWeb() ? new WebResourcesProvider() : new FileSystemResourcesProvider();
  }

  async loadAll(): Promise<void> {
    if (!Env.isCustomBrandingEnabled()) {
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

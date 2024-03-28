// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ConnectionHandle, needsMocks } from '@/parsec';
import { EventDistributor } from '@/services/eventDistributor';
import { ImportManager } from '@/services/importManager';
import { InformationManager } from '@/services/informationManager';

interface Injections {
  importManager: ImportManager;
  eventDistributor: EventDistributor;
  informationManager: InformationManager;
}

export const InjectionProviderKey = 'InjectionProvider';

export class InjectionProvider {
  injections: Map<ConnectionHandle, Injections>;
  default: Injections;

  constructor() {
    this.injections = new Map();
    this.default = {
      importManager: new ImportManager(),
      eventDistributor: new EventDistributor(),
      informationManager: new InformationManager(),
    };
  }

  createNewInjections(handle: ConnectionHandle, eventDistributor: EventDistributor): void {
    const inj = this.injections.get(handle);

    if (inj) {
      console.error('Already has injections for this handle', handle);
      return;
    }
    this.injections.set(handle, {
      importManager: new ImportManager(),
      eventDistributor: eventDistributor,
      informationManager: new InformationManager(),
    });
  }

  getInjections(handle: ConnectionHandle): Injections {
    const inj = this.injections.get(handle);

    if (!inj) {
      if (needsMocks()) {
        return this.getDefault();
      }
      console.warn('Could not get injections for handle', handle);
      return this.default;
    }
    return inj;
  }

  getDefault(): Injections {
    return this.default;
  }

  async clean(handle: ConnectionHandle): Promise<void> {
    const inj = this.injections.get(handle);
    if (!inj) {
      return;
    }
    await inj.importManager.stop();
    this.injections.delete(handle);
  }

  async cleanAll(): Promise<void> {
    for (const key of this.injections.keys()) {
      await this.clean(key);
    }
  }
}

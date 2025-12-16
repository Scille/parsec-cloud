// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ConnectionHandle, isWeb } from '@/parsec';
import { EventData, EventDistributor, Events } from '@/services/eventDistributor';
import { FileOperationManager } from '@/services/fileOperation/manager';
import { Information, InformationManager } from '@/services/informationManager';

export interface Injections {
  fileOperationManager: FileOperationManager;
  eventDistributor: EventDistributor;
  informationManager: InformationManager;
  isDefault: boolean;
}

export const InjectionProviderKey = 'InjectionProvider';

export class InjectionProvider {
  injections: Map<ConnectionHandle, Injections>;
  default: Injections;

  constructor() {
    this.injections = new Map();
    this.default = {
      fileOperationManager: new FileOperationManager(),
      eventDistributor: new EventDistributor(),
      informationManager: new InformationManager(),
      isDefault: true,
    };
  }

  createNewInjections(handle: ConnectionHandle, eventDistributor: EventDistributor): void {
    const inj = this.injections.get(handle);

    if (inj) {
      console.error('Already has injections for this handle', handle);
      return;
    }
    this.injections.set(handle, {
      fileOperationManager: new FileOperationManager(),
      eventDistributor: eventDistributor,
      informationManager: new InformationManager(handle),
      isDefault: false,
    });
  }

  hasInjections(handle: ConnectionHandle): boolean {
    return this.injections.get(handle) !== undefined;
  }

  getInjections(handle: ConnectionHandle): Injections {
    const inj = this.injections.get(handle);

    if (!inj) {
      if (window.isDev() && isWeb()) {
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

  async distributeEventToAll(event: Events, data?: EventData): Promise<void> {
    for (const injections of this.injections.values()) {
      await injections.eventDistributor.dispatchEvent(event, data);
    }
    this.default.eventDistributor.dispatchEvent(event, data);
  }

  async notifyAll(info: Information, mode: number): Promise<void> {
    for (const injections of this.injections.values()) {
      await injections.informationManager.present(info, mode);
    }
    this.default.informationManager.present(info, mode);
  }

  async clean(handle: ConnectionHandle): Promise<void> {
    const inj = this.injections.get(handle);
    if (!inj) {
      return;
    }
    await inj.fileOperationManager.stop();
    this.injections.delete(handle);
  }

  async cleanAll(): Promise<void> {
    for (const key of this.injections.keys()) {
      await this.clean(key);
    }
  }
}

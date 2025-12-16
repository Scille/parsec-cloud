// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryName } from '@/parsec';
import { FileOperationData } from '@/services/fileOperation/operationData';
import { OperationFailedErrors } from '@/services/fileOperation/types';
import { v4 as uuid4 } from 'uuid';

export enum FileOperationEvents {
  AllFinished = 'all-finished',
  Progress = 'progress',
  Finished = 'finished',
  Added = 'added',
  Started = 'started',
  Failed = 'failed',
  Finalizing = 'finalizing',
  Cancelled = 'cancelled',
}

export interface OperationProgressEventData {
  global: {
    progress: number;
    totalSize: number;
    currentSize: number;
    fileCount: number;
    fileIndex: number;
  };
  currentFile: {
    name: EntryName;
    progress: number;
    currentSize: number;
    totalSize: number;
  };
}

export interface OperationFailedEventData {
  error: OperationFailedErrors;
  details?: string;
}

export type FileOperationEventData = OperationProgressEventData | OperationFailedEventData;

export type FileOperationCallback = (
  event: FileOperationEvents,
  operationData?: FileOperationData,
  eventData?: FileOperationEventData,
) => Promise<void>;

export class FileEventRegistrationCanceller {
  distributor: FileOperationEventDistributor;
  id: string;

  constructor(id: string, distributor: FileOperationEventDistributor) {
    this.id = id;
    this.distributor = distributor;
  }

  cancel() {
    this.distributor._removeCallback(this.id);
  }
}

export class FileOperationEventDistributor {
  callbacks: Map<string, FileOperationCallback>;

  constructor() {
    this.callbacks = new Map<string, FileOperationCallback>();
  }

  registerCallback(cb: FileOperationCallback): FileEventRegistrationCanceller {
    const id = uuid4();
    this.callbacks.set(id, cb);
    return new FileEventRegistrationCanceller(id, this);
  }

  _removeCallback(id: string): void {
    this.callbacks.delete(id);
  }

  async distribute(event: FileOperationEvents, operationData?: FileOperationData, eventData?: FileOperationEventData): Promise<void> {
    for (const [_id, callback] of this.callbacks.entries()) {
      await callback(event, operationData, eventData);
    }
  }
}

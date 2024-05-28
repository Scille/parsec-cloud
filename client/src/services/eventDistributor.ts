// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { InvitationStatus, InvitationToken, WorkspaceID } from '@/parsec';
import { v4 as uuid4 } from 'uuid';

export const EventDistributorKey = 'eventDistributor';

enum Events {
  WorkspaceCreated = 1 << 0,
  WorkspaceFavorite = 1 << 1,
  Online = 1 << 2,
  Offline = 1 << 3,
  InvitationUpdated = 1 << 4,
  IncompatibleServer = 1 << 5,
  ClientRevoked = 1 << 6,
  ExpiredOrganization = 1 << 7,
  UpdateAvailability = 1 << 8,
  WorkspaceUpdated = 1 << 9,
  EntryUpdated = 1 << 10,
}

interface WorkspaceCreatedData {
  workspaceId: WorkspaceID;
}

interface InvitationUpdatedData {
  token: InvitationToken;
  status: InvitationStatus;
}

interface UpdateAvailabilityData {
  updateAvailable: boolean;
  version?: string;
}

type EventData = WorkspaceCreatedData | InvitationUpdatedData | UpdateAvailabilityData;

interface Callback {
  id: string;
  events: number;
  funct: (event: Events, data?: EventData) => Promise<void>;
}

class EventDistributor {
  private callbacks: Array<Callback>;

  constructor() {
    this.callbacks = [];
  }

  async dispatchEvent(event: Events, data?: EventData): Promise<void> {
    for (const cb of this.callbacks) {
      if (event & cb.events) {
        await cb.funct(event, data);
      }
    }
  }

  async registerCallback(events: number, funct: (event: Events, data?: EventData) => Promise<void>): Promise<string> {
    const id = uuid4();
    this.callbacks.push({ id: id, events: events, funct: funct });
    return id;
  }

  async removeCallback(id: string): Promise<void> {
    this.callbacks = this.callbacks.filter((cb) => cb.id !== id);
  }
}

export { EventData, EventDistributor, Events, InvitationUpdatedData, UpdateAvailabilityData, WorkspaceCreatedData };

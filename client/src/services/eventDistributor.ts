// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  AccessToken,
  ApiVersion,
  ConnectionHandle,
  DeviceInfo,
  EntryID,
  EntryName,
  FsPath,
  GreetingAttemptID,
  InvitationStatus,
  UserID,
  WorkspaceHandle,
  WorkspaceID,
  WorkspaceRole,
} from '@/parsec';
import { MenuAction } from '@/views/menu';
import { v4 as uuid4 } from 'uuid';

export const EventDistributorKey = 'eventDistributor';

enum Events {
  WorkspaceCreated = 1 << 0,
  Online = 1 << 2,
  Offline = 1 << 3,
  InvitationUpdated = 1 << 4,
  IncompatibleServer = 1 << 5,
  ClientRevoked = 1 << 6,
  ExpiredOrganization = 1 << 7,
  UpdateAvailability = 1 << 8,
  WorkspaceUpdated = 1 << 9,
  EntryUpdated = 1 << 10,
  EntrySynced = 1 << 11,
  TOSAcceptRequired = 1 << 12,
  LogoutRequested = 1 << 13,
  EntrySyncStarted = 1 << 14,
  GreetingAttemptReady = 1 << 15,
  GreetingAttemptCancelled = 1 << 16,
  GreetingAttemptJoined = 1 << 17,
  ClientStarted = 1 << 18,
  ClientStopped = 1 << 19,
  MenuAction = 1 << 20,
  ClientFrozen = 1 << 21,
  OrganizationNotFound = 1 << 22,
  DeviceCreated = 1 << 23,
  WorkspaceRoleUpdate = 1 << 24,
  EntryRenamed = 1 << 25,
  EntryDeleted = 1 << 26,
  EntrySyncProgress = 1 << 27,
  WorkspaceMountpointsSync = 1 << 28,
  OpenContextMenu = 1 << 28,
  AsyncEnrollmentUpdated = 1 << 29,
}

interface WorkspaceCreatedData {
  workspaceId: WorkspaceID;
}

interface InvitationUpdatedData {
  token: AccessToken;
  status: InvitationStatus;
}

interface GreetingAttemptReadyData {
  token: AccessToken;
  greetingAttempt: GreetingAttemptID;
}

interface GreetingAttemptCancelledData {
  token: AccessToken;
  greetingAttempt: GreetingAttemptID;
}

interface GreetingAttemptJoinedData {
  token: AccessToken;
  greetingAttempt: GreetingAttemptID;
}

interface UpdateAvailabilityData {
  updateAvailable: boolean;
  version?: string;
}

interface EntrySyncData {
  workspaceId: WorkspaceID;
  entryId: EntryID;
  way: 'inbound' | 'outbound';
}

interface IncompatibleServerData {
  version: ApiVersion;
  supportedVersions: Array<ApiVersion>;
}

interface ClientStatusUpdateData {
  handle: ConnectionHandle;
}

interface MenuActionData {
  action: MenuAction;
}

interface DeviceCreatedData {
  info: DeviceInfo;
}

interface WorkspaceRoleUpdateData {
  workspaceId: WorkspaceID;
  userId: UserID;
  oldRole: WorkspaceRole | null;
  newRole: WorkspaceRole | null;
}

interface EntryRenamedData {
  workspaceHandle: WorkspaceHandle;
  entryId: EntryID;
  oldPath: FsPath;
  newPath: FsPath;
  oldName: EntryName;
  newName: EntryName;
}

interface EntryDeletedData {
  workspaceHandle: WorkspaceHandle;
  entryId: EntryID;
  path: FsPath;
}

interface WorkspaceMountpointInfo {
  workspaceId: WorkspaceID;
  isMounted: boolean;
}

interface OpenContextualMenuData {
  event: Event;
}

type EventData =
  | WorkspaceCreatedData
  | InvitationUpdatedData
  | UpdateAvailabilityData
  | EntrySyncData
  | IncompatibleServerData
  | GreetingAttemptReadyData
  | GreetingAttemptCancelledData
  | GreetingAttemptJoinedData
  | ClientStatusUpdateData
  | MenuActionData
  | DeviceCreatedData
  | WorkspaceRoleUpdateData
  | EntryRenamedData
  | EntryDeletedData
  | WorkspaceMountpointInfo
  | OpenContextualMenuData;

interface Callback {
  id: string;
  events: number;
  funct: (event: Events, data?: EventData) => Promise<void>;
}

class EventDistributor {
  private callbacks: Array<Callback>;
  private timeouts: Map<Events, number>;

  constructor() {
    this.callbacks = [];
    this.timeouts = new Map<number, Events>();
  }

  async dispatchEvent(
    event: Events,
    data?: EventData,
    options: { aggregateTime?: number; delay?: number } = { aggregateTime: undefined, delay: undefined },
  ): Promise<void> {
    async function sendToAll(callbacks: Array<Callback>, event: Events, data?: EventData): Promise<void> {
      for (const cb of callbacks) {
        if (event & cb.events) {
          cb.funct(event, data);
        }
      }
    }

    if (options.aggregateTime !== undefined && options.delay !== undefined) {
      console.warn('Cannot have both aggregateTime and delay set, ignoring this event.');
      return;
    }
    // In some cases, events can occur very close to each other, leading to some heavy operations.
    // We can aggregate those cases in order to distribute only one event if multiple occur in a short
    // time lapse.
    if (options.aggregateTime !== undefined) {
      if (data) {
        // Can't have data with an aggregateTime, we wouldn't know what data to use
        console.warn('Cannot have an aggregate time with data, ignoring this event.');
        return;
      }
      // Clear previous interval if any
      if (this.timeouts.has(event)) {
        const interval = this.timeouts.get(event);
        this.timeouts.delete(event);
        window.clearInterval(interval);
      }
      // Create a new timeout
      const interval = window.setTimeout(async () => {
        await sendToAll(this.callbacks, event, undefined);
      }, options.aggregateTime);
      // Add it to the list
      this.timeouts.set(event, interval);
    } else if (options.delay !== undefined) {
      window.setTimeout(
        async (d?: EventData) => {
          await sendToAll(this.callbacks, event, d);
        },
        options.delay,
        data,
      );
    } else {
      await sendToAll(this.callbacks, event, data);
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

export {
  DeviceCreatedData,
  EntryDeletedData,
  EntryRenamedData,
  EntrySyncData,
  EventData,
  EventDistributor,
  Events,
  IncompatibleServerData,
  InvitationUpdatedData,
  MenuActionData,
  OpenContextualMenuData,
  UpdateAvailabilityData,
  WorkspaceCreatedData,
  WorkspaceMountpointInfo,
  WorkspaceRoleUpdateData,
};

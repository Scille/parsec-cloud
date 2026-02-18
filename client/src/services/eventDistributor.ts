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
  WorkspaceCreated = 'workspace-created',
  Online = 'online',
  Offline = 'offline',
  InvitationUpdated = 'invitation-updated',
  IncompatibleServer = 'incompatible-server',
  ClientRevoked = 'client-revoked',
  ExpiredOrganization = 'expired-organization',
  UpdateAvailability = 'update-availability',
  WorkspaceUpdated = 'workspace-updated',
  EntryUpdated = 'entry-updated',
  EntrySynced = 'entry-synced',
  TOSAcceptRequired = 'tos-accept-required',
  LogoutRequested = 'logout-requested',
  EntrySyncStarted = 'entry-sync-started',
  GreetingAttemptReady = 'greeting-attempt-ready',
  GreetingAttemptCancelled = 'greeting-attempt-cancelled',
  GreetingAttemptJoined = 'greeting-attempt-joined',
  ClientStarted = 'client-started',
  ClientStopped = 'client-stopped',
  MenuAction = 'menu-action',
  ClientFrozen = 'client-frozen',
  OrganizationNotFound = 'organization-not-found',
  DeviceCreated = 'device-created',
  WorkspaceRoleUpdate = 'workspace-role-update',
  EntryRenamed = 'entry-renamed',
  EntryDeleted = 'entry-deleted',
  EntrySyncProgress = 'entry-sync-progress',
  WorkspaceMountpointsSync = 'workspace-mountpoints-sync',
  OpenContextMenu = 'open-context-menu',
  AsyncEnrollmentUpdated = 'async-enrollment-updated',
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
  events: Array<Events>;
  funct: (event: Events, data?: EventData) => Promise<void>;
}

class EventDistributor {
  private callbacks: Array<Callback>;
  private timeouts: Map<Events, number>;

  constructor() {
    this.callbacks = [];
    this.timeouts = new Map<Events, number>();
  }

  async dispatchEvent(
    event: Events,
    data?: EventData,
    options: { aggregateTime?: number; delay?: number } = { aggregateTime: undefined, delay: undefined },
  ): Promise<void> {
    async function sendToAll(callbacks: Array<Callback>, event: Events, data?: EventData): Promise<void> {
      for (const cb of callbacks) {
        if (cb.events.includes(event)) {
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

  async registerCallback(events: Array<Events>, funct: (event: Events, data?: EventData) => Promise<void>): Promise<string> {
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

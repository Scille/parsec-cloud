// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { WorkspaceRole } from '@/parsec';
import { InformationDataType, InformationLevel } from '@/services/informationManager';
import { Notification as MsNotification, NotificationManager } from '@/services/notificationManager';
import { MsReportTheme } from 'megashark-lib';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

describe.skip('Notification Manager', () => {
  let NOTIFS: MsNotification[];
  let notificationManager: NotificationManager;
  const FIRST_NOTIF_ID = '123';
  const SECOND_NOTIF_ID = '456';
  const THIRD_NOTIF_ID = '789';

  beforeEach(() => {
    notificationManager = new NotificationManager();

    vi.useFakeTimers();
    vi.setSystemTime(new Date(2000, 1, 1));

    NOTIFS = [
      new MsNotification({
        id: FIRST_NOTIF_ID,
        message: 'A',
        level: InformationLevel.Warning,
        theme: MsReportTheme.Warning,
      }),
      new MsNotification({
        id: SECOND_NOTIF_ID,
        message: 'B',
        level: InformationLevel.Info,
        theme: MsReportTheme.Info,
        data: {
          type: InformationDataType.WorkspaceRoleChanged,
          workspaceHandle: 42,
          oldRole: WorkspaceRole.Reader,
          newRole: WorkspaceRole.Contributor,
        },
      }),
      new MsNotification({
        id: THIRD_NOTIF_ID,
        message: 'C',
        level: InformationLevel.Error,
        theme: MsReportTheme.Error,
      }),
    ];
  });

  afterEach(() => {
    notificationManager.clear();
    vi.useRealTimers();
  });

  it('should start with no notifications', async () => {
    const notificationManager = new NotificationManager();

    expect(notificationManager).to.exist;
    expect(notificationManager.notifications.value).to.deep.equal([]);
    expect(notificationManager.getNotifications()).to.deep.equal([]);
    expect(notificationManager.hasUnreadNotifications()).to.be.false;
  });

  it('should return an empty list if there are no notifications', () => {
    const notificationManager = new NotificationManager();
    const notifications = notificationManager.getNotifications();
    expect(notifications.length).to.deep.equal(0);
  });

  it('should instantiate Notification with Information object without errors', () => {
    const notification = new MsNotification(NOTIFS[0].information);
    expect(notification).to.exist;
  });

  it('should add notifications to the list', async () => {
    expect(notificationManager.notifications.value).to.deep.equal([]);
    notificationManager.add(NOTIFS[2].information);
    // we expect NOTIFS[2] to be in notifications
    expect(notificationManager.getNotifications()).to.deep.equal(NOTIFS.slice(2));
    expect(notificationManager.hasUnreadNotifications()).to.be.true;

    notificationManager.add(NOTIFS[0].information);
    notificationManager.add(NOTIFS[1].information);
    expect(notificationManager.getNotifications().length).to.equal(3);

    notificationManager.clear();
    expect(notificationManager.getNotifications()).to.deep.equal([]);
    expect(notificationManager.hasUnreadNotifications()).to.be.false;
  });

  it('should mark notifications as read', async () => {
    expect(notificationManager.hasUnreadNotifications()).to.be.false;
    notificationManager.add(NOTIFS[0].information);
    expect(notificationManager.hasUnreadNotifications()).to.be.true;
    expect(notificationManager.getNotifications()).to.deep.equal([NOTIFS[0]]);
    notificationManager.markAsRead(FIRST_NOTIF_ID, true);
    expect(notificationManager.hasUnreadNotifications()).to.be.false;
    notificationManager.markAsRead(FIRST_NOTIF_ID, false);
    expect(notificationManager.hasUnreadNotifications()).to.be.true;
  });

  it('should not modify the list if the specified id to mark as read is not found', () => {
    notificationManager.add(NOTIFS[0].information);
    expect(notificationManager.hasUnreadNotifications()).to.be.true;
    expect(notificationManager.getNotifications()).to.deep.equal([NOTIFS[0]]);
    notificationManager.markAsRead('nonexistent-id');
    expect(notificationManager.hasUnreadNotifications()).to.be.true;
    expect(notificationManager.getNotifications()).to.deep.equal([NOTIFS[0]]);
  });

  it('should clear all notifications', () => {
    notificationManager.add(NOTIFS[0].information);
    notificationManager.clear();
    expect(notificationManager.getNotifications()).to.have.length(0);
    expect(notificationManager.hasUnreadNotifications()).to.be.false;
  });
});

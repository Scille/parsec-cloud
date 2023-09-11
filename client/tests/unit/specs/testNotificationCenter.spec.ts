// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  Notification as MsNotification,
  NotificationLevel,
  NotificationCenter,
} from '@/services/notificationCenter';
import { vi } from 'vitest';
import { DateTime } from 'luxon';

describe('Notification Center', () => {
  let NOTIFS: MsNotification[];

  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date(2000, 1, 1));
    vi.mock('uuid', () => {
      const v4 = vi.fn();
      v4.mockReturnValue('1234');
      return { v4 };
    });

    NOTIFS = [
      {
        id: '1234',
        title: 'A',
        message: 'B',
        level: NotificationLevel.Warning,
        read: false,
        time: DateTime.now(),
        data: {},
      },
      {
        id: '1234',
        message: 'D',
        level: NotificationLevel.Info,
        read: false,
        time: DateTime.now(),
        data: {
          'key': 'value',
        },
      },
      {
        id: '1234',
        title: 'E',
        message: 'F',
        level: NotificationLevel.Error,
        read: false,
        time: DateTime.now(),
        data: {},
      },
    ];
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('Check initial state', async () => {
    const center = new NotificationCenter();

    expect(center.notifications).to.deep.equal([]);
    expect(center.getNotifications()).to.deep.equal([]);
    expect(center.hasUnreadNotifications()).to.be.false;
  });

  it('Adds notification to list', async () => {
    const center = new NotificationCenter();

    expect(center.notifications).to.deep.equal([]);
    center.showModal('A', 'B', NotificationLevel.Warning, true, false);
    expect(center.getNotifications()).to.deep.equal(NOTIFS.slice(0, 1));
    expect(center.hasUnreadNotifications()).to.be.true;

    center.showSnackbar('D', NotificationLevel.Info, true, false, {'key': 'value'});
    expect(center.getNotifications()).to.deep.equal(NOTIFS.slice(0, 2));

    center.addToList('E', 'F', NotificationLevel.Error, false);
    expect(center.getNotifications()).to.deep.equal(NOTIFS);

    center.clear();
    expect(center.getNotifications()).to.deep.equal([]);
    expect(center.hasUnreadNotifications()).to.be.false;
  });

  it('Do not add notification to list', async () => {
    const center = new NotificationCenter();

    expect(center.notifications).to.deep.equal([]);
    center.showModal('A', 'B', NotificationLevel.Warning, false, false);
    expect(center.getNotifications()).to.deep.equal([]);
    expect(center.hasUnreadNotifications()).to.be.false;

    center.showSnackbar('D', NotificationLevel.Info, false, false, {'key': 'value'});
    expect(center.getNotifications()).to.deep.equal([]);

    center.addToList('E', 'F', NotificationLevel.Error, false);
    expect(center.getNotifications()).to.deep.equal(NOTIFS.slice(2, 3));
    expect(center.hasUnreadNotifications()).to.be.true;
  });

  it('Mark as read', async () => {
    const center = new NotificationCenter();

    expect(center.hasUnreadNotifications()).to.be.false;
    center.showModal('A', 'B', NotificationLevel.Warning, true, false);
    expect(center.hasUnreadNotifications()).to.be.true;
    center.markAsRead('1234', true);
    expect(center.hasUnreadNotifications()).to.be.false;
    center.markAsRead('1234', false);
    expect(center.hasUnreadNotifications()).to.be.true;
  });
});

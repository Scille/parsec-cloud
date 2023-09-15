// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  Notification as MsNotification,
  NotificationLevel,
  NotificationCenter,
} from '@/services/notificationCenter';
import { vi } from 'vitest';
import { DateTime } from 'luxon';
import { modalController, toastController } from '@ionic/vue';

describe('Notification Center', () => {
  // mock ComposerTranslation
  function t(key: string): string {
    const map = new Map<string, string>([
      ['Notification.nextButton', 'nextButton'],
    ]);
    return map.get(key) as string;
  }
  let NOTIFS: MsNotification[];
  let center: NotificationCenter;

  beforeEach(() => {
    center = new NotificationCenter(t);

    vi.useFakeTimers();
    vi.setSystemTime(new Date(2000, 1, 1));
    vi.mock('uuid', () => {
      const v4 = vi.fn();
      v4.mockReturnValue('1234');
      return { v4 };
    });
    center._createAndPresentModal = vi.fn();
    const mockModalControllerDismiss = vi.spyOn(modalController, 'dismiss');
    mockModalControllerDismiss.mockReturnValue(new Promise(vi.fn()));
    center._createAndPresentToast = vi.fn();
    const mockToastControllerDismiss = vi.spyOn(toastController, 'dismiss');
    mockToastControllerDismiss.mockReturnValue(new Promise(vi.fn()));

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
    center.clear();
    vi.useRealTimers();
  });

  it('Check initial state', async () => {
    const center = new NotificationCenter(t);

    expect(center.notifications).to.deep.equal([]);
    expect(center.getNotifications()).to.deep.equal([]);
    expect(center.hasUnreadNotifications()).to.be.false;
  });

  it('Adds notification to list', async () => {
    expect(center.notifications).to.deep.equal([]);
    center.showModal(NOTIFS[0], {addToList: true});
    expect(center.getNotifications()).to.deep.equal(NOTIFS.slice(0, 1));
    expect(center.hasUnreadNotifications()).to.be.true;

    center.showToast(NOTIFS[1], {addToList: true});
    expect(center.getNotifications()).to.deep.equal(NOTIFS.slice(0, 2));

    center.addToList(NOTIFS[2]);
    expect(center.getNotifications()).to.deep.equal(NOTIFS);

    center.clear();
    expect(center.getNotifications()).to.deep.equal([]);
    expect(center.hasUnreadNotifications()).to.be.false;
  });

  it('Do not add notification to list', async () => {
    expect(center.notifications).to.deep.equal([]);
    center.showModal(NOTIFS[0]);
    expect(center.getNotifications()).to.deep.equal([]);
    expect(center.hasUnreadNotifications()).to.be.false;

    center.showToast(NOTIFS[1]);
    expect(center.getNotifications()).to.deep.equal([]);

    center.addToList(NOTIFS[2]);
    expect(center.getNotifications()).to.deep.equal(NOTIFS.slice(2, 3));
    expect(center.hasUnreadNotifications()).to.be.true;
  });

  it('Mark as read', async () => {
    expect(center.hasUnreadNotifications()).to.be.false;
    center.showModal(NOTIFS[0], {addToList: true});
    expect(center.hasUnreadNotifications()).to.be.true;
    center.markAsRead('1234', true);
    expect(center.hasUnreadNotifications()).to.be.false;
    center.markAsRead('1234', false);
    expect(center.hasUnreadNotifications()).to.be.true;
  });
});

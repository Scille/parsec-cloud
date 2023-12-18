// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Notification as MsNotification, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import { modalController, toastController } from '@ionic/vue';
import { DateTime } from 'luxon';
import { vi } from 'vitest';

describe('Notification Manager', () => {
  // mock ComposerTranslation
  function t(key: string): string {
    const map = new Map<string, string>([['Notification.nextButton', 'nextButton']]);
    return map.get(key) as string;
  }
  let NOTIFS: MsNotification[];
  let notificationManager: NotificationManager;

  beforeEach(() => {
    notificationManager = new NotificationManager(t);

    vi.useFakeTimers();
    vi.setSystemTime(new Date(2000, 1, 1));
    vi.mock('uuid', () => {
      const v4 = vi.fn();
      v4.mockReturnValue('1234');
      return { v4 };
    });
    notificationManager._createAndPresentModal = vi.fn();
    const mockModalControllerDismiss = vi.spyOn(modalController, 'dismiss');
    mockModalControllerDismiss.mockReturnValue(new Promise(vi.fn()));
    const mockToastManagerCreateAndPresent = vi.spyOn(notificationManager.toastManager, 'createAndPresent');
    mockToastManagerCreateAndPresent.mockReturnValue(new Promise(vi.fn()));
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
          key: 'value',
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
    notificationManager.clear();
    vi.useRealTimers();
  });

  it('Check initial state', async () => {
    const notificationManager = new NotificationManager(t);

    expect(notificationManager.notifications).to.deep.equal([]);
    expect(notificationManager.getNotifications()).to.deep.equal([]);
    expect(notificationManager.hasUnreadNotifications()).to.be.false;
  });

  it('Adds notification to list', async () => {
    expect(notificationManager.notifications).to.deep.equal([]);
    notificationManager.showModal(NOTIFS[0], { addToList: true });
    expect(notificationManager.getNotifications()).to.deep.equal(NOTIFS.slice(0, 1));
    expect(notificationManager.hasUnreadNotifications()).to.be.true;

    notificationManager.showToast(NOTIFS[1], { addToList: true });
    expect(notificationManager.getNotifications()).to.deep.equal(NOTIFS.slice(0, 2));

    notificationManager.addToList(NOTIFS[2]);
    expect(notificationManager.getNotifications()).to.deep.equal(NOTIFS);

    notificationManager.clear();
    expect(notificationManager.getNotifications()).to.deep.equal([]);
    expect(notificationManager.hasUnreadNotifications()).to.be.false;
  });

  it('Do not add notification to list', async () => {
    expect(notificationManager.notifications).to.deep.equal([]);
    notificationManager.showModal(NOTIFS[0]);
    expect(notificationManager.getNotifications()).to.deep.equal([]);
    expect(notificationManager.hasUnreadNotifications()).to.be.false;

    notificationManager.showToast(NOTIFS[1]);
    expect(notificationManager.getNotifications()).to.deep.equal([]);

    notificationManager.addToList(NOTIFS[2]);
    expect(notificationManager.getNotifications()).to.deep.equal(NOTIFS.slice(2, 3));
    expect(notificationManager.hasUnreadNotifications()).to.be.true;
  });

  it('Mark as read', async () => {
    expect(notificationManager.hasUnreadNotifications()).to.be.false;
    notificationManager.showModal(NOTIFS[0], { addToList: true });
    expect(notificationManager.hasUnreadNotifications()).to.be.true;
    notificationManager.markAsRead('1234', true);
    expect(notificationManager.hasUnreadNotifications()).to.be.false;
    notificationManager.markAsRead('1234', false);
    expect(notificationManager.hasUnreadNotifications()).to.be.true;
  });
});

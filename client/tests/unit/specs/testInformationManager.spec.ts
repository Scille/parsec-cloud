// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsReportTheme } from '@/components/core';
import { WorkspaceRole } from '@/parsec';
import {
  Information,
  InformationDataType,
  InformationLevel,
  InformationManager,
  InformationOptions,
  PresentationMode,
} from '@/services/informationManager';
import { vi } from 'vitest';

describe('Information Manager', () => {
  let informationManager: InformationManager;
  let INFOS: InformationOptions[];
  let mockToastManagerCreateAndPresent: any;
  let mockShowModal: any;

  beforeEach(() => {
    informationManager = new InformationManager();

    vi.useFakeTimers();
    vi.setSystemTime(new Date(2000, 1, 1));
    vi.mock('uuid', () => {
      const v4 = vi.fn();
      v4.mockReturnValue('1234');
      return { v4 };
    });
    const mockCreateAndPresentModal = vi.spyOn(informationManager as any, '_createAndPresentModal');
    mockCreateAndPresentModal.mockResolvedValue(vi.fn());
    mockShowModal = vi.spyOn(informationManager as any, 'showModal');
    mockToastManagerCreateAndPresent = vi.spyOn(informationManager.toastManager, 'createAndPresent');
    mockToastManagerCreateAndPresent.mockReturnValue(new Promise(vi.fn()));

    INFOS = [
      {
        message: 'information1',
        level: InformationLevel.Info,
      },
      {
        message: 'information2',
        level: InformationLevel.Success,
      },
      {
        message: 'information3',
        level: InformationLevel.Warning,
        data: {
          type: InformationDataType.WorkspaceRoleChanged,
          workspaceHandle: 42,
          oldRole: WorkspaceRole.Reader,
          newRole: WorkspaceRole.Contributor,
        },
      },
      {
        message: 'information4',
        level: InformationLevel.Error,
      },
    ];
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('Check initial state', async () => {
    const informationManager = new InformationManager();

    expect(informationManager).to.exist;
    expect(informationManager.notificationManager).to.exist;
    expect(informationManager.toastManager).to.exist;
    expect(informationManager.notificationManager.getNotifications()).to.deep.equal([]);
  });

  it('should instantiate Information class without errors', () => {
    const informationWithoutData = new Information(INFOS[0]);

    expect(informationWithoutData).to.exist;
    expect(informationWithoutData.data).to.be.undefined;

    const informationWithEmptyData = new Information(INFOS[1]);

    expect(informationWithEmptyData).to.exist;
    expect(informationWithEmptyData.data).to.be.undefined;

    const informationWithSomeData = new Information(INFOS[2]);

    expect(informationWithSomeData).to.exist;
    expect(informationWithSomeData.data).to.deep.equal({
      type: InformationDataType.WorkspaceRoleChanged,
      workspaceHandle: 42,
      oldRole: WorkspaceRole.Reader,
      newRole: WorkspaceRole.Contributor,
    });
  });

  it('should return the correct theme based on the InformationLevel', () => {
    const information1 = new Information(INFOS[0]);
    const information2 = new Information(INFOS[1]);
    const information3 = new Information(INFOS[2]);
    const information4 = new Information(INFOS[3]);

    expect(information1.theme).to.equal(MsReportTheme.Info);
    expect(information2.theme).to.equal(MsReportTheme.Success);
    expect(information3.theme).to.equal(MsReportTheme.Warning);
    expect(information4.theme).to.equal(MsReportTheme.Error);
  });

  it('should send a notification', () => {
    const information = new Information(INFOS[0]);
    informationManager.present(information, PresentationMode.Notification);
    expect(informationManager.notificationManager.getNotifications()).to.have.length(1);
    expect(informationManager.notificationManager.hasUnreadNotifications()).to.be.true;
  });

  it('should show a toast', async () => {
    const information = new Information(INFOS[0]);
    await informationManager.present(information, PresentationMode.Toast);
    expect(mockToastManagerCreateAndPresent.mock.calls.length).to.equal(1);
  });

  it('should show a modal', async () => {
    const information = new Information(INFOS[0]);
    await informationManager.present(information, PresentationMode.Modal);
    expect(mockShowModal.mock.calls.length).to.equal(1);
  });

  it('should trace to console', async () => {
    const information = new Information(INFOS[0]);
    const consoleSpy = vi.spyOn(console, 'trace');
    await informationManager.present(information, PresentationMode.Console);
    expect(consoleSpy.mock.calls.length).to.equal(1);
    expect(consoleSpy.mock.lastCall?.[0]).to.equal(`[${information.level}] ${information.message}`);
  });

  it('should handle forbidden usage of toast and modal at the same time', async () => {
    const information = new Information(INFOS[0]);
    const consoleErrorSpy = vi.spyOn(console, 'error');
    const expectedMessage = 'Error: forbidden usage, toasts and modals are not meant to be used at the same time.';
    await informationManager.present(information, PresentationMode.Toast | PresentationMode.Modal);
    expect(consoleErrorSpy.mock.calls.length).to.equal(1);
    expect(consoleErrorSpy.mock.lastCall?.[0]).to.equal(expectedMessage);
  });
});

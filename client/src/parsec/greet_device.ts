// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { DEFAULT_HANDLE, MOCK_WAITING_TIME, wait } from '@/parsec/internals';
import { getParsecHandle } from '@/parsec/routing';
import {
  ClientNewDeviceInvitationError,
  ClientStartInvitationGreetError,
  ConnectionHandle,
  DeviceGreetInProgress1Info,
  DeviceGreetInProgress2Info,
  DeviceGreetInProgress3Info,
  DeviceGreetInProgress4Info,
  DeviceGreetInitialInfo,
  DeviceLabel,
  GreetInProgressError,
  InvitationEmailSentStatus,
  NewInvitationInfo,
  Result,
} from '@/parsec/types';
import { BackendInvitationAddr, InvitationToken, SASCode, libparsec } from '@/plugins/libparsec';

export class DeviceGreet {
  handle: ConnectionHandle | null;
  canceller: ConnectionHandle | null;
  hostSASCode: SASCode;
  correctSASCode: SASCode;
  SASCodeChoices: SASCode[];
  requestedDeviceLabel: DeviceLabel;
  token: InvitationToken;
  invitationLink: BackendInvitationAddr;

  constructor() {
    this.handle = null;
    this.canceller = null;
    this.hostSASCode = '';
    this.correctSASCode = '';
    this.SASCodeChoices = [];
    this.requestedDeviceLabel = '';
    this.invitationLink = '';
    this.token = '';
  }

  async abort(): Promise<void> {
    if (this.canceller !== null && !needsMocks()) {
      libparsec.cancel(this.canceller);
    }
    if (this.handle !== null && !needsMocks()) {
      libparsec.claimerGreeterAbortOperation(this.handle);
    }
    this.canceller = null;
    this.handle = null;
    this.hostSASCode = '';
    this.requestedDeviceLabel = '';
  }

  _assertState(nullCanceller: boolean, nullHandle: boolean): void {
    if (this.token === '') {
      throw Error('Token should not be null, call createInvitation() first');
    }
    if (this.invitationLink === '') {
      throw Error('Invitation link should not be null, call createInvitation() first');
    }
    if (nullCanceller && this.canceller !== null) {
      throw Error('Canceller should be null');
    } else if (!nullCanceller && this.canceller === null) {
      throw Error('Canceller should not be null');
    }
    if (nullHandle && this.handle !== null) {
      throw Error('Handle should be null');
    } else if (!nullHandle && this.handle === null) {
      throw Error('Handle should not be null');
    }
  }

  async createInvitation(sendEmail = true): Promise<Result<NewInvitationInfo, ClientNewDeviceInvitationError>> {
    const clientHandle = getParsecHandle();

    if (clientHandle !== null && !needsMocks()) {
      const result = await libparsec.clientNewDeviceInvitation(clientHandle, sendEmail);
      if (result.ok) {
        this.invitationLink = result.value.addr;
        this.token = result.value.token;
      }
      return result;
    } else {
      // cspell:disable-next-line
      this.invitationLink = 'parsec://example.parsec.cloud/Org?action=claim_device&token=9ae715f49bc0468eac211e1028f15529';
      // cspell:disable-next-line
      this.token = '9ae715f49bc0468eac211e1028f15529';
      return {
        ok: true,
        value: {
          addr: this.invitationLink,
          token: this.token,
          emailSentStatus: InvitationEmailSentStatus.Success,
        },
      };
    }
  }

  async sendEmail(): Promise<boolean> {
    const clientHandle = getParsecHandle();
    if (clientHandle !== null && !needsMocks()) {
      const result = await libparsec.clientNewDeviceInvitation(clientHandle, true);
      return result.ok;
    } else {
      return true;
    }
  }

  async startGreet(): Promise<Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError>> {
    this._assertState(true, true);
    const clientHandle = getParsecHandle();

    if (clientHandle !== null && !needsMocks()) {
      const result = await libparsec.clientStartDeviceInvitationGreet(clientHandle, this.token);

      if (result.ok) {
        this.handle = result.value.handle;
      }
      return result;
    } else {
      this.handle = DEFAULT_HANDLE;
      await wait(MOCK_WAITING_TIME);
      return { ok: true, value: { handle: DEFAULT_HANDLE } };
    }
  }

  async initialWaitGuest(): Promise<Result<DeviceGreetInProgress1Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.greeterDeviceInitialDoWaitPeer(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
        this.hostSASCode = result.value.greeterSas;
      }
      return result;
    } else {
      this.hostSASCode = '2EDF';
      return {
        ok: true,
        value: { handle: DEFAULT_HANDLE, greeterSas: this.hostSASCode },
      };
    }
  }

  async waitGuestTrust(): Promise<Result<DeviceGreetInProgress2Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.greeterDeviceInProgress1DoWaitPeerTrust(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
        this.SASCodeChoices = result.value.claimerSasChoices;
        this.correctSASCode = result.value.claimerSas;
      }
      return result;
    } else {
      await wait(MOCK_WAITING_TIME);
      this.SASCodeChoices = ['1ABC', '2DEF', '3GHI', '4JKL'];
      this.correctSASCode = '2DEF';
      return {
        ok: true,
        value: {
          handle: DEFAULT_HANDLE,
          claimerSasChoices: this.SASCodeChoices,
          claimerSas: this.correctSASCode,
        },
      };
    }
  }

  async signifyTrust(): Promise<Result<DeviceGreetInProgress3Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.greeterDeviceInProgress2DoSignifyTrust(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
      }
      return result;
    } else {
      return { ok: true, value: { handle: DEFAULT_HANDLE } };
    }
  }

  async getClaimRequests(): Promise<Result<DeviceGreetInProgress4Info, GreetInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.greeterDeviceInProgress3DoGetClaimRequests(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
        this.requestedDeviceLabel = result.value.requestedDeviceLabel || '';
      }
      return result;
    } else {
      await wait(MOCK_WAITING_TIME);
      this.requestedDeviceLabel = 'My Device';
      return {
        ok: true,
        value: {
          handle: DEFAULT_HANDLE,
          requestedDeviceLabel: this.requestedDeviceLabel,
        },
      };
    }
  }

  async createDevice(): Promise<Result<null, GreetInProgressError>> {
    this._assertState(true, false);
    if (!needsMocks()) {
      this.canceller = await libparsec.newCanceller();
      const result = await libparsec.greeterDeviceInProgress4DoCreate(
        this.canceller,
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.handle!,
        this.requestedDeviceLabel,
      );
      this.canceller = null;
      this.handle = null;
      return result;
    } else {
      this.handle = null;
      return { ok: true, value: null };
    }
  }
}

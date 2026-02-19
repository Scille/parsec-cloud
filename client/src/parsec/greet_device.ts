// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
  NewInvitationInfo,
  Result,
  createDeviceInvitation,
} from '@/parsec';
import { generateNoHandleError } from '@/parsec/utils';
import { AccessToken, ParsecInvitationAddr, ParsecInvitationRedirectionURL, SASCode, libparsec } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';

export class DeviceGreet {
  handle: ConnectionHandle | null;
  canceller: ConnectionHandle | null;
  hostSASCode: SASCode;
  correctSASCode: SASCode;
  SASCodeChoices: SASCode[];
  requestedDeviceLabel: DeviceLabel;
  token: AccessToken;
  invitationLink: ParsecInvitationAddr | ParsecInvitationRedirectionURL;

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
    if (this.canceller !== null) {
      await libparsec.cancel(this.canceller);
    }
    if (this.handle !== null) {
      await libparsec.claimerGreeterAbortOperation(this.handle);
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

  setInvitationInformation(invitationLink: ParsecInvitationAddr | ParsecInvitationRedirectionURL, token: string): void {
    this.invitationLink = invitationLink;
    this.token = token;
  }

  async createInvitation(sendEmail = true): Promise<Result<NewInvitationInfo, ClientNewDeviceInvitationError>> {
    const result = await createDeviceInvitation(sendEmail);

    if (result.ok) {
      const [invitationAddr, _] = result.value.addr;
      this.invitationLink = invitationAddr;
      this.token = result.value.token;
    }
    return result;
  }

  async sendEmail(): Promise<boolean> {
    const clientHandle = getConnectionHandle();

    if (clientHandle !== null) {
      const result = await libparsec.clientNewDeviceInvitation(clientHandle, true);
      return result.ok;
    }
    return false;
  }

  async startGreet(): Promise<Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError>> {
    this._assertState(true, true);
    const clientHandle = getConnectionHandle();

    if (clientHandle !== null) {
      const result = await libparsec.clientStartDeviceInvitationGreet(clientHandle, this.token);

      if (result.ok) {
        this.handle = result.value.handle;
      }
      return result;
    }
    return generateNoHandleError<ClientStartInvitationGreetError>();
  }

  async initialWaitGuest(): Promise<Result<DeviceGreetInProgress1Info, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterDeviceInitialDoWaitPeer(this.canceller, this.handle!);
    this.canceller = null;
    if (result.ok) {
      this.handle = result.value.handle;
      this.hostSASCode = result.value.greeterSas;
    }
    return result;
  }

  async waitGuestTrust(): Promise<Result<DeviceGreetInProgress2Info, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterDeviceInProgress1DoWaitPeerTrust(this.canceller, this.handle!);
    if (result.ok) {
      this.handle = result.value.handle;
      this.SASCodeChoices = result.value.claimerSasChoices;
      this.correctSASCode = result.value.claimerSas;
    }
    this.canceller = null;
    return result;
  }

  async denyTrust(): Promise<Result<null, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterDeviceInProgress2DoDenyTrust(this.canceller, this.handle!);
    this.handle = null;
    this.canceller = null;
    return result;
  }

  async signifyTrust(): Promise<Result<DeviceGreetInProgress3Info, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterDeviceInProgress2DoSignifyTrust(this.canceller, this.handle!);
    if (result.ok) {
      this.handle = result.value.handle;
    }
    this.canceller = null;
    return result;
  }

  async getClaimRequests(): Promise<Result<DeviceGreetInProgress4Info, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterDeviceInProgress3DoGetClaimRequests(this.canceller, this.handle!);
    this.canceller = null;
    if (result.ok) {
      this.handle = result.value.handle;
      this.requestedDeviceLabel = result.value.requestedDeviceLabel || '';
    }
    return result;
  }

  async createDevice(): Promise<Result<null, GreetInProgressError>> {
    this._assertState(true, false);
    this.canceller = await libparsec.newCanceller();
    const result = await libparsec.greeterDeviceInProgress4DoCreate(this.canceller, this.handle!, this.requestedDeviceLabel);
    this.canceller = null;
    this.handle = null;
    return result;
  }
}

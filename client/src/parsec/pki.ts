// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultDeviceName } from '@/common/device';
import { getClientInfo } from '@/parsec/login';
import { getOrganizationInfo } from '@/parsec/organization';
import {
  AvailableDevice,
  AvailableDeviceTypeTag,
  HumanHandle,
  OrganizationID,
  ParsecAddr,
  ParsecOrganizationAddr,
  Result,
  UserProfile,
} from '@/parsec/types';
import { DateTime } from 'luxon';

/*
 * This is all mocked to be able to progress the GUI side without the bindings.
 */

export enum JoinRequestStatus {
  Pending = 'pending',
  Accepted = 'accepted',
  Rejected = 'rejected',
  Cancelled = 'cancelled',
}

export enum JoinRequestValidity {
  Unknown = 'unknown',
  Valid = 'valid',
  Invalid = 'invalid',
}

export interface LocalJoinRequest {
  status: JoinRequestStatus;
  humanHandle: HumanHandle;
  organization: OrganizationID;
  server: ParsecAddr;
  certificate: string;
}

export enum RequestJoinOrganizationErrorTag {
  Internal = 'internal',
}

export interface RequestJoinOrganizationError {
  tag: RequestJoinOrganizationErrorTag;
  error: string;
}

export interface OrganizationJoinRequest {
  humanHandle: HumanHandle;
  certificate: string;
  createdOn: DateTime;
  validity: JoinRequestValidity;
}

export enum ListLocalJoinRequestErrorTag {
  Internal = 'internal',
}

interface ListLocalJoinRequestError {
  tag: ListLocalJoinRequestErrorTag;
  error: string;
}

export enum ListOrganizationJoinRequestErrorTag {
  Internal = 'internal',
}

interface ListOrganizationJoinRequestError {
  tag: ListOrganizationJoinRequestErrorTag;
  error: string;
}

export enum UpdateLocalJoinStatusErrorTag {
  Internal = 'internal',
  InvalidStatus = 'invalid-status',
  NotFound = 'not-found',
}

interface UpdateLocalJoinStatusError {
  tag: UpdateLocalJoinStatusErrorTag;
  error: string;
}

export enum UpdateOrganizationJoinStatusErrorTag {
  Internal = 'internal',
  InvalidStatus = 'invalid-status',
  NotFound = 'not-found',
  InsufficientPermissions = 'insufficient-permissions',
}

interface UpdateOrganizationJoinStatusError {
  tag: UpdateOrganizationJoinStatusErrorTag;
  error: string;
}

export enum ValidateOrganizationJoinRequestErrorTag {
  Internal = 'internal',
}

interface ValidateOrganizationJoinRequestError {
  tag: ValidateOrganizationJoinRequestErrorTag;
  error: string;
}

export enum GetPkiJoinOrganizationLinkErrorTag {
  Internal = 'internal',
}

interface GetPkiJoinOrganizationLinkError {
  tag: GetPkiJoinOrganizationLinkErrorTag;
  error: string;
}

interface _Request {
  humanHandle: HumanHandle;
  certificate: string;
  validity: JoinRequestValidity;
  createdOn: DateTime;
  organization: OrganizationID;
  server: string;
  status: JoinRequestStatus;
}

const REQUESTS: Array<_Request> = [];

export async function requestJoinOrganization(
  link: ParsecOrganizationAddr,
): Promise<Result<LocalJoinRequest, RequestJoinOrganizationError>> {
  // Will be prompted for the certificate
  const newRequest: _Request = {
    status: JoinRequestStatus.Pending,
    validity: JoinRequestValidity.Unknown,
    createdOn: DateTime.utc(),
    humanHandle: {
      // cspell:disable-next-line
      label: 'Isaac Kleiner',
      // cspell:disable-next-line
      email: 'isaac.kleiner@blackmesa.nm',
    },
    organization: 'Black Mesa',
    server: link,
    certificate: '',
  };
  REQUESTS.push(newRequest);
  return {
    ok: true,
    value: {
      humanHandle: newRequest.humanHandle,
      status: newRequest.status,
      organization: newRequest.organization,
      server: newRequest.server,
      certificate: newRequest.certificate,
    },
  };
}

export async function listLocalJoinRequests(error = false): Promise<Result<Array<LocalJoinRequest>, ListLocalJoinRequestError>> {
  if (error) {
    return { ok: false, error: { tag: ListLocalJoinRequestErrorTag.Internal, error: 'generic error' } };
  }
  return {
    ok: true,
    value: REQUESTS.map((r) => {
      return {
        humanHandle: r.humanHandle,
        status: r.status,
        organization: r.organization,
        server: r.server,
        certificate: r.certificate,
      };
    }),
  };
}

export async function confirmLocalJoinRequest(request: LocalJoinRequest): Promise<Result<AvailableDevice, UpdateLocalJoinStatusError>> {
  if (request.status !== JoinRequestStatus.Accepted) {
    return { ok: false, error: { tag: UpdateLocalJoinStatusErrorTag.InvalidStatus, error: 'request not accepted' } };
  }
  const idx = REQUESTS.findIndex((r) => r.certificate === request.certificate);
  if (idx === -1) {
    return { ok: false, error: { tag: UpdateLocalJoinStatusErrorTag.NotFound, error: 'request does not exist' } };
  }
  REQUESTS.splice(idx, 1);
  return {
    ok: true,
    value: {
      keyFilePath: '/',
      createdOn: DateTime.utc(),
      protectedOn: DateTime.utc(),
      serverUrl: request.server,
      organizationId: request.organization,
      userId: 'user_id',
      deviceId: 'device_id',
      humanHandle: {
        label: request.humanHandle.label,
        email: request.humanHandle.email,
      },
      deviceLabel: getDefaultDeviceName(),
      ty: {
        tag: AvailableDeviceTypeTag.Smartcard,
      },
    },
  };
}

export async function cancelLocalJoinRequest(request: LocalJoinRequest): Promise<Result<null, UpdateLocalJoinStatusError>> {
  const idx = REQUESTS.findIndex((r) => r.certificate === request.certificate);
  if (idx === -1) {
    return { ok: false, error: { tag: UpdateLocalJoinStatusErrorTag.NotFound, error: 'request does not exist' } };
  }
  REQUESTS.splice(idx, 1);

  return { ok: true, value: null };
}

export async function listOrganizationJoinRequests(
  rootCertificate?: string,
  error = false,
): Promise<Result<Array<OrganizationJoinRequest>, ListOrganizationJoinRequestError>> {
  const joinRequests: OrganizationJoinRequest[] = [];
  if (error) {
    return { ok: false, error: { tag: ListOrganizationJoinRequestErrorTag.Internal, error: 'generic error' } };
  }
  if (!rootCertificate) {
    return { ok: true, value: joinRequests };
  } else {
    const updated: Array<OrganizationJoinRequest> = [];
    for (const jr of joinRequests) {
      const result = await isValidOrganizationJoinRequest(rootCertificate, jr);
      if (result.ok) {
        updated.push({ ...jr, validity: result.value ? JoinRequestValidity.Valid : JoinRequestValidity.Invalid });
      }
    }
    return { ok: true, value: updated };
  }
}

export async function acceptOrganizationJoinRequest(
  request: OrganizationJoinRequest,
  _profile: UserProfile,
): Promise<Result<null, UpdateOrganizationJoinStatusError>> {
  const joinRequests: OrganizationJoinRequest[] = [];
  const idx = joinRequests.findIndex((r) => r.certificate === request.certificate);
  if (idx === -1) {
    return { ok: false, error: { tag: UpdateOrganizationJoinStatusErrorTag.NotFound, error: 'request does not exist' } };
  }
  const result = await getClientInfo();
  if (!result.ok || result.value.currentProfile !== UserProfile.Admin) {
    return { ok: false, error: { tag: UpdateOrganizationJoinStatusErrorTag.InsufficientPermissions, error: 'not admin' } };
  }
  joinRequests.splice(idx, 1);
  return { ok: true, value: null };
}

export async function rejectOrganizationJoinRequest(
  request: OrganizationJoinRequest,
): Promise<Result<null, UpdateOrganizationJoinStatusError>> {
  const joinRequests: OrganizationJoinRequest[] = [];
  const idx = joinRequests.findIndex((r) => r.certificate === request.certificate);
  if (idx === -1) {
    return { ok: false, error: { tag: UpdateOrganizationJoinStatusErrorTag.NotFound, error: 'request does not exist' } };
  }
  const result = await getClientInfo();
  if (!result.ok || result.value.currentProfile !== UserProfile.Admin) {
    return { ok: false, error: { tag: UpdateOrganizationJoinStatusErrorTag.InsufficientPermissions, error: 'not admin' } };
  }
  return { ok: true, value: null };
}

export async function isValidOrganizationJoinRequest(
  rootCertificate: string,
  request: OrganizationJoinRequest,
): Promise<Result<boolean, ValidateOrganizationJoinRequestError>> {
  return { ok: true, value: rootCertificate.trim() === request.certificate.trim() };
}

export async function getPkiJoinOrganizationLink(): Promise<Result<string, GetPkiJoinOrganizationLinkError>> {
  const result = await getOrganizationInfo();
  if (!result.ok) {
    return { ok: false, error: { tag: GetPkiJoinOrganizationLinkErrorTag.Internal, error: 'failed to get organization info' } };
  }
  if (result.value.organizationAddr.includes('?')) {
    return { ok: true, value: `${result.value.organizationAddr}&a=pki_join` };
  }
  return { ok: true, value: `${result.value.organizationAddr}?a=pki_join` };
}

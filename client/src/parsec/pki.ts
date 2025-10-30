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
  ShowCertificateSelectionDialogError,
  ShowCertificateSelectionDialogErrorTag,
  UserProfile,
  X509CertificateReference,
} from '@/parsec/types';
import { libparsec } from '@/plugins/libparsec';
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

export const TMP_ROOT_CERTIFICATE = `-----BEGIN CERTIFICATE-----
MIIDpzCCAo+gAwIBAgIUQkM5bGFja01lc2FfVGVzdF9DQV8xMB4XDTI1MDkwMTAw
MDAwMFoXDTI3MDkwMTAwMDAwMFowfTELMAkGA1UEBhMCTkExDzANBgNVBAgMBk5l
d21leDEPMA0GA1UEBwwGTmV3IE1lejEVMBMGA1UECgwMQmxhY2sgTWVzYTEPMA0G
A1UECwwGUmVzZWFyY2gxJTAjBgNVBAMMHEdvcmRvbiBGcmVlbWFuIChCbGFjayBN
ZXNhKTEhMB8GCSqGSIb3DQEJARYSZ29yZG9uLmZyZWVtYW5AYmxhY2ttZXNhLm5t
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAw2V9gX9q9S1a0xi0n1Jq
0kQH0h6hO1f3rH5uQv4N1Uu3o7bHq0mB4YgG11k7m7L0F6m3q8zq2dZ8V9b3hY6V
RkR0Y3dObW9jay1vbmx5LWtleS1kYXRhLWJpdHMtcGxhY2Vob2xkZXIxMjM0NTY3
ODkwYWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXoxMjM0NTY3ODkwYWJjZGVmZ2hp
amtsbW5vcHFyK1RoaXNJc0Zha2VQdWJsaWNLZXlEYXRhQmFzZTY0XzEyMzQ1Njc4
OTBBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWjQ1Nk1PQ0tNT0NLRUQwQwIDAQAB
o4GSMIGPMA4GA1UdDwEB/wQEAwIFoDATBgNVHSUEDDAKBggrBgEFBQcDATAMBgNV
HRMBAf8EAjAAMB0GA1UdDgQWBBRGbWVzYV9Nb2NrX1NJR05FUl9OT1QwHwYDVR0j
BBgwFoAUQmxhY2tNZXNhX1Rlc3RfQ0FfUm9vdDApBgNVHREEIjAggR5nb3Jkb24u
ZnJlZW1hbkBibGFja21lc2Eubm2HBH8AAAEwEwYDVR0lBAwwCgYIKwYBBQUHAwEw
DQYJKoZIhvcNAQELBQADggEBABp0UX0uQm9ndXMtbW9jay1zaWduYXR1cmUtcGFk
ZGVkLXN0cmluZy1oZXJlLTEyMzQ1Njc4OTBhYmNkZWYxMjM0NTY3ODkwYWJjZGUx
MjM0NTZtb2NrLWJvZHktdGhhdC1sb29rcy1yZWFsaXN0aWMtZW5vdWdoLWxvbmcg
dG8tZm9vbC10b2xzLWFuZC10ZXN0cy1idXQtZmFrZS1mb3ItZGVtbzo7Ozs7Ozs7
Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7O2VuZC1vZi1tb2NrLXNpZ25hdHVyZQ==
-----END CERTIFICATE-----`;

const LOCAL_JOIN_REQUESTS = [
  {
    status: JoinRequestStatus.Pending,
    humanHandle: {
      label: 'Gordon Freeman',
      email: 'gordon.freeman@blackmesa.nm',
    },
    organization: 'Black Mesa',
    server: 'parsec3://localhost:6770?no_ssl=true',
    certificate: `-----BEGIN CERTIFICATE-----
MIIDpzCCAo+gAwIBAgIUQkM5bGFja01lc2FfVGVzdF9DQV8xMB4XDTI1MDkwMTAw
MDAwMFoXDTI3MDkwMTAwMDAwMFowfTELMAkGA1UEBhMCTkExDzANBgNVBAgMBk5l
d21leDEPMA0GA1UEBwwGTmV3IE1lejEVMBMGA1UECgwMQmxhY2sgTWVzYTEPMA0G
A1UECwwGUmVzZWFyY2gxJTAjBgNVBAMMHEdvcmRvbiBGcmVlbWFuIChCbGFjayBN
ZXNhKTEhMB8GCSqGSIb3DQEJARYSZ29yZG9uLmZyZWVtYW5AYmxhY2ttZXNhLm5t
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAw2V9gX9q9S1a0xi0n1Jq
0kQH0h6hO1f3rH5uQv4N1Uu3o7bHq0mB4YgG11k7m7L0F6m3q8zq2dZ8V9b3hY6V
RkR0Y3dObW9jay1vbmx5LWtleS1kYXRhLWJpdHMtcGxhY2Vob2xkZXIxMjM0NTY3
ODkwYWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXoxMjM0NTY3ODkwYWJjZGVmZ2hp
amtsbW5vcHFyK1RoaXNJc0Zha2VQdWJsaWNLZXlEYXRhQmFzZTY0XzEyMzQ1Njc4
OTBBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWjQ1Nk1PQ0tNT0NLRUQwQwIDAQAB
o4GSMIGPMA4GA1UdDwEB/wQEAwIFoDATBgNVHSUEDDAKBggrBgEFBQcDATAMBgNV
HRMBAf8EAjAAMB0GA1UdDgQWBBRGbWVzYV9Nb2NrX1NJR05FUl9OT1QwHwYDVR0j
BBgwFoAUQmxhY2tNZXNhX1Rlc3RfQ0FfUm9vdDApBgNVHREEIjAggR5nb3Jkb24u
ZnJlZW1hbkBibGFja21lc2Eubm2HBH8AAAEwEwYDVR0lBAwwCgYIKwYBBQUHAwEw
DQYJKoZIhvcNAQELBQADggEBABp0UX0uQm9ndXMtbW9jay1zaWduYXR1cmUtcGFk
ZGVkLXN0cmluZy1oZXJlLTEyMzQ1Njc4OTBhYmNkZWYxMjM0NTY3ODkwYWJjZGUx
MjM0NTZtb2NrLWJvZHktdGhhdC1sb29rcy1yZWFsaXN0aWMtZW5vdWdoLWxvbmcg
dG8tZm9vbC10b2xzLWFuZC10ZXN0cy1idXQtZmFrZS1mb3ItZGVtbzo7Ozs7Ozs7
Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7O2VuZC1vZi1tb2NrLXNpZ25hdHVyZQ==
-----END CERTIFICATE-----`,
  },
  {
    status: JoinRequestStatus.Rejected,
    humanHandle: {
      label: 'GLaDOS',
      email: 'glados@aperturescience.mi',
    },
    organization: 'Aperture Science',
    server: 'parsec3://localhost:6770?no_ssl=true',
    certificate: `-----BEGIN CERTIFICATE-----
MIIDqTCCApGgAwIBAgIUTmljZV9Nb2NrX0NhX0lkZW50aXR5MB4XDTI1MDkwMTAw
MDAwMFoXDTI3MDkwMTAwMDAwMFowgZ0xCzAJBgNVBAYTAk5BMRAwDgYDVQQIDAdJ
bnRlcnN0MRAwDgYDVQQHDAdJbnRlcnN0MRwwGgYDVQQKDBNXZXlsYW5kLVl1dGFu
aSBDb3JwLjEWMBQGA1UECwwNRXhwbG9yYXRpb24gQ3JldzEhMB8GA1UEAwwYRWxs
ZW4gUmlwbGV5IChXZXlsYW5kLVl1dGFuaSkxITAfBgkqhkiG9w0BCQEWEmVyaXBs
ZXlAd2V5dHVhbmkuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA
a2Ztb2NrLWNlcnRpZmljYXRlLWJhc2U2NC1ibG9iLWZpbGxlci1oZXJlLTEyMzQ1
Njc4OTBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5ejEyMzQ1Njc4OTBhYmNkZWZn
aGlqa2xtbm9wcXJzdHV2d3h5emZha2Utc2lnbmF0dXJlLXN0cmluZy1tb2NrLWRh
dGEtYmFzZTY0LWhvbGRlci0xMjM0NTY3ODkwYWJjZGVmZwIDAQABo4GSMIGPMA4G
A1UdDwEB/wQEAwIFoDATBgNVHSUEDDAKBggrBgEFBQcDATAMBgNVHRMBAf8EAjAA
MB0GA1UdDgQWBBQkUmlwbGV5X0Zha2VfU2lnbmVyMB8GA1UdIwQYMBaAFHdleWxh
bmRfeXV0YW5pX1Rlc3RfQ0EwKQYDVR0RBCIwIIEWZXJpcGxleUB3ZXlsYW5keXUu
Y29thwR/AAABMBMGA1UdJQQMMAoGCCsGAQUFBwMBMA0GCSqGSIb3DQEBCwUAA4IB
AQCMbW9jay1ibG9iLWZvci1yZXBsZXktd2V5bGFuZC15dXRhbmktc2FmZS1tdWNr
LXZhbHVlLTEyMzQ1Njc4OTBhYmNkZWYxMjM0NTY3ODkwYWJjZGVmbW9jay1zaWdu
YXR1cmUtZW5kLW9mLWZpbGxlci1ibG9i
-----END CERTIFICATE-----`,
  },
  {
    status: JoinRequestStatus.Accepted,
    humanHandle: {
      label: 'Eli Vance',
      email: 'eli.vance@blackmesa.nm',
    },
    organization: 'Black Mesa',
    server: 'parsec3://localhost:6770?no_ssl=true',
    certificate: `-----BEGIN CERTIFICATE-----
MIIDqzCCApOgAwIBAgIUc2xpZWltb2NrY2Fmb29iYXJmMB4XDTI1MDkwMTAwMDAw
MFoXDTI3MDkwMTAwMDAwMFowgZ8xCzAJBgNVBAYTAk5ZMRAwDgYDVQQIDAdOZXcg
WW9yazEQMA4GA1UEBwwHTmV3IFlvcmsxFzAVBgNVBAoMDkdob3N0YnVzdGVycyBJ
bmMuMRQwEgYDVQQLDAtQYXJhbm9ybWFsMR8wHQYDVQQDDBZQZXRlciBWZW5rbWFu
IChHb3N0YnVzdGVyKTEhMB8GCSqGSIb3DQEJARYUcGV0ZXIudmVua21hbkBnaG9z
dC5ueYwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQDoZmFrZS1iYXNl
NjQtZGF0YS1ibG9iLWZvci12ZW5rbWFuLWdob3N0YnVzdGVycy1pYy1jZXJ0aWZp
Y2F0ZS1wZW0tc2FmZS1ibG9iLTEyMzQ1Njc4OTBhYmNkZWZnaGlqa2xtbm9wcXJz
dHV2d3h5ejEyMzQ1Njc4OTBhYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5ei1zaWdu
YXR1cmUtZmFrZS1kYXRhLWZpbGxlci1ibG9iLWhlcmUtZW5kcwIDAQABo4GSMIGP
MA4GA1UdDwEB/wQEAwIFoDATBgNVHSUEDDAKBggrBgEFBQcDATAMBgNVHRMBAf8E
AjAAMB0GA1UdDgQWBBRlbnhtYW5fRmFrZV9DZXJ0X1NpZ25lcjAfBgNVHSMEGDAW
gBTBZmFrZV9HaG9zdGJ1c3RlcnNfQ0FfUm9vdDApBgNVHREEIjAggRZwZXRlci52
ZW5rbWFuQGdob3N0Lm55hwR/AAABMBMGA1UdJQQMMAoGCCsGAQUFBwMBMA0GCSqG
SIb3DQEBCwUAA4IBAQBgYmxvYi1mYWtlLW1vY2stdGVzdC1nbG9iLWJ1c3RlcnMt
Y2VydGlmaWNhdGUtYmFzZTY0LWRhdGEtdGhhdC1sb29rcy1yZWFsaXN0aWMtYnV0
LWlzLW5vdC1hY3R1YWxseS12YWxpZC1zdHJpbmctYmFzZTY0LWZpbGxlci1ibG9i
LXNhdmUtZm9yLWZha2UtdXNlcw==
-----END CERTIFICATE-----`,
  },
];

const ORGANIZATION_JOIN_REQUESTS: Array<OrganizationJoinRequest> = LOCAL_JOIN_REQUESTS.map((ljr) => {
  return {
    humanHandle: ljr.humanHandle,
    certificate: ljr.certificate,
    validity: JoinRequestValidity.Unknown,
    createdOn: DateTime.utc(),
  };
});

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
    certificate: TMP_ROOT_CERTIFICATE,
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
  if (error) {
    return { ok: false, error: { tag: ListOrganizationJoinRequestErrorTag.Internal, error: 'generic error' } };
  }
  if (!rootCertificate) {
    return { ok: true, value: ORGANIZATION_JOIN_REQUESTS };
  } else {
    const updated: Array<OrganizationJoinRequest> = [];
    for (const jr of ORGANIZATION_JOIN_REQUESTS) {
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
  const idx = ORGANIZATION_JOIN_REQUESTS.findIndex((r) => r.certificate === request.certificate);
  if (idx === -1) {
    return { ok: false, error: { tag: UpdateOrganizationJoinStatusErrorTag.NotFound, error: 'request does not exist' } };
  }
  const result = await getClientInfo();
  if (!result.ok || result.value.currentProfile !== UserProfile.Admin) {
    return { ok: false, error: { tag: UpdateOrganizationJoinStatusErrorTag.InsufficientPermissions, error: 'not admin' } };
  }
  ORGANIZATION_JOIN_REQUESTS.splice(idx, 1);
  return { ok: true, value: null };
}

export async function rejectOrganizationJoinRequest(
  request: OrganizationJoinRequest,
): Promise<Result<null, UpdateOrganizationJoinStatusError>> {
  const idx = ORGANIZATION_JOIN_REQUESTS.findIndex((r) => r.certificate === request.certificate);
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

export async function selectCertificate(): Promise<Result<X509CertificateReference | null, ShowCertificateSelectionDialogError>> {
  if (!(await isSmartcardAvailable())) {
    return { ok: false, error: { tag: ShowCertificateSelectionDialogErrorTag.CannotOpenStore, error: 'smartcard not available' } };
  }
  return await libparsec.showCertificateSelectionDialogWindowsOnly();
}

export async function isSmartcardAvailable(): Promise<boolean> {
  return false;
}

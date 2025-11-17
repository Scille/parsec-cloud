// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultDeviceName } from '@/common/device';
import { getClientConfig } from '@/parsec/internals';
import {
  AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyTag,
  AsyncEnrollmentRequest,
  AsyncEnrollmentUntrusted,
  AvailableDevice,
  AvailableDeviceTypeTag,
  AvailablePendingAsyncEnrollmentIdentitySystemTag,
  ClientAcceptAsyncEnrollmentError,
  ClientGetAsyncEnrollmentAddrError,
  ClientListAsyncEnrollmentsError,
  ClientRejectAsyncEnrollmentError,
  DeviceSaveStrategy,
  OpenBaoListSelfEmailsError,
  ParsecAsyncEnrollmentAddr,
  ParsedParsecAddrTag,
  PendingAsyncEnrollmentInfoTag,
  Result,
  ShowCertificateSelectionDialogError,
  ShowCertificateSelectionDialogErrorTag,
  SubmitAsyncEnrollmentError,
  SubmitAsyncEnrollmentIdentityStrategy,
  SubmitAsyncEnrollmentIdentityStrategyOpenBao,
  SubmitAsyncEnrollmentIdentityStrategyPKI,
  SubmitAsyncEnrollmentIdentityStrategyTag,
  SubmitterCancelAsyncEnrollmentError,
  SubmitterFinalizeAsyncEnrollmentError,
  SubmitterListLocalAsyncEnrollmentsError,
  UserProfile,
  X509CertificateReference,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { AsyncEnrollmentIdentitySystemTag, HumanHandle, libparsec, X509URIFlavorValueTag } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';
import { OpenBaoClient } from '@/services/openBao';
import { DateTime } from 'luxon';

const _ASYNC_ENROLLMENT_PARSEC_API = {
  async requestJoinOrganization(
    link: ParsecAsyncEnrollmentAddr,
    identityStrategy: SubmitAsyncEnrollmentIdentityStrategy,
  ): Promise<Result<null, SubmitAsyncEnrollmentError>> {
    const result = await libparsec.submitAsyncEnrollment(getClientConfig(), link, true, getDefaultDeviceName(), identityStrategy);
    if (result.ok) {
      return { ok: true, value: null };
    }
    return result;
  },

  async listJoinRequests(): Promise<Result<Array<AsyncEnrollmentRequest>, SubmitterListLocalAsyncEnrollmentsError>> {
    const result = await libparsec.submitterListAsyncEnrollments(getClientConfig().configDir);

    if (!result.ok) {
      return result;
    }
    const list: Array<AsyncEnrollmentRequest> = [];
    for (const enrollment of result.value) {
      enrollment.submittedOn = DateTime.fromSeconds(enrollment.submittedOn as any as number);
      const infoResult = await libparsec.submitterGetAsyncEnrollmentInfo(getClientConfig(), enrollment.addr, enrollment.enrollmentId);
      const addrResult = await libparsec.parseParsecAddr(enrollment.addr);
      if (infoResult.ok && addrResult.ok && addrResult.value.tag !== ParsedParsecAddrTag.Server) {
        infoResult.value.submittedOn = DateTime.fromSeconds(infoResult.value.submittedOn as any as number);
        if (infoResult.value.tag === PendingAsyncEnrollmentInfoTag.Accepted) {
          infoResult.value.acceptedOn = DateTime.fromSeconds(infoResult.value.acceptedOn as any as number);
        } else if (infoResult.value.tag === PendingAsyncEnrollmentInfoTag.Cancelled) {
          infoResult.value.cancelledOn = DateTime.fromSeconds(infoResult.value.cancelledOn as any as number);
        } else if (infoResult.value.tag === PendingAsyncEnrollmentInfoTag.Rejected) {
          infoResult.value.rejectedOn = DateTime.fromSeconds(infoResult.value.rejectedOn as any as number);
        }
        const request = { info: infoResult.value, enrollment: enrollment, organizationId: addrResult.value.organizationId };

        if (infoResult.value.tag === PendingAsyncEnrollmentInfoTag.Cancelled) {
          await deleteJoinRequest(request);
        } else {
          list.push(request);
        }
      }
    }
    return { ok: true, value: list };
  },

  async confirmJoinRequest(
    request: AsyncEnrollmentRequest,
    saveStrategy: DeviceSaveStrategy,
    identityStrategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  ): Promise<Result<AvailableDevice, SubmitterFinalizeAsyncEnrollmentError>> {
    (request.enrollment.submittedOn as any as number) = request.enrollment.submittedOn.toSeconds();
    const result = await libparsec.submitterFinalizeAsyncEnrollment(
      getClientConfig(),
      request.enrollment.filePath,
      saveStrategy,
      identityStrategy,
    );
    request.enrollment.submittedOn = DateTime.fromSeconds(request.enrollment.submittedOn as any as number);
    if (result.ok) {
      result.value.createdOn = DateTime.fromSeconds(result.value.createdOn as any as number);
      result.value.protectedOn = DateTime.fromSeconds(result.value.protectedOn as any as number);
    }
    return result;
  },

  async deleteJoinRequest(request: AsyncEnrollmentRequest): Promise<Result<null, SubmitterCancelAsyncEnrollmentError>> {
    return await libparsec.submitterCancelAsyncEnrollment(getClientConfig(), request.enrollment.addr, request.enrollment.enrollmentId);
  },

  async listAsyncEnrollments(): Promise<Result<Array<AsyncEnrollmentUntrusted>, ClientListAsyncEnrollmentsError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientListAsyncEnrollmentsError>();
    }

    const result = await libparsec.clientListAsyncEnrollments(handle);
    if (result.ok) {
      result.value = result.value.map((item) => {
        item.submittedOn = DateTime.fromSeconds(item.submittedOn as any as number);
        return item;
      });
    }
    return result;
  },

  async acceptAsyncEnrollment(
    request: AsyncEnrollmentUntrusted,
    profile: UserProfile,
    identityStrategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  ): Promise<Result<null, ClientAcceptAsyncEnrollmentError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientAcceptAsyncEnrollmentError>();
    }
    return await libparsec.clientAcceptAsyncEnrollment(handle, profile, request.enrollmentId, identityStrategy);
  },

  async rejectAsyncEnrollment(request: AsyncEnrollmentUntrusted): Promise<Result<null, ClientRejectAsyncEnrollmentError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientRejectAsyncEnrollmentError>();
    }
    return await libparsec.clientRejectAsyncEnrollment(handle, request.enrollmentId);
  },

  async selectCertificate(): Promise<Result<X509CertificateReference | undefined, ShowCertificateSelectionDialogError>> {
    if (!(await isSmartcardAvailable())) {
      return { ok: false, error: { tag: ShowCertificateSelectionDialogErrorTag.CannotOpenStore, error: 'smartcard not available' } };
    }
    const result = await libparsec.showCertificateSelectionDialogWindowsOnly();
    if (result.ok && result.value === null) {
      return { ok: true, value: undefined };
    }
    return result as Result<X509CertificateReference | undefined, ShowCertificateSelectionDialogError>;
  },

  async isSmartcardAvailable(): Promise<boolean> {
    return await libparsec.isPkiAvailable();
  },
};

const _ASYNC_ENROLLMENT_MOCKED_API = {
  async requestJoinOrganization(
    _link: ParsecAsyncEnrollmentAddr,
    _identityStrategy: SubmitAsyncEnrollmentIdentityStrategy,
  ): Promise<Result<null, SubmitAsyncEnrollmentError>> {
    return { ok: true, value: null };
  },

  async listJoinRequests(): Promise<Result<Array<AsyncEnrollmentRequest>, SubmitterListLocalAsyncEnrollmentsError>> {
    const REQUESTS: Array<AsyncEnrollmentRequest> = [
      {
        info: {
          tag: PendingAsyncEnrollmentInfoTag.Accepted,
          submittedOn: DateTime.utc(),
          acceptedOn: DateTime.utc(),
        },
        enrollment: {
          filePath: '/path1',
          submittedOn: DateTime.utc(),
          addr: 'parsec3://localhost:6770/BlackMesa?no_ssl=true&a=pki_enrollment',
          enrollmentId: '1',
          requestedDeviceLabel: 'DeviceLabel',
          requestedHumanHandle: {
            label: 'Gordon Freeman',
            email: 'gordon.freeman@blackmesa.nm',
          },
          identitySystem: {
            tag: AvailablePendingAsyncEnrollmentIdentitySystemTag.PKI,
            certificateRef: {
              uris: [
                {
                  tag: X509URIFlavorValueTag.WindowsCNG,
                  x1: {
                    issuer: new Uint8Array(),
                    serialNumber: new Uint8Array(),
                  },
                },
              ],
              hash: 'abcd2',
            },
          },
        },
        organizationId: 'BlackMesa',
      },
      {
        info: {
          tag: PendingAsyncEnrollmentInfoTag.Cancelled,
          submittedOn: DateTime.utc(),
          cancelledOn: DateTime.utc(),
        },
        enrollment: {
          filePath: '/path2',
          submittedOn: DateTime.utc(),
          addr: 'parsec3://localhost:6770/BlackMesa?no_ssl=true&a=pki_enrollment',
          enrollmentId: '2',
          requestedDeviceLabel: 'DeviceLabel',
          requestedHumanHandle: {
            label: 'Gordon Freeman',
            email: 'gordon.freeman@blackmesa.nm',
          },
          identitySystem: {
            tag: AvailablePendingAsyncEnrollmentIdentitySystemTag.PKI,
            certificateRef: {
              uris: [
                {
                  tag: X509URIFlavorValueTag.WindowsCNG,
                  x1: {
                    issuer: new Uint8Array(),
                    serialNumber: new Uint8Array(),
                  },
                },
              ],
              hash: 'abcd2',
            },
          },
        },
        organizationId: 'BlackMesa',
      },
      {
        info: {
          tag: PendingAsyncEnrollmentInfoTag.Rejected,
          submittedOn: DateTime.utc(),
          rejectedOn: DateTime.utc(),
        },
        enrollment: {
          filePath: '/path1',
          submittedOn: DateTime.utc(),
          addr: 'parsec3://localhost:6770/BlackMesa?no_ssl=true&a=pki_enrollment',
          enrollmentId: '1',
          requestedDeviceLabel: 'DeviceLabel',
          requestedHumanHandle: {
            label: 'Gordon Freeman',
            email: 'gordon.freeman@blackmesa.nm',
          },
          identitySystem: {
            tag: AvailablePendingAsyncEnrollmentIdentitySystemTag.PKI,
            certificateRef: {
              uris: [
                {
                  tag: X509URIFlavorValueTag.WindowsCNG,
                  x1: {
                    issuer: new Uint8Array(),
                    serialNumber: new Uint8Array(),
                  },
                },
              ],
              hash: 'abcd2',
            },
          },
        },
        organizationId: 'BlackMesa',
      },
      {
        info: {
          tag: PendingAsyncEnrollmentInfoTag.Submitted,
          submittedOn: DateTime.utc(),
        },
        enrollment: {
          filePath: '/path1',
          submittedOn: DateTime.utc(),
          addr: 'parsec3://localhost:6770/BlackMesa?no_ssl=true&a=pki_enrollment',
          enrollmentId: '1',
          requestedDeviceLabel: 'DeviceLabel',
          requestedHumanHandle: {
            label: 'Gordon Freeman',
            email: 'gordon.freeman@blackmesa.nm',
          },
          identitySystem: {
            tag: AvailablePendingAsyncEnrollmentIdentitySystemTag.PKI,
            certificateRef: {
              uris: [
                {
                  tag: X509URIFlavorValueTag.WindowsCNG,
                  x1: {
                    issuer: new Uint8Array(),
                    serialNumber: new Uint8Array(),
                  },
                },
              ],
              hash: 'abcd2',
            },
          },
        },
        organizationId: 'BlackMesa',
      },
    ];

    return { ok: true, value: REQUESTS };
  },

  async confirmJoinRequest(
    request: AsyncEnrollmentRequest,
    _saveStrategy: DeviceSaveStrategy,
    _identityStrategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  ): Promise<Result<AvailableDevice, SubmitterFinalizeAsyncEnrollmentError>> {
    return {
      ok: true,
      value: {
        keyFilePath: '/keyFile_Path',
        createdOn: DateTime.utc(),
        protectedOn: DateTime.utc(),
        serverAddr: 'parsec3://localhost:6770?no_ssl=true',
        organizationId: request.organizationId,
        userId: 'userId',
        deviceId: 'deviceId',
        humanHandle: {
          label: request.enrollment.requestedHumanHandle.label,
          email: request.enrollment.requestedHumanHandle.email,
        },
        deviceLabel: request.enrollment.requestedDeviceLabel,
        ty: {
          tag: AvailableDeviceTypeTag.Keyring,
        },
      },
    };
  },

  async deleteJoinRequest(_request: AsyncEnrollmentRequest): Promise<Result<null, SubmitterCancelAsyncEnrollmentError>> {
    return { ok: true, value: null };
  },

  async listAsyncEnrollments(): Promise<Result<Array<AsyncEnrollmentUntrusted>, ClientListAsyncEnrollmentsError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientListAsyncEnrollmentsError>();
    }

    const ENROLLMENTS: Array<AsyncEnrollmentUntrusted> = [
      {
        enrollmentId: '1',
        submittedOn: DateTime.utc(),
        untrustedRequestedDeviceLabel: 'Device Label PKI',
        untrustedRequestedHumanHandle: {
          label: 'Gordon Freeman',
          email: 'gordon.freeman@blackmesa.nm',
        },
        identitySystem: {
          tag: AsyncEnrollmentIdentitySystemTag.PKI,
          x509RootCertificateCommonName: 'Certificate Common Name',
          x509RootCertificateSubject: new Uint8Array(),
        },
      },
      {
        enrollmentId: '2',
        submittedOn: DateTime.utc(),
        untrustedRequestedDeviceLabel: 'Device Label PKI',
        untrustedRequestedHumanHandle: {
          label: 'Gordon Freeman',
          email: 'gordon.freeman@blackmesa.nm',
        },
        identitySystem: {
          tag: AsyncEnrollmentIdentitySystemTag.PKI,
          x509RootCertificateCommonName: 'Certificate Common Name',
          x509RootCertificateSubject: new Uint8Array(),
        },
      },
      {
        enrollmentId: '3',
        submittedOn: DateTime.utc(),
        untrustedRequestedDeviceLabel: 'Device Label SSO',
        untrustedRequestedHumanHandle: {
          label: 'Gordon Freeman',
          email: 'gordon.freeman@blackmesa.nm',
        },
        identitySystem: {
          tag: AsyncEnrollmentIdentitySystemTag.OpenBao,
        },
      },
      {
        enrollmentId: '4',
        submittedOn: DateTime.utc(),
        untrustedRequestedDeviceLabel: 'Device Label SSO',
        untrustedRequestedHumanHandle: {
          label: 'UNKNOWN',
          email: 'UNKNOWN',
        },
        identitySystem: {
          tag: AsyncEnrollmentIdentitySystemTag.PKICorrupted,
          reason: 'Corrupted',
        },
      },
    ];

    return { ok: true, value: ENROLLMENTS };
  },

  async acceptAsyncEnrollment(
    _request: AsyncEnrollmentUntrusted,
    _profile: UserProfile,
    _identityStrategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  ): Promise<Result<null, ClientAcceptAsyncEnrollmentError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientAcceptAsyncEnrollmentError>();
    }
    return { ok: true, value: null };
  },

  async rejectAsyncEnrollment(_request: AsyncEnrollmentUntrusted): Promise<Result<null, ClientRejectAsyncEnrollmentError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientRejectAsyncEnrollmentError>();
    }
    return { ok: true, value: null };
  },

  async selectCertificate(): Promise<Result<X509CertificateReference | undefined, ShowCertificateSelectionDialogError>> {
    return {
      ok: true,
      value: {
        uris: [
          {
            tag: X509URIFlavorValueTag.WindowsCNG,
            x1: {
              issuer: new Uint8Array(),
              serialNumber: new Uint8Array(),
            },
          },
        ],
        // cspell:disable-next-line
        hash: 'ijkl',
      },
    };
  },

  async isSmartcardAvailable(): Promise<boolean> {
    return true;
  },
};

async function getAsyncEnrollmentAddr(): Promise<Result<ParsecAsyncEnrollmentAddr, ClientGetAsyncEnrollmentAddrError>> {
  const handle = getConnectionHandle();

  if (!handle) {
    return generateNoHandleError<ClientGetAsyncEnrollmentAddrError>();
  }
  return libparsec.clientGetAsyncEnrollmentAddr(handle);
}

async function getOpenBaoEmails(client: OpenBaoClient): Promise<Result<Array<string>, OpenBaoListSelfEmailsError>> {
  const connInfo = client.getConnectionInfo();
  const result = await libparsec.openbaoListSelfEmails(
    connInfo.server,
    connInfo.secretMountpoint,
    connInfo.transitMountpoint,
    connInfo.userId,
    connInfo.token,
  );

  if (!result.ok) {
    return result;
  }

  return result;
}

function makeRequestPkiIdentityStrategy(certificate: X509CertificateReference): SubmitAsyncEnrollmentIdentityStrategyPKI {
  return {
    tag: SubmitAsyncEnrollmentIdentityStrategyTag.PKI,
    certificateReference: certificate,
  };
}

function makeAcceptPkiIdentityStrategy(certificate: X509CertificateReference): AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI {
  return {
    tag: AcceptFinalizeAsyncEnrollmentIdentityStrategyTag.PKI,
    certificateReference: certificate,
  };
}

function makeRequestOpenBaoIdentityStrategy(client: OpenBaoClient, humanHandle: HumanHandle): SubmitAsyncEnrollmentIdentityStrategyOpenBao {
  const connInfo = client.getConnectionInfo();

  return {
    tag: SubmitAsyncEnrollmentIdentityStrategyTag.OpenBao,
    openbaoServerUrl: connInfo.server,
    openbaoTransitMountPath: connInfo.transitMountpoint,
    openbaoSecretMountPath: connInfo.secretMountpoint,
    openbaoEntityId: connInfo.userId,
    openbaoAuthToken: connInfo.token,
    openbaoPreferredAuthId: connInfo.provider,
    requestedHumanHandle: humanHandle,
  };
}

function makeAcceptOpenBaoIdentityStrategy(client: OpenBaoClient): AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao {
  const connInfo = client.getConnectionInfo();

  return {
    tag: AcceptFinalizeAsyncEnrollmentIdentityStrategyTag.OpenBao,
    openbaoServerUrl: connInfo.server,
    openbaoTransitMountPath: connInfo.transitMountpoint,
    openbaoSecretMountPath: connInfo.secretMountpoint,
    openbaoEntityId: connInfo.userId,
    openbaoAuthToken: connInfo.token,
  };
}

// Some glue to switch between mocked and libparsec implementation
// depending on a variable (useful for test/dev when not on Windows).
type PkiImpl = typeof _ASYNC_ENROLLMENT_PARSEC_API;

function pkiCurrentImpl(): PkiImpl {
  if ((window as any).TESTING_PKI) {
    return _ASYNC_ENROLLMENT_MOCKED_API;
  }
  return _ASYNC_ENROLLMENT_PARSEC_API;
}

function bind<K extends keyof PkiImpl>(key: K) {
  return (...args: Parameters<PkiImpl[K]>): ReturnType<PkiImpl[K]> => {
    const impl = pkiCurrentImpl()[key];
    return (impl as any)(...args) as ReturnType<PkiImpl[K]>;
  };
}

const requestJoinOrganization = bind('requestJoinOrganization');
const listJoinRequests = bind('listJoinRequests');
const confirmJoinRequest = bind('confirmJoinRequest');
const deleteJoinRequest = bind('deleteJoinRequest');
const listAsyncEnrollments = bind('listAsyncEnrollments');
const acceptAsyncEnrollment = bind('acceptAsyncEnrollment');
const rejectAsyncEnrollment = bind('rejectAsyncEnrollment');
const selectCertificate = bind('selectCertificate');
const isSmartcardAvailable = bind('isSmartcardAvailable');

export {
  acceptAsyncEnrollment,
  confirmJoinRequest,
  deleteJoinRequest,
  getAsyncEnrollmentAddr,
  getOpenBaoEmails,
  isSmartcardAvailable,
  listAsyncEnrollments,
  listJoinRequests,
  makeAcceptOpenBaoIdentityStrategy,
  makeAcceptPkiIdentityStrategy,
  makeRequestOpenBaoIdentityStrategy,
  makeRequestPkiIdentityStrategy,
  rejectAsyncEnrollment,
  requestJoinOrganization,
  selectCertificate,
};

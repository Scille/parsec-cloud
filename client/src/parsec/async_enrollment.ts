// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultDeviceName } from '@/common/device';
import { getClientConfig } from '@/parsec/internals';
import { parseParsecAddr } from '@/parsec/organization';
import {
  AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyTag,
  AsyncEnrollmentIdentitySystem,
  AsyncEnrollmentIdentitySystemTag,
  AsyncEnrollmentRequest,
  AsyncEnrollmentUntrusted,
  AvailableDevice,
  AvailableDeviceTypeTag,
  AvailablePendingAsyncEnrollmentIdentitySystem,
  AvailablePendingAsyncEnrollmentIdentitySystemTag,
  ClientAcceptAsyncEnrollmentError,
  ClientGetAsyncEnrollmentAddrError,
  ClientListAsyncEnrollmentsError,
  ClientRejectAsyncEnrollmentError,
  DeviceSaveStrategy,
  HumanHandle,
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
  SubmitterCancelAsyncEnrollmentErrorTag,
  SubmitterFinalizeAsyncEnrollmentError,
  SubmitterFinalizeAsyncEnrollmentErrorTag,
  SubmitterListLocalAsyncEnrollmentsError,
  UserProfile,
  X509CertificateReference,
  X509URIFlavorValueTag,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import {
  ClientAcceptAsyncEnrollmentErrorTag,
  ClientRejectAsyncEnrollmentErrorTag,
  libparsec,
  ParsecAsyncEnrollmentAddrAndRedirectionURL,
  SubmitAsyncEnrollmentErrorTag,
} from '@/plugins/libparsec';
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

const REQUESTS = new Array<AsyncEnrollmentRequest>();

const _ASYNC_ENROLLMENT_MOCKED_API = {
  async requestJoinOrganization(
    link: ParsecAsyncEnrollmentAddr,
    identityStrategy: SubmitAsyncEnrollmentIdentityStrategy,
  ): Promise<Result<null, SubmitAsyncEnrollmentError>> {
    let identitySystem: AvailablePendingAsyncEnrollmentIdentitySystem;
    let humanHandle: HumanHandle;

    if (identityStrategy.tag === SubmitAsyncEnrollmentIdentityStrategyTag.OpenBao) {
      identitySystem = {
        tag: AvailablePendingAsyncEnrollmentIdentitySystemTag.OpenBao,
        openbaoEntityId: identityStrategy.openbaoEntityId,
        openbaoPreferredAuthId: identityStrategy.openbaoPreferredAuthId,
      };
      humanHandle = identityStrategy.requestedHumanHandle;
    } else {
      identitySystem = {
        tag: AvailablePendingAsyncEnrollmentIdentitySystemTag.PKI,
        certificateRef: identityStrategy.certificateReference,
      };
      humanHandle = {
        label: 'Gordon Freeman',
        email: 'gordon.freeman@blackmesa.nm',
      };
    }
    const id = crypto.randomUUID();
    const addrResult = await parseParsecAddr(link);

    if (!addrResult.ok || addrResult.value.tag !== ParsedParsecAddrTag.AsyncEnrollment) {
      return { ok: false, error: { tag: SubmitAsyncEnrollmentErrorTag.Internal, error: 'invalid link' } };
    }

    REQUESTS.push({
      info: {
        tag: PendingAsyncEnrollmentInfoTag.Submitted,
        submittedOn: DateTime.utc(),
      },
      enrollment: {
        filePath: `/${id}`,
        submittedOn: DateTime.utc(),
        addr: link,
        enrollmentId: id,
        requestedDeviceLabel: 'DeviceLabel',
        requestedHumanHandle: humanHandle,
        identitySystem: identitySystem,
      },
      organizationId: addrResult.value.organizationId,
    });
    return { ok: true, value: null };
  },

  async listJoinRequests(): Promise<Result<Array<AsyncEnrollmentRequest>, SubmitterListLocalAsyncEnrollmentsError>> {
    return { ok: true, value: [...REQUESTS] };
  },

  async confirmJoinRequest(
    request: AsyncEnrollmentRequest,
    _saveStrategy: DeviceSaveStrategy,
    _identityStrategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  ): Promise<Result<AvailableDevice, SubmitterFinalizeAsyncEnrollmentError>> {
    const reqIndex = REQUESTS.findIndex((req) => req.enrollment.enrollmentId === request.enrollment.enrollmentId);

    if (reqIndex === -1) {
      return { ok: false, error: { tag: SubmitterFinalizeAsyncEnrollmentErrorTag.EnrollmentNotFoundOnServer, error: 'not found' } };
    }
    REQUESTS.splice(reqIndex, 1);
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
        totpOpaqueKeyId: null,
        deviceLabel: request.enrollment.requestedDeviceLabel,
        ty: {
          tag: AvailableDeviceTypeTag.Keyring,
        },
      },
    };
  },

  async deleteJoinRequest(request: AsyncEnrollmentRequest): Promise<Result<null, SubmitterCancelAsyncEnrollmentError>> {
    const reqIndex = REQUESTS.findIndex((req) => req.enrollment.enrollmentId === request.enrollment.enrollmentId);

    if (reqIndex === -1) {
      return { ok: false, error: { tag: SubmitterCancelAsyncEnrollmentErrorTag.NotFound, error: 'not found' } };
    }
    REQUESTS.splice(reqIndex, 1);
    return { ok: true, value: null };
  },

  async listAsyncEnrollments(): Promise<Result<Array<AsyncEnrollmentUntrusted>, ClientListAsyncEnrollmentsError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientListAsyncEnrollmentsError>();
    }

    const ENROLLMENTS: Array<AsyncEnrollmentUntrusted> = REQUESTS.filter(
      (req) => req.info.tag === PendingAsyncEnrollmentInfoTag.Submitted,
    ).map((req) => {
      let identitySystem: AsyncEnrollmentIdentitySystem;

      if (req.enrollment.identitySystem.tag === AvailablePendingAsyncEnrollmentIdentitySystemTag.PKI) {
        identitySystem = {
          tag: AsyncEnrollmentIdentitySystemTag.PKI,
          x509RootCertificateCommonName: 'Common Name',
          x509RootCertificateSubject: new Uint8Array([1, 2, 3, 4]),
        };
      } else {
        identitySystem = {
          tag: AsyncEnrollmentIdentitySystemTag.OpenBao,
        };
      }
      return {
        enrollmentId: req.enrollment.enrollmentId,
        submittedOn: req.enrollment.submittedOn,
        untrustedRequestedDeviceLabel: req.enrollment.requestedDeviceLabel,
        untrustedRequestedHumanHandle: req.enrollment.requestedHumanHandle,
        identitySystem: identitySystem,
      };
    });

    return { ok: true, value: ENROLLMENTS };
  },

  async acceptAsyncEnrollment(
    request: AsyncEnrollmentUntrusted,
    _profile: UserProfile,
    _identityStrategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  ): Promise<Result<null, ClientAcceptAsyncEnrollmentError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientAcceptAsyncEnrollmentError>();
    }
    const found = REQUESTS.find((req) => req.enrollment.enrollmentId === request.enrollmentId);
    if (!found) {
      return { ok: false, error: { tag: ClientAcceptAsyncEnrollmentErrorTag.EnrollmentNotFound, error: 'not found' } };
    }
    found.info = {
      tag: PendingAsyncEnrollmentInfoTag.Accepted,
      submittedOn: found.info.submittedOn,
      acceptedOn: DateTime.utc(),
    };

    return { ok: true, value: null };
  },

  async rejectAsyncEnrollment(request: AsyncEnrollmentUntrusted): Promise<Result<null, ClientRejectAsyncEnrollmentError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientRejectAsyncEnrollmentError>();
    }
    const found = REQUESTS.find((req) => req.enrollment.enrollmentId === request.enrollmentId);
    if (!found) {
      return { ok: false, error: { tag: ClientRejectAsyncEnrollmentErrorTag.EnrollmentNotFound, error: 'not found' } };
    }
    found.info = {
      tag: PendingAsyncEnrollmentInfoTag.Rejected,
      submittedOn: found.info.submittedOn,
      rejectedOn: DateTime.utc(),
    };

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

async function getAsyncEnrollmentAddr(): Promise<Result<ParsecAsyncEnrollmentAddrAndRedirectionURL, ClientGetAsyncEnrollmentAddrError>> {
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

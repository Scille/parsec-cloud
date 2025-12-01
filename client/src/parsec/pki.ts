// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultDeviceName } from '@/common/device';
import { getClientConfig } from '@/parsec/internals';
import { SaveStrategy } from '@/parsec/login';
import {
  AvailableDevice,
  AvailableDeviceTypeTag,
  HumanHandle,
  ListPkiLocalPendingError,
  ParsecOrganizationAddr,
  ParsecPkiEnrollmentAddr,
  PkiEnrollmentAcceptError,
  PkiEnrollmentFinalizeError,
  PkiEnrollmentFinalizeErrorTag,
  PkiEnrollmentListError,
  PkiEnrollmentListItem,
  PkiEnrollmentListItemTag,
  PkiEnrollmentListItemValid,
  PkiEnrollmentRejectError,
  PkiEnrollmentSubmitError,
  PkiEnrollmentSubmitPayload,
  PkiGetAddrError,
  PKIInfoItemTag,
  PkiLocalRequest,
  RemoveDeviceError,
  Result,
  ShowCertificateSelectionDialogError,
  ShowCertificateSelectionDialogErrorTag,
  UserProfile,
  X509CertificateReference,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { InvalidityReasonTag, libparsec, X509URIFlavorValueTag } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';
import { DateTime } from 'luxon';

const _PKI_PARSEC_API = {
  async requestJoinOrganization(
    certificate: X509CertificateReference,
    humanHandle: HumanHandle,
    link: ParsecOrganizationAddr,
  ): Promise<Result<null, PkiEnrollmentSubmitError>> {
    const result = await libparsec.pkiEnrollmentSubmit(getClientConfig(), link, certificate, humanHandle, getDefaultDeviceName(), false);
    if (result.ok) {
      return { ok: true, value: null };
    }
    return result;
  },

  async listLocalJoinRequests(): Promise<Result<Array<PkiLocalRequest>, ListPkiLocalPendingError>> {
    const result = await libparsec.listPkiLocalPendingEnrollments(getClientConfig().configDir);

    if (!result.ok) {
      return result;
    }
    const list: Array<PkiLocalRequest> = [];
    for (const enrollment of result.value) {
      enrollment.submittedOn = DateTime.fromSeconds(enrollment.submittedOn as any as number);
      const infoResult = await libparsec.pkiEnrollmentInfo(getClientConfig(), enrollment.addr, enrollment.enrollmentId);
      if (infoResult.ok) {
        infoResult.value.submittedOn = DateTime.fromSeconds(infoResult.value.submittedOn as any as number);
        if (infoResult.value.tag === PKIInfoItemTag.Accepted) {
          infoResult.value.acceptedOn = DateTime.fromSeconds(infoResult.value.acceptedOn as any as number);
        } else if (infoResult.value.tag === PKIInfoItemTag.Cancelled) {
          infoResult.value.cancelledOn = DateTime.fromSeconds(infoResult.value.cancelledOn as any as number);
        } else if (infoResult.value.tag === PKIInfoItemTag.Rejected) {
          infoResult.value.rejectedOn = DateTime.fromSeconds(infoResult.value.rejectedOn as any as number);
        }
        list.push({ info: infoResult.value, enrollment: enrollment });
      }
    }
    return { ok: true, value: list };
  },

  async confirmLocalJoinRequest(request: PkiLocalRequest): Promise<Result<AvailableDevice, PkiEnrollmentFinalizeError>> {
    if (request.info.tag !== PKIInfoItemTag.Accepted) {
      return {
        ok: false,
        error: {
          tag: PkiEnrollmentFinalizeErrorTag.Internal,
          error: `Invalid state: should be ${PKIInfoItemTag.Accepted} but is ${request.info.tag}`,
        },
      };
    }
    (request.enrollment.submittedOn as any as number) = request.enrollment.submittedOn.toSeconds();
    (request.info.submittedOn as any as number) = request.info.submittedOn.toSeconds();
    (request.info.acceptedOn as any as number) = request.info.acceptedOn.toSeconds();
    return await libparsec.pkiEnrollmentFinalize(
      getClientConfig(),
      SaveStrategy.useSmartCard(request.enrollment.certRef),
      request.info.answer,
      request.enrollment,
    );
  },

  async cancelLocalJoinRequest(request: PkiLocalRequest): Promise<Result<null, RemoveDeviceError>> {
    return await libparsec.pkiRemoveLocalPending(getClientConfig(), request.enrollment.enrollmentId);
  },

  async listOrganizationJoinRequests(): Promise<Result<Array<PkiEnrollmentListItem>, PkiEnrollmentListError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<PkiEnrollmentListError>();
    }

    const result = await libparsec.clientPkiListEnrollments(handle);
    if (result.ok) {
      result.value = result.value.map((item) => {
        item.submittedOn = DateTime.fromSeconds(item.submittedOn as any as number);
        return item;
      });
    }
    return result;
  },

  async getPkiJoinOrganizationLink(): Promise<Result<ParsecPkiEnrollmentAddr, PkiGetAddrError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<PkiGetAddrError>();
    }

    return await libparsec.clientPkiGetAddr(handle);
  },

  async acceptOrganizationJoinRequest(
    request: PkiEnrollmentListItemValid,
    profile: UserProfile,
    adminCert: X509CertificateReference,
  ): Promise<Result<null, PkiEnrollmentAcceptError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<PkiEnrollmentAcceptError>();
    }
    return await libparsec.clientPkiEnrollmentAccept(
      handle,
      profile,
      request.enrollmentId,
      request.payload.humanHandle,
      adminCert,
      request.payload,
    );
  },

  async rejectOrganizationJoinRequest(request: PkiEnrollmentListItem): Promise<Result<null, PkiEnrollmentRejectError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<PkiEnrollmentRejectError>();
    }
    return await libparsec.clientPkiEnrollmentReject(handle, request.enrollmentId);
  },

  async selectCertificate(): Promise<Result<X509CertificateReference | undefined, ShowCertificateSelectionDialogError>> {
    if (!(await this.isSmartcardAvailable())) {
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

const _PKI_MOCKED_API = {
  async requestJoinOrganization(
    _certificate: X509CertificateReference,
    _humanHandle: HumanHandle,
    _link: ParsecOrganizationAddr,
  ): Promise<Result<null, PkiEnrollmentSubmitError>> {
    return { ok: true, value: null };
  },

  async listLocalJoinRequests(): Promise<Result<Array<PkiLocalRequest>, ListPkiLocalPendingError>> {
    const PAYLOAD: PkiEnrollmentSubmitPayload = {
      verifyKey: new Uint8Array(),
      publicKey: new Uint8Array(),
      deviceLabel: 'Label',
      humanHandle: {
        label: 'Gordon Freeman',
        email: 'gordon.freeman@blackmesa.nm',
      },
    };

    const REQUESTS: Array<PkiLocalRequest> = [
      {
        info: {
          tag: PKIInfoItemTag.Accepted,
          answer: {
            userId: 'userId1',
            deviceId: 'deviceId1',
            deviceLabel: 'Label1',
            humanHandle: {
              label: 'Gordon Freeman',
              email: 'gordon.freeman@blackmesa.nm',
            },
            profile: UserProfile.Standard,
            rootVerifyKey: new Uint8Array(),
          },
          submittedOn: DateTime.utc(),
          acceptedOn: DateTime.utc(),
        },
        enrollment: {
          certRef: {
            uris: [
              {
                tag: X509URIFlavorValueTag.WindowsCNG,
                x1: new Uint8Array(),
              },
            ],
            hash: 'abcd1',
          },
          addr: 'parsec3://localhost:6770/BlackMesa?no_ssl=true&a=pki_enrollment',
          submittedOn: DateTime.utc(),
          enrollmentId: '1',
          payload: PAYLOAD,
          encryptedKey: new Uint8Array(),
          encryptedKeyAlgo: 'algorithm',
          ciphertext: new Uint8Array(),
        },
      },
      {
        info: {
          tag: PKIInfoItemTag.Cancelled,
          submittedOn: DateTime.utc(),
          cancelledOn: DateTime.utc(),
        },
        enrollment: {
          certRef: {
            uris: [
              {
                tag: X509URIFlavorValueTag.WindowsCNG,
                x1: new Uint8Array(),
              },
            ],
            hash: 'abcd2',
          },
          addr: 'parsec3://localhost:6770/BlackMesa?no_ssl=true&a=pki_enrollment',
          submittedOn: DateTime.utc(),
          enrollmentId: '2',
          payload: PAYLOAD,
          encryptedKey: new Uint8Array(),
          encryptedKeyAlgo: 'algorithm',
          ciphertext: new Uint8Array(),
        },
      },
      {
        info: {
          tag: PKIInfoItemTag.Rejected,
          submittedOn: DateTime.utc(),
          rejectedOn: DateTime.utc(),
        },
        enrollment: {
          certRef: {
            uris: [
              {
                tag: X509URIFlavorValueTag.WindowsCNG,
                x1: new Uint8Array(),
              },
            ],
            hash: 'abcd3',
          },
          addr: 'parsec3://localhost:6770/BlackMesa?no_ssl=true&a=pki_enrollment',
          submittedOn: DateTime.utc(),
          enrollmentId: '3',
          payload: PAYLOAD,
          encryptedKey: new Uint8Array(),
          encryptedKeyAlgo: 'algorithm',
          ciphertext: new Uint8Array(),
        },
      },
      {
        info: {
          tag: PKIInfoItemTag.Submitted,
          submittedOn: DateTime.utc(),
        },
        enrollment: {
          certRef: {
            uris: [
              {
                tag: X509URIFlavorValueTag.WindowsCNG,
                x1: new Uint8Array(),
              },
            ],
            hash: 'abcd4',
          },
          addr: 'parsec3://localhost:6770/BlackMesa?no_ssl=true&a=pki_enrollment',
          submittedOn: DateTime.utc(),
          enrollmentId: '4',
          payload: PAYLOAD,
          encryptedKey: new Uint8Array(),
          encryptedKeyAlgo: 'algorithm',
          ciphertext: new Uint8Array(),
        },
      },
    ];

    return { ok: true, value: REQUESTS };
  },

  async confirmLocalJoinRequest(request: PkiLocalRequest): Promise<Result<AvailableDevice, PkiEnrollmentFinalizeError>> {
    if (request.info.tag !== PKIInfoItemTag.Accepted) {
      return {
        ok: false,
        error: {
          tag: PkiEnrollmentFinalizeErrorTag.Internal,
          error: `Invalid state: should be ${PKIInfoItemTag.Accepted} but is ${request.info.tag}`,
        },
      };
    }
    (request.enrollment.submittedOn as any as number) = request.enrollment.submittedOn.toSeconds();
    (request.info.submittedOn as any as number) = request.info.submittedOn.toSeconds();
    (request.info.acceptedOn as any as number) = request.info.acceptedOn.toSeconds();
    return {
      ok: true,
      value: {
        keyFilePath: '/keyfilepath',
        createdOn: DateTime.utc(),
        protectedOn: DateTime.utc(),
        serverAddr: 'parsec3://localhost:6770?no_ssl=true',
        organizationId: 'MyOrg',
        userId: 'userId',
        deviceId: 'deviceId',
        humanHandle: {
          label: 'Gordon Freeman',
          email: 'gordon.freeman@blackmesa.nm',
        },
        deviceLabel: 'Label',
        ty: {
          tag: AvailableDeviceTypeTag.Smartcard,
        },
      },
    };
  },

  async cancelLocalJoinRequest(_request: PkiLocalRequest): Promise<Result<null, RemoveDeviceError>> {
    return { ok: true, value: null };
  },

  async listOrganizationJoinRequests(): Promise<Result<Array<PkiEnrollmentListItem>, PkiEnrollmentListError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<PkiEnrollmentListError>();
    }

    const ENROLLMENTS: Array<PkiEnrollmentListItem> = [
      {
        tag: PkiEnrollmentListItemTag.Invalid,
        enrollmentId: '2',
        submittedOn: DateTime.utc(),
        reason: { tag: InvalidityReasonTag.Untrusted },
        details: 'Cannot trust the certificate',
      },
      {
        tag: PkiEnrollmentListItemTag.Valid,
        enrollmentId: '1',
        submittedOn: DateTime.utc(),
        payload: {
          verifyKey: new Uint8Array(),
          publicKey: new Uint8Array(),
          deviceLabel: 'Label',
          humanHandle: {
            label: 'Gordon Freeman',
            email: 'gordon.freeman@blackmesa.nm',
          },
        },
      },
      {
        tag: PkiEnrollmentListItemTag.Invalid,
        enrollmentId: '2',
        submittedOn: DateTime.utc(),
        reason: { tag: InvalidityReasonTag.CannotOpenStore },
        details: 'Could not open the certificate store',
      },
      {
        tag: PkiEnrollmentListItemTag.Invalid,
        enrollmentId: '3',
        submittedOn: DateTime.utc(),
        reason: { tag: InvalidityReasonTag.InvalidRootCertificate },
        details: 'Could not open the certificate store',
      },
      {
        tag: PkiEnrollmentListItemTag.Invalid,
        enrollmentId: '4',
        submittedOn: DateTime.utc(),
        reason: { tag: InvalidityReasonTag.DateTimeOutOfRange },
        details: 'Could not open the certificate store',
      },
      {
        tag: PkiEnrollmentListItemTag.Invalid,
        enrollmentId: '5',
        submittedOn: DateTime.utc(),
        reason: { tag: InvalidityReasonTag.CannotGetCertificateInfo },
        details: 'Could not open the certificate store',
      },
    ];

    return { ok: true, value: ENROLLMENTS };
  },

  async getPkiJoinOrganizationLink(): Promise<Result<ParsecPkiEnrollmentAddr, PkiGetAddrError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<PkiGetAddrError>();
    }

    return { ok: true, value: 'parsec3://localhost:6770/MyOrg?no_ssl=true&a=pki_enrollment' };
  },

  async acceptOrganizationJoinRequest(
    _request: PkiEnrollmentListItemValid,
    _profile: UserProfile,
    _adminCert: X509CertificateReference,
  ): Promise<Result<null, PkiEnrollmentAcceptError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<PkiEnrollmentAcceptError>();
    }
    return { ok: true, value: null };
  },

  async rejectOrganizationJoinRequest(_request: PkiEnrollmentListItem): Promise<Result<null, PkiEnrollmentRejectError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<PkiEnrollmentRejectError>();
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
            x1: new Uint8Array(),
          },
        ],
        hash: 'ijkl',
      },
    };
  },

  async isSmartcardAvailable(): Promise<boolean> {
    return true;
  },
};

// Some glue to switch between mocked and libparsec implementation
// depending on a variable (useful for test/dev when not on Windows).
type PkiImpl = typeof _PKI_PARSEC_API;

function pkiCurrentImpl(): PkiImpl {
  if ((window as any).TESTING_PKI) {
    return _PKI_MOCKED_API;
  }
  return _PKI_PARSEC_API;
}

function bind<K extends keyof PkiImpl>(key: K) {
  return (...args: Parameters<PkiImpl[K]>) => (pkiCurrentImpl()[key] as any)(...args);
}

const requestJoinOrganization = bind('requestJoinOrganization');
const listLocalJoinRequests = bind('listLocalJoinRequests');
const confirmLocalJoinRequest = bind('confirmLocalJoinRequest');
const cancelLocalJoinRequest = bind('cancelLocalJoinRequest');
const listOrganizationJoinRequests = bind('listOrganizationJoinRequests');
const getPkiJoinOrganizationLink = bind('getPkiJoinOrganizationLink');
const acceptOrganizationJoinRequest = bind('acceptOrganizationJoinRequest');
const rejectOrganizationJoinRequest = bind('rejectOrganizationJoinRequest');
const selectCertificate = bind('selectCertificate');
const isSmartcardAvailable = bind('isSmartcardAvailable');

export {
  acceptOrganizationJoinRequest,
  cancelLocalJoinRequest,
  confirmLocalJoinRequest,
  getPkiJoinOrganizationLink,
  isSmartcardAvailable,
  listLocalJoinRequests,
  listOrganizationJoinRequests,
  rejectOrganizationJoinRequest,
  requestJoinOrganization,
  selectCertificate,
};

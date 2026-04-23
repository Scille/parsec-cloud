// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultDeviceName } from '@/common/device';
import { toHex } from '@/common/utils';
import { isWeb } from '@/parsec/environment';
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
  CertificatePurpose,
  CertificateWithDetailsValid,
  ClientAcceptAsyncEnrollmentError,
  ClientAcceptAsyncEnrollmentErrorTag,
  ClientGetAsyncEnrollmentAddrError,
  ClientListAsyncEnrollmentsError,
  ClientRejectAsyncEnrollmentError,
  ClientRejectAsyncEnrollmentErrorTag,
  DeviceSaveStrategy,
  EmailSentStatus,
  EmailSentStatusTag,
  HumanHandle,
  OpenBaoListSelfEmailsError,
  ParsecAsyncEnrollmentAddr,
  ParsecAsyncEnrollmentAddrAndRedirectionURL,
  ParsedParsecAddrTag,
  PendingAsyncEnrollmentInfoTag,
  PkiHandle,
  PkiSystemInitError,
  PkiSystemInitErrorTag,
  PkiSystemListUserCertificateError,
  Result,
  ShowCertificateSelectionDialogError,
  SubmitAsyncEnrollmentError,
  SubmitAsyncEnrollmentErrorTag,
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
  AvailablePkiCertificate,
  AvailablePkiCertificateTag,
  AvailablePkiCertificateValid,
  libparsec,
  PkiOpenUserCertificatePrivateKeyError,
  PkiPrivateKeyCloseError,
  UserX509CertificateLoadErrorTag,
} from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';
import { OpenBaoClient } from '@/services/openBao';
import { DateTime } from 'luxon';
import { toRaw } from 'vue';

const _ASYNC_ENROLLMENT_PARSEC_API = {
  async initPki(): Promise<Result<null, PkiSystemInitError>> {
    if (isWeb()) {
      // Retrieve information from the meta tags defined in index.html

      const WEB_PARSEC_SERVER = 'scws-web_parsec_server';
      const SCWS_LOCATION_NAME = 'scws-scwsapi_js-location';
      const SCWS_APP_CERTIFICATE = 'scws-web_application_certificate';

      const scwsapiLocationTag = document.querySelector(`meta[name="${SCWS_LOCATION_NAME}"`) as HTMLMetaElement | null;
      if (!scwsapiLocationTag?.content) {
        console.log(`No meta tag '${SCWS_LOCATION_NAME}' present`);
        return { ok: false, error: { tag: PkiSystemInitErrorTag.NotAvailable, error: `Not meta tag '${SCWS_LOCATION_NAME}' present` } };
      }
      const scwsapiAppCertificateTag = document.querySelector(`meta[name="${SCWS_APP_CERTIFICATE}"`) as HTMLMetaElement | null;
      if (!scwsapiAppCertificateTag?.content) {
        console.log(`No meta tag '${SCWS_APP_CERTIFICATE}' present`);
        return { ok: false, error: { tag: PkiSystemInitErrorTag.NotAvailable, error: `Not meta tag '${SCWS_APP_CERTIFICATE}' present` } };
      }
      const parsecServerTag = document.querySelector(`meta[name="${WEB_PARSEC_SERVER}"`) as HTMLMetaElement | null;
      if (!parsecServerTag?.content) {
        console.log(`No meta tag '${WEB_PARSEC_SERVER}' present`);
        return { ok: false, error: { tag: PkiSystemInitErrorTag.NotAvailable, error: `Not meta tag '${WEB_PARSEC_SERVER}' present` } };
      }

      try {
        // Import scwsapi dynamically and make it available in the global scope
        const scwsapi = await import(scwsapiLocationTag.content);
        (globalThis as any).SCWS = scwsapi.SCWS;
        (globalThis as any).WEB_APPLICATION_CERTIFICATE = scwsapiAppCertificateTag.content;
      } catch (err: any) {
        console.error(`Failed to import scwsapi: ${err.toString()}`);
        return { ok: false, error: { tag: PkiSystemInitErrorTag.NotAvailable, error: `Failed to import scwsapi (${err.toString()})` } };
      }
      return await libparsec.pkiInitForScws(window.getConfigDir(), parsecServerTag.content);
    } else {
      return await libparsec.pkiInitForNative(window.getConfigDir());
    }
  },

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
  ): Promise<Result<EmailSentStatus, ClientAcceptAsyncEnrollmentError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientAcceptAsyncEnrollmentError>();
    }
    return await libparsec.clientAcceptAsyncEnrollment(handle, profile, request.enrollmentId, identityStrategy, true);
  },

  async rejectAsyncEnrollment(request: AsyncEnrollmentUntrusted): Promise<Result<null, ClientRejectAsyncEnrollmentError>> {
    const handle = getConnectionHandle();

    if (!handle) {
      return generateNoHandleError<ClientRejectAsyncEnrollmentError>();
    }
    return await libparsec.clientRejectAsyncEnrollment(handle, request.enrollmentId);
  },

  async selectCertificate(): Promise<Result<X509CertificateReference | undefined, ShowCertificateSelectionDialogError>> {
    const result = await libparsec.showCertificateSelectionDialogWindowsOnly();
    if (result.ok && result.value === null) {
      return { ok: true, value: undefined };
    }
    return result as Result<X509CertificateReference | undefined, ShowCertificateSelectionDialogError>;
  },

  async listCertificates(
    purpose: CertificatePurpose,
  ): Promise<Result<Array<CertificateWithDetailsValid>, PkiSystemListUserCertificateError>> {
    const result = await libparsec.pkiListUserCertificates();

    if (result.ok) {
      const filtered: Array<AvailablePkiCertificateValid> = result.value.filter((cert) => {
        if (cert.tag !== AvailablePkiCertificateTag.Valid) {
          window.electronAPI.log('warn', `Invalid certificate: ${cert.invalidReason.tag}`);
          return false;
        }
        switch (purpose) {
          case CertificatePurpose.Both:
            return cert.details.canEncrypt && cert.details.canSign;
          case CertificatePurpose.Encrypt:
            return cert.details.canEncrypt;
          case CertificatePurpose.Sign:
            return cert.details.canSign;
        }
      }) as Array<AvailablePkiCertificateValid>;

      result.value = filtered.map((cert) => {
        cert.details.notAfter = DateTime.fromSeconds(cert.details.notAfter as any as number);
        cert.details.notBefore = DateTime.fromSeconds(cert.details.notBefore as any as number);
        (cert as CertificateWithDetailsValid).isExpired = () => cert.details.notAfter < DateTime.utc();
        (cert as CertificateWithDetailsValid).getName = () => cert.friendlyName ?? cert.details.commonName ?? '';
        (cert as CertificateWithDetailsValid).getSerial = () => toHex(cert.details.serial);
        return cert;
      });
      return result as { ok: true; value: Array<CertificateWithDetailsValid> };
    }
    return result;
  },

  async openCertificate(certRef: X509CertificateReference): Promise<Result<PkiHandle, PkiOpenUserCertificatePrivateKeyError>> {
    return libparsec.pkiOpenUserCertificatePrivateKey(toRaw(certRef));
  },

  async closeCertificate(handle: PkiHandle): Promise<Result<null, PkiPrivateKeyCloseError>> {
    return libparsec.pkiPrivateKeyClose(handle);
  },
};

const REQUESTS = new Array<AsyncEnrollmentRequest>();

function generateFakeCertificateReference(): X509CertificateReference {
  return {
    uris: [
      {
        tag: X509URIFlavorValueTag.WindowsCNG,
        x1: {
          issuer: new Uint8Array(),
          serialNumber: new Uint8Array(),
        },
      },
    ],
    hash: crypto.randomUUID(),
  };
}

const FAKE_CERTS: Array<AvailablePkiCertificate> = [
  {
    tag: AvailablePkiCertificateTag.Valid,
    reference: generateFakeCertificateReference(),
    friendlyName: 'Gordon Freeman',
    details: {
      commonName: 'Gordon Freeman',
      subject: [],
      issuer: [],
      notBefore: DateTime.fromISO('1998-11-19T08:00:00'),
      notAfter: DateTime.fromISO('2056-12-31T10:30:00'),
      serial: new Uint8Array([45, 23, 42, 12, 34, 23, 42, 5, 97, 98]),
      emails: ['gordon.freeman@blackmesa.nm', 'gordon.freeman@wanadoo.fr'],
      canSign: true,
      canEncrypt: true,
    },
  },
  {
    tag: AvailablePkiCertificateTag.Valid,
    reference: generateFakeCertificateReference(),
    friendlyName: 'Eli Vance',
    details: {
      commonName: 'Eli Vance',
      subject: [],
      issuer: [],
      notBefore: DateTime.fromISO('1998-11-19T08:00:00'),
      notAfter: DateTime.fromISO('2007-10-10T10:30:00'),
      serial: new Uint8Array([86, 45, 42, 7, 65, 64, 23, 23, 5, 97, 98]),
      emails: ['eli.vance@blackmesa.nm'],
      canSign: true,
      canEncrypt: true,
    },
  },
  {
    tag: AvailablePkiCertificateTag.Valid,
    reference: generateFakeCertificateReference(),
    friendlyName: 'Eli Vance',
    details: {
      commonName: 'Eli Vance',
      subject: [],
      issuer: [],
      notBefore: DateTime.fromISO('1998-11-19T08:00:00'),
      notAfter: DateTime.fromISO('2007-10-10T10:30:00'),
      serial: new Uint8Array([86, 45, 42, 78, 65, 64, 23, 23, 5, 97, 98]),
      emails: ['eli.vance@blackmesa.nm'],
      canSign: true,
      canEncrypt: true,
    },
  },
  {
    tag: AvailablePkiCertificateTag.Valid,
    reference: generateFakeCertificateReference(),
    friendlyName: 'Eli Vance',
    details: {
      commonName: 'Eli Vance',
      subject: [],
      issuer: [],
      notBefore: DateTime.fromISO('1998-11-19T08:00:00'),
      notAfter: DateTime.fromISO('2027-10-10T10:30:00'),
      serial: new Uint8Array([86, 45, 78, 7, 65, 64, 23, 23, 5, 97, 98]),
      emails: ['eli.vance@blackmesa.nm'],
      canSign: true,
      canEncrypt: true,
    },
  },
  {
    tag: AvailablePkiCertificateTag.Valid,
    reference: generateFakeCertificateReference(),
    friendlyName: 'Eli Vance',
    details: {
      commonName: 'Eli Vance',
      subject: [],
      issuer: [],
      notBefore: DateTime.fromISO('1998-11-19T08:00:00'),
      notAfter: DateTime.fromISO('2027-10-10T10:30:00'),
      serial: new Uint8Array([86, 23, 42, 7, 65, 64, 23, 23, 5, 97, 98]),
      emails: ['eli.vance@blackmesa.nm'],
      canSign: true,
      canEncrypt: true,
    },
  },
  {
    tag: AvailablePkiCertificateTag.Valid,
    reference: generateFakeCertificateReference(),
    friendlyName: 'Eli Vance',
    details: {
      commonName: 'Eli Vance',
      subject: [],
      issuer: [],
      notBefore: DateTime.fromISO('1998-11-19T08:00:00'),
      notAfter: DateTime.fromISO('2007-10-10T10:30:00'),
      serial: new Uint8Array([86, 45, 42, 7, 65, 64, 23, 23, 57, 97, 98]),
      emails: ['eli.vance@blackmesa.nm'],
      canSign: true,
      canEncrypt: true,
    },
  },
  {
    tag: AvailablePkiCertificateTag.Valid,
    reference: generateFakeCertificateReference(),
    friendlyName: 'Eli Vance',
    details: {
      commonName: 'Eli Vance',
      subject: [],
      issuer: [],
      notBefore: DateTime.fromISO('1998-11-19T08:00:00'),
      notAfter: DateTime.fromISO('2027-10-10T10:30:00'),
      serial: new Uint8Array([86, 45, 42, 7, 65, 64, 23, 23, 5, 73, 98]),
      emails: ['eli.vance@blackmesa.nm'],
      canSign: true,
      canEncrypt: true,
    },
  },
  {
    tag: AvailablePkiCertificateTag.Valid,
    reference: generateFakeCertificateReference(),
    friendlyName: 'Eli Vance',
    details: {
      commonName: 'Eli Vance',
      subject: [],
      issuer: [],
      notBefore: DateTime.fromISO('1998-11-19T08:00:00'),
      notAfter: DateTime.fromISO('2007-10-10T10:30:00'),
      serial: new Uint8Array([86, 45, 42, 7, 65, 64, 79, 23, 5, 97, 98]),
      emails: ['eli.vance@blackmesa.nm'],
      canSign: true,
      canEncrypt: true,
    },
  },
  {
    tag: AvailablePkiCertificateTag.Valid,
    reference: generateFakeCertificateReference(),
    friendlyName: 'Eli Vance',
    details: {
      commonName: 'Eli Vance',
      subject: [],
      issuer: [],
      notBefore: DateTime.fromISO('1998-11-19T08:00:00'),
      notAfter: DateTime.fromISO('2027-10-10T10:30:00'),
      serial: new Uint8Array([86, 45, 42, 21, 65, 64, 23, 23, 5, 97, 98]),
      emails: ['eli.vance@blackmesa.nm'],
      canSign: true,
      canEncrypt: true,
    },
  },
  {
    tag: AvailablePkiCertificateTag.Valid,
    reference: generateFakeCertificateReference(),
    // cspell:disable-next-line
    friendlyName: 'Isaac Kleiner',
    details: {
      // cspell:disable-next-line
      commonName: 'Isaac Kleiner',
      subject: [],
      issuer: [],
      notBefore: DateTime.fromISO('1998-11-19T08:00:00'),
      notAfter: DateTime.fromISO('2032-08-08T11:45:00'),
      serial: new Uint8Array([76, 34, 21, 4, 65, 23, 68, 8, 34, 98]),
      // cspell:disable-next-line
      emails: ['isaac.kleiner@blackmesa.nm'],
      canSign: true,
      canEncrypt: false,
    },
  },
  {
    tag: AvailablePkiCertificateTag.Invalid,
    reference: generateFakeCertificateReference(),
    invalidReason: { tag: UserX509CertificateLoadErrorTag.InvalidEmail },
  },
];

const _ASYNC_ENROLLMENT_MOCKED_API = {
  async initPki(): Promise<Result<null, PkiSystemInitError>> {
    return { ok: true, value: null };
  },

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
        certificateRef: (FAKE_CERTS[identityStrategy.pkiPrivateKeyHandle] as AvailablePkiCertificateValid).reference,
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
  ): Promise<Result<EmailSentStatus, ClientAcceptAsyncEnrollmentError>> {
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

    return { ok: true, value: { tag: EmailSentStatusTag.Success } };
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

  async listCertificates(
    purpose: CertificatePurpose,
  ): Promise<Result<Array<CertificateWithDetailsValid>, PkiSystemListUserCertificateError>> {
    const filtered: Array<CertificateWithDetailsValid> = FAKE_CERTS.filter((cert) => {
      if (cert.tag !== AvailablePkiCertificateTag.Valid) {
        return false;
      }
      switch (purpose) {
        case CertificatePurpose.Both:
          return cert.details.canEncrypt && cert.details.canSign;
        case CertificatePurpose.Encrypt:
          return cert.details.canEncrypt;
        case CertificatePurpose.Sign:
          return cert.details.canSign;
      }
    }) as Array<CertificateWithDetailsValid>;

    return {
      ok: true,
      value: filtered.map((cert) => {
        (cert as CertificateWithDetailsValid).isExpired = () => cert.details.notAfter < DateTime.utc();
        (cert as CertificateWithDetailsValid).getName = () => cert.friendlyName ?? cert.details.commonName ?? '';
        (cert as CertificateWithDetailsValid).getSerial = () => toHex(cert.details.serial);
        return cert;
      }),
    };
  },

  async openCertificate(certRef: X509CertificateReference): Promise<Result<PkiHandle, PkiOpenUserCertificatePrivateKeyError>> {
    return { ok: true, value: FAKE_CERTS.findIndex((cert) => certRef.hash === cert.reference.hash) };
  },

  async closeCertificate(_handle: PkiHandle): Promise<Result<null, PkiPrivateKeyCloseError>> {
    return { ok: true, value: null };
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

function makeRequestPkiIdentityStrategy(handle: PkiHandle): SubmitAsyncEnrollmentIdentityStrategyPKI {
  return {
    tag: SubmitAsyncEnrollmentIdentityStrategyTag.PKI,
    pkiPrivateKeyHandle: handle,
  };
}

function makeAcceptPkiIdentityStrategy(handle: PkiHandle): AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI {
  return {
    tag: AcceptFinalizeAsyncEnrollmentIdentityStrategyTag.PKI,
    pkiPrivateKeyHandle: handle,
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

async function isSmartcardAvailable(): Promise<boolean> {
  const result = await initPki();
  return result.ok;
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
const initPki = bind('initPki');
const listCertificates = bind('listCertificates');
const openCertificate = bind('openCertificate');
const closeCertificate = bind('closeCertificate');

export {
  acceptAsyncEnrollment,
  closeCertificate,
  confirmJoinRequest,
  deleteJoinRequest,
  getAsyncEnrollmentAddr,
  getOpenBaoEmails,
  initPki,
  isSmartcardAvailable,
  listAsyncEnrollments,
  listCertificates,
  listJoinRequests,
  makeAcceptOpenBaoIdentityStrategy,
  makeAcceptPkiIdentityStrategy,
  makeRequestOpenBaoIdentityStrategy,
  makeRequestPkiIdentityStrategy,
  openCertificate,
  rejectAsyncEnrollment,
  requestJoinOrganization,
  selectCertificate,
};

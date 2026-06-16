// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultDeviceName } from '@/common/device';
import { toHex } from '@/common/utils';
import { isWeb } from '@/parsec/environment';
import { getClientConfig } from '@/parsec/internals';
import {
  AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyTag,
  AsyncEnrollmentRequest,
  AsyncEnrollmentUntrusted,
  AvailableDevice,
  CertificatePart,
  CertificatePurpose,
  CertificateWithDetailsValid,
  ClientAcceptAsyncEnrollmentError,
  ClientGetAsyncEnrollmentAddrError,
  ClientListAsyncEnrollmentsError,
  ClientRejectAsyncEnrollmentError,
  DeviceSaveStrategy,
  EmailSentStatus,
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
import {
  AvailablePkiCertificateTag,
  AvailablePkiCertificateValid,
  libparsec,
  PkiOpenUserCertificatePrivateKeyError,
  PkiPrivateKeyCloseError,
} from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';
import { OpenBaoClient } from '@/services/openBao';
import { DateTime } from 'luxon';
import { toRaw } from 'vue';

function certSubjectKey(cert: AvailablePkiCertificateValid): string {
  return JSON.stringify(cert.details.subject.map((dn) => `${dn.tag}:${dn.x1}`).sort());
}

function toCertPart(cert: AvailablePkiCertificateValid): CertificatePart {
  return {
    reference: cert.reference,
    friendlyName: cert.friendlyName,
    details: cert.details,
  };
}

function pickBestByExpiry(existing: CertificatePart | undefined, candidate: AvailablePkiCertificateValid): CertificatePart {
  if (!existing || candidate.details.notAfter > existing.details.notAfter) {
    return toCertPart(candidate);
  }
  return existing;
}

function makeCertificateWithDetails(signCert?: CertificatePart, encryptCert?: CertificatePart): CertificateWithDetailsValid {
  const primary = signCert ?? encryptCert;
  return {
    signCert,
    encryptCert,
    isExpired: () => {
      const signExpired = signCert ? signCert.details.notAfter < DateTime.utc() : false;
      const encryptExpired = encryptCert ? encryptCert.details.notAfter < DateTime.utc() : false;
      return signExpired || encryptExpired;
    },
    getName: () => primary?.friendlyName ?? primary?.details.commonName ?? '',
    getSerial: () => {
      const parts: string[] = [];
      if (signCert) {
        parts.push(toHex(signCert.details.serial));
      }
      if (encryptCert && encryptCert !== signCert) {
        parts.push(toHex(encryptCert.details.serial));
      }
      return parts.join('-');
    },
  };
}

function bundleCertificates(certs: Array<AvailablePkiCertificateValid>, purpose: CertificatePurpose): Array<CertificateWithDetailsValid> {
  const groups = new Map<string, { signCert?: CertificatePart; encryptCert?: CertificatePart }>();

  for (const cert of certs) {
    const key = certSubjectKey(cert);
    const group = groups.get(key) ?? {};

    switch (purpose) {
      case CertificatePurpose.Both:
        if (cert.details.canSign) {
          group.signCert = pickBestByExpiry(group.signCert, cert);
        }
        if (cert.details.canEncrypt) {
          group.encryptCert = pickBestByExpiry(group.encryptCert, cert);
        }
        break;
      case CertificatePurpose.Sign:
        if (cert.details.canSign) {
          group.signCert = pickBestByExpiry(group.signCert, cert);
        }
        break;
      case CertificatePurpose.Encrypt:
        if (cert.details.canEncrypt) {
          group.encryptCert = pickBestByExpiry(group.encryptCert, cert);
        }
        break;
    }

    if (group.signCert || group.encryptCert) {
      groups.set(key, group);
    }
  }

  const result: Array<CertificateWithDetailsValid> = [];
  for (const group of groups.values()) {
    if (purpose === CertificatePurpose.Both && (!group.signCert || !group.encryptCert)) {
      continue;
    }
    result.push(makeCertificateWithDetails(group.signCert, group.encryptCert));
  }
  return result;
}

let pkiInitialized = false;

const _ASYNC_ENROLLMENT_PARSEC_API = {
  async initPki(): Promise<Result<null, PkiSystemInitError>> {
    if (isWeb()) {
      // Retrieve information from the meta tags defined in index.html and use it to
      // init the PKI with SCWS.
      // We're juggling with three different contexts here:
      // - The GUI has access to the `document` object necessary to retrieve the meta tags
      // - The web worker (assuming the browser supports shared worker) runs in a different context
      //   and needs to set up a global state
      // - libparsec runs in the context of the worker but cannot import javascript libraries dynamically
      // So we do some loading part here, we forward the elements to a proxy inside the worker that wraps "pkiInitForScws"
      // to import the module and set a global state, and finally the real `libparsec.pkiInitForScws` is called.

      if ((window as any).TESTING_MOCKED_SCWS) {
        window.electronAPI.log('info', 'PKI is mocked');
        // To test the PKI we pretend SCWS is enabled, but
        // we are going to rely on the testbed.
        const result = await (libparsec.pkiInitForScws as any)(
          window.getConfigDir(),
          import.meta.env.PARSEC_APP_TESTBED_SERVER || window.TESTBED_SERVER_URL,
          '',
          '',
          true, // skipScwsApiImport since `scwsapi.js` won't be used (and is not available anyway)
        );
        pkiInitialized = result.ok;
        return result;
      }

      const parsedRes = await libparsec.tryConvertHttpToParsecAddr(window.origin);
      if (!parsedRes.ok) {
        console.warn(`PKI: Failed to parse ${window.origin} into a parsec addr`);
        return {
          ok: false,
          error: {
            tag: PkiSystemInitErrorTag.NotAvailable,
            error: `Cannot parse origin: ${parsedRes.error.tag}: ${parsedRes.error.error}`,
          },
        };
      }

      const SCWS_LOCATION_NAME = 'scws-scwsapi_js-location';
      const SCWS_APP_CERTIFICATE = 'scws-web_application_certificate';

      const scwsapiLocation = (document.querySelector(`meta[name="${SCWS_LOCATION_NAME}"`) as HTMLMetaElement | null)?.content;
      if (!scwsapiLocation) {
        console.log(`PKI: No meta tag '${SCWS_LOCATION_NAME}' present`);
        return { ok: false, error: { tag: PkiSystemInitErrorTag.NotAvailable, error: `No meta tag '${SCWS_LOCATION_NAME}' present` } };
      }
      const scwsapiAppCertificate = (document.querySelector(`meta[name="${SCWS_APP_CERTIFICATE}"`) as HTMLMetaElement | null)?.content;
      if (!scwsapiAppCertificate) {
        console.log(`PKI: No meta tag '${SCWS_APP_CERTIFICATE}' present`);
        return { ok: false, error: { tag: PkiSystemInitErrorTag.NotAvailable, error: `No meta tag '${SCWS_APP_CERTIFICATE}' present` } };
      }

      // Signature differs between what the proxy expects and what libparsec has defined,
      // so we just cast. Trust me bro.
      const result = await (libparsec.pkiInitForScws as any)(
        window.getConfigDir(),
        parsedRes.value,
        scwsapiLocation,
        scwsapiAppCertificate,
      );
      pkiInitialized = result.ok;
      return result;
    } else {
      const result = await libparsec.pkiInitForNative(window.getConfigDir());
      pkiInitialized = result.ok;
      return result;
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
      const validCerts: Array<AvailablePkiCertificateValid> = result.value.filter((cert) => {
        if (cert.tag !== AvailablePkiCertificateTag.Valid) {
          window.electronAPI.log('warn', `Invalid certificate: ${cert.invalidReason.tag}`);
          return false;
        }
        return true;
      }) as Array<AvailablePkiCertificateValid>;

      for (const cert of validCerts) {
        cert.details.notAfter = DateTime.fromSeconds(cert.details.notAfter as any as number);
        cert.details.notBefore = DateTime.fromSeconds(cert.details.notBefore as any as number);
      }

      return { ok: true, value: bundleCertificates(validCerts, purpose) };
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

function makeRequestPkiIdentityStrategy(signHandle: PkiHandle, encryptHandle: PkiHandle): SubmitAsyncEnrollmentIdentityStrategyPKI {
  return {
    tag: SubmitAsyncEnrollmentIdentityStrategyTag.PKI,
    pkiSignPrivateKeyHandle: signHandle,
    pkiEncryptPrivateKeyHandle: encryptHandle,
  };
}

function makeAcceptPkiIdentityStrategy(signHandle: PkiHandle, encryptHandle: PkiHandle): AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI {
  return {
    tag: AcceptFinalizeAsyncEnrollmentIdentityStrategyTag.PKI,
    pkiSignPrivateKeyHandle: signHandle,
    pkiEncryptPrivateKeyHandle: encryptHandle,
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
  return pkiInitialized;
}

const requestJoinOrganization = _ASYNC_ENROLLMENT_PARSEC_API.requestJoinOrganization;
const listJoinRequests = _ASYNC_ENROLLMENT_PARSEC_API.listJoinRequests;
const confirmJoinRequest = _ASYNC_ENROLLMENT_PARSEC_API.confirmJoinRequest;
const deleteJoinRequest = _ASYNC_ENROLLMENT_PARSEC_API.deleteJoinRequest;
const listAsyncEnrollments = _ASYNC_ENROLLMENT_PARSEC_API.listAsyncEnrollments;
const acceptAsyncEnrollment = _ASYNC_ENROLLMENT_PARSEC_API.acceptAsyncEnrollment;
const rejectAsyncEnrollment = _ASYNC_ENROLLMENT_PARSEC_API.rejectAsyncEnrollment;
const selectCertificate = _ASYNC_ENROLLMENT_PARSEC_API.selectCertificate;
const initPki = _ASYNC_ENROLLMENT_PARSEC_API.initPki;
const listCertificates = _ASYNC_ENROLLMENT_PARSEC_API.listCertificates;
const openCertificate = _ASYNC_ENROLLMENT_PARSEC_API.openCertificate;
const closeCertificate = _ASYNC_ENROLLMENT_PARSEC_API.closeCertificate;

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

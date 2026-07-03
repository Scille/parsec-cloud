// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { X509Certificate } from 'crypto';
import { Certificate, session } from 'electron';
import { readFile } from 'node:fs/promises';

export async function setupCustomCaFile(path: string | undefined) {
  if (path === undefined) {
    return; // nothing to do
  }

  let fileData: string;
  try {
    fileData = await readFile(path, 'utf8');
  } catch (e) {
    console.warn(`Failed to load custom ca file at '${path}', ignoring (${e})`);
    return;
  }

  const caCert = new X509Certificate(fileData);
  console.log(`Loaded custom CA with fingerprint ${caCert.fingerprint256}`);

  function isTrustedChain(certificate: Certificate): boolean {
    let cert = certificate;
    while (cert) {
      try {
        const x509 = new X509Certificate(cert.data);

        if (x509.checkIssued(caCert)) {
          return x509.verify(caCert.publicKey);
        }
      } catch (e) {}
      cert = cert.issuerCert; // Try next certificate
    }
    return false; // no more cert to try in the chain: not trusted
  }

  // https://www.electronjs.org/docs/latest/api/session#sessetcertificateverifyprocproc
  session.defaultSession.setCertificateVerifyProc((request, callback) => {
    const cert = request.certificate;

    if (isTrustedChain(cert)) {
      callback(0); // Indicate success in certificate trust
    } else {
      callback(-3); // fallback to chromium verification result
    }
  });
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { session } from 'electron';

enum CspDirective {
  DefaultSrc = 'default-src',
  ScriptSrc = 'script-src',
  ConnectSrc = 'connect-src',
  ImgSrc = 'img-src',
  StyleSrc = 'style-src',
  FontSrc = 'font-src',
  FrameSrc = 'frame-src',
  WorkerSrc = 'worker-src',
  MediaSrc = 'media-src',
}

function buildContentSecurityPolicy(customProtocol: string, allowWasmEvaluation: boolean, allowJavaScriptEvaluation: boolean): string {
  const scriptSources = [customProtocol, "'unsafe-inline'"];
  if (allowWasmEvaluation) {
    scriptSources.push("'wasm-unsafe-eval'");
  }
  if (allowJavaScriptEvaluation) {
    scriptSources.push("'unsafe-eval'");
  }
  scriptSources.push('https://*.stripe.com');

  const cspRules: Array<[CspDirective, Array<string>]> = [
    [CspDirective.DefaultSrc, [customProtocol, 'devtools:']],
    [CspDirective.ScriptSrc, scriptSources],
    [CspDirective.ImgSrc, [customProtocol, 'blob:', 'data:', 'https:', 'http:']],
    [CspDirective.StyleSrc, [customProtocol, "'unsafe-inline'", 'data:', 'https:', 'http:']],
    [CspDirective.FontSrc, [customProtocol, 'data:', 'https:*', 'http:*']],
    [CspDirective.ConnectSrc, [customProtocol, 'https:', 'http:', 'wss:', 'ws:']],
    [CspDirective.WorkerSrc, [customProtocol, 'blob:', 'https:', 'http:']],
    [CspDirective.FrameSrc, [customProtocol, 'https:', 'http:']],
    [CspDirective.MediaSrc, [customProtocol, 'blob:', 'data:', 'https:', 'http:']],
  ];

  return cspRules.map(([directive, sources]) => `${directive} ${sources.join(' ')}`).join('; ');
}

// Set a CSP up for our application based on the custom scheme.
export function setupContentSecurityPolicy(customScheme: string): void {
  const customProtocol = `${customScheme}:`;
  const parsecCsp = buildContentSecurityPolicy(customProtocol, false, false);
  // Editics iframe contains loads onlyoffice-x2t, which itself compiles its
  // packaged WASM module, but does not need JavaScript eval.
  const editicsHostCsp = buildContentSecurityPolicy(customProtocol, true, false);
  // OnlyOffice itself also uses `new Function()` for its templates and
  // localization, hence this deliberately narrower document-only exception.
  const onlyOfficeEditorCsp = buildContentSecurityPolicy(customProtocol, true, true);

  session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
    const responseHeaders = { ...details.responseHeaders };

    if (details.resourceType === 'mainFrame') {
      // Keep WebAssembly compilation disabled for the Parsec application.
      responseHeaders['Content-Security-Policy'] = [parsecCsp];
    } else if (details.resourceType === 'subFrame') {
      const parsedUrl = new URL(details.url);
      if (parsedUrl.protocol === customProtocol) {
        if (parsedUrl.pathname.startsWith('/editics/')) {
          responseHeaders['Content-Security-Policy'] = [editicsHostCsp];
        }
        if (parsedUrl.pathname.startsWith('/onlyoffice/')) {
          responseHeaders['Content-Security-Policy'] = [onlyOfficeEditorCsp];
        }
      }
    }

    callback({ responseHeaders });
  });
}

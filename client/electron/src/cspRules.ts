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
}

// Set a CSP up for our application based on the custom scheme
export function setupContentSecurityPolicy(customScheme: string): void {
  const customProtocol = `${customScheme}:`;

  const CspRules: Array<[CspDirective, Array<string>]> = [
    [CspDirective.DefaultSrc, [customProtocol, 'devtools:']],
    [CspDirective.ScriptSrc, [customProtocol, "'unsafe-inline'", 'https://*.stripe.com']],
    [CspDirective.ImgSrc, [customProtocol, 'data:', 'https:', 'http:']],
    [CspDirective.StyleSrc, [customProtocol, "'unsafe-inline'", 'data:', 'https:', 'http:']],
    [CspDirective.FontSrc, [customProtocol, 'data:', 'https:*', 'http:*']],
    [CspDirective.ConnectSrc, [customProtocol, 'https:', 'http:', 'wss:', 'ws:']],
    [CspDirective.WorkerSrc, [customProtocol, 'https:', 'http:']],
    [CspDirective.FrameSrc, [customProtocol, 'https:', 'http:']],
  ];

  const rules: Array<string> = [];
  for (const [directive, hosts] of CspRules) {
    rules.push(`${directive} ${hosts.join(' ')}`);
  }
  const CSP_RULE = rules.join('; ');

  session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
    if (details.resourceType !== 'mainFrame') {
      return callback({ responseHeaders: details.responseHeaders });
    }

    const responseHeaders = { ...details.responseHeaders };
    responseHeaders['Content-Security-Policy'] = [CSP_RULE];
    callback({ responseHeaders });
  });
}

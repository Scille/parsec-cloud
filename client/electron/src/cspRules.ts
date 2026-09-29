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
  ObjectSrc = 'object-src',
  BaseUri = 'base-uri',
  FormAction = 'form-action',
  FrameAncestors = 'frame-ancestors',
}

function buildPolicy(rules: Array<[CspDirective, Array<string>]>): string {
  return rules.map(([directive, sources]) => `${directive} ${sources.join(' ')}`).join('; ');
}

// Scheme serving editics (i.e. OnlyOffice) in its own origin, see `setup.ts`.
const EDITICS_PROTOCOL = 'parsec-editics:';

export function setupContentSecurityPolicy(customScheme: string): void {
  const customProtocol = `${customScheme}:`;

  const CSP_RULE = buildPolicy([
    [CspDirective.DefaultSrc, [customProtocol, 'devtools:']],
    [CspDirective.ScriptSrc, [customProtocol, "'unsafe-inline'", 'https://*.stripe.com']],
    [CspDirective.ImgSrc, [customProtocol, 'blob:', 'data:', 'https:', 'http:']],
    [CspDirective.StyleSrc, [customProtocol, "'unsafe-inline'", 'data:', 'https:', 'http:']],
    [CspDirective.FontSrc, [customProtocol, 'data:', 'https:*', 'http:*']],
    [CspDirective.ConnectSrc, [customProtocol, 'https:', 'http:', 'wss:', 'ws:']],
    [CspDirective.WorkerSrc, [customProtocol, 'blob:', 'https:', 'http:']],
    [CspDirective.FrameSrc, ['https:', 'http:', EDITICS_PROTOCOL]],
    [CspDirective.MediaSrc, [customProtocol, 'blob:', 'data:', 'https:', 'http:']],
  ]);
  const EDITICS_CSP_RULE = buildPolicy([
    [CspDirective.DefaultSrc, [EDITICS_PROTOCOL]],
    // x2t compiles its WASM module, OnlyOffice uses `new Function()`, and
    // the vendor editor pages contain inline `<script>` blocks.
    [CspDirective.ScriptSrc, [EDITICS_PROTOCOL, "'wasm-unsafe-eval'", "'unsafe-eval'", "'unsafe-inline'"]],
    [CspDirective.StyleSrc, [EDITICS_PROTOCOL, "'unsafe-inline'"]],
    [CspDirective.ImgSrc, [EDITICS_PROTOCOL, 'blob:', 'data:']],
    [CspDirective.FontSrc, [EDITICS_PROTOCOL, 'blob:', 'data:']],
    [CspDirective.ConnectSrc, [EDITICS_PROTOCOL, 'blob:', 'data:']],
    [CspDirective.WorkerSrc, [EDITICS_PROTOCOL, 'blob:']],
    [CspDirective.FrameSrc, [EDITICS_PROTOCOL]],
    [CspDirective.MediaSrc, [EDITICS_PROTOCOL, 'blob:', 'data:']],
    [CspDirective.ObjectSrc, ["'none'"]],
    [CspDirective.BaseUri, [EDITICS_PROTOCOL]],
    [CspDirective.FormAction, ["'none'"]],
    // Only the app can embed editics
    [CspDirective.FrameAncestors, [customProtocol, EDITICS_PROTOCOL]],
  ]);

  session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
    // Everything served by the editics scheme gets its policy, whatever the
    // resource type: workers get their policy from their own script response.
    if (details.url.startsWith(EDITICS_PROTOCOL)) {
      const responseHeaders = { ...details.responseHeaders };
      responseHeaders['Content-Security-Policy'] = [EDITICS_CSP_RULE];
      return callback({ responseHeaders });
    }

    if (details.resourceType !== 'mainFrame') {
      return callback({ responseHeaders: details.responseHeaders });
    }

    const responseHeaders = { ...details.responseHeaders };
    responseHeaders['Content-Security-Policy'] = [CSP_RULE];
    callback({ responseHeaders });
  });
}

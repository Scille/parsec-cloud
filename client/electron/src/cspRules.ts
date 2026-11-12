// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

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

// Unlike the web app (where CSP is handled by the Parsec server hosting the
// editics pages), Electron serves them from a static custom scheme and cannot
// know at build time which Parsec server the host page will talk to (a single
// app can have several logged-in devices, each with its own server).
//
// Instead, the trusted app declares the origins of the servers hosting the
// editics sessions right before an editics page is needed (see `openDocument`
// in `src/services/editics.ts`), and they are allowed in `connect-src` for all
// the editics pages. The origins are deliberately not narrowed down per page
// (i.e. an editics page is allowed to connect to any declared origin, even
// unrelated to the document it edits): there are very few of them (most likely
// a single one, the server of the logged-in device), and this keeps the
// mechanism simple.
//
// Note the editics pages themselves cannot influence those origins: they have
// no access to the `nativeAPI` provided by the preload script (which only runs
// in the main frame, i.e. the app itself), and cross-origin access to the
// parent window is forbidden by the browser.
const editicsServerOrigins = new Set<string>();

export function declareEditicsServerOrigin(origin: string): void {
  // The origin comes from the trusted app, but is still validated out of
  // caution: only a plain `http(s)://host[:port]` origin is accepted (see
  // `getEditicsSessionServerUrl` in `src/services/editics.ts`).
  if (!/^https?:\/\/[^/]+$/.test(origin)) {
    return;
  }
  // `Set` takes care of the deduplication
  editicsServerOrigins.add(origin);
}

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

  // Built on each request: the declared server origins may change at any time
  // (a new device could be logged in, or a new editics session started).
  const buildEditicsPolicy = (): string =>
    buildPolicy([
      [CspDirective.DefaultSrc, [EDITICS_PROTOCOL]],
      // x2t compiles its WASM module, OnlyOffice uses `new Function()`, and
      // the vendor editor pages contain inline `<script>` blocks.
      [CspDirective.ScriptSrc, [EDITICS_PROTOCOL, "'wasm-unsafe-eval'", "'unsafe-eval'", "'unsafe-inline'"]],
      [CspDirective.StyleSrc, [EDITICS_PROTOCOL, "'unsafe-inline'"]],
      [CspDirective.ImgSrc, [EDITICS_PROTOCOL, 'blob:', 'data:']],
      [CspDirective.FontSrc, [EDITICS_PROTOCOL, 'blob:', 'data:']],
      // In collaborative mode the editics host page connects to the Parsec
      // server hosting the editics session: only the origins declared by the
      // trusted app are allowed (see `editicsServerOrigins` above).
      [CspDirective.ConnectSrc, [EDITICS_PROTOCOL, 'blob:', 'data:', ...editicsServerOrigins]],
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
      responseHeaders['Content-Security-Policy'] = [buildEditicsPolicy()];
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

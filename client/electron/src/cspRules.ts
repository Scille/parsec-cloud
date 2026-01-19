// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { session } from 'electron';
import log from 'electron-log/main';
import { electronIsDev } from './utils';

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

interface CspDirectiveRule {
  type: CspDirective;
  sources: string[];
}

interface UrlPattern {
  host: RegExp;
  protocol?: string;
  path?: RegExp;
}

class CspRule {
  public readonly urlPattern: UrlPattern;
  private directiveRules: CspDirectiveRule[];
  private customScheme: string;

  constructor(urlPattern: UrlPattern, customScheme: string, directiveRules: CspDirectiveRule[]) {
    this.urlPattern = urlPattern;
    this.customScheme = customScheme;
    this.directiveRules = directiveRules;
  }

  public toString(): string {
    return this.directiveRules.map((rule) => `${rule.type} ${this.customScheme}: ${rule.sources.join(' ')}`).join('; ');
  }
}

// Set a CSP up for our application based on the custom scheme
export function setupContentSecurityPolicy(customScheme: string): void {
  const rules: CspRule[] = [];

  // Main app page (parsec-desktop://-/...)
  rules.push(
    new CspRule({ host: /^-$/, protocol: `${customScheme}:` }, customScheme, [
      { type: CspDirective.DefaultSrc, sources: ["'unsafe-inline'", "'unsafe-eval'", 'data:', 'blob:'] },
      {
        type: CspDirective.FrameSrc,
        sources: [
          'data:',
          'blob:',
          'https://*.stripe.com',
          'https://m.stripe.network',
          'https://*.hcaptcha.com',
          'https://*.parsec.cloud',
          ...(electronIsDev ? ['http://localhost:3000', 'http://safe.localhost:3000'] : []),
        ],
      },
      {
        type: CspDirective.ConnectSrc,
        sources: ['data:', 'blob:', 'https://*.stripe.com', 'https://*.hcaptcha.com', 'wss://*.parsec.cloud', 'ws://*.parsec.cloud'],
      },
      { type: CspDirective.ImgSrc, sources: ['data:', 'blob:', 'https://*.stripe.com'] },
      {
        type: CspDirective.ScriptSrc,
        sources: ["'unsafe-inline'", "'unsafe-eval'", 'https://*.stripe.com', 'https://hcaptcha.com', 'https://m.stripe.network'],
      },
      { type: CspDirective.StyleSrc, sources: ["'unsafe-inline'", 'https://*.stripe.com'] },
      { type: CspDirective.FontSrc, sources: ['data:'] },
      { type: CspDirective.WorkerSrc, sources: ['blob:', 'https://m.stripe.network'] },
    ]),
  );

  if (electronIsDev) {
    // DevTools in development mode
    rules.push(
      new CspRule({ host: /^devtools$/, protocol: 'devtools:' }, customScheme, [
        { type: CspDirective.DefaultSrc, sources: ["'unsafe-inline'", "'unsafe-eval'", 'data:', 'blob:', 'devtools:'] },
        { type: CspDirective.ScriptSrc, sources: ["'unsafe-inline'", "'unsafe-eval'"] },
        { type: CspDirective.StyleSrc, sources: ["'unsafe-inline'"] },
        { type: CspDirective.ImgSrc, sources: ['data:', 'blob:'] },
        { type: CspDirective.FontSrc, sources: ['data:'] },
      ]),
    );
  }

  // Parsec Cloud services
  rules.push(
    new CspRule({ host: /.+\.parsec\.cloud$/ }, customScheme, [
      { type: CspDirective.DefaultSrc, sources: ['data:', 'blob:', 'https://*.parsec.cloud'] },
      {
        type: CspDirective.ScriptSrc,
        sources: ["'unsafe-inline'", "'unsafe-eval'", 'https://*.stripe.com', 'https://hcaptcha.com', 'https://*.parsec.cloud'],
      },
      {
        type: CspDirective.ConnectSrc,
        sources: [
          'wss://*.parsec.cloud',
          'ws://*.parsec.cloud',
          'data:',
          'blob:',
          'https://*.stripe.com',
          'https://*.hcaptcha.com',
          'https://*.parsec.cloud',
        ],
      },
      { type: CspDirective.ImgSrc, sources: ['data:', 'blob:', 'https://*.stripe.com', 'https://*.parsec.cloud'] },
      { type: CspDirective.StyleSrc, sources: ["'unsafe-inline'", 'https://*.stripe.com', 'https://*.parsec.cloud'] },
      { type: CspDirective.FontSrc, sources: ['data:', 'https://*.parsec.cloud'] },
      {
        type: CspDirective.FrameSrc,
        sources: [
          'data:',
          'blob:',
          'https://*.stripe.com',
          'https://m.stripe.network',
          'https://*.hcaptcha.com',
          'https://*.parsec.cloud',
          ...(electronIsDev ? ['http://localhost:3000', 'http://safe.localhost:3000'] : []),
        ],
      },
      { type: CspDirective.WorkerSrc, sources: ['blob:', 'https://m.stripe.network', 'https://*.parsec.cloud'] },
    ]),
  );

  // CryptPad sandbox - OnlyOffice specific path (needs unsafe-eval, must come before general safe.localhost rule)
  rules.push(
    new CspRule({ host: /^cryptpad-.+-safe\.parsec\.cloud$/, path: /^\/common\/onlyoffice/ }, customScheme, [
      { type: CspDirective.DefaultSrc, sources: ['http://safe.localhost:3000', 'data:', 'blob:'] },
      { type: CspDirective.FrameSrc, sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'data:', 'blob:'] },
      {
        type: CspDirective.ScriptSrc,
        sources: ['http://safe.localhost:3000', 'http://localhost:3000', "'unsafe-inline'", "'unsafe-eval'"],
      },
      { type: CspDirective.StyleSrc, sources: ['http://safe.localhost:3000', "'unsafe-inline'"] },
      { type: CspDirective.ImgSrc, sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'data:', 'blob:'] },
      {
        type: CspDirective.ConnectSrc,
        sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'ws://safe.localhost:3000', 'data:', 'blob:'],
      },
      { type: CspDirective.FontSrc, sources: ['http://safe.localhost:3000', 'data:'] },
      { type: CspDirective.WorkerSrc, sources: ['http://safe.localhost:3000', 'blob:'] },
    ]),
  );

  // CryptPad sandbox (general rule)
  rules.push(
    new CspRule({ host: /^cryptpad-.+-safe\.parsec\.cloud$/ }, customScheme, [
      { type: CspDirective.DefaultSrc, sources: ['http://safe.localhost:3000', 'data:', 'blob:'] },
      { type: CspDirective.FrameSrc, sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'data:', 'blob:'] },
      { type: CspDirective.ScriptSrc, sources: ['http://safe.localhost:3000', 'http://localhost:3000', "'unsafe-inline'"] },
      { type: CspDirective.StyleSrc, sources: ['http://safe.localhost:3000', "'unsafe-inline'"] },
      { type: CspDirective.ImgSrc, sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'data:', 'blob:'] },
      {
        type: CspDirective.ConnectSrc,
        sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'ws://safe.localhost:3000', 'data:', 'blob:'],
      },
      { type: CspDirective.FontSrc, sources: ['http://safe.localhost:3000', 'data:'] },
      { type: CspDirective.WorkerSrc, sources: ['http://safe.localhost:3000', 'blob:'] },
    ]),
  );

  // Stripe payment integration
  rules.push(
    new CspRule({ host: /.+\.stripe\.com$/ }, customScheme, [
      { type: CspDirective.DefaultSrc, sources: [] },
      { type: CspDirective.ScriptSrc, sources: ['https://*.stripe.com', "'unsafe-inline'"] },
      { type: CspDirective.ConnectSrc, sources: ['https://*.stripe.com'] },
      { type: CspDirective.FrameSrc, sources: ['https://*.stripe.com', 'https://m.stripe.network'] },
      { type: CspDirective.ImgSrc, sources: ['https://*.stripe.com', 'data:'] },
      { type: CspDirective.StyleSrc, sources: ['https://*.stripe.com', "'unsafe-inline'"] },
    ]),
  );

  // Stripe CDN
  rules.push(
    new CspRule({ host: /^b\.stripecdn\.com$/ }, customScheme, [
      { type: CspDirective.DefaultSrc, sources: [] },
      { type: CspDirective.ScriptSrc, sources: ['https://b.stripecdn.com'] },
      { type: CspDirective.ConnectSrc, sources: ['https://b.stripecdn.com'] },
      { type: CspDirective.FrameSrc, sources: ['https://b.stripecdn.com'] },
      { type: CspDirective.ImgSrc, sources: ['https://b.stripecdn.com', 'data:'] },
      { type: CspDirective.StyleSrc, sources: ['https://b.stripecdn.com', "'unsafe-inline'"] },
    ]),
  );

  // Stripe network
  rules.push(
    new CspRule({ host: /^m\.stripe\.network$/ }, customScheme, [
      { type: CspDirective.FrameSrc, sources: ['https://m.stripe.network'] },
      { type: CspDirective.WorkerSrc, sources: ['https://m.stripe.network', 'blob:'] },
      { type: CspDirective.ScriptSrc, sources: ['https://m.stripe.network', "'unsafe-inline'"] },
    ]),
  );

  // hCaptcha
  rules.push(
    new CspRule({ host: /^hcaptcha\.com$/ }, customScheme, [
      { type: CspDirective.ScriptSrc, sources: ['https://hcaptcha.com', "'unsafe-inline'"] },
    ]),
  );

  // hCaptcha subdomains
  rules.push(
    new CspRule({ host: /.+\.hcaptcha\.com$/ }, customScheme, [
      { type: CspDirective.ConnectSrc, sources: ['https://*.hcaptcha.com'] },
      { type: CspDirective.FrameSrc, sources: ['https://*.hcaptcha.com'] },
    ]),
  );

  if (electronIsDev) {
    // Development testbed
    rules.push(
      new CspRule({ host: /^localhost:6770$/ }, customScheme, [
        { type: CspDirective.ConnectSrc, sources: ['http://localhost:6770', 'ws://localhost:6770'] },
      ]),
    );

    // CryptPad development server
    rules.push(
      new CspRule({ host: /^localhost:3000$/ }, customScheme, [
        { type: CspDirective.DefaultSrc, sources: ['http://localhost:3000', 'data:', 'blob:'] },
        { type: CspDirective.FrameSrc, sources: ['http://localhost:3000', 'http://safe.localhost:3000', 'data:', 'blob:'] },
        {
          type: CspDirective.ScriptSrc,
          sources: ['http://localhost:3000', 'http://safe.localhost:3000', "'unsafe-inline'", "'unsafe-eval'"],
        },
        { type: CspDirective.StyleSrc, sources: ['http://localhost:3000', 'http://safe.localhost:3000', "'unsafe-inline'"] },
        { type: CspDirective.ImgSrc, sources: ['http://localhost:3000', 'http://safe.localhost:3000', 'data:', 'blob:'] },
        {
          type: CspDirective.ConnectSrc,
          sources: [
            'http://localhost:3000',
            'http://safe.localhost:3000',
            'ws://localhost:3000',
            'ws://safe.localhost:3000',
            'data:',
            'blob:',
          ],
        },
        { type: CspDirective.FontSrc, sources: ['http://localhost:3000', 'http://safe.localhost:3000', 'data:'] },
        { type: CspDirective.WorkerSrc, sources: ['http://localhost:3000', 'http://safe.localhost:3000', 'blob:'] },
      ]),
    );

    // CryptPad sandbox - OnlyOffice specific path (needs unsafe-eval, must come before general safe.localhost rule)
    rules.push(
      new CspRule({ host: /^safe\.localhost:3000$/, path: /^\/common\/onlyoffice/ }, customScheme, [
        { type: CspDirective.DefaultSrc, sources: ['http://safe.localhost:3000', 'data:', 'blob:'] },
        { type: CspDirective.FrameSrc, sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'data:', 'blob:'] },
        {
          type: CspDirective.ScriptSrc,
          sources: ['http://safe.localhost:3000', 'http://localhost:3000', "'unsafe-inline'", "'unsafe-eval'"],
        },
        { type: CspDirective.StyleSrc, sources: ['http://safe.localhost:3000', "'unsafe-inline'"] },
        { type: CspDirective.ImgSrc, sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'data:', 'blob:'] },
        {
          type: CspDirective.ConnectSrc,
          sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'ws://safe.localhost:3000', 'data:', 'blob:'],
        },
        { type: CspDirective.FontSrc, sources: ['http://safe.localhost:3000', 'data:'] },
        { type: CspDirective.WorkerSrc, sources: ['http://safe.localhost:3000', 'blob:'] },
      ]),
    );

    // CryptPad sandbox (general rule)
    rules.push(
      new CspRule({ host: /^safe\.localhost:3000$/ }, customScheme, [
        { type: CspDirective.DefaultSrc, sources: ['http://safe.localhost:3000', 'data:', 'blob:'] },
        { type: CspDirective.FrameSrc, sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'data:', 'blob:'] },
        { type: CspDirective.ScriptSrc, sources: ['http://safe.localhost:3000', 'http://localhost:3000', "'unsafe-inline'"] },
        { type: CspDirective.StyleSrc, sources: ['http://safe.localhost:3000', "'unsafe-inline'"] },
        { type: CspDirective.ImgSrc, sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'data:', 'blob:'] },
        {
          type: CspDirective.ConnectSrc,
          sources: ['http://safe.localhost:3000', 'http://localhost:3000', 'ws://safe.localhost:3000', 'data:', 'blob:'],
        },
        { type: CspDirective.FontSrc, sources: ['http://safe.localhost:3000', 'data:'] },
        { type: CspDirective.WorkerSrc, sources: ['http://safe.localhost:3000', 'blob:'] },
      ]),
    );
  }

  // Handle special cases with empty host or theme host
  rules.push(new CspRule({ host: /^$/ }, customScheme, [{ type: CspDirective.DefaultSrc, sources: ["'none'"] }]));

  rules.push(
    new CspRule({ host: /^theme$/, protocol: 'devtools:' }, customScheme, [
      { type: CspDirective.DefaultSrc, sources: ["'unsafe-inline'", 'data:'] },
      { type: CspDirective.StyleSrc, sources: ["'unsafe-inline'"] },
    ]),
  );

  session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
    const responseHeaders = { ...details.responseHeaders };
    const url = new URL(details.url);

    // Find matching rule based on protocol, domain, and optionally path
    const matchingRule = rules.find((rule) => {
      // Check protocol if specified
      if (rule.urlPattern.protocol && rule.urlPattern.protocol !== url.protocol) {
        return false;
      }

      // Check host pattern
      if (!rule.urlPattern.host.test(url.host)) {
        return false;
      }

      // Check path pattern if specified
      if (rule.urlPattern.path && !rule.urlPattern.path.test(url.pathname)) {
        return false;
      }

      return true;
    });

    if (!matchingRule) {
      log.error(
        `No CSP rule found for protocol: ${url.protocol}, host: ${url.host}, path: ${url.pathname.substring(0, 100)}${
          url.pathname.length > 100 ? '...' : ''
        }`,
      );
      // Apply a strict CSP to block potentially dangerous resources
      responseHeaders['Content-Security-Policy'] = ["default-src 'none'"];
      callback({ responseHeaders });
      return;
    }

    let cspString = matchingRule.toString();

    // Add devtools support in dev mode
    if (electronIsDev && !cspString.includes('devtools://')) {
      cspString = cspString.replace(CspDirective.DefaultSrc, `${CspDirective.DefaultSrc} devtools://*`);
    }

    responseHeaders['Content-Security-Policy'] = [cspString];
    callback({ responseHeaders });
  });
}

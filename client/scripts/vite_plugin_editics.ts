// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Serve/bundles the editics host page (i.e. `editics/*.html`). This must be done
// outside of the main app since since editics host page is loaded in an iframe
// hence must stay standalone (no shared chunks with the app bundle, no `@/` imports).

import { build as esbuildBundle } from 'esbuild';
import crypto from 'crypto';
import fs from 'fs';
import path from 'path';
import { Plugin, ConfigEnv, PluginOption, loadEnv } from 'vite';
import { viteStaticCopy } from 'vite-plugin-static-copy';

interface EditicsHostPagesOptions {
  // Same target as the app build (see `build.target` in vite.config.ts).
  buildTarget: string | string[];
  rootDir: string;
  // e.g. `editics/offline.html`
  htmlPath: string;
  // e.g. `offline.ts`
  scriptEntry: string;
  // Build-time injected constants (see the `editics-build-time-paths` plugin).
  define: Record<string, string>;
}

function buildEditicsHostPage(options: EditicsHostPagesOptions): Plugin {
  return {
    name: 'build-editics-host-pages',
    apply: 'build',
    // `post` is required for the `editics-offline-page` meta tag injection into `index.html`
    generateBundle: {
      order: 'post',
      async handler(_options, bundle) {
        const htmlPath = path.resolve(options.rootDir, options.htmlPath);
        const htmlSource = await fs.promises.readFile(htmlPath, 'utf8');

        // The script entry file must be in the same directory as the HTML file
        if (options.scriptEntry.indexOf('/') !== -1) {
          throw new Error(`Invalid script entry \`${options.scriptEntry}\`: should be a file name, not a path`);
        }

        // Replace the `<script type="module" src="./<entry>">` tag with the
        // bundled code inlined.
        const scriptTag = new RegExp(`<script\\s+[^>]*src="\\./${options.scriptEntry}"[^>]*>\\s*</script>`);
        if (htmlSource.match(scriptTag)?.length !== 1) {
          throw new Error(`${htmlPath}: cannot find \`<script type="module" src="./${options.scriptEntry}">\``);
        }

        const result = await esbuildBundle({
          entryPoints: [path.resolve(options.rootDir, path.dirname(options.htmlPath), options.scriptEntry)],
          // Only used to name the in-memory outputs (nothing is written, see
          // `write: false` below); the sourcemap output path requires it.
          outdir: options.scriptEntry,
          bundle: true,
          // Keep the module semantics of the replaced tag (isolated scope,
          // deferred execution); the bundle is self-contained anyway.
          format: 'esm',
          target: options.buildTarget,
          minify: true,
          sourcemap: true,
          write: false,
          logLevel: 'silent',
          define: options.define,
        });
        const jsOutput = result.outputFiles.find((file) => file.path.endsWith('.js'));
        const mapOutput = result.outputFiles.find((file) => file.path.endsWith('.js.map'));
        if (!jsOutput || !mapOutput) {
          throw new Error(`${htmlPath}: unexpected esbuild output for ${options.scriptEntry}`);
        }

        // The bundle is inlined in the page: since the page name is hashed
        // for cache busting (see below), the script does not need a hashed
        // name of its own.
        const basename = path.basename(options.htmlPath, '.html');
        // esbuild emits `<entry>.js.map`; rename it after the HTML file so the
        // `sourceMappingURL` of the inlined script resolves relative to the
        // document URL.
        const mapFileName = path.posix.join(path.dirname(options.htmlPath), `${basename}.html.map`);
        const code = jsOutput.text.replace(/^\/\/# sourceMappingURL=.*$/m, `//# sourceMappingURL=${path.basename(mapFileName)}`);

        const htmlWithScript = htmlSource.replace(scriptTag, () => `<script type="module">\n${code}\n</script>`);
        this.emitFile({ type: 'asset', fileName: mapFileName, source: mapOutput.text });

        // Hash the final HTML for cache busting: since the script is inlined,
        // the hash covers the whole editics host page (including the bundle).
        const htmlHash = crypto.createHash('sha256').update(htmlWithScript).digest('hex').slice(0, 8);
        const htmlFileName = path.posix.join(path.dirname(options.htmlPath), `${basename}-${htmlHash}.html`);
        this.emitFile({ type: 'asset', fileName: htmlFileName, source: htmlWithScript });

        // Advertise the hashed host page name to the app through a meta tag
        // injected in `index.html`.
        // This must run after Vite's HTML plugin has emitted the final `index.html`,
        // hence the `post` ordering in this bundler.
        const indexHtml = Object.values(bundle).find(
          (file): file is Extract<typeof file, { type: 'asset' }> => file.type === 'asset' && file.fileName === 'index.html',
        );
        if (!indexHtml) {
          throw new Error('Cannot find the emitted `index.html` to inject `editics-offline-page` tag');
        }
        const indexHtmlSource = indexHtml.source;
        const metaTag = `<meta name="editics-offline-page" content="${htmlFileName}">`;
        if (!/<head>/i.test(indexHtmlSource)) {
          throw new Error('Cannot find `<head>` in `index.html` to inject the editics host page name');
        }
        indexHtml.source = indexHtmlSource.replace(/<head>/i, `<head>\n    ${metaTag}`);
      },
    },
  };
}

export default function generateEditicsPlugins(env: ConfigEnv, buildTarget: string | string[]): PluginOption[] {
  const plugins: PluginOption[] = [];

  const ENABLE_EDITICS_VARIABLE = 'PARSEC_APP_ENABLE_EDITICS';
  const isEditicsEnabled = loadEnv(env.mode, process.cwd(), ENABLE_EDITICS_VARIABLE)[ENABLE_EDITICS_VARIABLE] === 'true';
  if (!isEditicsEnabled) {
    return plugins;
  }

  // The OnlyOffice assets are copied verbatim: they are not part of Vite's
  // build, hence they get no hashed name nor cache busting query parameter.
  // Cache busting is instead provided by the dependency version: the packages
  // are copied into versioned folders inside the editics folder (e.g.
  // `dist/editics/onlyoffice/9.3.0/`, since they are only used by editics),
  // and the host page URLs are injected at build time (see the
  // `editics-build-time-paths` plugin below).
  function readPackageVersion(pkgName: string): string {
    const pkgJsonPath = path.join(import.meta.dirname, '..', 'node_modules', pkgName, 'package.json');
    const { version } = JSON.parse(fs.readFileSync(pkgJsonPath, 'utf8'));
    // `+` (e.g. `9.3.0.140+parsec0`) is decoded as a space by some
    // servers/CDNs, hence it is not a safe character for a URL path.
    return version.replace(/\+/g, '-');
  }

  const onlyofficeVersion = readPackageVersion('onlyoffice-editor');
  const x2tVersion = readPackageVersion('onlyoffice-x2t');
  // Paths relative to the host page (i.e. `editics/offline.html`, which lives
  // in the same folder as the OnlyOffice asset trees).
  const onlyofficeBase = `onlyoffice/${onlyofficeVersion}/`;
  const x2tBase = `onlyoffice-x2t/${x2tVersion}/`;

  const define: Record<string, string> = {
    __EDITICS_ONLYOFFICE_BASE__: JSON.stringify(onlyofficeBase),
    __EDITICS_X2T_BASE__: JSON.stringify(x2tBase),
  };

  // Expose the versioned asset paths to the host page sources at dev time
  // (for the release build they are provided to esbuild instead, see
  // `buildEditicsHostPage` below).
  plugins.push({
    name: 'editics-build-time-paths',
    config() {
      return { define };
    },
  });

  const targets = [
    {
      src: ['node_modules/onlyoffice-editor/**/*', '!node_modules/onlyoffice-editor/**/package.json'],
      dest: `editics/onlyoffice/${onlyofficeVersion}`,
      rename: { stripBase: 2 },
    },
    // OnlyOffice's editor pages register this worker at the root of their
    // asset tree (`/editics/onlyoffice/<version>/document_editor_service_worker.js`),
    // while the vendor package stores it under `sdkjs/common/serviceworker`...
    {
      src: 'node_modules/onlyoffice-editor/sdkjs/common/serviceworker/document_editor_service_worker.js',
      dest: `editics/onlyoffice/${onlyofficeVersion}`,
      rename: { stripBase: 5 },
    },
    {
      src: ['node_modules/onlyoffice-x2t/**/*', '!node_modules/onlyoffice-x2t/**/package.json'],
      dest: `editics/onlyoffice-x2t/${x2tVersion}`,
      rename: { stripBase: 2 },
    },
    {
      src: 'editics/templates/*',
      dest: 'onlyoffice-templates',
      rename: { stripBase: 2 },
    },
  ];
  plugins.push(viteStaticCopy({ targets }));

  if (env.command === 'build') {
    // Bundle the editics host pages (i.e. `editics/*.html`) when in release.
    // In dev mode, Vite out-of-the-box serves the pages and transpiles the
    // TypeScript entries on the fly.
    // plugins.push(editicsHostPages({ buildTarget, rootDir: path.join(import.meta.dirname, '..') }));
    const rootDir = path.join(import.meta.dirname, '..');
    plugins.push(
      buildEditicsHostPage({
        rootDir,
        htmlPath: 'editics/offline.html',
        scriptEntry: 'offline.ts',
        buildTarget,
        define,
      }),
    );
  } else {
    // OnlyOffice  builds font URLs by concatenating the fonts directory (i.e.
    // `/editics/onlyoffice/<version>/fonts/`) with `/fonts/<name>.ttf` (see
    // `onlyoffice-editor/sdkjs/common/AllFonts.js`).
    // So we end up with a doubled slash in the URL (e.g.
    // `editics/onlyoffice/<version>/fonts//fonts/arial.ttf`),
    // but `vite-plugin-static-copy` only recognizes canonized paths...
    const doubledSlashFontPrefix = `/editics/onlyoffice/${onlyofficeVersion}/fonts//fonts/`;
    const canonizedFontPrefix = `/editics/onlyoffice/${onlyofficeVersion}/fonts/fonts/`;
    plugins.push({
      name: 'normalize-onlyoffice-font-urls',
      configureServer(server) {
        server.middlewares.use((req, _res, next) => {
          if (req.url?.startsWith(doubledSlashFontPrefix)) {
            req.url = req.url.replace(doubledSlashFontPrefix, canonizedFontPrefix);
          }
          next();
        });
      },
    });

    // Serve the editics origin (i.e. `editics.<host>`) with the CSP it is expected
    // to have in release, so missing directives show up early.
    // By default the policy is only reported (violations are logged in the Vite
    // terminal, nothing is blocked); set `PARSEC_APP_EDITICS_DEV_CSP=enforce` to
    // enforce it instead.
    const DEV_CSP_VARIABLE = 'PARSEC_APP_EDITICS_DEV_CSP';
    const enforceDevCsp = loadEnv(env.mode, process.cwd(), DEV_CSP_VARIABLE)[DEV_CSP_VARIABLE] === 'enforce';
    plugins.push({
      name: 'editics-dev-csp',
      configureServer(server) {
        server.middlewares.use((req, res, next) => {
          if (req.method === 'POST' && req.url === EDITICS_DEV_CSP_REPORT_PATH) {
            let body = '';
            req.setEncoding('utf8');
            req.on('data', (chunk: string) => {
              body += chunk;
            });
            req.on('end', () => {
              server.config.logger.warn(formatCspReport(body), { timestamp: true });
              res.statusCode = 204;
              res.end();
            });
            return;
          }

          const host = req.headers.host;
          if (host?.startsWith('editics.')) {
            const protocol = server.config.server.https ? 'https' : 'http';
            const parentOrigin = `${protocol}://${host.slice('editics.'.length)}`;
            res.setHeader(
              enforceDevCsp ? 'Content-Security-Policy' : 'Content-Security-Policy-Report-Only',
              buildEditicsDevCsp(parentOrigin),
            );
          }
          next();
        });
      },
    });
  }

  return plugins;
}

const EDITICS_DEV_CSP_REPORT_PATH = '/__editics-csp-report';

// Starting point of the editics origin's policy, meant to be tightened/extended
// according to the reported violations.
function buildEditicsDevCsp(parentOrigin: string): string {
  return [
    "default-src 'self'",
    // x2t compiles its WASM module, OnlyOffice uses `new Function()`
    "script-src 'self' 'wasm-unsafe-eval' 'unsafe-eval' 'unsafe-inline'",
    "style-src 'self' 'unsafe-inline'",
    "img-src 'self' blob: data:",
    "font-src 'self' blob: data:",
    "connect-src 'self' blob: data:",
    "worker-src 'self' blob:",
    "frame-src 'self'",
    "media-src 'self' blob: data:",
    "object-src 'none'",
    "base-uri 'self'",
    "form-action 'none'",
    `frame-ancestors 'self' ${parentOrigin}`,
    `report-uri ${EDITICS_DEV_CSP_REPORT_PATH}`,
  ].join('; ');
}

function formatCspReport(body: string): string {
  try {
    const report = JSON.parse(body)['csp-report'] as Record<string, unknown>;
    const directive = report['effective-directive'] ?? report['violated-directive'];
    const location = report['source-file'] ? ` at ${report['source-file']}:${report['line-number']}` : '';
    return `[editics CSP] ${directive} blocked ${report['blocked-uri']} (in ${report['document-uri']}${location})`;
  } catch {
    return `[editics CSP] unparsable report: ${body}`;
  }
}

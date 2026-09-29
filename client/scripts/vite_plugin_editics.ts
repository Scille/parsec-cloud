// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Serve/bundles the editics host page (i.e. `editics/*.html`). This must be done
// outside of the main app since since editics host page is loaded in an iframe
// hence must stay standalone (no shared chunks with the app bundle, no `@/` imports).

import { build as esbuildBundle } from 'esbuild';
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
}

function buildEditicsHostPage(options: EditicsHostPagesOptions): Plugin {
  return {
    name: 'build-editics-host-pages',
    apply: 'build',
    async generateBundle() {
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
      });
      const jsOutput = result.outputFiles.find((file) => file.path.endsWith('.js'));
      const mapOutput = result.outputFiles.find((file) => file.path.endsWith('.js.map'));
      if (!jsOutput || !mapOutput) {
        throw new Error(`${htmlPath}: unexpected esbuild output for ${options.scriptEntry}`);
      }

      // esbuild emits `<basename>.js.map`; rename it after the HTML file so
      // the `sourceMappingURL` of the inlined script resolves relative to
      // the document URL.
      const mapFileName = path.posix.join(path.dirname(options.htmlPath), `${path.basename(options.htmlPath)}.map`);
      const code = jsOutput.text.replace(/^\/\/# sourceMappingURL=.*$/m, `//# sourceMappingURL=${path.basename(mapFileName)}`);

      const inlined = htmlSource.replace(scriptTag, () => `<script type="module">\n${code}\n</script>`);
      this.emitFile({ type: 'asset', fileName: options.htmlPath, source: inlined });
      this.emitFile({ type: 'asset', fileName: mapFileName, source: mapOutput.text });
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

  const targets = [
    {
      src: ['node_modules/onlyoffice-editor/**/*', '!node_modules/onlyoffice-editor/**/package.json'],
      dest: 'onlyoffice',
      rename: { stripBase: 2 },
    },
    // OnlyOffice's editor pages register this worker at the root of their
    // asset tree (`/onlyoffice/document_editor_service_worker.js`), while
    // the vendor package stores it under `sdkjs/common/serviceworker`...
    {
      src: 'node_modules/onlyoffice-editor/sdkjs/common/serviceworker/document_editor_service_worker.js',
      dest: 'onlyoffice',
      rename: { stripBase: 5 },
    },
    {
      src: ['node_modules/onlyoffice-x2t/**/*', '!node_modules/onlyoffice-x2t/**/package.json'],
      dest: 'onlyoffice-x2t',
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
      }),
    );
  } else {
    // OnlyOffice  builds font URLs by concatenating the fonts directory (i.e.
    // `/onlyoffice/fonts/`) with `/fonts/<name>.ttf` (see `onlyoffice-editor/sdkjs/common/AllFonts.js`).
    // So we end up with a doubled slash in the URL (e.g. `onlyoffice/fonts//fonts/arial.ttf`),
    // but `vite-plugin-static-copy` only recognizes canonized paths...
    plugins.push({
      name: 'normalize-onlyoffice-font-urls',
      configureServer(server) {
        server.middlewares.use((req, _res, next) => {
          if (req.url?.startsWith('/onlyoffice/fonts//fonts/')) {
            req.url = req.url.replace('/onlyoffice/fonts//fonts/', '/onlyoffice/fonts/fonts/');
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
    "script-src 'self' 'wasm-unsafe-eval' 'unsafe-eval'",
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
    `frame-ancestors ${parentOrigin}`,
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

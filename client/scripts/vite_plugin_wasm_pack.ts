// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import fs from 'fs-extra';
import crypto from 'crypto';
import path from 'path';
import { PluginOption, ResolvedConfig } from 'vite';

type WasmPackCrate = {
  path: string;
  name: string;
};

function vitePluginWasmPack(
  crates: WasmPackCrate[],
  // If the wasm module runs in a web worker, we are required to instantiate this
  // plugin multiple times:
  // - One main plugin instance
  // - Another instance for each web worker
  //
  // However we only need to copy the wasm file once ! Hence this flag that should
  // only be enabled for the main plugin instance.
  emitReleaseAsset: boolean,
): PluginOption {
  const prefix = '@vite-plugin-wasm-pack@';
  const pkg = 'pkg'; // default folder of wasm-pack module
  let configIsWorker: boolean;
  let configIsProduction: boolean;
  let configAssetsDir: string;
  let configBase: string;

  function retrieveCrate(source: string): WasmPackCrate | null {
    for (let i = 0; i < crates.length; i++) {
      if (crates[i].name === source) {
        return crates[i];
      }
    }
    return null;
  }

  function generateDigest(content: crypto.BinaryLike): string {
    return crypto.createHash('sha256').update(content).digest('hex').substring(0, 8);
  }

  function getJsEntryPointPath(crate: WasmPackCrate): string {
    return path.join(crate.path, pkg, `${crate.name}.js`);
  }

  function getJsInternalPath(crate: WasmPackCrate, relativePath: string): string {
    return path.join(crate.path, pkg, relativePath);
  }

  function getWasmFilePath(crate: WasmPackCrate): string {
    return path.join(crate.path, pkg, `${crate.name}_bg.wasm`);
  }

  // Sanity check to make sure the crates have been built
  for (const crate of crates) {
    // Ignore the .d.ts files given they get modifed among .wasm/.js
    const targets = [getJsEntryPointPath(crate), getWasmFilePath(crate)];
    for (const target of targets) {
      if (!fs.existsSync(target)) {
        throw new Error(`*** Can't find \`${target}\`, run \`./make.py web-dev-install\` first ***`);
      }
    }
  }

  return {
    name: 'vite-plugin-wasm-pack',
    enforce: 'pre',

    configResolved(resolvedConfig: ResolvedConfig): void {
      configIsWorker = resolvedConfig.isWorker;
      configIsProduction = resolvedConfig.isProduction;
      configAssetsDir = resolvedConfig.build.assetsDir;
      configBase = resolvedConfig.base;

      if (!configBase.startsWith('/')) {
        throw new Error('`BASE_URL` is required to be absolute (typically `BASE_URL="/client"`)');
      }
      // Sanity check: vite is supposed to take care of missing trailing `/` in BASE_URL
      if (!configBase.endsWith('/')) {
        throw new Error('`resolvedConfig.base` is missing a trailing `/`');
      }
    },

    transform(code: string, id: string): void {
      // Add watchers to detect changes on the crate
      if (id.indexOf(prefix) === 0) {
        const source = id.replace(prefix, '');
        const crate = retrieveCrate(source);
        if (crate) {
          const targets = [getJsEntryPointPath(crate), getWasmFilePath(crate)];
          for (const target of targets) {
            this.addWatchFile(path.resolve(target));
          }
        }
      }
    },

    async resolveId(source: string, importer: string | undefined): Promise<string | null> {
      // Handle import of the crate itself (e.g. `import * from 'libparsec_bindings_web'`)
      const crate = retrieveCrate(source);
      if (crate) {
        return prefix + source;
      }

      // Handle imports within the crate itself (e.g. `import * from './snippets/wasm-streams/inline0.js'`;)
      if (importer && importer.indexOf(prefix) === 0) {
        const crateName = importer.replace(prefix, '');
        const crate = retrieveCrate(crateName);
        if (crate) {
          // e.g. `@vite-plugin-wasm-pack@libparsec_bindings_web@./snippets/wasm-streams/inline0.js`
          return `${prefix}${crateName}@${source}`;
        }
      }

      return null;
    },

    async load(id: string): Promise<string | null> {
      if (id.indexOf(prefix) === 0) {
        const source = id.replace(prefix, '');
        const splitted = source.split('@');

        let crate: WasmPackCrate | null = null;
        let relativePath: string | null = null;
        switch (splitted.length) {
          case 1: {
            crate = retrieveCrate(splitted[0]);
            break;
          }

          case 2: {
            crate = retrieveCrate(splitted[0]);
            relativePath = splitted[1];
            break;
          }
        }

        if (!crate) {
          return null;
        }

        if (relativePath) {
          const code = await fs.promises.readFile(getJsInternalPath(crate, relativePath), {
            encoding: 'utf-8',
          });
          return code;
        } else {
          let code = await fs.promises.readFile(getJsEntryPointPath(crate), {
            encoding: 'utf-8',
          });

          // Patch the load path according to the asset directory

          let wasmFileName = `${crate.name}_bg.wasm`;
          if (configIsProduction) {
            const content = fs.readFileSync(getWasmFilePath(crate));
            wasmFileName = `${crate.name}_bg-${generateDigest(content)}.wasm`;
          }

          const regex = /module_or_path = new URL\('(.+)'.+;/g;
          let found = false;
          code = code.replace(regex, (_match, _group1) => {
            // Here we are patching the URL path to the `XXX_bg.wasm` file.
            //
            // We'd rather avoid using an absolute path in order to allow the
            // output to be served from an arbitrary prefix (e.g. `/client` when
            // served from the Parsec server).
            //
            // However relative resolution is a bit tricky:
            // - For web worker, the base path is the worker's script (e.g. `/assets/worker.js`)
            //   which lives in the same asset folder as the `XXX_bg.wasm` file.
            // - For the main web page, the base path is set with the `<base>`'s href
            //   tag (e.g. `<base href="/client/">`). In this case, we need to append
            //   the asset folder to the relative path (e.g. `./assets/XXX_bg.wasm`).
            //
            // TODO: for whatever reason, in the main web page, JS's `fetch` doesn't
            // use the `document.baseURI` (i.e. `<base>`'s href) to resolve relative
            // URLs (like it is supposed to !).
            // So instead we do the resolve ourselves using `BASE_URL` (which hence must
            // is required to itself be absolute !) and provide an absolute URL.
            found = true;
            if (configIsWorker) {
              return `module_or_path = "${wasmFileName}";`;
            } else {
              return `module_or_path = "${configBase}${configAssetsDir}/${wasmFileName}";`;
            }
          });

          if (!found) {
            throw new Error(`Can't find the URL path in the JS entrypoint of \`${crate.name}\``);
          }

          return code;
        }
      }
      return null;
    },

    // Serve the .wasm files
    configureServer({ middlewares }): () => void {
      const targets = new Map<string, string>(
        crates.map((crate) => {
          const wasmFileName = `${crate.name}_bg.wasm`;
          const wasmFilePath = path.join(crate.path, pkg, wasmFileName);
          return [wasmFileName, wasmFilePath];
        }),
      );
      return () => {
        middlewares.use((req, res, next) => {
          if (typeof req.originalUrl === 'string') {
            const basename = path.basename(req.originalUrl);
            const wasmFilePath = targets.get(basename);
            if (wasmFilePath) {
              res.setHeader(
                'Cache-Control',
                'no-cache, no-store, must-revalidate',
              );
              res.writeHead(200, { 'Content-Type': 'application/wasm' });
              fs.createReadStream(wasmFilePath).pipe(res);
            } else {
              next();
            }
          }
        });
      };
    },

    // Copy all .wasm files in the asset folder
    buildEnd(): void {
      if (!emitReleaseAsset) {
        return;
      }
      for (const crate of crates) {
        const content = fs.readFileSync(getWasmFilePath(crate));
        const wasmFileName = `${crate.name}_bg-${generateDigest(content)}.wasm`;
        this.emitFile({
          type: 'asset',
          fileName: `${configAssetsDir}/${wasmFileName}`,
          source: content,
        });
      }
    },
  };
}

export default vitePluginWasmPack;

import fs from 'fs-extra';
import crypto from 'crypto';
import path from 'path';
import { PluginOption } from 'vite';

type WasmPackCrate = {
  path: string;
  name: string;
};

function vitePluginWasmPack(
  crates: WasmPackCrate[]
): PluginOption {
  const prefix = '@vite-plugin-wasm-pack@';
  const pkg = 'pkg'; // default folder of wasm-pack module
  let configBase: string;
  let configAssetsDir: string;
  let configIsProduction: boolean;

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

    configResolved(resolvedConfig): void {
      configIsProduction = resolvedConfig.isProduction;
      configBase = resolvedConfig.base;
      configAssetsDir = resolvedConfig.build.assetsDir;
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

    async resolveId(source: string): Promise<string|null> {
      const crate = retrieveCrate(source);
      if (crate) {
        return prefix + source;
      }
      return null;
    },

    async load(id: string): Promise<string|null> {
      if (id.indexOf(prefix) === 0) {
        const source = id.replace(prefix, '');
        const crate = retrieveCrate(source);
        if (crate) {
          let code = await fs.promises.readFile(getJsEntryPointPath(crate), {
            encoding: 'utf-8'
          });

          // Patch the load path according to the asset directory

          let wasmFileName = `${crate.name}_bg.wasm`;
          // Cypress uses `vite build` so it appears as production
          if (configIsProduction && !process.env.CYPRESS) {
            const content = fs.readFileSync(getWasmFilePath(crate));
            wasmFileName = `${crate.name}_bg-${generateDigest(content)}.wasm`;
          }

          const regex = /input = new URL\('(.+)'.+;/g;
          code = code.replace(regex, (_match, group1) => {
            const assetUrl = path.posix.join(
              configBase,
              configAssetsDir,
              wasmFileName
            );
            return `input = "${assetUrl}";`;
          });

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
        })
      );
      return () => {
        middlewares.use((req, res, next) => {
          if (typeof req.url === 'string') {
            const basename = path.basename(req.url);
            const wasmFilePath = targets.get(basename);
            if (wasmFilePath) {
              res.setHeader(
                'Cache-Control',
                'no-cache, no-store, must-revalidate'
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
      // Cypress is not able to serve assets (everything is supposed to be inlined
      // in the final .js test file).
      // If you wonder how the wasm file is provided in this case, have a look at
      // `fetch_libparsec_wasm` task in `cypress.config.ts` ;-)
      if (process.env.CYPRESS) {
        return;
      }
      for (const crate of crates) {
        const content = fs.readFileSync(getWasmFilePath(crate));
        const wasmFileName = `${crate.name}_bg-${generateDigest(content)}.wasm`;
        this.emitFile({
          type: 'asset',
          fileName: `${configAssetsDir}/${wasmFileName}`,
          source: content
        });
      }
    }
  };
}

export default vitePluginWasmPack;

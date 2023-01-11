const path = require('path');
const fs = require('fs');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

let platform = undefined;

// 1) First detect for what platform we are building for (web or native)

// Try to detect platform from the npm run command
// (e.g. `npm run web:open` => platform  is web)
if (process.env.npm_lifecycle_event !== undefined && process.env.npm_lifecycle_event !== 'npx') {

  if (
    process.env.npm_lifecycle_event.startsWith('web:')
    || process.env.npm_lifecycle_event.startsWith('test:')
  ) {
    platform = 'web';
  } else if (process.env.npm_lifecycle_event.startsWith('native:')) {
    platform = 'native';
  } else {
    throw new Error(`Don't know what platform to use for npm lifecycle event \`${process.env.npm_lifecycle_event}\``);
  }

} else {
  // If we are here `vue-cli-service` is run directly, in this case the
  // platform should be passed by a PLATFORM environ variable
  if (process.env.PLATFORM !== undefined) {
    console.log(`PLATFORM environ set to \`${process.env.PLATFORM}\``);
    if (process.env.PLATFORM === 'web') {
      platform = 'web';
    } else if (process.env.PLATFORM === 'native') {
      platform = 'native';
    } else {
      throw new Error('Invalid value for PLATFORM environ variable, accepted values: `web`/`native`');
    }

  } else {
    // Ain't nobody got time to set environ variable !
    console.log('PLATFORM environ variable not set, defaulting to `web`');
    platform = 'web';
  }
}

// 2) Now that we know the platform, we can create `trampoline.ts` accordingly

const libparsecPluginPath = path.join(__dirname, 'src/plugins/libparsec/');
const src = path.join(libparsecPluginPath, `trampoline-${platform}.ts.in`);
const dst = path.join(libparsecPluginPath, 'trampoline.ts');
console.log(`Copy ${src} -> ${dst}`);
fs.copyFileSync(src, dst);

// 3) Finally add the packaging of the Wasm stuff if the platform requires it

if (platform === 'web') {

  // Note WasmPackPlugin correctly infer the Rust profile from the current mode
  // (e.g. js mode `production` -> Rust profile `release`)
  module.exports = {
    configureWebpack: {
      experiments: {
        asyncWebAssembly: true
      },
      plugins: [
        new WasmPackPlugin(
          {
            crateDirectory: path.join(__dirname, '../bindings/web'),
            outDir: path.join(__dirname, '../bindings/web/pkg')
          }
        )
      ]
    }
  };

}

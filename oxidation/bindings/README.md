# LibParsec bindings

## 1. Desktop (electron)

- NPM scripts using LibParsec needs cargo to work : <https://doc.rust-lang.org/cargo/getting-started/installation.html>
- To develop on Windows, you need to install VS BuildTools 2019 :
    - Download the installer here : <https://visualstudio.microsoft.com/fr/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16>
    - Select in the individual modules : *"MSVC v142 - VS 2019 C++ x64/x86 Build Tools"*

Install electron dependencies and build:

```bash
# In /bindings/electron
npm install
npm run build  # Generate index.node (basically a .so that node can load)
```

> *Note: `../client/electron` project automatically does `npm build` and copy `index.node` where needed.*

## 2. Android

Requirements:

- Install Android Studio
- In `File > Settings > Appearance & Behavior > System Settings > Android SDK > SDK Platform > Show Package Details` : Install Android SDK 30.0.3
- In `File > Settings > Appearance & Behavior > System Settings > Android SDK > SDK Tools > Show Package Details` : Install Android NDK 22.1.7171670 and Android SDK Command-line Tools
- Python command should be available in PATH environment variable (used by `rust-android-gradle` plugin)

Install Rust targets for cross compilation:

```bash
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
```

*Note: The `bindings/android` directory is a valid Android project to generate a libparsec AAR.
However the AAR doesn't have to be built when using `../client/android` project (given it depends explicitly on the `bindings/android/libparsec`).*

## 3. iOS (ffi)

`<WIP>`

## 4. Web

Install wasm dependencies and build:

Requirements:

- wasm-pack: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`

Build:

```bash
# In /bindings/web
wasm-pack build
```

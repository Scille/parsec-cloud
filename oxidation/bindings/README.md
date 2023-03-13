# LibParsec bindings

- [LibParsec bindings](#libparsec-bindings)
  - [1. Desktop (electron)](#1-desktop-electron)
  - [2. Android](#2-android)
    - [Requirements](#requirements)
      - [Permission issue](#permission-issue)
      - [Install Android Studio](#install-android-studio)
      - [Install Android tools](#install-android-tools)
      - [Install required rust targets](#install-required-rust-targets)
      - [Install Java development kit to build the application](#install-java-development-kit-to-build-the-application)
    - [Building the Android apps](#building-the-android-apps)
  - [3. iOS (ffi)](#3-ios-ffi)
  - [4. Web](#4-web)

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

### Requirements

You can either install `Android Studio` or `Android tools`

#### Permission issue

If you have permission issue when installing the packages `android sdk` or `android ndk`.
It indicate that you can't write to the `ANDROID_HOME` (or `ANDROID_SDK_HOME` (they should be the same value)).

1. Copy `ANDROID_HOME` to a place in your computer where you have `write` access.
2. Update `ANDROID_HOME` & `ANDROID_SDK_HOME` env variables to the new path.

#### Install Android Studio

1. Download Android studio at <https://developer.android.com/studio#android-studio-downloads>
2. Install Android Studio
3. In `File > Settings > Appearance & Behavior > System Settings > Android SDK > SDK Platform > Show Package Details` :

    - Install Android SDK `30.0.3`
    - Install Android NDK `22.1.7171670`
    - Android SDK Command-line Tools

4. Python command should be available in PATH environment variable (used by `rust-android-gradle` plugin)

#### Install Android tools

1. Download android tools at <https://developer.android.com/studio#command-line-tools-only>
2. Ensure you have `openjdk-8` installed (it is required to run `android-tools`'s commands).
3. Install Android `SDK-30.0.3` and `NDK-22.1.7171670` by running

    ```shell
    JAVA_HOME=/jdk8/path sdkmanager --install "ndk;<ndk version>" "build-tools;<build tool version>"
    ```

#### Install required rust targets

Install Rust targets for cross compilation:

```shell
rustup target add \
  armv7-linux-androideabi \
  i686-linux-android \
  aarch64-linux-android \
  x86_64-linux-android \
```

> The `bindings/android` directory is a valid Android project to generate a libparsec AAR.
> However the AAR doesn't have to be built when using `../client/android` project (given it depends explicitly on the `bindings/android/libparsec`).

#### Install Java development kit to build the application

Having installed `openjdk-8` in the step [Install Android tools](#install-android-tools) wont be enough to build the android apps.

For that we will be using the last LTS of `openjdk`:

- Install the last java LTS development kit `JDK-17`.
- If need to be, update your `JAVA_HOME` to point to the installation path of `jdk-17`

### Building the Android apps

```shell
bash ./gradlew assembleRelease
```

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

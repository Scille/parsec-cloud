# LibParsec bindings

## Desktop (neon)

```
cd bindings/neon/
npm install
npm build  # Generate index.node (basically a .so that node can load)
```

Note `../client/electron` project automatically does `npm build` and copy `index.node` where needed.

## Android

Requirements:

- Install Android studio
- Install Android Ndk 22.1.7171670
- Install Android Sdk 30.0.3 and command line tools
- python command should be available (used by `rust-android-gradle` plugin)

Install Rust targets for cross compilation:

```shell
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add aarch64-linux-android
rustup target add x86_64-linux-android
ndk 23.1.7779620
sdk 30
```

The `bindings/android` directory is a valid Android project to generate a libparsec AAR.

However the AAR doesn't have to be built when using `../client/android` project (given it depends explictly on the `bindings/android/libparsec`).

### iOS (ffi)

`<WIP>`

## Web

`<WIP>`

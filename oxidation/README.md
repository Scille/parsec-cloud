Architecture
------------

- `libparsec/` main project source code
- `libparsec/crates/` subcrates were the code actually resides
- `libparsec/crates/platform_*` platform-specific code (e.g. web, native, android etc.),
  this is use to access platform-specific components (e.g. mountpoint, file storage,
  websocket) and to create an async runtime (given web has it own way of doing this)
- `bindings/` projects to package libparsec for a platform
- `bindings/neon/` package in NodeJS format for desktop
- `bindings/android/` Android project to generate an AAR using Jni API
- `bindings/ffi/` C API, used for iOS
- `bindings/web/` Web project using WASM
- `client/` HTML/Js application containing the GUI and packaging stuff for each platform
- `client/electron/`  Desktop using electron, relies on bindings/neon to access libparsec
- `client/android/` Android using Capacitor, relies on bindings/android to access libparsec
- `client/ios/` iOS using Capacitor, relies on bindings/ffi to access libparsec

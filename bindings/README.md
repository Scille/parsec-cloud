# Libparsec bindings

This directory contains libparsec bindings for different platforms.

- [Bindings generator](#bindings-generator)
- [Platforms](#platforms)
  - [Electron (desktop)](#electron-desktop)
  - [Web](#web)
  - [Android](#android)

## Bindings generator

To avoid errors and the tedious work of writing bindings code, the libparsec bindings are generated
with a Python-based API from the `generator` directory.

See [./generator/README.md](generator/README.md).

## Platforms

### Electron (desktop)

Bindings for the Electron-based desktop application can be found in the `electron` directory.

See [./electron/README.md](electron/README.md).

### Web

Bindings for the web application can be found in the `web` directory.

See [./web/README.md](web/README.md).

### Android

Bindings for the Android application can be found in the `android` directory.

> [!IMPORTANT]
> Android support is out of date and currently not functional.

See [./android/README.md](android/README.md).

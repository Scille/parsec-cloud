# Libparsec Android bindings

This directory contains libparsec bindings for Android.

> [!CAUTION]
> Android support is out of date and currently not functional.
>
> The instructions below are only kept for historical reasons.

## Requirements

You can either install `Android Studio` or `Android tools`

### Install Android Studio

1. Download Android studio at <https://developer.android.com/studio#android-studio-downloads>
2. Install Android Studio
3. In `File > Settings > Appearance & Behavior > System Settings > Android SDK > SDK Platform > Show Package Details` :

    - Install Android SDK `30.0.3`
    - Install Android NDK `23.2.8568313` (see [variable.gradle](../client/android/variables.gradle) for the up to date version to use)
    - Android SDK Command-line Tools

4. Python command should be available in PATH environment variable (used by `rust-android-gradle` plugin)

### Install Android tools

1. Create a writable folder to store `android sdk` tools & packages.

   ```shell
   mkdir ~/.android-sdk
   ```

2. Configure the env variable

   ```shell
   export ANDROID_HOME=~/.android-sdk
   export ANDROID_SDK_ROOT=~/.android-sdk
   ```

3. Download android tools at <https://developer.android.com/studio#command-line-tools-only>
4. Extract the archive to a temporary folder

   ```shell
   # TEMP_DIR=$(mktemp -d) # You can use this command to get a temp folder.
   unzip -d "$TEMP_DIR" <cmdline-tools.zip>
   ```

5. Find the latest version of `cmdline-tools`

   ```shell
   $ $TEMP_DIR/cmdline-tools/bin/sdkmanager --sdk_root=$ANDROID_HOME --list | grep cmdline-tools
     cmdline-tools;1.0    | 1.0          | Android SDK Command-line Tools
     cmdline-tools;2.1    | 2.1          | Android SDK Command-line Tools
     cmdline-tools;3.0    | 3.0          | Android SDK Command-line Tools
     cmdline-tools;4.0    | 4.0          | Android SDK Command-line Tools
     cmdline-tools;5.0    | 5.0          | Android SDK Command-line Tools
     cmdline-tools;6.0    | 6.0          | Android SDK Command-line Tools
     cmdline-tools;7.0    | 7.0          | Android SDK Command-line Tools
     cmdline-tools;8.0    | 8.0          | Android SDK Command-line Tools
     cmdline-tools;9.0    | 9.0          | Android SDK Command-line Tools
     cmdline-tools;latest | 9.0          | Android SDK Command-line Tools (latest)
   ```

6. Install `cmdline-tools`

   ```shell
   $TEMP_DIR/cmdline-tools/bin/sdkmanager --install "cmdline-tools;latest"
   ```

   > Note: using `latest` may not allow reproducible build on different dev env.
   > But you can replace the value by another one (`0.9` for example).

7. Update your `PATH` variable

   ```shell
   export PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin"
   ```

   > Note: the path is `latest/bin` because we install `cmdline-tools` a the tag `latest`.
   > For `9.0`, it will be `9.0/bin` for example.

8. Install Android `SDK-30.0.3` and `NDK-23.2.8568313` (see [variable.gradle](../client/android/variables.gradle) for the up to date version to use) by running

    ```shell
    sdkmanager --install "ndk;<ndk version>" "build-tools;<build tool version>"
    ```

### Install required rust targets

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

### Install Java development kit to build the application

Install the latest LTS of `openjdk`:

- Install the latest java LTS development kit (currently `JDK-17`).
- If need to be, update your `JAVA_HOME` to point to the installed jdk version folder.

## Update the Gradle dependencies locks

To regenerate the lock file use:

```shell
bash ./gradlew dependencies --write-locks
```

If you want to update a specific list of dependencies use:

```shell
bash ./gradlew classes --update-locks <dependencies comma separated>
```

For both command above, if you want to update the dependencies lock for a specific module use: `-p <module-path>`

A more detailed documentation can be found here [Gradle - locking dependencies](https://docs.gradle.org/current/userguide/dependency_locking.html)

## Update Gradle verification metadata

```shell
bash ./gradle --write-verification-metadata sha256 help
```

> It you want to update the verification metadata for a specific module use: `-p <module-path>`

A more detailed documentation can be found here [Gradle - dependency verification](https://docs.gradle.org/current/userguide/dependency_verification.html#sub:enabling-verification)

## Why do we lock dependencies version & comparing dependencies checksum

In most languages pinning dependency version and storing each dependency checksum is a single step (e.g. see `Cargo.lock`, `poetry.lock`), but with Gradle this is divided into two steps:

- Dependency locking consists of pinning the list of dependencies (including transitive ones) used by the project.
  It ensure that the transitive dependencies don't update unexpectedly (outside of PR that bump a dependency version).
- Verification metadata contains the list of dependency checksums (still with transitive ones).
  The dependency metadata (verification metadata) ensure that a dependency isn't tempered by comparing it's checksum.

> When updating a dependency, you will need to update the lock & metadata

## Building the Android apps

```shell
bash ./gradlew assembleRelease
```

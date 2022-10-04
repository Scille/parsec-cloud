MacOS installer for Parsec
==========================

Foreword
--------

To release an app on MacOS, even without dealing with the App Store, we must go through a few steps for the app to run on a user's Mac. Otherwise, the user might encounter warning messages while trying to open the app, or the app won't even launch at all because of Gatekeeper: https://en.wikipedia.org/wiki/Gatekeeper_(macOS)

1. __Build the app__: We use PyInstaller to compile Python code into an `.app` bundle.
2. __Sign the app__: Using an active Apple Developer account, we sign our code and its dependencies with a custom certificate created using the account.
3. __Notarize the app__: Notarizing the app is equivalent to asking Apple to approve it for distribution. Since MacOS 10.14.5, all applications downloaded outside of the App Store must be notarized to run. See https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution
4. __Staple the app__: An app notarization will be checked with an online request when a user tries to execute it. Stapling "attaches" the notarization to the app bundle itself, so that this check can be performed offline.
5. __Bundle into .dmg__: A nice format for distribution in MacOS is a `.dmg` bundle, which compresses the app and allows the user to simply drag and drop the app inside their `/Applications` folder.

Build steps
-----------

### 1 - Use a compatible Python version

The app building is done with PyInstaller. Most versions of Python from 3.9 onwards will work, though some raises errors upon opening the app. If such an problem occurs, an easy solution is to switch to a working Python version with `pyenv`, such as 3.9.0.
Be mindful that using `pyenv`, the python version must be installed with the `--enable-framework` option:
```shell
$ PYTHON_CONFIGURE_OPTS="--enable-framework" pyenv install 3.9.0
```


### 2 - Build the application

To make the .app:

```shell
$ sh macos_pyinstaller.sh
```

If a version is already built, you can use the `-f` or `--force` option to bypass the confirmation dialogs when building the new one.
You can also use the `-i` or `--install` option to directly install the application in the local `/Applications` folder after building.


### 3 - Sign the application

```shell
$ codesign --deep --force -s <identity> --timestamp -v --entitlements entitlements.plist -o runtime dist/parsec.app
```

`identity` being the signing certificate, to be downloaded with proper access here : https://developer.apple.com/account/resources/certificates/list
From PyInstaller versions >=4.4, the bundle will already be signed. To avoid notarization issues with this signature, the `--force` option is used to override it.

You can check the codesign using:
```shell
$ codesign -vvv --deep --strict dist/parsec.app
```
The next step can only be successful if this check is.


### 4 - Zip and Notarize the app

A version of Xcode must be installed locally to use the following `xcrun altool` commands.

Going through this process for the first time, an application-specific password has to be generated here:
https://appleid.apple.com/account/manage
And then be stored in the keychain:
```shell
$ xcrun altool --store-password-in-keychain-item "AC_PASSWORD" -u "your-apple-id" -p "your-password"
```

The Apple ID is usually the email address used for the Apple account.

The app now has to be zipped to then be notarized using the ID and password:

```shell
$ ditto -c -k --keepParent "dist/parsec.app" dist/parsec.zip
$ xcrun altool --notarize-app -t osx -f dist/parsec.zip \
    --primary-bundle-id com.scille.parsec -u "your-apple-id" --password "@keychain:AC_PASSWORD"
```

If you encounter issues using the keychain, you can skip the corresponding step and directly input the generated password in the last command.

While notarizing, you can check the progress using this command, using the `RequestUUID` received after uploading for notarization:

```shell
$ xcrun altool --notarization-info <RequestUUID> -u "your-apple-id" --password "your-password"
```


### 5 - Staple the app

Once the notarization is done, you should have received a confirmation email.

We can now staple the app:

```shell
$ xcrun stapler staple -v "dist/parsec.app"
```

Once stapled, we need to re-zip the app to distribute the stapled version:

```shell
$ rm dist/parsec.zip
$ ditto -c -k --sequesterRsrc --keepParent "dist/parsec.app" dist/parsec.zip
```

#### Check with Gatekeeper

Steps 4 and 5 exist to comply with Gatekeeper.
We can check if both steps were successful by asking Gatekeeper directly:

```shell
$ spctl --assess --type execute -vvv "dist/parsec.app"
```


### 6 - Bundle into .dmg

The app is now ready to be bundled for distribution. To this end, we create a drag & drop `.dmg` folder.
A `.dmg` background is needed at this point, many neutral backgrounds can be found with a quick search, until we eventually design our own.

#### Create the .dmg

To create the `.dmg`, open `Disk Utility`, then follow *File > New Image > Blank Image*, then only modify the *Size* to be enough so that Parsec and the background image can fit.

Once done, mount the Disk Image, and put the background image in a `.background` folder at its root. The Disk Image should be mounted in `/Volumes` by default.

Then, copy the previously codesigned, notarized and stapled app in the same mounted volume, along with an alias to `/Applications`.

In *View > Show View Options*, restrict the mounted volume to `Always open in icon view` and `Browse in icon view` to force the right format upon opening the `.dmg`. In the same window the icon sizes can be adjusted to match the background.

Drag and arrange the icons to match the background, then eject the Disk Image.

#### Compress the .dmg

The `.dmg` can now be compressed and set as read-only.

Open `Disk Utility` again, then *Images > Convert*, select the `.dmg` and compress it with the name that will be used for distribution, in our case `parsec-{version}-macos-amd64.dmg`.

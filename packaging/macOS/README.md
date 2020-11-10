MacOS installer for Parsec
==========================

Build steps
-----------

### 1 - Use a compatible Python version

The app building is done with PyInstaller. For it to work properly, you must use a version of Python anterior to 3.8, otherwise the app won't launch at all. Any 3.6.x or 3.7.x version should work fine.


### 2 - Build the application

To make the .app:

```shell
sh macos_pyinstaller.sh
```

If a version is already built, you can use the `-f` or `--force` option to bypass the confirmation dialogs when building the new one.
You can also use the `-i` or `--install` option to directly install the application in the local `/Applications` folder after building.


### 3 - Sign the application

```shell
codesign --deep -s <identity> --timestamp -v --entitlements entitlements.plist -o runtime dist/parsec.app
```

`identity` being the signing certificate, to be downloaded with proper access here : https://developer.apple.com/account/resources/certificates/list

You can check the codesign using:
```shell
codesign -vvv --deep --strict dist/parsec.app
```
The next step can only be successful if this check is.


### 4 - Zip and Notarize the app

The Apple ID to be used is usually the email address used for the Apple account, and an application-specific password for this script has to be generated here : https://appleid.apple.com/account/manage

It can then be stored in the keychain:
```shell
xcrun altool --store-password-in-keychain-item "AC_PASSWORD" -u "your-apple-id" -p "your-password"
```

The app now has to be .zipped to then be notarized using the ID and password:

```shell
ditto -c -k --keepParent "dist/parsec.app" dist/parsec.zip
xcrun altool --notarize-app -t osx -f dist/parsec.zip \
    --primary-bundle-id com.scille.parsec -u "your-apple-id" --password "@keychain:AC_PASSWORD"
```

If you encounter issues using the keychain, you can skip the corresponding step and directly input the generated password in the last command.

While notarizing, you can check the progress using this command, using the `RequestUUID` received after uploading for notarization:

```shell
xcrun altool --notarization-info <RequestUUID> -u "your-apple-id" --password "your-password"
```


### 5 - Staple the app

Once the notarization is done, you should have received a confirmation email.

The notarization will be checked online by the user's Mac, and for it to work offline, we can staple the app, which will "attach" the notarization ticket to it.

```shell
xcrun staple stapler "dist/parsec.app"
```

Once stapled, we need to re-zip the app to distribute the stapled version:

```shell
rm dist/parsec.zip
ditto -c -k --sequesterRsrc --keepParent "dist/parsec.app" dist/parsec.zip
```

### 6 - Check with Gatekeeper

Steps 4 and 5 exist to comply with Gatekeeper ( https://en.wikipedia.org/wiki/Gatekeeper_(macOS) )
We can verify if steps 4 and 5 were successful by asking Gatekeeper directly:

```shell
spctl --assess --type execute -vvv "dist/parsec.app"
```

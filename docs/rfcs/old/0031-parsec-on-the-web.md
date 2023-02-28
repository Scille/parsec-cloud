# Parsec on the web

From [ISSUE-2546](https://github.com/Scille/parsec-cloud/issues/2546)

## Parsec Web

With the Rust rewrite of core and the new shinny VueJS GUI, Parsec will soon be able to run on the web \o/

Here we will have a look at the specificity of the Web platform and what evolution can web make to accommodate them.

### 1) Virtual drive mountpoint

#### The issue

For obvious security reason, there is no Web API to allow a random website to mount a virtual drive on the user's machine. This is also true for browser extension.

Virtual drive is the main way to interact with data stored in Parsec when using the desktop and mobile clients.

On web this is replaced by two approaches:

1) the Parsec app embed code that knows how to display/edit the most common file formats
2) for other file formats, user has to download the file so that he can open it with a software install on his computer

This implies:

- In `2)`, if the file has been modified it must be uploaded manually by the user
- In `2)`, the file ends up in a clear text form on the user's computer. This is not good for security, especially given the user must remove the file manually which is cumbersome and they have no real incentive for doing it.
- `1)` is obviously much more convenient than `2)` for the user ;-)
- `1)` is easy enough to display PDF and images
- we are working hard for `1)` to support display and edition of Office files ^^

#### Evolutions

So the virtual drive is a must for any advanced usage, however being able to display and edit the most common files type directly from the Parsec app is a good thing. And this true even on the non web version of Parsec:

- on Android, opening a file with an external application often triggers export and sync of the file (e.g. when opening a docx file with the Google suite, Google Drive kicks in and wants to save the file no matter what...)
- support of Office files within Parsec allows us to add support for collaborative edition \o/

### 2) App distribution & update model

From the point of view of the data exchanged with the outside world, a application lifecycle can be break down into two parts:

1) application installation/update
2) application update

| Type                             | Example                              | installation/update | usage         |
| -------------------------------- | ------------------------------------ | ------------------- | ------------- |
| Offline native app with OS store | LibreOffice on Ubuntu                | OS store            | n/a           |
| Offline native app               | LibreOffice on Windows               | libreoffice.org     | n/a           |
| Web app                          | Google doc                           | google.com          | google.com    |
| Web app as browser extension     | Bitwarden (password manager)         | mozilla.org         | bitwarden.com |
| Hybrid app single peer           | Steam (gaming native app) on Windows | steam.com           | steam.com     |
| Hybrid app any peer              | Mumble (VoIP native app) on Windows  | mumble.info         | any domain !  |

What we can see is a native app comes with a strong separation between the installation/update step and the usage one. On the contrary a website is in charge of both sending the web app the client should run *and* handle application logic&data later on.

This is not a big deal in a traditional scenario:

- The client's browser has a solid sandbox so a compromised server cannot harm it
- The server has clear text access to all the application data, so an attacker doesn't need to send malicious code to retrieve those data[^1]

[^1]: We don't consider [XSS attacks](https://en.wikipedia.org/wiki/Cross-site_scripting) here

#### The issue with distributing the application

However things are different for a Zero-trust application such as Parsec:

- The client application is responsible to ensure Zero-trust
- Hence corrupting the source providing the client application can breaks Zero-trust

Of course any application source can be corrupted, so the idea is more "how complicated" does it gets:

For a native app, the attacker has to impersonated the trusted source (which is getting harder now that people install application from stores instead of downloading them from random websites). Alternatively the attacker has to get access to the app store account of the developer.

In either case, the attack is visible:

- the corrupted application must be made broadly available (typically any user of Parsec will get the corrupted update)
- it's hard for attacker to hide it attack (it has to removing application from the app store and uninstall it from all computer), which makes post-mortem analysis much easier
- the attacker has to find a way to exfiltrate data from the application

On the opposite, a web application allows much stealthier attacks:

- The server can serve different code to different clients (e.g. depending on they IP address) to only attack a specific target
- The corrupted code is kept in the client browser cache for a short amount of time
- The exfiltration vector is trivial: just send back data to the server (they can even be disguised as legitimate encrypted data to make things even harder to detect !)
- Hacking the application developer is no longer required: only having access to the server serving the application is enough.
- The attacker can be the administrator of the server itself!

The last point is very important, considering an actor hosting a Parsec server that ends up in a legal obligation to break Zero-trust:

- With the users using native app there is no way to break Zero-trust (appart from putting the application developer in legal obligation too, hence creating a backdoor that is publicly visible)
- With the users using a web app, it is possible to serve a modified version of the Parsec client that returns the users private keys to the server.

#### So is Parsec web application completely broken (and hence useless) ?

As we saw, a web application reduces security compared to a native one. However that can be acceptable tradeoff for certain usecases:

1) Showcasing Parsec. Providing Parsec as a Web application makes it much simpler for user to play with it and discover the product.
2) In corporate environment. In this case there is no distrust between users and administrators of the Parsec server given both are part of the company. On top of that Big company administrators are responsible for managing the computers and hence have all the latitude to install keylogger and spywares. In this context, Parsec is part of a "defense in depth" strategy to protect company resources and easing the Zero-trust guarantee can be acceptable (see for instance the new sequestered organization optional feature that allow the company to recover encrypted data).
3) In small business environment. Small business don't have resources for advanced administration/hosting/user formation so thing should be kept simple; on the other hand they main threat model is cryptolockers and data leaks (typically a computer with they financial information in clear gets stolen). In this case trusting a 3rd party such as Parsec SAAS can be considered ok.

In other words, using Parsec as a web application depends of the sensitivity of the exchanged data and the trust in the hosting party ¯\\_(ツ)_/¯

Last but not least, web application can also be distributed as web browser extension. In this case the distribution model is similar to regular native application (the application is provided by the developer, and available publicly on the mozilla/chrome extension store). Of course the downside of this approach is the user has to install an extension which is cumbersome compared to just browsing a website.

#### A case study: Bitwarden

The [Bitwarden password manager](https://bitwarden.com/) is similar to Parsec in many ways:

- It is open source and very cool ;-)
- It is a [Zero-trust end-to-end solution](https://bitwarden.com/images/resources/security-white-paper-download.pdf/) (all data are encrypted client side with a symmetrical key itself protected by a key derivated from the user's password)
- It support a [broad number of platform](https://bitwarden.com/download/): Native, mobile, web and web extension.

[There document](https://bitwarden.com/help/what-encryption-is-used/) is also very good [at explaining](https://bitwarden.com/crypto.html) what they do !

An interesting point in Bitwarden is it uses a single authentication to access both the user data and authenticate (the user's password is derivated in two different way to create a secret shared with the server for authentication and a never shared secret used to decrypt the data encryption key).

### 3) Parsec Device

#### One Parsec device per machine approach

In Parsec there is a separation between a user and its devices: each different machine the user wants to access a Parsec organization from has its own signing key (and on the opposite the decryption keys are shared between all the different machines as it constitutes the entry point to what the user can access).

The benefit of this approach is to be able to precisely identify which device has been used to add information to the system (given manifest/certificates/etc. must be signed by a device to be considered valid).

The corollary of this approach is each new machine must be enrolled by an existing one.
From a security point of view this is a good thing given it works similarly as two-factory-authentication.
On the other hand it makes creating a new device more complex.

Things are getting worse here when considering Parsec web:

- Local storage for a web page has poor to none durability. Hence it doesn't seem possible to store important data such as device key file only here.
- User may connect from the same machine with different browsers (which don't share their local storage). This further increases the amount of device enrollments and might get the user lost if they don't remember which browser has been used to create their previous device.

#### Device info stored locally

All the private information a device needs (so both device signing key and user decryption keys) are only stored locally in a "device key file" (DKF).

The reasons for this are twofold:

- Allow the use of Parsec offline: if the device key file is stored server side it would be impossible to login with no server access
- Avoid the "all egg in one basket" risk where all device key file would be stored server side encrypted with the user's password. The main threat being against brute force attack in case the user choose a weak password.

However this approach has a big downside: given each machine has its own independent device key file, they all have their own independent way of securing it.
Hence changing the password protecting a device key file on one machine doesn't do anything on another machine. This is counter intuitive compared to how most applications work :/

#### Evolutions of the Device Key File (DKF)

It appears clear that Parsec web requires storing the DKF server side.

On top of that the two reasons for device stored locally can be overcome:

1) Forcing user to use strong password is a good practice anyway
2) The DKF can be kept in a local cache after the first login

Of course power users will want to keep security at its highest,  so we shouldn't make DKF server side storage the only choice. "Luckily" we have no choice but supporting the existing local-only DKF approach for backward compatibility ;-)

A way to implement both only and local DKF:

![evolve-login-online-dkf drawio](https://user-images.githubusercontent.com/3187637/177140042-a1aae8d1-5dac-40a8-ad7a-e0fa8895e0a2.svg)

- A part of the login page allows to login from a server. On top of that an "advanced" button is present to display additional config (i.e. the address of the server to use).
- All existing device key files are listed on the login page

When selecting an existing login or filling a new login, the following dialogue appears:

![evolve-login-confirm drawio](https://user-images.githubusercontent.com/3187637/177140452-63f81e79-ee96-459b-830a-cc24b0da58bf.svg)

This dialogue should typically also allows displaying advanced information about the login (typically server address, if the key file is stored in the server on only in local)
From this dialogue, a configuration button should be present to change the login protection (switching to/from smartcard protection, uploading the key file to the server, remove the device key file from the machine etc.)

With this approach, the device key file format should be updated to add a `stored_in_server` boolean flag. This flag would allow to store the cache on online-stored DKF as regular local-only-stored DKF.

On top of that the organization configuration (so also `organization_config` API) should be updated to add a boolean indicating if online-stored DKF is allowed or not.

#### Retrieving the DKF from the server

We should probably do something similar to [what is done by Bitwarden](https://bitwarden.com/images/resources/security-white-paper-download.pdf):

![image](https://user-images.githubusercontent.com/3187637/177143794-a787540d-a774-43af-ac9b-f254f775507a.png)

The idea is:

- first have to authenticate to the server to retrieve the encrypted DKF
- then decrypt locally the DKF

This way an attacker cannot just download the encrypted DKF and try to bruteforce it

#### The recovery device

When device are stored online, recovery is much easier: only the key encrypting the DKF should be exported.
I guess the GUI should contain the two exports approaches (exporting a device file + the recover key or just the recovery key) depending on if the DKF is stored online or not.

### 4) Parsec SAAS authentication

#### The issue with the SAAS authentication

When hosting Parsec on the SAAS, there are two type of authentication:

- Parsec authentication: this is the regular one to decrypt the device file key locally present on the machine and start encrypted data
- SAAS authentication: this is only possible for the user that created the Parsec organization in the SAAS website. It is used to manage the SAAS subscription (create new organization, see organization data usage, setup payment, get invoices etc.).

The key point here is those two authentication have nothing in common from a technical point of view but appear very similar from a user point of view (having two different kind of account for the same service is pretty weird).

Worst: it is likely the user mixes his passwords and send to the SAAS authentication his password protecting his device key file!
Right now the SAAS website is not able to do anything with this password given the device key file is only stored locally, but 1) it is nevertheless not great for security and 2) Parsec web will most likely force us to store the device key file server side :(

#### Evolutions with the SAAS authentication

A good improvement here would be to unify both Parsec and SAAS authentications.
However given Parsec authentication is only possible in the Parsec client (given it is in fact decryption of a device key file), unifying authentication means we would only be able to do SAAS authentication in the Parsec client (but with the Parsec web client this is pretty transparent from the user point of view \o/).

This is currently doable with the Qt5-based GUI, but should be much easier with web-based GUI:

![Parsec unified SAAS authentication](https://user-images.githubusercontent.com/3187637/177138142-1d9e6159-34c0-45be-8b3b-75d57d46487e.svg)

1. User does Parsec authentication on the Parsec client, the device signing key is now available. The Parsec client detects the current organization is hosted on the SAAS and the user is owner of the organization, it then provide a "Manage subscription" button to access the SAAS website from with the client.
2. When this button is clicked, the Parsec client uses the device signing key to sign an authentication token (typically json containing organizationid/userid/timestamp).
3. The Parsec client then creates an IFrame targeting the SAAS website[^2] and pass the authentication token a part of the URL[^3].
4. The SAAS website transfer the authentication token to the Parsec server which will check it signature and return the content of the payload
5. The SAAS website can then reply to the webpage with a session cookie
6. In case the SAAS website starts refusing the session cookie (e.g. it has become too old), the webpage uses the [Window.postMessage()](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage) API to notify the Parsec client. In such case the Parsec client just has to restart operation from step 2 to solve the issue.

[^2]: At this point, the SAAS webpage can use the [Window.opener](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener) API to detect it lives in an IFrame, and otherwise displays an error message inviting the user to access the page from within the parsec client
[^3]: Passing a signed message in the url is probably a bad idea given the server log will display them, we should instead pass it as a header or use a dedicate auth route. However this is slightly more complex to explain in the schema: the signed message should be in the private (i.e. stuff after `#`) of the url, then it's the javascript of the page that send it to the server.

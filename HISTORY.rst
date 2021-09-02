History
=======


.. towncrier release notes start


Parsec v2.5.0 (2021-09-02)
--------------------------

Bugfixes
~~~~~~~~

* Fixed a bug on MacOS where the window would freeze after the invitation
  process  (`#1786 <https://github.com/Scille/parsec-cloud/issues/1786>`__)
* Made the QR code easier to read by removing the logo and changing its color
  (`#1787 <https://github.com/Scille/parsec-cloud/issues/1787>`__)
* Generate the proper error when creating a file with a name larger than 255
  bytes on linux  (`#1813 <https://github.com/Scille/parsec-
  cloud/issues/1813>`__)
* Fix file opening on Windows and MacOS (`#1822
  <https://github.com/Scille/parsec-cloud/issues/1822>`__)

Client/Backend API evolutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Add active user limit configurable on a per-organization basis. Also add
  --organization-initial-user-profile-outsider-allowed and --organization-
  initial-active-users-limit options in `backend run` command.  (`#1766
  <https://github.com/Scille/parsec-cloud/issues/1766>`__)
* Remove most parts of APIv1 (only `organization_bootsrap` command is kept from
  APIv1 for backward compatibility). Remove `expiration_date` from
  `organization_config` command. Introduce the administration REST api to create
  & get informations on organizations.  (`#1810
  <https://github.com/Scille/parsec-cloud/issues/1810>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Images from email invitations are now hosted directly on the Parsec server
  instead of relying on parsec.cloud website. (`#1780
  <https://github.com/Scille/parsec-cloud/issues/1780>`__)
* Change Parsec server license to Business Source License 1.1 (BSLv1.1).
  (`#1785 <https://github.com/Scille/parsec-cloud/issues/1785>`__)
* Improve claim/greet dialog in GUI when invitation is deleted.  (`#1806
  <https://github.com/Scille/parsec-cloud/issues/1806>`__)
* Improve the file size formatting by displaying for significant figures when
  needed.  (`#1808 <https://github.com/Scille/parsec-cloud/issues/1808>`__)
* Improve error reports sent by telemetry and CLI arguments documentation.
  (`#1823 <https://github.com/Scille/parsec-cloud/issues/1823>`__)


Parsec v2.4.2 (2021-07-06)
--------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Made the macFUSE pop-up during MacOS installation more user-friendly  (`#1777
  <https://github.com/Scille/parsec-cloud/issues/1777>`__)


Parsec v2.4.1 (2021-06-29)
--------------------------

Bugfixes
~~~~~~~~

* Fix database migration script n°6.  (`#1774 <https://github.com/Scille/parsec-
  cloud/issues/1774>`__)


Parsec v2.4.0 (2021-06-29)
--------------------------

Features
~~~~~~~~

* Adds the outsider profile management in the GUI  (`#1720
  <https://github.com/Scille/parsec-cloud/issues/1720>`__)
* Add QR code on device invitation (`#1652 <https://github.com/Scille/parsec-
  cloud/issues/1652>`__)
* Introduce OUTSIDER organization user profile: an outsider cannot see the
  identity of other users within the organization. On top of that it is only
  allowed to be READER/CONTRIBUTOR on shared workspaces.  (`#1727
  <https://github.com/Scille/parsec-cloud/issues/1727>`__)
* Add `.sb-` temporary directories to the confined pattern list. Those
  directories appear on MacOS when editing `.doc` and `.docx` files.  (`#1764
  <https://github.com/Scille/parsec-cloud/issues/1764>`__)

Bugfixes
~~~~~~~~

* Added the pop-up widget to download latest app version on MacOS  (`#1736
  <https://github.com/Scille/parsec-cloud/issues/1736>`__)
* Fix some alignments issues with the workspace widgets.  (`#1761
  <https://github.com/Scille/parsec-cloud/issues/1761>`__)
* Fix error handling for drag&drop in GUI. (`#1732
  <https://github.com/Scille/parsec-cloud/issues/1732>`__)
* Fix possible crash when sync occurs right after a workspace reencryption.
  (`#1730 <https://github.com/Scille/parsec-cloud/issues/1730>`__)

Deprecations and Removals
~~~~~~~~~~~~~~~~~~~~~~~~~

* Change the file link URL format so that file path is encrypted. This change
  breaks compatibility with previous file url format.  (`#1637
  <https://github.com/Scille/parsec-cloud/issues/1637>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Server on-organization-bootstrap webhook now allow 2xx return status instead
  of only 200.  (`#1750 <https://github.com/Scille/parsec-cloud/issues/1750>`__)
* Add red color to remove widget dialogue confirmation button in GUI.  (`#1758
  <https://github.com/Scille/parsec-cloud/issues/1758>`__)
* Reword telemetry related dialogue in GUI. (`#1759
  <https://github.com/Scille/parsec-cloud/issues/1759>`__)


Parsec v2.3.1 (2021-05-10)
--------------------------

Bugfixes
~~~~~~~~

* Fix blocking calls related to the local storage that might slow down the
  application.  (`#1713 <https://github.com/Scille/parsec-cloud/issues/1713>`__)
* Fix a regression that broke the "Remount workspace at a given timestamp"
  button.  (`#1723 <https://github.com/Scille/parsec-cloud/issues/1723>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Update recommanded macFUSE version to 4.1.0 for mountpoint on macOS.  (`#1718
  <https://github.com/Scille/parsec-cloud/issues/1718>`__)


Parsec v2.3.0 (2021-05-04)
--------------------------

Features
~~~~~~~~

* Allow read access to a workspace during a re-encryption.  (`#1650
  <https://github.com/Scille/parsec-cloud/issues/1650>`__)

Bugfixes
~~~~~~~~

* Fixed Dock icon behaviour on MacOS when app was closed with red X.  (`#1519
  <https://github.com/Scille/parsec-cloud/issues/1519>`__)
* Fix the server's stucking while it waits for a peer.  (`#1625
  <https://github.com/Scille/parsec-cloud/issues/1625>`__)
* Added filename normalization to fix conflicts on special characters on MacOS.
  (`#1645 <https://github.com/Scille/parsec-cloud/issues/1645>`__)
* Fix confusing dialog when logging out with an on-going reencryption.  (`#1663
  <https://github.com/Scille/parsec-cloud/issues/1663>`__)
* Fix some blinking with the workspace buttons, especially while doing a
  reencryption.  (`#1665 <https://github.com/Scille/parsec-
  cloud/issues/1665>`__)
* Enforce NFC string normalization for organization/device/user/entry id and
  human handle.  (`#1708 <https://github.com/Scille/parsec-
  cloud/issues/1708>`__)
* Fix an issue with fuse mounpoints on linux where the shutdown procedure might
  block forever  (`#1716 <https://github.com/Scille/parsec-
  cloud/issues/1716>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Update CLI command `parsec core bootstrap_organization` to accept params for
  human/device label/email.  (`#1674 <https://github.com/Scille/parsec-
  cloud/issues/1674>`__)
* Improve synchronization performance by running the block uploads in parallel
  (`#1678 <https://github.com/Scille/parsec-cloud/issues/1678>`__)
* Improve Windows installer for smaller size and faster install time. Also fix
  uninstall when previous version has been installed in a custom location.
  (`#1690 <https://github.com/Scille/parsec-cloud/issues/1690>`__)


Parsec v2.2.4 (2021-03-18)
--------------------------

Features
~~~~~~~~

* Made password validation stronger in the GUI (`#1601
  <https://github.com/Scille/parsec-cloud/issues/1601>`__)
* Added MacOS Big Sur compatibility  (`#1640 <https://github.com/Scille/parsec-
  cloud/issues/1640>`__)

Bugfixes
~~~~~~~~

* Fix server event dispatching when a PostgreSQL database connection terminates
  unexpectedly.  (`#1634 <https://github.com/Scille/parsec-
  cloud/issues/1634>`__)
* Fix unhandled exception in GUI when offline and workspace author UserInfo is
  not in cache. Fix view on inconstent files in GUI. (`#1641
  <https://github.com/Scille/parsec-cloud/issues/1641>`__)
* Fixed a mountpoint issue in MacOS that could cause errors during login or
  unmounting a workspace.  (`#1644 <https://github.com/Scille/parsec-
  cloud/issues/1644>`__)
* Fixed style issues on dark mode MacOS (`#1646
  <https://github.com/Scille/parsec-cloud/issues/1646>`__)
* Fix issue where workspace preview does not update when changes are made while
  on maintenance.  (`#1658 <https://github.com/Scille/parsec-
  cloud/issues/1658>`__)

Deprecations and Removals
~~~~~~~~~~~~~~~~~~~~~~~~~

* Remove massively unused `--log-filter` option from `core gui` and `backend
  run` commands. (`#1639 <https://github.com/Scille/parsec-
  cloud/issues/1639>`__)

Client/Backend API evolutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Bump api version to 1.3; Add the number of workspaces in the organization
  stats  (`#1655 <https://github.com/Scille/parsec-cloud/issues/1655>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Fix backend server infinite wait on HTTP-invalid incoming request.  (`#1611
  <https://github.com/Scille/parsec-cloud/issues/1611>`__)
* Disable logging to file by default when running the GUI client.  (`#1638
  <https://github.com/Scille/parsec-cloud/issues/1638>`__)


Parsec v2.2.3 (2021-01-29)
--------------------------

Features
--------

* Added MacOS version for release

Bugfixes
~~~~~~~~

* Improved workspace loading performance (less query for reencryption) (`#1619
  <https://github.com/Scille/parsec-cloud/issues/1619>`__)


Parsec v2.2.2 (2020-12-15)
--------------------------

No significant changes.


Parsec v2.2.1 (2020-12-15)
--------------------------

Features
--------

* Improve backend HTTP welcome page, we no longer use html like it's 1997
  (`#1603 <https://github.com/Scille/parsec-cloud/issues/1603>`__)

Bugfixes
~~~~~~~~

* Fix unhandled error on linux/macOS when logout occures during mountpoint
  processing. (`#1607 <https://github.com/Scille/parsec-cloud/issues/1607>`__)


Parsec v2.2.0 (2020-12-14)
--------------------------

Features
~~~~~~~~

* Added email in workspace sharing dialog  (`#1514
  <https://github.com/Scille/parsec-cloud/issues/1514>`__)
* Reworked the dialog to see a workspace as it was to make it a little bit
  sexier  (`#1512 <https://github.com/Scille/parsec-cloud/issues/1512>`__)
* Allow copy/cut/paste files from different workspaces.  (`#1183
  <https://github.com/Scille/parsec-cloud/issues/1183>`__)
* Backend can now force https redirection (see `--forward-proto-enforce-https`
  parameter).  (`#1466 <https://github.com/Scille/parsec-cloud/issues/1466>`__)
* Add a spinner when opening a folder in the gui  (`#1442
  <https://github.com/Scille/parsec-cloud/issues/1442>`__)
* Add macOS compatibility  (`#1441 <https://github.com/Scille/parsec-
  cloud/issues/1441>`__)
* Inviting a user already member of an organization is no longer allowed by the
  backend server (`#1332 <https://github.com/Scille/parsec-
  cloud/issues/1332>`__)
* Add widget to import and export keys  (`#1520
  <https://github.com/Scille/parsec-cloud/issues/1520>`__)
* Added a warning message when a user choses their password (`#525
  <https://github.com/Scille/parsec-cloud/issues/525>`__)

Bugfixes
~~~~~~~~

* Fix the go back in time for workspace.  (`#1568
  <https://github.com/Scille/parsec-cloud/issues/1568>`__)
* Made copy and cut of files asynchronous in the GUI  (`#1560
  <https://github.com/Scille/parsec-cloud/issues/1560>`__)
* Cleaned choices when creating an organization in the GUI (`#1596
  <https://github.com/Scille/parsec-cloud/issues/1596>`__)
* Mount workspace if needed when a file link is clicked  (`#1531
  <https://github.com/Scille/parsec-cloud/issues/1531>`__)
* Displays an error message when failing to open a file  (`#1525
  <https://github.com/Scille/parsec-cloud/issues/1525>`__)
* Fix an error when opening a workspace in the file explorer  (`#1541
  <https://github.com/Scille/parsec-cloud/issues/1541>`__)
* Fixed overflow error in loading dialog (`#1543
  <https://github.com/Scille/parsec-cloud/issues/1543>`__)
* Fix uncatched error in GUI when bootstrapping organization with an invalid url
  (`#1593 <https://github.com/Scille/parsec-cloud/issues/1593>`__)
* Improved GUI style on MacOS  (`#1447 <https://github.com/Scille/parsec-
  cloud/issues/1447>`__)
* Trim the user name  (`#1544 <https://github.com/Scille/parsec-
  cloud/issues/1544>`__)
* Improved import error messages  (`#1491 <https://github.com/Scille/parsec-
  cloud/issues/1491>`__)
* Display a correct error message if the time on the machine is not correctly
  set when creating a new org  (`#1475 <https://github.com/Scille/parsec-
  cloud/issues/1475>`__)
* Clear workspace list when spinner is displayed  (`#1515
  <https://github.com/Scille/parsec-cloud/issues/1515>`__)
* Fixed crash on MacOS when closing a dialog  (`#1538
  <https://github.com/Scille/parsec-cloud/issues/1538>`__)
* Improved error message when trying to mount a workspace with no drives
  available on Windows (`#1542 <https://github.com/Scille/parsec-
  cloud/issues/1542>`__)
* Fix synchronization potentially not triggered after a file resize  (`#1579
  <https://github.com/Scille/parsec-cloud/issues/1579>`__)
* Hide return button on login screen when there's only one device  (`#1505
  <https://github.com/Scille/parsec-cloud/issues/1505>`__)

Client/Backend API evolutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Fix incorrect definitions of entry name type for workspace and folder
  manifests in api.  (`#1571 <https://github.com/Scille/parsec-
  cloud/issues/1571>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Log exceptions occuring in Qt slots  (`#1520
  <https://github.com/Scille/parsec-cloud/issues/1520>`__)
* Moved password change location in the same menu as the logout button (`#621
  <https://github.com/Scille/parsec-cloud/issues/621>`__)
* Make OSXFUSE download link clickable in GUI  (`#1585
  <https://github.com/Scille/parsec-cloud/issues/1585>`__)
* Add support for macOS  (`#1572 <https://github.com/Scille/parsec-
  cloud/issues/1572>`__)


Parsec v2.1.0 (2020-10-08)
--------------------------

Features
~~~~~~~~

* Ask directly for password if only one device is registered on the machine
  (`#1456 <https://github.com/Scille/parsec-cloud/issues/1456>`__)
* Better display for temporary workspaces  (`#1463
  <https://github.com/Scille/parsec-cloud/issues/1463>`__)
* Show a spinner while workspaces are loaded  (`#1432
  <https://github.com/Scille/parsec-cloud/issues/1432>`__)
* Add feature to display shared workspaces between two users  (`#1454
  <https://github.com/Scille/parsec-cloud/issues/1454>`__)
* Better display when user role on a workspace has been changed  (`#1418
  <https://github.com/Scille/parsec-cloud/issues/1418>`__)
* Adding Users Pagination for GUI.  (`#1452 <https://github.com/Scille/parsec-
  cloud/issues/1452>`__)
* Better display of workspace reencryption  (`#1423
  <https://github.com/Scille/parsec-cloud/issues/1423>`__)
* Display login and follow link on not logged organization file link click.
  (`#1405 <https://github.com/Scille/parsec-cloud/issues/1405>`__)
* Display the volume of an organization to admins  (`#1487
  <https://github.com/Scille/parsec-cloud/issues/1487>`__)
* Better indictation of the role of a user on a workspace  (`#1478
  <https://github.com/Scille/parsec-cloud/issues/1478>`__)
* Remember the previous position and size of the window  (`#1486
  <https://github.com/Scille/parsec-cloud/issues/1486>`__)
* Add parsec core cli envvar support  (`#1473 <https://github.com/Scille/parsec-
  cloud/issues/1473>`__)
* Display server address in user info tooltip  (`#1474
  <https://github.com/Scille/parsec-cloud/issues/1474>`__)

Bugfixes
~~~~~~~~

* Fix the reporting of exceptions with very long traces from the backend
  connection module.  (`#1340 <https://github.com/Scille/parsec-
  cloud/issues/1340>`__)
* Fix batch size in workspace reencryption leading to very slow operation.
  (`#1431 <https://github.com/Scille/parsec-cloud/issues/1431>`__)
* Fix a possible deadlock when cancelling the mounting of a workspace on linux.
  (`#1500 <https://github.com/Scille/parsec-cloud/issues/1500>`__)
* Avoid uncessary scrolling when displaying users and devices  (`#1449
  <https://github.com/Scille/parsec-cloud/issues/1449>`__)
* Improved workspaces loading  (`#1436 <https://github.com/Scille/parsec-
  cloud/issues/1436>`__)
* Fixed error message when the choosen org name already exists  (`#1345
  <https://github.com/Scille/parsec-cloud/issues/1345>`__)
* Fix an issue causing workspace files to not be closed properly.  (`#1391
  <https://github.com/Scille/parsec-cloud/issues/1391>`__)
* Refresh device list when logging out  (`#1453
  <https://github.com/Scille/parsec-cloud/issues/1453>`__)
* Validate button is disabled by default when chosing a password  (`#1459
  <https://github.com/Scille/parsec-cloud/issues/1459>`__)
* Refresh workspace list when closing the sharing dialog  (`#1495
  <https://github.com/Scille/parsec-cloud/issues/1495>`__)
* Improve client disconnection handling in the backend.  (`#1461
  <https://github.com/Scille/parsec-cloud/issues/1461>`__)
* Fixed blinking reencryption button  (`#1485 <https://github.com/Scille/parsec-
  cloud/issues/1485>`__)
* Fixed opening the GUI with a file link containing an unknown org  (`#1455
  <https://github.com/Scille/parsec-cloud/issues/1455>`__)

Deprecations and Removals
~~~~~~~~~~~~~~~~~~~~~~~~~

* Remove deprecated `parsec core apiv1` commands from the cli. (`#1440
  <https://github.com/Scille/parsec-cloud/issues/1440>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Improve error message in GUI on unexpected error.  (`#1481
  <https://github.com/Scille/parsec-cloud/issues/1481>`__)


Parsec v2.0.0 (2020-09-03)
--------------------------

No significant changes.


Parsec v1.15.2 (2020-09-02)
---------------------------

Bugfixes
~~~~~~~~

* Fix uncatched exception in GUI when listing workspaces while offline  (`#1412
  <https://github.com/Scille/parsec-cloud/issues/1412>`__)
* Fix error on Linux when using chmod/chown on mountpoint  (`#1409
  <https://github.com/Scille/parsec-cloud/issues/1409>`__)
* Contract and CGV link opens up properly  (`#1416
  <https://github.com/Scille/parsec-cloud/issues/1416>`__)
* Fixed timestamped workspace window not closing correctly on error  (`#1421
  <https://github.com/Scille/parsec-cloud/issues/1421>`__)
* Fix --backend-addr incorrectly always using localhost host in backend run
  command  (`#1425 <https://github.com/Scille/parsec-cloud/issues/1425>`__)
* Prevent unhandled exception when trying to open an unmounted workspace
  (`#1414 <https://github.com/Scille/parsec-cloud/issues/1414>`__)
* Allow to continue reencryption from the GUI if reencryption has already been
  started  (`#1422 <https://github.com/Scille/parsec-cloud/issues/1422>`__)
* Fix invite email in backend when not mocked (`#1410
  <https://github.com/Scille/parsec-cloud/issues/1410>`__)


Parsec v1.15.0 (2020-08-29)
---------------------------

Features
~~~~~~~~

* Updated the logos  (`#1316 <https://github.com/Scille/parsec-
  cloud/issues/1316>`__)
* Add a warning when chosing user role during the greet process  (`#1352
  <https://github.com/Scille/parsec-cloud/issues/1352>`__)
* Add support for confined (i.e temporary) files and directories. In this
  context, confined means files that are not meant to be synchronized with other
  clients  (`#990 <https://github.com/Scille/parsec-cloud/issues/990>`__)
* Moved user info to the top right  (`#1153 <https://github.com/Scille/parsec-
  cloud/issues/1153>`__)
* Explain password and confirmation mismatch  (`#1265
  <https://github.com/Scille/parsec-cloud/issues/1265>`__)
* Notify user when the current in used organization has expired  (`#1206
  <https://github.com/Scille/parsec-cloud/issues/1206>`__)
* Updated workspace sharing to be easier to use  (`#1138
  <https://github.com/Scille/parsec-cloud/issues/1138>`__)
* New organization creation process  (`#1257 <https://github.com/Scille/parsec-
  cloud/issues/1257>`__)
* Sexier login screen  (`#1130 <https://github.com/Scille/parsec-
  cloud/issues/1130>`__)
* Allows creating an organization on a custom metadata server  (`#1390
  <https://github.com/Scille/parsec-cloud/issues/1390>`__)
* Add one custom rsync to parsec  (`#953 <https://github.com/Scille/parsec-
  cloud/issues/953>`__)
* GUI allows organization creation on a custom backend  (`#1133
  <https://github.com/Scille/parsec-cloud/issues/1133>`__)

Bugfixes
~~~~~~~~

* Do not open new login tab in the gui if a file linked is clicked with an
  already opened organization  (`#1398 <https://github.com/Scille/parsec-
  cloud/issues/1398>`__)
* Do not display disconnected notification when login in  (`#1353
  <https://github.com/Scille/parsec-cloud/issues/1353>`__)
* Display the correct message when closing a connected tab  (`#1382
  <https://github.com/Scille/parsec-cloud/issues/1382>`__)
* Prevent spaces in organization name  (`#1256
  <https://github.com/Scille/parsec-cloud/issues/1256>`__)
* Check email validity when creating an organization/inviting a user  (`#1377
  <https://github.com/Scille/parsec-cloud/issues/1377>`__)
* Fixed organization creation window closing when passwords mismatch  (`#1376
  <https://github.com/Scille/parsec-cloud/issues/1376>`__)
* Do not restart claimer invitation process on an InviteAlreadyUsedError
  (`#1363 <https://github.com/Scille/parsec-cloud/issues/1363>`__)
* Fix email user invite generation  (`#1400 <https://github.com/Scille/parsec-
  cloud/issues/1400>`__)
* Fix inconsistence backend replies from an cancelled invite command  (`#1365
  <https://github.com/Scille/parsec-cloud/issues/1365>`__)
* Added workspace name in error message when removed from a workspace  (`#1385
  <https://github.com/Scille/parsec-cloud/issues/1385>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Devices keys filenames are no longer meaningful.  Device key files used to be
  stored in a directory named after the device slug in a file also named after
  the same device slug. As a result, the device path used to be very long (about
  200 characters).  Device key files are now stored directly in the devices
  directory using the device slughash and the `.keys` extension. The path is now
  much shorter  (`#1366 <https://github.com/Scille/parsec-cloud/issues/1366>`__)
* In order to simplify url validation in the GUI, parsec:// url without hostname
  part are now considered invalid instead of defaulting to localhost. (`#1402
  <https://github.com/Scille/parsec-cloud/issues/1402>`__)
* Inviting an user to join organization now display a confirmation pop-up.
  (`#1346 <https://github.com/Scille/parsec-cloud/issues/1346>`__)
* Invited users is now displayed before the organization users  (`#1351
  <https://github.com/Scille/parsec-cloud/issues/1351>`__)
* The winfsp and fuse mountpoints now always report 0 MB used over a 1 TB
  capacity. Those values are arbitrary but useful to the operating system,
  especially OSX.  (`#1401 <https://github.com/Scille/parsec-
  cloud/issues/1401>`__)


Parsec v1.14.0 (2020-08-06)
---------------------------

Features
~~~~~~~~

* Added some keyboard shortcuts  (`#1151 <https://github.com/Scille/parsec-
  cloud/issues/1151>`__)
* Added a "+" button to add a new tab  (`#1155
  <https://github.com/Scille/parsec-cloud/issues/1155>`__)
* Switched app font to Montserrat  (`#1147 <https://github.com/Scille/parsec-
  cloud/issues/1147>`__)
* Workspaces can now be enabled/disabled from the application. The workspace
  status is stored in the configuration in order to be restored at the next
  application startup.  (`#1159 <https://github.com/Scille/parsec-
  cloud/issues/1159>`__)
* Updated user list to look more like the device list  (`#1154
  <https://github.com/Scille/parsec-cloud/issues/1154>`__)
* Allows join organization to take a bootstrap org link  (`#1170
  <https://github.com/Scille/parsec-cloud/issues/1170>`__)
* Hide an already connected device from the list of available devices  (`#1139
  <https://github.com/Scille/parsec-cloud/issues/1139>`__)
* Added an automated email sending function on user invite to workspace  (`#1177
  <https://github.com/Scille/parsec-cloud/issues/1177>`__)
* Added additional text for the main menu  (`#1150
  <https://github.com/Scille/parsec-cloud/issues/1150>`__)
* Added optional RC channel updater  (`#1324 <https://github.com/Scille/parsec-
  cloud/issues/1324>`__)
* Display systray notification to make offline mode more obvious to the users
  (`#1330 <https://github.com/Scille/parsec-cloud/issues/1330>`__)

Bugfixes
~~~~~~~~

* Display author name in file history instead of DeviceID  (`#1270
  <https://github.com/Scille/parsec-cloud/issues/1270>`__)
* Fix GUI behavior when trying to share a workspace while not connected to the
  backend or wen providing an invalid user name  (`#1242
  <https://github.com/Scille/parsec-cloud/issues/1242>`__)
* Fixed revoked user exception handling and notification.  (`#1205
  <https://github.com/Scille/parsec-cloud/issues/1205>`__)
* Bootstrap organization widget made more responsive on low resolutions  (`#1169
  <https://github.com/Scille/parsec-cloud/issues/1169>`__)
* Fixed menu icons alignement and colors  (`#1149
  <https://github.com/Scille/parsec-cloud/issues/1149>`__)
* Fixed missing reject method on file history  (`#1239
  <https://github.com/Scille/parsec-cloud/issues/1239>`__)
* Fixed history window not showing when a file has a source.  (`#1182
  <https://github.com/Scille/parsec-cloud/issues/1182>`__)
* Fix realm access check in backend for user who has lost it role to this realm.
  (`#1184 <https://github.com/Scille/parsec-cloud/issues/1184>`__)
* Fix sharing error message causing unhandled exception in the GUI  (`#1241
  <https://github.com/Scille/parsec-cloud/issues/1241>`__)
* Fix Python 3.8 incompatibility (bug in trio_asyncio with postgresql)  (`#1194
  <https://github.com/Scille/parsec-cloud/issues/1194>`__)
* Fixed some hidden windows staying in memory  (`#1156
  <https://github.com/Scille/parsec-cloud/issues/1156>`__)
* Added clearer messages on failure to access a file by its link  (`#1167
  <https://github.com/Scille/parsec-cloud/issues/1167>`__)
* Improve high DPI support for the parsec application.  (`#1245
  <https://github.com/Scille/parsec-cloud/issues/1245>`__)
* Updating pynacl to 1.4.0 (`#1172 <https://github.com/Scille/parsec-
  cloud/issues/1172>`__)
* Fix history button in GUI  (`#1243 <https://github.com/Scille/parsec-
  cloud/issues/1243>`__)
* Fix error on Windows when using the mountpoint right after (<0.01s) it has
  been mounted. (`#1210 <https://github.com/Scille/parsec-cloud/issues/1210>`__)
* Path display no longer makes the window expand  (`#1162
  <https://github.com/Scille/parsec-cloud/issues/1162>`__)
* The workspaces are now mounted as separated drives on Windows. Also,
  workspaces with reader access are mounted as read-only volumes. This allows
  proper compatibility with Acrobat Reader and avoid path-length issues.
  (`#1081 <https://github.com/Scille/parsec-cloud/issues/1081>`__)
* Fixed deadlock when importing a file from a parsec workspace  (`#1188
  <https://github.com/Scille/parsec-cloud/issues/1188>`__)
* Fix GUI main windows not showing when use close button from the systray. Notif
  explaining Parsec is still running on GUI windows close only triggered once.
  (`#1295 <https://github.com/Scille/parsec-cloud/issues/1295>`__)
* Fix backend side connection auto-close on user revocation when the connection
  has been used to listen events. (`#1314 <https://github.com/Scille/parsec-
  cloud/issues/1314>`__)
* Fixed workspace title showing id instead of name  (`#1321
  <https://github.com/Scille/parsec-cloud/issues/1321>`__)
* Fix internal exception handling of the remote devices manager errors.  (`#1335
  <https://github.com/Scille/parsec-cloud/issues/1335>`__)

Client/Backend API evolutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Add --spontaneous-organization-bootstrap option to backend to allow
  bootstrapping an organization that haven't been created by administration
  beforehand. Add --oganization-bootstrap-webhook option to backend to notify a
  webhook URL on organization bootstrap.  (`#1281
  <https://github.com/Scille/parsec-cloud/issues/1281>`__)
* Update API to version 2.0 which improve handshake system and rework enrollment
  system for a SAS-based asynchronous one (better usability and security)
  (`#1119 <https://github.com/Scille/parsec-cloud/issues/1119>`__)
* API can now return stats about workspace such as metadata size and data size.
  (`#1176 <https://github.com/Scille/parsec-cloud/issues/1176>`__)
* Introduce outsider profil for user. Outsider users can read/write on
  workspaces they are invited to, but are not allowed to create workspaces. On
  top of that outsider users cannot see personnal informations (email &
  user/device name) of other users.  (`#1163 <https://github.com/Scille/parsec-
  cloud/issues/1163>`__)
* Adding some http request managment.  (`#1171
  <https://github.com/Scille/parsec-cloud/issues/1171>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Remove ``(shared by X)`` messages from workspace name.  (`#928
  <https://github.com/Scille/parsec-cloud/issues/928>`__)
* Add a high-level interface for workspace files.  (`#1190
  <https://github.com/Scille/parsec-cloud/issues/1190>`__)
* Consider https as default endpoint scheme for blockstore config in backend run
  cli (`#1143 <https://github.com/Scille/parsec-cloud/issues/1143>`__)
* Turn user_id and device_name fields into UUID to anonymize them. Personal
  informations are instead stored in human_handle and device_label fields which
  are not available to users with OUTSIDER profile.  (`#1174
  <https://github.com/Scille/parsec-cloud/issues/1174>`__)
* Change bytes symbol in English  (`#1221 <https://github.com/Scille/parsec-
  cloud/issues/1221>`__)
* Update WinFSP embedded package  (`#1223 <https://github.com/Scille/parsec-
  cloud/issues/1223>`__)
* Use 4 symbols from a 32-symbol alphabet as SAS code. The alphatbet is:
  ``ABCDEFGHJKLMNPQRSTUVWXYZ23456789``.  (`#1165
  <https://github.com/Scille/parsec-cloud/issues/1165>`__)
* Backend now able to retry first db connection  (`#1258
  <https://github.com/Scille/parsec-cloud/issues/1258>`__)
* Remove noop --db-drop-deleted-data option from backend run command  (`#1246
  <https://github.com/Scille/parsec-cloud/issues/1246>`__)
* Added docker-compose as a backend deployment option  (`#1233
  <https://github.com/Scille/parsec-cloud/issues/1233>`__)
* Add DPI aware option in the Windows installer options to fix blurry texts on
  some high-DPI screens.  (`#1203 <https://github.com/Scille/parsec-
  cloud/issues/1203>`__)
* Update windows installer to be less verbose. In particular: skip the
  components panel, hide installation details and advance automatically after
  completion.  (`#1126 <https://github.com/Scille/parsec-cloud/issues/1126>`__)
* Restrict read access for parsec directories to the current user. This includes
  configuration, data, config and workspace directories.  (`#940
  <https://github.com/Scille/parsec-cloud/issues/940>`__)
* Fix mount error when using Snap package on Debian when fuse is not installed.
  (`#1296 <https://github.com/Scille/parsec-cloud/issues/1296>`__)
* Run Parsec with regular user priviledges when the "Run Parsec" checkbox is
  ticked at the end of the windows installation.  (`#1303
  <https://github.com/Scille/parsec-cloud/issues/1303>`__)
* Updated instructions texts for the device invitation process  (`#1304
  <https://github.com/Scille/parsec-cloud/issues/1304>`__)


Parsec 1.13.0 (2020-04-29)
--------------------------

Features
~~~~~~~~

* Added a way to create an organization on the business website directly from
  the GUI  (`#1014 <https://github.com/Scille/parsec-cloud/issues/1014>`__)
* Add one migration tool in the cli.  (`#1116 <https://github.com/Scille/parsec-
  cloud/issues/1116>`__)
* Add an action to open the current directory in file explorer  (`#1107
  <https://github.com/Scille/parsec-cloud/issues/1107>`__)
* Add a contextual menu on workspace buttons  (`#1085
  <https://github.com/Scille/parsec-cloud/issues/1085>`__)
* Updated file icons to reflect the file format  (`#1093
  <https://github.com/Scille/parsec-cloud/issues/1093>`__)

Bugfixes
~~~~~~~~

* Allow closing of login in tab  (`#1101 <https://github.com/Scille/parsec-
  cloud/issues/1101>`__)
* Fixed GUI staying minimized when an URL is clicked  (`#1100
  <https://github.com/Scille/parsec-cloud/issues/1100>`__)
* Fix internal behavior involving cancelled tasks that could lead to unhandled
  errors logs.  (`#1123 <https://github.com/Scille/parsec-cloud/issues/1123>`__)
* Fix save operations on windows for some third party applications.  This is
  related to the mechanism used by third party applications to safely save
  files. This mechanism might use the `replace_if_exists` flag in the `rename`
  winfsp operation. This flag is now supported.  (`#1128
  <https://github.com/Scille/parsec-cloud/issues/1128>`__)
* Allows workspace owners to change the role of other owners  (`#870
  <https://github.com/Scille/parsec-cloud/issues/870>`__)
* Fixed alignment problem when displaying users  (`#1127
  <https://github.com/Scille/parsec-cloud/issues/1127>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Improve high CPU usage and blocking IO detection.  (`#1124
  <https://github.com/Scille/parsec-cloud/issues/1124>`__)
* Update API to version 1.2 which add human handle system  (`#1104
  <https://github.com/Scille/parsec-cloud/issues/1104>`__)


Parsec 1.12.0 (2020-04-14)
--------------------------

Bugfixes
~~~~~~~~

* Fix forbidden error during backend startup when some custom S3 providers
  (`#1094 <https://github.com/Scille/parsec-cloud/issues/1094>`__)
* Use "localhost" as the default hostname in the cli.  (`#1075
  <https://github.com/Scille/parsec-cloud/issues/1075>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Add `fs.entry.file_conflict_resolved` internal event to be notified when a
  file conflict has been resolved by copying and renaming the file with the
  local changes.  (`#1095 <https://github.com/Scille/parsec-
  cloud/issues/1095>`__)
* Add cancel button to "Parsec is already running, please close it" prompt in
  windows installer. (`#1103 <https://github.com/Scille/parsec-
  cloud/issues/1103>`__)
* Update the windows installer to be less verbose. In particular, the Winfsp
  installation becomes silent.  (`#1112 <https://github.com/Scille/parsec-
  cloud/issues/1112>`__)


Parsec 1.11.4 (2020-03-31)
--------------------------

No significant changes.


Parsec 1.11.3 (2020-03-31)
--------------------------

No significant changes.


Parsec 1.11.2 (2020-03-31)
--------------------------

No significant changes.


Parsec 1.11.1 (2020-03-31)
--------------------------

No significant changes.


Parsec 1.11.0 (2020-03-30)
--------------------------

Features
~~~~~~~~

* The overall appearance of the GUI has changed: new icons, new colors, new
  texts, and a few fixes  (`#952 <https://github.com/Scille/parsec-
  cloud/issues/952>`__)


Parsec 1.10.0 (2020-03-26)
--------------------------

Features
~~~~~~~~

* Improved updater now selects the right latest exe file on Windows  (`#1054
  <https://github.com/Scille/parsec-cloud/issues/1054>`__)

Bugfixes
~~~~~~~~

* Fix ``parsec backend init`` cli command crashing due to a missing
  ``init_tables.sql`` resource. (`#1052 <https://github.com/Scille/parsec-
  cloud/issues/1052>`__)
* Fix unhandled error message in GUI that could occur during sync with poor
  connection. (`#1055 <https://github.com/Scille/parsec-cloud/issues/1055>`__)
* Fix marker issue when listing many files in a directory.  (`#1039
  <https://github.com/Scille/parsec-cloud/issues/1039>`__)


Parsec 1.9.1 (2020-03-13)
-------------------------

Bugfixes
~~~~~~~~

* Added missing organization_update to admin cmds  (`#1032
  <https://github.com/Scille/parsec-cloud/issues/1032>`__)


Parsec 1.9.0 (2020-03-06)
-------------------------

Features
~~~~~~~~

* Only allows one log in tab in all situations  (`#963
  <https://github.com/Scille/parsec-cloud/issues/963>`__)

Bugfixes
~~~~~~~~

* Fixed invalid access to file table item  (`#1021
  <https://github.com/Scille/parsec-cloud/issues/1021>`__)
* Fix error handling during workspace reencryption detection when offline.
  (`#1016 <https://github.com/Scille/parsec-cloud/issues/1016>`__)
* Fix an error on linux when mounting a workspace when the workspace manifest is
  absent and the session is offline.  (`#1018 <https://github.com/Scille/parsec-
  cloud/issues/1018>`__)
* Fix invalid access to workspace_id on entry_updated  (`#1022
  <https://github.com/Scille/parsec-cloud/issues/1022>`__)
* Fix workspace_fs not available on event  (`#1001
  <https://github.com/Scille/parsec-cloud/issues/1001>`__)
* Fix access to invalid attribute on timestamped workspace  (`#1020
  <https://github.com/Scille/parsec-cloud/issues/1020>`__)
* Fix synchronization not triggered for newly created workspaces until they get
  files. (`#1023 <https://github.com/Scille/parsec-cloud/issues/1023>`__)


Parsec 1.8.0 (2020-03-03)
-------------------------

Features
~~~~~~~~

* Added a link to the documentation  (`#999 <https://github.com/Scille/parsec-
  cloud/issues/999>`__)
* Removed confirmation when opening a new tab  (`#993
  <https://github.com/Scille/parsec-cloud/issues/993>`__)

Bugfixes
~~~~~~~~

* Fix French translation for changelog  (`#994
  <https://github.com/Scille/parsec-cloud/issues/994>`__)
* Case insensitive extension matching when displaying file icon  (`#1007
  <https://github.com/Scille/parsec-cloud/issues/1007>`__)

Improved Documentation
~~~~~~~~~~~~~~~~~~~~~~

* Add french translation to the documentation (`#1005
  <https://github.com/Scille/parsec-cloud/issues/1005>`__)


Parsec 1.7.2 (2020-02-24)
-------------------------

No significant changes.


Parsec 1.7.1 (2020-02-24)
-------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Fix bug in sdist/bdist_wheel configuration that prevented release on pypi.org
  since 1.4.0 (`#992 <https://github.com/Scille/parsec-cloud/issues/992>`__)


Parsec 1.7.0 (2020-02-22)
-------------------------

Features
~~~~~~~~

* Add a way to copy/paste an internal link to a file  (`#937
  <https://github.com/Scille/parsec-cloud/issues/937>`__)
* Access a file directly using an url  (`#938 <https://github.com/Scille/parsec-
  cloud/issues/938>`__)

Bugfixes
~~~~~~~~

* Disable file operations for a reader  (`#981
  <https://github.com/Scille/parsec-cloud/issues/981>`__)
* Fix files display not being updated automatically  (`#980
  <https://github.com/Scille/parsec-cloud/issues/980>`__)


Parsec 1.6.0 (2020-02-12)
-------------------------

Features
~~~~~~~~

* Added a global menu to the GUI  (`#945 <https://github.com/Scille/parsec-
  cloud/issues/945>`__)
* Add a line under the tab bar  (`#942 <https://github.com/Scille/parsec-
  cloud/issues/942>`__)
* Removed tab title length limit  (`#944 <https://github.com/Scille/parsec-
  cloud/issues/944>`__)

Bugfixes
~~~~~~~~

* Clear password input when switching device on login  (`#946
  <https://github.com/Scille/parsec-cloud/issues/946>`__)
* Fix files display on low horizontal resolutions  (`#926
  <https://github.com/Scille/parsec-cloud/issues/926>`__)
* Display an error when trying to move a folder into itself  (`#935
  <https://github.com/Scille/parsec-cloud/issues/935>`__)
* Fix users and devices being hidden on low resolutions  (`#927
  <https://github.com/Scille/parsec-cloud/issues/927>`__)
* Disable Paste button if nothing has been copied/cut  (`#934
  <https://github.com/Scille/parsec-cloud/issues/934>`__)
* Fix menu bar being resized when changing window size  (`#932
  <https://github.com/Scille/parsec-cloud/issues/932>`__)


Parsec 1.5.0 (2020-01-20)
-------------------------

Features
~~~~~~~~

* Add copy, cut and paste to the Parsec file explorer  (`#855
  <https://github.com/Scille/parsec-cloud/issues/855>`__)

Bugfixes
~~~~~~~~

* Fix unhandled exception in backend when a client connected over ssl disconnect
  during handshake. (`#833 <https://github.com/Scille/parsec-
  cloud/issues/833>`__)
* Fix Organization bootstrap and user/device claim links encoding when their
  corresponding organization ID contains unicode. (`#884
  <https://github.com/Scille/parsec-cloud/issues/884>`__)
* Fix recreation of an organization by the administration as long as it hasn't
  been bootstrapped.  (`#885 <https://github.com/Scille/parsec-
  cloud/issues/885>`__)
* Clear displayed files on stat error  (`#920 <https://github.com/Scille/parsec-
  cloud/issues/920>`__)
* Fix a bug related to broken symlinks in the base directory for mountpoints
  after a hard shutdown.  (`#881 <https://github.com/Scille/parsec-
  cloud/issues/881>`__)
* Used new partial strategy to download manifests when rebuilding history to fix
  it not loading on a heavy workspace.  (`#888
  <https://github.com/Scille/parsec-cloud/issues/888>`__)
* Fix incorrect behavior when the backend accept anonymous connection to expired
  organization. (`#891 <https://github.com/Scille/parsec-cloud/issues/891>`__)
* Prevent winfsp from freezing the application when the mounting operation times
  out.  (`#905 <https://github.com/Scille/parsec-cloud/issues/905>`__)
* Prevent managers from inviting other users as managers  (`#916
  <https://github.com/Scille/parsec-cloud/issues/916>`__)
* Deal with special dash paths in fuse operations.  (`#904
  <https://github.com/Scille/parsec-cloud/issues/904>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Allow owners to switch the role of other owners  (`#870
  <https://github.com/Scille/parsec-cloud/issues/870>`__)


Parsec 1.4.0 (2019-12-06)
-------------------------

Bugfixes
~~~~~~~~

* Fix error handling of list&revoke user in GUI. (`#834
  <https://github.com/Scille/parsec-cloud/issues/834>`__)
* Fix mount error on Windows when workspace name is too long (`#838
  <https://github.com/Scille/parsec-cloud/issues/838>`__)
* Fix colored workspace button display  (`#851
  <https://github.com/Scille/parsec-cloud/issues/851>`__)
* Fix bug when the workspaces doesn't show up on new device creation until the
  user manifest is actually modified. (`#854 <https://github.com/Scille/parsec-
  cloud/issues/854>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Provide fusepy with the file system encoding. Also use EINVAL as fallback
  error code.  (`#827 <https://github.com/Scille/parsec-cloud/issues/827>`__)


Parsec 1.3.0 (2019-11-28)
-------------------------

Features
~~~~~~~~

* Add a button to manually add a new tab Do not open a new tab when launching
  the app without any parameters (`#774 <https://github.com/Scille/parsec-
  cloud/issues/774>`__)
* Allow only one Log-In tab (`#777 <https://github.com/Scille/parsec-
  cloud/issues/777>`__)
* Hide revoked users in workspace sharing dialog (`#780
  <https://github.com/Scille/parsec-cloud/issues/780>`__)
* Prevent tab change if a modal is open (`#820
  <https://github.com/Scille/parsec-cloud/issues/820>`__)
* Tab color changes when an instance receives a notification (`#821
  <https://github.com/Scille/parsec-cloud/issues/821>`__)

Bugfixes
~~~~~~~~

* Now handles inconsistent directories accessed from the GUI, tested mountpoint
  behaviour (`#782 <https://github.com/Scille/parsec-cloud/issues/782>`__)
* Fix infinite loop in IPC server (`#813 <https://github.com/Scille/parsec-
  cloud/issues/813>`__)
* Fix config not saved when updating from the settings tab when logged in.
  (`#815 <https://github.com/Scille/parsec-cloud/issues/815>`__)
* Fix duplication and infinite loading in view on directories containing many
  entries under Windows. (`#835 <https://github.com/Scille/parsec-
  cloud/issues/835>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Change the invitation token format to 6 random digits.  (`#819
  <https://github.com/Scille/parsec-cloud/issues/819>`__)


Parsec 1.2.1 (2019-11-20)
-------------------------

* Add view to Display changelog history in the GUI (`#788
  <https://github.com/Scille/parsec-cloud/issues/788>`__)


Parsec 1.2.0 (2019-11-15)
-------------------------

Features
~~~~~~~~

* Backend now checks if timestamp is not inferior of existant on vlob update, if
  it is, sends an error to client which temporarily goes offline to avoid the
  handling of this event in a retry loop.  (`#758
  <https://github.com/Scille/parsec-cloud/issues/758>`__)
* Add notification in GUI when an operation in the mountpoint failed in an
  unexpected manner. (`#759 <https://github.com/Scille/parsec-
  cloud/issues/759>`__)
* Limit a tab title to a few characters and add a tooltip to tabs  (`#775
  <https://github.com/Scille/parsec-cloud/issues/775>`__)
* Add tooltips to taskbar buttons  (`#776 <https://github.com/Scille/parsec-
  cloud/issues/776>`__)
* Removed duplicates and supposed minimal sync when listing versions of a path
  (`#784 <https://github.com/Scille/parsec-cloud/issues/784>`__)

Bugfixes
~~~~~~~~

* Fix crash on Linux when the ipc server lock file is located in a non existant
  directory (`#760 <https://github.com/Scille/parsec-cloud/issues/760>`__)
* Fix crash in ipc server when socket file path contains missing folder (only on
  windows).  (`#765 <https://github.com/Scille/parsec-cloud/issues/765>`__)
* Fix rights checking in winfsp operations. This issue used to cause a cffi
  crash on windows when some operations were performed on the file system.
  (`#770 <https://github.com/Scille/parsec-cloud/issues/770>`__)
* Fix len check in ``OrganizationID``/``UserID``/``DeviceName``/``DeviceID``
  when containing multibytes unicode characters. (`#794
  <https://github.com/Scille/parsec-cloud/issues/794>`__)
* Improve support of unicode in the mountpoint on Windows. (`#799
  <https://github.com/Scille/parsec-cloud/issues/799>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Improve logging output on backend server  (`#753
  <https://github.com/Scille/parsec-cloud/issues/753>`__)


Parsec 1.1.2 (2019-10-22)
-------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Small GUI improvements on white border around main tab and url
  error message display
* Remove dependency on pywin32 under Windows which cause packaging issue on
  previous version (`#750 <https://github.com/Scille/parsec-
  cloud/issues/750>`__)


Parsec 1.1.1 (2019-10-21)
-------------------------

Bugfixes
~~~~~~~~

* Fix argument parsing in backend cli commands (``PARSEC_CMD_ARGS`` env var, db
  param and S3 entry point default value) (`#749
  <https://github.com/Scille/parsec-cloud/issues/749>`__)


Parsec 1.1.0 (2019-10-21)
-------------------------

Features
~~~~~~~~

* Add support for IPC communication in GUI to have a single instance running.
  Also add tab support & handle parsec:// url as start argument.  (`#684
  <https://github.com/Scille/parsec-cloud/issues/684>`__)
* Rework backend cli argument and environ variable handling  (`#701
  <https://github.com/Scille/parsec-cloud/issues/701>`__)

Bugfixes
~~~~~~~~

* Fix pure HTTP query handling in backend (`#699
  <https://github.com/Scille/parsec-cloud/issues/699>`__)
* Fix long wait on GUI login with poor connection to the backend (`#706
  <https://github.com/Scille/parsec-cloud/issues/706>`__)
* Add missing check in core to enforce consistency of timestamps between a
  manifest and it author's role certificate (`#734
  <https://github.com/Scille/parsec-cloud/issues/734>`__)
* Fix fonts scaling on wayland (`#735 <https://github.com/Scille/parsec-
  cloud/issues/735>`__)
* Fix bug causing workspace mountpoint directory not being removed on
  application shutdown (`#737 <https://github.com/Scille/parsec-
  cloud/issues/737>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Allow dash character (i.e. ``-``) in OrganizationID, UserID & DeviceName
  (`#728 <https://github.com/Scille/parsec-cloud/issues/728>`__)


Parsec 1.0.2 (2019-10-01)
-------------------------

* Upgrade PyQt5 to 5.13.1 (`#690
  <https://github.com/Scille/parsec-cloud/issues/690>`__)
* Add keepalive pings on invite/claim requests (`#693
  <https://github.com/Scille/parsec-cloud/issues/693>`__)


Parsec 1.0.1 (2019-09-25)
-------------------------

* Upgrade wsproto to 0.15.0 to improve websocket compatibility (`#686
  <https://github.com/Scille/parsec-cloud/issues/686>`__)
* Replace CXFreeze by a custom script to generate win32 builds (`#685
  <https://github.com/Scille/parsec-cloud/issues/685>`__)
* Add organization status command in cli (`#683
  <https://github.com/Scille/parsec-cloud/issues/683>`__)
* User/device invitation get cancelled on server side when the user use the
  cancel button (`#682 <https://github.com/Scille/parsec-cloud/issues/682>`__)
* Add organization expiration date support in backend (`#680
  <https://github.com/Scille/parsec-cloud/issues/680>`__)
* Client connection to Backend specify a `/ws` resource endpoint (`#678
  <https://github.com/Scille/parsec-cloud/issues/678>`__)


Parsec 1.0.0 (2019-09-10)
-------------------------

* First stable release

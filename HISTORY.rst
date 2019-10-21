=======
History
=======


.. towncrier release notes start


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

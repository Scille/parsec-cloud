# Account recovery mechanism

From [ISSUE-1614](https://github.com/Scille/parsec-cloud/issues/1614)

Here is a draft of a potential strategy to implement an account recovery mechanism:

## Setup

1) user selects other user as trustee for recovering `DSI` (device secret information)

2) trustee generates `ETs`/`ETp` (secret and public parts of an encryption key), stores `ETs` in his user manifest and sends `ETp` to user

3) user generates `EUs`/`EUp` (secret and public parts of an encryption key)

4) user stores `Etp(EUp(USI))` along with `EUs` in cleartext on his device's machine

## Recovery

1) user sends `Etp(EUp(USI))` to trustee

2) trustee deciphers and returns `EUp(USI)`

3) user deciphers and obtain `USI`

## Notes

Might be good to store `Etp(EUp(USI))` in the backend to add a third party (hence preventing trustee from having access to `DSI` if he has access to the device's machine)
This way account recovery can start by the backend sending a recovery email to the user with a specific link to start the recovery.

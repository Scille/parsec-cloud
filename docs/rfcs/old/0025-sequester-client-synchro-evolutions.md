# Sequester - Client synchro evolutions

From [ISSUE-2466](https://github.com/Scille/parsec-cloud/issues/2466)

see [RFC-0022](0022-sequester-protocol-data-api-evolutions.md) for API evolutions

The idea is:

1) client has local changes, it wants to do a sync
2) block upload and manifest reshape are done
3) when the client tries to upload the encrypted manifest (i.e. through a `vlob_create` or a `vlob_update` command), the server returns an error to signify the command lack encryption for the sequester services[^1]
4) The client proceed then to verify the SequesterEncryptionKey (multiple can be provided) and SequesterSigningKey (there is only one) that have been provided by the server as part of the error (we expect RootVerifyKey -sign-> SequesterSigningKey  -sign-> SequesterEncryptionKey). If something is wrong a specific exception should be raised so that we can print a message to the user in the GUI (and after that the server connection should be considered crashed to avoid subsequent attempts).
5) The client encrypt the signed manifest with each of the `SequesterEncryptionKey`, then retry it `vlob_create/vlob_update` operation with the original encrypted manifest and all the encrypted for sequester services
6) The `SequesterEncryptionKey` should be kept in an in-memory cache so that step 3) is only done once[^2]

[^1]: Of course if there is no Sequester service configured (or if the Sequester feature is entirely disabled for the organization), the server just accept the upload ^^
[^2]: Step 3) can accors again only in case the sequester configuration changes while the client is connected (hence some of the sequester encryption sent by the client are no longer valid)

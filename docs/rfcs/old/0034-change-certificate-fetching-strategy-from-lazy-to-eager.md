# Change certificate fetching strategy from lazy to eager

From [ISSUE-3126](https://github.com/Scille/parsec-cloud/issues/3126)

## Current situation

Currently the Parsec client tries to only fetch the needed user/device/role certificates.
As [ISSUE-3125](https://github.com/Scille/parsec-cloud/issues/3125) showed this can produce slow down given any operation involving certificate can lead to a server request.

As a matter of fact we have put a 1h cache on the certificate used in the trustchain to avoid heavy slowdown when validating incoming data.
But the cache is not perfect given at any time a revocation certificate (or a new role certificate for a given user) can be issued so we should have a cache invalidation mechanism. However this invalidation mechanism doesn't currently exist and so we don't use cache when we actually want to get certificate to display info to user (e.g. to avoid the issue of having a user we've just revoked shown as still part of workspace), but this create the possible slowdown when server connection is lagging.

On top of that validating a vlob means we need to fetch it author's device certificate and the whole associated trustchain.
So we end up pretty fast with pretty much all the organization's certificates downloaded (we can imagine organization were groups of user only work is different workspace and hence only the certificate of a single group are download, but anyway the main point here we are likely to download a lot of certificates)

A certificates is small (typically ~250bytes/certificate) so it's not a big deal to download them all in a batch, on the other hand having to download a lot of them one by one is costly due to inflight time in the request to the server.

## Target situation

We should introduce a new `certificate_fetch` api to download all certificates for a given organization, this api would have an index parameter so that we can return a subset in case we already called this api in the past.
On top of that we should introduce a new certificate added event.

With this the client would do:

- when connection with the server is up, it calls `certificate_fetch` with the index it has kept in it local storage (or 0 if the local storage is empty). This call is part of the client connection init so we are guarantee the certificates are all available once we are considered online.
- when the a new certificate is create, the server increase a global counter that correspond to the index provided in `certificate_fetch`, the server also dispatch the `certificate_added`. Given how small the certifcate is, we should provide it directly into the event (the good thing being the client is guaranteed to have the trustchain to validate the new certificate if it call `certificate_fetch` first, then process all incoming event in fifo)

With this we have multiple benefits:

- almost never have to wait for server when validating data: the only time we can have a certificate miss is when we validate data created by a user that has been added extremely recently (basically a new user had time to upload new data, and our client to download it before our client processed the `certificate_added` event). However a simple counter to this is to call again `certificate_fetch` any time we encounter an unknown author id (and only after this call we can determine if the author is very recent or just doesn't exist)
- no longer need for the `human_find` and `user_get` api: we can do everything in local \o/ This should speed up all the view where we list the users (so users/devices views, but also the workspace sharing dialog or the workspace widget that show if it is shared). On top of that we can make all those functions synchronous which makes code simpler for the gui !

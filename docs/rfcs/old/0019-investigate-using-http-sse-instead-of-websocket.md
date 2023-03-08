# Investigate using http+sse instead of websocket to serve authenticated api

From [ISSUE-2358](https://github.com/Scille/parsec-cloud/issues/2358)

Current websocket api doesn't support requests multiplexing (i.e. it works in a strict request/reply order), so the client has to deal with connection pool which is complexe (and eat up more ressources than needed given we keep open multiple connections per client)

A low hanging fruit would be to modify the websocket api to support request multiplexing (typically by adding an id to each request).

But websocket is still more complex than regular http:

- company firewalls often block websocket by default
- http is stateless, so reconnection (and reuse of already opened connection) is transparente
- http2 handles head of line blocking which remove entirely the need for multiple connections to the backend

So it would be good if we can expose the authenticated api as http+sse (hence we could use [reqwest](https://github.com/seanmonstar/reqwest) in Rust than handle http2 and wasm32 out of the box \o/)

The tricky parts are: authentication, api version negotiation and listen_event command

## 1 - Authentication

With the current websocket-based api, the very first requests does a handshake which starts by exchanging the api versions, then proceed to a challenge/reply to authenticate the client.

There is two approches to implement this with http:

- stateful where the client would have to obtain a token from an `/auth` route before sending actuel commands
- stateless where the client would sign the commands send to server

pros/cons:

- stateful is a bit faster than stateless given cryptographic operations are only done once when obtaining the token.
- stateles should be simpler given less route to implement
- to obtain the token in stateful, the client has to sign something. This might be a timestamp or a something random provided by the server (in which case we have yet another round trip to handle, the backend must keep track of the random info, or sign it)

interesting stuff:

- <https://www.rfc-editor.org/rfc/rfc7616.html>
- <https://www.iana.org/assignments/http-authschemes/http-authschemes.xhtml>
- <https://github.com/Scille/vigiechiro-api/blob/33b1db51d74d148b234d18474be96cc7b617749c/vigiechiro/resources/fichiers.py#L154-L199> implementation of the AWS S3 request signature

## 2 - API version negotiation

With the current websocket-based api, the very first requests does a handshake which starts by exchanging the api versions supported by the server and the client to determine which should be used.

With a http api, we would have to replace this by headers:
1 - Client send a request to server, it uses the latest api version it supports and add the version number to the content-type header (e.g. `Content-Type: application/msgpack;v=2.6`)
2 - If server support the api version, it replies with the content-type header indicating it own version of the api (this is needed given api revision might not be the same between client and server)
3 - If server doesn't support the api version, it replies with a specific "api not supported" message which contains the list of api version it supports (e.g. `{"status": "unsupported_api_version", "supported_api_versions": ["1.10", "2.3"]}`). The client can then retry if it supports an older version.

On top of that, the information of what api version should be used with the server support would be lazily kept by the client so that only the first request would end up in 3)

## 3 - listen_event command

This should be done by SSE

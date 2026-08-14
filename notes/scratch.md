
### 3.1. Document save strategy





On top of that, I haven't settled yet on the the document save strategy. Basically there is two possible approaches here:

- Export way: The OnlyOffice session exports the document which is then written to a 3rd party storage. This is the solution used when integrating OnlyOffice with OwnCloud/NextCloud.
- Flow way: The content of the session (except the euphemeral events such as cursor movement) is continously saved by the server in a durable way. This is the solution used by Crytpad.

#### 3.1. Settle the save strategy

Once a document has been modified in the session, it must be saved in a durable way.

As said above, there is two possible approaches:

- Export way
- Flow way

A key point here is the document lifecycle:

- In export way, the document is stored as docx/pptx/etc. and has to be converted back and forth into bin format to be saved.
- In flow way, the document is converted once in bin format (similarly to what Google Drive does: when opening a document, a new copy of the document appear in the drive with same name but no extension, this is this document that is in bin format and is used in the edition session).
  Once the document has been converted, it is no longer stored as a file but as a initial document + events.

Both approaches have their own tradeof:

Pros for Flow way:

- Consume less space since only small events are stored (while export way requires the full document to be saved each time).
- Allows finer history (each event can be shown in the history). This is not a big pro though since having a history with a granularity of a couple of minutes is often already enough.
- Document conversion (which can be slow for big document and must run on the client side) occurs less often (basically only when a collaborative session is created, the data of the collorative session then *are* the document)
- Avoid modification conflict in the file since there is a single source of truth (aka the server handling the session) that orders the events and save them.

Pros for Export way:

- Better integration with the workspace since the document is an actual file and hence can be opened from the mountpoint
- Allow offline access and modification (since the document is an actual file that can be accessed from the mountpoint). Flow way could support offline access and modification, but this would requires more complexe code (e.g. need to store the data on local, need to deal with conflict due to offline modification)

On top of that, Parsec is designed to be hosted on a PASS (i.e. Heroku, Scalingo), hence it cannot just store data on its disk bust instead need to use a database (currently only using PostgreSQL for metadata and S3 for file chunks).

##### 3.2. (implemented) Investigate the communication protocol used between the OnlyOffice client and the server

See `notes/communication_protocol.md` for the write-up (message catalog, ephemeral/persistent
classification, session join/create flow, snapshot/ordering findings, and a real limitation found while
testing). Implemented in `client/public/onlyoffice-mock-server.js` (new) and wired into
`client/public/onlyoffice-host.html`, replacing the do-nothing `connectMockServer` stub from steps 1/2
with a real (client-side only, `localStorage` + `BroadcastChannel`, zero network) mock server and an
on-page debug panel logging every message. `client/src/services/onlyoffice.ts` and `FileEditor.vue` now
thread a stable `documentId` through so two tabs/users opening the same file join the same session.

In task 1 (Integration OnlyOffice client-side code in Parsec GUI) we have mocked the socketIO communication system so that the client never talks to the server.

Now in this step we want to modify this in order to simulate a server.

The goal here is to see the messages the client sends in order to know:

- Which one are euphemeral (e.g. cursor movement) and which one are not (e.g. text modification in the document). And if there is a way to confidently say if any given message type is euphemeral (for instance if all messages contains a flag, or if there is a small set of messages and we know all of them).
- How is handled new session (the client has to upload the initial document) and joining existing session (the client has to download the initial document, then the patches and apply them)
- How snapshot are handled (i.e. when a document has too many modification, a client is expected to send a full snapshot of the document so that joining users don't have to download all the patches, however already connected clients still prefere to receive a patch they can easily apply instead of the snapshot since it would mean downloading more data for them)
- How message ordering is handled. If multiple clients send a message concurrently, the server has order them (or maybe reject some of them) so that all the client can reach a common state for the document.
- How the client detects a session already exist (and do this check before reading the document from the storage in order to avoid reading it for nothing if the session already exists).

#### Validation condition

- Alice creates a new `Editor.docx` file in Workspace `wksp1`, this file contains some text (typically "Hello world" using Calibri font in size 14).
- Alice can open `Editor.docx` within Parsec GUI using OnlyOffice, the editor correctly displays the document text/font/size
- the message the client is supposed to send to the server are shown
- The mocked socketio don't actually send the request and instead return fake responses
- There is a mechanism to simulate messages send by other users (both euphemeral and non-euphemeral, e.g. text modification, cursor movement, user joining/leaving)
- No request are send to the server, the Parsec GUI doesn't try to save modified data back into Parsec

##### 3.3. Specify an architecture for the client/server communication

Once 3.1 and 3.2 are done we should have a better view at what OnlyOffice client requires.

On top of that there is some questions specific to Parsec end-to-end encrypted nature:

- How access control is achieved by the Parsec server (e.g. to prevent a user with READER access from sending an RPC containing a document modification).
  Typically should be done by having the server aware of all the OnlyOffice message types and which workspace role is required for each ones of them.
- How the message send by the client are encrypted so that the server doesn't have clear text access to the document.
  Typically should be done by using the workspace encryption key (so each RPC send a by client contains the index of the key used, the OnlyOffice message encrypted with the key, and some non sensitive info kept in clear for the server to check them).
- How workspace key rotation is handled. Typically the server check that the key index used in a RPC is the lastest (reject the RPC otherwise). Since each RPC also contains the key index, message encrypted with older keys (i.e. message from RPC that occured before the key rotation) can still be decrypted by Clients.

From this we should define the schema (request, reply):

- for the SSE connection (i.e. what parameter are passed and what content is received). Typically should be something like `GET /authenticated/{organization_id}/editics/{workspace_id}/{document_id}/join` and it should return 4xx/5xx HTTP status if something the request fails (e.g. session doesn't exist, user not allowed etc.).
- For the messages send by the server in the SSE connection
- for the RPC command. Typically a authenticated familly command named something like `editics_send_message`.
- Any other additional command that might be needed (typically to handle the save strategy)

### 4. Implementation of the OnlyOffice/Parsec communication and the save strategy

#### 4.1. Schema

TODO

#### 4.2. Server

TODO

#### 4.3. libparsec and GUI bindings

TODO

#### 4.4. GUI

TODO

#### Validation condition

- Alice creates a new `Editor.docx` file in Workspace `wksp1`, this file contains some text (typically "Hello world" using Calibri font in size 14).
- Bob sees the new `Editor.docx` file appearing in Workspace `wksp1` after a couple of seconds.
- Alice can open `Editor.docx` within Parsec GUI using OnlyOffice, the editor correctly displays the document text/font/size.
- Alice can modify the document from within the Parsec GUI (add text, change font etc.)
- Bob sees the modifications done by Alice.
- Bob cannot modify the document

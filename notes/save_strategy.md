## Overview

I'm considering integrating OnlyOffice in Parsec (so using it for collaborative edition of document in a end-to-end encrypted environment, this is very similar to what Cryptpad does)

Just like Cryptpad, I cannot use the OnlyOffice server (to stay end-to-end encrypted) and instead must re-implement the communication system in Parsec.
Basically this communication system consists of the server broadcasting to the clients in a session any message send by one of them.

On top of that, I haven't settled yet on the the document save strategy. Basically there is two possible approaches here:

- Export way: The OnlyOffice session exports the document which is then written to a 3rd party storage. This is the solution used when integrating OnlyOffice with OwnCloud/NextCloud.
- Flow way: The content of the session (except the euphemeral events such as cursor movement) is continously saved by the server in a durable way. This is the solution used by Crytpad.

## Pros and cons of each strategy

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

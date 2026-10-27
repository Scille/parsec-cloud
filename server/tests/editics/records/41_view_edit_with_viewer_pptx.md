## Info

- Timestamp: 2026-10-14T09:38:36.026Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: presentation.pptx (slide)
- Scenario:
  1. Alice (edit) connects.
  2. Bob joins in view mode.
  3. Alice moves her cursor, edits the document and sends a chat message.
  4. Bob sends a chat message back.

  NOTE: the cursor move emits nothing here: a viewer does not count toward the
  client-side edit-user count, so Alice stays in single-user mode and sends no
  cursor (the omission is part of the record); her save changes are on the
  contrary relayed to Bob (live viewer).

## Events

### 09:38:23.175       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/presentationeditor/main/../../../../doc/recmv7x37bfck60/c/?shardkey=recmv7x37bfck60&EIO=4&transport=websocket"
```

### 09:38:23.177   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "0L6bzNeKzt5tv_3iAAAy",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:38:23.187   <-  Alice           license

```json
{
  "type": "license",
  "license": {
    "type": 3,
    "light": false,
    "mode": 0,
    "rights": 1,
    "buildVersion": "9.3.0",
    "buildNumber": 138,
    "protectionSupport": true,
    "isAnonymousSupport": true,
    "liveViewerSupport": true,
    "branding": false,
    "customization": false,
    "advancedApi": false
  }
}
```

### 09:38:23.320   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv7x37bfck60",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 2,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 129,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmv7x37bfck60",
    "userid": "de10a11cec0010000000000000000000",
    "format": "pptx",
    "url": "http://host.containers.internal:33303/presentation.pptx",
    "title": "presentation.pptx",
    "nobase64": true,
    "outputformat": 8195,
    "convertToOrigin": ".pdf.xps.oxps.djvu"
  },
  "lang": "en",
  "mode": "edit",
  "permissions": {
    "edit": true
  },
  "encrypted": false,
  "IsAnonymousUser": false,
  "timezoneOffset": -120,
  "coEditingMode": "fast",
  "time": 437,
  "supportAuthChangesAck": true
}
```

### 09:38:23.333   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "m9w_w_8vteYxBwLgAAAz",
  "sessionTimeConnect": 1791970703187,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "m9w_w_8vteYxBwLgAAAz",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4MzdiZmNrNjAiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDcwMzE4N30sImlhdCI6MTc5MTk3MDcwMywiZXhwIjoxNzk0NTYyNzAzfQ.MxfloEouZUr2P_GyCYFMIDC1KmLcr9eZMzruEtTZyDo",
  "g_cAscSpellCheckUrl": "",
  "buildVersion": "9.3.0",
  "buildNumber": 138,
  "licenseType": 3,
  "settings": {
    "spellcheckerUrl": "",
    "reconnection": {
      "attempts": 50,
      "delay": 2000
    },
    "binaryChanges": false,
    "websocketMaxPayloadSize": 1572864,
    "maxChangesSize": 157286400,
    "limits_image_size": 26214400,
    "limits_image_types_upload": "jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif"
  },
  "openedAt": 1791977903324
}
```

### 09:38:23.444   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x37bfck60/Editor.bin/Editor.bin?md5=WJgotsnsT7yMzg13YeB4yg&expires=1794565296&shardkey=recmv7x37bfck60&filename=Editor.bin"
    },
    "openedAt": 1791977903324
  }
}
```

### 09:38:23.462   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:10"
}
```

### 09:38:23.558   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:95"
}
```

### 09:38:23.659   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 09:38:23.952   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:38:23.952   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1055 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":97400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:38:23.953   <-  Alice           message

```json
{
  "type": "message"
}
```

### 09:38:25.532       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/presentationeditor/main/../../../../doc/recmv7x37bfck60/c/?shardkey=recmv7x37bfck60&EIO=4&transport=websocket"
```

### 09:38:25.534   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "BPqMaoCxuxp3UDVhAAA0",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:38:25.570   <-  Bob             license

```json
{
  "type": "license",
  "license": {
    "type": 3,
    "light": false,
    "mode": 0,
    "rights": 1,
    "buildVersion": "9.3.0",
    "buildNumber": 138,
    "protectionSupport": true,
    "isAnonymousSupport": true,
    "liveViewerSupport": true,
    "branding": false,
    "customization": false,
    "advancedApi": false
  }
}
```

### 09:38:25.632   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv7x37bfck60",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10808c001000000000000000000000",
    "username": "Bob",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 2,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 129,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmv7x37bfck60",
    "userid": "de10808c001000000000000000000000",
    "format": "pptx",
    "url": "http://host.containers.internal:33303/presentation.pptx",
    "title": "presentation.pptx",
    "lcid": 9,
    "nobase64": true,
    "outputformat": 8195,
    "convertToOrigin": ".pdf.xps.oxps.djvu"
  },
  "lang": "en",
  "mode": "view",
  "permissions": {
    "edit": false
  },
  "encrypted": false,
  "IsAnonymousUser": false,
  "timezoneOffset": -120,
  "coEditingMode": "fast",
  "time": 632,
  "supportAuthChangesAck": true
}
```

### 09:38:25.642   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "GhEzWmtlLv0sBUK9AAA1",
  "sessionTimeConnect": 1791970705570,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "m9w_w_8vteYxBwLgAAAz",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "GhEzWmtlLv0sBUK9AAA1",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4MzdiZmNrNjAiLCJwZXJtaXNzaW9ucyI6eyJlZGl0IjpmYWxzZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTA4MDhjMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkJvYiIsImluZGV4IjoyfSwiY29FZGl0aW5nIjp7Im1vZGUiOiJmYXN0In0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDcwNTU3MH0sImlhdCI6MTc5MTk3MDcwNSwiZXhwIjoxNzk0NTYyNzA1fQ.3eS3szkUwoaNU62YDT0hHkK6XZCa5Xap_wFzCUP3u5U",
  "g_cAscSpellCheckUrl": "",
  "buildVersion": "9.3.0",
  "buildNumber": 138,
  "licenseType": 3,
  "settings": {
    "spellcheckerUrl": "",
    "reconnection": {
      "attempts": 50,
      "delay": 2000
    },
    "binaryChanges": false,
    "websocketMaxPayloadSize": 1572864,
    "maxChangesSize": 157286400,
    "limits_image_size": 26214400,
    "limits_image_types_upload": "jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif"
  },
  "openedAt": 1791977903324
}
```

### 09:38:25.642   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791970705637,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "m9w_w_8vteYxBwLgAAAz",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "GhEzWmtlLv0sBUK9AAA1",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 09:38:25.643   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x37bfck60/Editor.bin/Editor.bin?md5=WJgotsnsT7yMzg13YeB4yg&expires=1794565296&shardkey=recmv7x37bfck60&filename=Editor.bin"
    },
    "openedAt": 1791977903324
  }
}
```

### 09:38:25.651   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 09:38:25.707   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:56"
}
```

### 09:38:25.758   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 09:38:25.968   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 09:38:25.968   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:958 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":97400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:38:25.969   <-  Bob             message

```json
{
  "type": "message"
}
```

### 09:38:30.327   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 09:38:30.328   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 09:38:30.330   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAIAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgACQAAAA==\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 09:38:30.337   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7x37bfck60",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAIAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x37bfck60",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791970710000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 19,
  "syncChangesIndex": 19,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgACQAAAA==\"}"
}
```

### 09:38:30.337   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791970710000,
  "syncChangesIndex": 19
}
```

### 09:38:31.081   ->  Alice           message

```json
{
  "type": "message",
  "message": "Hello from Alice to the viewer"
}
```

### 09:38:31.082   <-  Alice           message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv7x37bfck60",
      "message": "Hello from Alice to the viewer",
      "time": 1791970711082,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 09:38:31.082   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv7x37bfck60",
      "message": "Hello from Alice to the viewer",
      "time": 1791970711082,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 09:38:31.531   ->  Bob             message

```json
{
  "type": "message",
  "message": "Hello from Bob the viewer"
}
```

### 09:38:31.532   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv7x37bfck60",
      "message": "Hello from Bob the viewer",
      "time": 1791970711531,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000",
      "username": "Bob"
    }
  ]
}
```

### 09:38:31.532   <-  Alice           message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv7x37bfck60",
      "message": "Hello from Bob the viewer",
      "time": 1791970711531,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000",
      "username": "Bob"
    }
  ]
}
```

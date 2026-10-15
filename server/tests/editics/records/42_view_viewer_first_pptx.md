## Info

- Timestamp: 2026-10-14T09:38:48.527Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: presentation.pptx (slide)
- Scenario:
  1. Bob (view) connects first: the session (and the document conversion) is
     created by a read-only participant.
  2. Alice (edit) joins: no waitAuth is involved (a viewer never holds the auth
     lock).
  3. Alice edits the document; her save changes are relayed to Bob (live
     viewer).

## Events

### 09:38:38.256       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/presentationeditor/main/../../../../doc/recmv7x3iw39ih0/c/?shardkey=recmv7x3iw39ih0&EIO=4&transport=websocket"
```

### 09:38:38.258   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "Sg4RVpv7TsryT9HPAAA2",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:38:38.265   <-  Bob             license

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

### 09:38:38.343   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv7x3iw39ih0",
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
    "id": "recmv7x3iw39ih0",
    "userid": "de10808c001000000000000000000000",
    "format": "pptx",
    "url": "http://host.containers.internal:33303/presentation.pptx",
    "title": "presentation.pptx",
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
  "time": 349,
  "supportAuthChangesAck": true
}
```

### 09:38:38.359   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "eEEunvFYI05QGqYrAAA3",
  "sessionTimeConnect": 1791970718264,
  "participants": [
    {
      "id": "de10808c0010000000000000000000001",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 1,
      "view": true,
      "connectionId": "eEEunvFYI05QGqYrAAA3",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4M2l3MzlpaDAiLCJwZXJtaXNzaW9ucyI6eyJlZGl0IjpmYWxzZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTA4MDhjMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkJvYiIsImluZGV4IjoxfSwiY29FZGl0aW5nIjp7Im1vZGUiOiJmYXN0In0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDcxODI2NH0sImlhdCI6MTc5MTk3MDcxOCwiZXhwIjoxNzk0NTYyNzE4fQ.X-iTXnQocxFZfQJvX47C-uWnxaViTCbf5Omos1Cked0",
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
  "openedAt": 1791977918350
}
```

### 09:38:38.458   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x3iw39ih0/Editor.bin/Editor.bin?md5=sp5dW_oJ4MVAfnI0EQ4r_Q&expires=1794565311&shardkey=recmv7x3iw39ih0&filename=Editor.bin"
    },
    "openedAt": 1791977918350
  }
}
```

### 09:38:38.531   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:71"
}
```

### 09:38:38.620   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:89"
}
```

### 09:38:38.721   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 09:38:38.994   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 09:38:38.994   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:988 memory:{\"totalJSHeapSize\":109000000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:38:38.995   <-  Bob             message

```json
{
  "type": "message"
}
```

### 09:38:40.059       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/presentationeditor/main/../../../../doc/recmv7x3iw39ih0/c/?shardkey=recmv7x3iw39ih0&EIO=4&transport=websocket"
```

### 09:38:40.061   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "stg3h9-Xs-4she5YAAA4",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:38:40.091   <-  Alice           license

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

### 09:38:40.187   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv7x3iw39ih0",
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
    "id": "recmv7x3iw39ih0",
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
  "time": 282,
  "supportAuthChangesAck": true
}
```

### 09:38:40.200   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "RnQQAmN7zh5nsBLSAAA5",
  "sessionTimeConnect": 1791970720091,
  "participants": [
    {
      "id": "de10808c0010000000000000000000001",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 1,
      "view": true,
      "connectionId": "eEEunvFYI05QGqYrAAA3",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000002",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 2,
      "view": false,
      "connectionId": "RnQQAmN7zh5nsBLSAAA5",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4M2l3MzlpaDAiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6Mn0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDcyMDA5MX0sImlhdCI6MTc5MTk3MDcyMCwiZXhwIjoxNzk0NTYyNzIwfQ.r37lXrrrTMYsOYoYqBbeOU7viKPYHDooZqFCURkvF4s",
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
  "openedAt": 1791977918350
}
```

### 09:38:40.200   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791970720192,
  "participants": [
    {
      "id": "de10808c0010000000000000000000001",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 1,
      "view": true,
      "connectionId": "eEEunvFYI05QGqYrAAA3",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000002",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 2,
      "view": false,
      "connectionId": "RnQQAmN7zh5nsBLSAAA5",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 09:38:40.200   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x3iw39ih0/Editor.bin/Editor.bin?md5=sp5dW_oJ4MVAfnI0EQ4r_Q&expires=1794565311&shardkey=recmv7x3iw39ih0&filename=Editor.bin"
    },
    "openedAt": 1791977918350
  }
}
```

### 09:38:40.586   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:376"
}
```

### 09:38:40.647   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:61"
}
```

### 09:38:40.699   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:52"
}
```

### 09:38:40.914   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:38:40.914   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:999 memory:{\"totalJSHeapSize\":109000000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:38:40.915   <-  Alice           message

```json
{
  "type": "message"
}
```

### 09:38:43.700   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 09:38:43.700   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 09:38:43.702   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAIAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000002\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgACQAAAA==\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 09:38:43.707   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAIAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x3iw39ih0",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791970723000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 19,
  "syncChangesIndex": 19,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000002\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgACQAAAA==\"}"
}
```

### 09:38:43.708   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791970723000,
  "syncChangesIndex": 19
}
```

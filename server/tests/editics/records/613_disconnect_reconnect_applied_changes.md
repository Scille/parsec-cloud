## Info

- Timestamp: 2026-10-21T16:17:32.051Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document.
  2. Bob joins: Alice and Bob edit together.
  3. Bob edits the document: Alice is idle, so she receives and applies his change — her client then keeps his change time as its lastOtherSaveTime.
  4. Alice loses her connection (the websocket is closed without a close frame).
  5. The document is not modified while she is away.
  6. Alice's connection comes back: her editor sends its auth with the previous sessionId and a lastOtherSaveTime set to Bob's change time (not -1: she had applied his change before losing the connection).
  7. The document server restores her edition session: the auth response and the lock table follow, with no authChanges (nothing changed while she was away).

## Events

### 16:17:19.085       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-007b17937481113b3c4c5d8fba08c2de/web-apps/apps/documenteditor/main/../../../../doc/recmvibf6zwvbml/c/?shardkey=recmvibf6zwvbml&EIO=4&transport=websocket"
```

### 16:17:19.089   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "kbixNVC_j-wkN8W3AAAC",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 16:17:19.111   <-  Alice           license

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

### 16:17:19.254   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvibf6zwvbml",
  "documentCallbackUrl": "http://host.containers.internal:39303/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 0,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 65,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmvibf6zwvbml",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:39303/document.docx",
    "title": "document.docx",
    "nobase64": true,
    "outputformat": 8193,
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
  "headingsColor": null,
  "coEditingMode": "fast",
  "time": 497,
  "supportAuthChangesAck": true
}
```

### 16:17:19.276   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "m9eo1HG_WkUehfvnAAAD",
  "sessionTimeConnect": 1792599439111,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "m9eo1HG_WkUehfvnAAAD",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmliZjZ6d3ZibWwiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjU5OTQzOTExMX0sImlhdCI6MTc5MjU5OTQzOSwiZXhwIjoxNzk1MTkxNDM5fQ.NypQCMzQLKX2-oK_RQq_iA_nL3GdhwmFSYh1V-UQk4A",
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
  "openedAt": 1792606639264
}
```

### 16:17:19.356   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvibf6zwvbml/Editor.bin/Editor.bin?md5=n1CZIIgGjNV1P2ieNCZ1qQ&expires=1795194032&shardkey=recmvibf6zwvbml&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmvibf6zwvbml/media/image1.png/image1.png?md5=3ULRl0iCH63YmaTBBHYLUw&expires=1795194032&shardkey=recmvibf6zwvbml&filename=image1.png"
    },
    "openedAt": 1792606639264
  }
}
```

### 16:17:19.361   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:4"
}
```

### 16:17:19.453   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:90"
}
```

### 16:17:19.555   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:103"
}
```

### 16:17:19.782   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 16:17:19.782   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:836 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 16:17:19.783   <-  Alice           message

```json
{
  "type": "message"
}
```

### 16:17:20.980       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-007b17937481113b3c4c5d8fba08c2de/web-apps/apps/documenteditor/main/../../../../doc/recmvibf6zwvbml/c/?shardkey=recmvibf6zwvbml&EIO=4&transport=websocket"
```

### 16:17:20.982   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "grOmmC_JzfoGORrWAAAE",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 16:17:21.008   <-  Bob             license

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

### 16:17:21.089   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmvibf6zwvbml",
  "documentCallbackUrl": "http://host.containers.internal:39303/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10808c001000000000000000000000",
    "username": "Bob",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 0,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 65,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmvibf6zwvbml",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:39303/document.docx",
    "title": "document.docx",
    "nobase64": true,
    "outputformat": 8193,
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
  "headingsColor": null,
  "coEditingMode": "fast",
  "time": 272,
  "supportAuthChangesAck": true
}
```

### 16:17:21.099   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "m9eo1HG_WkUehfvnAAAD",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 16:17:21.099   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792599441097,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "m9eo1HG_WkUehfvnAAAD",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "1bLibkchbaXt2gu6AAAF",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 16:17:21.100   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 16:17:21.101   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvibf6zwvbml/Editor.bin/Editor.bin?md5=n1CZIIgGjNV1P2ieNCZ1qQ&expires=1795194032&shardkey=recmvibf6zwvbml&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmvibf6zwvbml/media/image1.png/image1.png?md5=3ULRl0iCH63YmaTBBHYLUw&expires=1795194032&shardkey=recmvibf6zwvbml&filename=image1.png"
    },
    "openedAt": 1792606639264
  }
}
```

### 16:17:21.108   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "1bLibkchbaXt2gu6AAAF",
  "sessionTimeConnect": 1792599441008,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "m9eo1HG_WkUehfvnAAAD",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "1bLibkchbaXt2gu6AAAF",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmliZjZ6d3ZibWwiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTI1OTk0NDEwMDh9LCJpYXQiOjE3OTI1OTk0NDEsImV4cCI6MTc5NTE5MTQ0MX0.iPb35Y5Qf7XBvwTVlGo8BIfMTWMYHuMvmTowTksGuds",
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
  }
}
```

### 16:17:21.111   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:3"
}
```

### 16:17:21.117   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 16:17:21.118   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792599441117,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 16:17:21.571   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:46"
}
```

### 16:17:21.675   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:104"
}
```

### 16:17:21.822   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 16:17:21.822   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:889 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 16:17:21.823   <-  Bob             message

```json
{
  "type": "message"
}
```

### 16:17:21.854   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 16:17:21.855   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792599441855,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 16:17:22.876   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAIAAAA="
}
```

### 16:17:22.877   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1792599442877,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 16:17:23.356   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 16:17:23.357   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.359   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.366   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 2,
  "syncChangesIndex": 2,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}"
}
```

### 16:17:23.366   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792599443000,
  "syncChangesIndex": 2
}
```

### 16:17:23.396   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 2
}
```

### 16:17:23.397   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.401   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:23.402   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.407   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 4,
  "syncChangesIndex": 4,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}"
}
```

### 16:17:23.407   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 2,
  "time": 1792599443000,
  "syncChangesIndex": 4
}
```

### 16:17:23.436   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 4
}
```

### 16:17:23.438   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.440   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.445   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 6,
  "syncChangesIndex": 6,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}"
}
```

### 16:17:23.446   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 4,
  "time": 1792599443000,
  "syncChangesIndex": 6
}
```

### 16:17:23.476   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 6
}
```

### 16:17:23.477   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.478   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.482   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 8,
  "syncChangesIndex": 8,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}"
}
```

### 16:17:23.482   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 6,
  "time": 1792599443000,
  "syncChangesIndex": 8
}
```

### 16:17:23.539   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 8
}
```

### 16:17:23.539   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.549   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.553   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 10,
  "syncChangesIndex": 10,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}"
}
```

### 16:17:23.554   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 8,
  "time": 1792599443000,
  "syncChangesIndex": 10
}
```

### 16:17:23.596   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 10
}
```

### 16:17:23.597   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.599   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:23.600   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.604   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 12,
  "syncChangesIndex": 12,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}"
}
```

### 16:17:23.604   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 10,
  "time": 1792599443000,
  "syncChangesIndex": 12
}
```

### 16:17:23.636   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 12
}
```

### 16:17:23.637   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.639   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:23.641   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAkAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.644   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 14,
  "syncChangesIndex": 14,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAkAAAA=\"}"
}
```

### 16:17:23.644   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 12,
  "time": 1792599443000,
  "syncChangesIndex": 14
}
```

### 16:17:23.676   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 14
}
```

### 16:17:23.677   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.679   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:23.680   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.684   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 16,
  "syncChangesIndex": 16,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}"
}
```

### 16:17:23.684   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 14,
  "time": 1792599443000,
  "syncChangesIndex": 16
}
```

### 16:17:23.719   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 16
}
```

### 16:17:23.720   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.721   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:23.722   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAsAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.727   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 18,
  "syncChangesIndex": 18,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAsAAAA=\"}"
}
```

### 16:17:23.727   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 16,
  "time": 1792599443000,
  "syncChangesIndex": 18
}
```

### 16:17:23.769   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 18
}
```

### 16:17:23.769   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.771   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:23.773   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.777   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 20,
  "syncChangesIndex": 20,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}"
}
```

### 16:17:23.777   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 18,
  "time": 1792599443000,
  "syncChangesIndex": 20
}
```

### 16:17:23.798   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:23.836   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 20
}
```

### 16:17:23.837   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.838   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.842   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 22,
  "syncChangesIndex": 22,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}"
}
```

### 16:17:23.842   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 20,
  "time": 1792599443000,
  "syncChangesIndex": 22
}
```

### 16:17:23.876   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 22
}
```

### 16:17:23.877   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.879   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:23.880   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.884   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 24,
  "syncChangesIndex": 24,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}"
}
```

### 16:17:23.884   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 22,
  "time": 1792599443000,
  "syncChangesIndex": 24
}
```

### 16:17:23.919   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 24
}
```

### 16:17:23.919   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.921   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:23.923   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.927   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 26,
  "syncChangesIndex": 26,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}"
}
```

### 16:17:23.927   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 24,
  "time": 1792599443000,
  "syncChangesIndex": 26
}
```

### 16:17:23.968   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 26
}
```

### 16:17:23.969   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:23.970   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:23.972   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:23.976   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1792599443000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 28,
  "syncChangesIndex": 28,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}"
}
```

### 16:17:23.976   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 26,
  "time": 1792599443000,
  "syncChangesIndex": 28
}
```

### 16:17:23.998   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:24.036   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 28
}
```

### 16:17:24.037   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:24.038   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:24.042   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599444000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1792599444000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 30,
  "syncChangesIndex": 30,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}"
}
```

### 16:17:24.042   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 28,
  "time": 1792599444000,
  "syncChangesIndex": 30
}
```

### 16:17:24.076   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 30
}
```

### 16:17:24.077   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:24.078   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:24.080   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:24.084   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599444000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1792599444000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 32,
  "syncChangesIndex": 32,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}"
}
```

### 16:17:24.084   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 30,
  "time": 1792599444000,
  "syncChangesIndex": 32
}
```

### 16:17:24.118   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 32
}
```

### 16:17:24.119   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:24.120   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:24.122   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABMAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:24.126   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599444000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1792599444000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 34,
  "syncChangesIndex": 34,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABMAAAA=\"}"
}
```

### 16:17:24.126   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 32,
  "time": 1792599444000,
  "syncChangesIndex": 34
}
```

### 16:17:24.168   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 34
}
```

### 16:17:24.169   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:24.170   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:24.173   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 16:17:24.177   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599444000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibf6zwvbml",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1792599444000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 36,
  "syncChangesIndex": 36,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}"
}
```

### 16:17:24.177   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 34,
  "time": 1792599444000,
  "syncChangesIndex": 36
}
```

### 16:17:24.198   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:26.918       Alice           [Transport WebSocket] ws-close

```json
"monitor close, code 1006 (connection cut)"
```

### 16:17:26.920   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792599446920,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "1bLibkchbaXt2gu6AAAF",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 16:17:28.850       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-007b17937481113b3c4c5d8fba08c2de/web-apps/apps/documenteditor/main/../../../../doc/recmvibf6zwvbml/c/?shardkey=recmvibf6zwvbml&EIO=4&transport=websocket"
```

### 16:17:28.852   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "tk7UZeUKCWg-xtx5AAAG",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 16:17:28.854   <-  Alice           license

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

### 16:17:28.860   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvibf6zwvbml",
  "documentCallbackUrl": "http://host.containers.internal:39303/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "firstname": null,
    "lastname": null,
    "indexUser": 1
  },
  "editorType": 0,
  "lastOtherSaveTime": 1792599444000,
  "block": [],
  "sessionId": "m9eo1HG_WkUehfvnAAAD",
  "sessionTimeConnect": 1792599439111,
  "sessionTimeIdle": 1908,
  "documentFormatSave": 65,
  "isCloseCoAuthoring": false,
  "lang": "en",
  "mode": "edit",
  "permissions": {
    "edit": true
  },
  "encrypted": false,
  "IsAnonymousUser": false,
  "timezoneOffset": -120,
  "headingsColor": null,
  "coEditingMode": "fast",
  "jwtSession": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmliZjZ6d3ZibWwiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjU5OTQzOTExMX0sImlhdCI6MTc5MjU5OTQzOSwiZXhwIjoxNzk1MTkxNDM5fQ.NypQCMzQLKX2-oK_RQq_iA_nL3GdhwmFSYh1V-UQk4A",
  "time": 10104,
  "supportAuthChangesAck": true
}
```

### 16:17:28.867   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {}
}
```

### 16:17:28.868   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {}
}
```

### 16:17:28.868   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "m9eo1HG_WkUehfvnAAAD",
  "sessionTimeConnect": 1792599439111,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "1bLibkchbaXt2gu6AAAF",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "nE0Wvxcix4DhADArAAAH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
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
  }
}
```

### 16:17:28.868   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792599448867,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "1bLibkchbaXt2gu6AAAF",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "nE0Wvxcix4DhADArAAAH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 16:17:28.884   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

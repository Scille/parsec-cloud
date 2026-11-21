## Info

- Timestamp: 2026-11-20T15:36:42.414Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document and types while alone.
  2. Bob joins: the auth lock holder (Alice) has the `connectState`
     announcing him delayed (a slow transport), so Bob stays parked
     behind her auth lock (`waitAuth`).
  3. Alice keeps editing: her changes are saved to the session while
     Bob is still parked. The capture shows whether the document
     server broadcasts those live saves to the parked connection.
  4. The delayed `connectState` is delivered to Alice: her editor
     releases the auth lock (`unLockDocument{unlock:true}`), Bob
     completes his handshake (backlog + auth).
  5. Bob types in turn and the session settles.

## Events

### 15:36:04.471       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-561c72c7259962ec0b09bebad8b4d658/web-apps/apps/documenteditor/main/../../../../doc/drivemwp55pjz-word/c/?shardkey=drivemwp55pjz-word&EIO=4&transport=websocket"
```

### 15:36:04.474   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "3Z_j8ZkXtA-vYKi0AAAc",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:36:04.500   <-  Alice           license

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

### 15:36:04.623   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "drivemwp55pjz-word",
  "documentCallbackUrl": "http://host.containers.internal:8091/callback",
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
    "id": "drivemwp55pjz-word",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:8091/document.docx",
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
  "timezoneOffset": -60,
  "headingsColor": null,
  "coEditingMode": "fast",
  "time": 476,
  "supportAuthChangesAck": true
}
```

### 15:36:04.637   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "wmuf6ahqzJti0qriAAAd",
  "sessionTimeConnect": 1795188964499,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "wmuf6ahqzJti0qriAAAd",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJkcml2ZW13cDU1cGp6LXdvcmQiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5NTE4ODk2NDQ5OX0sImlhdCI6MTc5NTE4ODk2NCwiZXhwIjoxNzk3NzgwOTY0fQ.vHS9SXgXsJoqMb9D3QD-796VfMM2pkWCU_k-QZe9WDQ",
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
    "websocketMaxPayloadSize": 4096,
    "maxChangesSize": 157286400,
    "limits_image_size": 26214400,
    "limits_image_types_upload": "jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif"
  },
  "openedAt": 1795192564629
}
```

### 15:36:04.692   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/drivemwp55pjz-word/Editor.bin/Editor.bin?md5=scb6fzaScch2RVg6E4aX8w&expires=1797783557&shardkey=drivemwp55pjz-word&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/drivemwp55pjz-word/media/image1.png/image1.png?md5=SQN-FAd9-IqPmuCiQ0V1Wg&expires=1797783557&shardkey=drivemwp55pjz-word&filename=image1.png"
    },
    "openedAt": 1795192564629
  }
}
```

### 15:36:04.733   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:16"
}
```

### 15:36:04.814   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:80"
}
```

### 15:36:04.914   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 15:36:05.120   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:36:05.120   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:802 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:36:05.120   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:36:15.316   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:36:15.317   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:36:15.318   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAAAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAQAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAAAxAAAAAAMAAAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAAwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 15:36:15.323   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1795188975000,
  "syncChangesIndex": 5
}
```

### 15:36:18.877       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-561c72c7259962ec0b09bebad8b4d658/web-apps/apps/documenteditor/main/../../../../doc/drivemwp55pjz-word/c/?shardkey=drivemwp55pjz-word&EIO=4&transport=websocket"
```

### 15:36:18.879 <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "MtORrWCf3VrCYOE4AAAe",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:36:18.907 ->  Bob             [Transport WebSocket]

```json
{
  "raw": "40{\"data\":{\"type\":\"auth\",\"docid\":\"drivemwp55pjz-word\",\"documentCallbackUrl\":\"htt"
}
```

### 15:36:18.932 <-  Bob             [Transport WebSocket]

```json
{
  "raw": "40{\"sid\":\"gmEqT_7NCi8gsHddAAAf\"}"
}
```

### 15:36:18.933 <-  Bob             license

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

### 15:36:19.056 ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "drivemwp55pjz-word",
  "documentCallbackUrl": "http://host.containers.internal:8091/callback",
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
    "id": "drivemwp55pjz-word",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:8091/document.docx",
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
  "timezoneOffset": -60,
  "headingsColor": null,
  "coEditingMode": "fast",
  "time": 351,
  "supportAuthChangesAck": true
}
```

### 15:36:19.094 <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "wmuf6ahqzJti0qriAAAd",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:36:19.094 <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1795188979093,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "wmuf6ahqzJti0qriAAAd",
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
      "connectionId": "gmEqT_7NCi8gsHddAAAf",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:36:19.095 <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/drivemwp55pjz-word/Editor.bin/Editor.bin?md5=scb6fzaScch2RVg6E4aX8w&expires=1797783557&shardkey=drivemwp55pjz-word&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/drivemwp55pjz-word/media/image1.png/image1.png?md5=SQN-FAd9-IqPmuCiQ0V1Wg&expires=1797783557&shardkey=drivemwp55pjz-word&filename=image1.png"
    },
    "openedAt": 1795192564629
  }
}
```

### 15:36:19.441 ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 15:36:19.494 ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:52"
}
```

### 15:36:19.544 ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 15:36:25.756 ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 5
}
```

### 15:36:25.797 <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:36:25.823 ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAABAAAAAQAAAAEAAAAFAAAABQAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAAAyAAAAAAMAAAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": 5,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 15:36:25.855 <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAABAAAAAQAAAAEAAAAFAAAABQAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1795188985000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188985000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188985000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAAAyAAAAAAMAAAA=\"",
      "time": 1795188985000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1795188985000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 10,
  "syncChangesIndex": 10,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}"
}
```

### 15:36:25.855 <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": 1795188985000,
  "syncChangesIndex": 10
}
```

### 15:36:28.156 ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 10
}
```

### 15:36:28.177 <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:36:28.202 ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAACAAAAAgAAAAMAAAAKAAAACgAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAAAzAAAAAAMAAAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAACwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAADAAAAAgAAAAMAAAAKAAAACgAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAAA0AAAAAAMAAAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": 10,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 15:36:28.235 <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAACAAAAAgAAAAMAAAAKAAAACgAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAAAzAAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAACwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAADAAAAAgAAAAMAAAAKAAAACgAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAAA0AAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 20,
  "syncChangesIndex": 20,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}"
}
```

### 15:36:28.235 <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": 1795188988000,
  "syncChangesIndex": 20
}
```

### 15:36:33.374   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": 20
}
```

### 15:36:33.381   <-  Bob             authChanges

```json
{
  "type": "authChanges",
  "changes": [
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1795188975000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188975000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAQAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188975000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAAAxAAAAAAMAAAA=\"",
      "time": 1795188975000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAAwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1795188975000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAABAAAAAQAAAAEAAAAFAAAABQAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1795188985000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188985000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188985000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAAAyAAAAAAMAAAA=\"",
      "time": 1795188985000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1795188985000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAACAAAAAgAAAAMAAAAKAAAACgAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAAAzAAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAACwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAQAAAADAAAAAgAAAAMAAAAKAAAACgAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAAA0AAAAAAMAAAA=\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemwp55pjz-word",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1795188988000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:36:33.382   ->  Bob             authChangesAck

```json
{
  "type": "authChangesAck"
}
```

### 15:36:33.396   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3ABAAAAA="
}
```

### 15:36:33.396   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3ABAAAAA=",
      "time": 1795188993396,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:36:33.582   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "gmEqT_7NCi8gsHddAAAf",
  "sessionTimeConnect": 1795188978932,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "wmuf6ahqzJti0qriAAAd",
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
      "connectionId": "gmEqT_7NCi8gsHddAAAf",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJkcml2ZW13cDU1cGp6LXdvcmQiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTUxODg5Nzg5MzJ9LCJpYXQiOjE3OTUxODg5OTMsImV4cCI6MTc5Nzc4MDk5M30.MtTmO_XAi3gJiAz3YbtyR-soC34NnPgnaxKJpfmVLT0",
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
    "websocketMaxPayloadSize": 4096,
    "maxChangesSize": 157286400,
    "limits_image_size": 26214400,
    "limits_image_types_upload": "jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif"
  }
}
```

### 15:36:33.587   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onApplyChanges time:3"
}
```

### 15:36:33.745   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:36:33.745   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:14912 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:36:33.745   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:36:33.785   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:36:33.786   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1795188993785,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

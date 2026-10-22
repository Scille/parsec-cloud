## Info

- Timestamp: 2026-10-21T16:18:09.191Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document.
  2. Bob joins: Alice and Bob edit together.
  3. Bob edits the document: Alice is idle, so she receives and applies his change — her client then keeps his change time as its lastOtherSaveTime.
  4. Alice loses her connection (the websocket is closed without a close frame).
  5. Bob edits the document again while Alice is away.
  6. Alice's connection comes back: her editor sends its auth with the previous sessionId and a lastOtherSaveTime set to Bob's change time (not -1: she had applied his first change before losing the connection).
  7. The document changed while she was away, so the document server refuses the restore with error 4010 "Restore error. Document modified.", and Alice's client answers by closing the co-authoring session.

## Events

### 16:17:34.192       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-007b17937481113b3c4c5d8fba08c2de/web-apps/apps/documenteditor/main/../../../../doc/recmvibfiplq6wh/c/?shardkey=recmvibfiplq6wh&EIO=4&transport=websocket"
```

### 16:17:34.194   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "7MDOcTwVnc1dga3cAAAI",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 16:17:34.200   <-  Alice           license

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

### 16:17:34.317   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvibfiplq6wh",
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
    "id": "recmvibfiplq6wh",
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
  "time": 416,
  "supportAuthChangesAck": true
}
```

### 16:17:34.334   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "h1SkrTobP0jNMVpLAAAJ",
  "sessionTimeConnect": 1792599454200,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "h1SkrTobP0jNMVpLAAAJ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmliZmlwbHE2d2giLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjU5OTQ1NDIwMH0sImlhdCI6MTc5MjU5OTQ1NCwiZXhwIjoxNzk1MTkxNDU0fQ.SF9OtgKrsq6hbWf2IF8M-KX1rPud_ZQ2blWl0ommuKM",
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
  "openedAt": 1792606654324
}
```

### 16:17:34.398   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvibfiplq6wh/Editor.bin/Editor.bin?md5=RGkNrcuU4Es2HHtGINzLPQ&expires=1795194047&shardkey=recmvibfiplq6wh&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmvibfiplq6wh/media/image1.png/image1.png?md5=HoqljTqgsBo5y2aq-LaVhw&expires=1795194047&shardkey=recmvibfiplq6wh&filename=image1.png"
    },
    "openedAt": 1792606654324
  }
}
```

### 16:17:34.408   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:7"
}
```

### 16:17:34.558   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:83"
}
```

### 16:17:34.658   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 16:17:34.873   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 16:17:34.873   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:797 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 16:17:34.874   <-  Alice           message

```json
{
  "type": "message"
}
```

### 16:17:36.433       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-007b17937481113b3c4c5d8fba08c2de/web-apps/apps/documenteditor/main/../../../../doc/recmvibfiplq6wh/c/?shardkey=recmvibfiplq6wh&EIO=4&transport=websocket"
```

### 16:17:36.435   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "972P87mC020dQU_pAAAK",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 16:17:36.461   <-  Bob             license

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

### 16:17:36.532   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmvibfiplq6wh",
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
    "id": "recmvibfiplq6wh",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:39303/document.docx",
    "title": "document.docx",
    "lcid": 9,
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
  "time": 634,
  "supportAuthChangesAck": true
}
```

### 16:17:36.541   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "h1SkrTobP0jNMVpLAAAJ",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 16:17:36.541   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792599456539,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "h1SkrTobP0jNMVpLAAAJ",
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
      "connectionId": "uA447cq1HZbuK_YVAAAL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 16:17:36.542   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 16:17:36.542   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvibfiplq6wh/Editor.bin/Editor.bin?md5=RGkNrcuU4Es2HHtGINzLPQ&expires=1795194047&shardkey=recmvibfiplq6wh&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmvibfiplq6wh/media/image1.png/image1.png?md5=HoqljTqgsBo5y2aq-LaVhw&expires=1795194047&shardkey=recmvibfiplq6wh&filename=image1.png"
    },
    "openedAt": 1792606654324
  }
}
```

### 16:17:36.548   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:1"
}
```

### 16:17:36.548   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "uA447cq1HZbuK_YVAAAL",
  "sessionTimeConnect": 1792599456461,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "h1SkrTobP0jNMVpLAAAJ",
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
      "connectionId": "uA447cq1HZbuK_YVAAAL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmliZmlwbHE2d2giLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTI1OTk0NTY0NjF9LCJpYXQiOjE3OTI1OTk0NTYsImV4cCI6MTc5NTE5MTQ1Nn0.BKdkN3BHuSFLwwhVVEVNNo21rKdOQyXNcoAgtyq8ACE",
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

### 16:17:36.597   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:48"
}
```

### 16:17:36.605   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 16:17:36.606   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792599456606,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 16:17:36.701   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:105"
}
```

### 16:17:36.844   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 16:17:36.845   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:831 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 16:17:36.845   <-  Bob             message

```json
{
  "type": "message"
}
```

### 16:17:36.880   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 16:17:36.881   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792599456880,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 16:17:37.903   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAIAAAA="
}
```

### 16:17:37.903   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1792599457903,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 16:17:38.343   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 16:17:38.344   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.345   ->  Bob             saveChanges

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

### 16:17:38.354   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.355   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792599458000,
  "syncChangesIndex": 2
}
```

### 16:17:38.384   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:38.389   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 2
}
```

### 16:17:38.390   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.392   ->  Bob             saveChanges

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

### 16:17:38.396   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.396   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 2,
  "time": 1792599458000,
  "syncChangesIndex": 4
}
```

### 16:17:38.423   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 4
}
```

### 16:17:38.424   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.425   ->  Bob             saveChanges

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

### 16:17:38.429   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.429   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 4,
  "time": 1792599458000,
  "syncChangesIndex": 6
}
```

### 16:17:38.463   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 6
}
```

### 16:17:38.463   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.464   ->  Bob             saveChanges

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

### 16:17:38.477   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.478   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 6,
  "time": 1792599458000,
  "syncChangesIndex": 8
}
```

### 16:17:38.503   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 8
}
```

### 16:17:38.504   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.516   ->  Bob             saveChanges

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

### 16:17:38.520   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.520   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 8,
  "time": 1792599458000,
  "syncChangesIndex": 10
}
```

### 16:17:38.550   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 10
}
```

### 16:17:38.551   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.555   ->  Bob             saveChanges

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

### 16:17:38.559   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1792599458000,
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

### 16:17:38.559   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 10,
  "time": 1792599458000,
  "syncChangesIndex": 12
}
```

### 16:17:38.582   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:38.623   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 12
}
```

### 16:17:38.623   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.624   ->  Bob             saveChanges

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

### 16:17:38.628   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.628   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 12,
  "time": 1792599458000,
  "syncChangesIndex": 14
}
```

### 16:17:38.662   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:38.666   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 14
}
```

### 16:17:38.666   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.673   ->  Bob             saveChanges

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

### 16:17:38.678   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.678   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 14,
  "time": 1792599458000,
  "syncChangesIndex": 16
}
```

### 16:17:38.704   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:38.708   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 16
}
```

### 16:17:38.708   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.709   ->  Bob             saveChanges

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

### 16:17:38.712   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.713   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 16,
  "time": 1792599458000,
  "syncChangesIndex": 18
}
```

### 16:17:38.753   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:38.754   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 18
}
```

### 16:17:38.754   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.755   ->  Bob             saveChanges

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

### 16:17:38.759   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.759   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 18,
  "time": 1792599458000,
  "syncChangesIndex": 20
}
```

### 16:17:38.782   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:38.801   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 20
}
```

### 16:17:38.802   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.802   ->  Bob             saveChanges

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

### 16:17:38.806   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1792599458000,
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

### 16:17:38.806   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 20,
  "time": 1792599458000,
  "syncChangesIndex": 22
}
```

### 16:17:38.825   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:38.863   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 22
}
```

### 16:17:38.863   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.864   ->  Bob             saveChanges

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

### 16:17:38.869   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.869   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 22,
  "time": 1792599458000,
  "syncChangesIndex": 24
}
```

### 16:17:38.905   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:38.906   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 24
}
```

### 16:17:38.907   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.907   ->  Bob             saveChanges

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

### 16:17:38.911   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.911   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 24,
  "time": 1792599458000,
  "syncChangesIndex": 26
}
```

### 16:17:38.942   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:38.945   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 26
}
```

### 16:17:38.946   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.946   ->  Bob             saveChanges

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

### 16:17:38.950   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.950   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 26,
  "time": 1792599458000,
  "syncChangesIndex": 28
}
```

### 16:17:38.991   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:38.991   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 28
}
```

### 16:17:38.992   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:38.992   ->  Bob             saveChanges

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

### 16:17:38.996   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599458000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1792599458000,
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

### 16:17:38.996   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 28,
  "time": 1792599458000,
  "syncChangesIndex": 30
}
```

### 16:17:39.039   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:39.040   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 30
}
```

### 16:17:39.040   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:39.041   ->  Bob             saveChanges

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

### 16:17:39.045   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599459000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1792599459000,
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

### 16:17:39.045   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 30,
  "time": 1792599459000,
  "syncChangesIndex": 32
}
```

### 16:17:39.061   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:39.074   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 32
}
```

### 16:17:39.075   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:39.075   ->  Bob             saveChanges

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

### 16:17:39.079   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599459000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1792599459000,
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

### 16:17:39.079   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 32,
  "time": 1792599459000,
  "syncChangesIndex": 34
}
```

### 16:17:39.103   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:39.143   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 34
}
```

### 16:17:39.143   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:39.144   ->  Bob             saveChanges

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

### 16:17:39.155   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792599459000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvibfiplq6wh",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1792599459000,
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

### 16:17:39.155   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 34,
  "time": 1792599459000,
  "syncChangesIndex": 36
}
```

### 16:17:39.182   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 16:17:42.016       Alice           [Transport WebSocket] ws-close

```json
"monitor close, code 1006 (connection cut)"
```

### 16:17:42.018   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792599462017,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "uA447cq1HZbuK_YVAAAL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 16:17:44.703   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 36
}
```

### 16:17:44.703   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 16:17:44.704   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAYAAAAAAAAAAAAAAAMAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAMAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAcAAAACAAAAAAAAAAMAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAMAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 16:17:44.746   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 36,
  "time": 1792599464000,
  "syncChangesIndex": 55
}
```

### 16:18:05.978       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-007b17937481113b3c4c5d8fba08c2de/web-apps/apps/documenteditor/main/../../../../doc/recmvibfiplq6wh/c/?shardkey=recmvibfiplq6wh&EIO=4&transport=websocket"
```

### 16:18:05.980   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "O-M22nny4lVyD3YOAAAM",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 16:18:05.981   <-  Alice           license

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

### 16:18:05.987   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvibfiplq6wh",
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
  "lastOtherSaveTime": 1792599459000,
  "block": [],
  "sessionId": "h1SkrTobP0jNMVpLAAAJ",
  "sessionTimeConnect": 1792599454200,
  "sessionTimeIdle": 23919,
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
  "jwtSession": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmliZmlwbHE2d2giLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjU5OTQ1NDIwMH0sImlhdCI6MTc5MjU5OTQ1NCwiZXhwIjoxNzk1MTkxNDU0fQ.SF9OtgKrsq6hbWf2IF8M-KX1rPud_ZQ2blWl0ommuKM",
  "time": 32087,
  "supportAuthChangesAck": true
}
```

### 16:18:05.994   <-  Alice           refreshToken

```json
{
  "type": "refreshToken",
  "messages": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmliZmlwbHE2d2giLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6dHJ1ZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkyNTk5NDU0MjAwfSwiaWF0IjoxNzkyNTk5NDg1LCJleHAiOjE3OTUxOTE0ODV9.6I1SO65lxja8CCZOtRLx1-5dmtHAIMXy-9Jg7HTE9ME"
}
```

### 16:18:05.994   <-  Alice           error

```json
{
  "type": "error",
  "description": "Restore error. Document modified.",
  "code": 4010
}
```

### 16:18:05.995   ->  Alice           close

```json
{
  "type": "close"
}
```

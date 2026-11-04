## Info

- Timestamp: 2026-11-09T13:11:46.567Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document alone (Bob and Mallory never join) and types
     "Hello world": the whole batch is flushed as a single `saveChanges` of 14
     change fragments (`deleteIndex: null`), the change count going 0 → 14.
  2. Alice undoes all her typing (Ctrl+Z until the document is back to its
     original content — still nothing on the wire, the undo is client-side
     only since she is alone).
  3. Alice saves (the API behind Ctrl+S): the save carries no change and
     `deleteIndex: 0`, asking the server to destroy ALL its stored changes
     (UNDO support, RFC §2.2): the history is truncated 14 → 0, confirmed by
     the `unSaveLock{index: -1, syncChangesIndex: 0}` reply (a marker save,
     no new save point is established).
  4. Alice retypes the same "Hello world": the save appends 14 new change
     fragments after the truncated history (`deleteIndex: 0` again, now a
     no-op marker equal to the current change count), the count going 0 → 14.

     This is the equal-count collision: the change count is back to exactly
     its pre-undo value (14) while the stored history has been rewritten
     underneath — and the client asks for its save locks with
     `syncChangesIndex: 0` (the "just joined" sentinel, indistinguishable
     from a fresh editor) then `syncChangesIndex: 14` (matching the count).
     A server-side desynchronization check comparing the change count alone
     (be it OnlyOffice's `forceSave.index` comparison or the editics
     `len(session.changes)` comparison) cannot tell this rewritten history
     from the original one: same count, different changes.

## Events

### 13:11:30.688       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-afe0803dc41215885a9ed25958faf9d3/web-apps/apps/documenteditor/main/../../../../doc/drivemw9a5fgw-word/c/?shardkey=drivemw9a5fgw-word&EIO=4&transport=websocket"
```

### 13:11:30.691   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "OAI3ZJKJb69SyJNDAAAQ",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 13:11:30.723   <-  Alice           license

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

### 13:11:30.840   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "drivemw9a5fgw-word",
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
    "id": "drivemw9a5fgw-word",
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
  "time": 473,
  "supportAuthChangesAck": true
}
```

### 13:11:30.858   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "r4tCtCni1eiWx4iaAAAR",
  "sessionTimeConnect": 1794229890722,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "r4tCtCni1eiWx4iaAAAR",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJkcml2ZW13OWE1Zmd3LXdvcmQiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5NDIyOTg5MDcyMn0sImlhdCI6MTc5NDIyOTg5MCwiZXhwIjoxNzk2ODIxODkwfQ.8yBo3xwtaVBEGU2d8uiQ2PxuHzNh4mcMstRz4z1u7zA",
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
  "openedAt": 1794233490848
}
```

### 13:11:30.926   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/drivemw9a5fgw-word/Editor.bin/Editor.bin?md5=9JWcN5fTzXTNfadGrRYNHw&expires=1796824483&shardkey=drivemw9a5fgw-word&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/drivemw9a5fgw-word/media/image1.png/image1.png?md5=jv04y-Qkabp_URGGYjpZjA&expires=1796824483&shardkey=drivemw9a5fgw-word&filename=image1.png"
    },
    "openedAt": 1794233490848
  }
}
```

### 13:11:30.950   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:12"
}
```

### 13:11:31.034   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:82"
}
```

### 13:11:31.133   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 13:11:31.339   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 13:11:31.339   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:800 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 13:11:31.339   <-  Alice           message

```json
{
  "type": "message"
}
```

### 13:11:41.535   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 13:11:41.536   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 13:11:41.537   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAAAAAAAAAAAAAIAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAAAAAAEAAABIAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAQAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABvAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAIAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABQAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAACAAAAAAAAAAIAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAAB3AAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAsAAAA=\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 13:11:41.545   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1794229901000,
  "syncChangesIndex": 14
}
```

### 13:11:42.606   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 14
}
```

### 13:11:42.606   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 13:11:42.623   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": 0,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAAAAAA=\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 13:11:42.630   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": 1794229902000,
  "syncChangesIndex": 0
}
```

### 13:11:44.455   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 13:11:44.455   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 13:11:44.457   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAAAAAAAAAAAAAIAAAAAAAAAAAAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAAAAAAEAAABIAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAQAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABvAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAIAAAAAAAAAAAAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABQAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAACAAAAAAAAAAIAAAAAAAAAAAAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAAB3AAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": 0,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAsAAAA=\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 13:11:44.464   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": 1794229904000,
  "syncChangesIndex": 14
}
```

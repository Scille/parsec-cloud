## Info

- Timestamp: 2026-10-14T10:18:37.842Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice switches to strict co-editing (the API behind the Co-editing Mode:
     Strict button) and locks the Title paragraph by typing in it.
  2. Bob joins: his auth event carries the lock table as a dict (empty in every
     record so far).
  3. Bob (also strict) try on the Title is refused client-side.
  4. Bob locks and edits the Subtitle instead (his saveChanges, collaborative,
     releases his paragraph lock, the relay to Alice carrying its descriptor).
  5. Alice saves collaboratively: her saveChanges releases her paragraph lock
     and Bob receives its descriptor, leaving the Title free for Bob to lock and
     edit.

## Events

### 10:18:20.787       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/documenteditor/main/../../../../doc/recmv7yilbfs87o/c/?shardkey=recmv7yilbfs87o&EIO=4&transport=websocket"
```

### 10:18:20.790   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "mOqokb7C3IY1dRFKAAB4",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:18:20.801   <-  Alice           license

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

### 10:18:20.914   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv7yilbfs87o",
  "documentCallbackUrl": "http://host.containers.internal:38219/callback",
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
    "id": "recmv7yilbfs87o",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:38219/document.docx",
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
  "time": 424,
  "supportAuthChangesAck": true
}
```

### 10:18:20.930   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "552VuoIlc2FC4pmiAAB5",
  "sessionTimeConnect": 1791973100801,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "552VuoIlc2FC4pmiAAB5",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd5aWxiZnM4N28iLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MzEwMDgwMX0sImlhdCI6MTc5MTk3MzEwMCwiZXhwIjoxNzk0NTY1MTAwfQ.Tiz1QVgjOc38G5nQngusGNEmvLw1cQ92lgtEI8edKz8",
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
  "openedAt": 1791980300919
}
```

### 10:18:20.991   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7yilbfs87o/Editor.bin/Editor.bin?md5=YUCvEbdX5s7Rq96m-o9STg&expires=1794567693&shardkey=recmv7yilbfs87o&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv7yilbfs87o/media/image1.png/image1.png?md5=TNVvK-3A0CEreXiO1ddPBA&expires=1794567693&shardkey=recmv7yilbfs87o&filename=image1.png"
    },
    "openedAt": 1791980300919
  }
}
```

### 10:18:20.998   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:5"
}
```

### 10:18:21.142   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:87"
}
```

### 10:18:21.244   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:102"
}
```

### 10:18:21.473   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 10:18:21.473   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:795 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:18:21.474   <-  Alice           message

```json
{
  "type": "message"
}
```

### 10:18:22.466   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ""
}
```

### 10:18:22.497   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    "704"
  ]
}
```

### 10:18:22.498   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1791973102497,
      "user": "de10a11cec00100000000000000000001",
      "block": "704"
    }
  }
}
```

### 10:18:24.229   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 10:18:24.230   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:18:24.235   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAYAAAAAAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABMAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABjAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABrAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABkAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAACAAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAACAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABiAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAAB5AAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAACwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAYAAAAEAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAABBAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABpAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABjAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAEgAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":null}",
  "unlock": false,
  "releaseLocks": false
}
```

### 10:18:24.239   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791973104000,
  "syncChangesIndex": 23
}
```

### 10:18:24.330       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/documenteditor/main/../../../../doc/recmv7yilbfs87o/c/?shardkey=recmv7yilbfs87o&EIO=4&transport=websocket"
```

### 10:18:24.332   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "jqavvPwPKHXdzcY1AAB6",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:18:24.359   <-  Bob             license

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

### 10:18:24.433   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv7yilbfs87o",
  "documentCallbackUrl": "http://host.containers.internal:38219/callback",
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
    "id": "recmv7yilbfs87o",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:38219/document.docx",
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
  "time": 288,
  "supportAuthChangesAck": true
}
```

### 10:18:24.441   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "552VuoIlc2FC4pmiAAB5",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 10:18:24.441   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791973104439,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "552VuoIlc2FC4pmiAAB5",
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
      "connectionId": "Ui2UXJBDrCSGsNFzAAB7",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 10:18:24.442   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": 23
}
```

### 10:18:24.443   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7yilbfs87o/Editor.bin/Editor.bin?md5=YUCvEbdX5s7Rq96m-o9STg&expires=1794567693&shardkey=recmv7yilbfs87o&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv7yilbfs87o/media/image1.png/image1.png?md5=TNVvK-3A0CEreXiO1ddPBA&expires=1794567693&shardkey=recmv7yilbfs87o&filename=image1.png"
    },
    "openedAt": 1791980300919
  }
}
```

### 10:18:24.449   <-  Bob             authChanges

```json
{
  "type": "authChanges",
  "changes": [
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAYAAAAAAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABMAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABrAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAACAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAACAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABiAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAAB5AAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAACwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAYAAAAEAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEgAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973104000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 10:18:24.451   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:4"
}
```

### 10:18:24.451   ->  Bob             authChangesAck

```json
{
  "type": "authChangesAck"
}
```

### 10:18:24.551   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "Ui2UXJBDrCSGsNFzAAB7",
  "sessionTimeConnect": 1791973104359,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "552VuoIlc2FC4pmiAAB5",
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
      "connectionId": "Ui2UXJBDrCSGsNFzAAB7",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {
    "704": {
      "time": 1791973102497,
      "user": "de10a11cec00100000000000000000001",
      "block": "704"
    }
  },
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd5aWxiZnM4N28iLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5NzMxMDQzNTl9LCJpYXQiOjE3OTE5NzMxMDQsImV4cCI6MTc5NDU2NTEwNH0.O0x9hkqm3OtWeRLJn8AcWvSt-fOe_lstgGgCWRt4Eag",
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

### 10:18:24.896   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:46"
}
```

### 10:18:25.003   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:107"
}
```

### 10:18:25.160   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:1"
}
```

### 10:18:25.160   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onApplyChanges time:3"
}
```

### 10:18:25.160   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 10:18:25.160   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:892 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:18:25.160   <-  Bob             message

```json
{
  "type": "message"
}
```

### 10:18:25.204   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 10:18:25.205   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791973105205,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 10:18:26.171   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ""
}
```

### 10:18:26.172   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "",
      "time": 1791973106171,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 10:18:28.878   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    "708"
  ]
}
```

### 10:18:28.879   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1791973102497,
      "user": "de10a11cec00100000000000000000001",
      "block": "704"
    },
    "708": {
      "time": 1791973108879,
      "user": "de10808c0010000000000000000000002",
      "block": "708"
    }
  }
}
```

### 10:18:28.879   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1791973102497,
      "user": "de10a11cec00100000000000000000001",
      "block": "704"
    },
    "708": {
      "time": 1791973108879,
      "user": "de10808c0010000000000000000000002",
      "block": "708"
    }
  }
}
```

### 10:18:29.913   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 23
}
```

### 10:18:29.913   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:18:29.914   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAABgAAAAEAAABCAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAABwAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAACAAAAAEAAABiAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMQAxAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAACAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAACgAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAACwAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAADAAAAAEAAABjAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAADQAAAAEAAABrAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAADgAAAAEAAABzAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMQAxAAEAHAABAAAADwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAEAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAEAAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAEQAAAAEAAABoAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAEgAAAAEAAABlAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMQAxAAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAkAAAAGAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAFAAAAAEAAABzAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAFQAAAAEAAAB1AAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAFgAAAAEAAABiAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAFwAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAGAAAAAEAAABpAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAGQAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAGgAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAGwAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMQAxAAEAHAABAAAAHAAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAHAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMQAxAAEAHAABAAAAHQAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 10:18:29.922   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAABgAAAAEAAABCAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAABwAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAACAAAAAEAAABiAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMQAxAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAACAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAACgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAACwAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAADAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAADQAAAAEAAABrAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAADgAAAAEAAABzAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMQAxAAEAHAABAAAADwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAEAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAEAAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAEQAAAAEAAABoAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAEgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMQAxAAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAkAAAAGAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAFAAAAAEAAABzAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAFQAAAAEAAAB1AAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAFgAAAAEAAABiAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAFwAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAGAAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAGQAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAGgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAGwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMQAxAAEAHAABAAAAHAAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAHAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMQAxAAEAHAABAAAAHQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973109000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 55,
  "syncChangesIndex": 55,
  "endSaveChanges": true,
  "locks": [
    {
      "block": "708",
      "user": "de10808c0010000000000000000000002",
      "time": 1791973109921,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":null}"
}
```

### 10:18:29.922   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 23,
  "time": 1791973109000,
  "syncChangesIndex": 55
}
```

### 10:18:31.662   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 55
}
```

### 10:18:31.662   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:18:31.666   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAQAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAAgAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAABAAAAAAAAAAQAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABBAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABjAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAACAAAAAAAAAAQAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAACAAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAkAAAADAAAAAAAAAAQAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABzAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABzAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAEAAAAAAAAAAQAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAEgAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 10:18:31.673   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAQAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAAgAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAABAAAAAAAAAAQAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAACAAAAAAAAAAQAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAACAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAkAAAADAAAAAAAAAAQAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABzAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABzAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAEAAAAAAAAAAQAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEgAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973111000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 77,
  "syncChangesIndex": 77,
  "endSaveChanges": true,
  "locks": [
    {
      "block": "704",
      "user": "de10a11cec00100000000000000000001",
      "time": 1791973111673,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":null}"
}
```

### 10:18:31.674   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 55,
  "time": 1791973111000,
  "syncChangesIndex": 77
}
```

### 10:18:32.667   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 77
}
```

### 10:18:32.667   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:18:32.670   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 10:18:32.670   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 10:18:33.733   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    "704"
  ]
}
```

### 10:18:33.733   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1791973113733,
      "user": "de10808c0010000000000000000000002",
      "block": "704"
    }
  }
}
```

### 10:18:33.734   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1791973113733,
      "user": "de10808c0010000000000000000000002",
      "block": "704"
    }
  }
}
```

### 10:18:34.727   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 77
}
```

### 10:18:34.727   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:18:34.734   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABCAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABiAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABQAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAACAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABrAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABzAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAACwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAEAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABoAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAgAAAAGAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEwAAAAEAAABtAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAFAAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAFQAAAAEAAABuAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAFgAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAFwAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAHAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAGAAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 10:18:34.741   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABCAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABiAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAACAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABrAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABzAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAACwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAEAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABoAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAgAAAAGAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAFAAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAFQAAAAEAAABuAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAFgAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAFwAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAHAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yilbfs87o",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAGAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791973114000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 108,
  "syncChangesIndex": 108,
  "endSaveChanges": true,
  "locks": [
    {
      "block": "704",
      "user": "de10808c0010000000000000000000002",
      "time": 1791973114741,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":null}"
}
```

### 10:18:34.741   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 77,
  "time": 1791973114000,
  "syncChangesIndex": 108
}
```

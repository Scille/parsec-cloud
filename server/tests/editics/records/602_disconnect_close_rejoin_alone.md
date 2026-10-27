## Info

- Timestamp: 2026-10-15T09:58:55.830Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document alone.
  2. Alice edits the document.
  3. Alice closes her editor: the client sends no `close` event and the connection is closed (code 1001).
  4. Alice re-joins with a new editor: her auth carries a null sessionId (a fresh handshake, not a connection restore), and the document server replays her own earlier changes as authChanges.

## Events

### 09:58:46.792       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9d9a5c2af2/c/?shardkey=recmv9d9a5c2af2&EIO=4&transport=websocket"
```

### 09:58:46.795   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "ER1UgdyHRVewKMsuAABq",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:58:46.797   <-  Alice           license

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

### 09:58:46.918   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv9d9a5c2af2",
  "documentCallbackUrl": "http://host.containers.internal:33093/callback",
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
    "id": "recmv9d9a5c2af2",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33093/document.docx",
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
  "time": 402,
  "supportAuthChangesAck": true
}
```

### 09:58:46.937   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "V4GtwNiKNe6W89pwAABr",
  "sessionTimeConnect": 1792058326797,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "V4GtwNiKNe6W89pwAABr",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkOWE1YzJhZjIiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjA1ODMyNjc5N30sImlhdCI6MTc5MjA1ODMyNiwiZXhwIjoxNzk0NjUwMzI2fQ.V4_ESYlhzLZDeZT5NMiDPt3oAY_NIABYhW4XQqvwyp8",
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
  "openedAt": 1792065526925
}
```

### 09:58:46.996   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9d9a5c2af2/Editor.bin/Editor.bin?md5=vnYOiU7oU4LF5h_lxpOR6g&expires=1794652919&shardkey=recmv9d9a5c2af2&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9d9a5c2af2/media/image1.png/image1.png?md5=7iMptiKYR5AWlKxbVb8Gzg&expires=1794652919&shardkey=recmv9d9a5c2af2&filename=image1.png"
    },
    "openedAt": 1792065526925
  }
}
```

### 09:58:47.002   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:5"
}
```

### 09:58:47.140   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:82"
}
```

### 09:58:47.241   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 09:58:47.440   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:58:47.440   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:758 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:58:47.441   <-  Alice           message

```json
{
  "type": "message"
}
```

### 09:58:50.522   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 09:58:50.523   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 09:58:50.525   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAAAAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAQAAAACAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAYAAAAEAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 09:58:50.534   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792058330000,
  "syncChangesIndex": 24
}
```

### 09:58:51.176       Alice           [Transport WebSocket] ws-close

```json
"client close, code 1001"
```

### 09:58:51.954       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9d9a5c2af2/c/?shardkey=recmv9d9a5c2af2&EIO=4&transport=websocket"
```

### 09:58:51.955   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "ADAky2zz0jfwfPvYAABs",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:58:51.984   <-  Alice           license

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

### 09:58:52.064   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv9d9a5c2af2",
  "documentCallbackUrl": "http://host.containers.internal:33093/callback",
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
    "id": "recmv9d9a5c2af2",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33093/document.docx",
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
  "time": 279,
  "supportAuthChangesAck": true
}
```

### 09:58:52.080   <-  Alice           authChanges

```json
{
  "type": "authChanges",
  "changes": [
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAAAAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAQAAAACAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAYAAAAEAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv9d9a5c2af2",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1792058330000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 09:58:52.082   ->  Alice           authChangesAck

```json
{
  "type": "authChangesAck"
}
```

### 09:58:52.181   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "UPI2yPbV-dCTQ5nZAABt",
  "sessionTimeConnect": 1792058331984,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000002",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 2,
      "view": false,
      "connectionId": "UPI2yPbV-dCTQ5nZAABt",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkOWE1YzJhZjIiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6Mn0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjA1ODMzMTk4NH0sImlhdCI6MTc5MjA1ODMzMiwiZXhwIjoxNzk0NjUwMzMyfQ.Vi4aKnQtqTbc-3EaVfMGBMlhwmrbUQSZQ6eWfQeBS78",
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
  "openedAt": 1792065526925
}
```

### 09:58:52.183   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9d9a5c2af2/Editor.bin/Editor.bin?md5=vnYOiU7oU4LF5h_lxpOR6g&expires=1794652919&shardkey=recmv9d9a5c2af2&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9d9a5c2af2/media/image1.png/image1.png?md5=7iMptiKYR5AWlKxbVb8Gzg&expires=1794652919&shardkey=recmv9d9a5c2af2&filename=image1.png"
    },
    "openedAt": 1792065526925
  }
}
```

### 09:58:52.474   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:1"
}
```

### 09:58:52.518   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:44"
}
```

### 09:58:52.570   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:52"
}
```

### 09:58:52.723   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:1"
}
```

### 09:58:52.724   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onApplyChanges time:3"
}
```

### 09:58:52.724   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:58:52.724   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:815 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:58:52.724   <-  Alice           message

```json
{
  "type": "message"
}
```

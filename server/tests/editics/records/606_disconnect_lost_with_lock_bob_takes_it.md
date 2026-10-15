## Info

- Timestamp: 2026-10-15T10:00:34.909Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document alone and switches to strict co-editing.
  2. Alice takes the lock on the Title paragraph and edits it.
  3. Alice loses her connection (the websocket is closed without a close frame).
  4. Bob joins while Alice is away.
  5. Bob takes the element Alice had locked: the document server released Alice's lock when she disconnected, so Bob's getLock is granted and he edits and saves the element.
  6. Alice's connection comes back: her restore auth is refused with error 4010 "Restore error. Document modified." since the document changed while she was away.

## Events

### 10:00:04.299       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9daxx0h1uk/c/?shardkey=recmv9daxx0h1uk&EIO=4&transport=websocket"
```

### 10:00:04.301   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "IyKwszWRckM8AoAyAAB-",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:00:04.314   <-  Alice           license

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

### 10:00:04.439   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv9daxx0h1uk",
  "documentCallbackUrl": "http://host.containers.internal:38973/callback",
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
    "id": "recmv9daxx0h1uk",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:38973/document.docx",
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
  "time": 443,
  "supportAuthChangesAck": true
}
```

### 10:00:04.457   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "S2geaML6m9ZFSpjOAAB_",
  "sessionTimeConnect": 1792058404314,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "S2geaML6m9ZFSpjOAAB_",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYXh4MGgxdWsiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjA1ODQwNDMxNH0sImlhdCI6MTc5MjA1ODQwNCwiZXhwIjoxNzk0NjUwNDA0fQ._ECqx792rC-gjIw8J5X4IJFo6IoU3p15o8kn_11fke8",
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
  "openedAt": 1792065604447
}
```

### 10:00:04.516   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9daxx0h1uk/Editor.bin/Editor.bin?md5=9BHdTZUAL0gP3l9r_SsLhQ&expires=1794652997&shardkey=recmv9daxx0h1uk&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9daxx0h1uk/media/image1.png/image1.png?md5=pmFw3S4dkg1GWz6u0cduLQ&expires=1794652997&shardkey=recmv9daxx0h1uk&filename=image1.png"
    },
    "openedAt": 1792065604447
  }
}
```

### 10:00:04.572   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:10"
}
```

### 10:00:04.658   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:86"
}
```

### 10:00:04.762   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:104"
}
```

### 10:00:04.967   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 10:00:04.967   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:800 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:00:04.968   <-  Alice           message

```json
{
  "type": "message"
}
```

### 10:00:05.971   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ""
}
```

### 10:00:05.999   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    "704"
  ]
}
```

### 10:00:06.000   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1792058406000,
      "user": "de10a11cec00100000000000000000001",
      "block": "704"
    }
  }
}
```

### 10:00:07.647       Alice           [Transport WebSocket] ws-close

```json
"monitor close, code 1006 (connection cut)"
```

### 10:00:08.517       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9daxx0h1uk/c/?shardkey=recmv9daxx0h1uk&EIO=4&transport=websocket"
```

### 10:00:08.518   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "M5D4GtoQenYVT4lKAACA",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:00:08.545   <-  Bob             license

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

### 10:00:08.617   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv9daxx0h1uk",
  "documentCallbackUrl": "http://host.containers.internal:38973/callback",
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
    "id": "recmv9daxx0h1uk",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:38973/document.docx",
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
  "time": 261,
  "supportAuthChangesAck": true
}
```

### 10:00:08.631   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "u5VAJUTN2EtGrGH7AACB",
  "sessionTimeConnect": 1792058408545,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "u5VAJUTN2EtGrGH7AACB",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYXh4MGgxdWsiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIwNTg0MDg1NDV9LCJpYXQiOjE3OTIwNTg0MDgsImV4cCI6MTc5NDY1MDQwOH0.obzXqipQRq7EI1l8QWAbzfmJwyacyG9UsoSfhNA39Ao",
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
  "openedAt": 1792065604447
}
```

### 10:00:08.632   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9daxx0h1uk/Editor.bin/Editor.bin?md5=9BHdTZUAL0gP3l9r_SsLhQ&expires=1794652997&shardkey=recmv9daxx0h1uk&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9daxx0h1uk/media/image1.png/image1.png?md5=pmFw3S4dkg1GWz6u0cduLQ&expires=1794652997&shardkey=recmv9daxx0h1uk&filename=image1.png"
    },
    "openedAt": 1792065604447
  }
}
```

### 10:00:08.635   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:1"
}
```

### 10:00:09.100   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:41"
}
```

### 10:00:09.153   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:54"
}
```

### 10:00:09.298   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 10:00:09.298   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:824 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:00:09.298   <-  Bob             message

```json
{
  "type": "message"
}
```

### 10:00:10.289   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ""
}
```

### 10:00:10.300   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    "704"
  ]
}
```

### 10:00:10.301   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1792058410301,
      "user": "de10808c0010000000000000000000002",
      "block": "704"
    }
  }
}
```

### 10:00:11.289   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 10:00:11.290   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:00:11.298   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABCAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABiAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABQAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAACAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABrAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABzAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAACwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAEAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABoAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAgAAAAGAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEwAAAAEAAABtAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAFAAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAFQAAAAEAAABuAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAFgAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAFwAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAHAAAAAAAAAAcAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAGAAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":null}",
  "unlock": false,
  "releaseLocks": false
}
```

### 10:00:11.302   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792058411000,
  "syncChangesIndex": 31
}
```

### 10:00:31.663       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9daxx0h1uk/c/?shardkey=recmv9daxx0h1uk&EIO=4&transport=websocket"
```

### 10:00:31.665   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "3_KF3ABYp0of6JItAACC",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:00:31.666   <-  Alice           license

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

### 10:00:31.667   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv9daxx0h1uk",
  "documentCallbackUrl": "http://host.containers.internal:38973/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "firstname": null,
    "lastname": null,
    "indexUser": 1
  },
  "editorType": 0,
  "lastOtherSaveTime": -1,
  "block": [
    "704"
  ],
  "sessionId": "S2geaML6m9ZFSpjOAAB_",
  "sessionTimeConnect": 1792058404314,
  "sessionTimeIdle": 23985,
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
  "jwtSession": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYXh4MGgxdWsiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjA1ODQwNDMxNH0sImlhdCI6MTc5MjA1ODQwNCwiZXhwIjoxNzk0NjUwNDA0fQ._ECqx792rC-gjIw8J5X4IJFo6IoU3p15o8kn_11fke8",
  "time": 27671,
  "supportAuthChangesAck": true
}
```

### 10:00:31.674   <-  Alice           refreshToken

```json
{
  "type": "refreshToken",
  "messages": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYXh4MGgxdWsiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6dHJ1ZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkyMDU4NDA0MzE0fSwiaWF0IjoxNzkyMDU4NDMxLCJleHAiOjE3OTQ2NTA0MzF9.EEMjx7igDDiyJoI2dyKKh7Bcch0M-wLFbt_2Uaa_Znc"
}
```

### 10:00:31.674   <-  Alice           error

```json
{
  "type": "error",
  "description": "Restore error. Document modified.",
  "code": 4010
}
```

### 10:00:31.674   ->  Alice           close

```json
{
  "type": "close"
}
```

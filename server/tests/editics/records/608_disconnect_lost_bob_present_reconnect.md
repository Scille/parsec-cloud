## Info

- Timestamp: 2026-10-15T10:01:18.206Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document.
  2. Bob joins: Alice and Bob edit together.
  3. Alice loses her connection (the websocket is closed without a close frame).
  4. Bob sees Alice leave (connectState without Alice) and edits the document alone.
  5. Alice's connection comes back: her editor sends its auth with the previous sessionId.
  6. The document changed while she was away, so the document server refuses the restore with error 4010 "Restore error. Document modified.", and Alice's client answers by closing the co-authoring session.

## Events

### 10:00:47.969       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9dbvnd103d/c/?shardkey=recmv9dbvnd103d&EIO=4&transport=websocket"
```

### 10:00:47.971   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "ywt4Ycfo1gwFFD1DAACK",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:00:47.977   <-  Alice           license

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

### 10:00:48.093   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv9dbvnd103d",
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
    "id": "recmv9dbvnd103d",
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
  "time": 399,
  "supportAuthChangesAck": true
}
```

### 10:00:48.111   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "-3qobAeHS7q_tR9wAACL",
  "sessionTimeConnect": 1792058447977,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "-3qobAeHS7q_tR9wAACL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYnZuZDEwM2QiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjA1ODQ0Nzk3N30sImlhdCI6MTc5MjA1ODQ0OCwiZXhwIjoxNzk0NjUwNDQ4fQ.M4Rr1Uy4A-3h1ybacStgy5Phi6nkHdiniB-tMkHNIOo",
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
  "openedAt": 1792065648100
}
```

### 10:00:48.169   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9dbvnd103d/Editor.bin/Editor.bin?md5=bths36odFsvpF51XP1v-NA&expires=1794653041&shardkey=recmv9dbvnd103d&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9dbvnd103d/media/image1.png/image1.png?md5=q53BZU8xQTnfFBuJIWYAdg&expires=1794653041&shardkey=recmv9dbvnd103d&filename=image1.png"
    },
    "openedAt": 1792065648100
  }
}
```

### 10:00:48.182   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:11"
}
```

### 10:00:48.327   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:85"
}
```

### 10:00:48.428   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 10:00:48.626   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 10:00:48.626   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:768 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:00:48.627   <-  Alice           message

```json
{
  "type": "message"
}
```

### 10:00:50.187       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9dbvnd103d/c/?shardkey=recmv9dbvnd103d&EIO=4&transport=websocket"
```

### 10:00:50.189   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "Y8lcfdBw3b3j7FBVAACM",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:00:50.220   <-  Bob             license

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

### 10:00:50.296   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv9dbvnd103d",
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
    "id": "recmv9dbvnd103d",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:38973/document.docx",
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
  "time": 645,
  "supportAuthChangesAck": true
}
```

### 10:00:50.304   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "-3qobAeHS7q_tR9wAACL",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 10:00:50.304   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792058450303,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "-3qobAeHS7q_tR9wAACL",
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
      "connectionId": "mUUUS-oVM1Rg_3ZIAACN",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 10:00:50.305   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9dbvnd103d/Editor.bin/Editor.bin?md5=bths36odFsvpF51XP1v-NA&expires=1794653041&shardkey=recmv9dbvnd103d&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9dbvnd103d/media/image1.png/image1.png?md5=q53BZU8xQTnfFBuJIWYAdg&expires=1794653041&shardkey=recmv9dbvnd103d&filename=image1.png"
    },
    "openedAt": 1792065648100
  }
}
```

### 10:00:50.305   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 10:00:50.311   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 10:00:50.312   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792058450312,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 10:00:50.312   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "mUUUS-oVM1Rg_3ZIAACN",
  "sessionTimeConnect": 1792058450220,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "-3qobAeHS7q_tR9wAACL",
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
      "connectionId": "mUUUS-oVM1Rg_3ZIAACN",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYnZuZDEwM2QiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIwNTg0NTAyMjB9LCJpYXQiOjE3OTIwNTg0NTAsImV4cCI6MTc5NDY1MDQ1MH0.-XnBjsHBNgI2NUZM5FPRQf3fKmfzkBFMVdAdFVbmyXA",
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

### 10:00:50.316   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:6"
}
```

### 10:00:50.361   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:45"
}
```

### 10:00:50.465   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:104"
}
```

### 10:00:50.609   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 10:00:50.609   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:843 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:00:50.610   <-  Bob             message

```json
{
  "type": "message"
}
```

### 10:00:50.654   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 10:00:50.655   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792058450655,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 10:00:51.621       Alice           [Transport WebSocket] ws-close

```json
"monitor close, code 1006 (connection cut)"
```

### 10:00:51.623   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792058451622,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "mUUUS-oVM1Rg_3ZIAACN",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 10:00:54.386   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 10:00:54.387   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:00:54.389   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAAAAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAQAAAACAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAYAAAAEAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 10:00:54.393   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792058454000,
  "syncChangesIndex": 24
}
```

### 10:01:14.951       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9dbvnd103d/c/?shardkey=recmv9dbvnd103d&EIO=4&transport=websocket"
```

### 10:01:14.953   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "MyHahuH6iMyYJd7VAACO",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:01:14.954   <-  Alice           license

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

### 10:01:14.960   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv9dbvnd103d",
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
  "block": [],
  "sessionId": "-3qobAeHS7q_tR9wAACL",
  "sessionTimeConnect": 1792058447977,
  "sessionTimeIdle": 23322,
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
  "jwtSession": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYnZuZDEwM2QiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjA1ODQ0Nzk3N30sImlhdCI6MTc5MjA1ODQ0OCwiZXhwIjoxNzk0NjUwNDQ4fQ.M4Rr1Uy4A-3h1ybacStgy5Phi6nkHdiniB-tMkHNIOo",
  "time": 27266,
  "supportAuthChangesAck": true
}
```

### 10:01:14.966   <-  Alice           refreshToken

```json
{
  "type": "refreshToken",
  "messages": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYnZuZDEwM2QiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6dHJ1ZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkyMDU4NDQ3OTc3fSwiaWF0IjoxNzkyMDU4NDc0LCJleHAiOjE3OTQ2NTA0NzR9.FH9YVsSz-DcaVgM_XxzES78kI_M_Ln8gjkVa28Xwg7Y"
}
```

### 10:01:14.967   <-  Alice           error

```json
{
  "type": "error",
  "description": "Restore error. Document modified.",
  "code": 4010
}
```

### 10:01:14.968   ->  Alice           close

```json
{
  "type": "close"
}
```

## Info

- Timestamp: 2026-10-13T15:26:27.944Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario: Alice edits the document while Bob is authenticating.

## Events

### 15:26:15.476       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u2phbj5xw/c/?shardkey=recmv6u2phbj5xw&EIO=4&transport=websocket"
```

### 15:26:15.478   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "3AWKwUYAWmMN9r8HAAGG",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:26:15.483   <-  Alice           license

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

### 15:26:15.674   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6u2phbj5xw",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
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
    "id": "recmv6u2phbj5xw",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33041/document.docx",
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
  "time": 483,
  "supportAuthChangesAck": true
}
```

### 15:26:15.690   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "AgiRf4WwykmH7325AAGH",
  "sessionTimeConnect": 1791905175483,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "AgiRf4WwykmH7325AAGH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MnBoYmo1eHciLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTE3NTQ4M30sImlhdCI6MTc5MTkwNTE3NSwiZXhwIjoxNzk0NDk3MTc1fQ.71LQqnJYvdMZjoT6v5Hg7w9Zx6lXXZpFYhHYQesPHN4",
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
  "openedAt": 1791912375682
}
```

### 15:26:15.745   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u2phbj5xw/Editor.bin/Editor.bin?md5=ub1LMC4YrXvgHPzuxszMGQ&expires=1794499768&shardkey=recmv6u2phbj5xw&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u2phbj5xw/media/image1.png/image1.png?md5=jIz7r5GHHLJjlpFVpG6CCA&expires=1794499768&shardkey=recmv6u2phbj5xw&filename=image1.png"
    },
    "openedAt": 1791912375682
  }
}
```

### 15:26:15.803   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:5"
}
```

### 15:26:15.906   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:102"
}
```

### 15:26:16.006   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:102"
}
```

### 15:26:16.216   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:26:16.217   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:853 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":86400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:26:16.217   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:26:17.382       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u2phbj5xw/c/?shardkey=recmv6u2phbj5xw&EIO=4&transport=websocket"
```

### 15:26:17.436   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "YTn-EC3uWPPXSoirAAGI",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:26:17.496   <-  Bob             license

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

### 15:26:17.616   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6u2phbj5xw",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
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
    "id": "recmv6u2phbj5xw",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33041/document.docx",
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
  "time": 365,
  "supportAuthChangesAck": true
}
```

### 15:26:17.636   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905177623,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "AgiRf4WwykmH7325AAGH",
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
      "connectionId": "nIcr7jZZcrdYhUu7AAGJ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:26:18.058   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:26:18.078   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:26:19.051   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAIAAAA="
}
```

### 15:26:19.071   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:26:19.091   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:19.111   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:19.131   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791905179000,
  "syncChangesIndex": 3
}
```

### 15:26:19.152   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 3
}
```

### 15:26:19.172   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:19.242   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:19.262   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 3,
  "time": 1791905179000,
  "syncChangesIndex": 6
}
```

### 15:26:19.332   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 6
}
```

### 15:26:19.353   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:19.373   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAIAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAIAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAACAAAAAAAAAAIAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAkAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:19.393   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 6,
  "time": 1791905179000,
  "syncChangesIndex": 12
}
```

### 15:26:19.463   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 12
}
```

### 15:26:19.485   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:19.505   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:19.525   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 12,
  "time": 1791905179000,
  "syncChangesIndex": 16
}
```

### 15:26:19.545   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 16
}
```

### 15:26:19.565   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:19.585   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:19.605   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 16,
  "time": 1791905179000,
  "syncChangesIndex": 20
}
```

### 15:26:19.625   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 20
}
```

### 15:26:19.645   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:19.665   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:19.685   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 20,
  "time": 1791905179000,
  "syncChangesIndex": 23
}
```

### 15:26:19.756   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 23
}
```

### 15:26:19.776   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:19.797   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:19.817   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 23,
  "time": 1791905179000,
  "syncChangesIndex": 26
}
```

### 15:26:19.888   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 26
}
```

### 15:26:19.908   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:19.928   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:19.948   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 26,
  "time": 1791905179000,
  "syncChangesIndex": 30
}
```

### 15:26:20.670   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADYAOAAyAAAAAAA="
}
```

### 15:26:21.040   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 30
}
```

### 15:26:21.060   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:21.080   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAUAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:21.101   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 30,
  "time": 1791905181000,
  "syncChangesIndex": 33
}
```

### 15:26:21.173   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 33
}
```

### 15:26:21.192   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:21.263   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAgAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:21.283   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 33,
  "time": 1791905181000,
  "syncChangesIndex": 37
}
```

### 15:26:21.303   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 37
}
```

### 15:26:21.323   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:21.344   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:21.364   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 37,
  "time": 1791905181000,
  "syncChangesIndex": 41
}
```

### 15:26:21.384   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 41
}
```

### 15:26:21.404   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:21.424   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:21.444   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 41,
  "time": 1791905181000,
  "syncChangesIndex": 44
}
```

### 15:26:21.514   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 44
}
```

### 15:26:21.535   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:21.556   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:21.576   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 44,
  "time": 1791905181000,
  "syncChangesIndex": 48
}
```

### 15:26:21.647   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 48
}
```

### 15:26:21.667   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:21.687   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABEAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:21.707   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 48,
  "time": 1791905181000,
  "syncChangesIndex": 51
}
```

### 15:26:21.779   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 51
}
```

### 15:26:21.799   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:21.819   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:21.839   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 51,
  "time": 1791905181000,
  "syncChangesIndex": 53
}
```

### 15:26:23.117   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "AgiRf4WwykmH7325AAGH",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:26:23.167   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u2phbj5xw/Editor.bin/Editor.bin?md5=ub1LMC4YrXvgHPzuxszMGQ&expires=1794499768&shardkey=recmv6u2phbj5xw&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u2phbj5xw/media/image1.png/image1.png?md5=jIz7r5GHHLJjlpFVpG6CCA&expires=1794499768&shardkey=recmv6u2phbj5xw&filename=image1.png"
    },
    "openedAt": 1791912375682
  }
}
```

### 15:26:23.217   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "nIcr7jZZcrdYhUu7AAGJ",
  "sessionTimeConnect": 1791905177456,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "AgiRf4WwykmH7325AAGH",
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
      "connectionId": "nIcr7jZZcrdYhUu7AAGJ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MnBoYmo1eHciLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUxNzc0NTZ9LCJpYXQiOjE3OTE5MDUxNzgsImV4cCI6MTc5NDQ5NzE3OH0.9XCniubA8ua7FJpyV8w21kTKX74iV81nPknG9lD8rGY",
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

### 15:26:23.267   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905178078,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:23.318   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905179051,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:23.368   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 3,
  "syncChangesIndex": 3,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}"
}
```

### 15:26:23.418   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 6,
  "syncChangesIndex": 6,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}"
}
```

### 15:26:23.468   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAIAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAIAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAACAAAAAAAAAAIAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 12,
  "syncChangesIndex": 12,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAkAAAA=\"}"
}
```

### 15:26:23.519   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 16,
  "syncChangesIndex": 16,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}"
}
```

### 15:26:23.569   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 20,
  "syncChangesIndex": 20,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}"
}
```

### 15:26:23.618   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 23,
  "syncChangesIndex": 23,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}"
}
```

### 15:26:23.669   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 26,
  "syncChangesIndex": 26,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}"
}
```

### 15:26:23.719   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905179000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 30,
  "syncChangesIndex": 30,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}"
}
```

### 15:26:23.769   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADYAOAAyAAAAAAA=",
      "time": 1791905180670,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:23.819   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 33,
  "syncChangesIndex": 33,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAUAAAA=\"}"
}
```

### 15:26:23.869   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 37,
  "syncChangesIndex": 37,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAgAAAA=\"}"
}
```

### 15:26:23.919   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 41,
  "syncChangesIndex": 41,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}"
}
```

### 15:26:23.969   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 44,
  "syncChangesIndex": 44,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}"
}
```

### 15:26:24.020   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 48,
  "syncChangesIndex": 48,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}"
}
```

### 15:26:24.070   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 51,
  "syncChangesIndex": 51,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABEAAAA=\"}"
}
```

### 15:26:24.120   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2phbj5xw",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905181000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 53,
  "syncChangesIndex": 53,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}"
}
```

### 15:26:24.171   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 15:26:24.221   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:39"
}
```

### 15:26:24.271   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:108"
}
```

### 15:26:24.321   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:26:24.372   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:6112 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":86400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:26:24.422   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:26:24.472   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:24.522   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:24.573   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:24.624   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:24.674   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:24.724   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:24.774   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:24.824   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:26:24.875   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905184422,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

## Info

- Timestamp: 2026-10-13T15:28:34.862Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: presentation.pptx (slide)
- Scenario: Alice edits the document while Bob is authenticating.

## Events

### 15:28:25.141       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6u5hbvucxe/c/?shardkey=recmv6u5hbvucxe&EIO=4&transport=websocket"
```

### 15:28:25.143   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "-Nj8VGVf1-2cv__wAAGq",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:28:25.154   <-  Alice           license

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

### 15:28:25.376   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6u5hbvucxe",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
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
    "id": "recmv6u5hbvucxe",
    "userid": "de10a11cec0010000000000000000000",
    "format": "pptx",
    "url": "http://host.containers.internal:33041/presentation.pptx",
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
  "time": 526,
  "supportAuthChangesAck": true
}
```

### 15:28:25.392   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "hRf6UPt47KlMT5ZpAAGr",
  "sessionTimeConnect": 1791905305154,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "hRf6UPt47KlMT5ZpAAGr",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1NWhidnVjeGUiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTMwNTE1NH0sImlhdCI6MTc5MTkwNTMwNSwiZXhwIjoxNzk0NDk3MzA1fQ.GnJcVVlb3dh-Ag5plUzjxOp7wRQ6ssWvouH6MC1ihGQ",
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
  "openedAt": 1791912505383
}
```

### 15:28:25.496   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u5hbvucxe/Editor.bin/Editor.bin?md5=yyTMRM3LwJTqv_20WVZg7w&expires=1794499898&shardkey=recmv6u5hbvucxe&filename=Editor.bin"
    },
    "openedAt": 1791912505383
  }
}
```

### 15:28:25.512   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:9"
}
```

### 15:28:25.604   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:91"
}
```

### 15:28:25.707   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:104"
}
```

### 15:28:26.016   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:28:26.016   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1154 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":86400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:28:26.017   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:28:27.595       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6u5hbvucxe/c/?shardkey=recmv6u5hbvucxe&EIO=4&transport=websocket"
```

### 15:28:27.676   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "NLpgeEYFGfHtO2rCAAGs",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:28:27.737   <-  Bob             license

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

### 15:28:27.859   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6u5hbvucxe",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
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
    "id": "recmv6u5hbvucxe",
    "userid": "de10808c001000000000000000000000",
    "format": "pptx",
    "url": "http://host.containers.internal:33041/presentation.pptx",
    "title": "presentation.pptx",
    "lcid": 9,
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
  "time": 761,
  "supportAuthChangesAck": true
}
```

### 15:28:27.879   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905307865,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "hRf6UPt47KlMT5ZpAAGr",
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
      "connectionId": "4UjbVzJHAxVPxtr5AAGt",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:28:27.899   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:28:28.621   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:28:29.042   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:28:29.062   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:29.082   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgAAQAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:29.103   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791905309000,
  "syncChangesIndex": 3
}
```

### 15:28:29.123   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 3
}
```

### 15:28:29.143   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:29.163   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgAAwAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:29.184   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 3,
  "time": 1791905309000,
  "syncChangesIndex": 7
}
```

### 15:28:29.204   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 7
}
```

### 15:28:29.224   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:29.244   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgABQAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:29.263   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 7,
  "time": 1791905309000,
  "syncChangesIndex": 11
}
```

### 15:28:29.334   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 11
}
```

### 15:28:29.354   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:29.375   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgABwAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:29.395   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 11,
  "time": 1791905309000,
  "syncChangesIndex": 15
}
```

### 15:28:29.465   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 15
}
```

### 15:28:29.485   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:29.505   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgACQAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:29.525   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 15,
  "time": 1791905309000,
  "syncChangesIndex": 19
}
```

### 15:28:29.595   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADgACQAAAA=="
}
```

### 15:28:30.822   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "hRf6UPt47KlMT5ZpAAGr",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:28:30.872   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u5hbvucxe/Editor.bin/Editor.bin?md5=yyTMRM3LwJTqv_20WVZg7w&expires=1794499898&shardkey=recmv6u5hbvucxe&filename=Editor.bin"
    },
    "openedAt": 1791912505383
  }
}
```

### 15:28:30.922   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "4UjbVzJHAxVPxtr5AAGt",
  "sessionTimeConnect": 1791905307697,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "hRf6UPt47KlMT5ZpAAGr",
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
      "connectionId": "4UjbVzJHAxVPxtr5AAGt",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1NWhidnVjeGUiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUzMDc2OTd9LCJpYXQiOjE3OTE5MDUzMDcsImV4cCI6MTc5NDQ5NzMwN30.T0uFUkOcGRk9IaFR6pxW-7iJekv2X8bCt72-AhnBqIk",
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

### 15:28:30.972   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905308621,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:28:31.023   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 3,
  "syncChangesIndex": 3,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgAAQAAAA==\"}"
}
```

### 15:28:31.074   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 7,
  "syncChangesIndex": 7,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgAAwAAAA==\"}"
}
```

### 15:28:31.125   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 11,
  "syncChangesIndex": 11,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgABQAAAA==\"}"
}
```

### 15:28:31.175   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 15,
  "syncChangesIndex": 15,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgABwAAAA==\"}"
}
```

### 15:28:31.226   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u5hbvucxe",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791905309000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 19,
  "syncChangesIndex": 19,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgACQAAAA==\"}"
}
```

### 15:28:31.276   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgACQAAAA==",
      "time": 1791905309595,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:28:31.326   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 15:28:31.376   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:60"
}
```

### 15:28:31.426   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:111"
}
```

### 15:28:31.477   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:1"
}
```

### 15:28:31.528   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:28:31.579   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:4182 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":86400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:28:31.629   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onApplyChanges time:218"
}
```

### 15:28:31.679   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:4216 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":86400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:28:31.729   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:28:31.779   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

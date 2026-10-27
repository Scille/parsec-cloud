## Info

- Timestamp: 2026-10-14T09:39:17.065Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: presentation.pptx (slide)
- Scenario:
  1. Alice (edit) connects and edits the document.
  2. Bob (view) joins mid-edit: he receives the backlog of changes as
     authChanges (+ authChangesAck, live viewer) before his auth response.
  3. Alice edits again and her save changes are relayed to him.

## Events

### 09:39:04.312       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/presentationeditor/main/../../../../doc/recmv7x4326wq31/c/?shardkey=recmv7x4326wq31&EIO=4&transport=websocket"
```

### 09:39:04.314   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "J3Bg_JnGSslNOc3WAABA",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:39:04.318   <-  Alice           license

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

### 09:39:04.471   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv7x4326wq31",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
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
    "id": "recmv7x4326wq31",
    "userid": "de10a11cec0010000000000000000000",
    "format": "pptx",
    "url": "http://host.containers.internal:33303/presentation.pptx",
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
  "time": 448,
  "supportAuthChangesAck": true
}
```

### 09:39:04.486   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "UcAQW5GT-e9Lwcr_AABB",
  "sessionTimeConnect": 1791970744318,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "UcAQW5GT-e9Lwcr_AABB",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4NDMyNndxMzEiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDc0NDMxOH0sImlhdCI6MTc5MTk3MDc0NCwiZXhwIjoxNzk0NTYyNzQ0fQ._LSYPv1IuVj7JMQBMRrEs17wrHoW_TQrAxu2v6Ngpic",
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
  "openedAt": 1791977944477
}
```

### 09:39:04.584   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x4326wq31/Editor.bin/Editor.bin?md5=E_9G7in9gO0qBsspJ0IAFw&expires=1794565337&shardkey=recmv7x4326wq31&filename=Editor.bin"
    },
    "openedAt": 1791977944477
  }
}
```

### 09:39:04.594   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:5"
}
```

### 09:39:04.687   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:92"
}
```

### 09:39:04.788   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 09:39:05.081   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:39:05.082   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1048 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:39:05.082   <-  Alice           message

```json
{
  "type": "message"
}
```

### 09:39:07.853   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 09:39:07.853   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 09:39:07.855   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAIAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgACQAAAA==\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 09:39:07.861   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791970747000,
  "syncChangesIndex": 19
}
```

### 09:39:08.651       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/presentationeditor/main/../../../../doc/recmv7x4326wq31/c/?shardkey=recmv7x4326wq31&EIO=4&transport=websocket"
```

### 09:39:08.652   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "l1wdAQoDcdaqfIM8AABC",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:39:08.684   <-  Bob             license

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

### 09:39:08.730   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv7x4326wq31",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
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
    "id": "recmv7x4326wq31",
    "userid": "de10808c001000000000000000000000",
    "format": "pptx",
    "url": "http://host.containers.internal:33303/presentation.pptx",
    "title": "presentation.pptx",
    "nobase64": true,
    "outputformat": 8195,
    "convertToOrigin": ".pdf.xps.oxps.djvu"
  },
  "lang": "en",
  "mode": "view",
  "permissions": {
    "edit": false
  },
  "encrypted": false,
  "IsAnonymousUser": false,
  "timezoneOffset": -120,
  "coEditingMode": "fast",
  "time": 239,
  "supportAuthChangesAck": true
}
```

### 09:39:08.742   <-  Bob             authChanges

```json
{
  "type": "authChanges",
  "changes": [
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAIAAAAAAAAAAgAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791970747000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 09:39:08.745   ->  Bob             authChangesAck

```json
{
  "type": "authChangesAck"
}
```

### 09:39:08.843   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "T1HZuse6G9hgD5XMAABD",
  "sessionTimeConnect": 1791970748684,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "UcAQW5GT-e9Lwcr_AABB",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "T1HZuse6G9hgD5XMAABD",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4NDMyNndxMzEiLCJwZXJtaXNzaW9ucyI6eyJlZGl0IjpmYWxzZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTA4MDhjMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkJvYiIsImluZGV4IjoyfSwiY29FZGl0aW5nIjp7Im1vZGUiOiJmYXN0In0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDc0ODY4NH0sImlhdCI6MTc5MTk3MDc0OCwiZXhwIjoxNzk0NTYyNzQ4fQ.1nrSGBe4upKIO6xQyADyVJ0ardwZmbq8mpQ4LcluAao",
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
  "openedAt": 1791977944477
}
```

### 09:39:08.843   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791970748736,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "UcAQW5GT-e9Lwcr_AABB",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "T1HZuse6G9hgD5XMAABD",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 09:39:08.844   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x4326wq31/Editor.bin/Editor.bin?md5=E_9G7in9gO0qBsspJ0IAFw&expires=1794565337&shardkey=recmv7x4326wq31&filename=Editor.bin"
    },
    "openedAt": 1791977944477
  }
}
```

### 09:39:09.184   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:7"
}
```

### 09:39:09.241   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:58"
}
```

### 09:39:09.293   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:52"
}
```

### 09:39:09.510   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:1"
}
```

### 09:39:09.510   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 09:39:09.510   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:986 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:39:09.510   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onApplyChanges time:193"
}
```

### 09:39:09.510   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1017 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:39:09.511   <-  Bob             message

```json
{
  "type": "message"
}
```

### 09:39:12.213   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 19
}
```

### 09:39:12.213   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 09:39:12.215   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAQAAAAJAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAgAcAAEAAAAAAAAAAgAAACAAAAAAAAAA\",\"40;CAAAADIAMgAyADMAAgADAAEAAAABAAAACAAAADIAMgAyADcAAAAAAA==\",\"102;CAAAADIAMgAyADYAAgAcAAYAAAAAAAAAAQAAAFMAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAbQAAAAAAAAAAAQAAAHAAAAAAAAAAAAEAAABsAAAAAAAAAAABAAAAZQAAAAAAAAAA\",\"36;CAAAADIAMgAyADYAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAKAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAALAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAMAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAANAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAOAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAPAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAQAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAARAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": 19,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACQAAAA==\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 09:39:12.222   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAQAAAAJAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"36;CAAAADIAMgAyADgAAgAcAAEAAAAAAAAAAgAAACAAAAAAAAAA\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"40;CAAAADIAMgAyADMAAgADAAEAAAABAAAACAAAADIAMgAyADcAAAAAAA==\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"102;CAAAADIAMgAyADYAAgAcAAYAAAAAAAAAAQAAAFMAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAbQAAAAAAAAAAAQAAAHAAAAAAAAAAAAEAAABsAAAAAAAAAAABAAAAZQAAAAAAAAAA\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"36;CAAAADIAMgAyADYAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAKAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAALAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAMAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAANAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAOAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAPAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAQAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAARAAAACQAAABEAAAATAAAAEwAAABIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x4326wq31",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791970752000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 40,
  "syncChangesIndex": 40,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACQAAAA==\"}"
}
```

### 09:39:12.222   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": 1791970752000,
  "syncChangesIndex": 40
}
```

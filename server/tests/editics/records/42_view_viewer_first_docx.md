## Info

- Timestamp: 2026-10-14T09:36:58.278Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Bob (view) connects first: the session (and the document conversion) is
     created by a read-only participant.
  2. Alice (edit) joins: no waitAuth is involved (a viewer never holds the auth
     lock).
  3. Alice edits the document; her save changes are relayed to Bob (live
     viewer).

## Events

### 09:36:47.632       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/documenteditor/main/../../../../doc/recmv7x15laegv9/c/?shardkey=recmv7x15laegv9&EIO=4&transport=websocket"
```

### 09:36:47.634   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "tHVmNFIGHzIoOM1VAAAS",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:36:47.638   <-  Bob             license

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

### 09:36:47.679   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv7x15laegv9",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
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
    "id": "recmv7x15laegv9",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33303/document.docx",
    "title": "document.docx",
    "nobase64": true,
    "outputformat": 8193,
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
  "headingsColor": null,
  "coEditingMode": "fast",
  "time": 330,
  "supportAuthChangesAck": true
}
```

### 09:36:47.699   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "YzuDNbNT3xaC1pY5AAAT",
  "sessionTimeConnect": 1791970607638,
  "participants": [
    {
      "id": "de10808c0010000000000000000000001",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 1,
      "view": true,
      "connectionId": "YzuDNbNT3xaC1pY5AAAT",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4MTVsYWVndjkiLCJwZXJtaXNzaW9ucyI6eyJlZGl0IjpmYWxzZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTA4MDhjMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkJvYiIsImluZGV4IjoxfSwiY29FZGl0aW5nIjp7Im1vZGUiOiJmYXN0In0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDYwNzYzOH0sImlhdCI6MTc5MTk3MDYwNywiZXhwIjoxNzk0NTYyNjA3fQ.bTpwZdxXPQfnUUWhADKWB6_KSNgcicjSx0h992xfZ6o",
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
  "openedAt": 1791977807687
}
```

### 09:36:47.761   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x15laegv9/Editor.bin/Editor.bin?md5=Uq6Tx-JiK88X6WaVx1Yhdw&expires=1794565200&shardkey=recmv7x15laegv9&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv7x15laegv9/media/image1.png/image1.png?md5=Pt7Im6pmQEyNWtqFhyyYQA&expires=1794565200&shardkey=recmv7x15laegv9&filename=image1.png"
    },
    "openedAt": 1791977807687
  }
}
```

### 09:36:47.767   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:4"
}
```

### 09:36:47.983   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:83"
}
```

### 09:36:48.084   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 09:36:48.269   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 09:36:48.269   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:762 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:36:48.270   <-  Bob             message

```json
{
  "type": "message"
}
```

### 09:36:49.446       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/documenteditor/main/../../../../doc/recmv7x15laegv9/c/?shardkey=recmv7x15laegv9&EIO=4&transport=websocket"
```

### 09:36:49.448   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "ni_OQHTHSfZz0IbOAAAU",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:36:49.474   <-  Alice           license

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

### 09:36:49.566   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv7x15laegv9",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
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
    "id": "recmv7x15laegv9",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33303/document.docx",
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
  "time": 301,
  "supportAuthChangesAck": true
}
```

### 09:36:49.578   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "JmggrrN9639B1MDwAAAV",
  "sessionTimeConnect": 1791970609474,
  "participants": [
    {
      "id": "de10808c0010000000000000000000001",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 1,
      "view": true,
      "connectionId": "YzuDNbNT3xaC1pY5AAAT",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000002",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 2,
      "view": false,
      "connectionId": "JmggrrN9639B1MDwAAAV",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4MTVsYWVndjkiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6Mn0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDYwOTQ3NH0sImlhdCI6MTc5MTk3MDYwOSwiZXhwIjoxNzk0NTYyNjA5fQ.bLhENHgkIDuX3xoah6_qZW6K2e0SXUWgU0GGoK6hDSA",
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
  "openedAt": 1791977807687
}
```

### 09:36:49.578   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791970609570,
  "participants": [
    {
      "id": "de10808c0010000000000000000000001",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 1,
      "view": true,
      "connectionId": "YzuDNbNT3xaC1pY5AAAT",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000002",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 2,
      "view": false,
      "connectionId": "JmggrrN9639B1MDwAAAV",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 09:36:49.579   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x15laegv9/Editor.bin/Editor.bin?md5=Uq6Tx-JiK88X6WaVx1Yhdw&expires=1794565200&shardkey=recmv7x15laegv9&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv7x15laegv9/media/image1.png/image1.png?md5=Pt7Im6pmQEyNWtqFhyyYQA&expires=1794565200&shardkey=recmv7x15laegv9&filename=image1.png"
    },
    "openedAt": 1791977807687
  }
}
```

### 09:36:49.592   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 09:36:50.069   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:60"
}
```

### 09:36:50.175   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:107"
}
```

### 09:36:50.341   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:36:50.341   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:944 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:36:50.342   <-  Alice           message

```json
{
  "type": "message"
}
```

### 09:36:53.417   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 09:36:53.418   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 09:36:53.420   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAAAAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAQAAAACAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\",\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAYAAAAEAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000002\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 09:36:53.425   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7x15laegv9",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAAAAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAQAAAACAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAADAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAYAAAAEAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAFAAAAAAAAAAUAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x15laegv9",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791970613000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 24,
  "syncChangesIndex": 24,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000002\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}"
}
```

### 09:36:53.425   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791970613000,
  "syncChangesIndex": 24
}
```

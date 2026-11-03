## Info

- Timestamp: 2026-10-13T15:27:03.124Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Alice edits the document.
  2. Bob edits the document.
  3. Mallory edits the document.

## Events

### 15:26:42.202       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmv6u39zrruld/c/?shardkey=recmv6u39zrruld&EIO=4&transport=websocket"
```

### 15:26:42.204   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "0Xtt0FQOPVIIqyThAAGM",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:26:42.217   <-  Alice           license

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

### 15:26:42.435   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6u39zrruld",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 1,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 257,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmv6u39zrruld",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:33041/spreadsheet.xlsx",
    "title": "spreadsheet.xlsx",
    "lcid": 9,
    "nobase64": true,
    "outputformat": 8194,
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
  "time": 541,
  "supportAuthChangesAck": true
}
```

### 15:26:42.449   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "rZf4V020wqO6G0scAAGN",
  "sessionTimeConnect": 1791905202217,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "rZf4V020wqO6G0scAAGN",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1Mzl6cnJ1bGQiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTIwMjIxN30sImlhdCI6MTc5MTkwNTIwMiwiZXhwIjoxNzk0NDk3MjAyfQ.b2DiCk3hpJVF0ra9n0-ioBZ0Tu3ptOM363H5odxuE3g",
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
  "openedAt": 1791912402441
}
```

### 15:26:42.515   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u39zrruld/Editor.bin/Editor.bin?md5=PxEeDq6EQCEmO1VEWmuxOg&expires=1794499795&shardkey=recmv6u39zrruld&filename=Editor.bin"
    },
    "openedAt": 1791912402441
  }
}
```

### 15:26:42.612   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:16"
}
```

### 15:26:42.652   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:39"
}
```

### 15:26:42.753   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:102"
}
```

### 15:26:42.892   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:26:42.892   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:994 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:26:42.892   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 15:26:42.892   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:26:42.941   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:43.972   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:44.493       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmv6u39zrruld/c/?shardkey=recmv6u39zrruld&EIO=4&transport=websocket"
```

### 15:26:44.495   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "a9nyifkyxMoE0rT8AAGO",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:26:44.518   <-  Bob             license

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

### 15:26:44.694   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6u39zrruld",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10808c001000000000000000000000",
    "username": "Bob",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 1,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 257,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmv6u39zrruld",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:33041/spreadsheet.xlsx",
    "title": "spreadsheet.xlsx",
    "lcid": 9,
    "nobase64": true,
    "outputformat": 8194,
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
  "time": 786,
  "supportAuthChangesAck": true
}
```

### 15:26:44.702   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "rZf4V020wqO6G0scAAGN",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:26:44.702   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905204701,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "rZf4V020wqO6G0scAAGN",
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
      "connectionId": "eWvPwyzeBYj4HdJnAAGP",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:26:44.703   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u39zrruld/Editor.bin/Editor.bin?md5=PxEeDq6EQCEmO1VEWmuxOg&expires=1794499795&shardkey=recmv6u39zrruld&filename=Editor.bin"
    },
    "openedAt": 1791912402441
  }
}
```

### 15:26:44.710   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:26:44.717   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "eWvPwyzeBYj4HdJnAAGP",
  "sessionTimeConnect": 1791905204518,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "rZf4V020wqO6G0scAAGN",
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
      "connectionId": "eWvPwyzeBYj4HdJnAAGP",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1Mzl6cnJ1bGQiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUyMDQ1MTh9LCJpYXQiOjE3OTE5MDUyMDQsImV4cCI6MTc5NDQ5NzIwNH0.0mIU_XUn_WzIFNFgJoFAaCahowYn_SIoXkA0aUnEIUI",
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

### 15:26:44.724   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:3"
}
```

### 15:26:44.753   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:28"
}
```

### 15:26:44.835   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:82"
}
```

### 15:26:44.930   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:26:44.930   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1017 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:26:44.930   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:44.930   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:26:44.982   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:44.982   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905204982,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:46.074       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmv6u39zrruld/c/?shardkey=recmv6u39zrruld&EIO=4&transport=websocket"
```

### 15:26:46.076   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "u_JgpaE9MZQUJHeAAAGQ",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:26:46.113   <-  Mallory         license

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

### 15:26:46.305   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6u39zrruld",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de103a11031c00100000000000000000",
    "username": "Mallory",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 1,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 257,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmv6u39zrruld",
    "userid": "de103a11031c00100000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:33041/spreadsheet.xlsx",
    "title": "spreadsheet.xlsx",
    "lcid": 9,
    "nobase64": true,
    "outputformat": 8194,
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
  "time": 349,
  "supportAuthChangesAck": true
}
```

### 15:26:46.318   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "SmXKdaNcpK0St_jrAAGR",
  "sessionTimeConnect": 1791905206112,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "rZf4V020wqO6G0scAAGN",
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
      "connectionId": "eWvPwyzeBYj4HdJnAAGP",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de103a11031c001000000000000000003",
      "idOriginal": "de103a11031c00100000000000000000",
      "username": "Mallory",
      "indexUser": 3,
      "view": false,
      "connectionId": "SmXKdaNcpK0St_jrAAGR",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1Mzl6cnJ1bGQiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA1MjA2MTEyfSwiaWF0IjoxNzkxOTA1MjA2LCJleHAiOjE3OTQ0OTcyMDZ9.0UU24jnwM59Ks-BPt-ATRssHSifTqwa6pbi-Thtwfso",
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
  "openedAt": 1791912402441
}
```

### 15:26:46.319   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905206312,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "rZf4V020wqO6G0scAAGN",
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
      "connectionId": "eWvPwyzeBYj4HdJnAAGP",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de103a11031c001000000000000000003",
      "idOriginal": "de103a11031c00100000000000000000",
      "username": "Mallory",
      "indexUser": 3,
      "view": false,
      "connectionId": "SmXKdaNcpK0St_jrAAGR",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:26:46.319   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905206312,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "rZf4V020wqO6G0scAAGN",
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
      "connectionId": "eWvPwyzeBYj4HdJnAAGP",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de103a11031c001000000000000000003",
      "idOriginal": "de103a11031c00100000000000000000",
      "username": "Mallory",
      "indexUser": 3,
      "view": false,
      "connectionId": "SmXKdaNcpK0St_jrAAGR",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:26:46.319   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u39zrruld/Editor.bin/Editor.bin?md5=PxEeDq6EQCEmO1VEWmuxOg&expires=1794499795&shardkey=recmv6u39zrruld&filename=Editor.bin"
    },
    "openedAt": 1791912402441
  }
}
```

### 15:26:46.456   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:12"
}
```

### 15:26:46.485   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:29"
}
```

### 15:26:46.535   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:50"
}
```

### 15:26:46.622   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:26:46.622   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:663 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:26:46.622   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:46.623   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:26:46.671   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:46.672   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905206672,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:46.672   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905206672,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:47.669   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 15:26:47.670   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905207669,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:47.670   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905207669,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:47.723   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:47.723   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905207723,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:47.724   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905207723,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:48.116   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "bd503a47-b127-3cf1-eecc-6c1988d80a4d",
      "rangeOrObjectId": {
        "c1": 1,
        "r1": 2,
        "c2": 1,
        "r2": 2
      }
    }
  ]
}
```

### 15:26:48.117   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "bd503a47-b127-3cf1-eecc-6c1988d80a4d": {
      "time": 1791905208117,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "bd503a47-b127-3cf1-eecc-6c1988d80a4d",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 15:26:48.117   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "bd503a47-b127-3cf1-eecc-6c1988d80a4d": {
      "time": 1791905208117,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "bd503a47-b127-3cf1-eecc-6c1988d80a4d",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 15:26:48.117   <-  Mallory         getLock

```json
{
  "type": "getLock",
  "locks": {
    "bd503a47-b127-3cf1-eecc-6c1988d80a4d": {
      "time": 1791905208117,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "bd503a47-b127-3cf1-eecc-6c1988d80a4d",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 15:26:48.128   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 15:26:48.129   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905208129,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:48.129   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905208129,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:48.154   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:48.155   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905208154,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:48.155   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905208154,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:48.551   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:26:48.552   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:48.555   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\",\"14;CgAAAAE1AAAA/wAAAAA=\",\"99;XwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAAA/AAAAAAICAQIBAgkDMgAAAAAAAQkBHwAAAAAIEgAAAEEAbABpAGMAZQA6ACAANAAyAAEAAgADAgECAAMBBAEFAQYB\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:48.562   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\"",
      "time": 1791905208000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905208000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"99;XwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAAA/AAAAAAICAQIBAgkDMgAAAAAAAQkBHwAAAAAIEgAAAEEAbABpAGMAZQA6ACAANAAyAAEAAgADAgECAAMBBAEFAQYB\"",
      "time": 1791905208000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 3,
  "syncChangesIndex": 3,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "bd503a47-b127-3cf1-eecc-6c1988d80a4d",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1791905208562,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:48.562   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\"",
      "time": 1791905208000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905208000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"99;XwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAAA/AAAAAAICAQIBAgkDMgAAAAAAAQkBHwAAAAAIEgAAAEEAbABpAGMAZQA6ACAANAAyAAEAAgADAgECAAMBBAEFAQYB\"",
      "time": 1791905208000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 3,
  "syncChangesIndex": 3,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "bd503a47-b127-3cf1-eecc-6c1988d80a4d",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1791905208562,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:48.562   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791905208000,
  "syncChangesIndex": 3
}
```

### 15:26:48.588   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 3
}
```

### 15:26:48.588   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 3
}
```

### 15:26:48.589   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:48.589   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": true
}
```

### 15:26:48.607   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:48.607   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:48.637   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 3
}
```

### 15:26:48.637   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:48.651   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:48.652   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:48.745   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:48.746   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905208746,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:48.746   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905208746,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:49.154   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 15:26:49.155   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1791905209154,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:49.155   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1791905209154,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:49.181   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:49.182   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905209181,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:49.182   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905209181,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:50.189   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA"
}
```

### 15:26:50.190   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905210189,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:50.190   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905210189,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:50.689   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "c082d16f-7754-b952-69c6-f81fd495eab5",
      "rangeOrObjectId": {
        "c1": 3,
        "r1": 3,
        "c2": 3,
        "r2": 3
      }
    }
  ]
}
```

### 15:26:50.690   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "c082d16f-7754-b952-69c6-f81fd495eab5": {
      "time": 1791905210690,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "c082d16f-7754-b952-69c6-f81fd495eab5",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      }
    }
  }
}
```

### 15:26:50.690   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "c082d16f-7754-b952-69c6-f81fd495eab5": {
      "time": 1791905210690,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "c082d16f-7754-b952-69c6-f81fd495eab5",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      }
    }
  }
}
```

### 15:26:50.690   <-  Mallory         getLock

```json
{
  "type": "getLock",
  "locks": {
    "c082d16f-7754-b952-69c6-f81fd495eab5": {
      "time": 1791905210690,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "c082d16f-7754-b952-69c6-f81fd495eab5",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      }
    }
  }
}
```

### 15:26:50.696   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA"
}
```

### 15:26:50.696   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905210696,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:50.696   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905210696,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:50.711   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:50.712   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905210711,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:50.712   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905210711,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:50.735   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:50.736   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905210736,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:50.736   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905210736,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:51.231   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 3
}
```

### 15:26:51.232   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:51.234   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"105;ZQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABFAAAAAAIDAQIDAgkDOAAAAAAAAQkBJQAAAAAIGAAAAEEAbABpAGMAZQA6ACAASABlAGwAbABvAAEAAgADAgECAAMBBAEFAQYB\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:51.241   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905211000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"105;ZQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABFAAAAAAIDAQIDAgkDOAAAAAAAAQkBJQAAAAAIGAAAAEEAbABpAGMAZQA6ACAASABlAGwAbABvAAEAAgADAgECAAMBBAEFAQYB\"",
      "time": 1791905211000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 5,
  "syncChangesIndex": 5,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "c082d16f-7754-b952-69c6-f81fd495eab5",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1791905211241,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:51.241   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905211000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"105;ZQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABFAAAAAAIDAQIDAgkDOAAAAAAAAQkBJQAAAAAIGAAAAEEAbABpAGMAZQA6ACAASABlAGwAbABvAAEAAgADAgECAAMBBAEFAQYB\"",
      "time": 1791905211000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 5,
  "syncChangesIndex": 5,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "c082d16f-7754-b952-69c6-f81fd495eab5",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1791905211241,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:51.241   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 3,
  "time": 1791905211000,
  "syncChangesIndex": 5
}
```

### 15:26:51.269   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 5
}
```

### 15:26:51.269   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 5
}
```

### 15:26:51.270   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:51.270   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": true
}
```

### 15:26:51.288   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:51.289   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:51.317   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 5
}
```

### 15:26:51.318   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:51.336   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:51.337   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:51.709   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:51.710   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905211709,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:51.710   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905211709,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:51.745   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:51.746   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905211746,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:51.746   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905211746,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:51.771   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:51.772   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905211772,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:51.773   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905211772,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:52.771   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "97d223dc-d564-2ea7-58f7-217300d3e305",
      "rangeOrObjectId": {
        "c1": 1,
        "r1": 2,
        "c2": 1,
        "r2": 2
      }
    }
  ]
}
```

### 15:26:52.771   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "97d223dc-d564-2ea7-58f7-217300d3e305": {
      "time": 1791905212771,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "97d223dc-d564-2ea7-58f7-217300d3e305",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 15:26:52.771   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "97d223dc-d564-2ea7-58f7-217300d3e305": {
      "time": 1791905212771,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "97d223dc-d564-2ea7-58f7-217300d3e305",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 15:26:52.771   <-  Mallory         getLock

```json
{
  "type": "getLock",
  "locks": {
    "97d223dc-d564-2ea7-58f7-217300d3e305": {
      "time": 1791905212771,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "97d223dc-d564-2ea7-58f7-217300d3e305",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 15:26:52.780   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 15:26:52.781   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905212780,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:52.781   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905212780,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:52.805   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:52.806   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905212805,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:52.806   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905212805,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:52.807   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:52.807   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905212807,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:52.808   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905212807,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:53.094   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 5
}
```

### 15:26:53.095   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:53.096   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"95;WwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAAA7AAAAAAICAQIBAgkDLgAAAAAAAQkBGwAAAAAIDgAAAEIAbwBiADoAIAA0ADIAAQACAAMCAQIAAwEEAQUBBgE=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:53.103   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905213000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"95;WwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAAA7AAAAAAICAQIBAgkDLgAAAAAAAQkBGwAAAAAIDgAAAEIAbwBiADoAIAA0ADIAAQACAAMCAQIAAwEEAQUBBgE=\"",
      "time": 1791905213000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 7,
  "syncChangesIndex": 7,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "97d223dc-d564-2ea7-58f7-217300d3e305",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1791905213103,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:53.103   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905213000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"95;WwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAAA7AAAAAAICAQIBAgkDLgAAAAAAAQkBGwAAAAAIDgAAAEIAbwBiADoAIAA0ADIAAQACAAMCAQIAAwEEAQUBBgE=\"",
      "time": 1791905213000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 7,
  "syncChangesIndex": 7,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "97d223dc-d564-2ea7-58f7-217300d3e305",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1791905213103,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:53.104   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 5,
  "time": 1791905213000,
  "syncChangesIndex": 7
}
```

### 15:26:53.119   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 7
}
```

### 15:26:53.119   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 7
}
```

### 15:26:53.119   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:53.119   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": true
}
```

### 15:26:53.142   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:53.142   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:53.158   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 7
}
```

### 15:26:53.159   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:53.179   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:53.179   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:53.809   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 15:26:53.810   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1791905213809,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:53.810   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1791905213809,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:53.837   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:53.837   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:53.837   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905213837,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:53.838   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905213837,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:53.838   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905213837,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:53.838   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905213837,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:54.801   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "100cc57e-bb35-ef01-6bf9-c8c62c7e3df4",
      "rangeOrObjectId": {
        "c1": 3,
        "r1": 3,
        "c2": 3,
        "r2": 3
      }
    }
  ]
}
```

### 15:26:54.801   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "100cc57e-bb35-ef01-6bf9-c8c62c7e3df4": {
      "time": 1791905214801,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "100cc57e-bb35-ef01-6bf9-c8c62c7e3df4",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      }
    }
  }
}
```

### 15:26:54.801   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "100cc57e-bb35-ef01-6bf9-c8c62c7e3df4": {
      "time": 1791905214801,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "100cc57e-bb35-ef01-6bf9-c8c62c7e3df4",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      }
    }
  }
}
```

### 15:26:54.801   <-  Mallory         getLock

```json
{
  "type": "getLock",
  "locks": {
    "100cc57e-bb35-ef01-6bf9-c8c62c7e3df4": {
      "time": 1791905214801,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "100cc57e-bb35-ef01-6bf9-c8c62c7e3df4",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      }
    }
  }
}
```

### 15:26:54.808   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA"
}
```

### 15:26:54.808   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905214808,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:54.808   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905214808,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:54.871   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:54.871   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:54.871   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905214871,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:54.871   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905214871,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:54.871   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905214871,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:54.871   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905214871,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:55.249   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 7
}
```

### 15:26:55.249   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:55.251   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"101;YQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABBAAAAAAIDAQIDAgkDNAAAAAAAAQkBIQAAAAAIFAAAAEIAbwBiADoAIABIAGUAbABsAG8AAQACAAMCAQIAAwEEAQUBBgE=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:55.258   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905215000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"101;YQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABBAAAAAAIDAQIDAgkDNAAAAAAAAQkBIQAAAAAIFAAAAEIAbwBiADoAIABIAGUAbABsAG8AAQACAAMCAQIAAwEEAQUBBgE=\"",
      "time": 1791905215000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 9,
  "syncChangesIndex": 9,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "100cc57e-bb35-ef01-6bf9-c8c62c7e3df4",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1791905215257,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:55.258   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905215000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"101;YQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABBAAAAAAIDAQIDAgkDNAAAAAAAAQkBIQAAAAAIFAAAAEIAbwBiADoAIABIAGUAbABsAG8AAQACAAMCAQIAAwEEAQUBBgE=\"",
      "time": 1791905215000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 9,
  "syncChangesIndex": 9,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "100cc57e-bb35-ef01-6bf9-c8c62c7e3df4",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1791905215257,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:55.258   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 7,
  "time": 1791905215000,
  "syncChangesIndex": 9
}
```

### 15:26:55.272   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 9
}
```

### 15:26:55.272   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:55.291   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:55.291   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:55.304   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 9
}
```

### 15:26:55.305   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:55.324   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:55.325   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:55.809   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:55.809   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905215809,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:55.809   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905215809,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:55.872   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:26:55.872   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:55.873   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905215873,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:55.873   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905215873,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:55.873   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905215873,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:55.873   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905215873,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:56.810   ->  Mallory         getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "6e8c679a-e3ae-860a-edd4-a42e497e4228",
      "rangeOrObjectId": {
        "c1": 1,
        "r1": 2,
        "c2": 1,
        "r2": 2
      }
    }
  ]
}
```

### 15:26:56.811   <-  Mallory         getLock

```json
{
  "type": "getLock",
  "locks": {
    "6e8c679a-e3ae-860a-edd4-a42e497e4228": {
      "time": 1791905216811,
      "user": "de103a11031c001000000000000000003",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6e8c679a-e3ae-860a-edd4-a42e497e4228",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 15:26:56.811   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "6e8c679a-e3ae-860a-edd4-a42e497e4228": {
      "time": 1791905216811,
      "user": "de103a11031c001000000000000000003",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6e8c679a-e3ae-860a-edd4-a42e497e4228",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 15:26:56.811   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "6e8c679a-e3ae-860a-edd4-a42e497e4228": {
      "time": 1791905216811,
      "user": "de103a11031c001000000000000000003",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6e8c679a-e3ae-860a-edd4-a42e497e4228",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 15:26:56.820   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 15:26:56.820   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905216820,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:56.820   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905216820,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:56.872   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:56.872   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905216872,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:56.872   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905216872,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:56.921   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:56.922   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905216922,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:56.922   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905216922,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:57.305   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 9
}
```

### 15:26:57.306   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:57.318   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"103;YwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABDAAAAAAICAQIBAgkDNgAAAAAAAQkBIwAAAAAIFgAAAE0AYQBsAGwAbwByAHkAOgAgADQAMgABAAIAAwIBAgADAQQBBQEGAQ==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:57.325   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905217000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"103;YwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABDAAAAAAICAQIBAgkDNgAAAAAAAQkBIwAAAAAIFgAAAE0AYQBsAGwAbwByAHkAOgAgADQAMgABAAIAAwIBAgADAQQBBQEGAQ==\"",
      "time": 1791905217000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 11,
  "syncChangesIndex": 11,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6e8c679a-e3ae-860a-edd4-a42e497e4228",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de103a11031c001000000000000000003",
      "time": 1791905217325,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:57.326   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905217000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"103;YwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABDAAAAAAICAQIBAgkDNgAAAAAAAQkBIwAAAAAIFgAAAE0AYQBsAGwAbwByAHkAOgAgADQAMgABAAIAAwIBAgADAQQBBQEGAQ==\"",
      "time": 1791905217000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 11,
  "syncChangesIndex": 11,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6e8c679a-e3ae-860a-edd4-a42e497e4228",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de103a11031c001000000000000000003",
      "time": 1791905217325,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:57.326   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 9,
  "time": 1791905217000,
  "syncChangesIndex": 11
}
```

### 15:26:57.354   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 11
}
```

### 15:26:57.354   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:57.367   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 11
}
```

### 15:26:57.368   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": true
}
```

### 15:26:57.376   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:57.376   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:57.406   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 11
}
```

### 15:26:57.407   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:57.419   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:57.420   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:57.825   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 15:26:57.826   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1791905217825,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:57.826   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1791905217825,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:57.889   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:57.889   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905217889,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:57.890   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905217889,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:57.949   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:57.950   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905217949,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:57.950   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905217949,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:58.865   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA"
}
```

### 15:26:58.866   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905218866,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:58.866   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905218866,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:59.305   ->  Mallory         getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "c4e3b197-e324-c525-6ed9-574e3e606a81",
      "rangeOrObjectId": {
        "c1": 3,
        "r1": 3,
        "c2": 3,
        "r2": 3
      }
    }
  ]
}
```

### 15:26:59.306   <-  Mallory         getLock

```json
{
  "type": "getLock",
  "locks": {
    "c4e3b197-e324-c525-6ed9-574e3e606a81": {
      "time": 1791905219306,
      "user": "de103a11031c001000000000000000003",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "c4e3b197-e324-c525-6ed9-574e3e606a81",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      }
    }
  }
}
```

### 15:26:59.306   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "c4e3b197-e324-c525-6ed9-574e3e606a81": {
      "time": 1791905219306,
      "user": "de103a11031c001000000000000000003",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "c4e3b197-e324-c525-6ed9-574e3e606a81",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      }
    }
  }
}
```

### 15:26:59.306   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "c4e3b197-e324-c525-6ed9-574e3e606a81": {
      "time": 1791905219306,
      "user": "de103a11031c001000000000000000003",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "c4e3b197-e324-c525-6ed9-574e3e606a81",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      }
    }
  }
}
```

### 15:26:59.313   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA"
}
```

### 15:26:59.313   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905219313,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:59.314   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905219313,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:59.322   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:59.323   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905219322,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:59.323   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905219322,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:59.340   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:26:59.340   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905219340,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:59.340   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905219340,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:59.915   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 11
}
```

### 15:26:59.916   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:59.917   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"109;aQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABJAAAAAAIDAQIDAgkDPAAAAAAAAQkBKQAAAAAIHAAAAE0AYQBsAGwAbwByAHkAOgAgAEgAZQBsAGwAbwABAAIAAwIBAgADAQQBBQEGAQ==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:59.923   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905219000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"109;aQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABJAAAAAAIDAQIDAgkDPAAAAAAAAQkBKQAAAAAIHAAAAE0AYQBsAGwAbwByAHkAOgAgAEgAZQBsAGwAbwABAAIAAwIBAgADAQQBBQEGAQ==\"",
      "time": 1791905219000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 13,
  "syncChangesIndex": 13,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "c4e3b197-e324-c525-6ed9-574e3e606a81",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      },
      "user": "de103a11031c001000000000000000003",
      "time": 1791905219923,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:59.924   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u39zrruld",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791905219000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u39zrruld",
      "change": "\"109;aQAAAAE1EAECAAAANgABAwAAAAMAAAADAAAAAwAAAABJAAAAAAIDAQIDAgkDPAAAAAAAAQkBKQAAAAAIHAAAAE0AYQBsAGwAbwByAHkAOgAgAEgAZQBsAGwAbwABAAIAAwIBAgADAQQBBQEGAQ==\"",
      "time": 1791905219000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 13,
  "syncChangesIndex": 13,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "c4e3b197-e324-c525-6ed9-574e3e606a81",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 3,
          "c2": 3,
          "r2": 3
        }
      },
      "user": "de103a11031c001000000000000000003",
      "time": 1791905219923,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 15:26:59.924   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 11,
  "time": 1791905219000,
  "syncChangesIndex": 13
}
```

### 15:26:59.937   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 13
}
```

### 15:26:59.938   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:59.958   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:59.959   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:26:59.974   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 13
}
```

### 15:26:59.974   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:59.994   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:59.994   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 15:27:00.345   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:27:00.346   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905220346,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:27:00.346   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905220346,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:27:00.362   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:27:00.363   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905220363,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:27:00.363   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905220363,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:27:00.389   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA"
}
```

### 15:27:00.389   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905220389,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:27:00.389   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwANAAsADMALAA0ACwA",
      "time": 1791905220389,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

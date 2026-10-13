## Info

- Timestamp: 2026-10-13T15:22:50.422Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario: Alice opens the document, then Bob and Mallory join at the same time: their auth sends are held at the proxy and released back to back, so both auths reach the server before Alice sends her unLockDocument. The server waitAuths the first joiner (Bob) and resolves the document lock optimistically for the second (Mallory authenticates directly) — the real OnlyOffice 9.3.0 behavior.

## Events

### 15:22:43.167       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6ty60p3lzz/c/?shardkey=recmv6ty60p3lzz&EIO=4&transport=websocket"
```

### 15:22:43.169   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "01QfGT3GzlP4fia4AAEw",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:22:43.174   <-  Alice           license

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

### 15:22:43.289   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6ty60p3lzz",
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
    "id": "recmv6ty60p3lzz",
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
  "time": 394,
  "supportAuthChangesAck": true
}
```

### 15:22:43.307   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "w10I7oQoFyuEQTuQAAEx",
  "sessionTimeConnect": 1791904963174,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "w10I7oQoFyuEQTuQAAEx",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0eTYwcDNsenoiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNDk2MzE3NH0sImlhdCI6MTc5MTkwNDk2MywiZXhwIjoxNzk0NDk2OTYzfQ.su6ikMlSyW8FnkfyTeg52JCmS4X4Q4p_N24SHk7jCwk",
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
  "openedAt": 1791912163297
}
```

### 15:22:43.367   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6ty60p3lzz/Editor.bin/Editor.bin?md5=Hv3ut2bdyennzjEQdQCALA&expires=1794499556&shardkey=recmv6ty60p3lzz&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6ty60p3lzz/media/image1.png/image1.png?md5=9-pQiws8RKIRagz1npc0Ew&expires=1794499556&shardkey=recmv6ty60p3lzz&filename=image1.png"
    },
    "openedAt": 1791912163297
  }
}
```

### 15:22:43.374   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:3"
}
```

### 15:22:43.514   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:83"
}
```

### 15:22:43.617   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:103"
}
```

### 15:22:43.827   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:22:43.827   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:759 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:22:43.828   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:22:45.473       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6ty60p3lzz/c/?shardkey=recmv6ty60p3lzz&EIO=4&transport=websocket"
```

### 15:22:45.529       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6ty60p3lzz/c/?shardkey=recmv6ty60p3lzz&EIO=4&transport=websocket"
```

### 15:22:45.537   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "oK-FqTD1SZD7JhcWAAEy",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:22:45.557   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "mcGr0psZfohNo_kgAAEz",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:22:45.638   <-  Bob             license

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

### 15:22:45.679   <-  Mallory         license

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

### 15:22:45.799   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6ty60p3lzz",
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
    "id": "recmv6ty60p3lzz",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33041/document.docx",
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
  "time": 877,
  "supportAuthChangesAck": true
}
```

### 15:22:45.819   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6ty60p3lzz",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de103a11031c00100000000000000000",
    "username": "Mallory",
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
    "id": "recmv6ty60p3lzz",
    "userid": "de103a11031c00100000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33041/document.docx",
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
  "time": 952,
  "supportAuthChangesAck": true
}
```

### 15:22:45.839   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "w10I7oQoFyuEQTuQAAEx",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:22:45.890   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791904965803,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "w10I7oQoFyuEQTuQAAEx",
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
      "connectionId": "qbdHX8Cp3aoFX8bGAAE0",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:22:45.940   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6ty60p3lzz/Editor.bin/Editor.bin?md5=Hv3ut2bdyennzjEQdQCALA&expires=1794499556&shardkey=recmv6ty60p3lzz&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6ty60p3lzz/media/image1.png/image1.png?md5=9-pQiws8RKIRagz1npc0Ew&expires=1794499556&shardkey=recmv6ty60p3lzz&filename=image1.png"
    },
    "openedAt": 1791912163297
  }
}
```

### 15:22:45.990   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "_Pd3nooRpdS8o8fNAAE1",
  "sessionTimeConnect": 1791904965598,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "w10I7oQoFyuEQTuQAAEx",
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
      "connectionId": "qbdHX8Cp3aoFX8bGAAE0",
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
      "connectionId": "_Pd3nooRpdS8o8fNAAE1",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0eTYwcDNsenoiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA0OTY1NTk4fSwiaWF0IjoxNzkxOTA0OTY1LCJleHAiOjE3OTQ0OTY5NjV9.FyZHIgpEQmqCbhU_HifgJDKJDs8RhAdM292WMOCnEXk",
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
  "openedAt": 1791912163297
}
```

### 15:22:46.040   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791904965822,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "w10I7oQoFyuEQTuQAAEx",
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
      "connectionId": "qbdHX8Cp3aoFX8bGAAE0",
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
      "connectionId": "_Pd3nooRpdS8o8fNAAE1",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:22:46.090   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791904965822,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "w10I7oQoFyuEQTuQAAEx",
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
      "connectionId": "qbdHX8Cp3aoFX8bGAAE0",
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
      "connectionId": "_Pd3nooRpdS8o8fNAAE1",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:22:46.141   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6ty60p3lzz/Editor.bin/Editor.bin?md5=Hv3ut2bdyennzjEQdQCALA&expires=1794499556&shardkey=recmv6ty60p3lzz&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6ty60p3lzz/media/image1.png/image1.png?md5=9-pQiws8RKIRagz1npc0Ew&expires=1794499556&shardkey=recmv6ty60p3lzz&filename=image1.png"
    },
    "openedAt": 1791912163297
  }
}
```

### 15:22:46.191   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:22:46.241   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:22:46.293   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 15:22:46.343   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:45"
}
```

### 15:22:46.394   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:104"
}
```

### 15:22:46.444   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:1"
}
```

### 15:22:46.495   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:39"
}
```

### 15:22:46.545   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "qbdHX8Cp3aoFX8bGAAE0",
  "sessionTimeConnect": 1791904965577,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "w10I7oQoFyuEQTuQAAEx",
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
      "connectionId": "qbdHX8Cp3aoFX8bGAAE0",
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
      "connectionId": "_Pd3nooRpdS8o8fNAAE1",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0eTYwcDNsenoiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDQ5NjU1Nzd9LCJpYXQiOjE3OTE5MDQ5NjYsImV4cCI6MTc5NDQ5Njk2Nn0.XlOvbnULyaTyFMZPjeW5dnHeBPjjQ6b0f-gAPXSFcXs",
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

### 15:22:46.595   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "_Pd3nooRpdS8o8fNAAE1",
  "sessionTimeConnect": 1791904965598,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "w10I7oQoFyuEQTuQAAEx",
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
      "connectionId": "qbdHX8Cp3aoFX8bGAAE0",
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
      "connectionId": "_Pd3nooRpdS8o8fNAAE1",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 3,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0eTYwcDNsenoiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA0OTY1NTk4fSwiaWF0IjoxNzkxOTA0OTY2LCJleHAiOjE3OTQ0OTY5NjZ9.wyDkmKOhDH2P0AuOL64mfDc8X3kPaIcWBmxC3UH4eFY",
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

### 15:22:46.646   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791904966242,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:22:46.696   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791904966242,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:22:46.746   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:103"
}
```

### 15:22:46.796   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:22:46.846   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1483 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:22:46.897   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:22:46.947   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:22:46.997   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1757 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:22:47.047   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:22:47.097   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:22:47.147   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791904966897,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:22:47.197   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791904966897,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:22:47.249   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:22:47.300   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791904967048,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:22:47.350   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791904967048,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

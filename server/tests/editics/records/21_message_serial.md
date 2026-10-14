## Info

- Timestamp: 2026-10-13T15:25:01.912Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario: Alice, Bob and Mallory send chat messages one after the other.

## Events

### 15:24:51.898       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u0xci2kr0/c/?shardkey=recmv6u0xci2kr0&EIO=4&transport=websocket"
```

### 15:24:51.899   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "X7_qacC8Q-LEfW_qAAFo",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:24:51.907   <-  Alice           license

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

### 15:24:52.060   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6u0xci2kr0",
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
    "id": "recmv6u0xci2kr0",
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
  "time": 437,
  "supportAuthChangesAck": true
}
```

### 15:24:52.073   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "kxCOp2Ghyl93sviDAAFp",
  "sessionTimeConnect": 1791905091907,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kxCOp2Ghyl93sviDAAFp",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MHhjaTJrcjAiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTA5MTkwN30sImlhdCI6MTc5MTkwNTA5MiwiZXhwIjoxNzk0NDk3MDkyfQ.AdIMK_ToR8ZQTamkh9fGpCeHxIK6bzAU-HJ1on9lyNI",
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
  "openedAt": 1791912292064
}
```

### 15:24:52.134   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u0xci2kr0/Editor.bin/Editor.bin?md5=jMcyUDQnQ0EZsYsvI34XNA&expires=1794499685&shardkey=recmv6u0xci2kr0&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u0xci2kr0/media/image1.png/image1.png?md5=1vCh4g57p0qKLT6-cJR3Pg&expires=1794499685&shardkey=recmv6u0xci2kr0&filename=image1.png"
    },
    "openedAt": 1791912292064
  }
}
```

### 15:24:52.185   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:10"
}
```

### 15:24:52.270   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:85"
}
```

### 15:24:52.373   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:104"
}
```

### 15:24:52.581   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:24:52.581   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:786 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:24:52.582   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:24:54.111       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u0xci2kr0/c/?shardkey=recmv6u0xci2kr0&EIO=4&transport=websocket"
```

### 15:24:54.113   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "7j2n_4mnF-ytEnGJAAFq",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:24:54.142   <-  Bob             license

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

### 15:24:54.248   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6u0xci2kr0",
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
    "id": "recmv6u0xci2kr0",
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
  "time": 662,
  "supportAuthChangesAck": true
}
```

### 15:24:54.256   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "kxCOp2Ghyl93sviDAAFp",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:24:54.256   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905094255,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kxCOp2Ghyl93sviDAAFp",
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
      "connectionId": "WAzSEEMPkpWfYVJWAAFr",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:24:54.257   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u0xci2kr0/Editor.bin/Editor.bin?md5=jMcyUDQnQ0EZsYsvI34XNA&expires=1794499685&shardkey=recmv6u0xci2kr0&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u0xci2kr0/media/image1.png/image1.png?md5=1vCh4g57p0qKLT6-cJR3Pg&expires=1794499685&shardkey=recmv6u0xci2kr0&filename=image1.png"
    },
    "openedAt": 1791912292064
  }
}
```

### 15:24:54.264   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:24:54.270   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "WAzSEEMPkpWfYVJWAAFr",
  "sessionTimeConnect": 1791905094141,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kxCOp2Ghyl93sviDAAFp",
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
      "connectionId": "WAzSEEMPkpWfYVJWAAFr",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MHhjaTJrcjAiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUwOTQxNDF9LCJpYXQiOjE3OTE5MDUwOTQsImV4cCI6MTc5NDQ5NzA5NH0.A9oThVNJsnaCHukcwmgHL2iUGCop-Mbq0-GaKfgqaXM",
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

### 15:24:54.274   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:5"
}
```

### 15:24:54.332   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:58"
}
```

### 15:24:54.344   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:24:54.344   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905094344,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:24:54.434   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:102"
}
```

### 15:24:54.575   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:24:54.575   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:875 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:24:54.575   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:24:54.613   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:24:54.614   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905094614,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:24:55.652       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u0xci2kr0/c/?shardkey=recmv6u0xci2kr0&EIO=4&transport=websocket"
```

### 15:24:55.653   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "t8_gpdbJ-GRRIp_mAAFs",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:24:55.679   <-  Mallory         license

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

### 15:24:55.791   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6u0xci2kr0",
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
    "id": "recmv6u0xci2kr0",
    "userid": "de103a11031c00100000000000000000",
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
  "time": 231,
  "supportAuthChangesAck": true
}
```

### 15:24:55.804   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "PLO8B7RV_iwqg4LQAAFt",
  "sessionTimeConnect": 1791905095678,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kxCOp2Ghyl93sviDAAFp",
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
      "connectionId": "WAzSEEMPkpWfYVJWAAFr",
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
      "connectionId": "PLO8B7RV_iwqg4LQAAFt",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MHhjaTJrcjAiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA1MDk1Njc4fSwiaWF0IjoxNzkxOTA1MDk1LCJleHAiOjE3OTQ0OTcwOTV9.GD8dKp4-WLwyk_bQAikZT_egRet5UBL3DsdErRyhhSw",
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
  "openedAt": 1791912292064
}
```

### 15:24:55.804   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905095798,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kxCOp2Ghyl93sviDAAFp",
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
      "connectionId": "WAzSEEMPkpWfYVJWAAFr",
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
      "connectionId": "PLO8B7RV_iwqg4LQAAFt",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:24:55.804   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905095798,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kxCOp2Ghyl93sviDAAFp",
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
      "connectionId": "WAzSEEMPkpWfYVJWAAFr",
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
      "connectionId": "PLO8B7RV_iwqg4LQAAFt",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:24:55.805   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u0xci2kr0/Editor.bin/Editor.bin?md5=jMcyUDQnQ0EZsYsvI34XNA&expires=1794499685&shardkey=recmv6u0xci2kr0&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u0xci2kr0/media/image1.png/image1.png?md5=1vCh4g57p0qKLT6-cJR3Pg&expires=1794499685&shardkey=recmv6u0xci2kr0&filename=image1.png"
    },
    "openedAt": 1791912292064
  }
}
```

### 15:24:55.926   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:102"
}
```

### 15:24:55.972   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:46"
}
```

### 15:24:56.079   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:108"
}
```

### 15:24:56.227   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:24:56.227   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:551 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:24:56.228   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:24:56.267   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:24:56.268   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905096268,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:24:56.268   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905096268,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:24:57.596   ->  Alice           message

```json
{
  "type": "message",
  "message": "Hello from Alice"
}
```

### 15:24:57.596   <-  Alice           message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u0xci2kr0",
      "message": "Hello from Alice",
      "time": 1791905097596,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 15:24:57.597   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u0xci2kr0",
      "message": "Hello from Alice",
      "time": 1791905097596,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 15:24:57.597   <-  Mallory         message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u0xci2kr0",
      "message": "Hello from Alice",
      "time": 1791905097596,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 15:24:58.446   ->  Bob             message

```json
{
  "type": "message",
  "message": "Hello from Bob"
}
```

### 15:24:58.446   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u0xci2kr0",
      "message": "Hello from Bob",
      "time": 1791905098446,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000",
      "username": "Bob"
    }
  ]
}
```

### 15:24:58.446   <-  Alice           message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u0xci2kr0",
      "message": "Hello from Bob",
      "time": 1791905098446,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000",
      "username": "Bob"
    }
  ]
}
```

### 15:24:58.446   <-  Mallory         message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u0xci2kr0",
      "message": "Hello from Bob",
      "time": 1791905098446,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000",
      "username": "Bob"
    }
  ]
}
```

### 15:24:59.296   ->  Mallory         message

```json
{
  "type": "message",
  "message": "Hello from Mallory"
}
```

### 15:24:59.296   <-  Mallory         message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u0xci2kr0",
      "message": "Hello from Mallory",
      "time": 1791905099296,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000",
      "username": "Mallory"
    }
  ]
}
```

### 15:24:59.296   <-  Alice           message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u0xci2kr0",
      "message": "Hello from Mallory",
      "time": 1791905099296,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000",
      "username": "Mallory"
    }
  ]
}
```

### 15:24:59.296   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u0xci2kr0",
      "message": "Hello from Mallory",
      "time": 1791905099296,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000",
      "username": "Mallory"
    }
  ]
}
```

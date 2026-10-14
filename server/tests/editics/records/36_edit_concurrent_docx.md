## Info

- Timestamp: 2026-10-13T15:26:12.891Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario: Alice, Bob and Mallory start editing at the same time: their isSaveLocks are held by manual-flow and released back to back, so the server sees the three lock requests before granting any save lock.

## Events

### 15:26:00.271       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u2drm3pt6/c/?shardkey=recmv6u2drm3pt6&EIO=4&transport=websocket"
```

### 15:26:00.273   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "WNRx1ZRtRVP_OXW6AAGA",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:26:00.283   <-  Alice           license

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

### 15:26:00.468   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6u2drm3pt6",
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
    "id": "recmv6u2drm3pt6",
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
  "time": 507,
  "supportAuthChangesAck": true
}
```

### 15:26:00.484   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "KkHLXKwLz_EwfQO7AAGB",
  "sessionTimeConnect": 1791905160283,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "KkHLXKwLz_EwfQO7AAGB",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MmRybTNwdDYiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTE2MDI4M30sImlhdCI6MTc5MTkwNTE2MCwiZXhwIjoxNzk0NDk3MTYwfQ.DHTBRkKcHXMfcxlNkY4N2Sb86JEdIzzdCqVmrXtrIYs",
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
  "openedAt": 1791912360476
}
```

### 15:26:00.541   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u2drm3pt6/Editor.bin/Editor.bin?md5=VErZTasEq0-wgpzJj6JS2A&expires=1794499753&shardkey=recmv6u2drm3pt6&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u2drm3pt6/media/image1.png/image1.png?md5=SY1W4BrILW1gWGhMi2xStA&expires=1794499753&shardkey=recmv6u2drm3pt6&filename=image1.png"
    },
    "openedAt": 1791912360476
  }
}
```

### 15:26:00.588   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:11"
}
```

### 15:26:00.674   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:85"
}
```

### 15:26:00.776   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:103"
}
```

### 15:26:00.976   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:26:00.976   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:849 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:26:00.976   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:26:02.143       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u2drm3pt6/c/?shardkey=recmv6u2drm3pt6&EIO=4&transport=websocket"
```

### 15:26:02.145   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "cw_7bzkrFvUmpEEKAAGC",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:26:02.191   <-  Bob             license

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

### 15:26:02.334   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6u2drm3pt6",
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
    "id": "recmv6u2drm3pt6",
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
  "time": 344,
  "supportAuthChangesAck": true
}
```

### 15:26:02.343   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "KkHLXKwLz_EwfQO7AAGB",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:26:02.343   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905162341,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "KkHLXKwLz_EwfQO7AAGB",
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
      "connectionId": "agIMkvMKTsYbX83lAAGD",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:26:02.344   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u2drm3pt6/Editor.bin/Editor.bin?md5=VErZTasEq0-wgpzJj6JS2A&expires=1794499753&shardkey=recmv6u2drm3pt6&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u2drm3pt6/media/image1.png/image1.png?md5=SY1W4BrILW1gWGhMi2xStA&expires=1794499753&shardkey=recmv6u2drm3pt6&filename=image1.png"
    },
    "openedAt": 1791912360476
  }
}
```

### 15:26:02.348   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:26:02.355   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "agIMkvMKTsYbX83lAAGD",
  "sessionTimeConnect": 1791905162191,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "KkHLXKwLz_EwfQO7AAGB",
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
      "connectionId": "agIMkvMKTsYbX83lAAGD",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MmRybTNwdDYiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUxNjIxOTF9LCJpYXQiOjE3OTE5MDUxNjIsImV4cCI6MTc5NDQ5NzE2Mn0.wjrTX67WriXJB9kumXtBW2XOzjIUGGRopnMRZT_Ui-M",
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

### 15:26:02.735   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:26:02.736   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905162736,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:02.751   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:17"
}
```

### 15:26:02.805   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:53"
}
```

### 15:26:02.922   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:118"
}
```

### 15:26:03.067   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:26:03.067   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:962 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:26:03.068   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:26:03.115   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:26:03.116   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905163116,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:04.173       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u2drm3pt6/c/?shardkey=recmv6u2drm3pt6&EIO=4&transport=websocket"
```

### 15:26:04.175   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "Q9aTAKQ3IUc6e9TdAAGE",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:26:04.210   <-  Mallory         license

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

### 15:26:04.396   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6u2drm3pt6",
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
    "id": "recmv6u2drm3pt6",
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
  "time": 318,
  "supportAuthChangesAck": true
}
```

### 15:26:04.407   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "gR_bl9GLJEC1JWPEAAGF",
  "sessionTimeConnect": 1791905164210,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "KkHLXKwLz_EwfQO7AAGB",
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
      "connectionId": "agIMkvMKTsYbX83lAAGD",
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
      "connectionId": "gR_bl9GLJEC1JWPEAAGF",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MmRybTNwdDYiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA1MTY0MjEwfSwiaWF0IjoxNzkxOTA1MTY0LCJleHAiOjE3OTQ0OTcxNjR9.2a96dtuhnbHPrI1AE4wfnNnVOe0RbFi_dqpb7bPm8oY",
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
  "openedAt": 1791912360476
}
```

### 15:26:04.408   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905164402,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "KkHLXKwLz_EwfQO7AAGB",
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
      "connectionId": "agIMkvMKTsYbX83lAAGD",
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
      "connectionId": "gR_bl9GLJEC1JWPEAAGF",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:26:04.408   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905164402,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "KkHLXKwLz_EwfQO7AAGB",
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
      "connectionId": "agIMkvMKTsYbX83lAAGD",
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
      "connectionId": "gR_bl9GLJEC1JWPEAAGF",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:26:04.408   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u2drm3pt6/Editor.bin/Editor.bin?md5=VErZTasEq0-wgpzJj6JS2A&expires=1794499753&shardkey=recmv6u2drm3pt6&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u2drm3pt6/media/image1.png/image1.png?md5=SY1W4BrILW1gWGhMi2xStA&expires=1794499753&shardkey=recmv6u2drm3pt6&filename=image1.png"
    },
    "openedAt": 1791912360476
  }
}
```

### 15:26:04.437   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:15"
}
```

### 15:26:04.477   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:40"
}
```

### 15:26:04.579   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:103"
}
```

### 15:26:04.715   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:26:04.716   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:529 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:26:04.716   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:26:04.767   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:26:04.767   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905164767,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:04.768   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905164767,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:05.790   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAIAAAA="
}
```

### 15:26:05.791   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAIAAAA="
}
```

### 15:26:05.791   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905165791,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:05.791   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905165791,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:05.791   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905165791,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:05.791   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905165791,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:26:05.816   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAIAAAA="
}
```

### 15:26:05.817   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905165816,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:05.817   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905165816,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:26:07.058   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:26:07.109   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:26:07.159   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:26:07.210   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:07.260   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": true
}
```

### 15:26:07.310   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": true
}
```

### 15:26:07.361   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABBAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABsAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABpAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABjAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:07.412   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:26:07.462   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:26:07.512   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 6,
  "syncChangesIndex": 6,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}"
}
```

### 15:26:07.562   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAUAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 6,
  "syncChangesIndex": 6,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}"
}
```

### 15:26:07.614   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791905167000,
  "syncChangesIndex": 6
}
```

### 15:26:07.665   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:07.714   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": true
}
```

### 15:26:07.765   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAcAAAA="
}
```

### 15:26:07.815   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABCAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABiAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:07.866   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 6
}
```

### 15:26:07.916   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAcAAAA=",
      "time": 1791905167765,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:07.968   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAcAAAA=",
      "time": 1791905167765,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:26:08.018   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905167000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABCAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABiAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 10,
  "syncChangesIndex": 10,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}"
}
```

### 15:26:08.069   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905167000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABwAAAAEAAABCAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAABiAAAAAAMAAAA=\"",
      "time": 1791905167000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 10,
  "syncChangesIndex": 10,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}"
}
```

### 15:26:08.120   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 6,
  "time": 1791905167000,
  "syncChangesIndex": 10
}
```

### 15:26:08.170   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": true
}
```

### 15:26:08.221   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:08.271   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 10
}
```

### 15:26:08.321   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:26:08.372   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABNAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABhAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:26:08.422   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905168000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABNAAAAAAMAAAA=\"",
      "time": 1791905168000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791905168000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905168000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 14,
  "syncChangesIndex": 14,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}"
}
```

### 15:26:08.473   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905168000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABNAAAAAAMAAAA=\"",
      "time": 1791905168000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791905168000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u2drm3pt6",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905168000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 14,
  "syncChangesIndex": 14,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}"
}
```

### 15:26:08.523   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 10,
  "time": 1791905168000,
  "syncChangesIndex": 14
}
```

### 15:26:08.573   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:26:08.623   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

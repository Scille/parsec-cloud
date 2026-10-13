## Info

- Timestamp: 2026-10-13T15:24:29.942Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: presentation.pptx (slide)
- Scenario: Alice, Bob and Mallory move their cursors at the same time (concurrent).

## Events

### 15:24:18.984       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6u07ybsnhp/c/?shardkey=recmv6u07ybsnhp&EIO=4&transport=websocket"
```

### 15:24:18.986   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "y_NAUwzA4GlkFQVCAAFc",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:24:19.003   <-  Alice           license

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

### 15:24:19.148   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6u07ybsnhp",
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
    "id": "recmv6u07ybsnhp",
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
  "time": 435,
  "supportAuthChangesAck": true
}
```

### 15:24:19.162   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "lAZc1eFd1uYQhJIrAAFd",
  "sessionTimeConnect": 1791905059002,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "lAZc1eFd1uYQhJIrAAFd",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MDd5YnNuaHAiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTA1OTAwMn0sImlhdCI6MTc5MTkwNTA1OSwiZXhwIjoxNzk0NDk3MDU5fQ.QWhPxFfGWxxdlYeUY4O4fdkOyODx-SaWIZkOmJhDjow",
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
  "openedAt": 1791912259153
}
```

### 15:24:19.258   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u07ybsnhp/Editor.bin/Editor.bin?md5=AeTejW9naHCsrtjwRsWL-Q&expires=1794499652&shardkey=recmv6u07ybsnhp&filename=Editor.bin"
    },
    "openedAt": 1791912259153
  }
}
```

### 15:24:19.264   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:4"
}
```

### 15:24:19.353   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:89"
}
```

### 15:24:19.454   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 15:24:19.743   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:24:19.744   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1017 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:24:19.744   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:24:20.936       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6u07ybsnhp/c/?shardkey=recmv6u07ybsnhp&EIO=4&transport=websocket"
```

### 15:24:20.937   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "wgjz-QXmNuNnLMBfAAFe",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:24:20.969   <-  Bob             license

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

### 15:24:21.065   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6u07ybsnhp",
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
    "id": "recmv6u07ybsnhp",
    "userid": "de10808c001000000000000000000000",
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
  "time": 284,
  "supportAuthChangesAck": true
}
```

### 15:24:21.072   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "lAZc1eFd1uYQhJIrAAFd",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:24:21.072   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905061070,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "lAZc1eFd1uYQhJIrAAFd",
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
      "connectionId": "CwSUJYh5kZ1f18MsAAFf",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:24:21.072   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u07ybsnhp/Editor.bin/Editor.bin?md5=AeTejW9naHCsrtjwRsWL-Q&expires=1794499652&shardkey=recmv6u07ybsnhp&filename=Editor.bin"
    },
    "openedAt": 1791912259153
  }
}
```

### 15:24:21.078   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:24:21.084   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "CwSUJYh5kZ1f18MsAAFf",
  "sessionTimeConnect": 1791905060969,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "lAZc1eFd1uYQhJIrAAFd",
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
      "connectionId": "CwSUJYh5kZ1f18MsAAFf",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MDd5YnNuaHAiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUwNjA5Njl9LCJpYXQiOjE3OTE5MDUwNjEsImV4cCI6MTc5NDQ5NzA2MX0.hGQMtZUXub1IsIqwXhB95z8H5_W7jE1GGwM2pYSuT3g",
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

### 15:24:21.471   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:378"
}
```

### 15:24:21.531   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:60"
}
```

### 15:24:21.583   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:52"
}
```

### 15:24:21.798   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:24:21.798   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1007 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:24:21.798   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:24:22.936       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6u07ybsnhp/c/?shardkey=recmv6u07ybsnhp&EIO=4&transport=websocket"
```

### 15:24:22.937   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "HDwBwQlvA5rDQqAqAAFg",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:24:23.006   <-  Mallory         license

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

### 15:24:23.098   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6u07ybsnhp",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de103a11031c00100000000000000000",
    "username": "Mallory",
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
    "id": "recmv6u07ybsnhp",
    "userid": "de103a11031c00100000000000000000",
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
  "time": 267,
  "supportAuthChangesAck": true
}
```

### 15:24:23.112   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "h6TdahtLyhJni1cdAAFh",
  "sessionTimeConnect": 1791905063006,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "lAZc1eFd1uYQhJIrAAFd",
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
      "connectionId": "CwSUJYh5kZ1f18MsAAFf",
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
      "connectionId": "h6TdahtLyhJni1cdAAFh",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MDd5YnNuaHAiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA1MDYzMDA2fSwiaWF0IjoxNzkxOTA1MDYzLCJleHAiOjE3OTQ0OTcwNjN9._PsfLGj655n4b-b7EKOe9U22kwuKc-RoY_dRBNRn-tA",
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
  "openedAt": 1791912259153
}
```

### 15:24:23.112   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905063104,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "lAZc1eFd1uYQhJIrAAFd",
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
      "connectionId": "CwSUJYh5kZ1f18MsAAFf",
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
      "connectionId": "h6TdahtLyhJni1cdAAFh",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:24:23.112   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905063104,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "lAZc1eFd1uYQhJIrAAFd",
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
      "connectionId": "CwSUJYh5kZ1f18MsAAFf",
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
      "connectionId": "h6TdahtLyhJni1cdAAFh",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:24:23.113   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u07ybsnhp/Editor.bin/Editor.bin?md5=AeTejW9naHCsrtjwRsWL-Q&expires=1794499652&shardkey=recmv6u07ybsnhp&filename=Editor.bin"
    },
    "openedAt": 1791912259153
  }
}
```

### 15:24:23.131   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:6"
}
```

### 15:24:23.181   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:49"
}
```

### 15:24:23.233   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:52"
}
```

### 15:24:23.444   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:24:23.444   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:603 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:24:23.445   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:24:24.515   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:24.515   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905064515,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:24:24.515   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905064515,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:24:24.541   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:24.542   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905064541,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:24:24.542   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905064541,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:24:24.574   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:24.575   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905064574,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:24:24.575   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905064574,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:24:25.554   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:25.555   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905065555,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:24:25.555   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905065555,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:24:25.594   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:25.595   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905065594,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:24:25.595   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905065594,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

## Info

- Timestamp: 2026-10-15T10:00:45.842Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document.
  2. Bob joins: Alice and Bob edit together.
  3. Alice closes her editor: the client sends no `close` event and the connection is closed (code 1001).
  4. Bob receives the connectState without Alice.
  5. Alice re-joins with a new editor.

## Events

### 10:00:37.055       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9dbn7nuzbs/c/?shardkey=recmv9dbn7nuzbs&EIO=4&transport=websocket"
```

### 10:00:37.059   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "rwy6qSltrbHiccrhAACE",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:00:37.062   <-  Alice           license

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

### 10:00:37.183   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv9dbn7nuzbs",
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
    "id": "recmv9dbn7nuzbs",
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
  "time": 423,
  "supportAuthChangesAck": true
}
```

### 10:00:37.201   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "HEUkFjgaIVeXjOklAACF",
  "sessionTimeConnect": 1792058437062,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "HEUkFjgaIVeXjOklAACF",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYm43bnV6YnMiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjA1ODQzNzA2Mn0sImlhdCI6MTc5MjA1ODQzNywiZXhwIjoxNzk0NjUwNDM3fQ.nsLuYB4hTzfxWqeJm8FE4v12TCA5BsNv2yOGhF8-qvQ",
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
  "openedAt": 1792065637191
}
```

### 10:00:37.263   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9dbn7nuzbs/Editor.bin/Editor.bin?md5=ZAsSESP0W2wjteHeqY-nSQ&expires=1794653030&shardkey=recmv9dbn7nuzbs&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9dbn7nuzbs/media/image1.png/image1.png?md5=HMSZRX87uWrU28grl6DEgw&expires=1794653030&shardkey=recmv9dbn7nuzbs&filename=image1.png"
    },
    "openedAt": 1792065637191
  }
}
```

### 10:00:37.334   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:17"
}
```

### 10:00:37.415   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:80"
}
```

### 10:00:37.516   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 10:00:37.723   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 10:00:37.724   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:790 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:00:37.724   <-  Alice           message

```json
{
  "type": "message"
}
```

### 10:00:38.889       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9dbn7nuzbs/c/?shardkey=recmv9dbn7nuzbs&EIO=4&transport=websocket"
```

### 10:00:38.891   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "8OgzcxuVo_rE3pmUAACG",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:00:38.918   <-  Bob             license

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

### 10:00:39.001   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv9dbn7nuzbs",
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
    "id": "recmv9dbn7nuzbs",
    "userid": "de10808c001000000000000000000000",
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
  "time": 266,
  "supportAuthChangesAck": true
}
```

### 10:00:39.010   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "HEUkFjgaIVeXjOklAACF",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 10:00:39.010   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792058439008,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "HEUkFjgaIVeXjOklAACF",
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
      "connectionId": "6T38KH3f0kSGHlGJAACH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 10:00:39.011   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9dbn7nuzbs/Editor.bin/Editor.bin?md5=ZAsSESP0W2wjteHeqY-nSQ&expires=1794653030&shardkey=recmv9dbn7nuzbs&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9dbn7nuzbs/media/image1.png/image1.png?md5=HMSZRX87uWrU28grl6DEgw&expires=1794653030&shardkey=recmv9dbn7nuzbs&filename=image1.png"
    },
    "openedAt": 1792065637191
  }
}
```

### 10:00:39.014   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 10:00:39.021   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "6T38KH3f0kSGHlGJAACH",
  "sessionTimeConnect": 1792058438918,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "HEUkFjgaIVeXjOklAACF",
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
      "connectionId": "6T38KH3f0kSGHlGJAACH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYm43bnV6YnMiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIwNTg0Mzg5MTh9LCJpYXQiOjE3OTIwNTg0MzksImV4cCI6MTc5NDY1MDQzOX0.9rbh4elt6fyVA--C7aAMbxxkw6_RwCPdetyvvStq4vk",
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

### 10:00:39.027   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:3"
}
```

### 10:00:39.043   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 10:00:39.044   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792058439044,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 10:00:39.467   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:37"
}
```

### 10:00:39.573   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:105"
}
```

### 10:00:39.721   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 10:00:39.722   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:866 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:00:39.722   <-  Bob             message

```json
{
  "type": "message"
}
```

### 10:00:39.759   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 10:00:39.760   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792058439760,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 10:00:40.702       Alice           [Transport WebSocket] ws-close

```json
"client close, code 1001"
```

### 10:00:40.703   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792058440703,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "6T38KH3f0kSGHlGJAACH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 10:00:42.357       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9dbn7nuzbs/c/?shardkey=recmv9dbn7nuzbs&EIO=4&transport=websocket"
```

### 10:00:42.359   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "fK1TTh0PAv02AMgFAACI",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:00:42.389   <-  Alice           license

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

### 10:00:42.470   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv9dbn7nuzbs",
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
    "id": "recmv9dbn7nuzbs",
    "userid": "de10a11cec0010000000000000000000",
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
  "time": 255,
  "supportAuthChangesAck": true
}
```

### 10:00:42.475   <-  Alice           waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10808c0010000000000000000000002",
    "idOriginal": "de10808c001000000000000000000000",
    "username": "Bob",
    "indexUser": 2,
    "view": false,
    "connectionId": "6T38KH3f0kSGHlGJAACH",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 10:00:42.476   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792058442474,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "6T38KH3f0kSGHlGJAACH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000003",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 3,
      "view": false,
      "connectionId": "kBtg-rjw3Q2E0p72AACJ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 10:00:42.478   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9dbn7nuzbs/Editor.bin/Editor.bin?md5=ZAsSESP0W2wjteHeqY-nSQ&expires=1794653030&shardkey=recmv9dbn7nuzbs&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9dbn7nuzbs/media/image1.png/image1.png?md5=HMSZRX87uWrU28grl6DEgw&expires=1794653030&shardkey=recmv9dbn7nuzbs&filename=image1.png"
    },
    "openedAt": 1792065637191
  }
}
```

### 10:00:42.480   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 10:00:42.486   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "kBtg-rjw3Q2E0p72AACJ",
  "sessionTimeConnect": 1792058442389,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "6T38KH3f0kSGHlGJAACH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000003",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 3,
      "view": false,
      "connectionId": "kBtg-rjw3Q2E0p72AACJ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 3,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkYm43bnV6YnMiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6M30sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjA1ODQ0MjM4OX0sImlhdCI6MTc5MjA1ODQ0MiwiZXhwIjoxNzk0NjUwNDQyfQ._jLJJsH8Ec0W_6MGqKlzMAS2r9TEURTyRCh9N_lg6Bk",
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

### 10:00:42.492   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 10:00:42.535   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:43"
}
```

### 10:00:42.586   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:52"
}
```

### 10:00:42.730   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 10:00:42.730   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:401 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:00:42.730   <-  Alice           message

```json
{
  "type": "message"
}
```

### 10:00:42.764   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 10:00:42.765   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792058442765,
      "user": "de10a11cec00100000000000000000003",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

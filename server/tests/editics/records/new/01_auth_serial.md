## Info

- Timestamp: 2026-10-13T15:22:41.043Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario: Alice opens the document, then Bob joins, then Mallory.

## Events

### 15:22:33.662       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6txyoj9sjg/c/?shardkey=recmv6txyoj9sjg&EIO=4&transport=websocket"
```

### 15:22:33.664   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "JSzKfYfGjMPvjQC2AAEq",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:22:33.669   <-  Alice           license

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

### 15:22:33.785   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6txyoj9sjg",
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
    "id": "recmv6txyoj9sjg",
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
  "time": 401,
  "supportAuthChangesAck": true
}
```

### 15:22:33.801   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "i2S1YqUO9G5mwN72AAEr",
  "sessionTimeConnect": 1791904953669,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "i2S1YqUO9G5mwN72AAEr",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0eHlvajlzamciLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNDk1MzY2OX0sImlhdCI6MTc5MTkwNDk1MywiZXhwIjoxNzk0NDk2OTUzfQ.aAQKxMVQ-utJFD33458XVgVoUV0GrcQmf6LAgCqvisE",
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
  "openedAt": 1791912153792
}
```

### 15:22:33.866   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6txyoj9sjg/Editor.bin/Editor.bin?md5=b8pQ081HSBd8rIjaW-jPIg&expires=1794499546&shardkey=recmv6txyoj9sjg&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6txyoj9sjg/media/image1.png/image1.png?md5=N9PLUNFKXh3Jc171tJR9BQ&expires=1794499546&shardkey=recmv6txyoj9sjg&filename=image1.png"
    },
    "openedAt": 1791912153792
  }
}
```

### 15:22:33.937   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:15"
}
```

### 15:22:34.016   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:78"
}
```

### 15:22:34.116   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 15:22:34.324   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:22:34.324   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:766 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:22:34.324   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:22:35.491       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6txyoj9sjg/c/?shardkey=recmv6txyoj9sjg&EIO=4&transport=websocket"
```

### 15:22:35.493   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "ZC4H2ba-3WVtp6OqAAEs",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:22:35.520   <-  Bob             license

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

### 15:22:35.598   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6txyoj9sjg",
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
    "id": "recmv6txyoj9sjg",
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
  "time": 271,
  "supportAuthChangesAck": true
}
```

### 15:22:35.607   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "i2S1YqUO9G5mwN72AAEr",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:22:35.607   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791904955605,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "i2S1YqUO9G5mwN72AAEr",
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
      "connectionId": "dZSr93hCuPE4Tf7YAAEt",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:22:35.608   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6txyoj9sjg/Editor.bin/Editor.bin?md5=b8pQ081HSBd8rIjaW-jPIg&expires=1794499546&shardkey=recmv6txyoj9sjg&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6txyoj9sjg/media/image1.png/image1.png?md5=N9PLUNFKXh3Jc171tJR9BQ&expires=1794499546&shardkey=recmv6txyoj9sjg&filename=image1.png"
    },
    "openedAt": 1791912153792
  }
}
```

### 15:22:35.611   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:22:35.618   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "dZSr93hCuPE4Tf7YAAEt",
  "sessionTimeConnect": 1791904955520,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "i2S1YqUO9G5mwN72AAEr",
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
      "connectionId": "dZSr93hCuPE4Tf7YAAEt",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0eHlvajlzamciLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDQ5NTU1MjB9LCJpYXQiOjE3OTE5MDQ5NTUsImV4cCI6MTc5NDQ5Njk1NX0.vpQQXSN1YePqSwla4dDSm9nDhLJf7nCN36cKVcyTH9k",
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

### 15:22:35.626   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:8"
}
```

### 15:22:35.641   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:22:35.642   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791904955642,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:22:36.078   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:38"
}
```

### 15:22:36.183   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:106"
}
```

### 15:22:36.339   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:22:36.339   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:888 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:22:36.340   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:22:36.377   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:22:36.378   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791904956378,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:22:37.524       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6txyoj9sjg/c/?shardkey=recmv6txyoj9sjg&EIO=4&transport=websocket"
```

### 15:22:37.526   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "Dtmrvr_GF6MOklkzAAEu",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:22:37.557   <-  Mallory         license

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

### 15:22:37.634   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6txyoj9sjg",
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
    "id": "recmv6txyoj9sjg",
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
  "time": 268,
  "supportAuthChangesAck": true
}
```

### 15:22:37.648   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "tH6CFzTDDSP9r0a7AAEv",
  "sessionTimeConnect": 1791904957557,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "i2S1YqUO9G5mwN72AAEr",
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
      "connectionId": "dZSr93hCuPE4Tf7YAAEt",
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
      "connectionId": "tH6CFzTDDSP9r0a7AAEv",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0eHlvajlzamciLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA0OTU3NTU3fSwiaWF0IjoxNzkxOTA0OTU3LCJleHAiOjE3OTQ0OTY5NTd9.DGlh8ufimoxvE6zElkEHtFs1GU0DsQKyHnbApPBc-RA",
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
  "openedAt": 1791912153792
}
```

### 15:22:37.649   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791904957641,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "i2S1YqUO9G5mwN72AAEr",
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
      "connectionId": "dZSr93hCuPE4Tf7YAAEt",
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
      "connectionId": "tH6CFzTDDSP9r0a7AAEv",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:22:37.649   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791904957641,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "i2S1YqUO9G5mwN72AAEr",
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
      "connectionId": "dZSr93hCuPE4Tf7YAAEt",
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
      "connectionId": "tH6CFzTDDSP9r0a7AAEv",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:22:37.649   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6txyoj9sjg/Editor.bin/Editor.bin?md5=b8pQ081HSBd8rIjaW-jPIg&expires=1794499546&shardkey=recmv6txyoj9sjg&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6txyoj9sjg/media/image1.png/image1.png?md5=N9PLUNFKXh3Jc171tJR9BQ&expires=1794499546&shardkey=recmv6txyoj9sjg&filename=image1.png"
    },
    "openedAt": 1791912153792
  }
}
```

### 15:22:37.657   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:1"
}
```

### 15:22:37.703   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:46"
}
```

### 15:22:37.806   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:103"
}
```

### 15:22:37.957   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:22:37.957   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:471 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:22:37.958   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:22:37.993   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:22:37.994   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791904957993,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:22:37.994   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791904957993,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

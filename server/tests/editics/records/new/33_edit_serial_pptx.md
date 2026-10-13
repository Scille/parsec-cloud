## Info

- Timestamp: 2026-10-13T15:28:05.622Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: presentation.pptx (slide)
- Scenario: Alice, Bob and Mallory edit the document one after the other.

## Events

### 15:27:51.850       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6u4qxuzgti/c/?shardkey=recmv6u4qxuzgti&EIO=4&transport=websocket"
```

### 15:27:51.853   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "VA1-j0LRk82XHqVJAAGe",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:27:51.869   <-  Alice           license

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

### 15:27:52.088   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6u4qxuzgti",
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
    "id": "recmv6u4qxuzgti",
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
  "time": 535,
  "supportAuthChangesAck": true
}
```

### 15:27:52.103   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "n00duC42D_3CgsaAAAGf",
  "sessionTimeConnect": 1791905271869,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "n00duC42D_3CgsaAAAGf",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1NHF4dXpndGkiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTI3MTg2OX0sImlhdCI6MTc5MTkwNTI3MiwiZXhwIjoxNzk0NDk3MjcyfQ.QDMzYnifqS6R9J_k8XDB3Z_pHha5pBDepblIEMR-TbY",
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
  "openedAt": 1791912472095
}
```

### 15:27:52.198   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u4qxuzgti/Editor.bin/Editor.bin?md5=yQmP_simYwhIYdkSJt8X0g&expires=1794499865&shardkey=recmv6u4qxuzgti&filename=Editor.bin"
    },
    "openedAt": 1791912472095
  }
}
```

### 15:27:52.211   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:12"
}
```

### 15:27:52.306   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:94"
}
```

### 15:27:52.408   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:102"
}
```

### 15:27:52.694   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:27:52.695   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1130 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":91700000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:27:52.695   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:27:53.901       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6u4qxuzgti/c/?shardkey=recmv6u4qxuzgti&EIO=4&transport=websocket"
```

### 15:27:53.903   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "YJUCoDJy_cS7qMYOAAGg",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:27:53.939   <-  Bob             license

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

### 15:27:54.106   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6u4qxuzgti",
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
    "id": "recmv6u4qxuzgti",
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
  "time": 370,
  "supportAuthChangesAck": true
}
```

### 15:27:54.114   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "n00duC42D_3CgsaAAAGf",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:27:54.115   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905274113,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "n00duC42D_3CgsaAAAGf",
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
      "connectionId": "lTF_ZvueW5EF5XBqAAGh",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:27:54.115   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u4qxuzgti/Editor.bin/Editor.bin?md5=yQmP_simYwhIYdkSJt8X0g&expires=1794499865&shardkey=recmv6u4qxuzgti&filename=Editor.bin"
    },
    "openedAt": 1791912472095
  }
}
```

### 15:27:54.123   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:27:54.129   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "lTF_ZvueW5EF5XBqAAGh",
  "sessionTimeConnect": 1791905273938,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "n00duC42D_3CgsaAAAGf",
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
      "connectionId": "lTF_ZvueW5EF5XBqAAGh",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1NHF4dXpndGkiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUyNzM5Mzh9LCJpYXQiOjE3OTE5MDUyNzQsImV4cCI6MTc5NDQ5NzI3NH0.RUBKjtC3W5QQWtCjIXfr2bDG73r-fPcsKY_Q6aI5JJk",
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

### 15:27:54.522   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:24"
}
```

### 15:27:54.579   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:57"
}
```

### 15:27:54.682   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:103"
}
```

### 15:27:54.888   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:27:54.888   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1142 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":91700000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:27:54.888   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:27:56.066       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6u4qxuzgti/c/?shardkey=recmv6u4qxuzgti&EIO=4&transport=websocket"
```

### 15:27:56.067   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "S8L74vDDjxPoVhOBAAGi",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:27:56.087   <-  Mallory         license

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

### 15:27:56.280   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6u4qxuzgti",
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
    "id": "recmv6u4qxuzgti",
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
  "time": 354,
  "supportAuthChangesAck": true
}
```

### 15:27:56.292   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "p6ya3C6do3ErmgW7AAGj",
  "sessionTimeConnect": 1791905276087,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "n00duC42D_3CgsaAAAGf",
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
      "connectionId": "lTF_ZvueW5EF5XBqAAGh",
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
      "connectionId": "p6ya3C6do3ErmgW7AAGj",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1NHF4dXpndGkiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA1Mjc2MDg3fSwiaWF0IjoxNzkxOTA1Mjc2LCJleHAiOjE3OTQ0OTcyNzZ9.ECAhM9mhqa6AMB4XY-Y70sPO1ZRwDFgYkiclGvdGayg",
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
  "openedAt": 1791912472095
}
```

### 15:27:56.293   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905276287,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "n00duC42D_3CgsaAAAGf",
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
      "connectionId": "lTF_ZvueW5EF5XBqAAGh",
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
      "connectionId": "p6ya3C6do3ErmgW7AAGj",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:27:56.293   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905276287,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "n00duC42D_3CgsaAAAGf",
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
      "connectionId": "lTF_ZvueW5EF5XBqAAGh",
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
      "connectionId": "p6ya3C6do3ErmgW7AAGj",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:27:56.293   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u4qxuzgti/Editor.bin/Editor.bin?md5=yQmP_simYwhIYdkSJt8X0g&expires=1794499865&shardkey=recmv6u4qxuzgti&filename=Editor.bin"
    },
    "openedAt": 1791912472095
  }
}
```

### 15:27:56.317   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:12"
}
```

### 15:27:56.371   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:53"
}
```

### 15:27:56.483   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:112"
}
```

### 15:27:56.697   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:27:56.698   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:762 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":91700000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:27:56.698   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:27:57.779   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:27:57.780   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905277780,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:27:57.780   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905277780,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:27:58.239   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:27:58.240   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:27:58.241   ->  Alice           saveChanges

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

### 15:27:58.246   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"",
      "time": 1791905278000,
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

### 15:27:58.246   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"",
      "time": 1791905278000,
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

### 15:27:58.246   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791905278000,
  "syncChangesIndex": 3
}
```

### 15:27:58.279   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:27:58.295   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:27:58.314   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 3
}
```

### 15:27:58.315   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:27:58.317   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgAAgAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:27:58.321   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 5,
  "syncChangesIndex": 5,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgAAgAAAA==\"}"
}
```

### 15:27:58.322   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 5,
  "syncChangesIndex": 5,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgAAgAAAA==\"}"
}
```

### 15:27:58.322   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 3,
  "time": 1791905278000,
  "syncChangesIndex": 5
}
```

### 15:27:58.405   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 5
}
```

### 15:27:58.405   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:27:58.407   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgABAAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:27:58.411   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 9,
  "syncChangesIndex": 9,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgABAAAAA==\"}"
}
```

### 15:27:58.411   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 9,
  "syncChangesIndex": 9,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgABAAAAA==\"}"
}
```

### 15:27:58.411   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 5,
  "time": 1791905278000,
  "syncChangesIndex": 9
}
```

### 15:27:58.434   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 9
}
```

### 15:27:58.435   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:27:58.436   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"]",
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

### 15:27:58.440   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791905278000,
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

### 15:27:58.440   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791905278000,
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

### 15:27:58.440   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 9,
  "time": 1791905278000,
  "syncChangesIndex": 11
}
```

### 15:27:58.485   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 11
}
```

### 15:27:58.486   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:27:58.489   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgABgAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:27:58.492   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 13,
  "syncChangesIndex": 13,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgABgAAAA==\"}"
}
```

### 15:27:58.493   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 13,
  "syncChangesIndex": 13,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgABgAAAA==\"}"
}
```

### 15:27:58.493   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 11,
  "time": 1791905278000,
  "syncChangesIndex": 13
}
```

### 15:27:58.558   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 13
}
```

### 15:27:58.558   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:27:58.577   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgACAAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:27:58.583   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 17,
  "syncChangesIndex": 17,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgACAAAAA==\"}"
}
```

### 15:27:58.583   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 17,
  "syncChangesIndex": 17,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADgACAAAAA==\"}"
}
```

### 15:27:58.583   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 13,
  "time": 1791905278000,
  "syncChangesIndex": 17
}
```

### 15:27:58.621   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 17
}
```

### 15:27:58.621   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:27:58.629   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:27:58.630   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"]",
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

### 15:27:58.634   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791905278000,
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

### 15:27:58.635   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905278000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791905278000,
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

### 15:27:58.635   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 17,
  "time": 1791905278000,
  "syncChangesIndex": 19
}
```

### 15:27:58.661   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:27:58.730   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:27:58.791   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADgACQAAAA=="
}
```

### 15:27:58.791   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgACQAAAA==",
      "time": 1791905278791,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:27:58.791   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgACQAAAA==",
      "time": 1791905278791,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:27:59.436   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADgAAQAAAA=="
}
```

### 15:27:59.436   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgAAQAAAA==",
      "time": 1791905279436,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:27:59.437   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgAAQAAAA==",
      "time": 1791905279436,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:27:59.916   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 19
}
```

### 15:27:59.916   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:27:59.917   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAQAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAgAcAAEAAAAAAAAAAgAAACAAAAAAAAAA\",\"40;CAAAADIAMgAyADMAAgADAAEAAAABAAAACAAAADIAMgAyADcAAAAAAA==\",\"102;CAAAADIAMgAyADYAAgAcAAYAAAAAAAAAAQAAAFMAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAbQAAAAAAAAAAAQAAAHAAAAAAAAAAAAEAAABsAAAAAAAAAAABAAAAZQAAAAAAAAAA\",\"36;CAAAADIAMgAyADYAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAQAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:27:59.924   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAQAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"36;CAAAADIAMgAyADgAAgAcAAEAAAAAAAAAAgAAACAAAAAAAAAA\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"40;CAAAADIAMgAyADMAAgADAAEAAAABAAAACAAAADIAMgAyADcAAAAAAA==\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"102;CAAAADIAMgAyADYAAgAcAAYAAAAAAAAAAQAAAFMAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAbQAAAAAAAAAAAQAAAHAAAAAAAAAAAAEAAABsAAAAAAAAAAABAAAAZQAAAAAAAAAA\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"36;CAAAADIAMgAyADYAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 24,
  "syncChangesIndex": 24,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAQAAAA==\"}"
}
```

### 15:27:59.925   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 19,
  "time": 1791905279000,
  "syncChangesIndex": 24
}
```

### 15:27:59.925   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAQAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"36;CAAAADIAMgAyADgAAgAcAAEAAAAAAAAAAgAAACAAAAAAAAAA\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"40;CAAAADIAMgAyADMAAgADAAEAAAABAAAACAAAADIAMgAyADcAAAAAAA==\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"102;CAAAADIAMgAyADYAAgAcAAYAAAAAAAAAAQAAAFMAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAbQAAAAAAAAAAAQAAAHAAAAAAAAAAAAEAAABsAAAAAAAAAAABAAAAZQAAAAAAAAAA\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"36;CAAAADIAMgAyADYAAQAcAAEAAAAAAAAAAgAAACAAAAADAAAA\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 24,
  "syncChangesIndex": 24,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAQAAAA==\"}"
}
```

### 15:27:59.953   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:27:59.958   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 24
}
```

### 15:27:59.959   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:27:59.960   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAgAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:27:59.963   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 26,
  "syncChangesIndex": 26,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAgAAAA==\"}"
}
```

### 15:27:59.964   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAABAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 26,
  "syncChangesIndex": 26,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAgAAAA==\"}"
}
```

### 15:27:59.964   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 24,
  "time": 1791905279000,
  "syncChangesIndex": 26
}
```

### 15:27:59.991   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:27:59.996   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 26
}
```

### 15:27:59.996   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:27:59.997   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAwAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:00.001   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 28,
  "syncChangesIndex": 28,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAwAAAA==\"}"
}
```

### 15:28:00.002   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAACAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905279000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 28,
  "syncChangesIndex": 28,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAwAAAA==\"}"
}
```

### 15:28:00.002   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 26,
  "time": 1791905279000,
  "syncChangesIndex": 28
}
```

### 15:28:00.015   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.041   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 28
}
```

### 15:28:00.041   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:00.043   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABAAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:00.046   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 30,
  "syncChangesIndex": 30,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABAAAAA==\"}"
}
```

### 15:28:00.046   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAADAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 30,
  "syncChangesIndex": 30,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABAAAAA==\"}"
}
```

### 15:28:00.047   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 28,
  "time": 1791905280000,
  "syncChangesIndex": 30
}
```

### 15:28:00.086   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 30
}
```

### 15:28:00.087   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:00.089   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.091   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABQAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:00.094   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 32,
  "syncChangesIndex": 32,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABQAAAA==\"}"
}
```

### 15:28:00.094   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAEAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 32,
  "syncChangesIndex": 32,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABQAAAA==\"}"
}
```

### 15:28:00.094   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 30,
  "time": 1791905280000,
  "syncChangesIndex": 32
}
```

### 15:28:00.136   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 32
}
```

### 15:28:00.136   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:00.139   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.150   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABgAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:00.154   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 34,
  "syncChangesIndex": 34,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABgAAAA==\"}"
}
```

### 15:28:00.154   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAFAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 34,
  "syncChangesIndex": 34,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABgAAAA==\"}"
}
```

### 15:28:00.154   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 32,
  "time": 1791905280000,
  "syncChangesIndex": 34
}
```

### 15:28:00.185   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 34
}
```

### 15:28:00.185   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:00.190   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.191   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABwAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:00.195   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 36,
  "syncChangesIndex": 36,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABwAAAA==\"}"
}
```

### 15:28:00.195   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 34,
  "time": 1791905280000,
  "syncChangesIndex": 36
}
```

### 15:28:00.195   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAGAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 36,
  "syncChangesIndex": 36,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABwAAAA==\"}"
}
```

### 15:28:00.199   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.200   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADgACAAAAA=="
}
```

### 15:28:00.200   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgACAAAAA==",
      "time": 1791905280200,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:28:00.200   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgACAAAAA==",
      "time": 1791905280200,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:28:00.256   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.259   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.260   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 36
}
```

### 15:28:00.260   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:00.261   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACAAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:00.265   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 38,
  "syncChangesIndex": 38,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACAAAAA==\"}"
}
```

### 15:28:00.265   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAHAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 38,
  "syncChangesIndex": 38,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACAAAAA==\"}"
}
```

### 15:28:00.265   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 36,
  "time": 1791905280000,
  "syncChangesIndex": 38
}
```

### 15:28:00.289   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.290   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 38
}
```

### 15:28:00.290   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:00.293   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.293   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACQAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:00.296   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 40,
  "syncChangesIndex": 40,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACQAAAA==\"}"
}
```

### 15:28:00.296   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAIAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791905280000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 40,
  "syncChangesIndex": 40,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACQAAAA==\"}"
}
```

### 15:28:00.297   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 38,
  "time": 1791905280000,
  "syncChangesIndex": 40
}
```

### 15:28:00.317   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.338   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:00.476   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADYACQAAAA=="
}
```

### 15:28:00.476   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADYACQAAAA==",
      "time": 1791905280476,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:28:00.476   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADYACQAAAA==",
      "time": 1791905280476,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:28:01.248   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADgACAAAAA=="
}
```

### 15:28:01.248   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADYAAgAAAA=="
}
```

### 15:28:01.248   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgACAAAAA==",
      "time": 1791905281248,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:28:01.248   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgACAAAAA==",
      "time": 1791905281248,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:28:01.249   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADYAAgAAAA==",
      "time": 1791905281248,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:28:01.249   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADYAAgAAAA==",
      "time": 1791905281248,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:28:01.726   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 40
}
```

### 15:28:01.727   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:01.728   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAgAcAAEAAAABAAAAAQAAACgAAAAAAAAAAA==\",\"36;CAAAADIAMgAyADYAAQAcAAEAAAABAAAAAgAAACAAAAADAAAA\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAgAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:01.735   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAgAcAAEAAAABAAAAAQAAACgAAAAAAAAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"36;CAAAADIAMgAyADYAAQAcAAEAAAABAAAAAgAAACAAAAADAAAA\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 43,
  "syncChangesIndex": 43,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAgAAAA==\"}"
}
```

### 15:28:01.735   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAgAcAAEAAAABAAAAAQAAACgAAAAAAAAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"36;CAAAADIAMgAyADYAAQAcAAEAAAABAAAAAgAAACAAAAADAAAA\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 43,
  "syncChangesIndex": 43,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYAAgAAAA==\"}"
}
```

### 15:28:01.735   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 40,
  "time": 1791905281000,
  "syncChangesIndex": 43
}
```

### 15:28:01.768   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:01.772   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:01.773   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADYACQAAAA=="
}
```

### 15:28:01.773   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADYACQAAAA==",
      "time": 1791905281773,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:28:01.773   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADYACQAAAA==",
      "time": 1791905281773,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:28:01.786   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 43
}
```

### 15:28:01.786   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:01.811   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAACAAAAAQAAACgAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAADAAAAAQAAAGUAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABAAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:01.815   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAACAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAADAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 47,
  "syncChangesIndex": 47,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABAAAAA==\"}"
}
```

### 15:28:01.815   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAACAAAAAQAAACgAAAAAAwAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAADAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 47,
  "syncChangesIndex": 47,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABAAAAA==\"}"
}
```

### 15:28:01.815   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 43,
  "time": 1791905281000,
  "syncChangesIndex": 47
}
```

### 15:28:01.834   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:01.859   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:01.860   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 47
}
```

### 15:28:01.860   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:01.860   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAEAAAAAQAAAGQAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABQAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:01.864   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAEAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 49,
  "syncChangesIndex": 49,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABQAAAA==\"}"
}
```

### 15:28:01.864   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAEAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 49,
  "syncChangesIndex": 49,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABQAAAA==\"}"
}
```

### 15:28:01.864   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 47,
  "time": 1791905281000,
  "syncChangesIndex": 49
}
```

### 15:28:01.881   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:01.885   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:01.919   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 49
}
```

### 15:28:01.919   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:01.920   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAFAAAAAQAAAGkAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABgAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:01.923   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAFAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 51,
  "syncChangesIndex": 51,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABgAAAA==\"}"
}
```

### 15:28:01.924   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAFAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 51,
  "syncChangesIndex": 51,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABgAAAA==\"}"
}
```

### 15:28:01.924   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 49,
  "time": 1791905281000,
  "syncChangesIndex": 51
}
```

### 15:28:01.963   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:01.967   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:01.978   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 51
}
```

### 15:28:01.979   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:01.979   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAGAAAAAQAAAHQAAAAAAwAAAA==\",\"136;CAAAADIAMgAxADcAAQBWBAAAAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAEYcwEAAegDAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAQIAAAAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABwAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:01.983   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAGAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"136;CAAAADIAMgAxADcAAQBWBAAAAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAEYcwEAAegDAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAQIAAAAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 54,
  "syncChangesIndex": 54,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABwAAAA==\"}"
}
```

### 15:28:01.983   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAGAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"136;CAAAADIAMgAxADcAAQBWBAAAAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAEYcwEAAegDAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAQIAAAAAAA==\"",
      "time": 1791905281000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 54,
  "syncChangesIndex": 54,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYABwAAAA==\"}"
}
```

### 15:28:01.983   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 51,
  "time": 1791905281000,
  "syncChangesIndex": 54
}
```

### 15:28:02.020   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 54
}
```

### 15:28:02.021   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:02.021   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAHAAAAAQAAAGUAAAAAAwAAAA==\",\"194;CAAAADIAMgAxADcAAQBWBAAAAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAGQXwEAAdAHAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAEYcwEAAegDAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACAAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:02.025   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAHAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"194;CAAAADIAMgAxADcAAQBWBAAAAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAGQXwEAAdAHAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAEYcwEAAegDAAA=\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 57,
  "syncChangesIndex": 57,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACAAAAA==\"}"
}
```

### 15:28:02.025   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAHAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"194;CAAAADIAMgAxADcAAQBWBAAAAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAGQXwEAAdAHAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAEYcwEAAegDAAA=\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 57,
  "syncChangesIndex": 57,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACAAAAA==\"}"
}
```

### 15:28:02.025   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 54,
  "time": 1791905282000,
  "syncChangesIndex": 57
}
```

### 15:28:02.036   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:02.078   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:02.078   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 57
}
```

### 15:28:02.078   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:02.079   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAIAAAAAQAAAGQAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACQAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:02.083   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAIAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 59,
  "syncChangesIndex": 59,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACQAAAA==\"}"
}
```

### 15:28:02.083   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAIAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 59,
  "syncChangesIndex": 59,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACQAAAA==\"}"
}
```

### 15:28:02.083   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 57,
  "time": 1791905282000,
  "syncChangesIndex": 59
}
```

### 15:28:02.118   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 59
}
```

### 15:28:02.118   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:28:02.123   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:02.126   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:02.127   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADYAAQAcAAEAAAAJAAAAAQAAACkAAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACgAAAA==\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:28:02.130   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAJAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 61,
  "syncChangesIndex": 61,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACgAAAA==\"}"
}
```

### 15:28:02.130   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u4qxuzgti",
      "change": "\"37;CAAAADIAMgAyADYAAQAcAAEAAAAJAAAAAQAAACkAAAAAAwAAAA==\"",
      "time": 1791905282000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 61,
  "syncChangesIndex": 61,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"16;CAAAADIAMgAyADYACgAAAA==\"}"
}
```

### 15:28:02.131   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 59,
  "time": 1791905282000,
  "syncChangesIndex": 61
}
```

### 15:28:02.156   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:02.171   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:28:02.270   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADgACAAAAA=="
}
```

### 15:28:02.271   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgACAAAAA==",
      "time": 1791905282271,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:28:02.271   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADgACAAAAA==",
      "time": 1791905282271,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:28:02.295   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADYACgAAAA=="
}
```

### 15:28:02.296   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADYACgAAAA==",
      "time": 1791905282296,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:28:02.296   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADYACgAAAA==",
      "time": 1791905282296,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:28:02.796   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADYAEQAAAA=="
}
```

### 15:28:02.796   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADYAEQAAAA==",
      "time": 1791905282796,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:28:02.797   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADYAEQAAAA==",
      "time": 1791905282796,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

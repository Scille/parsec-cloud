## Info

- Timestamp: 2026-10-13T15:24:16.858Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: presentation.pptx (slide)
- Scenario: Alice, Bob and Mallory move their cursors one after the other (serial).

## Events

### 15:24:05.313       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6tzxej0ble/c/?shardkey=recmv6tzxej0ble&EIO=4&transport=websocket"
```

### 15:24:05.314   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "Pdg8tpc1uMab8lDSAAFW",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:24:05.322   <-  Alice           license

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

### 15:24:05.459   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6tzxej0ble",
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
    "id": "recmv6tzxej0ble",
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
  "time": 418,
  "supportAuthChangesAck": true
}
```

### 15:24:05.475   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "Iujo1O0_SPUGbXiyAAFX",
  "sessionTimeConnect": 1791905045321,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "Iujo1O0_SPUGbXiyAAFX",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0enhlajBibGUiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTA0NTMyMX0sImlhdCI6MTc5MTkwNTA0NSwiZXhwIjoxNzk0NDk3MDQ1fQ.QlN6LYd4kmpff3U6ThxcevwxsrP_ofU-E1W0WxtBhSE",
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
  "openedAt": 1791912245465
}
```

### 15:24:05.573   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6tzxej0ble/Editor.bin/Editor.bin?md5=wfx0llZ_2uLMQ74pxContA&expires=1794499638&shardkey=recmv6tzxej0ble&filename=Editor.bin"
    },
    "openedAt": 1791912245465
  }
}
```

### 15:24:05.597   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:11"
}
```

### 15:24:05.688   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:91"
}
```

### 15:24:05.789   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 15:24:06.086   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:24:06.086   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1033 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":97400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:24:06.086   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:24:07.636       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6tzxej0ble/c/?shardkey=recmv6tzxej0ble&EIO=4&transport=websocket"
```

### 15:24:07.639   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "r1HQKMxMBQL3c6u-AAFY",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:24:07.667   <-  Bob             license

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

### 15:24:07.781   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6tzxej0ble",
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
    "id": "recmv6tzxej0ble",
    "userid": "de10808c001000000000000000000000",
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
  "time": 662,
  "supportAuthChangesAck": true
}
```

### 15:24:07.789   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "Iujo1O0_SPUGbXiyAAFX",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:24:07.789   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905047788,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "Iujo1O0_SPUGbXiyAAFX",
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
      "connectionId": "jy-kwswn3jpFHFP8AAFZ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:24:07.790   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6tzxej0ble/Editor.bin/Editor.bin?md5=wfx0llZ_2uLMQ74pxContA&expires=1794499638&shardkey=recmv6tzxej0ble&filename=Editor.bin"
    },
    "openedAt": 1791912245465
  }
}
```

### 15:24:07.792   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:24:07.798   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "jy-kwswn3jpFHFP8AAFZ",
  "sessionTimeConnect": 1791905047667,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "Iujo1O0_SPUGbXiyAAFX",
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
      "connectionId": "jy-kwswn3jpFHFP8AAFZ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0enhlajBibGUiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUwNDc2Njd9LCJpYXQiOjE3OTE5MDUwNDcsImV4cCI6MTc5NDQ5NzA0N30.J8DOvNLKx1YWQRg-7HjMqmhmNNXQ49FG_LOfA2Nkqx4",
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

### 15:24:07.803   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 15:24:07.852   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:49"
}
```

### 15:24:07.903   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 15:24:08.125   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:24:08.125   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:997 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":97400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:24:08.125   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:24:09.254       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/presentationeditor/main/../../../../doc/recmv6tzxej0ble/c/?shardkey=recmv6tzxej0ble&EIO=4&transport=websocket"
```

### 15:24:09.256   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "3anxGGxSujBvgYnXAAFa",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:24:09.289   <-  Mallory         license

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

### 15:24:09.383   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6tzxej0ble",
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
    "id": "recmv6tzxej0ble",
    "userid": "de103a11031c00100000000000000000",
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
  "time": 227,
  "supportAuthChangesAck": true
}
```

### 15:24:09.396   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "eEE3bu1uNZ1ODF56AAFb",
  "sessionTimeConnect": 1791905049289,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "Iujo1O0_SPUGbXiyAAFX",
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
      "connectionId": "jy-kwswn3jpFHFP8AAFZ",
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
      "connectionId": "eEE3bu1uNZ1ODF56AAFb",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0enhlajBibGUiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA1MDQ5Mjg5fSwiaWF0IjoxNzkxOTA1MDQ5LCJleHAiOjE3OTQ0OTcwNDl9.-McR-pGgNUvmnoz0dP7TrkUo0YKiXOfknCjiLZlpikU",
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
  "openedAt": 1791912245465
}
```

### 15:24:09.396   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905049390,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "Iujo1O0_SPUGbXiyAAFX",
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
      "connectionId": "jy-kwswn3jpFHFP8AAFZ",
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
      "connectionId": "eEE3bu1uNZ1ODF56AAFb",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:24:09.396   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905049390,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "Iujo1O0_SPUGbXiyAAFX",
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
      "connectionId": "jy-kwswn3jpFHFP8AAFZ",
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
      "connectionId": "eEE3bu1uNZ1ODF56AAFb",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:24:09.396   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6tzxej0ble/Editor.bin/Editor.bin?md5=wfx0llZ_2uLMQ74pxContA&expires=1794499638&shardkey=recmv6tzxej0ble&filename=Editor.bin"
    },
    "openedAt": 1791912245465
  }
}
```

### 15:24:09.525   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:115"
}
```

### 15:24:09.576   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:51"
}
```

### 15:24:09.627   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 15:24:09.837   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:24:09.837   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:671 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":97400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:24:09.837   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:24:10.906   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:10.907   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905050907,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:24:10.907   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905050907,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:24:11.474   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:11.475   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905051475,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:24:11.475   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905051475,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:24:12.065   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:12.066   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905052065,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:24:12.066   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905052065,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:24:12.514   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:12.515   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905052515,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:24:12.515   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905052515,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:24:13.101   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:13.102   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905053101,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:24:13.102   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905053101,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:24:13.674   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:13.675   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905053675,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:24:13.675   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905053675,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:24:14.203   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "16;CAAAADIAMgAyADUAAAAAAA=="
}
```

### 15:24:14.203   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905054203,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:24:14.203   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "16;CAAAADIAMgAyADUAAAAAAA==",
      "time": 1791905054203,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

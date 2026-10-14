## Info

- Timestamp: 2026-10-13T15:23:51.479Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario: Alice, Bob and Mallory move their cursors at the same time (concurrent).

## Events

### 15:23:42.159       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmv6tzfijtc2c/c/?shardkey=recmv6tzfijtc2c&EIO=4&transport=websocket"
```

### 15:23:42.161   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "yoPDpdUjLj2GjgY4AAFM",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:23:42.174   <-  Alice           license

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

### 15:23:42.343   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6tzfijtc2c",
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
    "id": "recmv6tzfijtc2c",
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
  "time": 485,
  "supportAuthChangesAck": true
}
```

### 15:23:42.357   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "ZkIIDi_KQm6SC5NrAAFN",
  "sessionTimeConnect": 1791905022174,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ZkIIDi_KQm6SC5NrAAFN",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0emZpanRjMmMiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTAyMjE3NH0sImlhdCI6MTc5MTkwNTAyMiwiZXhwIjoxNzk0NDk3MDIyfQ.bka5ZUh9bu3eeXckiRTpLDN2sgXdsp5gmKssVt7TITc",
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
  "openedAt": 1791912222348
}
```

### 15:23:42.410   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6tzfijtc2c/Editor.bin/Editor.bin?md5=5Z8Q5zgVOhORcyiCQHhdig&expires=1794499615&shardkey=recmv6tzfijtc2c&filename=Editor.bin"
    },
    "openedAt": 1791912222348
  }
}
```

### 15:23:42.461   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:9"
}
```

### 15:23:42.513   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:52"
}
```

### 15:23:42.563   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:50"
}
```

### 15:23:42.693   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:23:42.693   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:831 memory:{\"totalJSHeapSize\":81400000,\"usedJSHeapSize\":68000000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:23:42.693   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 15:23:42.693   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:23:42.735   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:23:43.763   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:23:43.864       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmv6tzfijtc2c/c/?shardkey=recmv6tzfijtc2c&EIO=4&transport=websocket"
```

### 15:23:43.866   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "k-lhew1DpkF5jglXAAFO",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:23:43.893   <-  Bob             license

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

### 15:23:44.012   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6tzfijtc2c",
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
    "id": "recmv6tzfijtc2c",
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
  "time": 318,
  "supportAuthChangesAck": true
}
```

### 15:23:44.020   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "ZkIIDi_KQm6SC5NrAAFN",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:23:44.020   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905024019,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ZkIIDi_KQm6SC5NrAAFN",
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
      "connectionId": "DNFf_Z0GtzxmoVLnAAFP",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:23:44.021   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6tzfijtc2c/Editor.bin/Editor.bin?md5=5Z8Q5zgVOhORcyiCQHhdig&expires=1794499615&shardkey=recmv6tzfijtc2c&filename=Editor.bin"
    },
    "openedAt": 1791912222348
  }
}
```

### 15:23:44.024   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:23:44.030   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "DNFf_Z0GtzxmoVLnAAFP",
  "sessionTimeConnect": 1791905023893,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ZkIIDi_KQm6SC5NrAAFN",
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
      "connectionId": "DNFf_Z0GtzxmoVLnAAFP",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0emZpanRjMmMiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUwMjM4OTN9LCJpYXQiOjE3OTE5MDUwMjQsImV4cCI6MTc5NDQ5NzAyNH0.ggijEdoL2uDeOiv1pUrO-5_Xt8m1CoM43bLs42TyKRk",
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

### 15:23:44.045   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:8"
}
```

### 15:23:44.473   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:24"
}
```

### 15:23:44.533   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:60"
}
```

### 15:23:44.628   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:23:44.629   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:930 memory:{\"totalJSHeapSize\":81400000,\"usedJSHeapSize\":68000000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:23:44.629   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:23:44.629   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:23:44.668   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:23:44.669   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905024669,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:23:45.669   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:23:45.670   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905025670,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:23:45.770       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmv6tzfijtc2c/c/?shardkey=recmv6tzfijtc2c&EIO=4&transport=websocket"
```

### 15:23:45.771   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "XdwS2Swn4jxA7DYmAAFQ",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:23:45.801   <-  Mallory         license

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

### 15:23:45.956   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6tzfijtc2c",
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
    "id": "recmv6tzfijtc2c",
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
  "time": 307,
  "supportAuthChangesAck": true
}
```

### 15:23:45.972   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "C54NxT97yiCBhQf0AAFR",
  "sessionTimeConnect": 1791905025801,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ZkIIDi_KQm6SC5NrAAFN",
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
      "connectionId": "DNFf_Z0GtzxmoVLnAAFP",
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
      "connectionId": "C54NxT97yiCBhQf0AAFR",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ0emZpanRjMmMiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA1MDI1ODAxfSwiaWF0IjoxNzkxOTA1MDI1LCJleHAiOjE3OTQ0OTcwMjV9.87eJ6DJ6dVkDAO2ooNwQtMfbbuhRfR38doHjivnkaE0",
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
  "openedAt": 1791912222348
}
```

### 15:23:45.972   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905025964,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ZkIIDi_KQm6SC5NrAAFN",
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
      "connectionId": "DNFf_Z0GtzxmoVLnAAFP",
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
      "connectionId": "C54NxT97yiCBhQf0AAFR",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:23:45.972   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905025964,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ZkIIDi_KQm6SC5NrAAFN",
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
      "connectionId": "DNFf_Z0GtzxmoVLnAAFP",
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
      "connectionId": "C54NxT97yiCBhQf0AAFR",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:23:45.972   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6tzfijtc2c/Editor.bin/Editor.bin?md5=5Z8Q5zgVOhORcyiCQHhdig&expires=1794499615&shardkey=recmv6tzfijtc2c&filename=Editor.bin"
    },
    "openedAt": 1791912222348
  }
}
```

### 15:23:45.986   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:6"
}
```

### 15:23:46.012   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:26"
}
```

### 15:23:46.062   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:50"
}
```

### 15:23:46.148   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:23:46.149   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:496 memory:{\"totalJSHeapSize\":81400000,\"usedJSHeapSize\":68000000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:23:46.149   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:23:46.149   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:23:46.188   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 15:23:46.189   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905026188,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:23:46.189   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791905026188,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:23:47.136   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 15:23:47.137   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905027137,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:23:47.137   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905027137,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:23:47.137   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 15:23:47.137   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905027137,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:23:47.138   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791905027137,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:23:47.191   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA"
}
```

### 15:23:47.192   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905027192,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:23:47.192   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMwAsADMALAAzACwA",
      "time": 1791905027192,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

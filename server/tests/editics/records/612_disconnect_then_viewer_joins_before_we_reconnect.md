## Info

- Timestamp: 2026-10-16T17:20:26.376Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Alice opens the document, so she holds the document auth lock.
  2. Alice loses her connection (the websocket is closed without a close frame). Her auth lock is automatically released.
  3. Bob join as viewer, so he doesn't try to take the document auth lock and the edition session is still in single-editor mode.
  4. Alice's connection comes back: her editor sends its auth with the previous sessionId and take back the auth lock.
  5. Mallory join as editor, this time Alice has to release the auth lock and switch the edition session in multi-editor mode.

## Events

### 17:19:42.485       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/oommvb8fdzq-cell/c/?shardkey=oommvb8fdzq-cell&EIO=4&transport=websocket"
```

### 17:19:42.594   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "pWnaAoZTuWRyyIfJAAAE",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 17:19:42.599   <-  Alice           license

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

### 17:19:42.728   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "oommvb8fdzq-cell",
  "documentCallbackUrl": "http://host.containers.internal:42289/callback",
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
    "id": "oommvb8fdzq-cell",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:42289/spreadsheet.xlsx",
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
  "time": 535,
  "supportAuthChangesAck": true
}
```

### 17:19:42.746   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "ZmuBp7rWguyQuGlWAAAF",
  "sessionTimeConnect": 1792171182598,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ZmuBp7rWguyQuGlWAAAF",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJvb21tdmI4ZmR6cS1jZWxsIiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTBhMTFjZWMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkFsaWNlIiwiaW5kZXgiOjF9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIxNzExODI1OTh9LCJpYXQiOjE3OTIxNzExODIsImV4cCI6MTc5NDc2MzE4Mn0.p5FLgrPCuxHJYvKORnunMMoX7OtLGPADy0INWtAMMmo",
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
  "openedAt": 1792178382736
}
```

### 17:19:42.813   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/oommvb8fdzq-cell/Editor.bin/Editor.bin?md5=8eZEhNSAiJ2g-8fj-Ofmrw&expires=1794765775&shardkey=oommvb8fdzq-cell&filename=Editor.bin"
    },
    "openedAt": 1792178382736
  }
}
```

### 17:19:43.151   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:6"
}
```

### 17:19:43.190   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:39"
}
```

### 17:19:43.241   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 17:19:43.411   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 17:19:43.411   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1213 memory:{\"totalJSHeapSize\":191260050,\"usedJSHeapSize\":178644358,\"jsHeapSizeLimit\":4395630592}"
}
```

### 17:19:43.411   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 17:19:43.412   <-  Alice           message

```json
{
  "type": "message"
}
```

### 17:19:43.444   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 17:19:44.475   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 17:19:47.099       Alice           [Transport WebSocket] ws-close

```json
"monitor close, code 1006 (connection cut)"
```

### 17:19:48.524       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/oommvb8fdzq-cell/c/?shardkey=oommvb8fdzq-cell&EIO=4&transport=websocket"
```

### 17:19:48.526   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "qp56guTnEtP4y4-1AAAG",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 17:19:48.544   <-  Bob             license

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

### 17:19:48.559   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "oommvb8fdzq-cell",
  "documentCallbackUrl": "http://host.containers.internal:42289/callback",
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
    "id": "oommvb8fdzq-cell",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:42289/spreadsheet.xlsx",
    "title": "spreadsheet.xlsx",
    "lcid": 9,
    "nobase64": true,
    "outputformat": 8194,
    "convertToOrigin": ".pdf.xps.oxps.djvu"
  },
  "lang": "en",
  "mode": "view",
  "permissions": {
    "edit": false
  },
  "encrypted": false,
  "IsAnonymousUser": false,
  "timezoneOffset": -120,
  "coEditingMode": "fast",
  "time": 328,
  "supportAuthChangesAck": true
}
```

### 17:19:48.571   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "c9WWAayX3-avPT00AAAH",
  "sessionTimeConnect": 1792171188543,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "c9WWAayX3-avPT00AAAH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJvb21tdmI4ZmR6cS1jZWxsIiwicGVybWlzc2lvbnMiOnsiZWRpdCI6ZmFsc2V9LCJkc19lbmNyeXB0ZWQiOmZhbHNlfSwiZWRpdG9yQ29uZmlnIjp7InVzZXIiOnsiaWQiOiJkZTEwODA4YzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAwMCIsIm5hbWUiOiJCb2IiLCJpbmRleCI6Mn0sImNvRWRpdGluZyI6eyJtb2RlIjoiZmFzdCJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIxNzExODg1NDN9LCJpYXQiOjE3OTIxNzExODgsImV4cCI6MTc5NDc2MzE4OH0.041wHbsKA0RHXr7_uPs7ITQRhg1K3ulxouoCgZNkE1s",
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
  "openedAt": 1792178382736
}
```

### 17:19:48.572   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/oommvb8fdzq-cell/Editor.bin/Editor.bin?md5=8eZEhNSAiJ2g-8fj-Ofmrw&expires=1794765775&shardkey=oommvb8fdzq-cell&filename=Editor.bin"
    },
    "openedAt": 1792178382736
  }
}
```

### 17:19:48.575   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 17:19:48.600   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:25"
}
```

### 17:19:48.651   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:50"
}
```

### 17:19:48.699   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 17:19:48.699   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:465 memory:{\"totalJSHeapSize\":265810461,\"usedJSHeapSize\":230134293,\"jsHeapSizeLimit\":4395630592}"
}
```

### 17:19:48.699   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 17:19:48.700   <-  Bob             message

```json
{
  "type": "message"
}
```

### 17:20:10.514       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/oommvb8fdzq-cell/c/?shardkey=oommvb8fdzq-cell&EIO=4&transport=websocket"
```

### 17:20:10.516   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "TScWFshM5QuNSdojAAAI",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 17:20:10.519   <-  Alice           license

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

### 17:20:10.520   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "oommvb8fdzq-cell",
  "documentCallbackUrl": "http://host.containers.internal:42289/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "firstname": null,
    "lastname": null,
    "indexUser": 1
  },
  "editorType": 1,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": "ZmuBp7rWguyQuGlWAAAF",
  "sessionTimeConnect": 1792171182598,
  "sessionTimeIdle": 23397,
  "documentFormatSave": 257,
  "isCloseCoAuthoring": false,
  "lang": "en",
  "mode": "edit",
  "permissions": {
    "edit": true
  },
  "encrypted": false,
  "IsAnonymousUser": false,
  "timezoneOffset": -120,
  "coEditingMode": "fast",
  "jwtSession": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJvb21tdmI4ZmR6cS1jZWxsIiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTBhMTFjZWMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkFsaWNlIiwiaW5kZXgiOjF9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIxNzExODI1OTh9LCJpYXQiOjE3OTIxNzExODIsImV4cCI6MTc5NDc2MzE4Mn0.p5FLgrPCuxHJYvKORnunMMoX7OtLGPADy0INWtAMMmo",
  "time": 28327,
  "supportAuthChangesAck": true
}
```

### 17:20:10.522   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {}
}
```

### 17:20:10.523   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792171210522,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "c9WWAayX3-avPT00AAAH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kJs5Ku5zQnW-paeYAAAJ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 17:20:10.523   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "ZmuBp7rWguyQuGlWAAAF",
  "sessionTimeConnect": 1792171182598,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "c9WWAayX3-avPT00AAAH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kJs5Ku5zQnW-paeYAAAJ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
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

### 17:20:17.694       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/oommvb8fdzq-cell/c/?shardkey=oommvb8fdzq-cell&EIO=4&transport=websocket"
```

### 17:20:17.696   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "tdQKxqG6TOWZ6sgnAAAK",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 17:20:17.723   <-  Mallory         license

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

### 17:20:17.855   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "oommvb8fdzq-cell",
  "documentCallbackUrl": "http://host.containers.internal:42289/callback",
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
    "id": "oommvb8fdzq-cell",
    "userid": "de103a11031c00100000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:42289/spreadsheet.xlsx",
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
  "time": 356,
  "supportAuthChangesAck": true
}
```

### 17:20:17.863   <-  Mallory         waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "kJs5Ku5zQnW-paeYAAAJ",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 17:20:17.864   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792171217862,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "c9WWAayX3-avPT00AAAH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kJs5Ku5zQnW-paeYAAAJ",
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
      "connectionId": "Wm9gB8ZySy07V-XvAAAL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 17:20:17.864   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792171217862,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "c9WWAayX3-avPT00AAAH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kJs5Ku5zQnW-paeYAAAJ",
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
      "connectionId": "Wm9gB8ZySy07V-XvAAAL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 17:20:17.865   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/oommvb8fdzq-cell/Editor.bin/Editor.bin?md5=8eZEhNSAiJ2g-8fj-Ofmrw&expires=1794765775&shardkey=oommvb8fdzq-cell&filename=Editor.bin"
    },
    "openedAt": 1792178382736
  }
}
```

### 17:20:17.868   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 17:20:17.875   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "Wm9gB8ZySy07V-XvAAAL",
  "sessionTimeConnect": 1792171217722,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "c9WWAayX3-avPT00AAAH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "kJs5Ku5zQnW-paeYAAAJ",
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
      "connectionId": "Wm9gB8ZySy07V-XvAAAL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 3,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJvb21tdmI4ZmR6cS1jZWxsIiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTAzYTExMDMxYzAwMTAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6Ik1hbGxvcnkiLCJpbmRleCI6M30sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjE3MTIxNzcyMn0sImlhdCI6MTc5MjE3MTIxNywiZXhwIjoxNzk0NzYzMjE3fQ.1owdTV1ZFpbb2F5nkXIoXDD_YwORLpjxRSRJnNGdpnI",
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

### 17:20:17.883   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:5"
}
```

### 17:20:18.023   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:25"
}
```

### 17:20:18.073   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:50"
}
```

### 17:20:18.216   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 17:20:18.216   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:713 memory:{\"totalJSHeapSize\":208930572,\"usedJSHeapSize\":194264132,\"jsHeapSizeLimit\":4395630592}"
}
```

### 17:20:18.216   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 17:20:18.217   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 17:20:18.247   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 17:20:18.248   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792171218247,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 17:20:18.248   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792171218247,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 17:20:19.253   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 17:20:19.254   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792171219254,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 17:20:19.254   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792171219254,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

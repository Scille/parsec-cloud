## Info

- Timestamp: 2026-10-16T16:12:40.878Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document, so she holds the document auth lock.
  2. Alice loses her connection (the websocket is closed without a close frame). Her auth lock is automatically released.
  3. Bob join, so he now holds the document auth lock
  4. Alice's connection comes back: her editor sends its auth with the previous sessionId, Bob gets notified and releases the auth lock.

## Events

### 16:11:49.219       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-01864514503d664fa1e350c607b488cf/web-apps/apps/documenteditor/main/../../../../doc/oommvb60us8-word/c/?shardkey=oommvb60us8-word&EIO=4&transport=websocket"
```

### 16:11:49.333   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "TZDYtYSqoZHApecTAAAA",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 16:11:49.338   <-  Alice           license

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

### 16:11:49.473   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "oommvb60us8-word",
  "documentCallbackUrl": "http://host.containers.internal:33009/callback",
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
    "id": "oommvb60us8-word",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33009/document.docx",
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
  "time": 580,
  "supportAuthChangesAck": true
}
```

### 16:11:49.498   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "JV-SnUXr5tC9FyniAAAB",
  "sessionTimeConnect": 1792167109337,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "JV-SnUXr5tC9FyniAAAB",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJvb21tdmI2MHVzOC13b3JkIiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTBhMTFjZWMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkFsaWNlIiwiaW5kZXgiOjF9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIxNjcxMDkzMzd9LCJpYXQiOjE3OTIxNjcxMDksImV4cCI6MTc5NDc1OTEwOX0.F7B1nC-r44GIsghsWXs7uIt5oRbN1re622ISAQc3Bk8",
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
  "openedAt": 1792174309481
}
```

### 16:11:49.617   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/oommvb60us8-word/Editor.bin/Editor.bin?md5=8masuKpVmQUVFKWwreM5cQ&expires=1794761702&shardkey=oommvb60us8-word&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/oommvb60us8-word/media/image1.png/image1.png?md5=M0KjLtMtYNYK4VyJbU272Q&expires=1794761702&shardkey=oommvb60us8-word&filename=image1.png"
    },
    "openedAt": 1792174309481
  }
}
```

### 16:11:49.648   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:5"
}
```

### 16:11:49.762   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:114"
}
```

### 16:11:50.015   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:254"
}
```

### 16:11:50.240   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 16:11:50.240   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1166 memory:{\"totalJSHeapSize\":94233882,\"usedJSHeapSize\":82371890,\"jsHeapSizeLimit\":4395630592}"
}
```

### 16:11:50.241   <-  Alice           message

```json
{
  "type": "message"
}
```

### 16:12:08.809       Alice           [Transport WebSocket] ws-close

```json
"server close, code 1006"
```

### 16:12:11.608       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-01864514503d664fa1e350c607b488cf/web-apps/apps/documenteditor/main/../../../../doc/oommvb60us8-word/c/?shardkey=oommvb60us8-word&EIO=4&transport=websocket"
```

### 16:12:11.612   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "rXMNv0cUlPKwyW6oAAAA",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 16:12:11.644   <-  Bob             license

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

### 16:12:11.714   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "oommvb60us8-word",
  "documentCallbackUrl": "http://host.containers.internal:33009/callback",
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
    "id": "oommvb60us8-word",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33009/document.docx",
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
  "time": 267,
  "supportAuthChangesAck": true
}
```

### 16:12:11.734   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "O_8LVZpZ_GqvCOBHAAAB",
  "sessionTimeConnect": 1792167131643,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "O_8LVZpZ_GqvCOBHAAAB",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJvb21tdmI2MHVzOC13b3JkIiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTA4MDhjMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkJvYiIsImluZGV4IjoyfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkyMTY3MTMxNjQzfSwiaWF0IjoxNzkyMTY3MTMxLCJleHAiOjE3OTQ3NTkxMzF9.78oMhexgvMP7VMSdyqaNbeImrT96POHMLKAwsT1AnoE",
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
  "openedAt": 1792174309481
}
```

### 16:12:11.735   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/oommvb60us8-word/Editor.bin/Editor.bin?md5=8masuKpVmQUVFKWwreM5cQ&expires=1794761702&shardkey=oommvb60us8-word&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/oommvb60us8-word/media/image1.png/image1.png?md5=M0KjLtMtYNYK4VyJbU272Q&expires=1794761702&shardkey=oommvb60us8-word&filename=image1.png"
    },
    "openedAt": 1792174309481
  }
}
```

### 16:12:11.750   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 16:12:12.206   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:46"
}
```

### 16:12:12.311   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:105"
}
```

### 16:12:12.462   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 16:12:12.462   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:894 memory:{\"totalJSHeapSize\":146929156,\"usedJSHeapSize\":130239572,\"jsHeapSizeLimit\":4395630592}"
}
```

### 16:12:12.463   <-  Bob             message

```json
{
  "type": "message"
}
```

### 16:12:32.825       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-01864514503d664fa1e350c607b488cf/web-apps/apps/documenteditor/main/../../../../doc/oommvb60us8-word/c/?shardkey=oommvb60us8-word&EIO=4&transport=websocket"
```

### 16:12:32.827   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "ufCUBpy2L5_9LixzAAAC",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 16:12:32.829   <-  Alice           license

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

### 16:12:32.830   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "oommvb60us8-word",
  "documentCallbackUrl": "http://host.containers.internal:33009/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "firstname": null,
    "lastname": null,
    "indexUser": 1
  },
  "editorType": 0,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": "JV-SnUXr5tC9FyniAAAB",
  "sessionTimeConnect": 1792167109337,
  "sessionTimeIdle": 23989,
  "documentFormatSave": 65,
  "isCloseCoAuthoring": false,
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
  "jwtSession": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJvb21tdmI2MHVzOC13b3JkIiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTBhMTFjZWMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkFsaWNlIiwiaW5kZXgiOjF9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIxNjcxMDkzMzd9LCJpYXQiOjE3OTIxNjcxMDksImV4cCI6MTc5NDc1OTEwOX0.F7B1nC-r44GIsghsWXs7uIt5oRbN1re622ISAQc3Bk8",
  "time": 43938,
  "supportAuthChangesAck": true
}
```

### 16:12:32.832   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {}
}
```

### 16:12:32.833   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {}
}
```

### 16:12:32.833   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "JV-SnUXr5tC9FyniAAAB",
  "sessionTimeConnect": 1792167109337,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "O_8LVZpZ_GqvCOBHAAAB",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "mB4MLmaKencF6RbkAAAD",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
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

### 16:12:32.834   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792167152833,
  "participants": [
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": false,
      "connectionId": "O_8LVZpZ_GqvCOBHAAAB",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "mB4MLmaKencF6RbkAAAD",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 16:12:32.845   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 16:12:32.861   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 16:12:32.862   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792167152861,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 16:12:32.873   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 16:12:32.873   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792167152873,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

## Info

- Timestamp: 2026-10-17T06:50:05.961Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice and Bob (both in strict co-editing mode) race to lock the same Title
     paragraph: both click it and type before either `getLock` has been
     processed (their sends are held at the proxy and released back to back).
  2. Unlike the spreadsheet (record 55), the word editor locks by the stable
     element id, so the second request collides with the first in the lock
     table: the server resolves the conflict, and the reply to the loser
     carries the table WITHOUT its block (Alice: `{"704": Alice}` both times).
  3. The loser's optimistic edit is rolled back client-side - nothing happens
     on the wire.
  4. Alice saves her own edit (strict: flushed by the explicit save), which
     releases her paragraph lock: the relay to Bob carrying its descriptor.

## Events

### 06:49:54.687       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/documenteditor/main/../../../../doc/recmvc1e3jjipd3/c/?shardkey=recmvc1e3jjipd3&EIO=4&transport=websocket"
```

### 06:49:54.690   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "CtVafmWEXr3ywxOMAAAu",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 06:49:54.705   <-  Alice           license

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

### 06:49:54.840   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvc1e3jjipd3",
  "documentCallbackUrl": "http://host.containers.internal:40061/callback",
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
    "id": "recmvc1e3jjipd3",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:40061/document.docx",
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
  "time": 457,
  "supportAuthChangesAck": true
}
```

### 06:49:54.859   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "EYwrUuGT8-aHDQHGAAAv",
  "sessionTimeConnect": 1792219794705,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "EYwrUuGT8-aHDQHGAAAv",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMxZTNqamlwZDMiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjIxOTc5NDcwNX0sImlhdCI6MTc5MjIxOTc5NCwiZXhwIjoxNzk0ODExNzk0fQ.PPv0ItSf9O4IHaWsknkCkNKm3Yt4z8bulmODT4R8Q7I",
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
  "openedAt": 1792226994848
}
```

### 06:49:54.924   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc1e3jjipd3/Editor.bin/Editor.bin?md5=BbEGSqoTbVBeNVE-ulPRdg&expires=1794814387&shardkey=recmvc1e3jjipd3&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmvc1e3jjipd3/media/image1.png/image1.png?md5=TdERpITO_vRC1gc-o-kveA&expires=1794814387&shardkey=recmvc1e3jjipd3&filename=image1.png"
    },
    "openedAt": 1792226994848
  }
}
```

### 06:49:54.955   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:11"
}
```

### 06:49:55.042   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:86"
}
```

### 06:49:55.143   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 06:49:55.350   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 06:49:55.350   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:795 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 06:49:55.351   <-  Alice           message

```json
{
  "type": "message"
}
```

### 06:49:56.354   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ""
}
```

### 06:49:56.534       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/documenteditor/main/../../../../doc/recmvc1e3jjipd3/c/?shardkey=recmvc1e3jjipd3&EIO=4&transport=websocket"
```

### 06:49:56.536   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "lKWxppeu7K_Fg5OZAAAw",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 06:49:56.561   <-  Bob             license

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

### 06:49:56.637   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmvc1e3jjipd3",
  "documentCallbackUrl": "http://host.containers.internal:40061/callback",
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
    "id": "recmvc1e3jjipd3",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:40061/document.docx",
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

### 06:49:56.643   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "EYwrUuGT8-aHDQHGAAAv",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 06:49:56.643   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792219796642,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "EYwrUuGT8-aHDQHGAAAv",
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
      "connectionId": "mV3wv3TjnHtUvHubAAAx",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 06:49:56.644   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc1e3jjipd3/Editor.bin/Editor.bin?md5=BbEGSqoTbVBeNVE-ulPRdg&expires=1794814387&shardkey=recmvc1e3jjipd3&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmvc1e3jjipd3/media/image1.png/image1.png?md5=TdERpITO_vRC1gc-o-kveA&expires=1794814387&shardkey=recmvc1e3jjipd3&filename=image1.png"
    },
    "openedAt": 1792226994848
  }
}
```

### 06:49:56.645   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 06:49:56.652   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "mV3wv3TjnHtUvHubAAAx",
  "sessionTimeConnect": 1792219796561,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "EYwrUuGT8-aHDQHGAAAv",
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
      "connectionId": "mV3wv3TjnHtUvHubAAAx",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMxZTNqamlwZDMiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIyMTk3OTY1NjF9LCJpYXQiOjE3OTIyMTk3OTYsImV4cCI6MTc5NDgxMTc5Nn0.3sjuolUeAzJqB13Gwy74bzOOWYM21ocljbU1-kYpnSw",
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

### 06:49:56.658   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:8"
}
```

### 06:49:57.123   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:45"
}
```

### 06:49:57.175   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:52"
}
```

### 06:49:57.319   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 06:49:57.319   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:832 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 06:49:57.320   <-  Bob             message

```json
{
  "type": "message"
}
```

### 06:49:57.351   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 06:49:57.352   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1792219797352,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:49:58.324   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ""
}
```

### 06:49:58.325   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "",
      "time": 1792219798325,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:50:01.032   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    "704"
  ]
}
```

### 06:50:01.083   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    "704"
  ]
}
```

### 06:50:01.134   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1792219801032,
      "user": "de10a11cec00100000000000000000001",
      "block": "704"
    }
  }
}
```

### 06:50:01.184   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1792219801032,
      "user": "de10a11cec00100000000000000000001",
      "block": "704"
    }
  }
}
```

### 06:50:01.235   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1792219801032,
      "user": "de10a11cec00100000000000000000001",
      "block": "704"
    }
  }
}
```

### 06:50:01.285   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "704": {
      "time": 1792219801032,
      "user": "de10a11cec00100000000000000000001",
      "block": "704"
    }
  }
}
```

### 06:50:02.845   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 06:50:02.846   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 06:50:02.847   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABBAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 06:50:02.854   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvc1e3jjipd3",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1792219802000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmvc1e3jjipd3",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1792219802000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 2,
  "syncChangesIndex": 2,
  "endSaveChanges": true,
  "locks": [
    {
      "block": "704",
      "user": "de10a11cec00100000000000000000001",
      "time": 1792219802854,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":null}"
}
```

### 06:50:02.854   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792219802000,
  "syncChangesIndex": 2
}
```

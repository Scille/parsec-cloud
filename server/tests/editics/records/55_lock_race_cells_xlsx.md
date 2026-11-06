## Info

- Timestamp: 2026-10-17T06:48:25.511Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Alice and Bob race to lock the same cell B3: both click it and type
     before either `getLock` has been processed (their sends are held at the
     proxy and released back to back, so the server sees both requests before
     either client learns of the other's lock).
  2. The server keys spreadsheet locks by the client-generated block guid - no
     range-conflict check - so both overlapping blocks land in the lock table
     (each `getLock` reply/broadcast shows both entries).
  3. Both clients commit their edit: the two save cycles flow one after the
     other (no save lock contention: Bob's `isSaveLock` is only released after
     Alice's save completed), and both `saveChanges` are broadcast.

## Events

### 06:48:15.944       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvc1bzbuozy4/c/?shardkey=recmvc1bzbuozy4&EIO=4&transport=websocket"
```

### 06:48:15.947   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "3pvfK24A0aCGCvsNAAAq",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 06:48:15.958   <-  Alice           license

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

### 06:48:16.103   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvc1bzbuozy4",
  "documentCallbackUrl": "http://host.containers.internal:42007/callback",
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
    "id": "recmvc1bzbuozy4",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:42007/spreadsheet.xlsx",
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
  "time": 492,
  "supportAuthChangesAck": true
}
```

### 06:48:16.120   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "JdguvglkrgiEOBNfAAAr",
  "sessionTimeConnect": 1792219695958,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "JdguvglkrgiEOBNfAAAr",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMxYnpidW96eTQiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjIxOTY5NTk1OH0sImlhdCI6MTc5MjIxOTY5NiwiZXhwIjoxNzk0ODExNjk2fQ.IiJP2iM7Jhzb4SrWTfASvli0rueFD0F31w2Ig5t0NoE",
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
  "openedAt": 1792226896109
}
```

### 06:48:16.175   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc1bzbuozy4/Editor.bin/Editor.bin?md5=wUhFJHFAb4jZmq2-i0Mu0w&expires=1794814289&shardkey=recmvc1bzbuozy4&filename=Editor.bin"
    },
    "openedAt": 1792226896109
  }
}
```

### 06:48:16.256   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:9"
}
```

### 06:48:16.305   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:47"
}
```

### 06:48:16.354   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 06:48:16.491   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 06:48:16.491   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:876 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 06:48:16.491   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 06:48:16.492   <-  Alice           message

```json
{
  "type": "message"
}
```

### 06:48:16.525   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:48:17.565   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:48:18.090       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvc1bzbuozy4/c/?shardkey=recmvc1bzbuozy4&EIO=4&transport=websocket"
```

### 06:48:18.092   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "NFpDAMX_Ayf44y3qAAAs",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 06:48:18.122   <-  Bob             license

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

### 06:48:18.231   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmvc1bzbuozy4",
  "documentCallbackUrl": "http://host.containers.internal:42007/callback",
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
    "id": "recmvc1bzbuozy4",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:42007/spreadsheet.xlsx",
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
  "time": 728,
  "supportAuthChangesAck": true
}
```

### 06:48:18.237   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "JdguvglkrgiEOBNfAAAr",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 06:48:18.237   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792219698236,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "JdguvglkrgiEOBNfAAAr",
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
      "connectionId": "zy8WvbRjNsg9D5GXAAAt",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 06:48:18.238   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc1bzbuozy4/Editor.bin/Editor.bin?md5=wUhFJHFAb4jZmq2-i0Mu0w&expires=1794814289&shardkey=recmvc1bzbuozy4&filename=Editor.bin"
    },
    "openedAt": 1792226896109
  }
}
```

### 06:48:18.242   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 06:48:18.249   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "zy8WvbRjNsg9D5GXAAAt",
  "sessionTimeConnect": 1792219698121,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "JdguvglkrgiEOBNfAAAr",
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
      "connectionId": "zy8WvbRjNsg9D5GXAAAt",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMxYnpidW96eTQiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIyMTk2OTgxMjF9LCJpYXQiOjE3OTIyMTk2OTgsImV4cCI6MTc5NDgxMTY5OH0.ZmbRUSbbkVUpkNclYX_do62PTA7p27w4oEZ7QAVuIDA",
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

### 06:48:18.252   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 06:48:18.277   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:25"
}
```

### 06:48:18.327   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:50"
}
```

### 06:48:18.417   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 06:48:18.418   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:911 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 06:48:18.418   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 06:48:18.418   <-  Bob             message

```json
{
  "type": "message"
}
```

### 06:48:18.450   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:48:18.451   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792219698451,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:48:21.241   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "45785e59-d12c-010d-722e-1ed15fccd4c8",
      "rangeOrObjectId": {
        "c1": 1,
        "r1": 2,
        "c2": 1,
        "r2": 2
      }
    }
  ]
}
```

### 06:48:21.292   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "e882200e-7c34-21aa-9b1f-961a7de9c236",
      "rangeOrObjectId": {
        "c1": 1,
        "r1": 2,
        "c2": 1,
        "r2": 2
      }
    }
  ]
}
```

### 06:48:21.343   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 06:48:21.343   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1792219701343,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 06:48:21.394   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 06:48:21.394   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 06:48:21.397   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\",\"14;CgAAAAE1AAAA/wAAAAA=\",\"107;ZwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABHAAAAAAICAQIBAgkDOgAAAAAAAQkBJwAAAAAIGgAAAEEAbABpAGMAZQAgAHcAaQBuAHMAIABCADMAAQACAAMCAQIAAwEEAQUBBgE=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": true
}
```

### 06:48:21.402   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvc1bzbuozy4",
      "change": "\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\"",
      "time": 1792219701000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmvc1bzbuozy4",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792219701000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmvc1bzbuozy4",
      "change": "\"107;ZwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABHAAAAAAICAQIBAgkDOgAAAAAAAQkBJwAAAAAIGgAAAEEAbABpAGMAZQAgAHcAaQBuAHMAIABCADMAAQACAAMCAQIAAwEEAQUBBgE=\"",
      "time": 1792219701000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 3,
  "syncChangesIndex": 3,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "45785e59-d12c-010d-722e-1ed15fccd4c8",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1792219701402,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 06:48:21.403   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792219701000,
  "syncChangesIndex": 3
}
```

### 06:48:21.429   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 06:48:21.430   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1792219701430,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 06:48:21.444   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 06:48:21.444   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1792219701444,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:48:21.494   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 06:48:21.495   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 06:48:21.510   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"103;YwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABDAAAAAAICAQIBAgkDNgAAAAAAAQkBIwAAAAAIFgAAAEIAbwBiACAAdwBpAG4AcwAgAEIAMwABAAIAAwIBAgADAQQBBQEGAQ==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": true
}
```

### 06:48:21.516   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvc1bzbuozy4",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792219701000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvc1bzbuozy4",
      "change": "\"103;YwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAABDAAAAAAICAQIBAgkDNgAAAAAAAQkBIwAAAAAIFgAAAEIAbwBiACAAdwBpAG4AcwAgAEIAMwABAAIAAwIBAgADAQQBBQEGAQ==\"",
      "time": 1792219701000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 5,
  "syncChangesIndex": 5,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "e882200e-7c34-21aa-9b1f-961a7de9c236",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1792219701516,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 06:48:21.516   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 3,
  "time": 1792219701000,
  "syncChangesIndex": 5
}
```

### 06:48:21.544   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "45785e59-d12c-010d-722e-1ed15fccd4c8": {
      "time": 1792219701241,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "45785e59-d12c-010d-722e-1ed15fccd4c8",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 06:48:21.549   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 5
}
```

### 06:48:21.550   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 06:48:21.558   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 06:48:21.559   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 06:48:21.595   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "45785e59-d12c-010d-722e-1ed15fccd4c8": {
      "time": 1792219701241,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "45785e59-d12c-010d-722e-1ed15fccd4c8",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 06:48:21.646   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "45785e59-d12c-010d-722e-1ed15fccd4c8": {
      "time": 1792219701241,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "45785e59-d12c-010d-722e-1ed15fccd4c8",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    },
    "e882200e-7c34-21aa-9b1f-961a7de9c236": {
      "time": 1792219701292,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "e882200e-7c34-21aa-9b1f-961a7de9c236",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 06:48:21.657   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 5
}
```

### 06:48:21.657   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 06:48:21.658   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 06:48:21.659   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 06:48:21.696   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "45785e59-d12c-010d-722e-1ed15fccd4c8": {
      "time": 1792219701241,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "45785e59-d12c-010d-722e-1ed15fccd4c8",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    },
    "e882200e-7c34-21aa-9b1f-961a7de9c236": {
      "time": 1792219701292,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "e882200e-7c34-21aa-9b1f-961a7de9c236",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    }
  }
}
```

### 06:48:21.737   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 06:48:21.738   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1792219701737,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:48:22.470   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA"
}
```

### 06:48:22.470   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMwAsADEALAAzACwA",
      "time": 1792219702470,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

## Info

- Timestamp: 2026-10-17T06:47:26.646Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Alice and Bob co-edit (fast mode).
  2. Bob selects A3 and deletes the column A (the API behind the toolbar
     action): the structural change takes the whole-column lock and saves
     right away, his `saveChanges` carrying a non-null `excelAdditionalInfo`
     with the column lock-shift info in its Remove variant (`_recalcType: 2`,
     `m_bIsSaveIndex: false` - unlike record 53's Add insert), relayed
     as-is to Alice.
  3. Alice's follow-up save re-syncs the change index space: it carries the
     same blob with the save-index marker (`_recalcType: 1`,
     `m_bIsSaveIndex: true` - the flipped counterpart of Bob's Remove
     record) with an EMPTY changes list.

## Events

### 06:47:17.608       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvc1aqc8i3te/c/?shardkey=recmvc1aqc8i3te&EIO=4&transport=websocket"
```

### 06:47:17.611   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "4tNy1vBVRZQHhcO8AAAc",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 06:47:17.637   <-  Alice           license

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

### 06:47:17.780   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvc1aqc8i3te",
  "documentCallbackUrl": "http://host.containers.internal:35593/callback",
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
    "id": "recmvc1aqc8i3te",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:35593/spreadsheet.xlsx",
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
  "time": 478,
  "supportAuthChangesAck": true
}
```

### 06:47:17.794   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "fnDXE2kVT0e8mYPfAAAd",
  "sessionTimeConnect": 1792219637637,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "fnDXE2kVT0e8mYPfAAAd",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMxYXFjOGkzdGUiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjIxOTYzNzYzN30sImlhdCI6MTc5MjIxOTYzNywiZXhwIjoxNzk0ODExNjM3fQ.S9YgiEuZGH2_18stj76d4jA24FG38VhIFCVSDeUVo_k",
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
  "openedAt": 1792226837785
}
```

### 06:47:17.846   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc1aqc8i3te/Editor.bin/Editor.bin?md5=hhabcxnYBAUVIUtqcTmy4A&expires=1794814230&shardkey=recmvc1aqc8i3te&filename=Editor.bin"
    },
    "openedAt": 1792226837785
  }
}
```

### 06:47:17.898   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:8"
}
```

### 06:47:17.936   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:37"
}
```

### 06:47:17.986   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 06:47:18.119   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 06:47:18.120   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:812 memory:{\"totalJSHeapSize\":81400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 06:47:18.120   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 06:47:18.120   <-  Alice           message

```json
{
  "type": "message"
}
```

### 06:47:18.151   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:19.184   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:19.332       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvc1aqc8i3te/c/?shardkey=recmvc1aqc8i3te&EIO=4&transport=websocket"
```

### 06:47:19.334   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "nLKSVus4rAZaJYC_AAAe",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 06:47:19.358   <-  Bob             license

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

### 06:47:19.448   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmvc1aqc8i3te",
  "documentCallbackUrl": "http://host.containers.internal:35593/callback",
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
    "id": "recmvc1aqc8i3te",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:35593/spreadsheet.xlsx",
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
  "time": 282,
  "supportAuthChangesAck": true
}
```

### 06:47:19.454   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "fnDXE2kVT0e8mYPfAAAd",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 06:47:19.454   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792219639453,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "fnDXE2kVT0e8mYPfAAAd",
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
      "connectionId": "2kYQfkFgs3ptAlWOAAAf",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 06:47:19.455   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc1aqc8i3te/Editor.bin/Editor.bin?md5=hhabcxnYBAUVIUtqcTmy4A&expires=1794814230&shardkey=recmvc1aqc8i3te&filename=Editor.bin"
    },
    "openedAt": 1792226837785
  }
}
```

### 06:47:19.456   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 06:47:19.463   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "2kYQfkFgs3ptAlWOAAAf",
  "sessionTimeConnect": 1792219639357,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "fnDXE2kVT0e8mYPfAAAd",
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
      "connectionId": "2kYQfkFgs3ptAlWOAAAf",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMxYXFjOGkzdGUiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIyMTk2MzkzNTd9LCJpYXQiOjE3OTIyMTk2MzksImV4cCI6MTc5NDgxMTYzOX0.z9Lcc4U06k1IUnuFeyCA4DJZDIbm88u3ouDmLHc0Euo",
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

### 06:47:19.463   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 06:47:19.929   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:25"
}
```

### 06:47:19.987   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:58"
}
```

### 06:47:20.074   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 06:47:20.074   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:904 memory:{\"totalJSHeapSize\":81400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 06:47:20.074   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 06:47:20.074   <-  Bob             message

```json
{
  "type": "message"
}
```

### 06:47:20.109   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:20.110   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792219640110,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:47:21.116   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:21.117   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792219641117,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:47:22.156   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA"
}
```

### 06:47:22.157   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA",
      "time": 1792219642157,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:47:22.473   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": 1,
      "guid": "8c4a041b-2377-3ff0-6632-edf666ab7133",
      "rangeOrObjectId": {
        "c1": 0,
        "r1": 0,
        "c2": 0,
        "r2": 1048575
      }
    }
  ]
}
```

### 06:47:22.474   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "8c4a041b-2377-3ff0-6632-edf666ab7133": {
      "time": 1792219642474,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 1,
        "guid": "8c4a041b-2377-3ff0-6632-edf666ab7133",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 0,
          "c2": 0,
          "r2": 1048575
        }
      }
    }
  }
}
```

### 06:47:22.474   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "8c4a041b-2377-3ff0-6632-edf666ab7133": {
      "time": 1792219642474,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 1,
        "guid": "8c4a041b-2377-3ff0-6632-edf666ab7133",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 0,
          "c2": 0,
          "r2": 1048575
        }
      }
    }
  }
}
```

### 06:47:22.493   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 06:47:22.493   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 06:47:22.498   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"44;KAAAAAE2AQECAAAANgABAAAAAAAAAAAAAAAAAAAAAAAIAAAAAAIAAQIAAgA=\",\"44;KAAAAAE2AQECAAAANgABAAAAAAEAAAAAAAAAAQAAAAAIAAAAAAIBAQIAAgA=\",\"44;KAAAAAE2AQECAAAANgABAAAAAAIAAAAAAAAAAgAAAAAIAAAAAAICAQIAAgA=\",\"44;KAAAAAE2AQECAAAANgABAAAAAAMAAAAAAAAAAwAAAAAIAAAAAAIDAQIAAgA=\",\"45;KQAAAAE2AwECAAAANgABAAAAAAAAAAD/PwAA//8PAAYJAAAAAAIAAQIAAgQA\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":false}]}},\"indexRows\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 06:47:22.502   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:22.503   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792219642503,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 06:47:22.507   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvc1aqc8i3te",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792219642000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvc1aqc8i3te",
      "change": "\"44;KAAAAAE2AQECAAAANgABAAAAAAAAAAAAAAAAAAAAAAAIAAAAAAIAAQIAAgA=\"",
      "time": 1792219642000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvc1aqc8i3te",
      "change": "\"44;KAAAAAE2AQECAAAANgABAAAAAAEAAAAAAAAAAQAAAAAIAAAAAAIBAQIAAgA=\"",
      "time": 1792219642000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvc1aqc8i3te",
      "change": "\"44;KAAAAAE2AQECAAAANgABAAAAAAIAAAAAAAAAAgAAAAAIAAAAAAICAQIAAgA=\"",
      "time": 1792219642000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvc1aqc8i3te",
      "change": "\"44;KAAAAAE2AQECAAAANgABAAAAAAMAAAAAAAAAAwAAAAAIAAAAAAIDAQIAAgA=\"",
      "time": 1792219642000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvc1aqc8i3te",
      "change": "\"45;KQAAAAE2AwECAAAANgABAAAAAAAAAAD/PwAA//8PAAYJAAAAAAIAAQIAAgQA\"",
      "time": 1792219642000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 6,
  "syncChangesIndex": 6,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 1,
        "guid": "8c4a041b-2377-3ff0-6632-edf666ab7133",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 0,
          "c2": 0,
          "r2": 1048575
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1792219642507,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":false}]}},\"indexRows\":null}"
}
```

### 06:47:22.508   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792219642000,
  "syncChangesIndex": 6
}
```

### 06:47:22.538   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 6
}
```

### 06:47:22.539   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 06:47:22.573   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":1,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":true}]}},\"indexRows\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 06:47:22.575   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [],
  "changesIndex": 6,
  "syncChangesIndex": 6,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":1,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":true}]}},\"indexRows\":null}"
}
```

### 06:47:22.575   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 6,
  "time": 1792219642000,
  "syncChangesIndex": 6
}
```

### 06:47:23.196   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA"
}
```

### 06:47:23.197   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA",
      "time": 1792219643197,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:47:23.538   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:23.539   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792219643538,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

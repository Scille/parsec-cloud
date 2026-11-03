## Info

- Timestamp: 2026-10-17T06:47:11.043Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Alice and Bob co-edit (fast mode).
  2. Bob selects A3 and inserts a row there (the API behind the toolbar
     action): the structural change takes the whole-row lock (`subType: 4`,
     full row width) and saves right away, his `saveChanges` carrying a
     non-null `excelAdditionalInfo` with the row lock-shift info (`indexRows`
     non-null, `indexCols` null - the mirror of record 53's column
     insertion), relayed as-is to Alice.
  3. Alice's follow-up save re-syncs the change index space: it carries the
     same blob with the save-index marker (`_recalcType: 2`,
     `m_bIsSaveIndex: true` - the flipped counterpart of Bob's Add record)
     with an EMPTY changes list.

## Events

### 06:47:02.431       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvc1aelv4jsq/c/?shardkey=recmvc1aelv4jsq&EIO=4&transport=websocket"
```

### 06:47:02.434   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "YZQ2i-eF9H6MS0NMAAAY",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 06:47:02.471   <-  Alice           license

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

### 06:47:02.605   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvc1aelv4jsq",
  "documentCallbackUrl": "http://host.containers.internal:37295/callback",
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
    "id": "recmvc1aelv4jsq",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:37295/spreadsheet.xlsx",
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
  "time": 488,
  "supportAuthChangesAck": true
}
```

### 06:47:02.621   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "25HRV_5M2c5JHaVBAAAZ",
  "sessionTimeConnect": 1792219622471,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "25HRV_5M2c5JHaVBAAAZ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMxYWVsdjRqc3EiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjIxOTYyMjQ3MX0sImlhdCI6MTc5MjIxOTYyMiwiZXhwIjoxNzk0ODExNjIyfQ.bvEtWd5Ep7F-RDQTUiqMNv23ehjpK7ETydBwoxPbFfI",
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
  "openedAt": 1792226822610
}
```

### 06:47:02.686   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc1aelv4jsq/Editor.bin/Editor.bin?md5=HglI4M9TSV3FQK2kl3eBuw&expires=1794814215&shardkey=recmvc1aelv4jsq&filename=Editor.bin"
    },
    "openedAt": 1792226822610
  }
}
```

### 06:47:02.725   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:9"
}
```

### 06:47:02.763   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:37"
}
```

### 06:47:02.813   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 06:47:02.949   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 06:47:02.949   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:827 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 06:47:02.949   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 06:47:02.949   <-  Alice           message

```json
{
  "type": "message"
}
```

### 06:47:02.983   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:04.025   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:04.542       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvc1aelv4jsq/c/?shardkey=recmvc1aelv4jsq&EIO=4&transport=websocket"
```

### 06:47:04.544   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "pNcJ_kgG7-6YPC3lAAAa",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 06:47:04.565   <-  Bob             license

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

### 06:47:04.660   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmvc1aelv4jsq",
  "documentCallbackUrl": "http://host.containers.internal:37295/callback",
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
    "id": "recmvc1aelv4jsq",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:37295/spreadsheet.xlsx",
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
  "time": 697,
  "supportAuthChangesAck": true
}
```

### 06:47:04.666   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "25HRV_5M2c5JHaVBAAAZ",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 06:47:04.666   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792219624664,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "25HRV_5M2c5JHaVBAAAZ",
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
      "connectionId": "M5FPW82FTdQBen-BAAAb",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 06:47:04.667   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc1aelv4jsq/Editor.bin/Editor.bin?md5=HglI4M9TSV3FQK2kl3eBuw&expires=1794814215&shardkey=recmvc1aelv4jsq&filename=Editor.bin"
    },
    "openedAt": 1792226822610
  }
}
```

### 06:47:04.668   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 06:47:04.674   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "M5FPW82FTdQBen-BAAAb",
  "sessionTimeConnect": 1792219624565,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "25HRV_5M2c5JHaVBAAAZ",
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
      "connectionId": "M5FPW82FTdQBen-BAAAb",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMxYWVsdjRqc3EiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIyMTk2MjQ1NjV9LCJpYXQiOjE3OTIyMTk2MjQsImV4cCI6MTc5NDgxMTYyNH0.ka0MmGf7a0XZQ1p-li39d80Bwji7WYMRndU2Qp6e2uI",
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

### 06:47:04.675   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 06:47:04.702   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:27"
}
```

### 06:47:04.755   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:53"
}
```

### 06:47:04.845   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 06:47:04.846   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:878 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 06:47:04.846   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 06:47:04.846   <-  Bob             message

```json
{
  "type": "message"
}
```

### 06:47:04.883   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:04.884   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792219624883,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:47:06.421   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA"
}
```

### 06:47:06.422   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA",
      "time": 1792219626422,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:47:06.873   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": 4,
      "guid": "e6394554-65b4-fdd6-6e06-0726246e4eb0",
      "rangeOrObjectId": {
        "c1": 0,
        "r1": 2,
        "c2": 16383,
        "r2": 2
      }
    }
  ]
}
```

### 06:47:06.874   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "e6394554-65b4-fdd6-6e06-0726246e4eb0": {
      "time": 1792219626873,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 4,
        "guid": "e6394554-65b4-fdd6-6e06-0726246e4eb0",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 2,
          "c2": 16383,
          "r2": 2
        }
      }
    }
  }
}
```

### 06:47:06.874   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "e6394554-65b4-fdd6-6e06-0726246e4eb0": {
      "time": 1792219626873,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 4,
        "guid": "e6394554-65b4-fdd6-6e06-0726246e4eb0",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 2,
          "c2": 16383,
          "r2": 2
        }
      }
    }
  }
}
```

### 06:47:06.892   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 06:47:06.892   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:06.896   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 06:47:06.896   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792219626893,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 06:47:06.899   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"45;KQAAAAE2BAECAAAANgABAAAAAAIAAAD/PwAA//8PAAYJAAAAAAICAQICAgQB\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"indexCols\":null,\"indexRows\":{\"6\":{\"_arrElements\":[{\"_recalcType\":1,\"_position\":2,\"_count\":1,\"m_bIsSaveIndex\":false}]}}}",
  "unlock": false,
  "releaseLocks": true
}
```

### 06:47:06.907   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvc1aelv4jsq",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792219626000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvc1aelv4jsq",
      "change": "\"45;KQAAAAE2BAECAAAANgABAAAAAAIAAAD/PwAA//8PAAYJAAAAAAICAQICAgQB\"",
      "time": 1792219626000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 2,
  "syncChangesIndex": 2,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 4,
        "guid": "e6394554-65b4-fdd6-6e06-0726246e4eb0",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 2,
          "c2": 16383,
          "r2": 2
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1792219626906,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"indexCols\":null,\"indexRows\":{\"6\":{\"_arrElements\":[{\"_recalcType\":1,\"_position\":2,\"_count\":1,\"m_bIsSaveIndex\":false}]}}}"
}
```

### 06:47:06.907   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792219626000,
  "syncChangesIndex": 2
}
```

### 06:47:06.931   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 2
}
```

### 06:47:06.932   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 06:47:06.961   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"indexCols\":null,\"indexRows\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":2,\"_count\":1,\"m_bIsSaveIndex\":true}]}}}",
  "unlock": false,
  "releaseLocks": true
}
```

### 06:47:06.962   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [],
  "changesIndex": 2,
  "syncChangesIndex": 2,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"indexCols\":null,\"indexRows\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":2,\"_count\":1,\"m_bIsSaveIndex\":true}]}}}"
}
```

### 06:47:06.962   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 2,
  "time": 1792219626000,
  "syncChangesIndex": 2
}
```

### 06:47:07.448   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA"
}
```

### 06:47:07.449   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA",
      "time": 1792219627448,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 06:47:07.931   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 06:47:07.931   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792219627931,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

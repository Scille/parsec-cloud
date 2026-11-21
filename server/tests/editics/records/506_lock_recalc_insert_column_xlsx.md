## Info

- Timestamp: 2026-10-17T07:00:58.866Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Alice types in B3 WITHOUT Enter: the first keystroke takes the cell lock,
     and the open edit means no save cycle - the block stays held (the trick
     of record 50's D4 edit).
  2. Bob selects A3 and inserts a column there: the structural change takes
     the whole-column lock (`subType: 2`, full column height) and commits and
     saves right away, his `saveChanges` carrying the non-null
     `excelAdditionalInfo` (`indexCols` lock-shift info), relayed as-is to
     Alice.
  3. The document server recalculates the stored locked ranges of the other
     participants from that info: Alice's B3 block is stored as C3.
  4. Bob locks a cell of his own: the `getLock` reply/broadcast carries the
     full lock table - including Alice's block with its SHIFTED range - then
     his save releases his own block.
  5. Alice commits her open edit (the API behind the Enter key): her
     collaborative save releases her block, the released descriptor showing
     the shifted range again (C3).

## Events

### 07:00:45.875       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvc1s1ypb917/c/?shardkey=recmvc1s1ypb917&EIO=4&transport=websocket"
```

### 07:00:45.877   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "tjDU0uzK31xQV0MvAABG",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 07:00:45.911   <-  Alice           license

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

### 07:00:46.037   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvc1s1ypb917",
  "documentCallbackUrl": "http://host.containers.internal:43309/callback",
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
    "id": "recmvc1s1ypb917",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:43309/spreadsheet.xlsx",
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
  "time": 499,
  "supportAuthChangesAck": true
}
```

### 07:00:46.055   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "WtY0JakdCF_8Vj53AABH",
  "sessionTimeConnect": 1792220445911,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "WtY0JakdCF_8Vj53AABH",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMxczF5cGI5MTciLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjIyMDQ0NTkxMX0sImlhdCI6MTc5MjIyMDQ0NiwiZXhwIjoxNzk0ODEyNDQ2fQ.MFLrQLobrDD0i5M3lYe__UGGgkcohB8vN0UB0OqBEAQ",
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
  "openedAt": 1792227646045
}
```

### 07:00:46.108   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc1s1ypb917/Editor.bin/Editor.bin?md5=SPE40IoVGG6H-Ajig0670g&expires=1794815039&shardkey=recmvc1s1ypb917&filename=Editor.bin"
    },
    "openedAt": 1792227646045
  }
}
```

### 07:00:46.170   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:8"
}
```

### 07:00:46.211   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:39"
}
```

### 07:00:46.260   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 07:00:46.393   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 07:00:46.393   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:851 memory:{\"totalJSHeapSize\":86400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 07:00:46.393   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 07:00:46.393   <-  Alice           message

```json
{
  "type": "message"
}
```

### 07:00:46.427   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 07:00:47.472   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 07:00:47.986       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvc1s1ypb917/c/?shardkey=recmvc1s1ypb917&EIO=4&transport=websocket"
```

### 07:00:47.988   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "zw2Ra-CgTVqtMo_aAABI",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 07:00:48.017   <-  Bob             license

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

### 07:00:48.129   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmvc1s1ypb917",
  "documentCallbackUrl": "http://host.containers.internal:43309/callback",
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
    "id": "recmvc1s1ypb917",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:43309/spreadsheet.xlsx",
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
  "time": 721,
  "supportAuthChangesAck": true
}
```

### 07:00:48.137   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "WtY0JakdCF_8Vj53AABH",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 07:00:48.137   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792220448136,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "WtY0JakdCF_8Vj53AABH",
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
      "connectionId": "btL3vWmk1zT1PtR5AABJ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 07:00:48.138   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc1s1ypb917/Editor.bin/Editor.bin?md5=SPE40IoVGG6H-Ajig0670g&expires=1794815039&shardkey=recmvc1s1ypb917&filename=Editor.bin"
    },
    "openedAt": 1792227646045
  }
}
```

### 07:00:48.144   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 07:00:48.150   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "btL3vWmk1zT1PtR5AABJ",
  "sessionTimeConnect": 1792220448016,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "WtY0JakdCF_8Vj53AABH",
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
      "connectionId": "btL3vWmk1zT1PtR5AABJ",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMxczF5cGI5MTciLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTIyMjA0NDgwMTZ9LCJpYXQiOjE3OTIyMjA0NDgsImV4cCI6MTc5NDgxMjQ0OH0.bp6irNXN-268Lt0X-QrCmG75PaEyC6RB3g_QyWxqKRI",
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

### 07:00:48.155   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:3"
}
```

### 07:00:48.180   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:25"
}
```

### 07:00:48.231   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:52"
}
```

### 07:00:48.317   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 07:00:48.317   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:905 memory:{\"totalJSHeapSize\":86400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 07:00:48.317   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 07:00:48.317   <-  Bob             message

```json
{
  "type": "message"
}
```

### 07:00:48.350   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 07:00:48.351   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792220448350,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 07:00:49.974   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "6092b54d-ac19-3f25-9214-de5c25896b6b",
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

### 07:00:49.975   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "6092b54d-ac19-3f25-9214-de5c25896b6b": {
      "time": 1792220449975,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6092b54d-ac19-3f25-9214-de5c25896b6b",
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

### 07:00:49.975   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "6092b54d-ac19-3f25-9214-de5c25896b6b": {
      "time": 1792220449975,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6092b54d-ac19-3f25-9214-de5c25896b6b",
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

### 07:00:49.985   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 07:00:49.985   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1792220449985,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 07:00:50.005   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 07:00:50.006   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1792220450006,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 07:00:51.490   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA"
}
```

### 07:00:51.491   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA",
      "time": 1792220451491,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 07:00:51.944   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": 2,
      "guid": "de7221b9-bafe-bfa7-ce14-2b6f682a0ef8",
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

### 07:00:51.945   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "6092b54d-ac19-3f25-9214-de5c25896b6b": {
      "time": 1792220449975,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6092b54d-ac19-3f25-9214-de5c25896b6b",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    },
    "de7221b9-bafe-bfa7-ce14-2b6f682a0ef8": {
      "time": 1792220451945,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 2,
        "guid": "de7221b9-bafe-bfa7-ce14-2b6f682a0ef8",
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

### 07:00:51.945   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "6092b54d-ac19-3f25-9214-de5c25896b6b": {
      "time": 1792220449975,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6092b54d-ac19-3f25-9214-de5c25896b6b",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 2,
          "c2": 1,
          "r2": 2
        }
      }
    },
    "de7221b9-bafe-bfa7-ce14-2b6f682a0ef8": {
      "time": 1792220451945,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 2,
        "guid": "de7221b9-bafe-bfa7-ce14-2b6f682a0ef8",
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

### 07:00:51.965   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 07:00:51.965   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 07:00:51.969   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"45;KQAAAAE2BQECAAAANgABAAAAAAAAAAD/PwAA//8PAAYJAAAAAAIAAQIAAgQA\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":1,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":false}]}},\"indexRows\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 07:00:51.979   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvc1s1ypb917",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792220451000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvc1s1ypb917",
      "change": "\"45;KQAAAAE2BQECAAAANgABAAAAAAAAAAD/PwAA//8PAAYJAAAAAAIAAQIAAgQA\"",
      "time": 1792220451000,
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
        "subType": 2,
        "guid": "de7221b9-bafe-bfa7-ce14-2b6f682a0ef8",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 0,
          "c2": 0,
          "r2": 1048575
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1792220451979,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":1,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":false}]}},\"indexRows\":null}"
}
```

### 07:00:51.979   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1792220451000,
  "syncChangesIndex": 2
}
```

### 07:00:52.524   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA"
}
```

### 07:00:52.525   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA",
      "time": 1792220452525,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 07:00:53.564   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "fdbb65e1-16d4-f304-bf75-96c9625fba4f",
      "rangeOrObjectId": {
        "c1": 3,
        "r1": 1,
        "c2": 3,
        "r2": 1
      }
    }
  ]
}
```

### 07:00:53.565   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "6092b54d-ac19-3f25-9214-de5c25896b6b": {
      "time": 1792220449975,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6092b54d-ac19-3f25-9214-de5c25896b6b",
        "rangeOrObjectId": {
          "c1": 2,
          "r1": 2,
          "c2": 2,
          "r2": 2
        }
      }
    },
    "fdbb65e1-16d4-f304-bf75-96c9625fba4f": {
      "time": 1792220453565,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "fdbb65e1-16d4-f304-bf75-96c9625fba4f",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 1,
          "c2": 3,
          "r2": 1
        }
      }
    }
  }
}
```

### 07:00:53.565   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "6092b54d-ac19-3f25-9214-de5c25896b6b": {
      "time": 1792220449975,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6092b54d-ac19-3f25-9214-de5c25896b6b",
        "rangeOrObjectId": {
          "c1": 2,
          "r1": 2,
          "c2": 2,
          "r2": 2
        }
      }
    },
    "fdbb65e1-16d4-f304-bf75-96c9625fba4f": {
      "time": 1792220453565,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "fdbb65e1-16d4-f304-bf75-96c9625fba4f",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 1,
          "c2": 3,
          "r2": 1
        }
      }
    }
  }
}
```

### 07:00:53.571   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAzACwAMQAsADMALAAxACwA"
}
```

### 07:00:53.572   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAzACwAMQAsADMALAAxACwA",
      "time": 1792220453571,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 07:00:54.645   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 2
}
```

### 07:00:54.646   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 07:00:54.650   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"131;fwAAAAE1EAECAAAANgABAwAAAAEAAAADAAAAAQAAAABfAAAAAAIBAQIDAgkDUgAAAAAAAQkBPwAAAAAIMgAAAEIAbwBiACAAbABvAGMAawBzACAAYQAgAHMAaABpAGYAdABlAGQAIABjAGUAbABsAC4AAQACAAMCAQIAAwEEAQUBBgE=\"]",
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

### 07:00:54.657   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvc1s1ypb917",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792220454000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmvc1s1ypb917",
      "change": "\"131;fwAAAAE1EAECAAAANgABAwAAAAEAAAADAAAAAQAAAABfAAAAAAIBAQIDAgkDUgAAAAAAAQkBPwAAAAAIMgAAAEIAbwBiACAAbABvAGMAawBzACAAYQAgAHMAaABpAGYAdABlAGQAIABjAGUAbABsAC4AAQACAAMCAQIAAwEEAQUBBgE=\"",
      "time": 1792220454000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 4,
  "syncChangesIndex": 4,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "fdbb65e1-16d4-f304-bf75-96c9625fba4f",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 1,
          "c2": 3,
          "r2": 1
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1792220454656,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 07:00:54.657   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 2,
  "time": 1792220454000,
  "syncChangesIndex": 4
}
```

### 07:00:55.624   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 4
}
```

### 07:00:55.625   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 07:00:55.656   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\",\"14;CgAAAAE1AAAA/wAAAAA=\",\"109;aQAAAAE1EAECAAAANgABAgAAAAIAAAACAAAAAgAAAABJAAAAAAICAQICAgkDPAAAAAAAAQkBKQAAAAAIHAAAAEEAbABpAGMAZQAgAGwAbwBjAGsAcwAgAEIAMwABAAIAAwIBAgADAQQBBQEGAQ==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":true}]}},\"indexRows\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 07:00:55.661   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 07:00:55.662   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1792220455662,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 07:00:55.663   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmvc1s1ypb917",
      "change": "\"3450;dg0AAAE3FAAABWwNAAAACAAAAAABCFwNAAB7ACIAbQBhAGMAcgBvAHMAQQByAHIAYQB5ACIAOgBbAHsAIgBnAHUAaQBkACIAOgAiAGUAOABlAGEAMgBmAGIAMgA4ADgAMAA1ADQAZABlAGEAYQA2AGIAOAAyADEANQA4AGMAMAA0AGQAZQBlADMANwAiACwAIgBuAGEAbQBlACIAOgAiAEEASQAiACwAIgB2AGEAbAB1AGUAIgA6ACIAXAB0ACgAZgB1AG4AYwB0AGkAbwBuACgAKQBcAG4AXAB0AHsAXABuAFwAdABcAHQALwAqACoAXABuAFwAdABcAHQAIAAqACAARgB1AG4AYwB0AGkAbwBuACAAdABoAGEAdAAgAHIAZQB0AHUAcgBuAHMAIAB0AGgAZQAgAEEASQAgAGEAbgBzAHcAZQByAC4AXABuAFwAdABcAHQAIAAqACAAQABjAHUAcwB0AG8AbQBmAHUAbgBjAHQAaQBvAG4AXABuAFwAdABcAHQAIAAqACAAQABwAGEAcgBhAG0AIAB7AHMAdAByAGkAbgBnAH0AIAB2AGEAbAB1AGUAIABQAHIAbwBtAHAAdAAuAFwAbgBcAHQAXAB0ACAAKgAgAEAAcABhAHIAYQBtACAAewA/AGIAbwBvAGwAZQBhAG4AfQAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIABJAG4AZABpAGMAYQB0AG8AcgAgAHcAaABlAHQAaABlAHIAIAB0AGgAZQAgAEEASQAgAGYAdQBuAGMAdABpAG8AbgAgAHMAaABvAHUAbABkACAAYgBlACAAcwBhAHYAZQBkAC4AXABuAFwAdABcAHQAIAAqACAAQAByAGUAdAB1AHIAbgBzACAAewBzAHQAcgBpAG4AZwB9ACAAQQBuAHMAdwBlAHIAIAB2AGEAbAB1AGUALgBcAG4AXAB0AFwAdAAgACoALwBcAG4AXAB0AFwAdABhAHMAeQBuAGMAIABmAHUAbgBjAHQAaQBvAG4AIABBAEkAKAB2AGEAbAB1AGUALAAgAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AKQAgAHsAXABuAFwAdABcAHQAXAB0AGwAZQB0ACAAcwB5AHMAdABlAG0ATQBlAHMAcwBhAGcAZQAgAD0AIABcACIAQQBzACAAYQBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQAgAGUAeABwAGUAcgB0ACwAIAB5AG8AdQByACAAagBvAGIAIABpAHMAIAB0AG8AIABwAHIAbwB2AGkAZABlACAAYQBkAHYAYQBuAGMAZQBkACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACAAdABoAGEAdAAgAHAAZQByAGYAbwByAG0AIABjAG8AbQBwAGwAZQB4ACAAYwBhAGwAYwB1AGwAYQB0AGkAbwBuAHMAIABvAHIAIABkAGEAdABhACAAbQBhAG4AaQBwAHUAbABhAHQAaQBvAG4AcwAgAGEAcwAgAGQAZQBzAGMAcgBpAGIAZQBkACAAYgB5ACAAdABoAGUAIAB1AHMAZQByAC4AIABLAGUAZQBwACAAeQBvAHUAcgAgAGEAbgBzAHcAZQByAHMAIABhAHMAIABiAHIAaQBlAGYAIABhAHMAIABwAG8AcwBzAGkAYgBsAGUALgAgAEkAZgAgAHQAaABlACAAdQBzAGUAcgAgAGEAcwBrAHMAIABmAG8AcgAgAGYAbwByAG0AdQBsAGEAcwAsACAAcgBlAHQAdQByAG4AIABvAG4AbAB5ACAAdABoAGUAIABmAG8AcgBtAHUAbABhAC4AIABJAGYAIAB0AGgAZQAgAHUAcwBlAHIAIABhAHMAawBzACAAZgBvAHIAIABzAG8AbQBlAHQAaABpAG4AZwAsACAAYQBuAHMAdwBlAHIAIABiAHIAaQBlAGYAbAB5ACAAYQBuAGQAIABvAG4AbAB5ACAAdABoAGUAIAByAGUAcwB1AGwAdAAsACAAdwBpAHQAaABvAHUAdAAgAGQAZQBzAGMAcgBpAHAAdABpAG8AbgBzACAAbwByACAAcgBlAGYAbABlAGMAdABpAG8AbgBzAC4AIABJAGYAIAB5AG8AdQAgAHIAZQBjAGUAaQB2AGUAZAAgAGEAIAByAGUAcQB1AGUAcwB0ACAAdABoAGEAdAAgAGkAcwAgAG4AbwB0ACAAYgBhAHMAZQBkACAAbwBuACAARQB4AGMAZQBsACAAZgBvAHIAbQB1AGwAYQBzACwAIAB0AGgAZQBuACAAcwBpAG0AcABsAHkAIABhAG4AcwB3AGUAcgAgAHQAaABlACAAdABlAHgAdAAgAHIAZQBxAHUAZQBzAHQAIABhAHMAIABiAHIAaQBlAGYAbAB5ACAAYQBzACAAcABvAHMAcwBpAGIAbABlACwAIAB3AGkAdABoAG8AdQB0ACAAZABlAHMAYwByAGkAcAB0AGkAbwBuAHMAIABvAHIAIAByAGUAZgBsAGUAYwB0AGkAbwBuAHMAXAAiADsAXABuAFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAbgBlAHcAIABQAHIAbwBtAGkAcwBlACgAcgBlAHMAbwBsAHYAZQAgAD0APgAgACgAZgB1AG4AYwB0AGkAbwBuACgAKQB7AFwAbgBcAHQAXAB0AFwAdABcAHQAQQBwAGkALgBBAEkAKAB7ACAAdAB5AHAAZQAgADoAIABcACIAdABlAHgAdABcACIALAAgAGQAYQB0AGEAIAA6ACAAWwB7AHIAbwBsAGUAOgAgAFwAIgBzAHkAcwB0AGUAbQBcACIALAAgAGMAbwBuAHQAZQBuAHQAOgAgAHMAeQBzAHQAZQBtAE0AZQBzAHMAYQBnAGUAfQAsACAAewByAG8AbABlADoAXAAiAHUAcwBlAHIAXAAiACwAIABjAG8AbgB0AGUAbgB0ADoAIAB2AGEAbAB1AGUAfQBdACAAfQAsACAAZgB1AG4AYwB0AGkAbwBuACgAZABhAHQAYQApAHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAaQBmACAAKABkAGEAdABhAC4AZQByAHIAbwByACkAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHIAZQB0AHUAcgBuACAAcgBlAHMAbwBsAHYAZQAoAGQAYQB0AGEALgBlAHIAcgBvAHIAKQA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AHMAdwBpAHQAYwBoACAAKABkAGEAdABhAC4AdAB5AHAAZQApACAAewBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAYwBhAHMAZQAgAFwAIgB0AGUAeAB0AFwAIgA6AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAbABlAHQAIAByAGUAcwB1AGwAdAAgAD0AIABkAGEAdABhAC4AdABlAHgAdAAuAHQAcgBpAG0AKAApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABpAGYAIAAoAGkAcwBTAGEAdgBlAEEASQBGAHUAbgBjAHQAaQBvAG4AIAAhAD0APQAgAHQAcgB1AGUAKQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwB1AGwAdAAgAD0AIABcACIAQABAAFwAIgAgACsAIAByAGUAcwB1AGwAdAA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAcgBlAHMAbwBsAHYAZQAoAHIAZQBzAHUAbAB0ACkAOwBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AGIAcgBlAGEAawA7AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAB9AFwAbgBcAHQAXAB0AFwAdABcAHQAXAB0AFwAdABkAGUAZgBhAHUAbAB0ADoAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AHsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAXAAiACMARQBSAFIATwBSAFwAIgApADsAXABuAFwAdABcAHQAXAB0AFwAdABcAHQAXAB0AH0AXABuAFwAdABcAHQAXAB0AFwAdABcAHQAfQBcAG4AXAB0AFwAdABcAHQAXAB0AFwAdAByAGUAcwBvAGwAdgBlACgAZABhAHQAYQApAFwAbgBcAHQAXAB0AFwAdABcAHQAfQApADsAXABuAFwAdABcAHQAXAB0AH0AKQAoACkAKQA7AFwAbgBcAHQAXAB0AH0AXABuAFwAdABcAHQAQQBwAGkALgBBAGQAZABDAHUAcwB0AG8AbQBGAHUAbgBjAHQAaQBvAG4AKABBAEkAKQA7AFwAbgBcAHQAfQApACgAKQA7ACIAfQBdACwAIgBjAHUAcgByAGUAbgB0ACIAOgAtADEAfQACAQMB\"",
      "time": 1792220455000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmvc1s1ypb917",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1792220455000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmvc1s1ypb917",
      "change": "\"109;aQAAAAE1EAECAAAANgABAgAAAAIAAAACAAAAAgAAAABJAAAAAAICAQICAgkDPAAAAAAAAQkBKQAAAAAIHAAAAEEAbABpAGMAZQAgAGwAbwBjAGsAcwAgAEIAMwABAAIAAwIBAgADAQQBBQEGAQ==\"",
      "time": 1792220455000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 7,
  "syncChangesIndex": 7,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "6092b54d-ac19-3f25-9214-de5c25896b6b",
        "rangeOrObjectId": {
          "c1": 2,
          "r1": 2,
          "c2": 2,
          "r2": 2
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1792220455663,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":true}]}},\"indexRows\":null}"
}
```

### 07:00:55.663   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 4,
  "time": 1792220455000,
  "syncChangesIndex": 7
}
```

### 07:00:55.684   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 7
}
```

### 07:00:55.685   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 07:00:55.698   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 07:00:55.699   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 07:00:56.624   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 07:00:56.625   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1792220456625,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 07:00:56.684   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 07:00:56.685   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1792220456685,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

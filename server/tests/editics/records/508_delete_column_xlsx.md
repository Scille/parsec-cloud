## Info

- Timestamp: 2026-11-03T11:40:58.174Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Alice opens the document, Bob joins: both are in co-editing mode.
  2. Bob selects A3 and deletes column A (the position-0 edge case, no lock
     inside): the structural change takes the whole-column lock (`subType: 1`,
     DeleteColumns) and saves right away, his `saveChanges` carrying the
     `excelAdditionalInfo` lock-shift info (`_recalcType: 2`, `_position: 0`,
     `m_bIsSaveIndex: false`), relayed as-is to Alice. Alice's follow-up save
     re-emits it as a save-index marker (`m_bIsSaveIndex: true`) with an
     empty `changes` list, which the server skips.
  3. Alice selects D3 and types WITHOUT Enter: the first keystroke takes the
     D3 cell lock (c1=3, r1=2) and the open edit means no save cycle, so the
     block stays held.
  4. (raced at the proxy, like record 55) Bob selects D3 and deletes that
     column while Alice's `getLock` broadcast to him is held back: the server
     stores Alice's lock, has no range-conflict check (locks are keyed by the
     client-generated guid), so Bob's whole-column structural lock is granted
     and his `saveChanges` carries `_recalcType: 2`, `_position: 3`.
  5. The document server recalculates the stored locked ranges of the OTHER
     participants from that info: Alice's D3 block (INSIDE the deleted range)
     collapses onto the deletion point minus one, i.e. c1 = max(3-1, 3-1) = 2
     (cell C3), per the server's `_addRecalcIndex` + `getLockMe2` pipeline.
  6. Bob then locks F2 and Alice commits a new edit: the `getLock` lock
     table and her save's released-lock descriptor (`saveChanges.locks` with
     `releaseLocks: true`) both show Alice's block with its SHIFTED range
     (c1=2), and her save re-emits the deletion as a save-index marker.

## Events

### 11:39:51.945       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-a3fe5231f2e33dbfa76c025a0c81b4b2/web-apps/apps/spreadsheeteditor/main/../../../../doc/drivemw0m8gmq-cell/c/?shardkey=drivemw0m8gmq-cell&EIO=4&transport=websocket"
```

### 11:39:51.948   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "K4pYwnony1nKyXiBAAAg",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 11:39:51.977   <-  Alice           license

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

### 11:39:52.117   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "drivemw0m8gmq-cell",
  "documentCallbackUrl": "http://host.containers.internal:8091/callback",
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
    "id": "drivemw0m8gmq-cell",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:8091/spreadsheet.xlsx",
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
  "timezoneOffset": -60,
  "coEditingMode": "fast",
  "time": 492,
  "supportAuthChangesAck": true
}
```

### 11:39:52.137   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "icVeZW6kgQ4A42xXAAAh",
  "sessionTimeConnect": 1793705991977,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "icVeZW6kgQ4A42xXAAAh",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJkcml2ZW13MG04Z21xLWNlbGwiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MzcwNTk5MTk3N30sImlhdCI6MTc5MzcwNTk5MiwiZXhwIjoxNzk2Mjk3OTkyfQ.64ZAPSPkfOTA2xYFv7M6Em5Yx8rF0jJEw5VB5GRAD9k",
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
    "websocketMaxPayloadSize": 4096,
    "maxChangesSize": 157286400,
    "limits_image_size": 26214400,
    "limits_image_types_upload": "jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif"
  },
  "openedAt": 1793709592126
}
```

### 11:39:52.192   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/drivemw0m8gmq-cell/Editor.bin/Editor.bin?md5=wP9gxwmq0Tzzb-cBh-ZT3g&expires=1796300585&shardkey=drivemw0m8gmq-cell&filename=Editor.bin"
    },
    "openedAt": 1793709592126
  }
}
```

### 11:39:52.247   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:8"
}
```

### 11:39:52.287   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:39"
}
```

### 11:39:52.337   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 11:39:52.459   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 11:39:52.459   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:830 memory:{\"totalJSHeapSize\":81400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 11:39:52.459   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 11:39:52.459   <-  Alice           message

```json
{
  "type": "message"
}
```

### 11:39:52.489   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 11:39:53.490   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 11:39:59.810       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-a3fe5231f2e33dbfa76c025a0c81b4b2/web-apps/apps/spreadsheeteditor/main/../../../../doc/drivemw0m8gmq-cell/c/?shardkey=drivemw0m8gmq-cell&EIO=4&transport=websocket"
```

### 11:39:59.812   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "7jTWkWbOXg7RionXAAAi",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 11:39:59.837   <-  Bob             license

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

### 11:39:59.937   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "drivemw0m8gmq-cell",
  "documentCallbackUrl": "http://host.containers.internal:8091/callback",
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
    "id": "drivemw0m8gmq-cell",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:8091/spreadsheet.xlsx",
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
  "timezoneOffset": -60,
  "coEditingMode": "fast",
  "time": 295,
  "supportAuthChangesAck": true
}
```

### 11:39:59.943   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "icVeZW6kgQ4A42xXAAAh",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 11:39:59.943   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1793705999942,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "icVeZW6kgQ4A42xXAAAh",
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
      "connectionId": "6BZw8UdhMAoPcXJ8AAAj",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 11:39:59.944   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/drivemw0m8gmq-cell/Editor.bin/Editor.bin?md5=wP9gxwmq0Tzzb-cBh-ZT3g&expires=1796300585&shardkey=drivemw0m8gmq-cell&filename=Editor.bin"
    },
    "openedAt": 1793709592126
  }
}
```

### 11:39:59.946   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 11:39:59.953   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "6BZw8UdhMAoPcXJ8AAAj",
  "sessionTimeConnect": 1793705999837,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "icVeZW6kgQ4A42xXAAAh",
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
      "connectionId": "6BZw8UdhMAoPcXJ8AAAj",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJkcml2ZW13MG04Z21xLWNlbGwiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTM3MDU5OTk4Mzd9LCJpYXQiOjE3OTM3MDU5OTksImV4cCI6MTc5NjI5Nzk5OX0.tpCs035BA-HKo8__7QiHs9PnNfyWPk3OGiljKOZAp94",
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
    "websocketMaxPayloadSize": 4096,
    "maxChangesSize": 157286400,
    "limits_image_size": 26214400,
    "limits_image_types_upload": "jpg;jpeg;jpe;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif"
  }
}
```

### 11:39:59.955   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:3"
}
```

### 11:40:00.437   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:22"
}
```

### 11:40:00.488   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 11:40:00.576   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 11:40:00.576   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:931 memory:{\"totalJSHeapSize\":81400000,\"usedJSHeapSize\":72200000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 11:40:00.576   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 11:40:00.576   <-  Bob             message

```json
{
  "type": "message"
}
```

### 11:40:00.604   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 11:40:00.605   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1793706000604,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 11:40:01.621   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 11:40:01.621   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1793706001621,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 11:40:12.501   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA"
}
```

### 11:40:12.501   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA",
      "time": 1793706012501,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 11:40:12.988   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": 1,
      "guid": "0e0aaa3b-aee9-da18-bee5-52a45f1bdd20",
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

### 11:40:12.989   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "0e0aaa3b-aee9-da18-bee5-52a45f1bdd20": {
      "time": 1793706012988,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 1,
        "guid": "0e0aaa3b-aee9-da18-bee5-52a45f1bdd20",
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

### 11:40:12.989   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "0e0aaa3b-aee9-da18-bee5-52a45f1bdd20": {
      "time": 1793706012988,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 1,
        "guid": "0e0aaa3b-aee9-da18-bee5-52a45f1bdd20",
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

### 11:40:13.010   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 11:40:13.011   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1793706013010,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:13.021   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 11:40:13.021   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 11:40:13.024   ->  Bob             saveChanges

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

### 11:40:13.028   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1793706013000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"44;KAAAAAE2AQECAAAANgABAAAAAAAAAAAAAAAAAAAAAAAIAAAAAAIAAQIAAgA=\"",
      "time": 1793706013000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"44;KAAAAAE2AQECAAAANgABAAAAAAEAAAAAAAAAAQAAAAAIAAAAAAIBAQIAAgA=\"",
      "time": 1793706013000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"44;KAAAAAE2AQECAAAANgABAAAAAAIAAAAAAAAAAgAAAAAIAAAAAAICAQIAAgA=\"",
      "time": 1793706013000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"44;KAAAAAE2AQECAAAANgABAAAAAAMAAAAAAAAAAwAAAAAIAAAAAAIDAQIAAgA=\"",
      "time": 1793706013000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"45;KQAAAAE2AwECAAAANgABAAAAAAAAAAD/PwAA//8PAAYJAAAAAAIAAQIAAgQA\"",
      "time": 1793706013000,
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
        "guid": "0e0aaa3b-aee9-da18-bee5-52a45f1bdd20",
        "rangeOrObjectId": {
          "c1": 0,
          "r1": 0,
          "c2": 0,
          "r2": 1048575
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1793706013028,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":0,\"_count\":1,\"m_bIsSaveIndex\":false}]}},\"indexRows\":null}"
}
```

### 11:40:13.028   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1793706013000,
  "syncChangesIndex": 6
}
```

### 11:40:13.053   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 6
}
```

### 11:40:13.054   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 11:40:13.086   ->  Alice           saveChanges

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

### 11:40:13.088   <-  Bob             saveChanges

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

### 11:40:13.088   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 6,
  "time": 1793706013000,
  "syncChangesIndex": 6
}
```

### 11:40:13.541   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA"
}
```

### 11:40:13.541   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMgAsADAALAAyACwA",
      "time": 1793706013541,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 11:40:14.050   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 11:40:14.051   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1793706014050,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:24.330   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 11:40:24.816   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "d5a80069-d12d-8365-1863-b7cd01dc49d5",
      "rangeOrObjectId": {
        "c1": 3,
        "r1": 2,
        "c2": 3,
        "r2": 2
      }
    }
  ]
}
```

### 11:40:24.828   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 11:40:27.163   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "d5a80069-d12d-8365-1863-b7cd01dc49d5": {
      "time": 1793706027163,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "d5a80069-d12d-8365-1863-b7cd01dc49d5",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 2,
          "c2": 3,
          "r2": 2
        }
      }
    }
  }
}
```

### 11:40:27.163   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "d5a80069-d12d-8365-1863-b7cd01dc49d5": {
      "time": 1793706027163,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "d5a80069-d12d-8365-1863-b7cd01dc49d5",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 2,
          "c2": 3,
          "r2": 2
        }
      }
    }
  }
}
```

### 11:40:29.506   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 11:40:29.989   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": 1,
      "guid": "dc88501d-b4fc-ddfb-a093-923cf22d8337",
      "rangeOrObjectId": {
        "c1": 3,
        "r1": 0,
        "c2": 3,
        "r2": 1048575
      }
    }
  ]
}
```

### 11:40:31.993   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "d5a80069-d12d-8365-1863-b7cd01dc49d5": {
      "time": 1793706027163,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "d5a80069-d12d-8365-1863-b7cd01dc49d5",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 2,
          "c2": 3,
          "r2": 2
        }
      }
    },
    "dc88501d-b4fc-ddfb-a093-923cf22d8337": {
      "time": 1793706031993,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 1,
        "guid": "dc88501d-b4fc-ddfb-a093-923cf22d8337",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 0,
          "c2": 3,
          "r2": 1048575
        }
      }
    }
  }
}
```

### 11:40:31.993   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "d5a80069-d12d-8365-1863-b7cd01dc49d5": {
      "time": 1793706027163,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "d5a80069-d12d-8365-1863-b7cd01dc49d5",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 2,
          "c2": 3,
          "r2": 2
        }
      }
    },
    "dc88501d-b4fc-ddfb-a093-923cf22d8337": {
      "time": 1793706031993,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 1,
        "guid": "dc88501d-b4fc-ddfb-a093-923cf22d8337",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 0,
          "c2": 3,
          "r2": 1048575
        }
      }
    }
  }
}
```

### 11:40:33.026   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 6
}
```

### 11:40:34.513   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 11:40:34.825   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 11:40:35.521   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"45;KQAAAAE2AwECAAAANgABAwAAAAAAAAD/PwAA//8PAAYJAAAAAAIDAQIDAgQA\",\"45;KQAAAAE2NwECAAAANgAABRkAAAAAAAEJCwwAAAAAAgIBAgACAgIDAgACAQMB\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":3,\"_count\":1,\"m_bIsSaveIndex\":false}]}},\"indexRows\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 11:40:35.850   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 11:40:37.037   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1793706037000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"45;KQAAAAE2AwECAAAANgABAwAAAAAAAAD/PwAA//8PAAYJAAAAAAIDAQIDAgQA\"",
      "time": 1793706037000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"45;KQAAAAE2NwECAAAANgAABRkAAAAAAAEJCwwAAAAAAgIBAgACAgIDAgACAQMB\"",
      "time": 1793706037000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 9,
  "syncChangesIndex": 9,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": 1,
        "guid": "dc88501d-b4fc-ddfb-a093-923cf22d8337",
        "rangeOrObjectId": {
          "c1": 3,
          "r1": 0,
          "c2": 3,
          "r2": 1048575
        }
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1793706037036,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":2,\"_position\":3,\"_count\":1,\"m_bIsSaveIndex\":false}]}},\"indexRows\":null}"
}
```

### 11:40:37.037   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 6,
  "time": 1793706037000,
  "syncChangesIndex": 9
}
```

### 11:40:38.542   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1793706038542,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:38.542   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1793706038542,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:38.542   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1793706038542,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:38.542   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1793706038542,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:38.542   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1793706038542,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 11:40:38.570   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 11:40:38.570   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 9
}
```

### 11:40:38.571   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1793706038570,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:38.571   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 11:40:38.605   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":1,\"_position\":3,\"_count\":1,\"m_bIsSaveIndex\":true}]}},\"indexRows\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 11:40:38.607   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [],
  "changesIndex": 9,
  "syncChangesIndex": 9,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "d5a80069-d12d-8365-1863-b7cd01dc49d5",
        "rangeOrObjectId": {
          "c1": 2,
          "r1": 2,
          "c2": 2,
          "r2": 2
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1793706038606,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"indexCols\":{\"6\":{\"_arrElements\":[{\"_recalcType\":1,\"_position\":3,\"_count\":1,\"m_bIsSaveIndex\":true}]}},\"indexRows\":null}"
}
```

### 11:40:38.607   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 9,
  "time": 1793706038000,
  "syncChangesIndex": 9
}
```

### 11:40:38.610   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 11:40:38.611   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1793706038611,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 11:40:39.610   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 11:40:39.611   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1793706039610,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:39.626   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 11:40:39.627   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1793706039626,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 11:40:44.386   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAA1ACwAMQAsADUALAAxACwA"
}
```

### 11:40:44.387   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAA1ACwAMQAsADUALAAxACwA",
      "time": 1793706044386,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 11:40:44.870   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "f09b9ff4-ff35-a043-a6bd-73b3757852ba",
      "rangeOrObjectId": {
        "c1": 5,
        "r1": 1,
        "c2": 5,
        "r2": 1
      }
    }
  ]
}
```

### 11:40:44.871   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "f09b9ff4-ff35-a043-a6bd-73b3757852ba": {
      "time": 1793706044870,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "f09b9ff4-ff35-a043-a6bd-73b3757852ba",
        "rangeOrObjectId": {
          "c1": 5,
          "r1": 1,
          "c2": 5,
          "r2": 1
        }
      }
    }
  }
}
```

### 11:40:44.871   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "f09b9ff4-ff35-a043-a6bd-73b3757852ba": {
      "time": 1793706044870,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "f09b9ff4-ff35-a043-a6bd-73b3757852ba",
        "rangeOrObjectId": {
          "c1": 5,
          "r1": 1,
          "c2": 5,
          "r2": 1
        }
      }
    }
  }
}
```

### 11:40:44.878   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAA1ACwAMQAsADUALAAxACwA"
}
```

### 11:40:44.879   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAA1ACwAMQAsADUALAAxACwA",
      "time": 1793706044879,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 11:40:44.901   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA"
}
```

### 11:40:44.902   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAzACwAMgAsADMALAAyACwA",
      "time": 1793706044902,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:49.017   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwANAAsADEALAA0ACwA"
}
```

### 11:40:49.018   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwANAAsADEALAA0ACwA",
      "time": 1793706049017,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:49.520   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "fc1fc0e1-b2b8-3154-d92a-6b0ebf8e0f42",
      "rangeOrObjectId": {
        "c1": 1,
        "r1": 4,
        "c2": 1,
        "r2": 4
      }
    }
  ]
}
```

### 11:40:49.520   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "f09b9ff4-ff35-a043-a6bd-73b3757852ba": {
      "time": 1793706044870,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "f09b9ff4-ff35-a043-a6bd-73b3757852ba",
        "rangeOrObjectId": {
          "c1": 5,
          "r1": 1,
          "c2": 5,
          "r2": 1
        }
      }
    },
    "fc1fc0e1-b2b8-3154-d92a-6b0ebf8e0f42": {
      "time": 1793706049520,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "fc1fc0e1-b2b8-3154-d92a-6b0ebf8e0f42",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 4,
          "c2": 1,
          "r2": 4
        }
      }
    }
  }
}
```

### 11:40:49.520   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "f09b9ff4-ff35-a043-a6bd-73b3757852ba": {
      "time": 1793706044870,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "f09b9ff4-ff35-a043-a6bd-73b3757852ba",
        "rangeOrObjectId": {
          "c1": 5,
          "r1": 1,
          "c2": 5,
          "r2": 1
        }
      }
    },
    "fc1fc0e1-b2b8-3154-d92a-6b0ebf8e0f42": {
      "time": 1793706049520,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "fc1fc0e1-b2b8-3154-d92a-6b0ebf8e0f42",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 4,
          "c2": 1,
          "r2": 4
        }
      }
    }
  }
}
```

### 11:40:49.526   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwANAAsADEALAA0ACwA"
}
```

### 11:40:49.526   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwANAAsADEALAA0ACwA",
      "time": 1793706049526,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 11:40:49.859   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 9
}
```

### 11:40:49.860   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 11:40:49.861   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"103;YwAAAAE1EAECAAAANgABAQAAAAQAAAABAAAABAAAAABDAAAAAAIEAQIBAgkDNgAAAAAAAQkBIwAAAAAIFgAAAEEAbABpAGMAZQAgAGEAZwBhAGkAbgABAAIAAwIBAgADAQQBBQEGAQ==\",\"45;KQAAAAE2NwECAAAANgAABRkAAAAACQsMAAAAAAICAQIAAgICAwIAAQACAQMB\"]",
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

### 11:40:49.865   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1793706049000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"103;YwAAAAE1EAECAAAANgABAQAAAAQAAAABAAAABAAAAABDAAAAAAIEAQIBAgkDNgAAAAAAAQkBIwAAAAAIFgAAAEEAbABpAGMAZQAgAGEAZwBhAGkAbgABAAIAAwIBAgADAQQBBQEGAQ==\"",
      "time": 1793706049000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "drivemw0m8gmq-cell",
      "change": "\"45;KQAAAAE2NwECAAAANgAABRkAAAAACQsMAAAAAAICAQIAAgICAwIAAQACAQMB\"",
      "time": 1793706049000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 12,
  "syncChangesIndex": 12,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "fc1fc0e1-b2b8-3154-d92a-6b0ebf8e0f42",
        "rangeOrObjectId": {
          "c1": 1,
          "r1": 4,
          "c2": 1,
          "r2": 4
        }
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1793706049865,
      "changes": null
    }
  ],
  "excelAdditionalInfo": null
}
```

### 11:40:49.865   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 9,
  "time": 1793706049000,
  "syncChangesIndex": 12
}
```

### 11:40:50.537   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwANQAsADEALAA1ACwA"
}
```

### 11:40:50.538   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwANQAsADEALAA1ACwA",
      "time": 1793706050537,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

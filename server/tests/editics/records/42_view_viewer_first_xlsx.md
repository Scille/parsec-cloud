## Info

- Timestamp: 2026-10-14T09:37:53.158Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Bob (view) connects first: the session (and the document conversion) is
     created by a read-only participant.
  2. Alice (edit) joins: no waitAuth is involved (a viewer never holds the auth
     lock).
  3. Alice edits the document; her save changes are relayed to Bob (live
     viewer).

## Events

### 09:37:42.986       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmv7x2cargn62/c/?shardkey=recmv7x2cargn62&EIO=4&transport=websocket"
```

### 09:37:42.988   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "eAAkv4LxwzO4OWQ1AAAk",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:37:42.991   <-  Bob             license

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

### 09:37:43.047   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv7x2cargn62",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
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
    "id": "recmv7x2cargn62",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:33303/spreadsheet.xlsx",
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
  "time": 350,
  "supportAuthChangesAck": true
}
```

### 09:37:43.062   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "0VGn6LeqXzmJrThzAAAl",
  "sessionTimeConnect": 1791970662991,
  "participants": [
    {
      "id": "de10808c0010000000000000000000001",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 1,
      "view": true,
      "connectionId": "0VGn6LeqXzmJrThzAAAl",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4MmNhcmduNjIiLCJwZXJtaXNzaW9ucyI6eyJlZGl0IjpmYWxzZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTA4MDhjMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkJvYiIsImluZGV4IjoxfSwiY29FZGl0aW5nIjp7Im1vZGUiOiJmYXN0In0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDY2Mjk5MX0sImlhdCI6MTc5MTk3MDY2MywiZXhwIjoxNzk0NTYyNjYzfQ.HK1mxN_luOfE8H_3SzyxJj-D5DapOntnqaPPrpNZoJg",
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
  "openedAt": 1791977863054
}
```

### 09:37:43.122   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x2cargn62/Editor.bin/Editor.bin?md5=V0etQMCNsFzm2_uK8YXbVg&expires=1794565256&shardkey=recmv7x2cargn62&filename=Editor.bin"
    },
    "openedAt": 1791977863054
  }
}
```

### 09:37:43.128   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:4"
}
```

### 09:37:43.296   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:38"
}
```

### 09:37:43.399   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:103"
}
```

### 09:37:43.498   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 09:37:43.499   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:796 memory:{\"totalJSHeapSize\":76600000,\"usedJSHeapSize\":60300000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:37:43.499   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 09:37:43.499   <-  Bob             message

```json
{
  "type": "message"
}
```

### 09:37:44.622       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmv7x2cargn62/c/?shardkey=recmv7x2cargn62&EIO=4&transport=websocket"
```

### 09:37:44.624   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "cZRnpqIRIOMzHBk9AAAm",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:37:44.652   <-  Alice           license

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

### 09:37:44.781   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv7x2cargn62",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
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
    "id": "recmv7x2cargn62",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:33303/spreadsheet.xlsx",
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
  "time": 324,
  "supportAuthChangesAck": true
}
```

### 09:37:44.795   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "662Izrf2GoF_UJ5XAAAn",
  "sessionTimeConnect": 1791970664652,
  "participants": [
    {
      "id": "de10808c0010000000000000000000001",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 1,
      "view": true,
      "connectionId": "0VGn6LeqXzmJrThzAAAl",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000002",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 2,
      "view": false,
      "connectionId": "662Izrf2GoF_UJ5XAAAn",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4MmNhcmduNjIiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6Mn0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDY2NDY1Mn0sImlhdCI6MTc5MTk3MDY2NCwiZXhwIjoxNzk0NTYyNjY0fQ.ibuSR1avEwj3ZfIhncQT1ntDut2zhVP8Zpm22efqI7o",
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
  "openedAt": 1791977863054
}
```

### 09:37:44.795   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791970664788,
  "participants": [
    {
      "id": "de10808c0010000000000000000000001",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 1,
      "view": true,
      "connectionId": "0VGn6LeqXzmJrThzAAAl",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    },
    {
      "id": "de10a11cec00100000000000000000002",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 2,
      "view": false,
      "connectionId": "662Izrf2GoF_UJ5XAAAn",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 09:37:44.796   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x2cargn62/Editor.bin/Editor.bin?md5=V0etQMCNsFzm2_uK8YXbVg&expires=1794565256&shardkey=recmv7x2cargn62&filename=Editor.bin"
    },
    "openedAt": 1791977863054
  }
}
```

### 09:37:45.224   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:418"
}
```

### 09:37:45.254   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:30"
}
```

### 09:37:45.307   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:54"
}
```

### 09:37:45.396   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:37:45.396   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:936 memory:{\"totalJSHeapSize\":76600000,\"usedJSHeapSize\":60300000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:37:45.397   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 09:37:45.397   <-  Alice           message

```json
{
  "type": "message"
}
```

### 09:37:45.438   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 09:37:45.438   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA",
      "time": 1791970665438,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 09:37:46.475   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 09:37:46.476   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAABAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791970666476,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 09:37:46.883   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "68ab11bb-7519-5ee5-7cdc-a8dfa728833a",
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

### 09:37:46.883   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "68ab11bb-7519-5ee5-7cdc-a8dfa728833a": {
      "time": 1791970666883,
      "user": "de10a11cec00100000000000000000002",
      "block": {
        "sheetId": "6",
        "type": 1,
        "subType": null,
        "guid": "68ab11bb-7519-5ee5-7cdc-a8dfa728833a",
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

### 09:37:46.897   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA"
}
```

### 09:37:46.897   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": ",27;AgAAADYAARAAAAAxACwAMgAsADEALAAyACwA",
      "time": 1791970666897,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 09:37:48.315   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 09:37:48.316   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 09:37:48.322   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"14;CgAAAAE1AAAA/wAAAAA=\",\"99;XwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAAA/AAAAAAICAQIBAgkDMgAAAAAAAQkBHwAAAAAIEgAAAEEAbABpAGMAZQA6ACAANAAyAAEAAgADAgECAAMBBAEFAQYB\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": true,
  "deleteIndex": null,
  "excelAdditionalInfo": null,
  "unlock": false,
  "releaseLocks": false
}
```

### 09:37:48.328   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7x2cargn62",
      "change": "\"14;CgAAAAE1AAAA/wAAAAA=\"",
      "time": 1791970668000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7x2cargn62",
      "change": "\"99;XwAAAAE1EAECAAAANgABAQAAAAIAAAABAAAAAgAAAAA/AAAAAAICAQIBAgkDMgAAAAAAAQkBHwAAAAAIEgAAAEEAbABpAGMAZQA6ACAANAAyAAEAAgADAgECAAMBBAEFAQYB\"",
      "time": 1791970668000,
      "user": "de10a11cec00100000000000000000002",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 2,
  "syncChangesIndex": 2,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": null
}
```

### 09:37:48.328   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791970668000,
  "syncChangesIndex": 2
}
```

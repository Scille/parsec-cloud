## Info

- Timestamp: 2026-10-17T07:08:58.504Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: spreadsheet.xlsx (cell)
- Scenario:
  1. Alice (edit) and Bob (view mode) are connected.
  2. A stock viewer never sends `getLock` (the client-side view mode gates
     it), so a hand-crafted one is injected on his socket from the protocol
     monitor's MITM position.
  3. The document server denies a viewer taking locks: it answers
     `disconnectReason` (code 4004, "access deny") and closes the connection
     (the `ws-close` row), while Alice's session is unaffected - she receives
     the `connectState` broadcast of Bob's departure.

## Events

### 07:08:51.475       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvc22gooh4mc/c/?shardkey=recmvc22gooh4mc&EIO=4&transport=websocket"
```

### 07:08:51.478   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "HjDsGxtzXOa_t4sAAABK",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 07:08:51.490   <-  Alice           license

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

### 07:08:51.633   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmvc22gooh4mc",
  "documentCallbackUrl": "http://host.containers.internal:35605/callback",
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
    "id": "recmvc22gooh4mc",
    "userid": "de10a11cec0010000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:35605/spreadsheet.xlsx",
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
  "time": 475,
  "supportAuthChangesAck": true
}
```

### 07:08:51.652   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "r-wQkeredKENHruyAABL",
  "sessionTimeConnect": 1792220931489,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "r-wQkeredKENHruyAABL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMyMmdvb2g0bWMiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjIyMDkzMTQ4OX0sImlhdCI6MTc5MjIyMDkzMSwiZXhwIjoxNzk0ODEyOTMxfQ.ZZ5lIHVZU3YPF_K2cW2rDvCeBdkcIeMJk_5FtFs4OV4",
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
  "openedAt": 1792228131642
}
```

### 07:08:51.703   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc22gooh4mc/Editor.bin/Editor.bin?md5=wq7cnaZyvdipttvMK73fYw&expires=1794815524&shardkey=recmvc22gooh4mc&filename=Editor.bin"
    },
    "openedAt": 1792228131642
  }
}
```

### 07:08:51.780   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:14"
}
```

### 07:08:51.818   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:37"
}
```

### 07:08:51.868   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:51"
}
```

### 07:08:52.004   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 07:08:52.005   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:842 memory:{\"totalJSHeapSize\":81400000,\"usedJSHeapSize\":64000000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 07:08:52.005   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 07:08:52.005   <-  Alice           message

```json
{
  "type": "message"
}
```

### 07:08:52.035   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 07:08:53.077   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ",27;AgAAADYAABAAAAAwACwAMAAsADAALAAwACwA"
}
```

### 07:08:53.600       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-fac8fa5cf918b19a9638db113956657f/web-apps/apps/spreadsheeteditor/main/../../../../doc/recmvc22gooh4mc/c/?shardkey=recmvc22gooh4mc&EIO=4&transport=websocket"
```

### 07:08:53.601   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "h8xT99pGtas89zHqAABM",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 07:08:53.625   <-  Bob             license

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

### 07:08:53.639   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmvc22gooh4mc",
  "documentCallbackUrl": "http://host.containers.internal:35605/callback",
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
    "id": "recmvc22gooh4mc",
    "userid": "de10808c001000000000000000000000",
    "format": "xlsx",
    "url": "http://host.containers.internal:35605/spreadsheet.xlsx",
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
  "time": 622,
  "supportAuthChangesAck": true
}
```

### 07:08:53.652   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "nZu0x_Q_KKU5DjHzAABN",
  "sessionTimeConnect": 1792220933624,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "r-wQkeredKENHruyAABL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "nZu0x_Q_KKU5DjHzAABN",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdmMyMmdvb2g0bWMiLCJwZXJtaXNzaW9ucyI6eyJlZGl0IjpmYWxzZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTA4MDhjMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkJvYiIsImluZGV4IjoyfSwiY29FZGl0aW5nIjp7Im1vZGUiOiJmYXN0In0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjIyMDkzMzYyNH0sImlhdCI6MTc5MjIyMDkzMywiZXhwIjoxNzk0ODEyOTMzfQ.p4pkyLnF6DWE1vgucvsDwlSgJbrE7g_UeL0qb50NalE",
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
  "openedAt": 1792228131642
}
```

### 07:08:53.653   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792220933646,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "r-wQkeredKENHruyAABL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de10808c0010000000000000000000002",
      "idOriginal": "de10808c001000000000000000000000",
      "username": "Bob",
      "indexUser": 2,
      "view": true,
      "connectionId": "nZu0x_Q_KKU5DjHzAABN",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 07:08:53.653   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmvc22gooh4mc/Editor.bin/Editor.bin?md5=wq7cnaZyvdipttvMK73fYw&expires=1794815524&shardkey=recmvc22gooh4mc&filename=Editor.bin"
    },
    "openedAt": 1792228131642
  }
}
```

### 07:08:53.658   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 07:08:53.693   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:35"
}
```

### 07:08:53.743   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:50"
}
```

### 07:08:53.794   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 07:08:53.794   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:774 memory:{\"totalJSHeapSize\":81400000,\"usedJSHeapSize\":64000000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 07:08:53.794   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": false
}
```

### 07:08:53.795   <-  Bob             message

```json
{
  "type": "message"
}
```

### 07:08:55.390   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "sheetId": "6",
      "type": 1,
      "subType": null,
      "guid": "00000000-0000-0000-0000-000000000000",
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

### 07:08:55.391   <-  Bob             disconnectReason

```json
{
  "type": "disconnectReason",
  "code": 4004,
  "description": "access deny"
}
```

### 07:08:55.391   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1792220935391,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "r-wQkeredKENHruyAABL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 07:08:55.392       Bob             [Transport WebSocket] ws-close

```json
"server close, code 1005"
```

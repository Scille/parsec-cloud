## Info

- Timestamp: 2026-10-14T10:19:05.361Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: presentation.pptx (slide)
- Scenario:
  1. Alice switches to strict co-editing and locks the slide text shape by
     typing in it.
  2. Bob joins: his auth event carries the lock table as a list (empty in every
     record so far).
  3. Bob (also strict) cannot edit the locked shape: the slide has no other
     reachable text.
  4. Alice saves collaboratively: her saveChanges releases her shape lock and
     Bob receives its descriptor, leaving the shape free for Bob to lock and
     edit.

## Events

### 10:18:49.909       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/presentationeditor/main/../../../../doc/recmv7yj7srxir7/c/?shardkey=recmv7yj7srxir7&EIO=4&transport=websocket"
```

### 10:18:49.911   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "x-G4HKB3kBoG_Bi_AAB8",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:18:49.940   <-  Alice           license

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

### 10:18:50.046   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv7yj7srxir7",
  "documentCallbackUrl": "http://host.containers.internal:33679/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 2,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 129,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmv7yj7srxir7",
    "userid": "de10a11cec0010000000000000000000",
    "format": "pptx",
    "url": "http://host.containers.internal:33679/presentation.pptx",
    "title": "presentation.pptx",
    "nobase64": true,
    "outputformat": 8195,
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
  "time": 423,
  "supportAuthChangesAck": true
}
```

### 10:18:50.062   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "XgobXmsGNqRWwkJQAAB9",
  "sessionTimeConnect": 1791973129939,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "XgobXmsGNqRWwkJQAAB9",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [],
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd5ajdzcnhpcjciLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MzEyOTkzOX0sImlhdCI6MTc5MTk3MzEzMCwiZXhwIjoxNzk0NTY1MTMwfQ.1CVffCwCGTpY0K_lBuT_UoczNi6ivLYXUkurT60aY0E",
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
  "openedAt": 1791980330053
}
```

### 10:18:50.160   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7yj7srxir7/Editor.bin/Editor.bin?md5=mt-_YkHRXC6SG1cEnYJI-g&expires=1794567723&shardkey=recmv7yj7srxir7&filename=Editor.bin"
    },
    "openedAt": 1791980330053
  }
}
```

### 10:18:50.174   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:8"
}
```

### 10:18:50.264   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:89"
}
```

### 10:18:50.367   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:103"
}
```

### 10:18:50.669   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 10:18:50.669   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1032 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":86400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:18:50.669   <-  Alice           message

```json
{
  "type": "message"
}
```

### 10:18:51.698   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": ""
}
```

### 10:18:51.743   ->  Alice           getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "type": 1,
      "slideId": "2203",
      "objId": "2210",
      "guid": "2210"
    }
  ]
}
```

### 10:18:51.743   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "2210": {
      "time": 1791973131743,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "type": 1,
        "slideId": "2203",
        "objId": "2210",
        "guid": "2210"
      }
    }
  }
}
```

### 10:18:53.495   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 10:18:53.496   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:18:53.499   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAQAAAEwAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAAG8AAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGMAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGsAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAGQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAgAAACAAAAADAAAA\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGIAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAIAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAAHkAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAJAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAJAAAAAgAAACAAAAADAAAA\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAKAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAKAAAAAQAAAEEAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAALAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAALAAAAAQAAAGwAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAMAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAMAAAAAQAAAGkAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAANAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAANAAAAAQAAAGMAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAOAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAOAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAPAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAPAAAAAQAAAC4AAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAQAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAQAAAAAgAAACAAAAADAAAA\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":null}",
  "unlock": false,
  "releaseLocks": false
}
```

### 10:18:53.507   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791973133000,
  "syncChangesIndex": 35
}
```

### 10:18:53.590       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/presentationeditor/main/../../../../doc/recmv7yj7srxir7/c/?shardkey=recmv7yj7srxir7&EIO=4&transport=websocket"
```

### 10:18:53.592   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "tpMmZYDPXwlXJwEOAAB-",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 10:18:53.619   <-  Bob             license

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

### 10:18:53.687   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv7yj7srxir7",
  "documentCallbackUrl": "http://host.containers.internal:33679/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de10808c001000000000000000000000",
    "username": "Bob",
    "firstname": null,
    "lastname": null,
    "indexUser": -1
  },
  "editorType": 2,
  "lastOtherSaveTime": -1,
  "block": [],
  "sessionId": null,
  "sessionTimeConnect": null,
  "sessionTimeIdle": 0,
  "documentFormatSave": 129,
  "isCloseCoAuthoring": false,
  "openCmd": {
    "c": "open",
    "id": "recmv7yj7srxir7",
    "userid": "de10808c001000000000000000000000",
    "format": "pptx",
    "url": "http://host.containers.internal:33679/presentation.pptx",
    "title": "presentation.pptx",
    "nobase64": true,
    "outputformat": 8195,
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
  "time": 279,
  "supportAuthChangesAck": true
}
```

### 10:18:53.694   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "XgobXmsGNqRWwkJQAAB9",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 10:18:53.694   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791973133693,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "XgobXmsGNqRWwkJQAAB9",
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
      "connectionId": "CiC0vuBc1oDD4TDgAAB_",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 10:18:53.695   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7yj7srxir7/Editor.bin/Editor.bin?md5=mt-_YkHRXC6SG1cEnYJI-g&expires=1794567723&shardkey=recmv7yj7srxir7&filename=Editor.bin"
    },
    "openedAt": 1791980330053
  }
}
```

### 10:18:53.695   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": 35
}
```

### 10:18:53.702   <-  Bob             authChanges

```json
{
  "type": "authChanges",
  "changes": [
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"180;CAAAADIAMgAyADgAAgAcAAwAAAAAAAAAAQAAAFAAAAAAAAAAAAEAAAByAAAAAAAAAAABAAAAZQAAAAAAAAAAAQAAAHMAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbgAAAAAAAAAAAQAAAHQAAAAAAAAAAAEAAABhAAAAAAAAAAABAAAAdAAAAAAAAAAAAQAAAGkAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbgAAAAAAAAAA\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAAAAAAAQAAAEwAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAABAAAAAQAAAG8AAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAACAAAAAQAAAGMAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAADAAAAAQAAAGsAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAEAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAFAAAAAQAAAGQAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAGAAAAAgAAACAAAAADAAAA\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAHAAAAAQAAAGIAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAIAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAIAAAAAQAAAHkAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAJAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAJAAAAAgAAACAAAAADAAAA\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAKAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAKAAAAAQAAAEEAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAALAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAALAAAAAQAAAGwAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAMAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAMAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAANAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAANAAAAAQAAAGMAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAOAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAOAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAPAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAPAAAAAQAAAC4AAAAAAwAAAA==\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAQAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAQAAAAAgAAACAAAAADAAAA\"",
      "time": 1791973133000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 10:18:53.704   ->  Bob             authChangesAck

```json
{
  "type": "authChangesAck"
}
```

### 10:18:53.705   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:5"
}
```

### 10:18:53.803   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "CiC0vuBc1oDD4TDgAAB_",
  "sessionTimeConnect": 1791973133619,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "XgobXmsGNqRWwkJQAAB9",
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
      "connectionId": "CiC0vuBc1oDD4TDgAAB_",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": [
    {
      "time": 1791973131743,
      "user": "de10a11cec00100000000000000000001",
      "block": {
        "type": 1,
        "slideId": "2203",
        "objId": "2210",
        "guid": "2210"
      }
    }
  ],
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd5ajdzcnhpcjciLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5NzMxMzM2MTl9LCJpYXQiOjE3OTE5NzMxMzMsImV4cCI6MTc5NDU2NTEzM30.uguKvtPbsBfyYjq_mVTfdkjt4YlVEb9L1oz3eccRst8",
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

### 10:18:54.148   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:60"
}
```

### 10:18:54.204   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:55"
}
```

### 10:18:54.440   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:1"
}
```

### 10:18:54.440   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 10:18:54.440   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:999 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":86400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:18:54.440   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onApplyChanges time:208"
}
```

### 10:18:54.441   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:1032 memory:{\"totalJSHeapSize\":116000000,\"usedJSHeapSize\":86400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 10:18:54.441   <-  Bob             message

```json
{
  "type": "message"
}
```

### 10:18:55.667   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": ""
}
```

### 10:18:55.668   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "",
      "time": 1791973135668,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 10:18:59.131   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 35
}
```

### 10:18:59.132   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:18:59.133   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"89;CAAAADIAMgAyADgAAgAcAAUAAAAKAAAAAQAAAEEAAAAACgAAAAEAAABsAAAAAAoAAAABAAAAaQAAAAAKAAAAAQAAAGMAAAAACgAAAAEAAABlAAAAAAAAAAA=\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAKAAAAAgAAACAAAAADAAAA\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAALAAAAAQAAAEEAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAMAAAAAQAAAGwAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAANAAAAAQAAAGkAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAOAAAAAQAAAGMAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAPAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAQAAAAAgAAACAAAAADAAAA\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAARAAAAAQAAAHIAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAIAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAASAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAJAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAATAAAAAQAAAGwAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAKAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAUAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAALAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAVAAAAAQAAAGEAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAMAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAWAAAAAQAAAHMAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAANAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAXAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAOAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAYAAAAAQAAAHMAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAPAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAZAAAAAQAAAC4AAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAQAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAaAAAAAgAAACAAAAADAAAA\"]",
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

### 10:18:59.142   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"89;CAAAADIAMgAyADgAAgAcAAUAAAAKAAAAAQAAAEEAAAAACgAAAAEAAABsAAAAAAoAAAABAAAAaQAAAAAKAAAAAQAAAGMAAAAACgAAAAEAAABlAAAAAAAAAAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAKAAAAAgAAACAAAAADAAAA\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAALAAAAAQAAAEEAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAMAAAAAQAAAGwAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAANAAAAAQAAAGkAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAOAAAAAQAAAGMAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAPAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAQAAAAAgAAACAAAAADAAAA\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAARAAAAAQAAAHIAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAIAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAASAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAJAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAATAAAAAQAAAGwAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAKAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAUAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAALAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAVAAAAAQAAAGEAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAMAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAWAAAAAQAAAHMAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAANAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAXAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAOAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAYAAAAAQAAAHMAAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAPAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAZAAAAAQAAAC4AAAAAAwAAAA==\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAQAAAAAAAAABAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAaAAAAAgAAACAAAAADAAAA\"",
      "time": 1791973139000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 70,
  "syncChangesIndex": 70,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "type": 1,
        "slideId": "2203",
        "objId": "2210",
        "guid": "2210"
      },
      "user": "de10a11cec00100000000000000000001",
      "time": 1791973139142,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":null}"
}
```

### 10:18:59.143   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 35,
  "time": 1791973139000,
  "syncChangesIndex": 70
}
```

### 10:19:00.135   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 70
}
```

### 10:19:00.135   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:19:00.139   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": true,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 10:19:00.139   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": -1,
  "time": -1,
  "syncChangesIndex": -1
}
```

### 10:19:01.204   ->  Bob             getLock

```json
{
  "type": "getLock",
  "block": [
    {
      "type": 1,
      "slideId": "2203",
      "objId": "2210",
      "guid": "2210"
    }
  ]
}
```

### 10:19:01.205   <-  Bob             getLock

```json
{
  "type": "getLock",
  "locks": {
    "2210": {
      "time": 1791973141204,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "type": 1,
        "slideId": "2203",
        "objId": "2210",
        "guid": "2210"
      }
    }
  }
}
```

### 10:19:01.205   <-  Alice           getLock

```json
{
  "type": "getLock",
  "locks": {
    "2210": {
      "time": 1791973141204,
      "user": "de10808c0010000000000000000000002",
      "block": {
        "type": 1,
        "slideId": "2203",
        "objId": "2210",
        "guid": "2210"
      }
    }
  }
}
```

### 10:19:02.247   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 70
}
```

### 10:19:02.247   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 10:19:02.263   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"101;CAAAADIAMgAyADgAAgAcAAYAAAALAAAAAQAAAEEAAAAACwAAAAEAAABsAAAAAAsAAAABAAAAaQAAAAALAAAAAQAAAGMAAAAACwAAAAEAAABlAAAAAAsAAAACAAAAIAAAAAAAAAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAALAAAAAQAAAEIAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAMAAAAAQAAAG8AAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAANAAAAAQAAAGIAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAOAAAAAgAAACAAAAADAAAA\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAPAAAAAQAAAHQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAQAAAAAQAAAGEAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAARAAAAAQAAAGsAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAASAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAIAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAATAAAAAQAAAHMAAAAAAwAAAA==\",\"136;CAAAADIAMgAxADcAAQBWBAAAAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAEYcwEAAegDAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAQIAAAAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAJAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAUAAAAAgAAACAAAAADAAAA\",\"194;CAAAADIAMgAxADcAAQBWBAAAAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAGQXwEAAdAHAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAEYcwEAAegDAAA=\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAKAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAVAAAAAQAAAHQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAALAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAWAAAAAQAAAGgAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAMAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAXAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAANAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAYAAAAAgAAACAAAAADAAAA\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAOAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAZAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAPAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAaAAAAAQAAAGwAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAQAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAbAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAARAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAcAAAAAQAAAG0AAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAASAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAdAAAAAQAAAGUAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAATAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAeAAAAAQAAAG4AAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAUAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAfAAAAAQAAAHQAAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAVAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"37;CAAAADIAMgAyADgAAQAcAAEAAAAgAAAAAQAAAC4AAAAAAwAAAA==\",\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAWAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"36;CAAAADIAMgAyADgAAQAcAAEAAAAhAAAAAgAAACAAAAADAAAA\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":null}",
  "unlock": false,
  "releaseLocks": true
}
```

### 10:19:02.268   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAAAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"101;CAAAADIAMgAyADgAAgAcAAYAAAALAAAAAQAAAEEAAAAACwAAAAEAAABsAAAAAAsAAAABAAAAaQAAAAALAAAAAQAAAGMAAAAACwAAAAEAAABlAAAAAAsAAAACAAAAIAAAAAAAAAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAALAAAAAQAAAEIAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAABAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAMAAAAAQAAAG8AAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAACAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAANAAAAAQAAAGIAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAADAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAOAAAAAgAAACAAAAADAAAA\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAEAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAPAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAFAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAQAAAAAQAAAGEAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAGAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAARAAAAAQAAAGsAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAHAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAASAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAIAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAATAAAAAQAAAHMAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"136;CAAAADIAMgAxADcAAQBWBAAAAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAEYcwEAAegDAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAQIAAAAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAIAAAAJAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAUAAAAAgAAACAAAAADAAAA\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"194;CAAAADIAMgAxADcAAQBWBAAAAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAGQXwEAAdAHAAAAAQAAAAABAAEY8AEAAQABAAEAAQEAAAABMOADAAEBAAAAATDgAwAAAQABAAAAAAABGPABAAEAAQEAAAABAgAAAAEBAAAAAAEBAgAAAAEYcwEAAegDAAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAKAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAVAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAALAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAWAAAAAQAAAGgAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAMAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAXAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAANAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAYAAAAAgAAACAAAAADAAAA\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAOAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAZAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAPAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAaAAAAAQAAAGwAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAQAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAbAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAARAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAcAAAAAQAAAG0AAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAASAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAdAAAAAQAAAGUAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAATAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAeAAAAAQAAAG4AAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAUAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAfAAAAAQAAAHQAAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAVAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"37;CAAAADIAMgAyADgAAQAcAAEAAAAgAAAAAQAAAC4AAAAAAwAAAA==\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"68;AgAAADEA//8BAIg1qKnj9QEA2AAAAAEAAAAWAAAAAAAAABYAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv7yj7srxir7",
      "change": "\"36;CAAAADIAMgAyADgAAQAcAAEAAAAhAAAAAgAAACAAAAADAAAA\"",
      "time": 1791973142000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 119,
  "syncChangesIndex": 119,
  "endSaveChanges": true,
  "locks": [
    {
      "block": {
        "type": 1,
        "slideId": "2203",
        "objId": "2210",
        "guid": "2210"
      },
      "user": "de10808c0010000000000000000000002",
      "time": 1791973142268,
      "changes": null
    }
  ],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":null}"
}
```

### 10:19:02.268   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 70,
  "time": 1791973142000,
  "syncChangesIndex": 119
}
```

# Example OnlyOffice session

## Overview

This session has been captured using [oo-protocol-monitor/README.md], it consists of:

1. John creates the session
2. John modifies the document
3. John send a message in the chat
4. Kate joins the session
5. Kate modifies the document
6. Kate send a message in the chat

## Events

### 09:53:23.243       John Smith      ws-open

```json
"wss://site.docs.onlyoffice.com/9.4.1-e9f43897e5cfcbabaf9c2dac6f595fee/web-apps/apps/documenteditor/main/../../../../doc/34dcd6fd-4697-4319-94f5-059ffee038c4/c/?shardkey=34dcd6fd-4697-4319-94f5-059ffee038c4&EIO=4&transport=websocket"
```

### 09:53:23.728       John Smith      ws-open

```json
"wss://site.docs.onlyoffice.com/9.4.1-e9f43897e5cfcbabaf9c2dac6f595fee/web-apps/apps/documenteditor/main/../../../../doc/6e57f907-b4a9-44c8-bee6-ef81229f75ef/c/?shardkey=6e57f907-b4a9-44c8-bee6-ef81229f75ef&EIO=4&transport=websocket"
```

### 09:53:24.078   <-  John Smith      open

```json
{
  "eio": "open",
  "payload": {
    "sid": "ZhNlVDTH6M4Tl0C1AG1W",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:53:24.265   <-  John Smith      license

```json
{
  "type": "license",
  "payload": {
    "type": "license",
    "license": {
      "type": 3,
      "light": false,
      "mode": 0,
      "rights": 1,
      "buildVersion": "9.4.1",
      "buildNumber": 15,
      "protectionSupport": true,
      "isAnonymousSupport": true,
      "liveViewerSupport": true,
      "branding": true,
      "customization": true,
      "advancedApi": true
    }
  }
}
```

### 09:53:24.391   ->  John Smith      auth

```json
{
  "type": "auth",
  "payload": {
    "type": "auth",
    "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
    "token": "fghhfgsjdgfjs",
    "user": {
      "id": "78e1e841",
      "username": "John Smith",
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
      "id": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
      "userid": "78e1e841",
      "format": "docx",
      "url": "https://static.onlyoffice.com/assets/docs/samples/demo.docx",
      "title": "Example Document Title.docx",
      "lcid": 9,
      "nobase64": true,
      "outputformat": 8193,
      "convertToOrigin": ".pdf.xps.oxps.djvu"
    },
    "lang": "en",
    "permissions": {
      "edit": true,
      "review": true
    },
    "encrypted": false,
    "IsAnonymousUser": false,
    "timezoneOffset": -120,
    "headingsColor": null,
    "coEditingMode": "fast",
    "jwtOpen": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJmaWxlVHlwZSI6ImRvY3giLCJrZXkiOiI2ZTU3ZjkwNy1iNGE5LTQ0YzgtYmVlNi1lZjgxMjI5Zjc1ZWYiLCJ0aXRsZSI6IkV4YW1wbGUgRG9jdW1lbnQgVGl0bGUuZG9jeCIsInVybCI6Imh0dHBzOi8vc3RhdGljLm9ubHlvZmZpY2UuY29tL2Fzc2V0cy9kb2NzL3NhbXBsZXMvZGVtby5kb2N4IiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZSwicmV2aWV3Ijp0cnVlfX0sImRvY3VtZW50VHlwZSI6IndvcmQiLCJlZGl0b3JDb25maWciOnsibGFuZyI6ImVuIiwidXNlciI6eyJpZCI6Ijc4ZTFlODQxIiwibmFtZSI6IkpvaG4gU21pdGgifSwiY3VzdG9taXphdGlvbiI6eyJoaWRlUmlnaHRNZW51Ijp0cnVlLCJpbnRlZ3JhdGlvbk1vZGUiOiJlbWJlZCIsImFub255bW91cyI6eyJyZXF1ZXN0IjpmYWxzZX19LCJwbHVnaW5zIjp7InBsdWdpbnNEYXRhIjpbImh0dHBzOi8vb25seW9mZmljZS5jb20vcGx1Z2luLXJhaW5ib3cvY29uZmlnLmpzb24iXX19LCJ3aWR0aCI6IjEwMCUiLCJoZWlnaHQiOiIxMDAlIiwiaWF0IjoxNzg3NTY1MjAxfQ.y0Ouj5lucPCEwGVc72lFSZxW-6N0rJ24PSi51HHht0o",
    "time": 885,
    "supportAuthChangesAck": true
  }
}
```

### 09:53:24.561   <-  John Smith      auth

```json
{
  "type": "auth",
  "payload": {
    "type": "auth",
    "result": 1,
    "sessionId": "B5DBR2rgl8wxx1vBAG1X",
    "sessionTimeConnect": 1787565204160,
    "participants": [
      {
        "id": "78e1e8411",
        "idOriginal": "78e1e841",
        "username": "John Smith",
        "indexUser": 1,
        "view": false,
        "connectionId": "B5DBR2rgl8wxx1vBAG1X",
        "isCloseCoAuthoring": false,
        "isLiveViewer": false,
        "encrypted": false
      }
    ],
    "locks": {},
    "indexUser": 1,
    "hasForgotten": false,
    "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiI2ZTU3ZjkwNy1iNGE5LTQ0YzgtYmVlNi1lZjgxMjI5Zjc1ZWYiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlLCJyZXZpZXciOnRydWV9LCJkc19lbmNyeXB0ZWQiOmZhbHNlfSwiZWRpdG9yQ29uZmlnIjp7InVzZXIiOnsiaWQiOiI3OGUxZTg0MSIsIm5hbWUiOiJKb2huIFNtaXRoIiwiaW5kZXgiOjF9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3ODc1NjUyMDQxNjB9LCJpYXQiOjE3ODc1NjUyMDQsImV4cCI6MTc5MDE1NzIwNH0._FZBQkeBxGPn8h4K0xfzwQW6b3NHnIwgsw6aReLVMxw",
    "g_cAscSpellCheckUrl": "",
    "buildVersion": "9.4.1",
    "buildNumber": 15,
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
    "openedAt": 1787572404472
  }
}
```

### 09:53:25.004   <-  John Smith      documentOpen

```json
{
  "type": "documentOpen",
  "payload": {
    "type": "documentOpen",
    "data": {
      "type": "open",
      "status": "ok",
      "data": {
        "Editor.bin": "https://site.docs.onlyoffice.com/cache/files/9.4.1-15/data/site/6e57f907-b4a9-44c8-bee6-ef81229f75ef/Editor.bin/Editor.bin?md5=5Zt0Ecl58nD_tLfEjgAIcw&expires=1790159797&shardkey=6e57f907-b4a9-44c8-bee6-ef81229f75ef&filename=Editor.bin"
      },
      "openedAt": 1787572404472
    }
  }
}
```

### 09:53:25.460   ->  John Smith      clientLog

```json
{
  "type": "clientLog",
  "payload": {
    "type": "clientLog",
    "level": "debug",
    "msg": "onDownloadFile time:455"
  }
}
```

### 09:53:25.582   ->  John Smith      clientLog

```json
{
  "type": "clientLog",
  "payload": {
    "type": "clientLog",
    "level": "debug",
    "msg": "onOpenDocument time:122"
  }
}
```

### 09:53:25.685   ->  John Smith      clientLog

```json
{
  "type": "clientLog",
  "payload": {
    "type": "clientLog",
    "level": "debug",
    "msg": "onLoadFonts time:103"
  }
}
```

### 09:53:26.109   ->  John Smith      getMessages

```json
{
  "type": "getMessages",
  "payload": {
    "type": "getMessages"
  }
}
```

### 09:53:26.110   ->  John Smith      clientLog

```json
{
  "type": "clientLog",
  "payload": {
    "type": "clientLog",
    "level": "debug",
    "msg": "onDocumentContentReady time:2227 memory:{\"totalJSHeapSize\":143222092,\"usedJSHeapSize\":103422756,\"jsHeapSizeLimit\":4395630592}"
  }
}
```

### 09:53:26.295   <-  John Smith      message

```json
{
  "type": "message",
  "payload": {
    "type": "message"
  }
}
```

### 09:53:45.127   ->  John Smith      isSaveLock

```json
{
  "type": "isSaveLock",
  "payload": {
    "type": "isSaveLock",
    "syncChangesIndex": 0
  }
}
```

### 09:53:45.276   <-  John Smith      saveLock

```json
{
  "type": "saveLock",
  "payload": {
    "type": "saveLock",
    "saveLock": false
  }
}
```

### 09:53:45.277   ->  John Smith      saveChanges

```json
{
  "type": "saveChanges",
  "payload": {
    "type": "saveChanges",
    "changes": "[\"66;AgAAADEA//8BAJaq/QAwjg4AjgAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xAAAAA5AC4ANAAuADEALgAxADUA\",\"127;CAAAADEAMQAzADcAAgAcAAgAAAAAAAAAAQAAAFcAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbAAAAAAAAAAAAQAAAGMAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbQAAAAAAAAAAAQAAAGUAAAAAAAAAAAIAAAAgAAAAAAAAAA==\"]",
    "startSaveChanges": true,
    "endSaveChanges": true,
    "isCoAuthoring": false,
    "isExcel": false,
    "deleteIndex": null,
    "excelAdditionalInfo": "{\"UserId\":\"78e1e8411\",\"UserShortId\":\"78e1e841\",\"CursorInfo\":\"16;CAAAADEAMQAzADcAAAAAAA==\"}",
    "unlock": false,
    "releaseLocks": false
  }
}
```

### 09:53:45.434   <-  John Smith      unSaveLock

```json
{
  "type": "unSaveLock",
  "payload": {
    "type": "unSaveLock",
    "index": 0,
    "time": 1787565225000,
    "syncChangesIndex": 2
  }
}
```

### 09:53:57.413   ->  John Smith      message

```json
{
  "type": "message",
  "payload": {
    "type": "message",
    "message": "hello"
  }
}
```

### 09:53:57.559   <-  John Smith      message

```json
{
  "type": "message",
  "payload": {
    "type": "message",
    "messages": [
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "message": "hello",
        "time": 1787565237488,
        "user": "78e1e8411",
        "useridoriginal": "78e1e841",
        "username": "John Smith"
      }
    ]
  }
}
```

### 09:54:06.972       Kate Cage       ws-open

```json
"wss://site.docs.onlyoffice.com/9.4.1-e9f43897e5cfcbabaf9c2dac6f595fee/web-apps/apps/documenteditor/main/../../../../doc/6e57f907-b4a9-44c8-bee6-ef81229f75ef/c/?shardkey=6e57f907-b4a9-44c8-bee6-ef81229f75ef&EIO=4&transport=websocket"
```

### 09:54:07.438   <-  Kate Cage       open

```json
{
  "eio": "open",
  "payload": {
    "sid": "KPSdqCQv7WzC32m0ADUp",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:54:07.588   <-  Kate Cage       license

```json
{
  "type": "license",
  "payload": {
    "type": "license",
    "license": {
      "type": 3,
      "light": false,
      "mode": 0,
      "rights": 1,
      "buildVersion": "9.4.1",
      "buildNumber": 15,
      "protectionSupport": true,
      "isAnonymousSupport": true,
      "liveViewerSupport": true,
      "branding": true,
      "customization": true,
      "advancedApi": true
    }
  }
}
```

### 09:54:07.694   ->  Kate Cage       auth

```json
{
  "type": "auth",
  "payload": {
    "type": "auth",
    "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
    "token": "fghhfgsjdgfjs",
    "user": {
      "id": "F89d8069ba2b",
      "username": "Kate Cage",
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
      "id": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
      "userid": "F89d8069ba2b",
      "format": "docx",
      "url": "https://static.onlyoffice.com/assets/docs/samples/demo.docx",
      "title": "Example Document Title.docx",
      "lcid": 9,
      "nobase64": true,
      "outputformat": 8193,
      "convertToOrigin": ".pdf.xps.oxps.djvu"
    },
    "lang": "en",
    "permissions": {
      "edit": true,
      "review": true
    },
    "encrypted": false,
    "IsAnonymousUser": false,
    "timezoneOffset": -120,
    "headingsColor": null,
    "coEditingMode": "fast",
    "jwtOpen": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJmaWxlVHlwZSI6ImRvY3giLCJrZXkiOiI2ZTU3ZjkwNy1iNGE5LTQ0YzgtYmVlNi1lZjgxMjI5Zjc1ZWYiLCJ0aXRsZSI6IkV4YW1wbGUgRG9jdW1lbnQgVGl0bGUuZG9jeCIsInVybCI6Imh0dHBzOi8vc3RhdGljLm9ubHlvZmZpY2UuY29tL2Fzc2V0cy9kb2NzL3NhbXBsZXMvZGVtby5kb2N4IiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZSwicmV2aWV3Ijp0cnVlfX0sImRvY3VtZW50VHlwZSI6IndvcmQiLCJlZGl0b3JDb25maWciOnsibGFuZyI6ImVuIiwidXNlciI6eyJpZCI6IkY4OWQ4MDY5YmEyYiIsIm5hbWUiOiJLYXRlIENhZ2UifSwiY3VzdG9taXphdGlvbiI6eyJoaWRlUmlnaHRNZW51Ijp0cnVlLCJpbnRlZ3JhdGlvbk1vZGUiOiJlbWJlZCIsImFub255bW91cyI6eyJyZXF1ZXN0IjpmYWxzZX19LCJwbHVnaW5zIjp7InBsdWdpbnNEYXRhIjpbImh0dHBzOi8vb25seW9mZmljZS5jb20vcGx1Z2luLXJhaW5ib3cvY29uZmlnLmpzb24iXX19LCJ3aWR0aCI6IjEwMCUiLCJoZWlnaHQiOiIxMDAlIiwiaWF0IjoxNzg3NTY1MjAxfQ.pZxXArXKYkNiycjESffFyhf1PIL0n_I9UzJm-tKPsyk",
    "time": 959,
    "supportAuthChangesAck": true
  }
}
```

### 09:54:07.860   <-  Kate Cage       waitAuth

```json
{
  "type": "waitAuth",
  "payload": {
    "type": "waitAuth",
    "lockDocument": {
      "id": "78e1e8411",
      "idOriginal": "78e1e841",
      "username": "John Smith",
      "indexUser": 1,
      "view": false,
      "connectionId": "B5DBR2rgl8wxx1vBAG1X",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  }
}
```

### 09:54:07.862   <-  John Smith      connectState

```json
{
  "type": "connectState",
  "payload": {
    "type": "connectState",
    "participantsTimestamp": 1787565247782,
    "participants": [
      {
        "id": "78e1e8411",
        "idOriginal": "78e1e841",
        "username": "John Smith",
        "indexUser": 1,
        "view": false,
        "connectionId": "B5DBR2rgl8wxx1vBAG1X",
        "isCloseCoAuthoring": false,
        "isLiveViewer": false,
        "encrypted": false
      },
      {
        "id": "F89d8069ba2b2",
        "idOriginal": "F89d8069ba2b",
        "username": "Kate Cage",
        "indexUser": 2,
        "view": false,
        "connectionId": "OmdoXKlGvy7_gFB0ADUq",
        "isCloseCoAuthoring": false,
        "isLiveViewer": false,
        "encrypted": false
      }
    ],
    "waitAuth": true
  }
}
```

### 09:54:07.863   ->  John Smith      unLockDocument

```json
{
  "type": "unLockDocument",
  "payload": {
    "type": "unLockDocument",
    "isSave": false,
    "unlock": true,
    "deleteIndex": 2
  }
}
```

### 09:54:07.887   ->  John Smith      cursor

```json
{
  "type": "cursor",
  "payload": {
    "type": "cursor",
    "cursor": "16;CAAAADEAMQAzADcAAAAAAA=="
  }
}
```

### 09:54:07.889   <-  Kate Cage       documentOpen

```json
{
  "type": "documentOpen",
  "payload": {
    "type": "documentOpen",
    "data": {
      "type": "open",
      "status": "ok",
      "data": {
        "Editor.bin": "https://site.docs.onlyoffice.com/cache/files/9.4.1-15/data/site/6e57f907-b4a9-44c8-bee6-ef81229f75ef/Editor.bin/Editor.bin?md5=5Zt0Ecl58nD_tLfEjgAIcw&expires=1790159797&shardkey=6e57f907-b4a9-44c8-bee6-ef81229f75ef&filename=Editor.bin"
      },
      "openedAt": 1787572404472
    }
  }
}
```

### 09:54:08.022   <-  Kate Cage       authChanges

```json
{
  "type": "authChanges",
  "payload": {
    "type": "authChanges",
    "changes": [
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "change": "\"66;AgAAADEA//8BAJaq/QAwjg4AjgAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xAAAAA5AC4ANAAuADEALgAxADUA\"",
        "time": 1787565225000,
        "user": "78e1e8411",
        "useridoriginal": "78e1e841"
      },
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "change": "\"127;CAAAADEAMQAzADcAAgAcAAgAAAAAAAAAAQAAAFcAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbAAAAAAAAAAAAQAAAGMAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbQAAAAAAAAAAAQAAAGUAAAAAAAAAAAIAAAAgAAAAAAAAAA==\"",
        "time": 1787565225000,
        "user": "78e1e8411",
        "useridoriginal": "78e1e841"
      }
    ]
  }
}
```

### 09:54:08.023   ->  Kate Cage       authChangesAck

```json
{
  "type": "authChangesAck",
  "payload": {
    "type": "authChangesAck"
  }
}
```

### 09:54:08.038   <-  Kate Cage       cursor

```json
{
  "type": "cursor",
  "payload": {
    "type": "cursor",
    "messages": [
      {
        "cursor": "16;CAAAADEAMQAzADcAAAAAAA==",
        "time": 1787565247963,
        "user": "78e1e8411",
        "useridoriginal": "78e1e841"
      }
    ]
  }
}
```

### 09:54:08.058   ->  Kate Cage       clientLog

```json
{
  "type": "clientLog",
  "payload": {
    "type": "clientLog",
    "level": "debug",
    "msg": "onDownloadFile time:168"
  }
}
```

### 09:54:08.135   ->  Kate Cage       clientLog

```json
{
  "type": "clientLog",
  "payload": {
    "type": "clientLog",
    "level": "debug",
    "msg": "onOpenDocument time:77"
  }
}
```

### 09:54:08.185   ->  Kate Cage       clientLog

```json
{
  "type": "clientLog",
  "payload": {
    "type": "clientLog",
    "level": "debug",
    "msg": "onLoadFonts time:51"
  }
}
```

### 09:54:08.225   <-  Kate Cage       auth

```json
{
  "type": "auth",
  "payload": {
    "type": "auth",
    "result": 1,
    "sessionId": "OmdoXKlGvy7_gFB0ADUq",
    "sessionTimeConnect": 1787565247517,
    "participants": [
      {
        "id": "78e1e8411",
        "idOriginal": "78e1e841",
        "username": "John Smith",
        "indexUser": 1,
        "view": false,
        "connectionId": "B5DBR2rgl8wxx1vBAG1X",
        "isCloseCoAuthoring": false,
        "isLiveViewer": false,
        "encrypted": false
      },
      {
        "id": "F89d8069ba2b2",
        "idOriginal": "F89d8069ba2b",
        "username": "Kate Cage",
        "indexUser": 2,
        "view": false,
        "connectionId": "OmdoXKlGvy7_gFB0ADUq",
        "isCloseCoAuthoring": false,
        "isLiveViewer": false,
        "encrypted": false
      }
    ],
    "messages": [
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "message": "hello",
        "time": 1787565237488,
        "user": "78e1e8411",
        "useridoriginal": "78e1e841",
        "username": "John Smith"
      }
    ],
    "locks": {},
    "indexUser": 2,
    "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiI2ZTU3ZjkwNy1iNGE5LTQ0YzgtYmVlNi1lZjgxMjI5Zjc1ZWYiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlLCJyZXZpZXciOnRydWV9LCJkc19lbmNyeXB0ZWQiOmZhbHNlfSwiZWRpdG9yQ29uZmlnIjp7InVzZXIiOnsiaWQiOiJGODlkODA2OWJhMmIiLCJuYW1lIjoiS2F0ZSBDYWdlIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3ODc1NjUyNDc1MTd9LCJpYXQiOjE3ODc1NjUyNDgsImV4cCI6MTc5MDE1NzI0OH0.9c4i9aeAuEtZ_zKCv4Omx_oeb0DPWtX1gRQLSs47bcw",
    "g_cAscSpellCheckUrl": "",
    "buildVersion": "9.4.1",
    "buildNumber": 15,
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
}
```

### 09:54:08.231   ->  Kate Cage       clientLog

```json
{
  "type": "clientLog",
  "payload": {
    "type": "clientLog",
    "level": "debug",
    "msg": "onApplyChanges time:3"
  }
}
```

### 09:54:08.543   ->  Kate Cage       getMessages

```json
{
  "type": "getMessages",
  "payload": {
    "type": "getMessages"
  }
}
```

### 09:54:08.543   ->  Kate Cage       clientLog

```json
{
  "type": "clientLog",
  "payload": {
    "type": "clientLog",
    "level": "debug",
    "msg": "onDocumentContentReady time:1545 memory:{\"totalJSHeapSize\":197559455,\"usedJSHeapSize\":164139795,\"jsHeapSizeLimit\":4395630592}"
  }
}
```

### 09:54:08.613   ->  Kate Cage       cursor

```json
{
  "type": "cursor",
  "payload": {
    "type": "cursor",
    "cursor": "16;CAAAADEAMQAzADcAAAAAAA=="
  }
}
```

### 09:54:08.726   <-  Kate Cage       message

```json
{
  "type": "message",
  "payload": {
    "type": "message",
    "messages": [
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "message": "hello",
        "time": 1787565237488,
        "user": "78e1e8411",
        "useridoriginal": "78e1e841",
        "username": "John Smith"
      }
    ]
  }
}
```

### 09:54:08.798   <-  John Smith      cursor

```json
{
  "type": "cursor",
  "payload": {
    "type": "cursor",
    "messages": [
      {
        "cursor": "16;CAAAADEAMQAzADcAAAAAAA==",
        "time": 1787565248691,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b"
      }
    ]
  }
}
```

### 09:54:13.474   ->  Kate Cage       cursor

```json
{
  "type": "cursor",
  "payload": {
    "type": "cursor",
    "cursor": "16;CAAAADEAMQAzADcAAAAAAA=="
  }
}
```

### 09:54:13.625   <-  John Smith      cursor

```json
{
  "type": "cursor",
  "payload": {
    "type": "cursor",
    "messages": [
      {
        "cursor": "16;CAAAADEAMQAzADcAAAAAAA==",
        "time": 1787565253551,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b"
      }
    ]
  }
}
```

### 09:54:14.512   ->  Kate Cage       cursor

```json
{
  "type": "cursor",
  "payload": {
    "type": "cursor",
    "cursor": "16;CAAAADEAMQAzADcAAAAAAA=="
  }
}
```

### 09:54:14.661   <-  John Smith      cursor

```json
{
  "type": "cursor",
  "payload": {
    "type": "cursor",
    "messages": [
      {
        "cursor": "16;CAAAADEAMQAzADcAAAAAAA==",
        "time": 1787565254587,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b"
      }
    ]
  }
}
```

### 09:54:20.353   ->  Kate Cage       isSaveLock

```json
{
  "type": "isSaveLock",
  "payload": {
    "type": "isSaveLock",
    "syncChangesIndex": 2
  }
}
```

### 09:54:20.500   <-  Kate Cage       saveLock

```json
{
  "type": "saveLock",
  "payload": {
    "type": "saveLock",
    "saveLock": false
  }
}
```

### 09:54:20.501   ->  Kate Cage       saveChanges

```json
{
  "type": "saveChanges",
  "payload": {
    "type": "saveChanges",
    "changes": "[\"66;AgAAADEA//8BAJaq/QAwjg4ApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xAAAAA5AC4ANAAuADEALgAxADUA\",\"37;CAAAADEAMQAzADcAAQAcAAEAAAAAAAAAAQAAAEYAAAAAAwAAAA==\"]",
    "startSaveChanges": true,
    "endSaveChanges": true,
    "isCoAuthoring": true,
    "isExcel": false,
    "deleteIndex": null,
    "excelAdditionalInfo": "{\"UserId\":\"F89d8069ba2b2\",\"UserShortId\":\"F89d8069ba2b\",\"CursorInfo\":\"16;CAAAADEAMQAzADcAAQAAAA==\"}",
    "unlock": false,
    "releaseLocks": true
  }
}
```

### 09:54:20.654   <-  Kate Cage       unSaveLock

```json
{
  "type": "unSaveLock",
  "payload": {
    "type": "unSaveLock",
    "index": 2,
    "time": 1787565260000,
    "syncChangesIndex": 4
  }
}
```

### 09:54:20.656   <-  John Smith      saveChanges

```json
{
  "type": "saveChanges",
  "payload": {
    "type": "saveChanges",
    "changes": [
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "change": "\"66;AgAAADEA//8BAJaq/QAwjg4ApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xAAAAA5AC4ANAAuADEALgAxADUA\"",
        "time": 1787565260000,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b"
      },
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "change": "\"37;CAAAADEAMQAzADcAAQAcAAEAAAAAAAAAAQAAAEYAAAAAAwAAAA==\"",
        "time": 1787565260000,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b"
      }
    ],
    "changesIndex": 4,
    "syncChangesIndex": 4,
    "endSaveChanges": true,
    "locks": [],
    "excelAdditionalInfo": "{\"UserId\":\"F89d8069ba2b2\",\"UserShortId\":\"F89d8069ba2b\",\"CursorInfo\":\"16;CAAAADEAMQAzADcAAQAAAA==\"}"
  }
}
```

### 09:54:20.672   ->  Kate Cage       isSaveLock

```json
{
  "type": "isSaveLock",
  "payload": {
    "type": "isSaveLock",
    "syncChangesIndex": 4
  }
}
```

### 09:54:20.698   ->  John Smith      unLockDocument

```json
{
  "type": "unLockDocument",
  "payload": {
    "type": "unLockDocument",
    "isSave": false,
    "unlock": false,
    "deleteIndex": null,
    "releaseLocks": true
  }
}
```

### 09:54:20.820   <-  Kate Cage       saveLock

```json
{
  "type": "saveLock",
  "payload": {
    "type": "saveLock",
    "saveLock": false
  }
}
```

### 09:54:20.820   ->  Kate Cage       saveChanges

```json
{
  "type": "saveChanges",
  "payload": {
    "type": "saveChanges",
    "changes": "[\"66;AgAAADEA//8BAJaq/QAwjg4ALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xAAAAA5AC4ANAAuADEALgAxADUA\",\"37;CAAAADEAMQAzADcAAQAcAAEAAAABAAAAAQAAAG8AAAAAAwAAAA==\",\"37;CAAAADEAMQAzADcAAQAcAAEAAAACAAAAAQAAAG8AAAAAAwAAAA==\"]",
    "startSaveChanges": true,
    "endSaveChanges": true,
    "isCoAuthoring": true,
    "isExcel": false,
    "deleteIndex": null,
    "excelAdditionalInfo": "{\"UserId\":\"F89d8069ba2b2\",\"UserShortId\":\"F89d8069ba2b\",\"CursorInfo\":\"16;CAAAADEAMQAzADcAAwAAAA==\"}",
    "unlock": false,
    "releaseLocks": true
  }
}
```

### 09:54:20.974   <-  Kate Cage       unSaveLock

```json
{
  "type": "unSaveLock",
  "payload": {
    "type": "unSaveLock",
    "index": 4,
    "time": 1787565260000,
    "syncChangesIndex": 7
  }
}
```

### 09:54:20.976   <-  John Smith      saveChanges

```json
{
  "type": "saveChanges",
  "payload": {
    "type": "saveChanges",
    "changes": [
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "change": "\"66;AgAAADEA//8BAJaq/QAwjg4ALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xAAAAA5AC4ANAAuADEALgAxADUA\"",
        "time": 1787565260000,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b"
      },
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "change": "\"37;CAAAADEAMQAzADcAAQAcAAEAAAABAAAAAQAAAG8AAAAAAwAAAA==\"",
        "time": 1787565260000,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b"
      },
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "change": "\"37;CAAAADEAMQAzADcAAQAcAAEAAAACAAAAAQAAAG8AAAAAAwAAAA==\"",
        "time": 1787565260000,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b"
      }
    ],
    "changesIndex": 7,
    "syncChangesIndex": 7,
    "endSaveChanges": true,
    "locks": [],
    "excelAdditionalInfo": "{\"UserId\":\"F89d8069ba2b2\",\"UserShortId\":\"F89d8069ba2b\",\"CursorInfo\":\"16;CAAAADEAMQAzADcAAwAAAA==\"}"
  }
}
```

### 09:54:20.992   ->  Kate Cage       isSaveLock

```json
{
  "type": "isSaveLock",
  "payload": {
    "type": "isSaveLock",
    "syncChangesIndex": 7
  }
}
```

### 09:54:21.012   ->  John Smith      unLockDocument

```json
{
  "type": "unLockDocument",
  "payload": {
    "type": "unLockDocument",
    "isSave": false,
    "unlock": false,
    "deleteIndex": null,
    "releaseLocks": true
  }
}
```

### 09:54:21.140   <-  Kate Cage       saveLock

```json
{
  "type": "saveLock",
  "payload": {
    "type": "saveLock",
    "saveLock": false
  }
}
```

### 09:54:21.140   ->  Kate Cage       saveChanges

```json
{
  "type": "saveChanges",
  "payload": {
    "type": "saveChanges",
    "changes": "[\"66;AgAAADEA//8BAJaq/QAwjg4ApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xAAAAA5AC4ANAAuADEALgAxADUA\",\"36;CAAAADEAMQAzADcAAQAcAAEAAAADAAAAAgAAACAAAAADAAAA\"]",
    "startSaveChanges": true,
    "endSaveChanges": true,
    "isCoAuthoring": true,
    "isExcel": false,
    "deleteIndex": null,
    "excelAdditionalInfo": "{\"UserId\":\"F89d8069ba2b2\",\"UserShortId\":\"F89d8069ba2b\",\"CursorInfo\":\"16;CAAAADEAMQAzADcABAAAAA==\"}",
    "unlock": false,
    "releaseLocks": true
  }
}
```

### 09:54:21.294   <-  Kate Cage       unSaveLock

```json
{
  "type": "unSaveLock",
  "payload": {
    "type": "unSaveLock",
    "index": 7,
    "time": 1787565261000,
    "syncChangesIndex": 9
  }
}
```

### 09:54:21.298   <-  John Smith      saveChanges

```json
{
  "type": "saveChanges",
  "payload": {
    "type": "saveChanges",
    "changes": [
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "change": "\"66;AgAAADEA//8BAJaq/QAwjg4ApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xAAAAA5AC4ANAAuADEALgAxADUA\"",
        "time": 1787565261000,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b"
      },
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "change": "\"36;CAAAADEAMQAzADcAAQAcAAEAAAADAAAAAgAAACAAAAADAAAA\"",
        "time": 1787565261000,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b"
      }
    ],
    "changesIndex": 9,
    "syncChangesIndex": 9,
    "endSaveChanges": true,
    "locks": [],
    "excelAdditionalInfo": "{\"UserId\":\"F89d8069ba2b2\",\"UserShortId\":\"F89d8069ba2b\",\"CursorInfo\":\"16;CAAAADEAMQAzADcABAAAAA==\"}"
  }
}
```

### 09:54:21.330   ->  John Smith      unLockDocument

```json
{
  "type": "unLockDocument",
  "payload": {
    "type": "unLockDocument",
    "isSave": false,
    "unlock": false,
    "deleteIndex": null,
    "releaseLocks": true
  }
}
```

### 09:54:34.049   ->  Kate Cage       message

```json
{
  "type": "message",
  "payload": {
    "type": "message",
    "message": "2nd message"
  }
}
```

### 09:54:34.198   <-  Kate Cage       message

```json
{
  "type": "message",
  "payload": {
    "type": "message",
    "messages": [
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "message": "2nd message",
        "time": 1787565274124,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b",
        "username": "Kate Cage"
      }
    ]
  }
}
```

### 09:54:34.201   <-  John Smith      message

```json
{
  "type": "message",
  "payload": {
    "type": "message",
    "messages": [
      {
        "docid": "6e57f907-b4a9-44c8-bee6-ef81229f75ef",
        "message": "2nd message",
        "time": 1787565274124,
        "user": "F89d8069ba2b2",
        "useridoriginal": "F89d8069ba2b",
        "username": "Kate Cage"
      }
    ]
  }
}
```

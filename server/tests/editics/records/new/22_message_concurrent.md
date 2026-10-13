## Info

- Timestamp: 2026-10-13T15:25:14.777Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario: Alice and Bob send chat messages at the same time (manual-flow holds both sends until both are out).

## Events

### 15:25:04.033       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u16pszarn/c/?shardkey=recmv6u16pszarn&EIO=4&transport=websocket"
```

### 15:25:04.034   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "-rB9OfwmkJTFIX0yAAFu",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:25:04.037   <-  Alice           license

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

### 15:25:04.186   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6u16pszarn",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
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
    "id": "recmv6u16pszarn",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33041/document.docx",
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
  "time": 422,
  "supportAuthChangesAck": true
}
```

### 15:25:04.202   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "ppB-QFlR9fOOAgDsAAFv",
  "sessionTimeConnect": 1791905104037,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ppB-QFlR9fOOAgDsAAFv",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MTZwc3phcm4iLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTEwNDAzN30sImlhdCI6MTc5MTkwNTEwNCwiZXhwIjoxNzk0NDk3MTA0fQ.QuIT9Nsc7rxNRjAA-yMqC0HKbohjBAP-1YFXP9BMiho",
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
  "openedAt": 1791912304193
}
```

### 15:25:04.267   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u16pszarn/Editor.bin/Editor.bin?md5=cyPvOvNTdycRlvv3BjzGOQ&expires=1794499697&shardkey=recmv6u16pszarn&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u16pszarn/media/image1.png/image1.png?md5=2UR8kyOAoQJtBu5-UXDQ9Q&expires=1794499697&shardkey=recmv6u16pszarn&filename=image1.png"
    },
    "openedAt": 1791912304193
  }
}
```

### 15:25:04.315   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:10"
}
```

### 15:25:04.401   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:85"
}
```

### 15:25:04.502   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 15:25:04.704   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:25:04.704   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:773 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:25:04.705   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:25:05.870       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u16pszarn/c/?shardkey=recmv6u16pszarn&EIO=4&transport=websocket"
```

### 15:25:05.871   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "dhYco8D4BvfMKbxZAAFw",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:25:05.903   <-  Bob             license

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

### 15:25:06.018   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6u16pszarn",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
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
    "id": "recmv6u16pszarn",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33041/document.docx",
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
  "time": 295,
  "supportAuthChangesAck": true
}
```

### 15:25:06.027   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "ppB-QFlR9fOOAgDsAAFv",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:25:06.027   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905106025,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ppB-QFlR9fOOAgDsAAFv",
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
      "connectionId": "Z9K1VoBe8CfxS58kAAFx",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:25:06.028   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u16pszarn/Editor.bin/Editor.bin?md5=cyPvOvNTdycRlvv3BjzGOQ&expires=1794499697&shardkey=recmv6u16pszarn&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u16pszarn/media/image1.png/image1.png?md5=2UR8kyOAoQJtBu5-UXDQ9Q&expires=1794499697&shardkey=recmv6u16pszarn&filename=image1.png"
    },
    "openedAt": 1791912304193
  }
}
```

### 15:25:06.409   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:25:06.415   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:25:06.415   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905106415,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:25:06.416   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "Z9K1VoBe8CfxS58kAAFx",
  "sessionTimeConnect": 1791905105902,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ppB-QFlR9fOOAgDsAAFv",
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
      "connectionId": "Z9K1VoBe8CfxS58kAAFx",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MTZwc3phcm4iLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUxMDU5MDJ9LCJpYXQiOjE3OTE5MDUxMDYsImV4cCI6MTc5NDQ5NzEwNn0.Jv2trFzS9V5lo1rI8FHAXCo1hUtFYkUac88WJ5EQoSQ",
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

### 15:25:06.430   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:9"
}
```

### 15:25:06.473   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:42"
}
```

### 15:25:06.580   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:108"
}
```

### 15:25:06.724   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:25:06.724   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:885 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:25:06.724   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:25:06.757   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:25:06.758   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905106757,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:25:07.851       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u16pszarn/c/?shardkey=recmv6u16pszarn&EIO=4&transport=websocket"
```

### 15:25:07.852   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "iQ-8buWl6wqPmVK9AAFy",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:25:07.887   <-  Mallory         license

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

### 15:25:07.995   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6u16pszarn",
  "documentCallbackUrl": "http://host.containers.internal:33041/callback",
  "token": "fghhfgsjdgfjs",
  "user": {
    "id": "de103a11031c00100000000000000000",
    "username": "Mallory",
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
    "id": "recmv6u16pszarn",
    "userid": "de103a11031c00100000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33041/document.docx",
    "title": "document.docx",
    "lcid": 9,
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
  "time": 286,
  "supportAuthChangesAck": true
}
```

### 15:25:08.009   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "j27H0RqWZxhR23ooAAFz",
  "sessionTimeConnect": 1791905107887,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ppB-QFlR9fOOAgDsAAFv",
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
      "connectionId": "Z9K1VoBe8CfxS58kAAFx",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de103a11031c001000000000000000003",
      "idOriginal": "de103a11031c00100000000000000000",
      "username": "Mallory",
      "indexUser": 3,
      "view": false,
      "connectionId": "j27H0RqWZxhR23ooAAFz",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MTZwc3phcm4iLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA1MTA3ODg3fSwiaWF0IjoxNzkxOTA1MTA4LCJleHAiOjE3OTQ0OTcxMDh9.cYwCdkNblJ_sS7du-_FjpubfdAWvlc3_Ycxm9iN57jg",
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
  "openedAt": 1791912304193
}
```

### 15:25:08.009   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905108002,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ppB-QFlR9fOOAgDsAAFv",
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
      "connectionId": "Z9K1VoBe8CfxS58kAAFx",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de103a11031c001000000000000000003",
      "idOriginal": "de103a11031c00100000000000000000",
      "username": "Mallory",
      "indexUser": 3,
      "view": false,
      "connectionId": "j27H0RqWZxhR23ooAAFz",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:25:08.009   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905108002,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "ppB-QFlR9fOOAgDsAAFv",
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
      "connectionId": "Z9K1VoBe8CfxS58kAAFx",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    },
    {
      "id": "de103a11031c001000000000000000003",
      "idOriginal": "de103a11031c00100000000000000000",
      "username": "Mallory",
      "indexUser": 3,
      "view": false,
      "connectionId": "j27H0RqWZxhR23ooAAFz",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:25:08.010   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u16pszarn/Editor.bin/Editor.bin?md5=cyPvOvNTdycRlvv3BjzGOQ&expires=1794499697&shardkey=recmv6u16pszarn&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u16pszarn/media/image1.png/image1.png?md5=2UR8kyOAoQJtBu5-UXDQ9Q&expires=1794499697&shardkey=recmv6u16pszarn&filename=image1.png"
    },
    "openedAt": 1791912304193
  }
}
```

### 15:25:08.036   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:8"
}
```

### 15:25:08.089   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:52"
}
```

### 15:25:08.194   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:106"
}
```

### 15:25:08.330   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:25:08.330   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:511 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:25:08.330   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:25:08.364   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:25:08.364   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905108364,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:25:08.364   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905108364,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:25:10.153   ->  Alice           message

```json
{
  "type": "message",
  "message": "Concurrent hello from Alice"
}
```

### 15:25:10.203   ->  Bob             message

```json
{
  "type": "message",
  "message": "Concurrent hello from Bob"
}
```

### 15:25:10.254   <-  Alice           message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u16pszarn",
      "message": "Concurrent hello from Alice",
      "time": 1791905110153,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 15:25:10.304   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u16pszarn",
      "message": "Concurrent hello from Alice",
      "time": 1791905110153,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 15:25:10.355   <-  Mallory         message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u16pszarn",
      "message": "Concurrent hello from Alice",
      "time": 1791905110153,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 15:25:10.405   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u16pszarn",
      "message": "Concurrent hello from Bob",
      "time": 1791905110204,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000",
      "username": "Bob"
    }
  ]
}
```

### 15:25:10.457   <-  Alice           message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u16pszarn",
      "message": "Concurrent hello from Bob",
      "time": 1791905110204,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000",
      "username": "Bob"
    }
  ]
}
```

### 15:25:10.507   <-  Mallory         message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u16pszarn",
      "message": "Concurrent hello from Bob",
      "time": 1791905110204,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000",
      "username": "Bob"
    }
  ]
}
```

## Info

- Timestamp: 2026-10-13T15:25:24.800Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario: Alice sends a chat message while Bob is authenticating.

## Events

### 15:25:16.919       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u1gn4ot3b/c/?shardkey=recmv6u1gn4ot3b&EIO=4&transport=websocket"
```

### 15:25:16.921   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "D6Syg7AGHk532WDWAAF0",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:25:16.934   <-  Alice           license

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

### 15:25:17.101   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6u1gn4ot3b",
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
    "id": "recmv6u1gn4ot3b",
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
  "time": 472,
  "supportAuthChangesAck": true
}
```

### 15:25:17.116   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "x8zm8e4yzTtntjsHAAF1",
  "sessionTimeConnect": 1791905116934,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "x8zm8e4yzTtntjsHAAF1",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MWduNG90M2IiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTExNjkzNH0sImlhdCI6MTc5MTkwNTExNywiZXhwIjoxNzk0NDk3MTE3fQ.zLY7nWjQTooGGD-kAp-Cs1qHVIb2ZIfysOz7xuEJcmA",
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
  "openedAt": 1791912317108
}
```

### 15:25:17.178   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u1gn4ot3b/Editor.bin/Editor.bin?md5=h35YNUxtFdHj6hzdaip6Lw&expires=1794499710&shardkey=recmv6u1gn4ot3b&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u1gn4ot3b/media/image1.png/image1.png?md5=uCRhloWVsFfY04Ns6stSKA&expires=1794499710&shardkey=recmv6u1gn4ot3b&filename=image1.png"
    },
    "openedAt": 1791912317108
  }
}
```

### 15:25:17.221   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:7"
}
```

### 15:25:17.307   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:84"
}
```

### 15:25:17.406   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 15:25:17.611   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:25:17.612   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:814 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:25:17.612   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:25:19.144       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u1gn4ot3b/c/?shardkey=recmv6u1gn4ot3b&EIO=4&transport=websocket"
```

### 15:25:19.219   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "Q_trAiaxXmmzfzWTAAF2",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:25:19.279   <-  Bob             license

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

### 15:25:19.399   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6u1gn4ot3b",
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
    "id": "recmv6u1gn4ot3b",
    "userid": "de10808c001000000000000000000000",
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
  "time": 733,
  "supportAuthChangesAck": true
}
```

### 15:25:19.420   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905119406,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "x8zm8e4yzTtntjsHAAF1",
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
      "connectionId": "MPdWuotS-GzE5aDzAAF3",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:25:19.440   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:25:19.460   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:25:20.434   ->  Alice           message

```json
{
  "type": "message",
  "message": "Hello Bob, from mid-auth Alice"
}
```

### 15:25:20.455   <-  Alice           message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u1gn4ot3b",
      "message": "Hello Bob, from mid-auth Alice",
      "time": 1791905120435,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 15:25:21.130   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "x8zm8e4yzTtntjsHAAF1",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:25:21.180   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u1gn4ot3b/Editor.bin/Editor.bin?md5=h35YNUxtFdHj6hzdaip6Lw&expires=1794499710&shardkey=recmv6u1gn4ot3b&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u1gn4ot3b/media/image1.png/image1.png?md5=uCRhloWVsFfY04Ns6stSKA&expires=1794499710&shardkey=recmv6u1gn4ot3b&filename=image1.png"
    },
    "openedAt": 1791912317108
  }
}
```

### 15:25:21.230   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "MPdWuotS-GzE5aDzAAF3",
  "sessionTimeConnect": 1791905119239,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "x8zm8e4yzTtntjsHAAF1",
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
      "connectionId": "MPdWuotS-GzE5aDzAAF3",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MWduNG90M2IiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUxMTkyMzl9LCJpYXQiOjE3OTE5MDUxMTksImV4cCI6MTc5NDQ5NzExOX0.DlTE4OXlabILSFejemrxtbAtujxx6lKk167Drrufd3U",
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

### 15:25:21.280   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905119460,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:25:21.330   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u1gn4ot3b",
      "message": "Hello Bob, from mid-auth Alice",
      "time": 1791905120435,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 15:25:21.380   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 15:25:21.430   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:42"
}
```

### 15:25:21.481   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:105"
}
```

### 15:25:21.531   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:25:21.581   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:2746 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:25:21.631   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:25:21.683   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv6u1gn4ot3b",
      "message": "Hello Bob, from mid-auth Alice",
      "time": 1791905120435,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 15:25:21.732   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905121632,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

## Info

- Timestamp: 2026-10-13T15:25:57.705Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice edits the document.
  2. Bob edits the document.
  3. Mallory edits the document.

## Events

### 15:25:38.185       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u1x1v1bbi/c/?shardkey=recmv6u1x1v1bbi&EIO=4&transport=websocket"
```

### 15:25:38.186   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "lD-2W9Rd1l4tJAYGAAF6",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:25:38.199   <-  Alice           license

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

### 15:25:38.384   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv6u1x1v1bbi",
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
    "id": "recmv6u1x1v1bbi",
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
  "time": 488,
  "supportAuthChangesAck": true
}
```

### 15:25:38.401   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "afvM4l73Ih5feLatAAF7",
  "sessionTimeConnect": 1791905138199,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "afvM4l73Ih5feLatAAF7",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MXgxdjFiYmkiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTkwNTEzODE5OX0sImlhdCI6MTc5MTkwNTEzOCwiZXhwIjoxNzk0NDk3MTM4fQ.mG--c7YQqVYgvXlUybAoVWf4EQfMI0L8tBQ8fqvouyg",
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
  "openedAt": 1791912338392
}
```

### 15:25:38.456   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u1x1v1bbi/Editor.bin/Editor.bin?md5=E8hAtpRtjE7eTb0EmSNL-A&expires=1794499731&shardkey=recmv6u1x1v1bbi&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u1x1v1bbi/media/image1.png/image1.png?md5=R3sShFr1w0v3kXgND1VRrA&expires=1794499731&shardkey=recmv6u1x1v1bbi&filename=image1.png"
    },
    "openedAt": 1791912338392
  }
}
```

### 15:25:38.482   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:6"
}
```

### 15:25:38.565   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:82"
}
```

### 15:25:38.665   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 15:25:38.869   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 15:25:38.869   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:804 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:25:38.869   <-  Alice           message

```json
{
  "type": "message"
}
```

### 15:25:40.037       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u1x1v1bbi/c/?shardkey=recmv6u1x1v1bbi&EIO=4&transport=websocket"
```

### 15:25:40.039   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "Ug4n6kvLPtkCRn7xAAF8",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:25:40.066   <-  Bob             license

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

### 15:25:40.166   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv6u1x1v1bbi",
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
    "id": "recmv6u1x1v1bbi",
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
  "time": 300,
  "supportAuthChangesAck": true
}
```

### 15:25:40.174   <-  Bob             waitAuth

```json
{
  "type": "waitAuth",
  "lockDocument": {
    "id": "de10a11cec00100000000000000000001",
    "idOriginal": "de10a11cec0010000000000000000000",
    "username": "Alice",
    "indexUser": 1,
    "view": false,
    "connectionId": "afvM4l73Ih5feLatAAF7",
    "isCloseCoAuthoring": false,
    "isLiveViewer": false,
    "encrypted": false
  }
}
```

### 15:25:40.174   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905140173,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "afvM4l73Ih5feLatAAF7",
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
      "connectionId": "5rXuAZI3kMWtA4kSAAF9",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": true
}
```

### 15:25:40.176   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u1x1v1bbi/Editor.bin/Editor.bin?md5=E8hAtpRtjE7eTb0EmSNL-A&expires=1794499731&shardkey=recmv6u1x1v1bbi&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u1x1v1bbi/media/image1.png/image1.png?md5=R3sShFr1w0v3kXgND1VRrA&expires=1794499731&shardkey=recmv6u1x1v1bbi&filename=image1.png"
    },
    "openedAt": 1791912338392
  }
}
```

### 15:25:40.176   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": true,
  "deleteIndex": null
}
```

### 15:25:40.183   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "5rXuAZI3kMWtA4kSAAF9",
  "sessionTimeConnect": 1791905140065,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "afvM4l73Ih5feLatAAF7",
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
      "connectionId": "5rXuAZI3kMWtA4kSAAF9",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MXgxdjFiYmkiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDgwOGMwMDEwMDAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQm9iIiwiaW5kZXgiOjJ9LCJkc19pc0Nsb3NlQ29BdXRob3JpbmciOmZhbHNlLCJkc19zZXNzaW9uVGltZUNvbm5lY3QiOjE3OTE5MDUxNDAwNjV9LCJpYXQiOjE3OTE5MDUxNDAsImV4cCI6MTc5NDQ5NzE0MH0.YCzmL8NizujJd17W4Ll0gAqxY2eyqxIv9JLsNKBZytM",
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

### 15:25:40.183   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:1"
}
```

### 15:25:40.188   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:25:40.188   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905140188,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:25:40.603   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:42"
}
```

### 15:25:40.708   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:105"
}
```

### 15:25:40.844   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 15:25:40.844   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:869 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:25:40.844   <-  Bob             message

```json
{
  "type": "message"
}
```

### 15:25:40.877   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:25:40.878   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905140878,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:25:41.977       Mallory         [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8888/9.3.0-1e034f22cadb5da6153c560a091113fa/web-apps/apps/documenteditor/main/../../../../doc/recmv6u1x1v1bbi/c/?shardkey=recmv6u1x1v1bbi&EIO=4&transport=websocket"
```

### 15:25:41.978   <-  Mallory         [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "YdnMdWJOUqeaNdU9AAF-",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 15:25:42.008   <-  Mallory         license

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

### 15:25:42.126   ->  Mallory         auth

```json
{
  "type": "auth",
  "docid": "recmv6u1x1v1bbi",
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
    "id": "recmv6u1x1v1bbi",
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
  "time": 281,
  "supportAuthChangesAck": true
}
```

### 15:25:42.139   <-  Mallory         auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "ZM5GI923yBRJJckJAAF_",
  "sessionTimeConnect": 1791905142008,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "afvM4l73Ih5feLatAAF7",
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
      "connectionId": "5rXuAZI3kMWtA4kSAAF9",
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
      "connectionId": "ZM5GI923yBRJJckJAAF_",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 3,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjZ1MXgxdjFiYmkiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMDNhMTEwMzFjMDAxMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiTWFsbG9yeSIsImluZGV4IjozfSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxOTA1MTQyMDA4fSwiaWF0IjoxNzkxOTA1MTQyLCJleHAiOjE3OTQ0OTcxNDJ9.UrCd2Few64ckwXuOMV6zPq9uZh-jF0_PiWqLIfl5KeU",
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
  "openedAt": 1791912338392
}
```

### 15:25:42.140   <-  Bob             connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905142132,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "afvM4l73Ih5feLatAAF7",
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
      "connectionId": "5rXuAZI3kMWtA4kSAAF9",
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
      "connectionId": "ZM5GI923yBRJJckJAAF_",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:25:42.140   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791905142132,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "afvM4l73Ih5feLatAAF7",
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
      "connectionId": "5rXuAZI3kMWtA4kSAAF9",
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
      "connectionId": "ZM5GI923yBRJJckJAAF_",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 15:25:42.140   <-  Mallory         documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8888/cache/files/data/recmv6u1x1v1bbi/Editor.bin/Editor.bin?md5=E8hAtpRtjE7eTb0EmSNL-A&expires=1794499731&shardkey=recmv6u1x1v1bbi&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8888/cache/files/data/recmv6u1x1v1bbi/media/image1.png/image1.png?md5=R3sShFr1w0v3kXgND1VRrA&expires=1794499731&shardkey=recmv6u1x1v1bbi&filename=image1.png"
    },
    "openedAt": 1791912338392
  }
}
```

### 15:25:42.168   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:8"
}
```

### 15:25:42.231   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:63"
}
```

### 15:25:42.336   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:105"
}
```

### 15:25:42.473   ->  Mallory         getMessages

```json
{
  "type": "getMessages"
}
```

### 15:25:42.474   ->  Mallory         clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:518 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 15:25:42.474   <-  Mallory         message

```json
{
  "type": "message"
}
```

### 15:25:42.508   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAAAAAA="
}
```

### 15:25:42.509   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905142509,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:25:42.509   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAAAAAA=",
      "time": 1791905142509,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:25:43.506   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAIAAAA="
}
```

### 15:25:43.507   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905143507,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:25:43.507   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905143507,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:25:43.986   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 15:25:43.987   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:43.988   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:43.995   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905143000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1791905143000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 2,
  "syncChangesIndex": 2,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}"
}
```

### 15:25:43.995   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905143000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1791905143000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 2,
  "syncChangesIndex": 2,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}"
}
```

### 15:25:43.995   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791905143000,
  "syncChangesIndex": 2
}
```

### 15:25:44.011   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.021   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.032   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 2
}
```

### 15:25:44.033   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.036   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.040   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 4,
  "syncChangesIndex": 4,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}"
}
```

### 15:25:44.040   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 4,
  "syncChangesIndex": 4,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}"
}
```

### 15:25:44.040   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 2,
  "time": 1791905144000,
  "syncChangesIndex": 4
}
```

### 15:25:44.068   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 4
}
```

### 15:25:44.069   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.072   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.075   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 6,
  "syncChangesIndex": 6,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}"
}
```

### 15:25:44.076   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 4,
  "time": 1791905144000,
  "syncChangesIndex": 6
}
```

### 15:25:44.076   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 6,
  "syncChangesIndex": 6,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}"
}
```

### 15:25:44.110   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 6
}
```

### 15:25:44.111   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.112   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.116   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 8,
  "syncChangesIndex": 8,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}"
}
```

### 15:25:44.116   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 6,
  "time": 1791905144000,
  "syncChangesIndex": 8
}
```

### 15:25:44.116   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 8,
  "syncChangesIndex": 8,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}"
}
```

### 15:25:44.157   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 8
}
```

### 15:25:44.158   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.158   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.163   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 10,
  "syncChangesIndex": 10,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}"
}
```

### 15:25:44.163   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 10,
  "syncChangesIndex": 10,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}"
}
```

### 15:25:44.163   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 8,
  "time": 1791905144000,
  "syncChangesIndex": 10
}
```

### 15:25:44.208   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 10
}
```

### 15:25:44.209   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.221   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.225   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 12,
  "syncChangesIndex": 12,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}"
}
```

### 15:25:44.225   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 12,
  "syncChangesIndex": 12,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}"
}
```

### 15:25:44.225   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 10,
  "time": 1791905144000,
  "syncChangesIndex": 12
}
```

### 15:25:44.257   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 12
}
```

### 15:25:44.258   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.267   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAkAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.271   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 14,
  "syncChangesIndex": 14,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAkAAAA=\"}"
}
```

### 15:25:44.271   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 14,
  "syncChangesIndex": 14,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAkAAAA=\"}"
}
```

### 15:25:44.271   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 12,
  "time": 1791905144000,
  "syncChangesIndex": 14
}
```

### 15:25:44.307   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 14
}
```

### 15:25:44.308   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.309   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.313   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 16,
  "syncChangesIndex": 16,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}"
}
```

### 15:25:44.313   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 16,
  "syncChangesIndex": 16,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}"
}
```

### 15:25:44.313   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 14,
  "time": 1791905144000,
  "syncChangesIndex": 16
}
```

### 15:25:44.357   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 16
}
```

### 15:25:44.358   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.360   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.362   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAsAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.366   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 18,
  "syncChangesIndex": 18,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAsAAAA=\"}"
}
```

### 15:25:44.366   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 18,
  "syncChangesIndex": 18,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAsAAAA=\"}"
}
```

### 15:25:44.366   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 16,
  "time": 1791905144000,
  "syncChangesIndex": 18
}
```

### 15:25:44.422   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 18
}
```

### 15:25:44.422   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.424   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.429   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.433   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 20,
  "syncChangesIndex": 20,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}"
}
```

### 15:25:44.433   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 20,
  "syncChangesIndex": 20,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}"
}
```

### 15:25:44.433   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 18,
  "time": 1791905144000,
  "syncChangesIndex": 20
}
```

### 15:25:44.477   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.477   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 20
}
```

### 15:25:44.478   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.480   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.481   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.485   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 22,
  "syncChangesIndex": 22,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}"
}
```

### 15:25:44.485   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 22,
  "syncChangesIndex": 22,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}"
}
```

### 15:25:44.485   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 20,
  "time": 1791905144000,
  "syncChangesIndex": 22
}
```

### 15:25:44.499   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.540   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 22
}
```

### 15:25:44.541   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.543   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.545   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.548   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 24,
  "syncChangesIndex": 24,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}"
}
```

### 15:25:44.548   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 24,
  "syncChangesIndex": 24,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}"
}
```

### 15:25:44.548   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 22,
  "time": 1791905144000,
  "syncChangesIndex": 24
}
```

### 15:25:44.552   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.579   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.586   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 24
}
```

### 15:25:44.587   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.587   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.591   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 26,
  "syncChangesIndex": 26,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}"
}
```

### 15:25:44.591   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 26,
  "syncChangesIndex": 26,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}"
}
```

### 15:25:44.591   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 24,
  "time": 1791905144000,
  "syncChangesIndex": 26
}
```

### 15:25:44.598   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.627   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.631   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 26
}
```

### 15:25:44.631   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.632   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.636   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 28,
  "syncChangesIndex": 28,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}"
}
```

### 15:25:44.636   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 28,
  "syncChangesIndex": 28,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}"
}
```

### 15:25:44.636   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 26,
  "time": 1791905144000,
  "syncChangesIndex": 28
}
```

### 15:25:44.665   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.674   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 28
}
```

### 15:25:44.675   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.676   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.678   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.681   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 30,
  "syncChangesIndex": 30,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}"
}
```

### 15:25:44.681   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 30,
  "syncChangesIndex": 30,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}"
}
```

### 15:25:44.682   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 28,
  "time": 1791905144000,
  "syncChangesIndex": 30
}
```

### 15:25:44.699   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.725   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 30
}
```

### 15:25:44.725   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.727   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.729   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.732   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 32,
  "syncChangesIndex": 32,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}"
}
```

### 15:25:44.733   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 32,
  "syncChangesIndex": 32,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}"
}
```

### 15:25:44.733   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 30,
  "time": 1791905144000,
  "syncChangesIndex": 32
}
```

### 15:25:44.739   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.751   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.786   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 32
}
```

### 15:25:44.787   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.787   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABMAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.791   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 34,
  "syncChangesIndex": 34,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABMAAAA=\"}"
}
```

### 15:25:44.791   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 34,
  "syncChangesIndex": 34,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABMAAAA=\"}"
}
```

### 15:25:44.791   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 32,
  "time": 1791905144000,
  "syncChangesIndex": 34
}
```

### 15:25:44.798   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.826   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.830   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 34
}
```

### 15:25:44.830   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:44.831   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:44.834   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 36,
  "syncChangesIndex": 36,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}"
}
```

### 15:25:44.834   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905144000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 36,
  "syncChangesIndex": 36,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}"
}
```

### 15:25:44.835   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 34,
  "time": 1791905144000,
  "syncChangesIndex": 36
}
```

### 15:25:44.873   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:44.876   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:45.630   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADYAOAAyAAAAAAA="
}
```

### 15:25:45.630   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADYAOAAyAAAAAAA=",
      "time": 1791905145630,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:25:45.630   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADYAOAAyAAAAAAA=",
      "time": 1791905145630,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:25:45.960   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 36
}
```

### 15:25:45.961   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:45.962   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:45.968   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905145000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\"",
      "time": 1791905145000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 38,
  "syncChangesIndex": 38,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAQAAAA=\"}"
}
```

### 15:25:45.968   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905145000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\"",
      "time": 1791905145000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 38,
  "syncChangesIndex": 38,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAQAAAA=\"}"
}
```

### 15:25:45.968   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 36,
  "time": 1791905145000,
  "syncChangesIndex": 38
}
```

### 15:25:45.985   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.009   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 38
}
```

### 15:25:46.009   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.012   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.014   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAUAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.018   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 40,
  "syncChangesIndex": 40,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAUAAAA=\"}"
}
```

### 15:25:46.018   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 40,
  "syncChangesIndex": 40,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAUAAAA=\"}"
}
```

### 15:25:46.018   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 38,
  "time": 1791905146000,
  "syncChangesIndex": 40
}
```

### 15:25:46.047   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 40
}
```

### 15:25:46.048   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.050   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.052   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAYAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.055   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 42,
  "syncChangesIndex": 42,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAYAAAA=\"}"
}
```

### 15:25:46.056   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 42,
  "syncChangesIndex": 42,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAYAAAA=\"}"
}
```

### 15:25:46.056   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 40,
  "time": 1791905146000,
  "syncChangesIndex": 42
}
```

### 15:25:46.070   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.100   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.115   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 42
}
```

### 15:25:46.116   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.116   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAcAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.120   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 44,
  "syncChangesIndex": 44,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAcAAAA=\"}"
}
```

### 15:25:46.120   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 44,
  "syncChangesIndex": 44,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAcAAAA=\"}"
}
```

### 15:25:46.120   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 42,
  "time": 1791905146000,
  "syncChangesIndex": 44
}
```

### 15:25:46.161   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.162   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 44
}
```

### 15:25:46.162   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.164   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.165   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAgAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.169   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 46,
  "syncChangesIndex": 46,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAgAAAA=\"}"
}
```

### 15:25:46.169   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 46,
  "syncChangesIndex": 46,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAgAAAA=\"}"
}
```

### 15:25:46.169   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 44,
  "time": 1791905146000,
  "syncChangesIndex": 46
}
```

### 15:25:46.209   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.210   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 46
}
```

### 15:25:46.210   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.212   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.214   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAkAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.217   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 48,
  "syncChangesIndex": 48,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAkAAAA=\"}"
}
```

### 15:25:46.217   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 48,
  "syncChangesIndex": 48,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAkAAAA=\"}"
}
```

### 15:25:46.217   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 46,
  "time": 1791905146000,
  "syncChangesIndex": 48
}
```

### 15:25:46.243   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.261   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.266   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 48
}
```

### 15:25:46.266   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.267   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.271   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 50,
  "syncChangesIndex": 50,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}"
}
```

### 15:25:46.271   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 50,
  "syncChangesIndex": 50,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}"
}
```

### 15:25:46.271   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 48,
  "time": 1791905146000,
  "syncChangesIndex": 50
}
```

### 15:25:46.319   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.320   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 50
}
```

### 15:25:46.320   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.322   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.342   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAsAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.347   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 52,
  "syncChangesIndex": 52,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAsAAAA=\"}"
}
```

### 15:25:46.347   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 52,
  "syncChangesIndex": 52,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAsAAAA=\"}"
}
```

### 15:25:46.347   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 50,
  "time": 1791905146000,
  "syncChangesIndex": 52
}
```

### 15:25:46.368   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 52
}
```

### 15:25:46.368   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.370   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.373   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.377   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 54,
  "syncChangesIndex": 54,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}"
}
```

### 15:25:46.377   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 54,
  "syncChangesIndex": 54,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}"
}
```

### 15:25:46.377   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 52,
  "time": 1791905146000,
  "syncChangesIndex": 54
}
```

### 15:25:46.419   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.420   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 54
}
```

### 15:25:46.420   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.423   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.426   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA0AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.430   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 56,
  "syncChangesIndex": 56,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA0AAAA=\"}"
}
```

### 15:25:46.430   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 56,
  "syncChangesIndex": 56,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA0AAAA=\"}"
}
```

### 15:25:46.430   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 54,
  "time": 1791905146000,
  "syncChangesIndex": 56
}
```

### 15:25:46.467   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 56
}
```

### 15:25:46.468   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.469   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.475   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.476   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA4AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.480   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 58,
  "syncChangesIndex": 58,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA4AAAA=\"}"
}
```

### 15:25:46.480   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 58,
  "syncChangesIndex": 58,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA4AAAA=\"}"
}
```

### 15:25:46.480   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 56,
  "time": 1791905146000,
  "syncChangesIndex": 58
}
```

### 15:25:46.523   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.524   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 58
}
```

### 15:25:46.524   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.526   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.547   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.551   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 60,
  "syncChangesIndex": 60,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}"
}
```

### 15:25:46.552   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 58,
  "time": 1791905146000,
  "syncChangesIndex": 60
}
```

### 15:25:46.552   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 60,
  "syncChangesIndex": 60,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}"
}
```

### 15:25:46.570   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.610   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.610   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 60
}
```

### 15:25:46.611   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.612   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABEAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.615   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 63,
  "syncChangesIndex": 63,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABEAAAA=\"}"
}
```

### 15:25:46.615   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 63,
  "syncChangesIndex": 63,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABEAAAA=\"}"
}
```

### 15:25:46.616   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 60,
  "time": 1791905146000,
  "syncChangesIndex": 63
}
```

### 15:25:46.620   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.638   ->  Alice           cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADYAOAAyABEAAAA="
}
```

### 15:25:46.639   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADYAOAAyABEAAAA=",
      "time": 1791905146639,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:25:46.639   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADYAOAAyABEAAAA=",
      "time": 1791905146639,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ]
}
```

### 15:25:46.641   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.666   ->  Alice           isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 63
}
```

### 15:25:46.667   <-  Alice           saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:46.667   ->  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:46.671   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 65,
  "syncChangesIndex": 65,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}"
}
```

### 15:25:46.671   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905146000,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000"
    }
  ],
  "changesIndex": 65,
  "syncChangesIndex": 65,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10a11cec00100000000000000000001\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}"
}
```

### 15:25:46.671   <-  Alice           unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 63,
  "time": 1791905146000,
  "syncChangesIndex": 65
}
```

### 15:25:46.710   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:46.717   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:47.474   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAIAAAA="
}
```

### 15:25:47.475   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905147474,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:25:47.475   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905147474,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:25:47.949   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 65
}
```

### 15:25:47.950   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:47.951   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:47.957   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905147000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1791905147000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 67,
  "syncChangesIndex": 67,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}"
}
```

### 15:25:47.958   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905147000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1791905147000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 67,
  "syncChangesIndex": 67,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}"
}
```

### 15:25:47.958   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 65,
  "time": 1791905147000,
  "syncChangesIndex": 67
}
```

### 15:25:47.980   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:47.989   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.001   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 67
}
```

### 15:25:48.002   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.003   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.007   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 69,
  "syncChangesIndex": 69,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}"
}
```

### 15:25:48.007   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 69,
  "syncChangesIndex": 69,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}"
}
```

### 15:25:48.007   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 67,
  "time": 1791905148000,
  "syncChangesIndex": 69
}
```

### 15:25:48.044   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.045   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 69
}
```

### 15:25:48.045   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.047   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.050   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 71,
  "syncChangesIndex": 71,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}"
}
```

### 15:25:48.051   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 71,
  "syncChangesIndex": 71,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}"
}
```

### 15:25:48.051   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 69,
  "time": 1791905148000,
  "syncChangesIndex": 71
}
```

### 15:25:48.092   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.093   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 71
}
```

### 15:25:48.093   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.095   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.098   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 73,
  "syncChangesIndex": 73,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}"
}
```

### 15:25:48.098   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 73,
  "syncChangesIndex": 73,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}"
}
```

### 15:25:48.098   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 71,
  "time": 1791905148000,
  "syncChangesIndex": 73
}
```

### 15:25:48.140   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 73
}
```

### 15:25:48.141   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.142   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.144   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.148   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 75,
  "syncChangesIndex": 75,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}"
}
```

### 15:25:48.148   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 75,
  "syncChangesIndex": 75,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}"
}
```

### 15:25:48.148   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 73,
  "time": 1791905148000,
  "syncChangesIndex": 75
}
```

### 15:25:48.179   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.199   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 75
}
```

### 15:25:48.200   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.211   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.215   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 77,
  "syncChangesIndex": 77,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}"
}
```

### 15:25:48.215   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 77,
  "syncChangesIndex": 77,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}"
}
```

### 15:25:48.215   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 75,
  "time": 1791905148000,
  "syncChangesIndex": 77
}
```

### 15:25:48.235   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.237   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 77
}
```

### 15:25:48.237   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.250   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAkAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.254   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 79,
  "syncChangesIndex": 79,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAkAAAA=\"}"
}
```

### 15:25:48.254   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 79,
  "syncChangesIndex": 79,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAkAAAA=\"}"
}
```

### 15:25:48.254   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 77,
  "time": 1791905148000,
  "syncChangesIndex": 79
}
```

### 15:25:48.302   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.304   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.305   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 79
}
```

### 15:25:48.305   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.319   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.323   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 81,
  "syncChangesIndex": 81,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}"
}
```

### 15:25:48.323   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 81,
  "syncChangesIndex": 81,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}"
}
```

### 15:25:48.323   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 79,
  "time": 1791905148000,
  "syncChangesIndex": 81
}
```

### 15:25:48.361   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.364   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.365   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 81
}
```

### 15:25:48.365   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.366   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAsAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.369   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 83,
  "syncChangesIndex": 83,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAsAAAA=\"}"
}
```

### 15:25:48.369   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 83,
  "syncChangesIndex": 83,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAsAAAA=\"}"
}
```

### 15:25:48.369   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 81,
  "time": 1791905148000,
  "syncChangesIndex": 83
}
```

### 15:25:48.408   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.411   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.411   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 83
}
```

### 15:25:48.412   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.413   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.416   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 85,
  "syncChangesIndex": 85,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}"
}
```

### 15:25:48.416   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 85,
  "syncChangesIndex": 85,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}"
}
```

### 15:25:48.416   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 83,
  "time": 1791905148000,
  "syncChangesIndex": 85
}
```

### 15:25:48.420   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.460   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.460   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 85
}
```

### 15:25:48.461   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.461   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.465   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 87,
  "syncChangesIndex": 87,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}"
}
```

### 15:25:48.465   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 87,
  "syncChangesIndex": 87,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}"
}
```

### 15:25:48.465   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 85,
  "time": 1791905148000,
  "syncChangesIndex": 87
}
```

### 15:25:48.469   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.499   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.514   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 87
}
```

### 15:25:48.514   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.515   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.518   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 89,
  "syncChangesIndex": 89,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}"
}
```

### 15:25:48.519   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 89,
  "syncChangesIndex": 89,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}"
}
```

### 15:25:48.519   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 87,
  "time": 1791905148000,
  "syncChangesIndex": 89
}
```

### 15:25:48.542   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.548   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.552   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 89
}
```

### 15:25:48.552   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.553   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.557   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 91,
  "syncChangesIndex": 91,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}"
}
```

### 15:25:48.557   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 91,
  "syncChangesIndex": 91,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}"
}
```

### 15:25:48.557   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 89,
  "time": 1791905148000,
  "syncChangesIndex": 91
}
```

### 15:25:48.589   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.608   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.610   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 91
}
```

### 15:25:48.610   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.611   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.615   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 93,
  "syncChangesIndex": 93,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}"
}
```

### 15:25:48.615   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 93,
  "syncChangesIndex": 93,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}"
}
```

### 15:25:48.615   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 91,
  "time": 1791905148000,
  "syncChangesIndex": 93
}
```

### 15:25:48.619   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.642   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.656   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 93
}
```

### 15:25:48.656   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.657   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.661   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 95,
  "syncChangesIndex": 95,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}"
}
```

### 15:25:48.661   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 95,
  "syncChangesIndex": 95,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}"
}
```

### 15:25:48.661   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 93,
  "time": 1791905148000,
  "syncChangesIndex": 95
}
```

### 15:25:48.668   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.706   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 95
}
```

### 15:25:48.706   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.708   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.710   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.713   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 97,
  "syncChangesIndex": 97,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}"
}
```

### 15:25:48.714   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 97,
  "syncChangesIndex": 97,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}"
}
```

### 15:25:48.714   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 95,
  "time": 1791905148000,
  "syncChangesIndex": 97
}
```

### 15:25:48.740   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.759   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.760   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 97
}
```

### 15:25:48.760   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.761   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABMAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.764   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 99,
  "syncChangesIndex": 99,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABMAAAA=\"}"
}
```

### 15:25:48.764   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 99,
  "syncChangesIndex": 99,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABMAAAA=\"}"
}
```

### 15:25:48.765   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 97,
  "time": 1791905148000,
  "syncChangesIndex": 99
}
```

### 15:25:48.793   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.808   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.809   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 99
}
```

### 15:25:48.809   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:48.810   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:48.814   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 101,
  "syncChangesIndex": 101,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}"
}
```

### 15:25:48.814   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905148000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 101,
  "syncChangesIndex": 101,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}"
}
```

### 15:25:48.814   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 99,
  "time": 1791905148000,
  "syncChangesIndex": 101
}
```

### 15:25:48.819   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:48.842   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:49.659   ->  Bob             cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADYAOAAyAAAAAAA="
}
```

### 15:25:49.660   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADYAOAAyAAAAAAA=",
      "time": 1791905149660,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:25:49.660   <-  Mallory         cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADYAOAAyAAAAAAA=",
      "time": 1791905149660,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ]
}
```

### 15:25:49.994   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 101
}
```

### 15:25:49.995   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:49.996   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.002   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905149000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\"",
      "time": 1791905149000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 103,
  "syncChangesIndex": 103,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAQAAAA=\"}"
}
```

### 15:25:50.003   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905149000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\"",
      "time": 1791905149000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 103,
  "syncChangesIndex": 103,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAQAAAA=\"}"
}
```

### 15:25:50.003   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 101,
  "time": 1791905149000,
  "syncChangesIndex": 103
}
```

### 15:25:50.020   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.053   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.054   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 103
}
```

### 15:25:50.054   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.065   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAUAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.069   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 105,
  "syncChangesIndex": 105,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAUAAAA=\"}"
}
```

### 15:25:50.069   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 105,
  "syncChangesIndex": 105,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAUAAAA=\"}"
}
```

### 15:25:50.069   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 103,
  "time": 1791905150000,
  "syncChangesIndex": 105
}
```

### 15:25:50.083   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 105
}
```

### 15:25:50.083   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.085   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAYAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.088   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 107,
  "syncChangesIndex": 107,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAYAAAA=\"}"
}
```

### 15:25:50.088   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 107,
  "syncChangesIndex": 107,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAYAAAA=\"}"
}
```

### 15:25:50.088   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 105,
  "time": 1791905150000,
  "syncChangesIndex": 107
}
```

### 15:25:50.148   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.152   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.153   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 107
}
```

### 15:25:50.153   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.155   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAcAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.158   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 109,
  "syncChangesIndex": 109,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAcAAAA=\"}"
}
```

### 15:25:50.159   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 109,
  "syncChangesIndex": 109,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAcAAAA=\"}"
}
```

### 15:25:50.159   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 107,
  "time": 1791905150000,
  "syncChangesIndex": 109
}
```

### 15:25:50.188   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.228   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.229   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 109
}
```

### 15:25:50.229   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.230   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAkAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.234   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 112,
  "syncChangesIndex": 112,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAkAAAA=\"}"
}
```

### 15:25:50.234   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 112,
  "syncChangesIndex": 112,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAkAAAA=\"}"
}
```

### 15:25:50.234   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 109,
  "time": 1791905150000,
  "syncChangesIndex": 112
}
```

### 15:25:50.260   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.278   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.278   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 112
}
```

### 15:25:50.279   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.279   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.283   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 114,
  "syncChangesIndex": 114,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}"
}
```

### 15:25:50.283   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 114,
  "syncChangesIndex": 114,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}"
}
```

### 15:25:50.283   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 112,
  "time": 1791905150000,
  "syncChangesIndex": 114
}
```

### 15:25:50.314   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.326   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.327   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 114
}
```

### 15:25:50.327   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.328   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAsAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.332   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 116,
  "syncChangesIndex": 116,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAsAAAA=\"}"
}
```

### 15:25:50.332   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 116,
  "syncChangesIndex": 116,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAsAAAA=\"}"
}
```

### 15:25:50.332   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 114,
  "time": 1791905150000,
  "syncChangesIndex": 116
}
```

### 15:25:50.362   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.377   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.377   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 116
}
```

### 15:25:50.378   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.379   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.382   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 118,
  "syncChangesIndex": 118,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}"
}
```

### 15:25:50.382   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 116,
  "time": 1791905150000,
  "syncChangesIndex": 118
}
```

### 15:25:50.382   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 118,
  "syncChangesIndex": 118,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}"
}
```

### 15:25:50.389   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.431   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 118
}
```

### 15:25:50.432   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.434   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.436   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA0AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.439   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 120,
  "syncChangesIndex": 120,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA0AAAA=\"}"
}
```

### 15:25:50.439   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 120,
  "syncChangesIndex": 120,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA0AAAA=\"}"
}
```

### 15:25:50.440   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 118,
  "time": 1791905150000,
  "syncChangesIndex": 120
}
```

### 15:25:50.477   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.481   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.481   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 120
}
```

### 15:25:50.482   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.483   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA4AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.486   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 122,
  "syncChangesIndex": 122,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA4AAAA=\"}"
}
```

### 15:25:50.486   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 122,
  "syncChangesIndex": 122,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA4AAAA=\"}"
}
```

### 15:25:50.487   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 120,
  "time": 1791905150000,
  "syncChangesIndex": 122
}
```

### 15:25:50.514   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.535   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.535   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 122
}
```

### 15:25:50.536   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.537   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.541   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 124,
  "syncChangesIndex": 124,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}"
}
```

### 15:25:50.541   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 124,
  "syncChangesIndex": 124,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}"
}
```

### 15:25:50.541   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 122,
  "time": 1791905150000,
  "syncChangesIndex": 124
}
```

### 15:25:50.585   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.585   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 124
}
```

### 15:25:50.586   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.588   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABAAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.591   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 126,
  "syncChangesIndex": 126,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABAAAAA=\"}"
}
```

### 15:25:50.591   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 126,
  "syncChangesIndex": 126,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABAAAAA=\"}"
}
```

### 15:25:50.591   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 124,
  "time": 1791905150000,
  "syncChangesIndex": 126
}
```

### 15:25:50.636   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.677   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.677   ->  Bob             isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 126
}
```

### 15:25:50.678   <-  Bob             saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:50.679   ->  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:50.692   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 130,
  "syncChangesIndex": 130,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}"
}
```

### 15:25:50.693   <-  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905150000,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000"
    }
  ],
  "changesIndex": 130,
  "syncChangesIndex": 130,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de10808c0010000000000000000000002\",\"UserShortId\":\"de10808c001000000000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}"
}
```

### 15:25:50.693   <-  Bob             unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 126,
  "time": 1791905150000,
  "syncChangesIndex": 130
}
```

### 15:25:50.741   ->  Mallory         unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:50.749   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:51.497   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADcAMAA3AAIAAAA="
}
```

### 15:25:51.498   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905151498,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:25:51.498   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADcAMAA3AAIAAAA=",
      "time": 1791905151498,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:25:51.940   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 130
}
```

### 15:25:51.941   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:51.942   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:51.948   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905151000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1791905151000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 132,
  "syncChangesIndex": 132,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}"
}
```

### 15:25:51.949   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905151000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAgAAAAEAAABIAAAAAAMAAAA=\"",
      "time": 1791905151000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 132,
  "syncChangesIndex": 132,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAMAAAA=\"}"
}
```

### 15:25:51.949   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 130,
  "time": 1791905151000,
  "syncChangesIndex": 132
}
```

### 15:25:51.958   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:51.996   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 132
}
```

### 15:25:51.996   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.002   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.005   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.009   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 134,
  "syncChangesIndex": 134,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}"
}
```

### 15:25:52.009   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAAwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 134,
  "syncChangesIndex": 134,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAQAAAA=\"}"
}
```

### 15:25:52.009   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 132,
  "time": 1791905152000,
  "syncChangesIndex": 134
}
```

### 15:25:52.064   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.066   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.067   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 134
}
```

### 15:25:52.067   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.068   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.072   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 136,
  "syncChangesIndex": 136,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}"
}
```

### 15:25:52.072   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABAAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 136,
  "syncChangesIndex": 136,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAUAAAA=\"}"
}
```

### 15:25:52.072   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 134,
  "time": 1791905152000,
  "syncChangesIndex": 136
}
```

### 15:25:52.111   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 136
}
```

### 15:25:52.112   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.114   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.117   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.118   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.122   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 138,
  "syncChangesIndex": 138,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}"
}
```

### 15:25:52.122   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABQAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 138,
  "syncChangesIndex": 138,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAYAAAA=\"}"
}
```

### 15:25:52.122   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 136,
  "time": 1791905152000,
  "syncChangesIndex": 138
}
```

### 15:25:52.161   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 138
}
```

### 15:25:52.162   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.164   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.166   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.168   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.172   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 140,
  "syncChangesIndex": 140,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}"
}
```

### 15:25:52.172   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAABgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 140,
  "syncChangesIndex": 140,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAcAAAA=\"}"
}
```

### 15:25:52.173   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 138,
  "time": 1791905152000,
  "syncChangesIndex": 140
}
```

### 15:25:52.211   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 140
}
```

### 15:25:52.211   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.213   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.215   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.217   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.221   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 142,
  "syncChangesIndex": 142,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}"
}
```

### 15:25:52.221   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAABwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 142,
  "syncChangesIndex": 142,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAgAAAA=\"}"
}
```

### 15:25:52.221   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 140,
  "time": 1791905152000,
  "syncChangesIndex": 142
}
```

### 15:25:52.271   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.274   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.276   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 142
}
```

### 15:25:52.277   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.311   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.316   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 145,
  "syncChangesIndex": 145,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}"
}
```

### 15:25:52.316   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACAAAAAEAAABmAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACQAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 145,
  "syncChangesIndex": 145,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAoAAAA=\"}"
}
```

### 15:25:52.316   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 142,
  "time": 1791905152000,
  "syncChangesIndex": 145
}
```

### 15:25:52.362   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.364   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.409   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 145
}
```

### 15:25:52.409   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.410   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.414   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 148,
  "syncChangesIndex": 148,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}"
}
```

### 15:25:52.414   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACgAAAAEAAABvAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAACwAAAAEAAABtAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 148,
  "syncChangesIndex": 148,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AAwAAAA=\"}"
}
```

### 15:25:52.415   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 145,
  "time": 1791905152000,
  "syncChangesIndex": 148
}
```

### 15:25:52.435   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.443   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.467   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 148
}
```

### 15:25:52.468   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.469   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.473   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 150,
  "syncChangesIndex": 150,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}"
}
```

### 15:25:52.473   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAADAAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 150,
  "syncChangesIndex": 150,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA0AAAA=\"}"
}
```

### 15:25:52.473   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 148,
  "time": 1791905152000,
  "syncChangesIndex": 150
}
```

### 15:25:52.521   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 150
}
```

### 15:25:52.521   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.524   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.527   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.550   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.554   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 152,
  "syncChangesIndex": 152,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}"
}
```

### 15:25:52.554   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADQAAAAEAAABBAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 152,
  "syncChangesIndex": 152,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA4AAAA=\"}"
}
```

### 15:25:52.554   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 150,
  "time": 1791905152000,
  "syncChangesIndex": 152
}
```

### 15:25:52.582   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 152
}
```

### 15:25:52.583   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.583   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.587   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 154,
  "syncChangesIndex": 154,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}"
}
```

### 15:25:52.587   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADgAAAAEAAABsAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 154,
  "syncChangesIndex": 154,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3AA8AAAA=\"}"
}
```

### 15:25:52.587   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 152,
  "time": 1791905152000,
  "syncChangesIndex": 154
}
```

### 15:25:52.588   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.613   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.617   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 154
}
```

### 15:25:52.617   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.618   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.622   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 156,
  "syncChangesIndex": 156,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}"
}
```

### 15:25:52.622   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAADwAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 156,
  "syncChangesIndex": 156,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABAAAAA=\"}"
}
```

### 15:25:52.622   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 154,
  "time": 1791905152000,
  "syncChangesIndex": 156
}
```

### 15:25:52.641   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.643   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.667   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 156
}
```

### 15:25:52.667   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.668   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.672   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 158,
  "syncChangesIndex": 158,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}"
}
```

### 15:25:52.672   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 156,
  "time": 1791905152000,
  "syncChangesIndex": 158
}
```

### 15:25:52.672   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEAAAAAEAAABjAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 158,
  "syncChangesIndex": 158,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABEAAAA=\"}"
}
```

### 15:25:52.711   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.716   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.737   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 158
}
```

### 15:25:52.738   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.739   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.743   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 160,
  "syncChangesIndex": 160,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}"
}
```

### 15:25:52.743   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEQAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 160,
  "syncChangesIndex": 160,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABIAAAA=\"}"
}
```

### 15:25:52.743   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 158,
  "time": 1791905152000,
  "syncChangesIndex": 160
}
```

### 15:25:52.785   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.788   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.791   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 160
}
```

### 15:25:52.791   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:52.828   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\",\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:52.833   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 164,
  "syncChangesIndex": 164,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}"
}
```

### 15:25:52.833   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADcAMAA3AAEAHAABAAAAEgAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAABAAAAAAAAAAEAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADcAMAA3AAEAHAABAAAAEwAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905152000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 164,
  "syncChangesIndex": 164,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADcAMAA3ABQAAAA=\"}"
}
```

### 15:25:52.833   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 160,
  "time": 1791905152000,
  "syncChangesIndex": 164
}
```

### 15:25:52.869   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:52.886   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:53.623   ->  Mallory         cursor

```json
{
  "type": "cursor",
  "cursor": "14;BgAAADYAOAAyAAMAAAA="
}
```

### 15:25:53.624   <-  Alice           cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADYAOAAyAAMAAAA=",
      "time": 1791905153623,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:25:53.624   <-  Bob             cursor

```json
{
  "type": "cursor",
  "messages": [
    {
      "cursor": "14;BgAAADYAOAAyAAMAAAA=",
      "time": 1791905153623,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ]
}
```

### 15:25:54.097   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 164
}
```

### 15:25:54.098   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.099   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAQAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.105   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 166,
  "syncChangesIndex": 166,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAQAAAA=\"}"
}
```

### 15:25:54.105   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 164,
  "time": 1791905154000,
  "syncChangesIndex": 166
}
```

### 15:25:54.106   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAAwAAAAEAAABFAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 166,
  "syncChangesIndex": 166,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAQAAAA=\"}"
}
```

### 15:25:54.134   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.137   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.182   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 166
}
```

### 15:25:54.183   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.184   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAYAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.187   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 169,
  "syncChangesIndex": 169,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAYAAAA=\"}"
}
```

### 15:25:54.187   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABQAAAAEAAABpAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 169,
  "syncChangesIndex": 169,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAYAAAA=\"}"
}
```

### 15:25:54.188   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 166,
  "time": 1791905154000,
  "syncChangesIndex": 169
}
```

### 15:25:54.207   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.248   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 169
}
```

### 15:25:54.249   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.252   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.253   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAcAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.257   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 171,
  "syncChangesIndex": 171,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAcAAAA=\"}"
}
```

### 15:25:54.257   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABgAAAAEAAAB0AAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 171,
  "syncChangesIndex": 171,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAcAAAA=\"}"
}
```

### 15:25:54.257   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 169,
  "time": 1791905154000,
  "syncChangesIndex": 171
}
```

### 15:25:54.277   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 171
}
```

### 15:25:54.278   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.280   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.283   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.286   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAgAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.290   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 173,
  "syncChangesIndex": 173,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAgAAAA=\"}"
}
```

### 15:25:54.290   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 171,
  "time": 1791905154000,
  "syncChangesIndex": 173
}
```

### 15:25:54.290   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAABwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 173,
  "syncChangesIndex": 173,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAgAAAA=\"}"
}
```

### 15:25:54.329   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.345   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.346   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 173
}
```

### 15:25:54.347   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.347   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAkAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.351   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 175,
  "syncChangesIndex": 175,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAkAAAA=\"}"
}
```

### 15:25:54.351   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACAAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 175,
  "syncChangesIndex": 175,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAkAAAA=\"}"
}
```

### 15:25:54.351   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 173,
  "time": 1791905154000,
  "syncChangesIndex": 175
}
```

### 15:25:54.394   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 175
}
```

### 15:25:54.394   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.397   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.401   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.402   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.406   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 177,
  "syncChangesIndex": 177,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}"
}
```

### 15:25:54.406   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAACQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 177,
  "syncChangesIndex": 177,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAoAAAA=\"}"
}
```

### 15:25:54.406   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 175,
  "time": 1791905154000,
  "syncChangesIndex": 177
}
```

### 15:25:54.452   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 177
}
```

### 15:25:54.453   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.457   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.460   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.462   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAsAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.465   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 179,
  "syncChangesIndex": 179,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAsAAAA=\"}"
}
```

### 15:25:54.466   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACgAAAAEAAABoAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 179,
  "syncChangesIndex": 179,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAsAAAA=\"}"
}
```

### 15:25:54.466   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 177,
  "time": 1791905154000,
  "syncChangesIndex": 179
}
```

### 15:25:54.504   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 179
}
```

### 15:25:54.504   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.507   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.511   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.533   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.537   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 181,
  "syncChangesIndex": 181,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}"
}
```

### 15:25:54.537   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAACwAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 181,
  "syncChangesIndex": 181,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAAwAAAA=\"}"
}
```

### 15:25:54.537   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 179,
  "time": 1791905154000,
  "syncChangesIndex": 181
}
```

### 15:25:54.548   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 181
}
```

### 15:25:54.549   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.552   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA0AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.555   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 183,
  "syncChangesIndex": 183,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA0AAAA=\"}"
}
```

### 15:25:54.555   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADAAAAAEAAABhAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 183,
  "syncChangesIndex": 183,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA0AAAA=\"}"
}
```

### 15:25:54.555   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 181,
  "time": 1791905154000,
  "syncChangesIndex": 183
}
```

### 15:25:54.556   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.596   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 183
}
```

### 15:25:54.597   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.599   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.602   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.605   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA4AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.608   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 185,
  "syncChangesIndex": 185,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA4AAAA=\"}"
}
```

### 15:25:54.609   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADQAAAAEAAABkAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 185,
  "syncChangesIndex": 185,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA4AAAA=\"}"
}
```

### 15:25:54.609   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 183,
  "time": 1791905154000,
  "syncChangesIndex": 185
}
```

### 15:25:54.675   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.678   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.680   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 185
}
```

### 15:25:54.680   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.695   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.700   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 187,
  "syncChangesIndex": 187,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}"
}
```

### 15:25:54.700   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADgAAAAEAAABlAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 187,
  "syncChangesIndex": 187,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyAA8AAAA=\"}"
}
```

### 15:25:54.700   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 185,
  "time": 1791905154000,
  "syncChangesIndex": 187
}
```

### 15:25:54.738   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 187
}
```

### 15:25:54.739   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.741   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.767   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.771   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\",\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABEAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.775   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 190,
  "syncChangesIndex": 190,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABEAAAA=\"}"
}
```

### 15:25:54.775   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAALQEAAAIAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAADwAAAAEAAAByAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"35;BgAAADYAOAAyAAEAHAABAAAAEAAAAAEAAAAuAAAAAAMAAAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 190,
  "syncChangesIndex": 190,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABEAAAA=\"}"
}
```

### 15:25:54.775   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 187,
  "time": 1791905154000,
  "syncChangesIndex": 190
}
```

### 15:25:54.821   ->  Mallory         isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 190
}
```

### 15:25:54.821   <-  Mallory         saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 15:25:54.824   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.827   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.829   ->  Mallory         saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\",\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": true,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}",
  "unlock": false,
  "releaseLocks": true
}
```

### 15:25:54.833   <-  Alice           saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 192,
  "syncChangesIndex": 192,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}"
}
```

### 15:25:54.833   <-  Bob             saveChanges

```json
{
  "type": "saveChanges",
  "changes": [
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"68;AgAAADEA//8BADpiZOfZRQAApwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAA9v///xIAAAA5AC4AMwAuADAALgAxADMAOAA=\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    },
    {
      "docid": "recmv6u1x1v1bbi",
      "change": "\"34;BgAAADYAOAAyAAEAHAABAAAAEQAAAAIAAAAgAAAAAwAAAA==\"",
      "time": 1791905154000,
      "user": "de103a11031c001000000000000000003",
      "useridoriginal": "de103a11031c00100000000000000000"
    }
  ],
  "changesIndex": 192,
  "syncChangesIndex": 192,
  "endSaveChanges": true,
  "locks": [],
  "excelAdditionalInfo": "{\"UserId\":\"de103a11031c001000000000000000003\",\"UserShortId\":\"de103a11031c00100000000000000000\",\"CursorInfo\":\"14;BgAAADYAOAAyABIAAAA=\"}"
}
```

### 15:25:54.833   <-  Mallory         unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 190,
  "time": 1791905154000,
  "syncChangesIndex": 192
}
```

### 15:25:54.851   ->  Bob             unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

### 15:25:54.877   ->  Alice           unLockDocument

```json
{
  "type": "unLockDocument",
  "isSave": false,
  "unlock": false,
  "deleteIndex": null,
  "releaseLocks": true
}
```

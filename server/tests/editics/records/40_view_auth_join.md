## Info

- Timestamp: 2026-10-14T09:36:30.293Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice (edit) connects and sends a chat message.
  2. Bob joins in view (read-only) mode: the server authenticates him directly
     (a viewer never gets a waitAuth), his auth response carries the chat
     history and the participant list with his `view: true` flag.
  3. Bob sends a chat message back.

## Events

### 09:36:21.091       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/documenteditor/main/../../../../doc/recmv7x0l3a8gqh/c/?shardkey=recmv7x0l3a8gqh&EIO=4&transport=websocket"
```

### 09:36:21.094   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "WDUvunPr2N1lMn3dAAAK",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:36:21.106   <-  Alice           license

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

### 09:36:21.227   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv7x0l3a8gqh",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
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
    "id": "recmv7x0l3a8gqh",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33303/document.docx",
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
  "time": 452,
  "supportAuthChangesAck": true
}
```

### 09:36:21.250   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "Qsxny57d8Dnz579_AAAL",
  "sessionTimeConnect": 1791970581106,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "Qsxny57d8Dnz579_AAAL",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4MGwzYThncWgiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDU4MTEwNn0sImlhdCI6MTc5MTk3MDU4MSwiZXhwIjoxNzk0NTYyNTgxfQ.-6yGqvsHLI0Tm0uhr1GKl6P2sHk-EFizpU_b00IiA5k",
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
  "openedAt": 1791977781237
}
```

### 09:36:21.332   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x0l3a8gqh/Editor.bin/Editor.bin?md5=_jM9UttNSllOQDw9AtD1-A&expires=1794565174&shardkey=recmv7x0l3a8gqh&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv7x0l3a8gqh/media/image1.png/image1.png?md5=VDDNbIa1yKqc8sN1wMdX8w&expires=1794565174&shardkey=recmv7x0l3a8gqh&filename=image1.png"
    },
    "openedAt": 1791977781237
  }
}
```

### 09:36:21.386   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:10"
}
```

### 09:36:21.482   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:95"
}
```

### 09:36:21.583   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 09:36:21.815   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:36:21.816   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:850 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:36:21.816   <-  Alice           message

```json
{
  "type": "message"
}
```

### 09:36:23.204   ->  Alice           message

```json
{
  "type": "message",
  "message": "Hello from Alice (before Bob joins)"
}
```

### 09:36:23.205   <-  Alice           message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv7x0l3a8gqh",
      "message": "Hello from Alice (before Bob joins)",
      "time": 1791970583205,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 09:36:23.918       Bob             [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-c8008123070aec5108081d557fecc47f/web-apps/apps/documenteditor/main/../../../../doc/recmv7x0l3a8gqh/c/?shardkey=recmv7x0l3a8gqh&EIO=4&transport=websocket"
```

### 09:36:23.920   <-  Bob             [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "leI824gncYZlFgbIAAAM",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:36:23.948   <-  Bob             license

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

### 09:36:23.967   ->  Bob             auth

```json
{
  "type": "auth",
  "docid": "recmv7x0l3a8gqh",
  "documentCallbackUrl": "http://host.containers.internal:33303/callback",
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
    "id": "recmv7x0l3a8gqh",
    "userid": "de10808c001000000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33303/document.docx",
    "title": "document.docx",
    "nobase64": true,
    "outputformat": 8193,
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
  "headingsColor": null,
  "coEditingMode": "fast",
  "time": 240,
  "supportAuthChangesAck": true
}
```

### 09:36:23.978   <-  Bob             auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "GP2OEJZF_BaSCxL4AAAN",
  "sessionTimeConnect": 1791970583948,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "Qsxny57d8Dnz579_AAAL",
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
      "connectionId": "GP2OEJZF_BaSCxL4AAAN",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "messages": [
    {
      "docid": "recmv7x0l3a8gqh",
      "message": "Hello from Alice (before Bob joins)",
      "time": 1791970583205,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ],
  "locks": {},
  "indexUser": 2,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjd4MGwzYThncWgiLCJwZXJtaXNzaW9ucyI6eyJlZGl0IjpmYWxzZX0sImRzX2VuY3J5cHRlZCI6ZmFsc2V9LCJlZGl0b3JDb25maWciOnsidXNlciI6eyJpZCI6ImRlMTA4MDhjMDAxMDAwMDAwMDAwMDAwMDAwMDAwMDAwIiwibmFtZSI6IkJvYiIsImluZGV4IjoyfSwiY29FZGl0aW5nIjp7Im1vZGUiOiJmYXN0In0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MTk3MDU4Mzk0OH0sImlhdCI6MTc5MTk3MDU4MywiZXhwIjoxNzk0NTYyNTgzfQ.lLn1kXewLn66HMAW8ATkUXsmqnLZULWuJd9oOqglTBU",
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
  "openedAt": 1791977781237
}
```

### 09:36:23.978   <-  Alice           connectState

```json
{
  "type": "connectState",
  "participantsTimestamp": 1791970583972,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "Qsxny57d8Dnz579_AAAL",
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
      "connectionId": "GP2OEJZF_BaSCxL4AAAN",
      "isCloseCoAuthoring": false,
      "isLiveViewer": true,
      "encrypted": false
    }
  ],
  "waitAuth": false
}
```

### 09:36:23.979   <-  Bob             documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv7x0l3a8gqh/Editor.bin/Editor.bin?md5=_jM9UttNSllOQDw9AtD1-A&expires=1794565174&shardkey=recmv7x0l3a8gqh&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv7x0l3a8gqh/media/image1.png/image1.png?md5=VDDNbIa1yKqc8sN1wMdX8w&expires=1794565174&shardkey=recmv7x0l3a8gqh&filename=image1.png"
    },
    "openedAt": 1791977781237
  }
}
```

### 09:36:23.988   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:2"
}
```

### 09:36:24.509   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:48"
}
```

### 09:36:24.561   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:52"
}
```

### 09:36:24.701   ->  Bob             getMessages

```json
{
  "type": "getMessages"
}
```

### 09:36:24.702   ->  Bob             clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:859 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":76600000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:36:24.702   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv7x0l3a8gqh",
      "message": "Hello from Alice (before Bob joins)",
      "time": 1791970583205,
      "user": "de10a11cec00100000000000000000001",
      "useridoriginal": "de10a11cec0010000000000000000000",
      "username": "Alice"
    }
  ]
}
```

### 09:36:26.019   ->  Bob             message

```json
{
  "type": "message",
  "message": "Hello from Bob the viewer"
}
```

### 09:36:26.020   <-  Bob             message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv7x0l3a8gqh",
      "message": "Hello from Bob the viewer",
      "time": 1791970586020,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000",
      "username": "Bob"
    }
  ]
}
```

### 09:36:26.020   <-  Alice           message

```json
{
  "type": "message",
  "messages": [
    {
      "docid": "recmv7x0l3a8gqh",
      "message": "Hello from Bob the viewer",
      "time": 1791970586020,
      "user": "de10808c0010000000000000000000002",
      "useridoriginal": "de10808c001000000000000000000000",
      "username": "Bob"
    }
  ]
}
```

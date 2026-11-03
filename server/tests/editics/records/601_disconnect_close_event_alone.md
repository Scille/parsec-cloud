## Info

- Timestamp: 2026-10-15T09:58:44.664Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document alone.
  2. Alice sends the `close` client event (the event the editor sends when it must tell the server it is leaving, e.g. on rights revocation).
  3. The document server answers with a new session token (refreshToken), then closes the connection after about 10 seconds.
  4. Alice closes her editor.

## Events

### 09:58:30.241       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9d8xdmfho5/c/?shardkey=recmv9d8xdmfho5&EIO=4&transport=websocket"
```

### 09:58:30.243   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "ARLQDnrYLehkthlaAABo",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:58:30.249   <-  Alice           license

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

### 09:58:30.361   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv9d8xdmfho5",
  "documentCallbackUrl": "http://host.containers.internal:33093/callback",
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
    "id": "recmv9d8xdmfho5",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "http://host.containers.internal:33093/document.docx",
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
  "time": 395,
  "supportAuthChangesAck": true
}
```

### 09:58:30.377   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "OYvlL41saWmtllK3AABp",
  "sessionTimeConnect": 1792058310249,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "OYvlL41saWmtllK3AABp",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkOHhkbWZobzUiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjA1ODMxMDI0OX0sImlhdCI6MTc5MjA1ODMxMCwiZXhwIjoxNzk0NjUwMzEwfQ.ttfOecGVyzOG-w9SJh2Npyn66xQgvJdGu1FvXtY66n0",
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
  "openedAt": 1792065510368
}
```

### 09:58:30.435   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9d8xdmfho5/Editor.bin/Editor.bin?md5=LQgDpYiJy_s74WII1ZaLhQ&expires=1794652903&shardkey=recmv9d8xdmfho5&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9d8xdmfho5/media/image1.png/image1.png?md5=bQIF4LZMyjooR2exJDTGOg&expires=1794652903&shardkey=recmv9d8xdmfho5&filename=image1.png"
    },
    "openedAt": 1792065510368
  }
}
```

### 09:58:30.445   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:4"
}
```

### 09:58:30.592   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:81"
}
```

### 09:58:30.693   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 09:58:30.894   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:58:30.894   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:762 memory:{\"totalJSHeapSize\":97400000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:58:30.894   <-  Alice           message

```json
{
  "type": "message"
}
```

### 09:58:31.894   ->  Alice           close

```json
{
  "type": "close"
}
```

### 09:58:31.895   <-  Alice           refreshToken

```json
{
  "type": "refreshToken",
  "messages": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkOHhkbWZobzUiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6dHJ1ZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkyMDU4MzEwMjQ5fSwiaWF0IjoxNzkyMDU4MzExLCJleHAiOjE3OTQ2NTAzMTF9.suV5wDU5cFuUfmXNb70vtm07cwU1sNZ2tUy5UbNhQ6k"
}
```

### 09:58:41.947       Alice           [Transport WebSocket] ws-close

```json
"client close, code 1001"
```

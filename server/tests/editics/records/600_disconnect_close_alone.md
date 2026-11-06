## Info

- Timestamp: 2026-10-15T09:58:28.121Z
- OnlyOffice version: 9.3.0 (build 138)
- Document: document.docx (word)
- Scenario:
  1. Alice opens the document alone.
  2. Alice closes her editor: the client sends no `close` event, the connection is closed (code 1001) and the editor does not reconnect.

## Events

### 09:58:23.735       Alice           [Transport WebSocket] ws-open

```json
"ws://127.0.0.1:8080/9.3.0-95fe30f45debd69df6ca3b0ba5e849c3/web-apps/apps/documenteditor/main/../../../../doc/recmv9d8sbqq59e/c/?shardkey=recmv9d8sbqq59e&EIO=4&transport=websocket"
```

### 09:58:23.738   <-  Alice           [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "QoXtYvPWWDJO98_PAABm",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 09:58:23.750   <-  Alice           license

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

### 09:58:23.866   ->  Alice           auth

```json
{
  "type": "auth",
  "docid": "recmv9d8sbqq59e",
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
    "id": "recmv9d8sbqq59e",
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
  "time": 423,
  "supportAuthChangesAck": true
}
```

### 09:58:23.883   <-  Alice           auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "VXv0WOf_QjioucMMAABn",
  "sessionTimeConnect": 1792058303750,
  "participants": [
    {
      "id": "de10a11cec00100000000000000000001",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "VXv0WOf_QjioucMMAABn",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiJyZWNtdjlkOHNicXE1OWUiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlfSwiZHNfZW5jcnlwdGVkIjpmYWxzZX0sImVkaXRvckNvbmZpZyI6eyJ1c2VyIjp7ImlkIjoiZGUxMGExMWNlYzAwMTAwMDAwMDAwMDAwMDAwMDAwMDAiLCJuYW1lIjoiQWxpY2UiLCJpbmRleCI6MX0sImRzX2lzQ2xvc2VDb0F1dGhvcmluZyI6ZmFsc2UsImRzX3Nlc3Npb25UaW1lQ29ubmVjdCI6MTc5MjA1ODMwMzc1MH0sImlhdCI6MTc5MjA1ODMwMywiZXhwIjoxNzk0NjUwMzAzfQ.p-i0xZ3sgo3pR27XUN-BDQARXasOnYKYvGdfQXYDChg",
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
  "openedAt": 1792065503874
}
```

### 09:58:23.940   <-  Alice           documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "http://127.0.0.1:8080/cache/files/data/recmv9d8sbqq59e/Editor.bin/Editor.bin?md5=t1S9qEOZdKUOuRw_a40Jcw&expires=1794652896&shardkey=recmv9d8sbqq59e&filename=Editor.bin",
      "media/image1.png": "http://127.0.0.1:8080/cache/files/data/recmv9d8sbqq59e/media/image1.png/image1.png?md5=EcBlpR_CsDh5hMSN1xkG5Q&expires=1794652896&shardkey=recmv9d8sbqq59e&filename=image1.png"
    },
    "openedAt": 1792065503874
  }
}
```

### 09:58:24.002   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:11"
}
```

### 09:58:24.084   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:80"
}
```

### 09:58:24.184   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:101"
}
```

### 09:58:24.393   ->  Alice           getMessages

```json
{
  "type": "getMessages"
}
```

### 09:58:24.393   ->  Alice           clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:779 memory:{\"totalJSHeapSize\":91700000,\"usedJSHeapSize\":81400000,\"jsHeapSizeLimit\":3760000000}"
}
```

### 09:58:24.394   <-  Alice           message

```json
{
  "type": "message"
}
```

### 09:58:25.406       Alice           [Transport WebSocket] ws-close

```json
"client close, code 1001"
```

## Info

- Timestamp: 2026-10-12T14:03:49.106Z
- OnlyOffice version: 10.0.0 (build 169)

## Events

### 14:03:27.787       Alice      [Transport WebSocket] ws-open

```json
"wss://site.docs.onlyoffice.com/10.0.0-c8623c2f0b7dc9b9da7f9b0a9977fb5b/web-apps/apps/documenteditor/main/../../../../doc/087148be-7ddd-4ebd-b558-02f0930e81d4/c/?shardkey=087148be-7ddd-4ebd-b558-02f0930e81d4&EIO=4&transport=websocket"
```

### 14:03:28.118   <-  Alice      [Transport Engine.IO] open

```json
{
  "eio": "open",
  "payload": {
    "sid": "KncjmwtEARkdUbEuAFvH",
    "upgrades": [],
    "pingInterval": 25000,
    "pingTimeout": 20000,
    "maxPayload": 100000000
  }
}
```

### 14:03:28.277   <-  Alice      license

```json
{
  "type": "license",
  "license": {
    "type": 3,
    "light": false,
    "mode": 0,
    "rights": 1,
    "buildVersion": "10.0.0",
    "buildNumber": 169,
    "protectionSupport": true,
    "isAnonymousSupport": true,
    "liveViewerSupport": true,
    "branding": true,
    "customization": true,
    "advancedApi": true
  },
  "aiPluginSettingsAllowBrowserProxy": true
}
```

### 14:03:28.387   ->  Alice      auth

```json
{
  "type": "auth",
  "docid": "087148be-7ddd-4ebd-b558-02f0930e81d4",
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
    "id": "087148be-7ddd-4ebd-b558-02f0930e81d4",
    "userid": "de10a11cec0010000000000000000000",
    "format": "docx",
    "url": "https://static.onlyoffice.com/assets/docs/samples/demo.docx",
    "title": "Example Document Title.docx",
    "lcid": 9,
    "nobase64": true,
    "outputformat": 8193,
    "convertToOrigin": ".pdf.xps.oxps.djvu.ofd"
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
  "jwtOpen": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJmaWxlVHlwZSI6ImRvY3giLCJrZXkiOiIwODcxNDhiZS03ZGRkLTRlYmQtYjU1OC0wMmYwOTMwZTgxZDQiLCJ0aXRsZSI6IkV4YW1wbGUgRG9jdW1lbnQgVGl0bGUuZG9jeCIsInVybCI6Imh0dHBzOi8vc3RhdGljLm9ubHlvZmZpY2UuY29tL2Fzc2V0cy9kb2NzL3NhbXBsZXMvZGVtby5kb2N4IiwicGVybWlzc2lvbnMiOnsiZWRpdCI6dHJ1ZSwicmV2aWV3Ijp0cnVlfX0sImRvY3VtZW50VHlwZSI6IndvcmQiLCJlZGl0b3JDb25maWciOnsibGFuZyI6ImVuIiwidXNlciI6eyJpZCI6Ijc4ZTFlODQxIiwibmFtZSI6IkpvaG4gU21pdGgifSwiY3VzdG9taXphdGlvbiI6eyJoaWRlUmlnaHRNZW51Ijp0cnVlLCJpbnRlZ3JhdGlvbk1vZGUiOiJlbWJlZCIsImFub255bW91cyI6eyJyZXF1ZXN0IjpmYWxzZX19LCJwbHVnaW5zIjp7InBsdWdpbnNEYXRhIjpbImh0dHBzOi8vd3d3Lm9ubHlvZmZpY2UuY29tL3BsdWdpbi1yYWluYm93L2NvbmZpZy5qc29uIl19fSwid2lkdGgiOiIxMDAlIiwiaGVpZ2h0IjoiMTAwJSIsImlhdCI6MTc5MTgxMzgwNn0.jzvhXxLutpW4e8uWQjhmUmm2WIlyRQCB0hPOWQKdv3E",
  "time": 883,
  "supportAuthChangesAck": true
}
```

### 14:03:28.568   <-  Alice      auth

```json
{
  "type": "auth",
  "result": 1,
  "sessionId": "4a51295bf2d04b47b6269a8e33b120e0",
  "sessionTimeConnect": 1791813808199,
  "participants": [
    {
      "id": "4a51295bf2d04b47b6269a8e33b120e0",
      "idOriginal": "de10a11cec0010000000000000000000",
      "username": "Alice",
      "indexUser": 1,
      "view": false,
      "connectionId": "jPqt6t6oF5i9JIyxAFvI",
      "isCloseCoAuthoring": false,
      "isLiveViewer": false,
      "encrypted": false
    }
  ],
  "locks": {},
  "indexUser": 1,
  "hasForgotten": false,
  "jwt": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkb2N1bWVudCI6eyJrZXkiOiIwODcxNDhiZS03ZGRkLTRlYmQtYjU1OC0wMmYwOTMwZTgxZDQiLCJwZXJtaXNzaW9ucyI6eyJlZGl0Ijp0cnVlLCJyZXZpZXciOnRydWV9LCJkc19lbmNyeXB0ZWQiOmZhbHNlfSwiZWRpdG9yQ29uZmlnIjp7InVzZXIiOnsiaWQiOiI3OGUxZTg0MSIsIm5hbWUiOiJKb2huIFNtaXRoIiwiaW5kZXgiOjF9LCJkc192aWV3IjpmYWxzZSwiZHNfaXNDbG9zZUNvQXV0aG9yaW5nIjpmYWxzZSwiZHNfc2Vzc2lvblRpbWVDb25uZWN0IjoxNzkxODEzODA4MTk5fSwiaWF0IjoxNzkxODEzODA4LCJleHAiOjE3OTQ0MDU4MDh9.m1Kg0X78UeCWy4_wBWot7ltkPxHNiavByiHgnIcWBWs",
  "buildVersion": "10.0.0",
  "buildNumber": 169,
  "licenseType": 3,
  "settings": {
    "reconnection": {
      "attempts": 50,
      "delay": 2000
    },
    "binaryChanges": false,
    "websocketMaxPayloadSize": 1572864,
    "maxChangesSize": 157286400,
    "limits_image_size": 26214400,
    "limits_image_types_upload": "jpg;jpeg;jpe;jfif;jif;png;gif;bmp;svg;tiff;tif;webp;heic;heif;avif"
  },
  "openedAt": 1791821008470
}
```

### 14:03:28.998   <-  Alice      documentOpen

```json
{
  "type": "documentOpen",
  "data": {
    "type": "open",
    "status": "ok",
    "data": {
      "Editor.bin": "https://site.docs.onlyoffice.com/cache/files/10.0.0-169/data/site/087148be-7ddd-4ebd-b558-02f0930e81d4/Editor.bin/Editor.bin?md5=dhkK1Vd-BPuM7O7EDHX43A&expires=1794408401&shardkey=087148be-7ddd-4ebd-b558-02f0930e81d4&filename=Editor.bin"
    },
    "openedAt": 1791821008470
  }
}
```

### 14:03:29.492   ->  Alice      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDownloadFile time:491"
}
```

### 14:03:29.601   ->  Alice      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onOpenDocument time:109"
}
```

### 14:03:29.711   ->  Alice      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onLoadFonts time:111"
}
```

### 14:03:30.038   ->  Alice      getMessages

```json
{
  "type": "getMessages"
}
```

### 14:03:30.039   ->  Alice      clientLog

```json
{
  "type": "clientLog",
  "level": "debug",
  "msg": "onDocumentContentReady time:2248 memory:{\"totalJSHeapSize\":171198169,\"usedJSHeapSize\":125899981,\"jsHeapSizeLimit\":4395630592}"
}
```

### 14:03:30.191   <-  Alice      message

```json
{
  "type": "message"
}
```

### 14:03:48.793   ->  Alice      isSaveLock

```json
{
  "type": "isSaveLock",
  "syncChangesIndex": 0
}
```

### 14:03:48.945   <-  Alice      saveLock

```json
{
  "type": "saveLock",
  "saveLock": false
}
```

### 14:03:48.948   ->  Alice      saveChanges

```json
{
  "type": "saveChanges",
  "changes": "[\"70;AgAAADEA//8BAFOEF4I2jg4AjgAAAAMAAAAAAAAAAAAAAAAAAAAAAAAA9v///xQAAAAxADAALgAwAC4AMAAuADEANgA5AA==\",\"72;CAAAADEAMQAzADcAAgADAAMAAAABAAAACAAAADEAMQA0ADEAAQAAAAgAAAAxADEANAAyAAEAAAAIAAAAMQAxADQAMwAAAAAA\",\"165;CAAAADEAMQA0ADAAAgAcAAsAAAAAAAAAAQAAAFcAAAAAAAAAAAEAAABlAAAAAAAAAAABAAAAbAAAAAAAAAAAAQAAAGMAAAAAAAAAAAEAAABvAAAAAAAAAAABAAAAbQAAAAAAAAAAAQAAAGUAAAAAAAAAAAIAAAAgAAAAAAAAAAEAAAB0AAAAAAAAAAABAAAAbwAAAAAAAAAAAgAAACAAAAAAAAAA\",\"40;CAAAADEAMQAzADcAAgADAAEAAAAAAAAACAAAADEAMQA0ADAAAAAAAA==\"]",
  "startSaveChanges": true,
  "endSaveChanges": true,
  "isCoAuthoring": false,
  "isExcel": false,
  "deleteIndex": null,
  "excelAdditionalInfo": "{\"UserId\":\"4a51295bf2d04b47b6269a8e33b120e0\",\"UserShortId\":\"de10a11cec0010000000000000000000\",\"CursorInfo\":\"16;CAAAADEAMQA0ADQAAAAAAA==\"}",
  "unlock": false,
  "releaseLocks": false
}
```

### 14:03:49.106   <-  Alice      unSaveLock

```json
{
  "type": "unSaveLock",
  "index": 0,
  "time": 1791813829000,
  "syncChangesIndex": 4
}
```
